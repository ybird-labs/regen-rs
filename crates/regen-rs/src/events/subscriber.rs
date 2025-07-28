use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use tokio::sync::{RwLock, mpsc};

use futures::stream::{SplitSink, SplitStream, Stream};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::{net::TcpStream, task::JoinHandle};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{Message, client::IntoClientRequest},
};

use cosmos_sdk_proto::tendermint::abci::Event;

use crate::RegenError;

use log::{debug, error, info, trace, warn};

pub type SubscriptionId = u32;

pub enum Command {
    Subscribe(Subscription),
    Unsubscribe(SubscriptionId),
    Close,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub query: String,
}

pub struct EventSubscriber {
    next_id: AtomicU32,
    subscriptions: RwLock<HashMap<SubscriptionId, Subscription>>,
    command_tx: mpsc::Sender<Command>,
    event_rx: mpsc::Receiver<Event>,
    _supervisor_handle: JoinHandle<Result<(), RegenError>>,
}

impl EventSubscriber {
    pub async fn new(ws_url: &str) -> Result<Self, RegenError> {
        let (command_tx, command_rx) = mpsc::channel(100);
        let (event_tx, event_rx) = mpsc::channel(100);

        let ws_url = ws_url.to_string();

        let supervisor_handle =
            tokio::spawn(conection_supervisor(ws_url, command_rx, event_tx.clone()));

        Ok(Self {
            next_id: AtomicU32::new(0),
            subscriptions: RwLock::new(HashMap::new()),
            command_tx,
            event_rx,
            _supervisor_handle: supervisor_handle,
        })
    }

    pub async fn subscribe(&mut self, query: &str) -> Result<(), RegenError> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let subscription = Subscription {
            id,
            query: query.to_string(),
        };

        self.subscriptions
            .write()
            .await
            .insert(subscription.id, subscription.clone());

        self.command_tx
            .send(Command::Subscribe(subscription))
            .await
            .map_err(|e| {
                RegenError::Internal(format!("Failed to send subscribe command: {}", e))
            })?;

        Ok(())
    }

    pub async fn unsubscribe(&mut self, id: SubscriptionId) -> Result<(), RegenError> {
        self.subscriptions.write().await.remove(&id);

        Ok(())
    }
}

async fn conection_supervisor(
    ws_url: String,
    mut command_rx: mpsc::Receiver<Command>,
    event_tx: mpsc::Sender<Event>,
) -> Result<(), RegenError> {
    loop {
        let result = connect_and_run(ws_url.clone(), &mut command_rx, event_tx.clone()).await;
        if let Err(e) = result {
            error!("Connection failed: {}", e);
        }
    }
}

async fn connect_and_run(
    ws_url: String,
    command_rx: &mut mpsc::Receiver<Command>,
    event_tx: mpsc::Sender<Event>,
) -> Result<(), RegenError> {
    let request = ws_url
        .into_client_request()
        .map_err(RegenError::WebSocket)?;

    let (stream, _) = tokio_tungstenite::connect_async(request).await?;
    let (sink, stream) = stream.split();

    tokio::select! {
        _ = sink_loop(sink, command_rx) => {
            error!("Sink task failed");
        }
        _ = stream_loop(stream, event_tx) => {
            error!("Stream task failed");
        }
    }

    Ok(())
}

async fn sink_loop(
    mut sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    command_rx: &mut mpsc::Receiver<Command>,
) -> Result<(), RegenError> {
    while let Some(command) = command_rx.recv().await {
        let message = match command {
            Command::Subscribe(subscription) => {
                let subscribe_msg = json!({
                "jsonrpc": "2.0",
                "method": "subscribe",
                        "id": subscription.id,
                "params": {
                            "query": subscription.query
                        }
                    });
                Message::Text(subscribe_msg.to_string().into())
            }
            Command::Unsubscribe(id) => {
                let unsubscribe_msg = json!({
                            "jsonrpc": "2.0",
                            "method": "unsubscribe",
                            "id": id
                });
                Message::Text(unsubscribe_msg.to_string().into())
            }
            Command::Close => break,
        };

        sink.send(message).await.map_err(RegenError::WebSocket)?;
    }
    Ok(())
}

async fn stream_loop(
    mut stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    event_tx: mpsc::Sender<Event>,
) -> Result<(), RegenError> {
    info!("WebSocket stream loop started");

    while let Some(message) = stream.next().await {
        match message {
            Ok(Message::Text(text)) => {
                trace!(
                    "Received text message: {}",
                    text.chars().take(100).collect::<String>()
                );

                match serde_json::from_str::<Event>(&text) {
                    Ok(event) => {
                        debug!("Successfully parsed event of type: {}", event.r#type);

                        if event_tx.send(event).await.is_err() {
                            warn!("Event receiver dropped, closing stream loop");
                            return Err(RegenError::Internal(
                                "Event receiver channel dropped".to_string(),
                            ));
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Failed to parse event JSON: {} - Raw text: {}",
                            e,
                            text.chars().take(200).collect::<String>()
                        );
                    }
                }
            }
            Ok(Message::Close(close_frame)) => {
                info!("WebSocket connection closed: {:?}", close_frame);
                break;
            }
            Ok(Message::Ping(_)) => {
                trace!("Received ping (auto-handled)");
            }
            Ok(Message::Pong(_)) => {
                trace!("Received pong");
            }
            Ok(msg) => {
                debug!("Received other message type: {:?}", msg);
            }
            Err(e) => {
                error!("WebSocket connection error: {}", e);
                info!("WebSocket stream loop ended due to error");
                return Err(RegenError::WebSocket(e));
            }
        }
    }

    info!("WebSocket stream loop ended");
    Ok(())
}

impl Stream for EventSubscriber {
    type Item = Event;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.event_rx.poll_recv(cx)
    }
}

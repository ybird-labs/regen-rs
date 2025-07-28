use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use tokio::sync::{RwLock, mpsc};

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt, future::ok};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{Message, client::IntoClientRequest},
};

use cosmos_sdk_proto::{cosmos::upgrade::v1beta1::ModuleVersion, tendermint::abci::Event};

use crate::RegenError;

use log::{debug, error, info, trace, warn};

pub type SubscriptionId = u32;

pub enum Command {
    Subscribe(Subscription),
    Unsubscribe(SubscriptionId),
    Close,
}

pub struct Subscription {
    pub id: SubscriptionId,
    pub query: String,
}

pub struct EventSubscriber {
    ws_url: String,
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    next_id: AtomicU32,
    subscriptions: RwLock<HashMap<SubscriptionId, Subscription>>,
}

impl EventSubscriber {
    pub async fn new(ws_url: &str) -> Result<Self, RegenError> {
        let request = ws_url
            .into_client_request()
            .map_err(RegenError::WebSocket)?;

        let (stream, _) = tokio_tungstenite::connect_async(request).await?;

        Ok(Self {
            ws_url: ws_url.to_string(),
            stream,
            next_id: AtomicU32::new(0),
            subscriptions: RwLock::new(HashMap::new()),
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
            .insert(subscription.id, subscription);

        // let subscribe_msg = json!({
        //     "jsonrpc": "2.0",
        //     "method": "subscribe",
        //     "id": id,
        //     "params": {
        //         "query": query
        //     }
        // });

        // self.stream
        //     .send(Message::Text(subscribe_msg.to_string().into()))
        //     .await?;

        Ok(())
    }

    pub async fn unsubscribe(&mut self, id: SubscriptionId) -> Result<(), RegenError> {
        self.subscriptions.write().await.remove(&id);

        Ok(())
    }

    pub async fn connect(
        &mut self,
        command_rx: mpsc::Receiver<Command>,
        event_tx: mpsc::Sender<Event>,
    ) -> Result<(), RegenError> {
        let request = self
            .ws_url
            .clone()
            .into_client_request()
            .map_err(RegenError::WebSocket)?;

        let (stream, _) = tokio_tungstenite::connect_async(request).await?;
        let (sink, stream) = stream.split();

        let sink_handle = tokio::spawn(async move {
            sink_loop(sink, command_rx).await;
        });

        let stream_handle = tokio::spawn(async move {
            stream_loop(stream, event_tx).await;
        });

        tokio::join!(sink_handle, stream_handle);

        Ok(())
    }
}

async fn sink_loop(
    mut sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut command_rx: mpsc::Receiver<Command>,
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

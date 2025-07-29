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

struct EventSubscriberState {
    next_id: AtomicU32,
    subscriptions: RwLock<HashMap<SubscriptionId, Subscription>>,
    command_tx: mpsc::Sender<Command>,
    event_rx: mpsc::Receiver<Event>,
    supervisor_handle: JoinHandle<Result<(), RegenError>>,
}

impl EventSubscriberState {
    async fn new(ws_url: String) -> Result<Self, RegenError> {
        let (command_tx, command_rx) = mpsc::channel(100);
        let (event_tx, event_rx) = mpsc::channel(100);

        let supervisor_handle =
            tokio::spawn(connection_supervisor(ws_url, command_rx, event_tx.clone()));

        Ok(Self {
            next_id: AtomicU32::new(0),
            subscriptions: RwLock::new(HashMap::new()),
            command_tx,
            event_rx,
            supervisor_handle,
        })
    }
}

pub struct EventSubscriber {
    ws_url: String,
    state: EventSubscriberState,
}

impl Drop for EventSubscriber {
    fn drop(&mut self) {
        // Safety net: ensure cleanup even if graceful shutdown fails
        if !self.state.supervisor_handle.is_finished() {
            warn!("EventSubscriber dropped with running task, aborting for safety");
            self.state.supervisor_handle.abort();
        }
    }
}

impl EventSubscriber {
    pub async fn new(ws_url: &str) -> Result<Self, RegenError> {
        let state = EventSubscriberState::new(ws_url.to_string()).await?;

        Ok(Self {
            ws_url: ws_url.to_string(),
            state,
        })
    }

    pub async fn subscribe(&mut self, query: &str) -> Result<(), RegenError> {
        let id = self.state.next_id.fetch_add(1, Ordering::Relaxed);

        let subscription = Subscription {
            id,
            query: query.to_string(),
        };

        self.state
            .subscriptions
            .write()
            .await
            .insert(subscription.id, subscription.clone());

        self.state
            .command_tx
            .send(Command::Subscribe(subscription))
            .await
            .map_err(|e| RegenError::Internal(format!("Failed to send subscribe command: {e}")))?;

        Ok(())
    }

    pub async fn unsubscribe(&mut self, id: SubscriptionId) -> Result<(), RegenError> {
        self.state.subscriptions.write().await.remove(&id);

        self.state
            .command_tx
            .send(Command::Unsubscribe(id))
            .await
            .map_err(|e| {
                RegenError::Internal(format!("Failed to send unsubscribe command: {e}"))
            })?;

        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), RegenError> {
        self.state
            .command_tx
            .send(Command::Close)
            .await
            .map_err(|e| RegenError::Internal(format!("Failed to send close command: {e}")))?;
        Ok(())
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        self.state.event_rx.recv().await
    }

    pub fn is_active(&self) -> bool {
        !self.state.supervisor_handle.is_finished()
    }

    async fn reset_connection_and_state(&mut self) -> Result<(), RegenError> {
        let state = EventSubscriberState::new(self.ws_url.clone()).await?;
        self.state = state;
        Ok(())
    }

    /// Reconnects to the event stream.
    ///
    /// This will shutdown the current connection and create a new one.
    /// It will also reset the state of the event subscriber.
    ///
    /// This is useful when the connection is lost or when the event stream is not responding.
    ///
    pub async fn reconnect(&mut self) -> Result<(), RegenError> {
        if self.is_active() {
            self.shutdown().await?;
        }
        self.reset_connection_and_state().await?;

        Ok(())
    }
}

async fn connection_supervisor(
    ws_url: String,
    mut command_rx: mpsc::Receiver<Command>,
    event_tx: mpsc::Sender<Event>,
) -> Result<(), RegenError> {
    while !command_rx.is_closed() {
        let result = connect_and_run(ws_url.clone(), &mut command_rx, event_tx.clone()).await;

        if let Err(e) = result {
            error!("Connection failed: {e}");
            // Exponential backoff before retry
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        } else {
            break;
        }
    }

    info!("Command channel closed, supervisor shutting down gracefully");
    Ok(())
}

async fn connect_and_run(
    ws_url: String,
    command_rx: &mut mpsc::Receiver<Command>,
    event_tx: mpsc::Sender<Event>,
) -> Result<(), RegenError> {
    let request = ws_url
        .into_client_request()
        .map_err(|e| RegenError::WebSocket(Box::new(e)))?;

    let (stream, _) = tokio_tungstenite::connect_async(request)
        .await
        .map_err(|e| RegenError::WebSocket(Box::new(e)))?;
    let (sink, stream) = stream.split();

    tokio::select! {
        result = sink_loop(sink, command_rx) => {
            if let Err(e) = result {
                error!("Sink task failed: {e}");
                return Err(e);
            }
        }
        result = stream_loop(stream, event_tx) => {
            if let Err(e) = result {
                error!("Stream task failed: {e}");
                return Err(e);
            }
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

        sink.send(message)
            .await
            .map_err(|e| RegenError::WebSocket(Box::new(e)))?;
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
                info!("WebSocket connection closed: {close_frame:?}");
                break;
            }
            Ok(Message::Ping(_)) => {
                trace!("Received ping (auto-handled)");
            }
            Ok(Message::Pong(_)) => {
                trace!("Received pong");
            }
            Ok(msg) => {
                debug!("Received other message type: {msg:?}");
            }
            Err(e) => {
                error!("WebSocket connection error: {e}");
                info!("WebSocket stream loop ended due to error");
                return Err(RegenError::WebSocket(Box::new(e)));
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
        self.state.event_rx.poll_recv(cx)
    }
}

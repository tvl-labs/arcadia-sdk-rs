use alloy::primitives::Address;
use anyhow::{Context, Result};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use futures::{SinkExt, StreamExt};
use rand::RngCore;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::http::Request;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

use crate::types::ws::{WsBroadcastMessage, WsPayload};

/// Connects to Medusa WebSocket and spawns a task to handle messages
///
/// Returns: (task_handle, broadcast_receiver, payload_sender, close_sender)
///
/// - `broadcast_receiver`: Receive messages from the WebSocket
/// - `payload_sender`: Send payloads to the WebSocket (cloneable)
/// - `close_sender`: Send () to close the connection (cloneable)
/// - `task_handle`: JoinHandle for the background task
pub async fn create_medusa_ws_client(
    url: String,
    signer_address: Address,
) -> Result<(
    mpsc::Receiver<WsBroadcastMessage>,
    mpsc::Sender<WsPayload>,
    mpsc::Sender<()>,
    JoinHandle<()>,
)> {
    let url = Url::parse(&url).context("Failed to parse websocket URL")?;
    let host = url.host_str().context("Url must have a host")?;
    let port = url
        .port_or_known_default()
        .context("Url must have a port")?;

    let mut random_bytes = [0u8; 16];
    rand::rng().fill_bytes(&mut random_bytes);
    let key = STANDARD.encode(random_bytes);
    let request = Request::builder()
        .uri(url.as_str())
        .header("Host", format!("{}:{}", host, port))
        .header("Upgrade", "websocket")
        .header("Connection", "Upgrade")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", key)
        .body(())
        .context("Failed to build WS request")?;

    let (ws_stream, _) = connect_async(request).await?;
    let (mut ws_write, mut ws_read) = ws_stream.split();
    ws_write
        .send(Message::Text(
            serde_json::to_string(&WsPayload::AddSolver(signer_address))?.into(),
        ))
        .await?;

    let (broadcast_send, broadcast_recv) = mpsc::channel(100);
    let (payload_send, mut payload_recv) = mpsc::channel(100);
    let (close_send, mut close_recv) = mpsc::channel(1);

    let task_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                msg = ws_read.next() => {
                    match msg {
                        Some(Ok(Message::Text(raw_message))) => {
                            let raw_message = raw_message.replace("\\\"", "\"").replace("\\'", "'");
                            match serde_json::from_str(&raw_message) {
                                Ok(message) => {
                                    if let Err(e) = broadcast_send.send(message).await {
                                        tracing::error!("Broadcast receiver dropped: {}, closing connection", e);
                                        break;
                                    }
                                },
                                Err(e) => {
                                    tracing::error!("Failed to parse WS message: {}", e);
                                    continue;
                                }
                            };
                        }
                        Some(Ok(Message::Close(frame))) => {
                            tracing::warn!("WS connection closed: {:?}", frame);
                            break;
                        }
                        Some(Err(e)) => {
                            tracing::error!("WebSocket error: {}", e);
                            break;
                        }
                        None => {
                            tracing::warn!("WS connection closed");
                            break;
                        }
                        _ => {}
                    }
                }
                Some(payload) = payload_recv.recv() => {
                    match serde_json::to_string(&payload) {
                        Ok(payload) => {
                            if let Err(e) = ws_write.send(Message::Text(payload.into())).await {
                                tracing::error!("Failed to send WS payload to medusa: {}", e);
                                continue;
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to serialize payload: {}", e);
                            continue;
                        }
                    }
                }
                _ = close_recv.recv() => {
                    let _ = ws_write.close().await;
                    break;
                }

            }
        }
        let _ = ws_write.close().await;
        tracing::info!("WS connection closed");
    });
    Ok((broadcast_recv, payload_send, close_send, task_handle))
}

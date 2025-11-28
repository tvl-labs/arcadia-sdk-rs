use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::types::rpc_payloads::SignedAddSolver;
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
    signed_add_solver: SignedAddSolver,
) -> Result<(
    mpsc::Receiver<WsBroadcastMessage>,
    mpsc::Sender<WsPayload>,
    mpsc::Sender<()>,
    JoinHandle<()>,
)> {
    let (mut ws_stream, _) = connect_async(url).await?;
    ws_stream
        .send(Message::Text(
            serde_json::to_string(&WsPayload::AddSolver(signed_add_solver))?.into(),
        ))
        .await?;

    let (broadcast_send, broadcast_recv) = mpsc::channel(100);
    let (payload_send, mut payload_recv) = mpsc::channel(100);
    let (close_send, mut close_recv) = mpsc::channel(1);

    let task_handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                msg = ws_stream.next() => {
                    match msg {
                        Some(Ok(Message::Text(raw_message))) => {
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
                        Some(Ok(Message::Ping(ping))) => {
                            if let Err(e) = ws_stream.send(Message::Pong(ping)).await {
                                tracing::error!("Failed to send Pong message: {}", e);
                                break;
                            }
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
                            if let Err(e) = ws_stream.send(Message::Text(payload.into())).await {
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
                    break;
                }

            }
        }
        let _ = ws_stream.close(None).await;
        tracing::info!("WS connection closed");
    });
    Ok((broadcast_recv, payload_send, close_send, task_handle))
}

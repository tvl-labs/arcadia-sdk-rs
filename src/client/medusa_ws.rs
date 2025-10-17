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

use crate::types::medusa::ws::{WsBroadcastMessage, WsPayload};
pub struct MedusaWsClient {
    pub broadcast_recv: mpsc::Receiver<WsBroadcastMessage>,
    payload_send: mpsc::Sender<WsPayload>,
    close_send: mpsc::Sender<()>,
    task_handle: Option<JoinHandle<()>>,
}

impl MedusaWsClient {
    pub async fn new(url: String, signer_address: Address) -> Result<Self> {
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
                                            tracing::error!("Failed to forward WS broadcast message: {}", e);
                                            continue;
                                        }
                                    },
                                    Err(e) => {
                                        tracing::error!("Failed to parse WS message: {}", e);
                                        continue;
                                    }
                                };
                            }
                            _ => {}
                        }
                    }
                    payload = payload_recv.recv() => {
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
                    }

                }
            }
        });
        Ok(Self {
            broadcast_recv,
            payload_send,
            close_send,
            task_handle: Some(task_handle),
        })
    }

    pub async fn send(&mut self, payload: WsPayload) -> Result<()> {
        self.payload_send.send(payload).await?;
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        let _ = self.close_send.send(()).await;

        if let Some(task_handle) = self.task_handle.take() {
            match tokio::time::timeout(std::time::Duration::from_secs(5), task_handle).await {
                Ok(Ok(())) => Ok(()),
                Ok(Err(e)) => Err(anyhow::anyhow!("Background task panicked: {}", e)),
                Err(_) => Err(anyhow::anyhow!("Timeout waiting for connection to close")),
            }
        } else {
            Ok(())
        }
    }
}

impl Drop for MedusaWsClient {
    fn drop(&mut self) {
        let _ = self.close_send.try_send(());
    }
}

//! WebSocket endpoint for live match simulation.
//!
//! Each WebSocket connection gets its own `SimLiveSession` that lives on
//! the Tokio task stack. When the client disconnects, the session is
//! dropped and memory is freed.

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::Response;
use futures::{SinkExt, StreamExt};
use olm_core::sim_live_bridge::{SimMessage, SimResponse};

use crate::AppState;

pub async fn sim_handler(
    ws: WebSocketUpgrade,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(move |socket| sim_session(socket, id, state))
}

async fn sim_session(socket: WebSocket, _save_id: String, _state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // For now, sim_live is stubbed until the full module integration.
    // The session struct (SimLiveSession) is in olm_core but the
    // actual runtime (init/tick/run_to_completion) needs game state
    // which must be loaded from the store.
    let err = SimResponse::Error {
        message: "Live match simulation is not yet available in web mode".to_string(),
    };
    let _ = sender
        .send(Message::Text(serde_json::to_string(&err).unwrap()))
        .await;

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(t) => {
                let response = match serde_json::from_str::<SimMessage>(&t) {
                    Ok(_sim_msg) => SimResponse::Error {
                        message: "Not implemented yet".to_string(),
                    },
                    Err(e) => SimResponse::Error {
                        message: format!("Invalid message: {e}"),
                    },
                };
                if sender
                    .send(Message::Text(serde_json::to_string(&response).unwrap()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => continue,
        }
    }
}

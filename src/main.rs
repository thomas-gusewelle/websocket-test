use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use axum::extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State}
use axum::{response::{IntoResponse, Html}, routing::get, Json, Router};
// use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_chat=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState { user_set, tx });

    let app = Router::new()
        // .route("/", get(index))
        .route("/websocket", get(websocket_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
} 

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // let (mut sender, mut reciever) = stream.split();

    let mut rx = state.tx.subscribe();

    let msg = format!("Someone Joined");
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);
}

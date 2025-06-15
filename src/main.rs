mod handlers;

use std::net::SocketAddr;

use axum::{ routing::{any, get, post}, Router};
use tower_http::services::ServeDir;
use handlers::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let serve_dir = ServeDir::new("public");
    let app = Router::new()
    .fallback_service(serve_dir)
    .route("/", get(index))
    .route("/ws", any(ws_handler))
    .route("/previous", post(previous))
    .route("/play-pause", post(play_pause))
    .route("/next", post(next));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    tracing::debug!("listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
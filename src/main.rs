use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let server_dir = ServeDir::new("public");
    let app = Router::new().fallback_service(server_dir).route("/", get(root));

    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    tracing::debug!("listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(std::include_str!("../root.html"))
}
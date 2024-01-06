use axum::{extract::State, routing::get, serve, Router};
use frobnicate::{get_frobnicator, Frobnicate};
use std::sync::Arc;
use tokio::net::TcpListener;

mod frobnicate;

async fn handler(State(frobnicator): State<Arc<impl Frobnicate>>) -> String {
    frobnicator.frobnicate().await.into()
}

#[tokio::main]
async fn main() {
    let frobnicator = get_frobnicator();
    let app = Router::new()
        .route("/", get(handler))
        .with_state(Arc::new(frobnicator));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

use axum::{extract::State, routing::get, serve, Router};
#[cfg(feature = "bar")]
use std::env::args;
use std::future::Future;
use std::sync::Arc;
use tokio::net::TcpListener;

trait Frobnicate {
    fn frobnicate(&self) -> impl Future<Output = &str> + Send;
    // async fn frobnicate(&self) -> &str;
}

struct FooFrobnicator;

struct BarFrobnicator {
    name: String,
}

#[cfg(feature = "bar")]
impl BarFrobnicator {
    fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl Frobnicate for BarFrobnicator {
    async fn frobnicate(&self) -> &str {
        &self.name
    }
}

impl Frobnicate for FooFrobnicator {
    async fn frobnicate(&self) -> &str {
        "foo"
    }
}

async fn handler(State(frobnicator): State<Arc<impl Frobnicate>>) -> String {
    frobnicator.frobnicate().await.into()
}

fn get_frobnicator() -> impl Frobnicate {
    #[cfg(not(feature = "bar"))]
    return FooFrobnicator;

    #[cfg(feature = "bar")]
    {
        let msg = args().nth(1).unwrap_or("bar".into());
        BarFrobnicator::new(&msg)
    }
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

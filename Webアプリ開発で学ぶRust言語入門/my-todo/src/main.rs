use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 3000));

    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

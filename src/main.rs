extern crate tokio;
use axum::{serve, Router};

use tokio::net::TcpListener;

use tower_http::{services::ServeDir, trace::TraceLayer};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    println!("program start!");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = TcpListener::bind("127.0.0.1:3333")
        .await
        .expect("listener to bind");
    println!(
        "listening on {}",
        listener
            .local_addr()
            .expect("listener.local_addr() to unwrap")
    );

    let rt: Router = Router::new().nest_service("/assets", ServeDir::new("assets"));

    serve(listener, rt.layer(TraceLayer::new_for_http()))
        .await
        .expect("axum to launch serve()");
}

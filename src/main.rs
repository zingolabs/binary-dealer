extern crate tokio;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{instrument, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[instrument(level = Level::TRACE)]
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=trace,rustls=trace,tokio-rustls=trace,tower=trace,tower_http=trace,axum=trace,axum_server=trace,tracing=trace,tracing_subscriber=trace,tokio=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let https_port: u16 = 3953;
    tracing::info!("set https_port to : {}", https_port);

    let app: Router = Router::new()
        .nest_service("/getme", ServeDir::new("assets/getme"))
        .nest_service("/zcashd", ServeDir::new("assets/getme"))
        .nest_service("/zainod", ServeDir::new("assets/getme"))
        .layer(TraceLayer::new_for_http());
    tracing::info!("set app as : {:?}", &app);

    serve(app, https_port).await;
}

#[instrument(level = Level::TRACE name = "binary_dealer serve function")]
async fn serve(app: Router, https_port: u16) {
    tracing::debug!("serve function launched!");
    let addr = SocketAddr::from(([127, 0, 0, 1], https_port));
    tracing::info!("{} address set!", &addr);

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("key.pem"),
    )
    .await
    .expect("Rustls config to find cert and key");

    tracing::info!("Initialized config: {:?}", &config);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .expect("axum with rustls serving app to unwrap");
}

extern crate tokio;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf, str::FromStr};
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
    let https_port: u16 = 9073;
    tracing::info!("set https_port to : {}", https_port);

    let app: Router = Router::new()
        .nest_service("/lightwalletd", ServeDir::new("assets/lightwalletd"))
        .nest_service("/zainod", ServeDir::new("assets/zainod"))
        .nest_service("/zcashd", ServeDir::new("assets/zcashd"))
        .nest_service("/zcash-cli", ServeDir::new("assets/zcash-cli"))
        .nest_service("/zebrad", ServeDir::new("assets/zebrad"))
        .nest_service("/zingo-cli", ServeDir::new("assets/zingo-cli"))
        .layer(TraceLayer::new_for_http());
    tracing::info!("set app as : {:?}", &app);

    serve(app, https_port).await;
}

#[instrument(level = Level::TRACE name = "binary_dealer serve function")]
async fn serve(app: Router, https_port: u16) {
    tracing::debug!("serve function launched!");
    let addr = SocketAddr::from(([0, 0, 0, 0], https_port));
    tracing::info!("{} address set!", &addr);

    // configure let's encrypt CA certificate and private key used by https
    let config = RustlsConfig::from_pem_file(
        PathBuf::from_str("/etc/letsencrypt/live/zingoproxy.com/")
            .expect("to find letsencrypt dir")
            .join("fullchain.pem"),
        PathBuf::from_str("/etc/letsencrypt/live/zingoproxy.com/")
            .expect("to find letsencrypt dir")
            .join("privkey.pem"),
    )
    .await
    .expect("Rustls config to find cert and key");

    tracing::info!("Initialized config: {:?}", &config);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .expect("axum with rustls serving app to unwrap");
}

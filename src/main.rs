extern crate tokio;
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    println!("program start!");

    let https_port: u16 = 3953;
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
    .unwrap();

    let app: Router = Router::new().nest_service("/getme", ServeDir::new("assets/getme"));
    let addr = SocketAddr::from(([127, 0, 0, 1], https_port));
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .expect("axum with rustls serving app to unwrap");
}

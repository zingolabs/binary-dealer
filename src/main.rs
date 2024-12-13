extern crate tokio;
use axum::{response::Html, routing::get, serve, Router};

use tokio::{
    net::TcpListener,
    task::JoinSet,
    time::{sleep, Duration},
};

use tower_http::services::ServeFile;

#[tokio::main]
async fn main() {
    println!("program start!");

    // demo - async sanity check
    let mut set: JoinSet<()> = JoinSet::new();
    let the_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    for _v in the_vec {
        set.spawn(async move {
            sleep(Duration::from_millis(2)).await;
        });
    }
    set.join_all().await;
    println!(
        "async sanity check completed, expecting program to continue with running axum server"
    );

    let application = Router::new().route("/", get(handler));

    let listener = TcpListener::bind("127.0.0.1:3333")
        .await
        .expect("listener to bind");
    println!(
        "listening on {}",
        listener
            .local_addr()
            .expect("listener local_addr to unwrap")
    );

    //
    let rt: Router = Router::new().route_service("/assets", ServeFile::new("index.html"));

    //-----------

    serve(listener, application)
        .await
        .expect("axum to launch serve");
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Basic server with html inline functioning!</h1>")
}

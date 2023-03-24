use crate::routes::health::hc;
use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;
use std::net::{IpAddr, SocketAddr};

pub async fn startup() {
    let hc_router = Router::new().route("/", get(hc));

    let app = Router::new().nest("/hc", hc_router);

    let addr = SocketAddr::from(init_addr());

    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch!"))
}

pub fn init_app() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
}

fn init_addr() -> (IpAddr, u16) {
    let env_host = env::var_os("HOST").expect("HOST is undefined.");
    let ip_addr = env_host
        .into_string()
        .expect("HOST is invalid.")
        .parse::<IpAddr>()
        .expect("HOST is invalid.");

    let env_port = env::var_os("PORT").expect("PORT is undefined.");
    let port = env_port
        .into_string()
        .expect("PORT is invalid.")
        .parse::<u16>()
        .expect("PORT is invalid.");

    tracing::debug!("Init ip address.");
    (ip_addr, port)
}
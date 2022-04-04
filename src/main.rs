use std::{net::SocketAddr, str::FromStr};

use axum::{routing::get, AddExtensionLayer, Router};

mod config;
mod distribution;
mod handler;
mod model;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let appstate = config::AppState::new().await.unwrap();

    let app = Router::new()
        .route("/usage", get(handler::usage::usage))
        .route("/count", get(handler::query::count))
        .route("/sds", get(handler::query::sds))
        .route("/unaudited", get(handler::query::unaudited))
        .route("/sds/:code", get(handler::query::order_form))
        .route("/delivery/:code", get(handler::query::delivery))
        .route(
            "/orderform/status/:code",
            get(handler::query::order_form_status),
        )
        .route("/undistributed", get(handler::query::undistributed_codes))
        .route("/pools", get(handler::query::pools))
        .route("/codes", get(handler::query::codes))
        .layer(AddExtensionLayer::new(appstate));
    //.layer(AddExtensionLayer::new(shared_conn_pool))
    //.layer(AddExtensionLayer::new(shared_group));

    let addr = SocketAddr::from_str("0.0.0.0:3000").unwrap();

    tracing::info!("server listening on: {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

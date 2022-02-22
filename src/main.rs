use std::{net::SocketAddr, str::FromStr};

use axum::{routing::get, AddExtensionLayer, Router};

mod handler;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    //let conn_str = "server=tcp:localhost,1433;IntegratedSecurity=true;TrustServerCertificate=true".to_owned();
    let state = model::AppState::new().await.unwrap();

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
        .layer(AddExtensionLayer::new(state));

    let addr = SocketAddr::from_str("0.0.0.0:3000").unwrap();

    tracing::info!("server listening on: {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

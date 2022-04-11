
use axum::extract::{Extension, Path};
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};

use bb8::State;
use serde_json::Value;

use rusqlite::params;

use crate::model::distribution::get_deliverer_by_code;
use crate::model::sale_delivery::OrderForm;
use crate::model::sale_delivery::OrderFormStatus;
use crate::model::sale_delivery::SaleDelivery;
use crate::model::delivery::Deliverer;
use crate::model::Code;
use crate::utils::{diff, voucherdate};

use crate::config::AppState;


// pub async fn check_deliverer(Extension(state): Extension<AppState>, Json(d): Json<Deliverer>) -> impl IntoResponse {
//     let conn = state.sqlite_pool.get().await.unwrap();
//     match conn.execute(
//         "INSERT INTO delivery (name) VALUES (?1)", params![d.name]) {
//             Ok(_) => StatusCode::CREATED,
//             Err(_) =>  StatusCode::FORBIDDEN,
//         };
//     StatusCode::CONFLICT;
//     "hello"
// }

pub async fn hello(Json(d): Json<Deliverer>) -> String {
    format!("Hello, {}", d.name)
}
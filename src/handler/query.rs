use axum::extract::{Extension, Path};
use axum::Json;

use serde_json::Value;

use crate::model::distribution::get_deliverer_by_code;
use crate::model::sale_delivery::OrderForm;
use crate::model::sale_delivery::OrderFormStatus;
use crate::model::sale_delivery::SaleDelivery;
use crate::model::AppState;

pub async fn count(Extension(state): Extension<AppState>) -> Json<Value> {
    let c = SaleDelivery::get_count_by_diff(&state, 0).await.unwrap();
    Json(serde_json::json!({
        "count": c,
    }))
}

pub async fn sds(Extension(state): Extension<AppState>) -> Json<Value> {
    let sds = SaleDelivery::get_sale_deliveries(&state, 0).await.unwrap();
    Json(serde_json::json!(sds))
}

pub async fn unaudited(Extension(state): Extension<AppState>) -> Json<Value> {
    let codes = SaleDelivery::get_unaudited_sds(&state, 0).await.unwrap();
    Json(serde_json::json!(codes))
}

pub async fn order_form(
    Extension(state): Extension<AppState>,
    Path(code): Path<String>,
) -> Json<Value> {
    let of = OrderForm::get_by_code(&state, &code).await.unwrap();
    Json(serde_json::json!(of))
}

pub async fn delivery(Extension(state): Extension<AppState>, Path(code): Path<String>) -> String {
    get_deliverer_by_code(&state, code.as_str())
        .await
        .unwrap_or("未分配".into())
        .to_string()
}

pub async fn order_form_status(
    Extension(state): Extension<AppState>,
    Path(code): Path<String>,
) -> Json<Value> {
    let status = OrderFormStatus::get_status(&state, code.as_str())
        .await
        .unwrap();
    Json(serde_json::json!(status))
}

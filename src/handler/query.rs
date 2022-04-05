use axum::extract::{Extension, Path};
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};

use serde_json::Value;

use crate::model::distribution::get_deliverer_by_code;
use crate::model::sale_delivery::OrderForm;
use crate::model::sale_delivery::OrderFormStatus;
use crate::model::sale_delivery::SaleDelivery;
use crate::model::Code;
use crate::utils::{diff, voucherdate};

use crate::config::AppState;

pub async fn count(Extension(state): Extension<AppState>) -> Json<Value> {
    let c = SaleDelivery::get_count_by_diff(&state.mssql_pool, diff())
        .await
        .unwrap();
    Json(serde_json::json!({
        "voucherdate": voucherdate(),
        "count": c,
    }))
}

pub async fn sds(Extension(state): Extension<AppState>) -> Json<Value> {
    let sds = SaleDelivery::get_sale_deliveries(&state.mssql_pool, 0)
        .await
        .unwrap();
    Json(serde_json::json!(sds))
}

pub async fn unaudited(Extension(state): Extension<AppState>) -> Json<Value> {
    let codes = SaleDelivery::get_unaudited_sds(&state.mssql_pool, diff())
        .await
        .unwrap();
    Json(serde_json::json!(codes))
}

pub async fn order_form(
    Extension(state): Extension<AppState>,
    Path(code): Path<String>,
) -> Json<Value> {
    let of = OrderForm::get_by_code(&state.mssql_pool, &code)
        .await
        .unwrap();
    Json(serde_json::json!(of))
}

pub async fn delivery(Extension(state): Extension<AppState>, Path(code): Path<String>) -> String {
    get_deliverer_by_code(&state.mssql_pool, code.as_str())
        .await
        .unwrap_or(Some("未分配".into()))
        .unwrap_or("未分配".into())
}

pub async fn order_form_status(
    Extension(state): Extension<AppState>,
    Path(code): Path<String>,
) -> Json<Value> {
    let status = OrderFormStatus::get_status(&state.mssql_pool, code.as_str())
        .await
        .unwrap();
    Json(serde_json::json!(status))
}

pub async fn undistributed_codes(Extension(state): Extension<AppState>) -> Json<Value> {
    let codes = SaleDelivery::get_undistributed_codes(&state.mssql_pool)
        .await
        .unwrap();
    Json(serde_json::json!(codes))
}

pub async fn pools_update(Extension(mut state): Extension<AppState>) -> impl IntoResponse {
    let p = &mut state.group;

    //let latest_id = p.read().unwrap().latest_saledelivery_indb_id;
    let latest_id = 0;

    let codes = SaleDelivery::get_sale_deliveries_codes(&state.mssql_pool, diff(), latest_id)
        .await
        .unwrap();

    for code in codes {
        let of = OrderForm::get_by_code(&state.mssql_pool, &code)
            .await
            .unwrap();
        p.write().unwrap().add(of);
    }

    StatusCode::OK
    //Json(serde_json::json!(p.read().unwrap().to_owned()))
}

pub async fn pools(Extension(state): Extension<AppState>) -> Json<Value> {
    Json(serde_json::json!(state.group.read().unwrap().to_owned()))
}

pub async fn pools_reset(Extension(state): Extension<AppState>) -> impl IntoResponse {
    // let mut p = state.group;
    // p.write().unwrap().register_region(vec!["北区", "南区", "西区"]);
    // group.register_warehouse(vec!["七甸仓库", "王大桥33号仓库"]);
    // state.group.write().unwrap().clear();
    StatusCode::OK
}

pub async fn codes(Extension(state): Extension<AppState>) -> Json<Value> {
    let codes = SaleDelivery::get_sale_deliveries_codes(&state.mssql_pool, diff(), 0)
        .await
        .unwrap();
    Json(serde_json::json!(codes))
}

// pub async fn search_customer_sds(Extension(state): Extension<AppState>, Path(customer): Path<String>) -> Json<Value> {
//     let sds = SaleDelivery::get_sale_deliveries(&state, 0).await.unwrap().iter().filter(|f| f.customer == customer).collect::<Vec<_>>();
//     let mut ofs: Vec<OrderForm> = vec![];
//     for sd in sds {
//         match sd.code {
//             Code::SaleDelivery(code) => {
//                 let of = OrderForm::get_by_code(&state, &code).await.unwrap();
//                 &ofs.push(of);
//             }
//         }
//     }
//     Json(serde_json::json!(ofs))
// }

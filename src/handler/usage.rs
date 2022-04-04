use axum::Json;
use serde_json::Value;

pub async fn usage<'a>() -> Json<Value> {
    let data = r#"
        - GET /count -- 获取当日订单数
        - GET /sds -- 获取当日所有订单
        - GET /sds/{code} -- 获取订单
        - GET /unaudited -- 获取当日所有未审核订单
        - GET /delivery/{code} -- 查询订单送货人
        - GET /orderform/status/{code}
        - GET /undistributed -- 获取未生成配货单的销货单单据编号
        x GET /search/sds/{customer_name} -- 查询客户当日订单
        - GET /pools
    "#;
    let data: Vec<&str> = data
        .split('\n')
        .into_iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    Json(serde_json::json!(data))
}

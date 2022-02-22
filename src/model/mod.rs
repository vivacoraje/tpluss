use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use serde::Serialize;
use tiberius::{AuthMethod, Config};

pub mod distribution;
pub mod order_form;
pub mod sale_delivery;
//pub mod state;

#[derive(Clone)]
pub struct AppState {
    pub mssql_pool: Pool<ConnectionManager>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let mut config = Config::new();
        config.host("localhost");
        config.port(1433);
        config.authentication(AuthMethod::sql_server("SA", "kmvh5107288@"));
        config.database("UFTData878266_000001");
        config.trust_cert();

        let mgr = ConnectionManager::new(config);
        let mssql_pool = Pool::builder().max_size(3).build(mgr).await?;

        Ok(Self { mssql_pool })
    }
}

#[derive(Serialize)]
pub struct Warehouse(String);

#[derive(Serialize)]
#[serde(untagged)]
pub enum Code {
    SaleDelivery(String),
}

pub enum SaleOut {
    Out,    // 15, 已出库
    NouOut, //  305,
}

#[derive(Serialize)]
pub enum Executor {
    Spawn(String),
    WarehouseDep(String),
    Deliverer(String),
}

#[derive(Serialize)]
pub enum OrderFormStatus {
    Created(Executor),     // 已生单
    Distributed(Executor), // 已分配
    Delivery(Executor),    // 已送出
    Finished,              // 已完成
}

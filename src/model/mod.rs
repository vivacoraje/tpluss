use serde::Serialize;

pub mod distribution;
pub mod order_form;
pub mod sale_delivery;
pub mod delivery;

//pub mod state;

#[derive(Debug, Serialize, Hash, PartialEq, Eq, Clone)]
pub struct Warehouse(pub String);

#[derive(Debug, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct Region(pub String);

impl Default for Region {
    fn default() -> Self {
        Self("无".into())
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Code {
    SaleDelivery(String),
}

pub enum SaleOut {
    Out,    // 15, 已出库
    NouOut, //  305,
}

// SA
// 181 未审核
// 189 审核

#[derive(Serialize)]
pub enum Executor {
    Spawn(String),
    WarehouseDep(String),
    Deliverer(String),
}

#[derive(Serialize)]
pub enum FormStatus {
    Created(Executor),     // 已生单
    Distributed(Executor), // 已分配
    Delivery(Executor),    // 已送出
    Finished,              // 已完成
}

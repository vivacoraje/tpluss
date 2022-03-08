use serde::Serialize;
use tiberius::{numeric::Decimal, time::chrono::NaiveDateTime, Row};

use super::AppState;
use super::Code;
use super::Warehouse;
use crate::utils::code_prefix;

#[derive(Serialize)]
pub struct SaleDeliveryB {
    id: i32,
    id_sale_delivery_dto: i32,
    inventory: String,
    quantity: Decimal,
    composition_quantity: String,
    unit_exchange_rate: Decimal,
}

impl SaleDeliveryB {
    fn from_row(r: &Row) -> Self {
        Self {
            id: r.get::<i32, _>(0).unwrap(),
            id_sale_delivery_dto: r.get(1).unwrap(),
            inventory: r.get::<&str, _>(2).unwrap().into(),
            quantity: r.get::<Decimal, _>(3).unwrap().round_dp(3),
            composition_quantity: r.get::<&str, _>(4).unwrap().into(),
            unit_exchange_rate: r.get::<Decimal, _>(5).unwrap().round_dp(0),
        }
    }

    pub async fn get_items_by_id(
        state: &AppState,
        sale_delivery_id: i32,
    ) -> anyhow::Result<Vec<SaleDeliveryB>> {
        let sql = r#"
            SELECT 
                sb.id, idSaleDeliveryDTO, iv.name, quantity, compositionQuantity, unitExchangeRate 
            FROM 
                SA_SaleDelivery_b AS sb 
                JOIN AA_Inventory AS iv ON sb.idinventory=iv.id 
            WHERE idSaleDeliveryDTO=(@P1)
        "#;

        let mut pool = state.mssql_pool.get().await?;

        let items = pool
            .query(sql, &[&sale_delivery_id])
            .await?
            .into_first_result()
            .await?
            .iter()
            .map(|r| Self::from_row(r))
            .collect::<Vec<SaleDeliveryB>>();

        Ok(items)
    }
}

#[derive(Serialize)]
pub struct SaleDelivery {
    id: i32,
    pub code: Code,
    pub customer: String,
    phone: Option<String>,
    warehouse: Warehouse,
    is_saleout: i32,
    is_cancel: i32,
    voucher_state: i32,
    memo: String,
    amount: Decimal,
    maker: String,
    print_count: i32,
    voucher_date: NaiveDateTime,
    created_time: NaiveDateTime,
    updated_time: NaiveDateTime,
    clerk: Option<String>,
}

impl SaleDelivery {
    fn from_row(r: &Row) -> Self {
        Self {
            id: r.get::<i32, _>(0).unwrap(),
            code: Code::SaleDelivery(r.get::<&str, _>(1).unwrap().into()),
            customer: r.get::<&str, _>(2).unwrap().into(),
            phone: r.get::<&str, _>(3).and_then(|p| Some(String::from(p))),
            warehouse: Warehouse(r.get::<&str, _>(4).unwrap().into()),
            voucher_state: r.get::<i32, _>(5).unwrap(),
            is_saleout: r.get::<i32, _>(6).unwrap(),
            is_cancel: r.get::<i32, _>(7).unwrap(),
            maker: r.get::<&str, _>(8).unwrap().into(),
            amount: r.get::<Decimal, _>(9).unwrap().round_dp(3),
            memo: r.get::<&str, _>(10).unwrap().into(),
            print_count: r.get::<i32, _>(11).unwrap(),
            voucher_date: r.get::<NaiveDateTime, _>(12).unwrap(),
            created_time: r.get::<NaiveDateTime, _>(13).unwrap(),
            updated_time: r.get::<NaiveDateTime, _>(14).unwrap(),
            clerk: r.get::<&str, _>(15).and_then(|p| Some(String::from(p))),
        }
    }

    pub async fn get_count_by_diff(state: &AppState, diff: i32) -> anyhow::Result<i32> {
        let mut conn = state.mssql_pool.get().await?;
        let sql_str = r#"
            SELECT 
                COUNT(*) AS count
            FROM 
                SA_SaleDelivery 
            WHERE 
                DateDiff(dd,voucherdate, getdate())=(@P1) 
                AND idbusinesstype = 65"#;
                //AND (idwarehouse = 36 OR idwarehouse = 6)"#;
        let count = conn
            .query(sql_str, &[&diff])
            .await?
            .into_row()
            .await?
            .unwrap()
            .get::<i32, _>("count")
            .unwrap();
        Ok(count)
    }

    pub async fn get_sale_deliveries(state: &AppState, diff: i32) -> anyhow::Result<Vec<Self>> {
        let mut conn = state.mssql_pool.get().await?;

        let sql = r#"
            SELECT 
                sd.id, sd.code, 
                p.name AS customer, sd.CustomerPhone AS phone, 
                wh.name AS warehouse,  
                sd.voucherState, sd.isSaleOut, sd.isCancel, 
                sd.maker,           
                sd.amount, sd.memo, sd.PrintCount, 
                sd.voucherdate, sd.createdtime, sd.updated,
                ap.name AS clerk
            FROM 
                (((SA_SaleDelivery AS sd JOIN AA_Warehouse AS wh ON sd.idwarehouse=wh.id) 
                JOIN AA_Partner AS p ON sd.idcustomer=p.id) JOIN AA_Person AS ap ON sd.idclerk = ap.id)
            WHERE DateDiff(dd, voucherdate, getdate())=(@P1) AND sd.idbusinesstype=65
        "#;

        let r = conn
            .query(sql, &[&diff])
            .await?
            .into_first_result()
            .await?
            .iter()
            .map(|r| Self::from_row(r))
            .collect::<Vec<Self>>();

        Ok(r)
    }

    pub async fn get_unaudited_sds(
        state: &AppState,
        diff: i32,
    ) -> anyhow::Result<Vec<super::Code>> {
        let sql = r#"
            SELECT code
            FROM SA_SaleDelivery
            WHERE DateDiff(dd,voucherdate ,getdate())=(@P1) AND voucherState=181
        "#;
        let mut pool = state.mssql_pool.get().await?;

        let codes = pool
            .query(sql, &[&diff])
            .await?
            .into_first_result()
            .await?
            .iter()
            .map(|r| Code::SaleDelivery(r.get::<&str, _>(0).unwrap().into()))
            .collect::<Vec<Code>>();
        Ok(codes)
    }

    pub async fn get_sale_delivery_by_code(
        state: &AppState,
        code: &str,
    ) -> anyhow::Result<SaleDelivery> {
        let sql = r#"
            SELECT 
                sd.id, sd.code, 
                p.name AS customer, sd.CustomerPhone, 
                wh.name AS warehouse,  
                sd.voucherState, sd.isSaleOut, sd.isCancel, 
                sd.maker, 
                sd.amount, sd.memo, sd.PrintCount, 
                sd.voucherdate, sd.createdtime, sd.updated
            FROM 
                ((SA_SaleDelivery AS sd JOIN AA_Warehouse AS wh ON sd.idwarehouse=wh.id) 
                JOIN AA_Partner AS p ON sd.idcustomer=p.id) 
            WHERE sd.code=(@P1) 
        "#;

        let mut conn = state.mssql_pool.get().await?;
        let code = code.to_string();
        let r = conn.query(sql, &[&code]).await?.into_row().await?.unwrap();

        Ok(Self::from_row(&r))
    }

    pub async fn get_undistributed_codes(state: &AppState) -> anyhow::Result<Vec<String>> {
        let sql = r#"
            SELECT code 
            FROM SA_SaleDelivery 
            WHERE (code like (@P1)) AND
                idbusinesstype=65 AND
                code NOT IN (select sourcevouchercode 
                    FROM DI_Distribution_b 
                    WHERE sourcevouchercode like (@P2) GROUP BY sourcevouchercode);"#;

        let mut conn = state.mssql_pool.get().await?;

        let code = code_prefix();

        let r = conn
            .query(sql, &[&code, &code])
            .await?
            .into_first_result()
            .await?
            .iter()
            .map(|f| f.get::<&str, _>(0).unwrap().to_string())
            .collect::<Vec<String>>();

        Ok(r)
    }
}

#[derive(Serialize)]
pub struct OrderForm {
    #[serde(flatten)]
    sd: SaleDelivery,
    inventories: Vec<SaleDeliveryB>,
}

impl OrderForm {
    pub async fn get_by_code(state: &AppState, code: &str) -> anyhow::Result<OrderForm> {
        let sd = SaleDelivery::get_sale_delivery_by_code(state, code).await?;
        let inventories = SaleDeliveryB::get_items_by_id(state, sd.id).await?;
        Ok(Self { sd, inventories })
    }
}

#[derive(Serialize)]
pub struct OrderFormStatus {
    code: String,
    customer: String,
    status: super::OrderFormStatus,
}

impl OrderFormStatus {
    pub async fn get_status(state: &AppState, code: &str) -> anyhow::Result<OrderFormStatus> {
        let delivery = super::distribution::get_deliverer_by_code(state, code).await?;
        let sd = SaleDelivery::get_sale_delivery_by_code(state, code).await?;

        let status = if delivery == None {
            super::OrderFormStatus::Created(super::Executor::Spawn(sd.maker.clone()))
        } else {
            super::OrderFormStatus::Distributed(super::Executor::Deliverer(delivery.unwrap()))
        };

        Ok(Self {
            code: code.to_string(),
            customer: sd.customer,
            status,
        })
    }
}

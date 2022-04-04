use std::sync::{Arc, RwLock};

use anyhow::Ok;
use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::{AuthMethod, Config};

use crate::distribution::Group;

pub type PoolConnectionManager = Pool<ConnectionManager>;

#[derive(Clone)]
pub struct AppState {
    pub mssql_pool: PoolConnectionManager,
    pub group: Arc<RwLock<Group>>,
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

        let mut group = Group::new();
        group.register_region(vec!["北区", "南区", "西区"]);
        group.register_warehouse(vec!["七甸仓库", "王大桥33号仓库"]);

        let group = Arc::new(RwLock::new(group));

        Ok(Self { mssql_pool, group })
    }
}

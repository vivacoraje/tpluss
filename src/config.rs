
pub struct WebConfig {
    pub addr: String,
}

pub struct Config {
    pub web: WebCofnig,
    pub mssql: (),
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
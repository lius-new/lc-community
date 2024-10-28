use config::{Config, ConfigError};
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref AppCon: AppConfig = AppConfig::new("./config/default").unwrap();
}

#[derive(Debug, Deserialize)]
pub struct Service {
    /// 应用名称
    pub name: String,
    /// 应用运行端口
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    /// 数据库地址
    pub url: String,
    /// 数据库最大连接数
    pub max_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct Upload {
    /// 数据库地址
    pub article_covers: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AppConfig {
    /// 应用配置
    pub service: Service,
    // 应用数据库配置
    pub database: Database,
    // 应用上传配置
    pub upload: Upload,
}

impl AppConfig {
    /// 构建AppConfig实例
    pub fn new(config_name: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(config_name))
            .add_source(config::File::with_name("./config/production").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        config.try_deserialize()
    }
}

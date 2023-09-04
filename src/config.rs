use config::{self, FileFormat};
use serde::Deserialize;
use serde_json;
use std::fs::{self, read_to_string};
use std::net::{Ipv4Addr, TcpListener};
#[derive(Deserialize, Debug)]
pub struct Settings {
    pub pg_config: PgSettings,
    pub bind: Ipv4Addr,
    pub port: u32,
}

#[derive(Deserialize, Debug)]
pub struct PgSettings {
    pub username: String,
    pub password: String,
    pub port: String,
    pub host: String,
    pub database_name: String,
}

impl Settings {
    pub fn get_database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.pg_config.username,
            self.pg_config.password,
            self.pg_config.host,
            self.pg_config.port,
            self.pg_config.database_name
        )
    }

    pub fn get_tcp_address(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }
}

pub fn get_config<'a>(path: &'a str) -> Result<Settings, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::with_name(path))
        .build()?
        .try_deserialize::<Settings>()
}

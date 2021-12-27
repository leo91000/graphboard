pub use ::config::ConfigError;
use ::serde::Deserialize;
use postgres_protocol::escape::escape_identifier;
use std::{env, lazy::SyncLazy};

#[derive(Deserialize)]
pub struct Config {
  pub port: String,
  pub addr: String,
  pub pg: deadpool_postgres::Config,
}

impl Config {
  pub fn from_env() -> Result<Self, ConfigError> {
    let mut cfg = ::config::Config::new();
    cfg
      .set_default("port", 3000)?
      .set_default("port", 80)?
      .set_default("addr", "0.0.0.0")?
      .set_default("pg.pool.max_size", 16)?
      .merge(::config::Environment::new().separator("_"))?;
    cfg.try_into()
  }

  pub fn server_addr(&self) -> String {
    let mut server_addr = String::new();
    server_addr.push_str(self.addr.as_str());
    server_addr.push_str(":");
    server_addr.push_str(self.port.as_str());
    server_addr
  }
}

pub static GRAPHILE_WORKER_SCHEMA: SyncLazy<String> = SyncLazy::new(|| {
  escape_identifier(
    env::var("GRAPHILE_WORKER_SCHEMA")
      .unwrap_or("graphile_worker".to_string())
      .as_str(),
  )
});

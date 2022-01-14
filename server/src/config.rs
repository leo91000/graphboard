pub use ::config::ConfigError;
use ::serde::Deserialize;
use postgres_protocol::escape::escape_identifier;
use std::lazy::SyncLazy;

#[derive(Deserialize)]
pub struct Config {
  pub port: String,
  pub host: String,
  pub graphile_worker_schema: String,
  pub pg: deadpool_postgres::Config,
}

impl Config {
  pub fn from_env() -> Result<Self, ConfigError> {
    let mut cfg = ::config::Config::new();
    cfg
      .set_default("port", 80)?
      .set_default("host", "0.0.0.0")?
      .set_default("graphile_worker_schema", String::from("graphile_worker"))?
      .set_default("pg.pool.max_size", 16)?
      .merge(::config::Environment::new().separator("_"))?;
    cfg.set(
      "graphile_worker_schema",
      escape_identifier(cfg.get::<String>("graphile_worker_schema")?.as_str()),
    )?;
    cfg.try_into()
  }

  pub fn server_addr(&self) -> String {
    format!("{host}:{port}", host = self.host, port = self.port)
  }
}

pub static CONFIG: SyncLazy<Config> =
  SyncLazy::new(|| Config::from_env().expect("Invalid configuration : check for missing env"));

use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn init_telemetry() {
  let app_name = "graphboard_server";

  // Filter based on level - trace, debug, info, warn, error
  // Tunable via `RUST_LOG` env variable
  let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
  // Create a `tracing` layer to emit spans as structured logs to stdout
  let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
  // Combined them all together in a `tracing` subscriber
  let subscriber = Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(formatting_layer);

  tracing::subscriber::set_global_default(subscriber)
    .expect("Failed to install `tracing` subscriber.")
}

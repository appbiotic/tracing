#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

use std::sync::OnceLock;

use anyhow::Context;
use appbiotic_tracing_config::prost_serde::{Config, Preset};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry,
};

pub mod error;

use error::Result;

#[cfg(feature = "uniffi")]
#[uniffi::export]
pub fn appbiotic_tracing_init(config: Config) -> Result<()> {
    init(config)
}

pub fn init(config: Config) -> Result<()> {
    static CONFIG: OnceLock<Config> = OnceLock::new();

    CONFIG.get_or_init(|| {
        let registry = tracing_subscriber::registry();

        let mut layers: Vec<Box<dyn Layer<Registry> + Send + Sync>> = Vec::new();

        let env_filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env()
            .context("Failed to create tracing env filter")
            .unwrap();

        match &config.preset() {
            Preset::PlainText => layers.push(
                fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_writer(std::io::stdout)
                    .with_filter(env_filter)
                    .boxed(),
            ),
            Preset::Json => {
                layers.push(
                    tracing_subscriber::fmt::layer()
                        .json()
                        .with_file(true)
                        .with_line_number(true)
                        .with_writer(std::io::stdout)
                        .with_filter(env_filter)
                        .boxed(),
                );
            }
            _ => {
                panic!("Unknown logging preset");
            }
        }

        registry
            .with(layers)
            .try_init()
            .context("Failed to initialize tracing")
            .unwrap();

        info!("Tracing initialized");

        config
    });

    Ok(())
}

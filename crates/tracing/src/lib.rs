use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry,
};

pub enum TracingConfig {
    Preset(TracingPreset),
}

#[derive(Clone, Debug, strum::EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TracingPreset {
    PlainText,
    Json,
}

pub fn init(config: TracingConfig) -> Result<(), String> {
    let registry = tracing_subscriber::registry();

    let mut layers: Vec<Box<dyn Layer<Registry> + Send + Sync>> = Vec::new();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .map_err(|error| error.to_string())?;

    match config {
        TracingConfig::Preset(preset) => match preset {
            TracingPreset::PlainText => layers.push(
                fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_writer(std::io::stdout)
                    .with_filter(env_filter)
                    .boxed(),
            ),
            TracingPreset::Json => {
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
        },
    }

    registry
        .with(layers)
        .try_init()
        .map_err(|error| error.to_string())?;

    info!("Tracing initialized");

    Ok(())
}

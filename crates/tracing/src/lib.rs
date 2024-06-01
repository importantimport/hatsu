use hatsu_utils::AppError;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    fmt::Layer,
    layer::{Layered, SubscriberExt},
    util::SubscriberInitExt,
    EnvFilter,
    Registry,
};

pub fn init() -> Result<(), AppError> {
    let registry = tracing_subscriber::registry()
        .with(filter_layer())
        .with(fmt_layer())
        .with(tracing_error::ErrorLayer::default());

    #[cfg(feature = "console")]
    let registry = registry.with(console_subscriber::spawn());

    registry.init();

    Ok(())
}

fn filter_layer() -> EnvFilter {
    EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("HATSU_LOG")
        .from_env_lossy()
}

fn fmt_layer() -> Layer<Layered<EnvFilter, Registry>> {
    let fmt_layer = tracing_subscriber::fmt::layer();

    #[cfg(feature = "json")]
    let fmt_layer = fmt_layer.json();

    fmt_layer
}

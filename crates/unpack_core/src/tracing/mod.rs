use tracing::level_filters::LevelFilter;
use tracing_chrome::{ChromeLayerBuilder, FlushGuard};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_subscriber::{EnvFilter, Layer};
pub fn init_tracing() -> Option<FlushGuard> {
    let env = std::env::var("UNPACK_LOG").unwrap_or("".to_string());
    if env == "" {
        return None;
    }
    let (chrome_layer, guard) = ChromeLayerBuilder::new()
        .trace_style(tracing_chrome::TraceStyle::Async)
        .build();
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse(env)
        .expect("create env filter failed");

    tracing_subscriber::registry()
        .with(chrome_layer.with_filter(env_filter))
        .init();
    Some(guard)
}

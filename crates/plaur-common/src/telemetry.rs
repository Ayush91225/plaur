use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_telemetry() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_file(true)
        )
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "plaur=debug,tower_http=debug".into())
        )
        .init();
    
    Ok(())
}
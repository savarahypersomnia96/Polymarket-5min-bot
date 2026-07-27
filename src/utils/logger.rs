use anyhow::Result;
use std::fs::File;
use std::io::Write;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use text_pad_core::format_line;

pub fn init_logger(quiet_stdout: bool) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let header = format_line(&format!("polypulse v{}", env!("CARGO_PKG_VERSION")));

    if quiet_stdout {
        let path = std::env::var("LOG_FILE").unwrap_or_else(|_| "bot.log".to_string());
        let mut file = File::create(&path)?;
        file.write_all(header.as_bytes())?;
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else if let Ok(path) = std::env::var("LOG_FILE") {
        let mut file = File::create(path)?;
        file.write_all(header.as_bytes())?;
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(file)
                    .with_ansi(false),
            )
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .init();
    }

    Ok(())
}

pub fn tui_enabled_from_env() -> bool {
    if std::env::var("PLAIN_LOGS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return false;
    }
    !std::env::var("TUI_ENABLED")
        .map(|v| v == "0" || v.eq_ignore_ascii_case("false"))
        .unwrap_or(false)
}

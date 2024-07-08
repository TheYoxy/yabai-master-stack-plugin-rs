use color_eyre::eyre::{eyre, Result};
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

/// Initialize the logging system.
pub(crate) fn initialize_logging() -> Result<()> {
  let home = homedir::my_home()?.ok_or(eyre!("Failed to get home directory"))?;
  let filename = format!("{}.log", env!("CARGO_PKG_NAME"));
  let log_dir = home.join(".local").join("share").join(env!("CARGO_CRATE_NAME")).join("logs").join(filename);
  let parent = log_dir.parent().ok_or(eyre!("unable to create logging parent directory"))?;
  if !std::fs::exists(parent)? {
    std::fs::create_dir_all(parent)?;
  }
  let log_file = std::fs::File::create(log_dir)?;
  unsafe {
    std::env::set_var(
      "RUST_LOG",
      std::env::var("RUST_LOG").unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
    );
  }
  let file_subscriber = tracing_subscriber::fmt::layer()
    .with_file(true)
    .with_line_number(true)
    .with_writer(log_file)
    .with_target(false)
    .with_ansi(false)
    .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());
  tracing_subscriber::registry().with(file_subscriber).with(ErrorLayer::default()).try_init()?;
  Ok(())
}

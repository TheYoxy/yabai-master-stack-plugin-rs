use color_eyre::{eyre::bail, owo_colors::OwoColorize};
use log::{trace, warn};

use crate::{
  dry_mode::is_dry_mode,
  task::lock::is_locked,
  window_manager::WindowsManager,
  yabai::command::{message::YabaiMessage, to_command::Runnable},
};

impl WindowsManager {
  pub(crate) fn send_yabai_message(&self, message: YabaiMessage) -> color_eyre::Result<()> {
    match is_locked() {
      Ok(false) => {
        if message.is_write() && is_dry_mode() {
          warn!("Skipping {} as dry mode is enabled", message.blue());
        } else {
          trace!("Running yabai command: {}", message.blue());
          message.run()?;
        }
        Ok(())
      },
      Ok(true) => {
        bail!("Lockfile is already owned by another process");
      },
      Err(e) => {
        bail!("Could not check if lockfile exists: {}", e);
      },
    }
  }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod tests {
  use super::*;

  #[test]
  fn test_get_yabai_config() {
    let config = YabaiMessage::config().left_padding().unwrap();
    assert_eq!(config, 10.0);
  }
}

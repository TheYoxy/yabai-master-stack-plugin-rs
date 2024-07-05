use log::{debug, info};

use crate::{
  window_manager::{yabai::YabaiCommand, WindowsManager},
  yabai::window::{SplitType, Window},
};

type Result<T> = color_eyre::Result<T>;
impl WindowsManager {
  fn columnize_windows(&self, windows: Vec<Window>, split_type: SplitType) -> Result<()> {
    debug!("Columnizing {len} windows with split: {split_type:?}", len = windows.len());
    for window in windows {
      let window = self.get_updated_window_data(&window);
      if let Some(window) = window {
        if window.split_type == split_type {
          self.run_yabai_command(YabaiCommand::ToggleWindowSplit(window))?;
        }
      }
    }

    Ok(())
  }

  pub(crate) fn columnize_master_windows(&self) -> Result<()> {
    debug!("Columnizing master windows");
    let master_windows = self.get_master_windows()?;
    self.columnize_windows(master_windows, SplitType::Vertical)?;
    Ok(())
  }

  pub(crate) fn columnize_stack_windows(&self) -> Result<()> {
    debug!("Columnizing stack windows");
    if self.expected_current_num_master_windows == self.windows.len() {
      info!("Skipped colonizing stack windows bc there is no stack");
      return Ok(());
    }

    let stack_windows = self.get_stack_windows();
    self.columnize_windows(stack_windows, SplitType::Horizontal)?;

    Ok(())
  }
}

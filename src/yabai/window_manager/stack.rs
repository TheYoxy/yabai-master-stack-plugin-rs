use log::{debug, info};

use crate::{
  print_bool,
  yabai::{
    window::SplitType,
    window_manager::{yabai::YabaiCommand, WindowsManager},
  },
};

type Result<T> = color_eyre::Result<T>;

impl WindowsManager {
  pub(super) fn does_stack_exists(&self) -> bool {
    debug!("Checking if stack exists");
    let top_right_window = self.get_top_right_window();
    if let Some(top_right_window) = top_right_window {
      let result = top_right_window.frame.x != 0f64;
      debug!("Stack exists: {}", print_bool!(result, "yes", "no"));
      result
    } else {
      debug!("Stack does not exist");
      false
    }
  }

  pub(super) fn create_stack(&self) -> Result<()> {
    info!("Creating stack... ");
    for window in &self.windows.clone() {
      if window.split_type == SplitType::Horizontal {
        self.run_yabai_command(YabaiCommand::ToggleWindowSplit(window))?;
      };
    }

    self.columnize_stack_windows()?;
    self.columnize_master_windows()?;

    Ok(())
  }
}

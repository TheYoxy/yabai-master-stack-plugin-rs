use log::{debug, info};

use crate::{
  window_manager::{yabai::YabaiCommand, WindowsManager},
  yabai::{
    config::get_config,
    window::{SplitType, Window},
  },
};

impl WindowsManager {
  pub(crate) fn move_window_to_stack(&self, window: &Window) -> color_eyre::Result<()> {
    if self.expected_current_num_master_windows == self.windows.len() {
      info!("Skipped moving window {window} to stack because there is no stack.");
      Ok(())
    } else {
      info!("Moving window {window} to stack");
      let config = get_config()?;
      self.run_yabai_command(YabaiCommand::WarpDirection(window, &config.master_position))?;

      self.columnize_stack_windows()?;
      if self.windows.len() == 2 && window.split_type == SplitType::Horizontal {
        self.run_yabai_command(YabaiCommand::ToggleWindowSplit(window))?;
        info!("Splitting window {window} bc 2 windows and horizontal split type");
        return Ok(());
      }

      if self.is_stack_window(window).is_ok_and(|r| r) {
        debug!("Window {window} is already a stack window. Skipping...");
        return Ok(());
      }

      let stack_window = self.get_widest_stack_window();
      if let Some(stack_window) = stack_window {
        if stack_window.id == window.id {
          debug!("Window {window} is already the widest stack window. Skipping...");
          return Ok(());
        }

        self.run_yabai_command(YabaiCommand::WarpWindow(window, &stack_window))?;
        let window = self.get_updated_window_data(window);
        if let Some(window) = window {
          if self.windows.len() == 2 && window.split_type == SplitType::Horizontal {
            info!("Splitting window {window} bc 2 windows and horizontal split type");
            self.run_yabai_command(YabaiCommand::ToggleWindowSplit(window))?;
          } else if window.split_type == SplitType::Vertical {
            self.run_yabai_command(YabaiCommand::ToggleWindowSplit(window))?;
          }
        }
      } else {
        debug!("Unable to find widest stack window");
      }

      Ok(())
    }
  }
}

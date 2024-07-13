use log::{debug, info};

use crate::{
  window_manager::WindowsManager,
  yabai::{
    command::{message::YabaiMessage, toggle_selector::YabaiToggleSelector},
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
      let message = YabaiMessage::window(window).warp(config.master_position)?;
      self.send_yabai_message(message)?;

      self.columnize_stack_windows()?;
      if self.windows.len() == 2 && window.split_type == SplitType::Horizontal {
        let message = YabaiMessage::window(window).toggle(YabaiToggleSelector::Split)?;
        self.send_yabai_message(message)?;
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

        let message = YabaiMessage::window(window).warp(stack_window)?;
        self.send_yabai_message(message)?;
        let window = self.get_updated_window_data(window);
        if let Some(window) = window {
          if self.windows.len() == 2 && window.split_type == SplitType::Horizontal {
            info!("Splitting window {window} bc 2 windows and horizontal split type");
            let message = YabaiMessage::window(window).toggle(YabaiToggleSelector::Split)?;
            self.send_yabai_message(message)?;
          } else if window.split_type == SplitType::Vertical {
            info!("Splitting window {window} bc vertical split type");
            let message = YabaiMessage::window(window).toggle(YabaiToggleSelector::Split)?;
            self.send_yabai_message(message)?;
          }
        }
      } else {
        debug!("Unable to find widest stack window");
      }

      Ok(())
    }
  }
}

use log::debug;

use crate::{
  window_manager::{yabai::YabaiCommand, WindowsManager},
  yabai::{
    config::{get_config, MasterPosition},
    window::{SplitType, Window},
  },
};

type Result<T> = color_eyre::Result<T>;
impl WindowsManager {
  pub(crate) fn get_master_windows(&self) -> Result<Vec<Window>> {
    debug!("Looking for master windows");
    let config = get_config()?;
    let result = match config.master_position {
      MasterPosition::Left => self.get_left_window(),
      MasterPosition::Right => {
        let dividing_line_x_coordinate = self.get_dividing_line_x_coordinate()?;
        self.windows.clone().into_iter().filter(|window| window.frame.x >= dividing_line_x_coordinate).collect()
      },
    };

    Ok(result)
  }

  pub(crate) fn get_top_master_window(&self) -> Result<Option<Window>> {
    Ok(self.get_top_window(self.get_master_windows()?))
  }

  pub(crate) fn get_bottom_master_window(&self) -> Result<Option<Window>> {
    Ok(self.get_bottom_window(self.get_master_windows()?))
  }

  pub(crate) fn get_master_window(&self) -> Result<Option<Window>> {
    let master_windows = self.get_master_windows()?;
    Ok(master_windows.first().cloned())
  }

  pub(crate) fn is_master_window(&self, window: &Window) -> Result<bool> {
    let config = get_config()?;
    match config.master_position {
      MasterPosition::Left => self.is_windows_touching_left_edge(window),
      MasterPosition::Right => {
        let dividing_line_x_coordinate = self.get_dividing_line_x_coordinate()?;
        Ok(window.frame.x >= dividing_line_x_coordinate)
      },
    }
  }

  pub(crate) fn move_window_to_master(&self, window: &Window) -> Result<()> {
    debug!("Moving window {window} to master.");
    if self.expected_current_num_master_windows < self.windows.len() {
      let config = get_config()?;

      self.run_yabai_command(YabaiCommand::WarpDirection(window, &config.master_position))?;
    }

    if !self.is_master_window(window).is_ok_and(|r| r) {
      let master_window = self.get_widest_master_window()?;
      if let Some(master_window) = master_window {
        if master_window.id == window.id {
          return Ok(());
        }

        self.run_yabai_command(YabaiCommand::WarpWindow(window, &master_window))?;

        let window = self.get_updated_window_data(window);
        if let Some(window) = window {
          if window.split_type == SplitType::Vertical {
            self.run_yabai_command(YabaiCommand::ToggleWindowSplit(window))?;
          }
        }

        Ok(())
      } else {
        debug!("Couldn't find widest master window");
        Ok(())
      }
    } else {
      debug!("Window {window} is already in master.");
      Ok(())
    }
  }
}

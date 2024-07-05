use color_eyre::{eyre::eyre, owo_colors::OwoColorize};
use log::{debug, error, info, trace};

use crate::{
  window_manager::{layout_visibility::LayoutValidity, WindowsManager},
  yabai::config::{get_config, MasterPosition},
};

type Result<T> = color_eyre::Result<T>;

impl WindowsManager {
  pub(crate) fn is_valid_layout(&self, target_num_master_windows: Option<usize>) -> Result<LayoutValidity> {
    info!("Starting valid layout check...");
    if self.windows.is_empty() {
      info!("Layout is valid");
      return Ok(LayoutValidity::Valid);
    }
    let target_num_master_windows = target_num_master_windows.unwrap_or(self.expected_current_num_master_windows);
    debug!("Target number of master windows: {}", target_num_master_windows.blue());

    if target_num_master_windows > self.windows.len()
      && !self.windows.iter().all(|window| {
        self
          .is_windows_touching_left_edge(window)
          .inspect_err(|err| error!("Error while checking if windows touch: {err}"))
          .is_ok_and(|r| r)
      })
    {
      let reason = "The number of master windows is greater than the number of windows and not all windows are touching the left edge.".to_string();
      error!("Layout invalid: {}", reason.red());

      Ok(LayoutValidity::Invalid(reason))
    } else {
      let cur_num_master_windows = self.get_master_windows()?.len();
      if cur_num_master_windows != target_num_master_windows {
        let reason = format!("Number of master windows does not equal expected number of master windows ({cur_num_master_windows}/{target_num_master_windows})");
        error!("Layout invalid: {}", reason.red());
        return Ok(LayoutValidity::Invalid(reason));
      }

      for window in &self.windows.clone() {
        if self.is_middle_window(window) {
          let reason = format!("A middle window ({window}) was detected.",);
          error!("Layout invalid: {}", reason.red());
          return Ok(LayoutValidity::Invalid(reason));
        }
      }

      info!("Layout is valid");
      Ok(LayoutValidity::Valid)
    }
  }

  pub(crate) fn get_dividing_line_x_coordinate(&self) -> Result<f64> {
    trace!("get_dividing_line_x_coordinate() called.");
    let config = get_config()?;
    trace!("Master position: {master_position:?}", master_position = config.master_position);
    match config.master_position {
      MasterPosition::Left => {
        let top_left_window =
          self.get_top_left_window().ok_or(eyre!("get_dividing_line_x_coordinate: top_left_window is None"))?;
        debug!("Top left windows: {top_left_window}");

        let mut non_master_windows = self
          .windows
          .iter()
          .filter(|&window| {
            !self
              .is_windows_touching_left_edge(window)
              .inspect_err(|err| error!("Error while checking if windows touch: {err}"))
              .is_ok_and(|is_touching| is_touching)
          })
          .collect::<Vec<_>>();
        non_master_windows.sort_by(|window1, window2| window1.frame.x.total_cmp(&window2.frame.x));
        if non_master_windows.is_empty() {
          return Ok(self.display.frame.x);
        }
        let num_master_windows = self.windows.len() - non_master_windows.len();
        if num_master_windows >= self.expected_current_num_master_windows {
          let window =
            non_master_windows.first().ok_or(eyre!("get_dividing_line_x_coordinate: unable to find first()"))?;
          trace!("get_dividing_line_x_coordinate: {x}", x = window.frame.x);
          return Ok(window.frame.x);
        }

        let max = non_master_windows.len() - 1;
        for i in 0..max {
          let curr_window =
            non_master_windows.get(i).ok_or(eyre!("get_dividing_line_x_coordinate: curr_window is None"))?;
          let next_window =
            non_master_windows.get(i + 1).ok_or(eyre!("get_dividing_line_x_coordinate: next_window is None"))?;
          if curr_window.frame.x == next_window.frame.x
            && num_master_windows + i + 2 >= self.expected_current_num_master_windows
          {
            trace!("get_dividing_line_x_coordinate: {x}", x = curr_window.frame.x);
            return Ok(curr_window.frame.x);
          }
        }

        let window =
          non_master_windows.first().ok_or(eyre!("get_dividing_line_x_coordinate: unable to find first()"))?;

        trace!("get_dividing_line_x_coordinate: window.frame.x: {x}", x = window.frame.x);
        Ok(window.frame.x)
      },
      MasterPosition::Right => {
        let top_right_window = self.get_top_right_window();
        let top_right_window =
          top_right_window.ok_or(eyre!("get_dividing_line_x_coordinate: top_right_window is None"))?;

        debug!("Top right windows: {top_right_window}");

        if self.expected_current_num_master_windows == 1 {
          return Ok(top_right_window.frame.x);
        }

        let non_stack_windows = self
          .windows
          .iter()
          .filter(|&window| !self.is_stack_window(window).is_ok_and(|is_stack| is_stack))
          .collect::<Vec<_>>();
        let mut eligible_windows = non_stack_windows
          .clone()
          .into_iter()
          .filter(|&window| window.frame.x <= top_right_window.frame.x)
          .collect::<Vec<_>>();

        eligible_windows.sort_by(|a, b| a.frame.x.partial_cmp(&b.frame.x).unwrap());
        let num_windows_to_right_of_top_right_windows = non_stack_windows.len() - eligible_windows.len();
        if num_windows_to_right_of_top_right_windows >= self.expected_current_num_master_windows {
          return Ok(top_right_window.frame.x);
        }

        let max = eligible_windows.len() - 1;
        for i in 0..max {
          let curr_window =
            eligible_windows.get(i).ok_or(eyre!("get_dividing_line_x_coordinate: curr_window is None"))?;
          let next_window =
            eligible_windows.get(i + 1).ok_or(eyre!("get_dividing_line_x_coordinate: next_window is None"))?;
          if curr_window.frame.x == next_window.frame.x
            && num_windows_to_right_of_top_right_windows + i + 2 >= self.expected_current_num_master_windows
          {
            return Ok(curr_window.frame.x);
          }
        }

        Ok(top_right_window.frame.x)
      },
    }
  }
}

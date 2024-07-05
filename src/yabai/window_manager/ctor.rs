use color_eyre::eyre::eyre;
use color_eyre::owo_colors::{AnsiColors, OwoColorize};
use log::{debug, error, info, trace};

use crate::print_bool;
use crate::yabai::window_manager::yabai::{get_yabai_config, YabaiCommand, YabaiConfig};
use crate::yabai::{
    command::get_yabai_command,
    config::{get_config, MasterPosition},
    display::Display,
    spaces::Space,
    state::{write_state, State, StateForSpace},
    window::{SplitType, Window},
};

use super::layout_visibility::LayoutValidity;

pub struct WindowsManager {
    pub(super) display: Display,
    pub(super) space: Space,
    pub(super) expected_current_num_master_windows: usize,
    pub(super) windows: Vec<Window>,
}

type Result<T> = color_eyre::Result<T>;

impl WindowsManager {
    pub fn new(display: Display, space: Space, expected_current_num_master_windows: usize) -> Self {
        trace!("Creating new WindowsManager with {display:?} {space:?} {expected_current_num_master_windows}",  expected_current_num_master_windows = expected_current_num_master_windows.blue());
        Self {
            display,
            expected_current_num_master_windows,
            space,
            windows: vec![],
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        debug!("Initializing window manager");
        self.windows = self.get_windows_data()?;

        Ok(())
    }

    pub(super) fn get_windows_data(&self) -> Result<Vec<Window>> {
        debug!("Reading windows data from yabai");
        let output = get_yabai_command()?
            .args(["-m", "query", "--windows"])
            .output()?;
        let windows = serde_json::from_slice::<Vec<Window>>(&output.stdout)?;
        debug!("Found {len} windows", len = windows.len().blue());
        let windows: Vec<Window> = windows
            .into_iter()
            .filter(|window| {
                debug!(
                    "{window}: {} - {} - {} - {}",
                    print_bool!(window.is_floating, "floating", "not floating"),
                    print_bool!(window.is_minimized, "minimized", "not minimized"),
                    print_bool!(window.is_hidden, "hidden", "not hidden"),
                    print_bool!(window.is_visible, "visible", "invisible")
                );
                if window.is_floating
                    || self.display.index != window.display
                    || self.space.index != window.space
                {
                    return false;
                }

                !(window.is_minimized || window.is_hidden || !window.is_visible)
            })
            .collect();
        debug!(
            "{len} windows are handled by ymsp",
            len = windows.len().blue()
        );

        Ok(windows)
    }

    pub fn validate_state(&mut self, state: &mut State) -> Result<()> {
        debug!("Validating state: {state:?}");
        let space_state = state.get_space_mut(&self.space)?;

        let len = self.windows.len();
        debug!(
            "Current number of windows: {len} < {expected}",
            len = len.blue(),
            expected = self.expected_current_num_master_windows.blue()
        );
        if len < self.expected_current_num_master_windows {
            self.expected_current_num_master_windows = len;
            debug!("Expected number of master windows: {len}", len = len);
            *space_state = len;
        }
        debug!("State validated: {state:?}");

        write_state(state)?;

        Ok(())
    }

    pub(crate) fn get_master_windows(&self) -> Result<Vec<Window>> {
        debug!("Looking for master windows");
        let config = get_config()?;
        let result = match config.master_position {
            MasterPosition::Left => self
                .windows
                .clone()
                .into_iter()
                .filter(|window| {
                    self.is_windows_touching_left_edge(window)
                        .inspect_err(|err| {
                            error!("An error occurred while checking if windows touch: {err}")
                        })
                        .is_ok_and(|is_touching| is_touching)
                })
                .collect(),
            MasterPosition::Right => {
                let dividing_line_x_coordinate = self.get_dividing_line_x_coordinate()?;
                self.windows
                    .clone()
                    .into_iter()
                    .filter(|window| window.frame.x >= dividing_line_x_coordinate)
                    .collect()
            }
        };

        Ok(result)
    }
    pub(crate) fn get_master_window(&self) -> Result<Option<Window>> {
        let master_windows = self.get_master_windows()?;
        Ok(master_windows.first().cloned())
    }

    pub(super) fn get_stack_windows(&self) -> Vec<Window> {
        debug!(
            "Looking for stack windows in {len} windows",
            len = self.windows.len()
        );

        let result: Vec<Window> = self
            .windows
            .clone()
            .into_iter()
            .filter(|window| self.is_stack_window(window).is_ok_and(|is_stack| is_stack))
            .collect();
        debug!("Found {len} stack windows", len = result.len());

        result
    }

    pub(super) fn is_windows_touching_left_edge(&self, window: &Window) -> Result<bool> {
        let left_padding: f64 = get_yabai_config(YabaiConfig::LeftPadding)?;
        trace!(
            "Checking if {window} is touching the left edge {x} {dx}",
            x = window.frame.x.bright_blue(),
            dx = self.display.frame.x.bright_blue()
        );
        Ok(window.frame.x == self.display.frame.x + left_padding)
    }

    pub(super) fn get_top_left_window(&self) -> Option<Window> {
        trace!("Looking for top left window");
        if self.windows.is_empty() {
            trace!("No windows found");
            return None;
        }

        let mut left_windows = self
            .windows
            .clone()
            .into_iter()
            .filter(|window| {
                self.is_windows_touching_left_edge(window)
                    .inspect_err(|err| {
                        error!("An error occurred while checking if windows touch: {err}")
                    })
                    .is_ok_and(|is_touching| is_touching)
            })
            .collect::<Vec<_>>();
        if left_windows.is_empty() {
            trace!("No left windows found");
            return None;
        }
        trace!("Found {len} left windows", len = left_windows.len());
        trace!("Sorting windows by y coordinate");
        left_windows.sort_by(|window1, window2| window1.frame.y.total_cmp(&window2.frame.y));
        trace!("Getting top left window");
        left_windows.first().cloned()
    }

    pub(super) fn get_top_right_window(&self) -> Option<Window> {
        if self.windows.is_empty() {
            return None;
        }

        let window = self.windows.first();
        if let Some(window) = window {
            let mut lowest_y_coordinate = window.frame.y;
            for window in self.windows.iter() {
                if window.frame.y < lowest_y_coordinate {
                    lowest_y_coordinate = window.frame.y;
                }
            }

            let top_windows = self
                .windows
                .iter()
                .filter(|window| window.frame.y == lowest_y_coordinate)
                .collect::<Vec<_>>();
            let top_window = top_windows.first();
            if let Some(&top_window) = top_window {
                let mut top_window = top_window;
                for window in top_windows.iter() {
                    if window.frame.x > top_window.frame.x {
                        top_window = window
                    }
                }
                Some(top_window.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(super) fn get_dividing_line_x_coordinate(&self) -> Result<f64> {
        trace!("get_dividing_line_x_coordinate() called.");
        let config = get_config()?;
        trace!(
            "Master position: {master_position:?}",
            master_position = config.master_position
        );
        match config.master_position {
            MasterPosition::Left => {
                let top_left_window = self.get_top_left_window().ok_or(eyre!(
                    "get_dividing_line_x_coordinate: top_left_window is None"
                ))?;
                info!("Top left windows: {top_left_window}");

                let mut non_master_windows = self
                    .windows
                    .iter()
                    .filter(|&window| {
                        !self
                            .is_windows_touching_left_edge(window)
                            .inspect_err(|err| {
                                error!("Error while checking if windows touch: {err}")
                            })
                            .is_ok_and(|is_touching| is_touching)
                    })
                    .collect::<Vec<_>>();
                non_master_windows
                    .sort_by(|window1, window2| window1.frame.x.total_cmp(&window2.frame.x));
                if non_master_windows.is_empty() {
                    return Ok(self.display.frame.x);
                }
                let num_master_windows = self.windows.len() - non_master_windows.len();
                if num_master_windows >= self.expected_current_num_master_windows {
                    let window = non_master_windows.first().ok_or(eyre!(
                        "get_dividing_line_x_coordinate: unable to find first()"
                    ))?;
                    debug!("get_dividing_line_x_coordinate: {x}", x = window.frame.x);
                    return Ok(window.frame.x);
                }

                let max = non_master_windows.len() - 1;
                for i in 0..max {
                    let curr_window = non_master_windows
                        .get(i)
                        .ok_or(eyre!("get_dividing_line_x_coordinate: curr_window is None"))?;
                    let next_window = non_master_windows
                        .get(i + 1)
                        .ok_or(eyre!("get_dividing_line_x_coordinate: next_window is None"))?;
                    if curr_window.frame.x == next_window.frame.x
                        && num_master_windows + i + 2 >= self.expected_current_num_master_windows
                    {
                        debug!(
                            "get_dividing_line_x_coordinate: {x}",
                            x = curr_window.frame.x
                        );
                        return Ok(curr_window.frame.x);
                    }
                }

                let window = non_master_windows.first().ok_or(eyre!(
                    "get_dividing_line_x_coordinate: unable to find first()"
                ))?;

                debug!(
                    "get_dividing_line_x_coordinate: window.frame.x: {x}",
                    x = window.frame.x
                );
                Ok(window.frame.x)
            }
            MasterPosition::Right => {
                let top_right_window = self.get_top_right_window();
                let top_right_window = top_right_window.ok_or(eyre!(
                    "get_dividing_line_x_coordinate: top_right_window is None"
                ))?;

                info!("Top right windows: {top_right_window}");

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
                let num_windows_to_right_of_top_right_windows =
                    non_stack_windows.len() - eligible_windows.len();
                if num_windows_to_right_of_top_right_windows
                    >= self.expected_current_num_master_windows
                {
                    return Ok(top_right_window.frame.x);
                }

                let max = eligible_windows.len() - 1;
                for i in 0..max {
                    let curr_window = eligible_windows
                        .get(i)
                        .ok_or(eyre!("get_dividing_line_x_coordinate: curr_window is None"))?;
                    let next_window = eligible_windows
                        .get(i + 1)
                        .ok_or(eyre!("get_dividing_line_x_coordinate: next_window is None"))?;
                    if curr_window.frame.x == next_window.frame.x
                        && num_windows_to_right_of_top_right_windows + i + 2
                            >= self.expected_current_num_master_windows
                    {
                        return Ok(curr_window.frame.x);
                    }
                }

                Ok(top_right_window.frame.x)
            }
        }
    }

    pub(super) fn is_stack_window(&self, window: &Window) -> Result<bool> {
        trace!("Checking that {window} is not a stacked window");
        let config = get_config()?;
        let result = match config.master_position {
            MasterPosition::Left => {
                let dividing_line_x_coordinate = self.get_dividing_line_x_coordinate()?;
                debug!(
                    "Dividing line x coordinate: {dividing_line_x_coordinate} {x}",
                    x = window.frame.x
                );
                window.frame.x == dividing_line_x_coordinate
            }
            MasterPosition::Right => self
                .is_windows_touching_left_edge(window)
                .inspect_err(|err| error!("Error while checking if windows touch: {err}"))
                .is_ok_and(|is_touching| is_touching),
        };
        trace!(
            "{window} is a stacked window: {result}",
            result = if result {
                result.color(AnsiColors::Green)
            } else {
                result.color(AnsiColors::Red)
            }
        );
        Ok(result)
    }

    pub(super) fn get_updated_window_data(&self, window: &Window) -> Option<&Window> {
        trace!("Updating window data for {window}");
        self.windows.iter().find(move |win| win.id == window.id)
    }

    pub(crate) fn is_valid_layout(
        &self,
        target_num_master_windows: Option<usize>,
    ) -> Result<LayoutValidity> {
        info!("Starting valid layout check...");
        if self.windows.is_empty() {
            return Ok(LayoutValidity::Valid);
        }
        let target_num_master_windows =
            target_num_master_windows.unwrap_or(self.expected_current_num_master_windows);
        debug!("Target number of master windows: {target_num_master_windows}");

        if target_num_master_windows > self.windows.len()
            && !self.windows.iter().all(|window| {
                self.is_windows_touching_left_edge(window)
                    .inspect_err(|err| error!("Error while checking if windows touch: {err}"))
                    .is_ok_and(|r| r)
            })
        {
            let reason = "The number of master windows is greater than the number of windows and not all windows are touching the left edge.".to_string();
            debug!("Layout invalid: {reason}");

            Ok(LayoutValidity::Invalid(reason))
        } else {
            let cur_num_master_windows = self.get_master_windows()?.len();
            if target_num_master_windows != cur_num_master_windows {
                return Ok(LayoutValidity::Invalid(format!("Number of master windows does not equal expected number of master windows ({cur_num_master_windows}/{target_num_master_windows})")));
            }

            for window in &self.windows.clone() {
                if self.is_middle_window(window) {
                    return Ok(LayoutValidity::Invalid(format!(
                        "A middle window ({window}) was detected.",
                    )));
                }
            }

            Ok(LayoutValidity::Valid)
        }
    }

    pub(super) fn is_middle_window(&self, window: &Window) -> bool {
        trace!("Checking if {window} is a middle window");

        !self
            .is_stack_window(window)
            .inspect_err(|err| error!("Error while getting is stack window: {err}"))
            .is_ok_and(|r| r)
            && !self
                .is_master_window(window)
                .inspect_err(|err| error!("Error while getting is stack window: {err}"))
                .is_ok_and(|r| r)
    }

    pub(super) fn is_master_window(&self, window: &Window) -> Result<bool> {
        let config = get_config()?;
        match config.master_position {
            MasterPosition::Left => self.is_windows_touching_left_edge(window),
            MasterPosition::Right => {
                let dividing_line_x_coordinate = self.get_dividing_line_x_coordinate()?;
                Ok(window.frame.x >= dividing_line_x_coordinate)
            }
        }
    }

    pub(super) fn does_stack_exists(&self) -> bool {
        let top_right_window = self.get_top_right_window();
        if let Some(top_right_window) = top_right_window {
            top_right_window.frame.x != 0f64
        } else {
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

    pub(super) fn get_middle_windows(&self) -> Vec<&Window> {
        debug!("Looking for middle windows");
        self.windows
            .iter()
            .filter(|w| self.is_middle_window(w))
            .collect::<Vec<_>>()
    }

    pub(crate) fn move_window_to_master(&self, window: &Window) -> Result<()> {
        info!("Moving window {window} to master.");
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

    pub(crate) fn get_window_data(
        &self,
        process_id: usize,
        window_id: usize,
    ) -> color_eyre::Result<&Window> {
        let window = self
            .windows
            .iter()
            .find(|window| window.pid == process_id && window.id == window_id);

        window.ok_or(eyre!(
            "Window with id {window_id} and process id {process_id} not found."
        ))
    }
}

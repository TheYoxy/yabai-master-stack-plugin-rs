use std::{
  fmt::{Debug, Formatter},
  fs,
};

use color_eyre::{eyre::bail, owo_colors::OwoColorize};
use log::trace;

use crate::yabai::{
  commands::{get_yabai_command, RunCommand},
  config::{get_lockfile, MasterPosition, ToYabaiDirection},
  display::Display,
  window::Window,
  window_manager::ctor::WindowsManager,
};

#[derive(Debug)]
pub enum YabaiDirection {
  Prev,
  Next,
  Recent,
  Mouse,
  Largest,
  Smallest,
  Sibling,
  FirstNephew,
  SecondNephew,
  Uncle,
  FirstCousin,
  SecondCousin,

  First,
  Last,
  North,
  South,
  East,
  West,
}
impl std::fmt::Display for YabaiDirection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_yabai_direction().blue()) }
}
impl ToYabaiDirection for YabaiDirection {
  fn to_yabai_direction(&self) -> &str {
    match self {
      YabaiDirection::North => "north",
      YabaiDirection::South => "south",
      YabaiDirection::East => "east",
      YabaiDirection::West => "west",
      YabaiDirection::First => "first",
      YabaiDirection::Last => "last",
      YabaiDirection::Prev => "prev",
      YabaiDirection::Next => "next",
      YabaiDirection::Recent => "recent",
      YabaiDirection::Mouse => "mouse",
      YabaiDirection::Largest => "largest",
      YabaiDirection::Smallest => "smallest",
      YabaiDirection::Sibling => "sibling",
      YabaiDirection::FirstNephew => "first_nephew",
      YabaiDirection::SecondNephew => "second_nephew",
      YabaiDirection::Uncle => "uncle",
      YabaiDirection::FirstCousin => "first_cousin",
      YabaiDirection::SecondCousin => "second_cousin",
    }
  }
}

#[derive(Debug)]
pub enum YabaiCommand<'a> {
  WarpWindow(&'a Window, &'a Window),
  ToggleWindowSplit(&'a Window),
  WarpDirection(&'a Window, &'a MasterPosition),
  FocusWindow(&'a Window),
  FocusDirection(YabaiDirection),
}
impl<'a> std::fmt::Display for YabaiCommand<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      YabaiCommand::WarpWindow(_, _) => "WarpWindow",
      YabaiCommand::ToggleWindowSplit(_) => "ToggleWindowSplit",
      YabaiCommand::WarpDirection(_, _) => "WarpDirection",
      YabaiCommand::FocusWindow(_) => "FocusWindow",
      YabaiCommand::FocusDirection(_) => "FocusDirection",
    })
  }
}

#[derive(Debug)]
pub enum YabaiConfig {
  LeftPadding,
}
impl std::fmt::Display for YabaiConfig {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_yabai_command().blue()) }
}

impl YabaiConfig {
  pub fn to_yabai_command(&self) -> &str {
    match self {
      YabaiConfig::LeftPadding => "left_padding",
    }
  }
}

impl WindowsManager {
  pub(crate) fn run_yabai_command(&self, command: YabaiCommand) -> color_eyre::Result<()> {
    let lockfile = get_lockfile()?;
    let exists = lockfile.try_exists()?;
    if !exists {
      bail!("Lockfile is not longer owned by this process");
    }
    let pid = fs::read_to_string(lockfile)?;
    let pid: u32 = pid.parse()?;
    let current_pid = std::process::id();
    trace!("Checking lockfile pid: {} == {}", pid.blue(), current_pid.blue());
    if pid != current_pid {
      bail!("Lockfile is not longer owned by this process");
    }
    trace!("Lockfile pid is owned by this process");

    trace!("Running yabai command: {}", command.blue());
    match command {
      YabaiCommand::WarpWindow(window, master_window) => warp_window(window, master_window),
      YabaiCommand::ToggleWindowSplit(window) => toggle_window_split(window),
      YabaiCommand::WarpDirection(window, position) => warp_direction(window, position),
      YabaiCommand::FocusWindow(window) => focus_window(window),
      YabaiCommand::FocusDirection(direction) => focus_direction(&direction),
    }
  }
}

pub(super) fn get_yabai_config<T>(config: YabaiConfig) -> color_eyre::Result<T>
where
  T: serde::de::DeserializeOwned,
{
  trace!("Getting yabai config value of {}", config);
  let output = get_yabai_command()?.args(["-m", "config", config.to_yabai_command()]).run_command_with_output()?;

  let output = String::from_utf8(output.stdout)?;
  let output = serde_json::from_str(&output)?;
  Ok(output)
}

pub fn focus_window(window: &Window) -> color_eyre::Result<()> {
  trace!("focusing window: {:?}", window);
  get_yabai_command()?.args(["-m", "window", "--focus", window.id.to_string().as_str()]).run_command()
}

pub fn focus_display(display: &Display) -> color_eyre::Result<()> {
  trace!("focusing display: {}", display);
  get_yabai_command()?.args(["-m", "display", "--focus", display.index.to_string().as_str()]).run_command()
}

pub fn move_window_to_display(display: &Display) -> color_eyre::Result<()> {
  trace!("moving current window to display: {}", display);
  get_yabai_command()?.args(["-m", "window", "--display", display.index.to_string().as_str()]).run_command()
}

pub fn focus_direction<T>(direction: &T) -> color_eyre::Result<()>
where
  T: ToYabaiDirection + std::fmt::Display,
{
  trace!("focusing direction: {}", direction);
  get_yabai_command()?.args(["-m", "display", "--focus", direction.to_yabai_direction()]).run_command()
}

pub fn swap_direction(direction: &MasterPosition) -> color_eyre::Result<()> {
  trace!("focusing direction: {:?}", direction);
  get_yabai_command()?.args(["-m", "display", "--swap", direction.to_yabai_direction()]).run_command()
}

fn warp_direction(window: &Window, direction: &MasterPosition) -> color_eyre::Result<()> {
  trace!("warping window {window} to {direction} -> {direction}");
  get_yabai_command()?
    .args(["-m", "window", window.id.to_string().as_str(), "--warp", direction.to_yabai_direction()])
    .run_command()
}

fn warp_window(window: &Window, master_window: &Window) -> color_eyre::Result<()> {
  trace!("warping window {window} to master window {master_window}");
  get_yabai_command()?
    .args(["-m", "window", window.id.to_string().as_str(), "--warp", master_window.id.to_string().as_str()])
    .run_command()
}

fn toggle_window_split(window: &Window) -> color_eyre::Result<()> {
  use log::trace;
  trace!("splitting window {window}");
  get_yabai_command()?.args(["-m", "window", window.id.to_string().as_str(), "--toggle", "split"]).run_command()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::yabai::config::initialize_config;

  #[test]
  fn test_get_yabai_config() {
    initialize_config().unwrap();
    let config = get_yabai_config::<f64>(YabaiConfig::LeftPadding).unwrap();
    assert_eq!(config, 10.0);
  }
}

use std::fmt::{Debug, Formatter};

use color_eyre::{eyre::bail, owo_colors::OwoColorize};
use log::{trace, warn};

use crate::{
  dry_mode::is_dry_mode,
  task::lock::is_locked,
  window_manager::WindowsManager,
  yabai::{
    commands::{get_yabai_command, RunCommand},
    config::{MasterPosition, ToYabaiDirection},
    display::Display,
    window::Window,
  },
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
    match is_locked() {
      Ok(false) => {
        trace!("Running yabai command: {}", command.blue());
        match command {
          YabaiCommand::WarpWindow(window, master_window) => warp_window(window, master_window),
          YabaiCommand::ToggleWindowSplit(window) => toggle_window_split(window),
          YabaiCommand::WarpDirection(window, position) => warp_direction(window, position),
          YabaiCommand::FocusWindow(window) => focus_window(window),
          YabaiCommand::FocusDirection(direction) => focus_direction(&direction),
        }
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

pub(crate) fn get_yabai_config<T>(config: YabaiConfig) -> color_eyre::Result<T>
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
  trace!("focusing window: {window}");
  if is_dry_mode() {
    warn!("skipping focus window {window}");
    Ok(())
  } else {
    get_yabai_command()?.args(["-m", "window", "--focus", window.id.to_string().as_str()]).run_command()
  }
}

pub fn focus_display(display: &Display) -> color_eyre::Result<()> {
  trace!("focusing display: {display}");
  if is_dry_mode() {
    warn!("skipping focus display {display}");
    Ok(())
  } else {
    get_yabai_command()?.args(["-m", "display", "--focus", display.index.to_string().as_str()]).run_command()
  }
}

pub fn move_window_to_display(display: &Display) -> color_eyre::Result<()> {
  trace!("moving current window to display: {display}");
  if is_dry_mode() {
    warn!("skipping move window to display {display}");
    Ok(())
  } else {
    get_yabai_command()?.args(["-m", "window", "--display", display.index.to_string().as_str()]).run_command()
  }
}

pub fn focus_direction<T>(direction: &T) -> color_eyre::Result<()>
where
  T: ToYabaiDirection + std::fmt::Display,
{
  trace!("focusing direction: {direction}");
  if is_dry_mode() {
    warn!("skipping focus direction {direction}");
    Ok(())
  } else {
    get_yabai_command()?.args(["-m", "display", "--focus", direction.to_yabai_direction()]).run_command()
  }
}

pub fn swap_direction(direction: &MasterPosition) -> color_eyre::Result<()> {
  trace!("focusing direction: {:?}", direction);
  if is_dry_mode() {
    warn!("skipping swap direction {:?}", direction);
    Ok(())
  } else {
    get_yabai_command()?.args(["-m", "display", "--swap", direction.to_yabai_direction()]).run_command()
  }
}

fn warp_direction(window: &Window, direction: &MasterPosition) -> color_eyre::Result<()> {
  trace!("warping window {window} to {direction} -> {direction}");
  if is_dry_mode() {
    warn!("skipping warp window {window} to {direction} -> {direction}");
    Ok(())
  } else {
    get_yabai_command()?
      .args(["-m", "window", window.id.to_string().as_str(), "--warp", direction.to_yabai_direction()])
      .run_command()
  }
}

fn warp_window(window: &Window, master_window: &Window) -> color_eyre::Result<()> {
  trace!("warping window {window} to master window {master_window}");
  if is_dry_mode() {
    warn!("skipping warp window {window} to master window {master_window}");
    Ok(())
  } else {
    get_yabai_command()?
      .args(["-m", "window", window.id.to_string().as_str(), "--warp", master_window.id.to_string().as_str()])
      .run_command()
  }
}

fn toggle_window_split(window: &Window) -> color_eyre::Result<()> {
  trace!("splitting window {window}");
  if is_dry_mode() {
    warn!("skipping split window {window}");
    Ok(())
  } else {
    get_yabai_command()?.args(["-m", "window", window.id.to_string().as_str(), "--toggle", "split"]).run_command()
  }
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

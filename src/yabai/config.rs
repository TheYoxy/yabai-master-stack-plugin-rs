use core::fmt::Display;
use std::{fmt::Debug, path::PathBuf, sync::OnceLock};

use color_eyre::{
  eyre::{bail, eyre},
  owo_colors::OwoColorize,
};
use log::trace;
use serde::Deserialize;

use crate::print_bool;

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MasterPosition {
  Left,
  #[default]
  Right,
}

pub trait ToYabaiDirection {
  fn to_yabai_direction(&self) -> &str;
}

impl ToYabaiDirection for MasterPosition {
  fn to_yabai_direction(&self) -> &str {
    match self {
      MasterPosition::Left => "west",
      MasterPosition::Right => "east",
    }
  }
}

impl Display for MasterPosition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      MasterPosition::Left => "Left",
      MasterPosition::Right => "Right",
    };
    write!(f, "{:?}", str.yellow())
  }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct YabaiMasterStackPluginConfig {
  pub(crate) yabai_path: String,
  pub(crate) debug: bool,
  pub(crate) move_new_windows_to_master: bool,
  pub(crate) master_position: MasterPosition,
}

impl Display for YabaiMasterStackPluginConfig {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let debug = print_bool!(self.debug);
    let move_new_windows_to_master = print_bool!(self.move_new_windows_to_master);
    let path = print_bool!(
      self.yabai_path == YabaiMasterStackPluginConfig::default().yabai_path,
      self.yabai_path,
      self.yabai_path
    );
    write!(f, "YabaiMasterStackPluginConfig {{ yabai_path: {}, debug: {}, move_new_windows_to_master: {}, master_position: {} }}", path, debug, move_new_windows_to_master, self.master_position)
  }
}

impl ToYabaiDirection for YabaiMasterStackPluginConfig {
  fn to_yabai_direction(&self) -> &str { self.master_position.to_yabai_direction() }
}

impl Default for YabaiMasterStackPluginConfig {
  fn default() -> Self {
    Self {
      yabai_path: "/usr/local/bin/yabai".to_string(),
      debug: false,
      move_new_windows_to_master: false,
      master_position: Default::default(),
    }
  }
}

fn get_config_path() -> color_eyre::Result<PathBuf> {
  let path = homedir::my_home()?.ok_or(eyre!("Failed to get home directory"))?;

  Ok(path.join(".config").join("ymsp"))
}

pub fn check_config_path_exists() -> color_eyre::Result<()> {
  let path = get_config_path()?;
  let exists = path.exists();

  if exists {
    Ok(())
  } else {
    bail!("Configuration path {path:?} does not exist")
  }
}

pub fn get_state_path() -> color_eyre::Result<PathBuf> {
  let path = get_config_path()?;
  let state = path.join("state.json");

  Ok(state)
}

pub fn get_lockfile() -> color_eyre::Result<PathBuf> {
  let path = get_config_path()?;
  let lockfile = path.join("ymsp.lock");

  Ok(lockfile)
}

pub fn get_config_file() -> color_eyre::Result<PathBuf> {
  let path = get_config_path()?;
  let config_file_path = path.join("ymsp.config.json");
  Ok(config_file_path)
}

static CELL: OnceLock<YabaiMasterStackPluginConfig> = OnceLock::new();
pub fn initialize_config() -> color_eyre::Result<()> {
  trace!("Reading configuration");
  let config_file_path = get_config_file()?;
  trace!("Looking for file {config_file_path:?}", config_file_path = config_file_path.yellow());

  let exists = config_file_path.try_exists()?;
  let config = if exists {
    let file = std::fs::File::open(config_file_path)?;
    trace!("Reading configuration file");
    let data: YabaiMasterStackPluginConfig = serde_json::from_reader(file)?;
    trace!("Deserialized configuration: {data}");
    data
  } else {
    bail!("Configuration file {config_file_path:?} not found")
  };

  CELL.set(config).map_err(|_| eyre!("Failed to set config"))?;

  Ok(())
}

pub fn get_config() -> color_eyre::Result<YabaiMasterStackPluginConfig> {
  match CELL.get() {
    Some(value) => Ok(value.to_owned()),
    None => bail!("Config not set"),
  }
}

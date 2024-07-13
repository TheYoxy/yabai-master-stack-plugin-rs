use std::collections::HashMap;

use color_eyre::owo_colors::OwoColorize;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::yabai::{command::message::YabaiMessage, config::get_state_path, spaces::Space};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
  values: HashMap<usize, usize>,
}

pub trait StateForSpace {
  fn get_space(&self, space: &Space) -> color_eyre::Result<&usize>;
  fn get_space_mut(&mut self, space: &Space) -> color_eyre::Result<&mut usize>;
}

impl State {
  pub fn get_space(&self, space: &Space) -> color_eyre::Result<&usize> {
    use color_eyre::eyre::eyre;
    self.values.get(&space.id).ok_or(eyre!("Unable to get the space {id}", id = space.id))
  }

  pub fn get_space_mut(&mut self, space: &Space) -> color_eyre::Result<&mut usize> {
    use color_eyre::eyre::eyre;
    self.values.get_mut(&space.id).ok_or(eyre!("Unable to get the space {id}", id = space.id))
  }

  pub fn write_state(&self) -> color_eyre::Result<()> {
    let state_file_path = get_state_path()?;
    debug!("Writing state to {state_file_path:?}", state_file_path = state_file_path.yellow());
    let file = std::fs::File::create(&state_file_path)?;
    serde_json::to_writer(file, self)?;
    debug!("State written to {state_file_path:?}", state_file_path = state_file_path.yellow());

    Ok(())
  }

  pub fn read_state() -> color_eyre::Result<Self> {
    debug!("Reading base state");
    let state_file_path = get_state_path()?;
    debug!("Looking for state file at: {:?}", state_file_path.yellow());
    #[cfg(debug_assertions)]
    {
      use log::warn;
      warn!("Removing state file for testing purposes");
      std::fs::remove_file(&state_file_path)?;
    }
    let exists = state_file_path.try_exists()?;
    if exists {
      debug!("Reading state from {:?}", state_file_path);
      let file = std::fs::File::open(state_file_path)?;
      let mut state: State = serde_json::from_reader(file)?;
      debug!("Filling spaces in the state");
      let spaces = YabaiMessage::query().spaces()?;
      for space in &spaces {
        state.values.entry(space.id).or_insert(1);
      }

      for (space_id, _) in spaces.iter().enumerate() {
        if !spaces.iter().any(|space| space.id == space_id) {
          state.values.remove(&space_id);
        }
      }
      debug!("State: {state:?}");
      Ok(state)
    } else {
      debug!("Creating new state");
      let mut state = State::default();
      let spaces = YabaiMessage::query().spaces()?;
      for space in &spaces {
        debug!("Adding space {space_id} to the state", space_id = space.id.blue());
        state.values.entry(space.id).or_insert(1);
      }
      state.write_state()?;
      debug!("New state: {state:?}");
      Ok(state)
    }
  }
}

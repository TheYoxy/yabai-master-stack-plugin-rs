pub mod events {
  use log::trace;

  use crate::{
    task::create_initialized_windows_manager::InitializedWindowsManager,
    window_manager::layout_visibility::LayoutValidity, yabai::config::get_config,
  };

  pub fn on_yabai_start(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    trace!("Handling on yabai start event");
    let wm = &mut iwm.wm;

    let state = &iwm.state;
    let space = &iwm.space;
    let space_state = state.get_space(space)?;
    trace!("Updating windows...");
    wm.update_windows(*space_state)?;
    trace!("On yabai start event handled");

    Ok(())
  }

  pub fn window_created(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    trace!("Handling window created event");

    let wm = &mut iwm.wm;
    let state = &iwm.state;
    let space = &iwm.space;
    let layout_validity = wm.is_valid_layout(None)?;

    match layout_validity {
      LayoutValidity::Valid => {
        trace!("Layout is valid, no changes were made.");
        Ok(())
      },
      LayoutValidity::Invalid(_) => {
        trace!("Window created event handled");
        let process_id: usize = std::env::var("YABAI_PROCESS_ID")?.parse()?;
        let window_id: usize = std::env::var("YABAI_WINDOW_ID")?.parse()?;
        let cur_num_master_windows = wm.get_master_windows()?.len();
        let window = wm.get_window_data(process_id, window_id)?;
        let space_state = state.get_space(space)?;
        let config = get_config()?;
        if config.move_new_windows_to_master {
          // If the master is full, move a window from master to stack
          if cur_num_master_windows >= *space_state {
            let old_master_windows = wm.get_master_window()?;
            if let Some(old_master_windows) = old_master_windows {
              wm.move_window_to_stack(&old_master_windows)?;
            }
            wm.move_window_to_master(window)?;
          } else {
            wm.move_window_to_master(window)?;
          }
        } else if cur_num_master_windows > 1 && cur_num_master_windows <= *space_state {
          trace!("Moving new window {window} to master");
          wm.move_window_to_master(window)?;
        } else {
          trace!("Moving new window {window} to stack");
          wm.move_window_to_stack(window)?;
        }
        wm.update_windows(*space_state)?;
        Ok(())
      },
    }
  }

  pub(crate) fn window_moved(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    trace!("Handling window moved event");

    let wm = &mut iwm.wm;
    let state = &iwm.state;
    let space = &iwm.space;
    let space_state = state.get_space(space)?;
    trace!("Updating windows...");
    wm.update_windows(*space_state)?;
    trace!("Window moved event handled");

    Ok(())
  }
}

pub(crate) mod focus {
  use color_eyre::eyre::bail;
  use log::{debug, trace};

  use crate::{
    task::create_initialized_windows_manager::InitializedWindowsManager,
    window_manager::WindowsManager,
    yabai::{
      command::{
        direction_selector::YabaiDirectionSelector, message::YabaiMessage, to_command::Runnable,
        window_selector::YabaiWindowSelector,
      },
      config::get_config,
      window::Window,
    },
  };

  pub(crate) fn focus_master_window() -> color_eyre::Result<()> {
    let config = get_config()?;

    YabaiMessage::current_display().focus(config.master_position)?.run()?;

    Ok(())
  }

  /// Focus the window if any exists
  fn _focus_window(wm: &WindowsManager, window_to_focus: Option<Window>) -> color_eyre::Result<()> {
    if let Some(window_to_focus) = window_to_focus {
      debug!("Focusing window {}", window_to_focus);
      let message = YabaiMessage::current_window().focus(YabaiWindowSelector::Id(window_to_focus.id))?;
      wm.send_yabai_message(message)?;
      Ok(())
    } else {
      debug!("No window to focus on");
      Ok(())
    }
  }

  pub(crate) fn focus_up_window(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    let wm = &mut iwm.wm;
    let focused_window = wm.get_focused_window();

    if let Some(focused_window) = focused_window {
      if wm.is_master_window(focused_window)? && wm.is_top_window(wm.get_master_windows()?, focused_window) {
        let window_to_focus = wm.get_bottom_stack_window().or(wm.get_bottom_master_window()?);

        _focus_window(wm, window_to_focus)
      } else if wm.is_stack_window(focused_window)? && wm.is_top_window(wm.get_stack_windows(), focused_window) {
        let window_to_focus = wm.get_bottom_master_window()?;

        _focus_window(wm, window_to_focus)
      } else {
        trace!("Focusing north window");
        let message = YabaiMessage::current_window().focus(YabaiDirectionSelector::North)?;
        wm.send_yabai_message(message)
      }
    } else {
      trace!("No focused window, focusing first window");
      let message = YabaiMessage::current_window().focus(YabaiWindowSelector::First)?;
      wm.send_yabai_message(message)
    }
  }

  pub(crate) fn focus_down_window(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    let wm = &mut iwm.wm;
    let focused_window = wm.get_focused_window();

    if let Some(focused_window) = focused_window {
      if wm.is_master_window(focused_window)? && wm.is_bottom_window(wm.get_master_windows()?, focused_window) {
        let window_to_focus = wm.get_top_stack_window().or(wm.get_top_master_window()?);

        _focus_window(wm, window_to_focus)
      } else if wm.is_stack_window(focused_window)? && wm.is_bottom_window(wm.get_stack_windows(), focused_window) {
        let window_to_focus = wm.get_top_master_window()?;

        _focus_window(wm, window_to_focus)
      } else {
        trace!("Focusing south window");
        let message = YabaiMessage::current_window().focus(YabaiDirectionSelector::South)?;
        wm.send_yabai_message(message)
      }
    } else {
      trace!("No focused window, focusing first window");
      let message = YabaiMessage::current_window().focus(YabaiWindowSelector::First)?;
      wm.send_yabai_message(message)
    }
  }

  /// Focus the next display
  pub(crate) fn focus_next_display() -> color_eyre::Result<()> {
    let mut displays = YabaiMessage::query().displays()?;
    let focused_display = YabaiMessage::query().current_display()?;
    displays.sort_by(|d1, d2| d1.frame.x.total_cmp(&d2.frame.x));
    trace!("Displays: {displays:?}");
    let focused_display_order_index = displays.iter().position(|display| display.id == focused_display.id);
    if let Some(focused_display_order_index) = focused_display_order_index {
      let next_display = displays.get((focused_display_order_index + 1) % displays.len());
      if let Some(next_display) = next_display {
        trace!("Focusing next display: {next_display}");
        YabaiMessage::current_display().focus(next_display)?.run().map(|_| ())
      } else {
        bail!("Could not find next display in displays: {displays:?}")
      }
    } else {
      bail!("Could not find focused display in displays: {displays:?}")
    }
  }

  /// Focus the previous display
  pub(crate) fn focus_previous_display() -> color_eyre::Result<()> {
    let mut displays = YabaiMessage::query().displays()?;
    let focused_display = YabaiMessage::query().current_display()?;
    displays.sort_by(|d1, d2| d1.frame.x.total_cmp(&d2.frame.x));
    trace!("Displays: {displays:?}");
    let focused_display_order_index = displays.iter().position(|display| display.id == focused_display.id);
    if let Some(focused_display_order_index) = focused_display_order_index {
      let previous_display = displays.get(((focused_display_order_index - 1) + displays.len()) % displays.len());
      if let Some(previous_display) = previous_display {
        trace!("Focusing previous display: {previous_display}");
        YabaiMessage::current_display().focus(previous_display)?.run().map(|_| ())
      } else {
        bail!("Could not find previous display in displays: {displays:?}")
      }
    } else {
      bail!("Could not find focused display in displays: {displays:?}")
    }
  }
}

pub(crate) mod window_count {
  use color_eyre::{eyre::bail, owo_colors::OwoColorize};
  use log::{debug, trace};

  use crate::task::create_initialized_windows_manager::InitializedWindowsManager;

  pub(crate) fn increase_master_window_count(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    let wm = &mut iwm.wm;
    let state = &mut iwm.state;
    let space = &iwm.space;

    let space_state = state.get_space_mut(space)?;
    let windows = wm.windows();
    let current_state = *space_state;
    debug!("Current master window count: {} < {}", (current_state + 1).blue(), windows.len().blue());
    if current_state + 1 < windows.len() {
      *space_state += 1;
      wm.update_windows(current_state)?;
      trace!("Increased master window count to {}", current_state);
      state.write_state()
    } else {
      bail!("Cannot increase master window count above or equals to the number of windows in the space")
    }
  }

  pub(crate) fn decrease_master_window_count(iwm: &mut InitializedWindowsManager) -> color_eyre::Result<()> {
    let wm = &mut iwm.wm;
    let state = &mut iwm.state;
    let space = &iwm.space;

    let space_state = state.get_space_mut(space)?;
    if *space_state > 1 {
      *space_state -= 1;
      wm.update_windows(*space_state)?;
      trace!("Decreased master window count to {}", *space_state);
      state.write_state()
    } else {
      bail!("Cannot decrease master window count below 1");
    }
  }
}

pub(crate) mod move_window {
  use color_eyre::eyre::bail;
  use log::{info, trace};

  use crate::yabai::{
    command::{message::YabaiMessage, to_command::Runnable},
    config::get_config,
  };

  pub(crate) fn move_window_to_master() -> color_eyre::Result<()> {
    info!("moving current window to master");
    let config = get_config()?;

    // todo: check if the current window is already in the master position
    // let windows = get_windows()?;
    // debug!("Windows: {windows:?}");
    let message = YabaiMessage::current_window().swap(config.master_position)?;
    message.run()?;

    Ok(())
  }

  pub(crate) fn move_window_to_next_display() -> color_eyre::Result<()> {
    let mut displays = YabaiMessage::query().displays()?;
    let focused_display = YabaiMessage::query().current_display()?;
    displays.sort_by(|d1, d2| d1.frame.x.total_cmp(&d2.frame.x));
    trace!("Displays: {displays:?}");
    let focused_display_order_index = displays.iter().position(|display| display.id == focused_display.id);
    if let Some(focused_display_order_index) = focused_display_order_index {
      let next_display = displays.get((focused_display_order_index + 1) % displays.len());
      if let Some(next_display) = next_display {
        trace!("Moving window to next display: {next_display}");
        YabaiMessage::current_window().display(next_display)?.run().map(|_| ())
      } else {
        bail!("Could not find next display in displays: {displays:?}")
      }
    } else {
      bail!("Could not find focused display in displays: {displays:?}")
    }
  }
  pub(crate) fn move_window_to_previous_display() -> color_eyre::Result<()> {
    let mut displays = YabaiMessage::query().displays()?;
    let focused_display = YabaiMessage::query().current_display()?;
    displays.sort_by(|d1, d2| d1.frame.x.total_cmp(&d2.frame.x));
    trace!("Displays: {displays:?}");
    let focused_display_order_index = displays.iter().position(|display| display.id == focused_display.id);
    if let Some(focused_display_order_index) = focused_display_order_index {
      let previous_display = displays.get(((focused_display_order_index - 1) + displays.len()) % displays.len());
      if let Some(previous_display) = previous_display {
        trace!("Moving window to previous display: {previous_display}");
        YabaiMessage::current_window().display(previous_display)?.run().map(|_| ())
      } else {
        bail!("Could not find previous display in displays: {displays:?}")
      }
    } else {
      bail!("Could not find focused display in displays: {displays:?}")
    }
  }
  pub(crate) fn close_focused_window() -> color_eyre::Result<()> { todo!() }
}

pub(crate) mod completion {
  use clap::CommandFactory;
  use log::debug;

  use crate::{cli::Cli, task::CompletionArgs};

  fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    use clap_complete::generate;
    debug!("Generating completions for command: {:?}", cmd.get_name());
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
  }

  pub(crate) fn generate_completion(completion: &CompletionArgs) -> color_eyre::Result<()> {
    let mut cmd = Cli::command();
    debug!("Generating completions for shell: {:?}", completion);
    print_completions(completion.shell, &mut cmd);
    Ok(())
  }
}

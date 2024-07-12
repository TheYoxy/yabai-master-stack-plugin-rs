use crate::yabai::command::{
  display_selector::YabaiDisplaySelector, space_selector::YabaiSpaceSelector, window_selector::YabaiWindowSelector,
};

#[derive(Debug, Clone)]
pub enum YabaiWindowCommandType {
  /// Focus the given window.
  /// If none specified, focus the selected window instead.
  Focus(Option<YabaiWindowSelector>),
  /// Close the given window.
  /// If none specified, close the selected window instead.
  /// Only works on windows that provide a close button in its titlebar.
  Close(Option<YabaiWindowSelector>),
  /// Minimize the given window.
  /// If none specified, minimize the selected window instead.
  /// Only works on windows that provide a minimize button in its titlebar.
  Minimize(Option<YabaiWindowSelector>),
  /// Restore the given window if it is minimized.
  /// The window will only get focus if the owning application has focus.
  /// Note that you can also --focus a minimized window to restore it as the focused window.
  Deminimize(YabaiWindowSelector),
  /// Send the selected window to the given display.
  Display(YabaiDisplaySelector),
  /// Send the selected window to the given space.
  Space(YabaiSpaceSelector),
  /// Swap position of the selected window and the given window.
  Swap(YabaiWindowSelector),
  /// Re-insert the selected window, splitting the given window.
  Warp(YabaiWindowSelector),
  /// Stack the given window on top of the selected window.
  /// Any kind of warp operation performed on a stacked window will unstack it.
  Stack(YabaiWindowSelector),
}

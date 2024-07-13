use crate::yabai::command::to_argument::ToArgument;

#[derive(Debug, Clone)]
pub enum YabaiToggleSelector {
  Float,
  Sticky,
  Pip,
  Shadow,
  Split,
  ZoomParent,
  ZoomFullscreen,
  NativeFullscreen,
  Expose,
  Label(String),
}
impl ToArgument for YabaiToggleSelector {
  fn to_argument(&self) -> String {
    match self {
      YabaiToggleSelector::Float => "float".into(),
      YabaiToggleSelector::Sticky => "sticky".into(),
      YabaiToggleSelector::Pip => "pip".into(),
      YabaiToggleSelector::Shadow => "shadow".into(),
      YabaiToggleSelector::Split => "split".into(),
      YabaiToggleSelector::ZoomParent => "zoom-parent".into(),
      YabaiToggleSelector::ZoomFullscreen => "zoom-fullscreen".into(),
      YabaiToggleSelector::NativeFullscreen => "native-fullscreen".into(),
      YabaiToggleSelector::Expose => "expose".into(),
      YabaiToggleSelector::Label(label) => label.clone(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LayoutValidity {
  Valid,
  Invalid(String),
}

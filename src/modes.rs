use crate::errors::ModeFromStrError;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum ScreenReaderMode {
    BrowseMode,
    FocusMode,
    ObjectNavigationMode,
    /* Usually activated when pressing the Odilia key */
    CommandMode,
}

impl FromStr for ScreenReaderMode {
  type Err = ModeFromStrError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    use ModeFromStrError as E;
    match input {
      "BrowseMode" => Ok(ScreenReaderMode::BrowseMode),
      "FocusMode" => Ok(ScreenReaderMode::FocusMode),
      "CommandMode" => Ok(ScreenReaderMode::CommandMode),
      "ObjectNavigationMode" => Ok(ScreenReaderMode::ObjectNavigationMode),
      _ => Err(E::ModeNameNotFound)
    }
  }
}

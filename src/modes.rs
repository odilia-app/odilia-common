#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct ScreenReaderMode {
  pub name: String,
}

impl ScreenReaderMode {
  pub fn new(name: String) -> Self {
    ScreenReaderMode {
      name
    }
  }
}

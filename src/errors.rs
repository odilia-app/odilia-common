#[derive(Debug, Clone, Copy)]
pub enum KeyFromStrError {
    EmptyString,
    NoKey,
    InvalidKey,
    InvalidRepeat,
    InvalidModifier,
    InvalidMode,
}

#[derive(Debug, Clone, Copy)]
pub enum ModeFromStrError {
  ModeNameNotFound,
}

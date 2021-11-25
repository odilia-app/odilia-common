#[derive(Debug, Clone, Copy)]
pub enum KeyFromStrError {
    EmptyString,
    NoKey,
    EmptyKey,
    InvalidKey,
    InvalidRepeat,
    InvalidModifier,
    InvalidMode,
}

#[derive(Debug, Clone, Copy)]
pub enum ModeFromStrError {
    ModeNameNotFound,
}

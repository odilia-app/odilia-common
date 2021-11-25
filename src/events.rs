use crate::{elements::ElementType, modes::ScreenReaderMode};

pub enum ScreenReaderEventType {
    ChangeMode(ScreenReaderMode),
    Next(ElementType),
    Previous(ElementType),
}

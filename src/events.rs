use crate::elements::ElementTypes

pub enum ScreenReaderEventType {
    ChangeMode(ScreenReaderMode),
    Next(ElementType),
    Previous(ElementType),
}

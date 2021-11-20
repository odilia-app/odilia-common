pub enum ElementType {
    Heading,
    HeadingLevel1,
    HeadingLevel2,
    HeadingLevel3,
    HeadingLevel4,
    HeadingLevel5,
    HeadingLevel6,
    Button,
    Text,
    Table,
    TableCell,
    List,
    ListItem,
    Tab, // ?? is this what it is when you're looking at tabs in a dialog?
}

pub enum ScreenReaderMode {
    BrowseMode,
    FocusMode,
    ObjectNavigationMode,
}

pub enum ScreenReaderEventType {
    ChangeMode(ScreenReaderMode),
    Next(ElementType),
    Previous(ElementType),
}

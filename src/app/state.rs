#[derive(Clone, Copy, PartialEq)]
pub enum ActivePanel {
    Folders,
    Notes,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AppMode {
    Normal,
    CreatingNote,
    Searching,
}

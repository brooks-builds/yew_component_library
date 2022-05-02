#[derive(Clone, PartialEq)]
pub enum BBTextType {
    Normal,
    Title,
}

impl Default for BBTextType {
    fn default() -> Self {
        Self::Normal
    }
}

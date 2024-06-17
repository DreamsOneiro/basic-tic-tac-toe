#[derive(Copy, Clone)]
pub enum GridKey {
    X,
    O,
    None,
}

impl GridKey {
    pub fn as_char(&self) -> char {
        match *self {
            GridKey::X => 'X',
            GridKey::O => 'O',
            GridKey::None => ' ',
        }
    }
}


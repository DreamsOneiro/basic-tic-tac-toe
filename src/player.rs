pub use crate::key;

pub enum Player {
    P1,
    P2,
}

impl Player {
    pub fn get_key(&self) -> key::GridKey {
        match self {
            Player::P1 => key::GridKey::O,
            Player::P2 => key::GridKey::X,
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Player::P1 => 'O',
            Player::P2 => 'X',
        }
    }

    pub fn get_num(&self) -> usize {
        match self {
            Player::P1 => 1,
            Player::P2 => 2,
        }
    }
}

pub fn select_player(turn: &usize) -> Player {
    if turn % 2 == 0 {
        Player::P1
    } else {
        Player::P2
    }
}


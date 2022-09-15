#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    #[default]
    X,
    O,
}

impl Player {
    pub fn to_string(self) -> String {
        match self {
            Player::X => "X".to_string(),
            Player::O => "O".to_string(),
        }
    }
}

impl Player {
    pub fn next(self) -> Self {
        if self == Player::X {
            Player::O
        } else {
            Player::X
        }
    }
}

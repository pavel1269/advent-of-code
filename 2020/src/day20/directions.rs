#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Directions {
    Up,
    Right,
    Down,
    Left,
}

impl Directions {
    pub fn index(&self) -> usize {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }

    pub fn match_offset(&self) -> usize {
        self.index() + 2
    }

    pub fn count() -> usize {
        4
    }
}

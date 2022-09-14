use ufmt::derive::uDebug;
#[derive(Debug, uDebug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left, 
    Right
}

impl ufmt::uDisplay for Direction {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized {
        ufmt::uwrite!(f, "{:?}", self)
    }
}

impl Direction {
    pub fn get_opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn is_opposing(&self, rhs: Self) -> bool {
        self.get_opposite() == rhs
    }
}

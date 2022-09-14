use super::direction::Direction;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position<const X_LIM: isize, const Y_LIM: isize> {
    x: u8,
    y: u8,
}

impl<const X_LIM: isize, const Y_LIM: isize> ufmt::uDebug for Position<X_LIM, Y_LIM> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized {
        ufmt::uwrite!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl<const X_LIM: isize, const Y_LIM: isize> ufmt::uDisplay for Position<X_LIM, Y_LIM> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized {
        ufmt::uwrite!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl<const X_LIM: isize, const Y_LIM: isize> Position<X_LIM, Y_LIM> {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    pub fn offset(&self, x: isize, y: isize) -> Self {
        let x = x + (self.x as isize);
        let x = if x >= X_LIM {
            x % X_LIM
        } else if x < 0 {
            X_LIM + x
        } else {
            x
        };
        let y = y + (self.y as isize);
        let y = if y >= Y_LIM {
            y % Y_LIM
        } else if y < 0 {
            Y_LIM + y
        } else {
            y
        };
        Self {
            x: x as u8,
            y: y as u8,
        }
    }

    pub fn offset_dir(&self, dir: Direction) -> Self {
        self.offset_dir_scaled(dir, 1)
    }

    pub fn offset_dir_scaled(&self, dir: Direction, scale: isize) -> Self {
        match dir {
            Direction::Up => self.offset(0, scale),
            Direction::Down => self.offset(0, -scale),
            Direction::Left => self.offset(-scale, 0),
            Direction::Right => self.offset(scale, 0),
        }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }
}

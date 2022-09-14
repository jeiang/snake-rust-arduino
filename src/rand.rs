use crate::game::{position::Position, direction::Direction};
use picorand::{WyRand, RNG};

pub struct RandomGenerator {
    rg: RNG<WyRand, u8>,
}

impl core::ops::DerefMut for RandomGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rg
    }
}

impl core::ops::Deref for RandomGenerator {
    type Target = RNG<WyRand, u8>;

    fn deref(&self) -> &Self::Target {
        &self.rg
    }
}

impl RandomGenerator {
    pub fn new(seed: u64) -> Self {
        Self { 
            rg: RNG::<WyRand, u8>::new(seed)
        }
    }
}

pub trait Random {
    fn random(rand_gen: &mut RandomGenerator) -> Self;
}

impl<const X_LIM: isize, const Y_LIM: isize> Random for Position<X_LIM, Y_LIM> {
    fn random(rand_gen: &mut RandomGenerator) -> Self {
        let x = rand_gen.generate_range(1, X_LIM as usize) - 1;
        let y = rand_gen.generate_range(2, Y_LIM as usize) - 1;
        Self::new(x, y)
    }
}

impl Random for Direction {
    fn random(rand_gen: &mut RandomGenerator) -> Self {
        let i = rand_gen.generate_range(0, 3);
        match i {
            0 => Self::Up,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Right,
            _ => panic!("rand value % 4 >= 4")
        }
    }
}

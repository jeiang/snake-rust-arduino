pub mod direction;
pub mod position;
mod ringbuffer;
pub mod snake;

use ufmt::{uDisplay, uwrite, uwriteln};

use crate::rand::{Random, RandomGenerator};

use self::{direction::Direction, position::Position, snake::Snake};

const SNAKE_LEN: usize = 20;
const X_LIM: isize = 8;
const Y_LIM: isize = 8;

pub enum Command {
    Reset,
    Move(Direction),
}

pub enum GameResult {
    Continue,
    Died,
    Won,
    Restarting,
}

pub struct Game {
    snake: Snake<SNAKE_LEN, X_LIM, Y_LIM>,
    apple: Position<X_LIM, Y_LIM>,
    rand_gen: RandomGenerator,
}

impl Game {
    pub fn new(mut rand_gen: RandomGenerator) -> Self {
        let apple = Position::random(&mut rand_gen);
        Self {
            snake: Snake::new(
                Position::random(&mut rand_gen),
                3,
                Direction::random(&mut rand_gen),
            ),
            apple,
            rand_gen,
        }
    }

    pub fn step(&mut self, cmd: Command) -> GameResult {
        match cmd {
            Command::Reset => {
                self.reset();
                GameResult::Restarting
            }
            Command::Move(dir) => match self.snake.move_dir(dir, self.apple) {
                snake::MovementResult::BitSelf => {
                    self.reset();
                    GameResult::Died
                }
                snake::MovementResult::Moving => GameResult::Continue,
                snake::MovementResult::AteApple => {
                    // self.apple = {
                    //     while self.snake.check_overlap(&self.apple) {
                    //         self.apple = Position::random(&mut self.rand_gen);
                    //     }
                    //     self.apple
                    // };
                    self.apple = Position::random(&mut self.rand_gen);

                    GameResult::Continue
                }
                snake::MovementResult::AteAppleAndMaxed => {
                    self.reset();
                    GameResult::Won
                }
            },
        }
    }

    pub fn reset(&mut self) {
        self.snake = Snake::new(
            Position::random(&mut self.rand_gen),
            3,
            Direction::random(&mut self.rand_gen),
        );
        self.apple = Position::random(&mut self.rand_gen);
    }

    pub fn iter_snake(&self) -> ringbuffer::RingBufferIter<Position<X_LIM, Y_LIM>, SNAKE_LEN> {
        self.snake.iter()
    }

    pub fn apple(&self) -> Position<X_LIM, Y_LIM> {
        self.apple
    }

    pub fn last_snake_tail(&self) -> &Position<X_LIM, Y_LIM> {
        self.snake.snake_tail()
    }
}

impl uDisplay for Game {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        let mut array = [['-'; 8]; 8];

        array[self.apple.y() as usize][self.apple.x() as usize] = 'A';

        for i in self.snake.iter() {
            array[i.y() as usize][i.x() as usize] = 'S';
        }
        for row in array.iter().rev() {
            for col in row.iter() {
                uwrite!(f, "{} ", col)?;
            }
            uwriteln!(f, "")?;
        }

        Ok(())
    }
}

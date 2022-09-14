use super::ringbuffer::{RingBuffer, RingBufferIter};

use super::{direction::Direction, position::Position};

#[derive(Debug)]
pub struct Snake<const SNAKE_LEN: usize, const X_LIM: isize, const Y_LIM: isize> {
    body: RingBuffer<Position<X_LIM, Y_LIM>, SNAKE_LEN>,
    current_dir: Direction,
    snake_tail: Position<X_LIM, Y_LIM>,
}

pub enum MovementResult {
    BitSelf,
    Moving,
    AteApple,
    AteAppleAndMaxed,
}

impl<const SNAKE_LEN: usize, const X_LIM: isize, const Y_LIM: isize>
    Snake<SNAKE_LEN, X_LIM, Y_LIM>
{
    pub fn new(start_pos: Position<X_LIM, Y_LIM>, len: u8, dir: Direction) -> Self {
        let mut body = RingBuffer::new();
        for i in 0..(len as isize) {
            body.push(start_pos.offset_dir_scaled(dir, i));
        }
        Self {
            body,
            current_dir: dir,
            snake_tail: Default::default(),
        }
    }

    pub fn move_dir(
        &mut self,
        dir: Direction,
        apple_pos: Position<X_LIM, Y_LIM>,
    ) -> MovementResult {
        let dir = if dir.is_opposing(self.current_dir) {
            dir.get_opposite()
        } else {
            dir
        };

        let head = self.body.peek_back().unwrap().offset_dir(dir);
        if self.check_overlap(&head) {
            return MovementResult::BitSelf;
        }
        self.body.push(head);
        self.current_dir = dir;
        if apple_pos == head {
            if self.body.is_full() {
                MovementResult::AteAppleAndMaxed
            } else {
                MovementResult::AteApple
            }
        } else {
            if let Some(pos) = self.body.pop() {
                self.snake_tail = pos
            }
            MovementResult::Moving
        }
    }

    #[inline]
    pub fn check_overlap(&self, apple: &Position<X_LIM, Y_LIM>) -> bool {
        self.body.iter().any(|pos| pos == apple)
    }

    pub fn iter(&self) -> RingBufferIter<'_, Position<X_LIM, Y_LIM>, SNAKE_LEN> {
        self.body.iter()
    }

    // pub fn current_dir(&self) -> Direction {
    //     self.current_dir
    // }

    pub fn snake_tail(&self) -> &Position<X_LIM, Y_LIM> {
        &self.snake_tail
    }
}

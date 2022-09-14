#[derive(Debug)]
pub struct RingBuffer<T, const CAP: usize>
where
    T: Default + Copy,
{
    data: [T; CAP],
    start_idx: usize,
    end_idx: usize,
    empty: bool,
}

impl<T, const CAP: usize> RingBuffer<T, CAP>
where
    T: Default + Copy,
{
    #[inline]
    fn inc_wrap(value: usize) -> usize {
        Self::idx_offset(value, 1)
    }

    #[inline]
    fn idx_offset(value1: usize, value2: usize) -> usize {
        (value1 + value2) % CAP
    }

    pub fn new() -> Self {
        let data = [Default::default(); CAP];
        Self {
            data,
            start_idx: 0,
            end_idx: 0,
            empty: true,
        }
    }

    pub fn push(&mut self, value: T) -> bool {
        if self.end_idx == self.start_idx && !self.empty {
            false // buffer full
        } else {
            self.data[self.end_idx] = value;
            self.end_idx = Self::inc_wrap(self.end_idx);
            self.empty = false;
            true
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.empty {
            None
        } else {
            let data = self.data[self.start_idx];
            self.start_idx = Self::inc_wrap(self.start_idx);
            if self.start_idx == self.end_idx {
                self.empty = true;
            }
            Some(data)
        }
    }

    pub fn is_full(&self) -> bool {
        !self.empty && self.start_idx == self.end_idx
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn clear(&mut self) {
        self.start_idx = 0;
        self.end_idx = 0;
        self.empty = true;
    }

    pub fn len(&self) -> usize {
        if self.empty {
            0
        } else if self.start_idx == self.end_idx {
            CAP
        } else if self.start_idx < self.end_idx {
            self.end_idx - self.start_idx
        } else {
            CAP - (self.start_idx - self.end_idx)
        }
    }

    pub fn cap(&self) -> usize {
        CAP
    }

    pub fn iter(&self) -> RingBufferIter<'_, T, CAP> {
        RingBufferIter {
            buf: self,
            idx: 0,
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.len() {
            let idx = Self::idx_offset(self.start_idx, idx);
            Some(&self.data[idx])
        } else {
            None
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn peek_back(&self) -> Option<&T> {
        if !self.is_empty() {
            self.get(self.len() - 1)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RingBufferIter<'a, T, const CAP: usize>
where
    T: Default + Copy,
{
    buf: &'a RingBuffer<T, CAP>,
    idx: usize,
}

impl<'a, T, const CAP: usize> Iterator for RingBufferIter<'a, T, CAP>
where
    T: Default + Copy,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.buf.get(self.idx)?;
        self.idx += 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.idx,
            Some(if self.buf.len() < self.idx {
                0
            } else {
                self.buf.len() - self.idx
            }),
        )
    }
}

#[cfg(test)]
mod test {
    use super::RingBuffer;

    #[test]
    fn push_pop_with_no_wrap() {
        let mut buf = RingBuffer::<i32, 10>::new();
        assert_eq!(buf.start_idx, buf.end_idx);
        buf.push(1);
        assert_eq!(buf.start_idx + 1, buf.end_idx);
        buf.push(2);
        assert_eq!(buf.start_idx + 2, buf.end_idx);
        buf.push(3);
        assert_eq!(buf.start_idx + 3, buf.end_idx);
        buf.push(4);
        assert_eq!(buf.start_idx + 4, buf.end_idx);
        println!("{buf:?}");
        let mut last = buf.pop();
        assert_eq!(last, Some(1));
        last = buf.pop();
        assert_eq!(last, Some(2));
        last = buf.pop();
        assert_eq!(last, Some(3));
        last = buf.pop();
        assert_eq!(last, Some(4));
        last = buf.pop();
        assert_eq!(last, None);
        println!("{buf:?}");
    }

    #[test]
    fn push_pop_with_wrap() {
        let mut buf = RingBuffer::<i32, 3>::new();
        assert!(buf.push(1));
        assert!(buf.push(2));
        assert!(buf.push(3));
        assert!(!buf.push(4));
        println!("{buf:?}");
        assert_eq!(buf.pop(), Some(1));
        assert_eq!(buf.pop(), Some(2));
        assert_eq!(buf.pop(), Some(3));
        assert_eq!(buf.pop(), None);
    }

    #[test]
    fn cap() {
        let buf = RingBuffer::<i32, 4>::new();
        assert_eq!(buf.cap(), 4);
        let buf = RingBuffer::<i8, 4>::new();
        assert_eq!(buf.cap(), 4);
        let buf = RingBuffer::<bool, 10>::new();
        assert_eq!(buf.cap(), 10);
        let buf = RingBuffer::<&str, 40>::new();
        assert_eq!(buf.cap(), 40);
        let buf = RingBuffer::<(i8, i8), 256>::new();
        assert_eq!(buf.cap(), 256);
        let buf = RingBuffer::<i8, 0>::new();
        assert_eq!(buf.cap(), 0);
    }

    #[test]
    fn len() {
        let mut buf = RingBuffer::<i32, 3>::new();
        assert_eq!(buf.len(), 0);
        buf.push(1);
        assert_eq!(buf.len(), 1);
        buf.push(2);
        assert_eq!(buf.len(), 2);
        buf.push(3);
        assert_eq!(buf.len(), 3);
        buf.push(4);
        assert_eq!(buf.len(), 3);
        buf.pop();
        assert_eq!(buf.len(), 2);
        buf.pop();
        assert_eq!(buf.len(), 1);
        buf.pop();
        assert_eq!(buf.len(), 0);
        buf.pop();
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn is_full() {
        let mut buf = RingBuffer::<i32, 3>::new();
        assert!(!buf.is_full());
        buf.push(1);
        buf.push(2);
        buf.push(3);
        assert!(buf.is_full());
        buf.push(4);
        assert!(buf.is_full());
        buf.pop();
        assert!(!buf.is_full());
        buf.pop();
        buf.pop();
        buf.pop();
        assert!(!buf.is_full());
    }

    #[test]
    fn is_empty() {
        let mut buf = RingBuffer::<i32, 3>::new();
        assert!(buf.is_empty());
        buf.push(1);
        assert!(!buf.is_empty());
        buf.push(2);
        buf.push(3);
        buf.push(4);
        assert!(!buf.is_empty());
        buf.pop();
        assert!(!buf.is_empty());
        buf.pop();
        buf.pop();
        assert!(buf.is_empty());
        buf.pop();
        assert!(buf.is_empty());
    }

    #[test]
    fn clear() {
        let mut buf = RingBuffer::<i32, 3>::new();
        buf.push(1);
        buf.push(2);
        buf.push(3);
        buf.clear();
        println!("{buf:?}");
        assert!(buf.is_empty());
    }

    #[test]
    fn iter() {
        let mut buf = RingBuffer::<usize, 3>::new();
        buf.push(1);
        buf.push(2);
        buf.push(3);
        for (&buf_val, test_val) in buf.iter().zip(1..=3) {
            assert_eq!(buf_val, test_val);
        }
    }

    #[test]
    fn size_hint() {
        let mut buf = RingBuffer::<usize, 3>::new();
        buf.push(1);
        buf.push(2);
        buf.push(3);
        let mut iter = buf.iter();
        assert_eq!(iter.size_hint(), (0, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(1)));
        iter.next();
        assert_eq!(iter.size_hint(), (3, Some(0)));
        iter.next();
        assert_eq!(iter.size_hint(), (3, Some(0)));
    }

    #[test]
    fn peek_front() {
        let mut buf = RingBuffer::<i32, 3>::new();
        buf.push(1);
        buf.push(2);
        buf.push(3);
        assert_eq!(buf.peek_front(), Some(&1));
        buf.pop();
        assert_eq!(buf.peek_front(), Some(&2));
        buf.pop();
        assert_eq!(buf.peek_front(), Some(&3));
        buf.pop();
        assert_eq!(buf.peek_front(), None);
        buf.pop();
    }

    #[test]
    fn peek_back() {
        let mut buf = RingBuffer::<i32, 3>::new();
        assert_eq!(buf.peek_back(), None);
        buf.push(1);
        assert_eq!(buf.peek_back(), Some(&1));
        buf.push(2);
        assert_eq!(buf.peek_back(), Some(&2));
        buf.push(3);
        assert_eq!(buf.peek_back(), Some(&3));
    }
}

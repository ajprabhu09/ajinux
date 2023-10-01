pub struct RingBuf<T, const SIZE: usize> {
    pub start: usize,
    pub end: usize,
    pub len: usize,
    pub buf: [Option<T>; SIZE],
}

impl<T, const SIZE: usize> RingBuf<T, SIZE>
where
    T: Copy + Sized,
{
    pub const fn new() -> Self {
        Self {
            buf: [None; SIZE],
            start: 0,
            end: 0,
            len: 0,
        }
    }

    pub const fn empty(&self) -> bool {
        return self.len == 0;
    }
    pub const fn full(&self) -> bool {
        return self.len == SIZE;
    }

    pub fn push(&mut self, data: T) -> Result<(), &'static str> {
        if self.full() {
            return Err("full buffer");
        }
        self.buf[self.end] = Some(data);
        self.end += 1;
        self.len += 1;

        if self.end >= SIZE {
            self.end %= SIZE;
        }

        Ok(())
    }

    pub fn take(&mut self) -> Option<T> {
        if self.empty() {
            return None;
        }
        let head = self.buf[self.start];
        if head.is_none() {
            return None;
        }

        let head = head.unwrap();

        self.start += 1;
        self.len -= 1;

        if self.start >= SIZE {
            self.start %= SIZE;
        }

        return Some(head);
    }
}

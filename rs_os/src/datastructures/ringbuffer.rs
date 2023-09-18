

pub struct RingBuf<T,const SIZE: usize> {
    pub next_free: usize,
    pub len: usize,
    pub buf: [T; SIZE]
}

impl<T, const SIZE: usize> RingBuf<T,SIZE> where T: Copy + Sized + Default{

    pub fn new() -> Self{
        Self {
            buf: [T::default(); SIZE],
            next_free: 0,
            len: 0,
        }
    }

    pub const fn empty(&self) -> bool {
        return self.len == 0;
    }
    pub const fn full(&self) -> bool {
        return self.len == SIZE
    }

    pub fn push(&mut self, data: T) -> Result<(), &'static str> {

        if !self.full() {
            self.buf[self.next_free] = data;
            self.len += 1;
            self.next_free += 1;
            Ok(())
        } else {


            Ok(())
        }

    }

    
}

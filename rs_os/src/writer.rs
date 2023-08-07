

use core::borrow::BorrowMut;
use core::cell::{RefCell, Ref};
use core::{cell::UnsafeCell, fmt::Write};
use core::ops::{Deref, DerefMut};

use crate::vga::{ConsoleDisplay, VGADisplay, Color};
pub struct Writer<T: ConsoleDisplay> {
    pub display: RefCell<T>
}


impl core::fmt::Write for VGADisplay {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.put_bytes(s.as_bytes()).map_err(|err| {
            panic!("{:?}", err);
        })
    }
}





lazy_static::lazy_static! {
    pub static ref WRITER: Writer<VGADisplay> = Writer{
        display: RefCell::from(VGADisplay::default()),
    };
}

pub fn set_color(color: u8) {
    WRITER.display.borrow_mut().set_term_color(color);
}


unsafe impl Sync for Writer<VGADisplay> {

}

use crate::utils::asm::{self, iodelay};

pub struct Port(pub u16);

impl Port {
    pub fn send_byte(&self, val: u8) {
        unsafe {
            asm::outb(self.0, val);
            iodelay();
        }
    }

    pub fn read_byte(&self) -> u8 {
        unsafe { asm::inb(self.0) }
    }
}

use crate::utils::asm::{self, iodelay};

pub struct Port(u16);

#[allow(dead_code)]
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

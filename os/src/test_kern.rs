use core::panic::PanicInfo;

use crate::{info, interrupts, utils::asm, serial_info, io::writer::set_color, devices::vga::Color, kprint};


// This function is called on panic.


#[cfg(test)]


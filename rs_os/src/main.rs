#![no_std]
#![no_main]

mod asm;
mod vga;
use core::panic::PanicInfo;

use vga::{ConsoleDisplay, Text, DEFAULT_BG_COLOR, DEFAULT_FG_COLOR};
static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut display = vga::VGADisplay::default();
    display.put_bytes("ERROR: paniced at main".as_bytes());
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut display = vga::VGADisplay::default();
    // display.put_bytes("LOL".as_bytes());
    // display.put_byte(unsafe { asm::add(60) } as u8);
    display.put_bytes("HEllow\n \tWorld".as_bytes());
    display.hide_cursor();
    display.restore_cursor();
    loop {}
}

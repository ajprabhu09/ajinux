#![no_std]
#![no_main]
mod writer;


mod asm;
mod vga;
mod sync;
use core::{panic::PanicInfo, fmt::Write, ops::DerefMut};
mod mystd;

use mystd::vga_print::*;
use sync::spinlock::Mutex;

use lazy_static::__Deref;
use vga::{ConsoleDisplay, Text, DEFAULT_BG_COLOR, DEFAULT_FG_COLOR, BUFFER_WIDTH};
use writer::WRITER;

use crate::{writer::set_color, vga::{Color, delay}};
static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    set_color(Color::pack(Color::Black, Color::Red));
    print!("{}", _info);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // write!(WRITER.deref_mut(), "lol");
    let a = "1231".parse::<i32>();


    for _ in 0..BUFFER_WIDTH {
        print!("1111\x08");
    }

    a.unwrap();
    loop {}
}

#![no_std]
#![no_main]
mod writer;

mod asm;
mod sync;
mod vga;
use core::{fmt::Write, ops::DerefMut, panic::PanicInfo};
mod logging;

use logging::vga_print::*;
use logging::vga_log;
use sync::spinlock::Mutex;

use lazy_static::__Deref;
use vga::{ConsoleDisplay, Text, BUFFER_HEIGHT, BUFFER_WIDTH, DEFAULT_BG_COLOR, DEFAULT_FG_COLOR};
use writer::WRITER;

use crate::{
    vga::{delay, Color},
    writer::set_color,
};
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

    for _ in 1..1000 {
        info!("Starting ....");
    }

    a.unwrap();
    loop {}
}

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]

mod descriptors;
mod devices;
mod interrupts;
mod sync;
mod utils;

use core::{fmt::Write, ops::DerefMut, panic::PanicInfo};
mod logging;

use logging::vga_log;
use logging::vga_print::*;
use logging::writer;
use sync::spinlock::Mutex;

use devices::vga::{
    ConsoleDisplay, Text, BUFFER_HEIGHT, BUFFER_WIDTH, DEFAULT_BG_COLOR, DEFAULT_FG_COLOR,
};
use writer::WRITER;

use descriptors::idt;

use crate::devices::pic::pic_init;
use crate::{
    devices::vga::{delay, Color},
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

    info!("Starting kernel");
    #[cfg(test)]
    test_main();

    kernel_main();
    loop {}
}

pub fn kernel_main() {
    interrupts::interrupt_setup();

    // unsafe { utils::asm::enable_interrupts() }; // this fails if no handler is installed
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![allow(clippy::empty_loop)]
#![allow(clippy::needless_return)]
mod addr;
mod descriptors;
mod devices;
mod interrupts;
mod sync;
mod utils;
use core::panic::PanicInfo;
mod logging;

use logging::writer;
use utils::asm;

use crate::{devices::vga::Color, writer::set_color};


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

#[test_case]
fn test_breakpoint() {
    interrupts::setup::interrupt_setup();
    // unsafe { utils::asm::enable_interrupts() }; // this fails if no handler is installed
    unsafe { asm::int3() };
    info!("Breakpoint interrupt tested");
}

pub fn kernel_main() {
    interrupts::setup::interrupt_setup();
    unsafe { utils::asm::enable_interrupts() }; // this fails if no handler is installed
    // unsafe { asm::int3() };
    info!("Breakpoint interrupt tested");

}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

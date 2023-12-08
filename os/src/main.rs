#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_kern::test_runner)]
// #![reexport_test_harness_main = "test_main"]
#![feature(allocator_api)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![allow(clippy::empty_loop)]
#![allow(clippy::needless_return)]
#![feature(let_chains)]
extern crate alloc;
use alloc::vec::Vec;

use bootloader::BootInfo;
// extern crate alloc;
use kernel::{interrupts::timer::PIT_, io::reader::READER, *};
use user_tests::bytes::simple;


bootloader::entry_point!(kernel_main);


pub fn kernel_main(bootinfo: &'static BootInfo) -> ! {

    setup_boot_info(bootinfo);
    kprint!("\n\n");
    utils::asm::disable_interrupts(); // this fails if no handler is installed
    PIT_.setup(10);
    interrupts::setup::interrupt_setup();
    utils::asm::enable_interrupts(); // this fails if no handler is installed
    discover_pages();

    // WRITER.take().display.clear();
    loop {
        READER.take().input.process_buf_wait();
    }
}

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_kern::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![allow(clippy::empty_loop)]
#![allow(clippy::needless_return)]
#![feature(let_chains)]

// extern crate alloc;
pub mod test_kern;
pub mod addr;
pub mod descriptors;
pub mod devices;
pub mod interrupts;
pub mod sync;
pub mod utils;
pub use core::{fmt::Write, panic::PanicInfo, alloc::GlobalAlloc, ptr::{null, null_mut}};

use crate::{devices::vga::Color, io::writer::set_color};
pub mod datastructures;
pub mod io;
pub mod logging;
pub mod allocator;
pub mod cc;

// This function is called on panic.
// #[panic_handler]
// pub fn panic(_info: &PanicInfo) -> ! {
//     set_color(Color::pack(Color::Black, Color::Red));
//     print!("{}", _info);
//     loop {}
// }
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![allow(clippy::empty_loop)]
#![allow(clippy::needless_return)]
#![feature(let_chains)]

// extern crate alloc;
pub mod addr;
pub mod descriptors;
pub mod devices;
pub mod interrupts;
pub mod sync;

pub mod utils;
pub use core::{
    alloc::GlobalAlloc,
    fmt::Write,
    panic::PanicInfo,
    ptr::{null, null_mut},
};

use crate::{devices::vga::Color, io::writer::set_color};
pub mod allocator;
pub mod cc;
pub mod datastructures;
pub mod io;
pub mod logging;
pub mod paging;

// This function is called on panic.
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    set_color(Color::pack(Color::Black, Color::Red));
    kprint!("{}", _info);
    loop {}
}
#[cfg(test)]
mod test {
    pub fn test_runner(tests: &[&dyn Fn()]) {
        use crate::kprintln;
        kprintln!("Running {} tests", tests.len());
        for test in tests {
            test();
        }
    }




use bootloader::{bootinfo::MemoryRegionType, BootInfo};
// extern crate alloc;
use kernel::{
    allocator::page_alloc::PageAlloc,
    devices::{
        pit::PIT,
        vga::{Color, ConsoleDisplay},
    },
    io::{
        reader::READER,
        writer::{set_color, WRITER},
    },
    *, interrupts::timer::PIT_,
};

use crate::{kprint, utils, interrupts, kprintln, ksprint};

bootloader::entry_point!(kernel_main);



static mut ALLOC: PageAlloc<4096> = PageAlloc::default();

pub fn kernel_main(bootinfo: &'static BootInfo) -> ! {
    kprint!("\n");
    unsafe { utils::asm::disable_interrupts() }; // this fails if no handler is installed

    unsafe { PIT_.setup(10) };

    interrupts::setup::interrupt_setup();
    unsafe { utils::asm::enable_interrupts() }; // this fails if no handler is installed

    let usable_regions = bootinfo
        .memory_map
        .iter()
        .filter(|region| region.region_type == MemoryRegionType::Usable);

    // println!("{:#?}", bootinfo.physical_memory_offset as *mut ());
    for region in usable_regions {
        kprintln!("Setting up apges in region {:?}", region);
        unsafe {
            ALLOC.add_region(
                region.range.start_addr() + bootinfo.physical_memory_offset,
                region.range.end_addr() + bootinfo.physical_memory_offset,
            )
        };
    }

    // WRITER.take().display.clear();
    ksprint!("hello");
    loop {
        READER.take().input.process_buf_wait();
    }
}



}

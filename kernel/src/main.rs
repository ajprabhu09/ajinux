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
mod addr;
mod descriptors;
mod devices;
mod interrupts;
mod sync;
mod utils;
use core::{fmt::Write, panic::PanicInfo};
mod datastructures;
mod io;
mod logging;
use bootloader::{
    bootinfo::{MemoryRegion, MemoryRegionType},
    BootInfo,
};
use utils::asm;
mod allocator;
mod cc;
use crate::{
    devices::{keyboard::ConsoleInput, vga::Color},
    io::{
        reader::READER,
        writer::{set_color, WRITER},
    },
    sync::spinlock::Mutex, allocator::page_alloc::PageAlloc,
};
mod test_kern;

bootloader::entry_point!(kernel_main);

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    set_color(Color::pack(Color::Black, Color::Red));
    print!("{}", _info);
    loop {}
}

pub fn kernel_main(bootinfo: &'static BootInfo) -> ! {
    info!("Starting kernel");
    #[cfg(test)]
    test_main();

    interrupts::setup::interrupt_setup();
    unsafe { utils::asm::enable_interrupts() }; // this fails if no handler is installed
                                                // unsafe { asm::int3() };
    let val = unsafe { cc::func() };
    let mut allocator: PageAlloc<4096> = PageAlloc::default();

    let usable_regions = bootinfo
        .memory_map
        .iter()
        .filter(|region| region.region_type == MemoryRegionType::Usable);


    for region in usable_regions {
        println!("Setting up apges in region {:?}", region);
        // BUG: Adding a region causes a segment error
        allocator.add_region(region.range.start_addr(), region.range.end_addr());
    }

    loop {}
}

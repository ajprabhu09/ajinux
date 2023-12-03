#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_kern::test_runner)]
// #![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![allow(clippy::empty_loop)]
#![allow(clippy::needless_return)]
#![feature(let_chains)]

use bootloader::{bootinfo::MemoryRegionType, BootInfo};
// extern crate alloc;
use kernel::{allocator::page_alloc::PageAlloc, interrupts::timer::PIT_, io::reader::READER, *};

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
        serial_info!("Setting up apges in region {:?}", region);
        unsafe {
            ALLOC.add_region(
                region.range.start_addr() + bootinfo.physical_memory_offset,
                region.range.end_addr() + bootinfo.physical_memory_offset,
            )
        };
    }

    // WRITER.take().display.clear();
    loop {
        READER.take().input.process_buf_wait();
    }
}

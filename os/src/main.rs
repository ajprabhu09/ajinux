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

use bootloader::{BootInfo, bootinfo::MemoryRegionType};
// extern crate alloc;
use kernel::{*, io::{reader::READER, writer::{set_color, WRITER}}, devices::vga::{Color, ConsoleDisplay}, allocator::page_alloc::PageAlloc};

bootloader::entry_point!(kernel_main);

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    set_color(Color::pack(Color::Black, Color::Red));
    kprint!("{}", _info);
    loop {}
}


static mut ALLOC: PageAlloc<4096> = PageAlloc::default();


pub fn kernel_main(bootinfo: &'static BootInfo) -> ! {
    info!("Starting kernel");

    interrupts::setup::interrupt_setup();
    unsafe { utils::asm::enable_interrupts() }; // this fails if no handler is installed

    let usable_regions = bootinfo
        .memory_map
        .iter()
        .filter(|region| region.region_type == MemoryRegionType::Usable);

    // println!("{:#?}", bootinfo.physical_memory_offset as *mut ());
    for region in usable_regions {
        kprintln!("Setting up apges in region {:?}", region);
        unsafe { ALLOC.add_region(
                region.range.start_addr() + bootinfo.physical_memory_offset,
                 region.range.end_addr() + bootinfo.physical_memory_offset) };
    }

    WRITER.take().display.clear();


    loop {
        READER.take().input.process_buf_wait();
    }

}

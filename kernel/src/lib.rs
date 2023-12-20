#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(allocator_api)]
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

pub mod loader;
pub mod utils;

pub use core::{
    alloc::GlobalAlloc,
    fmt::Write,
    panic::PanicInfo,
    ptr::{null, null_mut},
};

use bootloader::{
    bootinfo::{self, MemoryRegionType},
    BootInfo,
};

use crate::{allocator::kernel_alloc::PAGE_ALLOC, devices::vga::Color, io::writer::set_color};
pub mod allocator;
pub mod cc;
pub mod datastructures;
pub mod io;
pub mod logging;
pub mod paging;
pub mod process;

pub static mut BOOT_INFO: Option<&'static BootInfo> = None;

pub fn setup_boot_info(val: &'static BootInfo) {
    unsafe { BOOT_INFO = Some(val) };
}
pub fn physical_memory_offset_val() -> u64 {
    return unsafe { BOOT_INFO.unwrap().physical_memory_offset };
}

pub fn discover_pages() {
    let bootinfo = unsafe { BOOT_INFO.unwrap() };
    let usable_regions = bootinfo
        .memory_map
        .iter()
        .filter(|region| region.region_type == MemoryRegionType::Usable);

    for region in usable_regions {
        serial_info!("Setting up apges in region {:?}", region);
        unsafe {
            PAGE_ALLOC.add_region(
                region.range.start_addr() + bootinfo.physical_memory_offset,
                region.range.end_addr() + bootinfo.physical_memory_offset,
            )
        };
    }
}

// This function is called on panic.
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    set_color(Color::pack(Color::Black, Color::Red));
    ksprintln!("{}", _info);
    loop {}
}

#[cfg(test)]
mod test {
    pub fn test_runner(tests: &[&dyn Fn()]) {
        use crate::kprintln;
        serial_info!("Running {} tests", tests.len());
        for test in tests {
            test();
        }
    }

    use bootloader::{bootinfo::MemoryRegionType, BootInfo};
    // extern crate alloc;
    use crate::{
        allocator::page_alloc::PageAlloc,
        devices::{
            pit::PIT,
            vga::{Color, ConsoleDisplay},
        },
        interrupts::timer::PIT_,
        io::{
            reader::READER,
            writer::{set_color, WRITER},
        },
        *,
    };

    use crate::{interrupts, kprint, kprintln, ksprint, utils};

    bootloader::entry_point!(kernel_main);

    static mut ALLOC: PageAlloc<4096> = PageAlloc::default();

    pub fn kernel_main(bootinfo: &'static BootInfo) -> ! {
        unsafe { utils::asm::disable_interrupts() }; // this fails if no handler is installed

        unsafe { PIT_.setup(10) };

        test_main();

        interrupts::setup::interrupt_setup();
        unsafe { utils::asm::enable_interrupts() }; // this fails if no handler is installed

        let usable_regions = bootinfo
            .memory_map
            .iter()
            .filter(|region| region.region_type == MemoryRegionType::Usable);

        // println!("{:#?}", bootinfo.physical_memory_offset as *mut ());
        for region in usable_regions {
            ksprintln!("Setting up apges in region {:?}", region);
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
}

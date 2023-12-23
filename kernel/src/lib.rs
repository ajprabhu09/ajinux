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

use bootloader::{bootinfo::MemoryRegionType, BootInfo};

use crate::{allocator::physical_mem_manager::PHY_MM, devices::vga::Color, io::writer::set_color};
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

    // BUG: LMM will not work if this is not done
    for region in usable_regions {
        serial_info!("Setting up Manager in region {:?}", region);
        unsafe {
            PHY_MM.add_region(
                region.range.start_addr() + physical_memory_offset_val(),
                region.range.end_addr() + physical_memory_offset_val(),
            )
        };

        // unsafe {
        //     LMM_ALLOC.add_region(
        //         region.range.start_addr() + bootinfo.physical_memory_offset,
        //         region.range.end_addr() + bootinfo.physical_memory_offset,
        //     )
        // };
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
        serial_info!("Running {} tests", tests.len());

        for test in tests {
            serial_info!("===============================");
            test();
            serial_info!("-------------------------------")
        }
    }

    use bootloader::BootInfo;
    // extern crate alloc;
    use crate::{interrupts::timer::PIT_, io::reader::READER, *};

    use crate::{interrupts, kprint, utils};

    bootloader::entry_point!(kernel_main);

    // static mut ALLOC: PageAlloc<4096> = PageAlloc::default();

    pub fn kernel_main(bootinfo: &'static BootInfo) -> ! {
        setup_boot_info(bootinfo);
        kprint!("\n\n");
        utils::asm::disable_interrupts(); // this fails if no handler is installed
        PIT_.setup(10);
        interrupts::setup::interrupt_setup();
        utils::asm::enable_interrupts(); // this fails if no handler is installed
        discover_pages();
        test_main();

        // WRITER.take().display.clear();
        loop {
            READER.take().input.process_buf_wait();
        }
    }
}

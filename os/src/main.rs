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

use alloc::vec;
use alloc::{collections::binary_heap, vec::Vec};
use bootloader::BootInfo;
use elfloader::*;
use kernel::allocator::kernel_alloc::PAGE_SIZE;
// extern crate alloc;
use kernel::loader::*;
use kernel::{
    addr::VirtAddr,
    allocator::kernel_alloc::{KERNEL_ALLOC, LMM_ALLOC},
    descriptors::reg::{GetReg, CR0, CR3},
    interrupts::timer::PIT_,
    io::reader::READER,
    loader::UserTestLoader,
    paging::{Table, TableEntry},
    *,
};
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

    let page_table = Table::from_addr::<512>(VirtAddr::as_canonical(CR3::get_reg()));
    let entries = page_table.entries.iter().filter(|x| x.present());
    // // serial_info!("page: {:?}", page);
    for entry in entries.enumerate() {
        serial_info!("ptr - {:?}  - {:?}", (entry.1 as *const TableEntry), entry);
    }

    let vec = vec![10; 1 << 14];

    loop {
        READER.take().input.process_buf_wait();
    }
}

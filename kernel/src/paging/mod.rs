use core::fmt::Debug;

use bitfield_struct::bitfield;

use crate::{addr::VirtAddr, allocator::kernel_alloc::PAGE_SIZE, serial_info};
extern crate static_assertions as sa;

#[bitfield(u64, debug = false)]
pub struct TableEntry {
    #[bits(1)]
    pub present: bool, // 1

    #[bits(1)]
    pub writable: bool, // 2

    #[bits(1)]
    pub user: bool, // 3

    #[bits(1)]
    pub write_through_caching: bool, // 4

    #[bits(1)]
    pub disable_caching: bool, // 5

    #[bits(1)]
    pub accessed: bool, // 6

    #[bits(1)]
    pub dirty: bool, // 7

    #[bits(1)]
    pub huge_page_or_null: bool, // 8
    // flush cache when address space is switched
    #[bits(1)]
    pub global: bool, // 9

    #[bits(3)]
    pub free_bits: u8, // 10 - 12 inc

    #[bits(40)]
    pub address: u64, // 13 - 52 inc

    #[bits(11)]
    pub free_bits2: u16, // 53 - 63 inc

    #[bits(1)]
    pub executable: bool, // 64
}

impl TableEntry {
    pub fn addr<const PAGE_SIZE: u64>(&self) -> VirtAddr {
        VirtAddr::from_bits(self.address() * PAGE_SIZE)
    }

    pub fn with_virt_addr(self, addr: VirtAddr) -> Self {
        return self.with_address(addr.0 / PAGE_SIZE);
    }
}

impl Debug for TableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TableEntry")
            .field("present", &self.present())
            .field("writable", &self.writable())
            .field("user", &self.user())
            .field("write_through_caching", &self.write_through_caching())
            .field("disable_caching", &self.disable_caching())
            .field("accessed", &self.accessed())
            .field("dirty", &self.dirty())
            .field("huge_page_or_null", &self.huge_page_or_null())
            .field("global", &self.global())
            .field("free_bits", &self.free_bits())
            .field("address", &self.addr::<PAGE_SIZE>())
            .field("free_bits2", &self.free_bits2())
            .field("executable", &self.executable())
            .finish()
    }
}

#[derive(Debug)]
pub struct Table<'a> {
    pub level: usize,
    pub entries: &'a mut [TableEntry],
}

impl<'a> Table<'a> {
    pub fn from_addr<const COUNT: usize>(addr: VirtAddr) -> Self {
        let ptr = unsafe { core::slice::from_raw_parts_mut(addr.0 as *mut TableEntry, COUNT) };
        Table {
            entries: ptr,
            level: 4,
        }
    }

    pub fn next(&self, index: u64) -> Option<Self> {
        let entry = self.entries.get(index as usize)?;
        // serial_info!("{:?}", entry);
        if !entry.present() {
            return None;
        }
        let table_addr = entry.addr::<PAGE_SIZE>();
        // serial_info!("Table Addr: {:?}", table_addr);
        let entry = Self::from_addr::<512>(table_addr);
        Some(Self {
            level: self.level - 1,
            entries: entry.entries,
        })
    }
    pub fn page(&self, index: u64) -> *const u8 {
        let entry = self.entries.get(index as usize).unwrap();
        let addr = entry.addr::<PAGE_SIZE>();
        serial_info!("{:?}", addr);
        let addr = addr.0 as *const u8;
        return addr;
    }

    pub fn set_entry(&mut self, index: u64, entry: TableEntry) {
        self.entries[index as usize] = entry;
    }
}

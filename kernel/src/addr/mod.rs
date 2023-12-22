use crate::{
    allocator::kernel_alloc::{PAGE_SIZE},
    paging::{Table, TableEntry},
    physical_memory_offset_val, serial_info,
};

use core::{fmt::Debug};

#[derive(Clone, Copy)]
pub struct VirtAddr(pub u64);

impl VirtAddr {
    pub const fn as_canonical(v: u64) -> Self {
        let shifter = 64 - 48;
        let val = (((v << shifter) as i64) >> shifter) as u64;
        return Self(val);
    }
    pub fn map_addr(&self, p4: &mut Table) {
        // TODO: new allocator
        // let table = p4;
        // let addr = self.0;
        // // 00000000 00000000 | 0000000000000000000 1 1111 1111| 0000 00000000 00000000
        // // sign (16)       | mask(9) |
        // let mask: u64 = 0b0000_0000_0000_0000_1111_1111_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        // let first = (addr & (mask)) >> (27 + 12);
        // let second = (addr & (mask >> 9)) >> (18 + 12);
        // let third = (addr & (mask >> 18)) >> (9 + 12);
        // let fourth = (addr & (mask >> 27)) >> (12);
        // let _offset = addr & (0xFFF);

        // let p3_table = table.next(first);
        // if p3_table.is_none() {
        //     // first entry leve
        //     let page = unsafe { LMM_ALLOC.alloc_page_virt_addr() };
        //     page.map_addr(table);
        //     table.set_entry(
        //         first,
        //         TableEntry::new().with_present(true).with_virt_addr(page),
        //     );
        // }
        // let mut p3_table = table.next(first).expect("table3 should have been mapped");

        // let p2_table = p3_table.next(second);
        // if p2_table.is_none() {
        //     // first entry leve
        //     let page = unsafe { LMM_ALLOC.alloc_page_virt_addr() };
        //     page.map_addr(table);
        //     p3_table.set_entry(
        //         second,
        //         TableEntry::new().with_present(true).with_virt_addr(page),
        //     );
        // }
        // let mut p2_table = table.next(first).expect("table2 should have been mapped");

        // let p1_table = p2_table.next(third);
        // if p1_table.is_none() {
        //     // first entry leve
        //     let page = unsafe { LMM_ALLOC.alloc_page_virt_addr() };
        //     page.map_addr(table);
        //     p2_table.set_entry(
        //         third,
        //         TableEntry::new().with_present(true).with_virt_addr(page),
        //     );
        // }
        // let mut p1_table = table.next(first).expect("table2 should have been mapped");

        // let page = VirtAddr::as_canonical(0);
        // // unsafe { LMM_ALLOC.alloc_page_virt_addr() }; //TODO: fix
        // p1_table.set_entry(
        //     fourth,
        //     TableEntry::new()
        //         .with_present(true)
        //         .with_address((page.0 + physical_memory_offset_val()) / PAGE_SIZE),
        // );
    }

    pub fn to_phy_addr(self, p4: &Table) -> Option<PhyAddr> {
        let table = p4;
        let addr = self.0;
        // 00000000 00000000 | 0000000000000000000 1 1111 1111| 0000 00000000 00000000
        // sign (16)       | mask(9) |
        let mask: u64 = 0b0000_0000_0000_0000_1111_1111_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        let first = (addr & (mask)) >> (27 + 12);
        let second = (addr & (mask >> 9)) >> (18 + 12);
        let third = (addr & (mask >> 18)) >> (9 + 12);
        let fourth = (addr & (mask >> 27)) >> (12);
        let _offset = addr & (0xFFF);
        serial_info!("1");
        let table2 = table.next(first)?;
        serial_info!("2");
        let table3 = table2.next(second)?;
        serial_info!("3");
        let table4 = table3.next(third)?;
        let page = table4.page(fourth);
        serial_info!("4");
        return Some(PhyAddr::from(page as u64));
    }

    pub fn from_ptr<T>(v: *const T) -> Self {
        Self::as_canonical(v as u64)
    }

    pub const fn into_bits(self) -> u64 {
        return self.0;
    }
    pub const fn from_bits(v: u64) -> Self {
        return Self::as_canonical(v);
    }
    pub const fn get(self) -> u64 {
        return self.0;
    }
}

impl Debug for VirtAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("VirtAddr")
            .field(&(self.0 as *mut u8))
            .finish()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhyAddr(pub u64);

impl PhyAddr {
    pub fn from(v: u64) -> Self {
        Self(v)
    }
}

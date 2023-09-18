use core::fmt::Debug;

use bitfield_struct::bitfield;

use crate::{info, println, utils::asm};

#[bitfield(u64)]
pub struct GdtEntry {
    #[bits(16)]
    limit: u16,

    #[bits(24)]
    base: u32,

    #[bits(1)]
    access: bool,

    #[bits(1)]
    rw: bool,

    #[bits(1)]
    direction: bool,

    #[bits(1)]
    executable: bool,

    #[bits(1)]
    descriptor_type: bool,

    #[bits(2)]
    privelege_level: u8,

    #[bits(1)]
    present: bool,

    #[bits(4)]
    limit2: u8,

    #[bits(4)]
    flags: u8,

    #[bits(8)]
    base2: u8,
}

#[repr(C)]
pub struct GlobalDescriptorTable {
    null_descriptor: GdtEntry,
    descriptors: [GdtEntry; 8191], // this is an upper limit please check GdtPointer for the actualsize
}
impl Debug for GlobalDescriptorTable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GlobalDescriptorTable")
            .field("null_descriptor", &self.null_descriptor)
            .field("descriptors1", &self.descriptors[0])
            .field("descriptors2", &self.descriptors[1])
            .field("code", &self.descriptors[7])
            .finish()
    }
}

#[repr(packed(2), C)]
#[derive(Debug)]
pub struct GdtPointer {
    pub size: u16,
    pub offset: *const GlobalDescriptorTable,
}
impl GlobalDescriptorTable {
    pub fn load(&self) {
        let pointer = GdtPointer {
            size: core::mem::size_of::<GlobalDescriptorTable>() as u16 - 1,
            offset: self as *const _,
        };
        unsafe { asm::lgdt(&pointer) };
    }
    pub fn read() -> *const Self {
        let ans = unsafe { asm::sgdt() };
        return ans.offset;
    }
}

// #[test_case]
pub fn print_gdt() {
    unsafe {
        let gdt = &*GlobalDescriptorTable::read();
        let mut count = 5;
        for (i, entry) in gdt.descriptors.iter().enumerate() {
            if (entry.executable() == true) {
                println!("{:?}, {:#?}", i, entry);
                count -= 1;
                if count == 0 {
                    break;
                }
            }
        }
    }
}

#[test_case]
pub fn test_gdtsize() {
    info!("Testing GDT size");
    assert_eq!(
        core::mem::size_of::<GlobalDescriptorTable>(),
        8192 * core::mem::size_of::<GdtEntry>()
    )
}

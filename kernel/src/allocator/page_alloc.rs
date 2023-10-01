use core::{marker::PhantomData, cell::RefCell, ptr::null};

use bootloader::{bootinfo::{MemoryRegion, self}, BootInfo};

use crate::{datastructures::no_alloc::linked_list::{LinkedList, Node}, println};



pub struct PageAlloc<const PAGE_SIZE: u64> {
    pub free_list: RefCell<LinkedList<[u8;0]>>,
    pub bootinfo: Option<&'static BootInfo>,
}


impl<const PAGE_SIZE: u64> PageAlloc<PAGE_SIZE> {
    pub const fn default() -> Self {
        Self { free_list: RefCell::new(LinkedList::default()), bootinfo: None}
    }

    pub fn print_reg(&self) {
        for i in self.free_list.borrow().iter() {
            println!("{:?}", unsafe{&*i});
        }
    }

    pub fn set_boot_infor(&mut self, bootinfo: &'static BootInfo) {
        self.bootinfo = Some(bootinfo);
    }

    pub fn add_region(&mut self, start: u64, end: u64) {
        let mut i = start;
        
        while i < end {
            self.free_list.borrow_mut().push_back(Node::from(i as *mut u8));
            i += PAGE_SIZE
        }
    }
}



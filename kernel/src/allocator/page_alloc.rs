use core::marker::PhantomData;

use bootloader::{bootinfo::MemoryRegion, BootInfo};

use crate::{datastructures::no_alloc::linked_list::{LinkedList, Node}, println};



pub struct PageAlloc<const PAGE_SIZE: u64> {
    free_list: LinkedList,
}



impl<const PAGE_SIZE: u64> PageAlloc<PAGE_SIZE> {
    pub const fn default() -> Self {
        Self { free_list: LinkedList::default() }
    }

    pub fn print_reg(&self) {
        for i in self.free_list.iter() {
            println!("{:?}", unsafe{&*i});
        }
    }

    pub fn add_region(&mut self, start: u64, end: u64) {
        let mut i = start;
        while i < end {
            self.free_list.push_back(Node::from(i as *mut u8));
            i += (PAGE_SIZE)
        }
    }
}
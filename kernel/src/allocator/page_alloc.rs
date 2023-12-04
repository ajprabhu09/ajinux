use core::cell::RefCell;

use bootloader::BootInfo;

use crate::{
    datastructures::no_alloc::linked_list::{LinkedList, Node},
    debug, serial_info, serial_debug,
};

pub struct PageAlloc<const PAGE_SIZE: u64> {
    pub free_list: RefCell<LinkedList<[u8; 0]>>,
    pub bootinfo: Option<&'static BootInfo>,
    pub total_size: u64,
}

impl<const PAGE_SIZE: u64> PageAlloc<PAGE_SIZE> {
    pub fn alloc_page(&mut self) -> *mut u8 {
        self.total_size -= PAGE_SIZE;
        return self.free_list.borrow_mut().pop_head() as *mut u8;
    }

    pub fn dealloc_page(&mut self, page: *mut u8) {
        self.total_size += PAGE_SIZE;
        self.free_list.borrow_mut().push_back(page as *mut Node<[u8;0]>);
    }

    pub const fn default() -> Self {
        Self {
            free_list: RefCell::new(LinkedList::default()),
            bootinfo: None,
            total_size: 0,
        }
    }

    pub fn print_reg(&self) {
        for i in self.free_list.borrow().iter() {
            serial_info!("{:?}", unsafe { &*i });
        }
    }

    pub fn set_boot_infor(&mut self, bootinfo: &'static BootInfo) {
        self.bootinfo = Some(bootinfo);
    }

    pub fn add_region(&mut self, start: u64, end: u64) {
        let mut i = start;

        while i < end {
            self.free_list
                .borrow_mut()
                .push_back(Node::from(i as *mut u8));
            i += PAGE_SIZE;
            self.total_size += PAGE_SIZE;
        }
        serial_debug!("total size is {:?} MB", self.total_size / 1_000_000);

        if i - end != 0 {
            debug!("left some memory on the table {:#02x}", i - end);
        }
    }
}

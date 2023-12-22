use bitfield_struct::bitfield;
use core::{cell::RefCell, fmt::Debug, mem::size_of, ptr::null_mut};

use bootloader::BootInfo;

use crate::{
    addr::VirtAddr,
    allocator::kernel_alloc::PAGE_SIZE,
    datastructures::no_alloc::linked_list::{LinkedList, Node}, serial_debug, serial_error, serial_info,
    utils::ptr_utils::{as_ref_mut, as_ref},
};

///The current Physicalmemorymanager is too generic for paging
///  We do not need size in the header afaik
/// To fix this either:
/// 1. keep a speerate list allocated from the generic list and then bit map and keep houskeeping i same allocation
/// 2. since rust keeps track of the sizes of its allocation we can probably omit the headers, seems risky but would prove a huge boost in performance
/// For now our allocator does not any of the above and simply allocates like malloc (slow af but is safe!)
/// 



/// The header ensure that all the 
#[bitfield(u64, debug = false)]
pub struct Header {
    #[bits(60)]
    pub size: u64,

    #[bits(3)]
    pub _unused: u8,

    #[bits(1)]
    pub allocated: bool,
}

impl Debug for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Header")
            .field("size", &(self.size() as *const ()))
            .field("size", &self.size())
            .field("allocated", &self.allocated())
            .finish()
    }
}




pub struct PhysicalMemAllocator {
    pub generic_free_list: RefCell<LinkedList<Header>>,
    pub page_free_list: RefCell<LinkedList<[u8;0]>>,
    pub bootinfo: Option<&'static BootInfo>,
    pub total_size: u64,
}

impl PhysicalMemAllocator {
    // pub fn has_page(&self) -> bool {
    //     !self.generic_free_list.borrow().empty()
    // }

    fn footer_mut<'a>(node: *mut Node<Header>) -> Option<&'a mut Header> {
        let node_ref = as_ref_mut(node)?;
        if node_ref.header.allocated() {
            serial_error!("called footer on allocated block");
            return None;
        }
        let ptr =
            ((node as usize) + (node_ref.header.size() as usize) - size_of::<u64>()) as *mut Header; // last word of the
        return as_ref_mut(ptr);
    }
    fn footer<'a>(node: *mut Node<Header>) -> Option<&'a Header> {
        let node_ref = as_ref_mut(node)?;
        if node_ref.header.allocated() {
            return None;
        }
        let ptr =
            ((node as usize) + (node_ref.header.size() as usize) - size_of::<u64>()) as *mut Header; // last word of the
        return unsafe { ptr.as_ref() };
    }

    fn best_fit(&self, size: i64) -> *mut Node<Header> {
        let Some((ptr, _size)) = self
            .generic_free_list
            .borrow()
            .iter()
            .map(|x| {
                let node_ptr =
                    as_ref_mut(x).expect("best_fit: this node should be valid");
                let size_block = (node_ptr.header.size() as i64) - (size);
                serial_debug!("got {:?} - {:?}", x, size_block);
                (x, size_block)
            })
            .filter(|(_ptr, nsize)| *nsize > 0)
            .min_by(|a, b| a.1.cmp(&b.1))
        else {
            serial_error!("could not find a best fit block atleast of size {:?}", size);
            return null_mut();
        };

        return ptr;
    }
    pub fn round_up(size: usize, round_up: usize) -> usize {
        return (size * round_up) / round_up;
    }

    pub fn extend_page_list(&mut self) {
        if self.page_free_list.borrow().empty() {

        }
    }

    pub fn alloc_page(&mut self) -> *mut u8 {
        return self.alloc(PAGE_SIZE as usize).0;
    }
    /// return buffer pointer and size of buffer ie, size in (block header + 8)
    pub fn alloc(&mut self, size: usize) -> (*mut u8, isize) {
        // min sizeof Node + Actual + Footer? , round up to 16 align
        let rounded_size = Self::round_up((size) + size_of::<Node<Header>>() + 8, 48);
        serial_debug!("Rounded to size {:?}", rounded_size);
        let node = self.best_fit(rounded_size as i64);
        serial_debug!("Best fit node is  {:?}", as_ref(node));
        let Some(node_ref) = as_ref_mut(node) else {
            serial_info!("node is null");
            return (null_mut(), -1);
        };
        let removed = self.generic_free_list.borrow_mut().remove(node);
        assert!(removed);
        let original_size = node_ref.header.size();

        let Some(footer) = Self::footer_mut(node) else {
            serial_info!("node has not footer!");
            return (null_mut(), -1);
        };
        footer.set_size(rounded_size as u64);
        footer.set_allocated(true);
        node_ref.header.set_allocated(true);
        node_ref.header.set_size(rounded_size as u64);

        // Slice found node and set to allocated on footer and header

        let left_over = ((node as usize) + rounded_size) as *mut Node<Header>;
        let left_over_ref =
            as_ref_mut(left_over).expect("a ptr add on a valid ref should not be null");

        let Some(footer) = Self::footer_mut(left_over) else {
            serial_info!("left_over has not footer!");
            return (null_mut(), -1);
        };
        footer.set_allocated(false);
        footer.set_size(original_size - (rounded_size as u64));
        left_over_ref.header.set_allocated(false);
        left_over_ref
            .header
            .set_size(original_size - (rounded_size as u64));

        self.generic_free_list.borrow_mut().push_front(left_over);
        serial_debug!("LEFT OVER: {:?}", left_over);
        self.generic_free_list.borrow().print_ptrlist(4);

        return (node_ref.data_ptr() as *mut u8, (rounded_size - 8) as isize);
    }

    pub fn alloc_page_virt_addr(&mut self) -> VirtAddr {
        return VirtAddr::as_canonical(self.alloc_page() as u64);
    }

    pub fn dealloc(&mut self, page: *mut u8) {
        let page = page as *mut Node<Header>;
        let page_ref = as_ref_mut(page).unwrap();
        page_ref.header.set_allocated(false);
        self.generic_free_list.borrow_mut().push_back(page);
    }

    pub const fn default() -> Self {
        Self {
            generic_free_list: RefCell::new(LinkedList::default()),
            bootinfo: None,
            total_size: 0,
            page_free_list: RefCell::new(LinkedList::default()),
        }
    }

    pub fn print_reg(&self) {
        for i in self.generic_free_list.borrow().iter() {
            serial_info!("{:?}", unsafe { &*i });
        }
    }

    pub fn set_boot_infor(&mut self, bootinfo: &'static BootInfo) {
        self.bootinfo = Some(bootinfo);
    }

    pub fn add_region(&mut self, start: u64, end: u64) {
        let node: *mut Node<Header> = Node::from(start as *mut u8);
        let node_ref = as_ref_mut(node).expect("cannot add 0 page region");
        node_ref.header = Header::new().with_allocated(false).with_size(end - start);
        let footer = Self::footer_mut(node).unwrap();
        footer.set_allocated(false);
        footer.set_size(end - start);
        self.generic_free_list.borrow_mut().push_back(node);
    }
}

impl Debug for Node<Header> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Node")
            .field("header", &self.header)
            .field("next", &self.next)
            .field("prev", &self.prev)
            .field("payload", &".....")
            .field("footer", &PhysicalMemAllocator::footer(self.untype_ptr() as *mut _))
            .finish()
    }
}

impl LinkedList<Header> {
    pub fn print_list(&self) {
        if self.len() == 0 {
            serial_info!("[]");
            return;
        }
        serial_info!("[");
        for x in self.iter() {
            serial_info!("\t {:?} {:?}", x, unsafe { &*x });
        }
        serial_info!("]");
    }
}

#[cfg(test)]
mod test {
    use core::mem::size_of;

    use crate::{
        allocator::{
            kernel_alloc::{PAGE_ALLOC, PAGE_SIZE},
            physical_mem::PhysicalMemAllocator,
        },
        datastructures::no_alloc::linked_list::Node,
        serial_info,
        utils::ptr_utils::as_ref_mut, serial_debug,
    };

    use super::Header;

    #[test_case]
    pub fn test_field_bits() {
        let a = Header::new().with_allocated(true).with_size(PAGE_SIZE);
        assert_eq!(a.size(), PAGE_SIZE)
    }

    #[test_case] 
    pub fn test_page_alignment_for_simple_alloc() {
        unsafe {
            serial_debug!("Testing page alignment");
            let a = PAGE_ALLOC.alloc(PAGE_SIZE as usize);
            serial_info!("got pointer and size {:?} ", a);

        }
    }

    #[test_case]
    pub fn test_allocation() {
        serial_info!("Testing Allocattion for pAGE Allocator");
        unsafe {
            PAGE_ALLOC.generic_free_list.borrow().print_list();
            let page = PAGE_ALLOC.alloc_page();
            PAGE_ALLOC.generic_free_list.borrow().print_list();
            assert_eq!(PAGE_ALLOC.generic_free_list.borrow_mut().len(), 2);

            serial_info!("{:?}", page);
            assert!(!page.is_null());
            let head = ((page as usize) - 8) as *mut Node<Header>;
            let head = as_ref_mut(head).unwrap();
            assert!(head.header.allocated());
            assert_eq!(
                head.header.size(),
                PhysicalMemAllocator::round_up((PAGE_SIZE as usize) + size_of::<Node<Header>>() + 8, 16)
                    as u64
            );
            PAGE_ALLOC.dealloc(page);
            assert_eq!(PAGE_ALLOC.generic_free_list.borrow().len(), 3);
        }
    }
}

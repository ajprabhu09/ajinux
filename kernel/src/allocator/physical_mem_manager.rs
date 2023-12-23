use core::{
    mem::size_of,
    ptr::{null, null_mut},
};

use bitfield_struct::bitfield;

use crate::{
    datastructures::no_alloc::linked_list::{LinkedList, Node},
    serial_debug, serial_error, serial_info,
    utils::ptr_utils::{as_ref, as_ref_mut},
};
use core::fmt::Debug;

// TODO: add docs

#[bitfield(u64, debug = false)]
pub struct RegionHeader {
    #[bits(60)]
    pub size: u64,

    // #[bits(2)]
    // pub prev_page_type: AllocType,

    // #[bits(1)]
    // pub prev_allocated: bool,
    #[bits(3)]
    _unused: u8,

    #[bits(1)]
    pub allocated: bool,
}

impl Debug for RegionHeader {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RegionHeader")
            .field("size", &(self.size() as *const ()))
            .field("size", &self.size())
            // .field("prev_page_type", &self.prev_page_type())
            // .field("prev_allocated", &self.prev_allocated())
            .field("allocated", &self.allocated())
            .finish()
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AllocType {
    Page4k = 1 << 12,
    Page2M = 1 << 21,
    Page1G = 1 << 30,
    Unk,
}

impl AllocType {
    pub const fn into_bits(self) -> u64 {
        match self {
            AllocType::Page1G => 3,
            AllocType::Page2M => 2,
            AllocType::Page4k => 1,
            AllocType::Unk => 0,
        }
    }
    pub const fn from_bits(v: u64) -> AllocType {
        let _4k: u64 = 1;
        let _1m: u64 = 2;
        let _1g: u64 = 3;

        if v == _4k {
            return AllocType::Page4k;
        }
        return AllocType::Unk;
    }
}

pub struct PhysMemManager {
    free_list_4k: LinkedList<[u8; 0]>,
    free_list_2m: LinkedList<[u8; 0]>,
    free_list_1g: LinkedList<[u8; 0]>,
    regions: LinkedList<RegionHeader>,
}

impl Debug for Node<RegionHeader> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Node")
            .field("header", &self.header)
            .field("next", &self.next)
            .field("prev", &self.prev)
            .field("payload", &".....")
            // .field(
            //     "footer",
            //     &PhysMemManager::footer(self.untype_ptr() as *mut _, self.header.size()),
            // )
            .finish()
    }
}

impl LinkedList<RegionHeader> {
    pub fn print_list(&self) {
        if self.len() == 0 {
            serial_info!("[]");
            return;
        }
        serial_info!("[");
        for x in self.iter() {
            serial_info!("\t {:?} {:#?}", x, unsafe { &*x });
        }
        serial_info!("]");
    }

    pub fn total_size(&self) -> u64 {
        self.iter().map(|x| as_ref(x).unwrap().header.size()).sum()
    }
}

impl PhysMemManager {
    fn free_list_space(&self, alloc_type: AllocType) -> u64 {
        match alloc_type {
            AllocType::Page4k => self
                .free_list_4k
                .iter()
                .map(|x| AllocType::Page4k as u64)
                .sum(),
            AllocType::Page2M => self
                .free_list_2m
                .iter()
                .map(|x| AllocType::Page2M as u64)
                .sum(),
            AllocType::Page1G => self
                .free_list_1g
                .iter()
                .map(|x| AllocType::Page1G as u64)
                .sum(),
            AllocType::Unk => self.regions.total_size(),
        }
    }

    pub fn space_available(&self) -> u64 {
        [
            AllocType::Page4k,
            AllocType::Page2M,
            AllocType::Page1G,
            AllocType::Unk,
        ]
        .iter()
        .map(|x| self.free_list_space(*x))
        .sum()
    }

    pub const fn default() -> Self {
        Self {
            free_list_4k: LinkedList::default(),
            free_list_2m: LinkedList::default(),
            free_list_1g: LinkedList::default(),
            regions: LinkedList::default(),
        }
    }

    fn footer<'a>(ptr: *mut Node<RegionHeader>, size: u64) -> &'a RegionHeader {
        //  TODO: assert node is not allocated
        // if node is allocated that means the head and footer cannot be seen
        let footer = ((ptr as u64) + size - 8) as *mut RegionHeader;
        return as_ref(footer).unwrap();
    }
    fn footer_mut<'a>(ptr: *mut Node<RegionHeader>, size: u64) -> &'a mut RegionHeader {
        //  TODO: assert node is not allocated
        // if node is allocated that means the head and footer cannot be seen
        let footer = ((ptr as u64) + size - 8) as *mut RegionHeader;
        return as_ref_mut(footer).unwrap();
    }

    fn take_page_from_regions(&mut self, alloc_type: AllocType) -> *mut u8 {
        if alloc_type == AllocType::Unk {
            serial_error!("Unk allocation type is not supported in this case");
            return null_mut();
        }
        let allocation_size = alloc_type as u64;

        let Some(region) = self
            .regions
            .iter()
            .filter(|a| {
                let node = unsafe { &(**a) };
                !node.header.allocated() && node.header.size() >= (alloc_type as u64)
            })
            .take(1)
            .reduce(|a, b| a)
        else {
            return null_mut();
        };

        self.regions.remove(region);

        let region_ref = as_ref_mut(region).unwrap();
        let left_over = ((region as u64) + allocation_size) as *mut Node<RegionHeader>;
        let left_over_ref = as_ref_mut(left_over).unwrap();
        let left_over_size = region_ref.header.size() - allocation_size;
        left_over_ref.header.set_size(left_over_size);
        // left_over_ref.header.set_prev_allocated(true);
        // left_over_ref.header.set_prev_page_type(alloc_type);
        left_over_ref.header.set_allocated(false);

        let footer = Self::footer_mut(left_over, left_over_size);
        footer.set_size(left_over_size);
        // footer.set_prev_allocated(true);
        // footer.set_prev_page_type(alloc_type);
        footer.set_allocated(false);

        self.regions.push_back(left_over);
        return region as *mut u8;
    }

    //TODO: add better compaction when page taking from regions list fails
    //

    fn alloc4k(&mut self) -> *mut u8 {
        if self.free_list_4k.empty() {
            return self.take_page_from_regions(AllocType::Page4k);
        }
        return self.free_list_4k.pop_head() as *mut _;
    }

    fn alloc2M(&mut self) -> *mut u8 {
        if self.free_list_2m.empty() {
            return self.take_page_from_regions(AllocType::Page2M);
        }
        return self.free_list_4k.pop_head() as *mut _;
    }
    fn alloc1G(&mut self) -> *mut u8 {
        if self.free_list_1g.empty() {
            return self.take_page_from_regions(AllocType::Page1G);
        }
        return self.free_list_1g.pop_head() as *mut _;
    }

    fn attempt_compaction_after_free(&mut self, ptr: *mut u8, size: AllocType) -> bool {
        let ptr = ((ptr as u64) + (size as u64)) as *mut Node<_>;
        if let Some(succ) = self.regions.iter().find(|x| *x == ptr) {
            let original_size = as_ref(succ).unwrap().header.size();
            self.regions.remove(succ);
            let new_node = as_ref_mut(ptr).unwrap();
            new_node.header.set_size((original_size) + (size as u64));
            self.regions.push_back(ptr);
            return true;
        }
        return false;
    }

    fn free4k(&mut self, ptr: *mut u8) {
        let compacted = self.attempt_compaction_after_free(ptr, AllocType::Page4k);
        if !compacted {
            serial_debug!("could not compact {:?}", ptr);
            let node = ptr as *mut _;
            self.free_list_4k.push_back(node);
        } else {
            serial_debug!("compacted {:?}", ptr);
        }
    }
    fn free2M(&mut self, ptr: *mut u8) {
        let compacted = self.attempt_compaction_after_free(ptr, AllocType::Page2M);
        if !compacted {
            serial_debug!("could not compact {:?}", ptr);

            let node = ptr as *mut _;
            self.free_list_2m.push_back(node);
        } else {
            serial_debug!("compacted {:?}", ptr);
        }
    }
    fn free1G(&mut self, ptr: *mut u8) {
        let compacted = self.attempt_compaction_after_free(ptr, AllocType::Page1G);
        if !compacted {
            serial_debug!("could not compact {:?}", ptr);

            let node = ptr as *mut _;
            self.free_list_1g.push_back(node);
        } else {
            serial_debug!("compacted {:?}", ptr);
        }
    }

    pub fn alloc(&mut self, alloc_type: AllocType) -> *mut u8 {
        match alloc_type {
            AllocType::Page4k => self.alloc4k(),
            AllocType::Page2M => self.alloc2M(),
            AllocType::Page1G => self.alloc1G(),
            _ => panic!("This cannot be an allocation"),
        }
    }

    pub fn free(&mut self, alloc_type: AllocType, ptr: *mut u8) {
        match alloc_type {
            AllocType::Page4k => self.free4k(ptr),
            AllocType::Page2M => self.free2M(ptr),
            AllocType::Page1G => self.free1G(ptr),
            _ => panic!("This cannot be an free"),
        }
    }

    pub fn add_region(&mut self, start: u64, end: u64) -> Result<(), &'static str> {
        if ((end - start) as usize) < 4 * size_of::<usize>() {
            return Err("region to small");
        }
        let node: *mut Node<RegionHeader> = Node::from(start as *mut u8);
        let node_ref = as_ref_mut(node).expect("cannot add 0 page region");
        node_ref.header.set_size(end - start);
        node_ref.header.set_allocated(false);
        // node_ref.header.set_prev_allocated(true);
        // node_ref.header.set_prev_page_type(AllocType::Unk);
        let footer = Self::footer_mut(node, node_ref.header.size());
        footer.set_allocated(false);
        // footer.set_prev_allocated(true);
        // footer.set_prev_page_type(AllocType::Unk);
        footer.set_size(node_ref.header.size());
        self.regions.push_back(node_ref.untype_ptr_mut());
        Ok(())
    }
}

pub static mut PHY_MM: PhysMemManager = PhysMemManager::default();

#[cfg(test)]
mod tests {
    use crate::{allocator::physical_mem_manager::AllocType, serial_info};

    use super::PHY_MM;

    #[test_case]
    pub fn test_basic_region_add() {
        serial_info!("Testing Phy Page region test");
        unsafe { PHY_MM.regions.print_list() };
    }
    #[test_case]
    pub fn test_basic_region_add() {
        serial_info!("Testing Slicing for page allocator");

        unsafe {
            let size = PHY_MM.space_available();
            let page1 = PHY_MM.take_page_from_regions(AllocType::Page4k);
            let page2 = PHY_MM.take_page_from_regions(AllocType::Page4k);
            let page3 = PHY_MM.take_page_from_regions(AllocType::Page4k);
            let page4 = PHY_MM.take_page_from_regions(AllocType::Page4k);
            let page5 = PHY_MM.take_page_from_regions(AllocType::Page4k);

            let page6 = PHY_MM.take_page_from_regions(AllocType::Page1G);

            assert_ne!(PHY_MM.space_available(), size);
            serial_info!(
                "number of 4k allocated {:?}",
                (size - PHY_MM.space_available()) / (AllocType::Page4k as u64)
            );

            PHY_MM.free(AllocType::Page4k, page1);
            PHY_MM.free(AllocType::Page4k, page2);
            PHY_MM.free(AllocType::Page4k, page3);
            PHY_MM.free(AllocType::Page4k, page4);
            PHY_MM.free(AllocType::Page4k, page5);
            PHY_MM.free(AllocType::Page1G, page6);
            assert!((page1 as u64) & ((AllocType::Page4k as u64) - 1) == 0);
            assert_eq!(PHY_MM.space_available(), size);
        };
    }
}

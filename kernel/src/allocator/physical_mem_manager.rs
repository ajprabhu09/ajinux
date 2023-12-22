use core::mem::size_of;

use bitfield_struct::bitfield;

use crate::{datastructures::no_alloc::linked_list::{LinkedList, Node}, utils::ptr_utils::as_ref_mut};




#[bitfield(u64)] 
pub struct RegionHeader {
    #[bits(60)]
    pub size: u64,

    #[bits(4)]
    pub _unused: u8,
}


pub struct PhysMemManager {
    free_list_4k: LinkedList<[u8;0]>,
    free_list_2m: LinkedList<[u8;0]>,
    free_list_1g: LinkedList<[u8;0]>,
    regions: LinkedList<RegionHeader>,
}

impl PhysMemManager {

    pub fn add_region(&mut self, start: u64, end: u64) -> Result<(), &'static str> {
        if ((end - start) as usize) < 4 * size_of::<usize>() {
            return Err("region to small");
        } 
        let node: *mut Node<RegionHeader> = Node::from(start as *mut u8);
        let node_ref = as_ref_mut(node).expect("cannot add 0 page region");
        node_ref.header.set_size(end - start);
        self.regions.push_back(node_ref.untype_ptr_mut());
        Ok(())
    }

}



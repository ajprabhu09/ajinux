use core::{alloc::GlobalAlloc, cell::UnsafeCell, ptr::null_mut};

use crate::serial_info;

pub struct KernelAllocator {
    heap_start: UnsafeCell<*mut u8>,
    heap_end: UnsafeCell<*mut u8>,
}

pub const PAGE_SIZE: u64 = 4096;

// TODO: change
const KERNEL_HEAP_START_DEFAULT: *mut u8 = null_mut();
const KERNEL_HEAP_END_DEFAULT: *mut u8 = null_mut();

#[global_allocator]
pub static KERNEL_ALLOC: KernelAllocator = KernelAllocator::default();

impl KernelAllocator {
    pub const fn default() -> Self {
        Self {
            heap_start: UnsafeCell::new(KERNEL_HEAP_START_DEFAULT),
            heap_end: UnsafeCell::new(KERNEL_HEAP_END_DEFAULT),
        }
    }
}

unsafe impl Sync for KernelAllocator {}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        return null_mut();
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {}
}

// unsafe impl Allocator for KernelAllocator {
//     fn allocate(
//         &self,
//         layout: core::alloc::Layout,
//     ) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
//         serial_info!("alloc ran: remaining size: {:?} B", unsafe {
//             PAGE_ALLOC.total_size
//         });
//         let page = unsafe { (PAGE_ALLOC.alloc_page()) };
//         let allocation = unsafe { core::slice::from_raw_parts_mut(page, layout.align()) };
//         return Ok(unsafe { NonNull::new_unchecked(allocation) });
//     }

//     unsafe fn deallocate(&self, ptr: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
//         serial_info!("dealloc ran dremaining size: {:?} B", unsafe {
//             PAGE_ALLOC.total_size
//         });
//         let page = ptr.as_ptr();
//         PAGE_ALLOC.dealloc_page(page)
//     }
// }

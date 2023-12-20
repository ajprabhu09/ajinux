use crate::descriptors::reg::*;

#[repr(C)]
struct TaskControlBlock {
    thread_id: usize,
    cr0: usize,
}

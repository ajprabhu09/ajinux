use core::marker::PhantomData;

use lazy_static::lazy_static;

use crate::{print, println};

#[repr(C, packed(2))]
pub struct InterruptDescriptorPointer {
    base: usize, // This is virtual address
    limit: u16,
}

#[repr(transparent)]
pub struct EntryOptions(u16);

#[repr(C)]
pub struct Entry<T> {
    offset1: u16,
    segment_selector: u16,
    options: EntryOptions,
    offset2: u16,
    offset3: u32,
    _reserved: u32,
    phantom: PhantomData<T>,
}

macro_rules! gen_entry_impl {
    ($if: ty) => {
        impl Entry<$if> {
            // pub fn 
        }
    };
}

gen_entry_impl!(HandlerFunc);
gen_entry_impl!(HandlerFuncWithErrCode);
gen_entry_impl!(DivergingHandlerFunc);
gen_entry_impl!(DivergingHandlerFuncWithErrCode);
gen_entry_impl!(PageFaultHandlerFunc);
pub struct InterruptDescriptorTable {
    pub divide_error: Entry<HandlerFunc>,
    pub debug: Entry<HandlerFunc>,
    pub non_maskable_interrupt: Entry<HandlerFunc>,
    pub breakpoint: Entry<HandlerFunc>,
    pub overflow: Entry<HandlerFunc>,
    pub bound_range_exceeded: Entry<HandlerFunc>,
    pub invalid_opcode: Entry<HandlerFunc>,
    pub device_not_available: Entry<HandlerFunc>,
    pub double_fault: Entry<DivergingHandlerFuncWithErrCode>,
    _coprocessor_segment_overrun: Entry<HandlerFunc>,
    pub invalid_tss: Entry<HandlerFuncWithErrCode>,
    pub segment_not_present: Entry<HandlerFuncWithErrCode>,
    pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
    pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
    pub page_fault: Entry<PageFaultHandlerFunc>,
    _reserved_1: Entry<HandlerFunc>,
    pub x87_floating_point: Entry<HandlerFunc>,
    pub alignment_check: Entry<HandlerFuncWithErrCode>,
    pub machine_check: Entry<DivergingHandlerFunc>,
    pub simd_floating_point: Entry<HandlerFunc>,
    pub virtualization: Entry<HandlerFunc>,
    pub cp_protection_exception: Entry<HandlerFuncWithErrCode>,
    _reserved_2: [Entry<HandlerFunc>; 6],
    pub hv_injection_exception: Entry<HandlerFunc>,
    pub vmm_communication_exception: Entry<HandlerFuncWithErrCode>,
    pub security_exception: Entry<HandlerFuncWithErrCode>,
    _reserved_3: Entry<HandlerFunc>,
    pub interrupts: [Entry<HandlerFunc>; 256 - 32],
}

impl InterruptDescriptorTable {
    pub fn new() {

    }
}




#[test_case]
pub fn test_idt_size() {
    assert_eq!(256*16, core::mem::size_of::<InterruptDescriptorTable>());
}


#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: usize,
    /// The code segment selector, padded with zeros.
    pub code_segment: usize,
    /// The flags register before the interrupt handler was invoked.
    pub cpu_flags: usize,
    /// The stack pointer at the time of the interrupt.
    pub stack_pointer: usize,
    /// The stack segment descriptor at the time of the interrupt (often zero in 64-bit mode).
    pub stack_segment: usize,

}

/// from x86_64 package https://github.com/rust-osdev/x86_64/blob/master/src/structures/idt.rs#L928
pub type HandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame);
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type PageFaultHandlerFunc =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame) -> !;
pub type DivergingHandlerFuncWithErrCode =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64) -> !;
pub type GeneralHandlerFunc = fn(InterruptStackFrame, index: u8, error_code: Option<u64>);

pub static mut MUT_GLOBAL: i32 = 1;

use core::marker::PhantomData;

use lazy_static::lazy_static;

use crate::utils::asm;
use crate::utils::bytes::ToplevelG;
use crate::{print, println};
#[repr(C, packed(2))]
pub struct InterruptDescriptorPointer {
    base: usize, // This is virtual address
    limit: u16,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EntryOptions(u16);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entry<T> {
    offset1: u16,
    segment_selector: u16,
    options: EntryOptions,
    offset2: u16,
    offset3: u32,
    _reserved: u32,
    phantom: PhantomData<T>,
}

impl<T> Entry<T> {
    pub const fn nil() -> Self {
        Self {
            offset1: 0,
            segment_selector: 0,
            options: EntryOptions(0b000_00000_1110_00_0), //
            offset2: 0,
            offset3: 0,
            _reserved: 0,
            phantom: PhantomData,
        }
    }
    pub fn with_entry_fn<F: IDFfn>(mut self, interrupt_fn: F) -> Self {
        let (o1,o2,o3) = interrupt_fn.to_offsets();
        self.offset1 = o1;
        self.offset2 = o2;
        self.offset3 = o3;
        self
    }
    pub fn with_options(mut self, bits: u16) -> Self {
        self.options = EntryOptions(bits);
        self
    }
}

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
    pub page_fault: Entry<HandlerFuncWithErrCode>,
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

#[repr(usize)]
pub enum InterruptIdx {
    Timer = 8,
}

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self {
            divide_error: Entry::nil(),
            debug: Entry::nil(),
            non_maskable_interrupt: Entry::nil(),
            breakpoint: Entry::nil(),
            overflow: Entry::nil(),
            bound_range_exceeded: Entry::nil(),
            invalid_opcode: Entry::nil(),
            device_not_available: Entry::nil(),
            double_fault: Entry::nil(),
            _coprocessor_segment_overrun: Entry::nil(),
            invalid_tss: Entry::nil(),
            segment_not_present: Entry::nil(),
            stack_segment_fault: Entry::nil(),
            general_protection_fault: Entry::nil(),
            page_fault: Entry::nil(),
            _reserved_1: Entry::nil(),
            x87_floating_point: Entry::nil(),
            alignment_check: Entry::nil(),
            machine_check: Entry::nil(),
            simd_floating_point: Entry::nil(),
            virtualization: Entry::nil(),
            cp_protection_exception: Entry::nil(),
            _reserved_2: [Entry::nil(); 6],
            hv_injection_exception: Entry::nil(),
            vmm_communication_exception: Entry::nil(),
            security_exception: Entry::nil(),
            _reserved_3: Entry::nil(),
            interrupts: [Entry::nil(); 256 - 32],
        }
    }

    pub fn load(&self) {
        let base_ptr = self as (*const InterruptDescriptorTable);
        let base = base_ptr as usize;
        let limit = 256;
        let ptr = InterruptDescriptorPointer{ base, limit };
        unsafe { asm::lidt(&ptr) };


    }

}

#[test_case]
pub fn test_idt_size() {
    assert_eq!(256 * 16, core::mem::size_of::<InterruptDescriptorTable>());
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

pub trait IDFfn {
    fn to_offsets(&self) -> (u16, u16, u32);
}
/// from x86_64 package https://github.com/rust-osdev/x86_64/blob/master/src/structures/idt.rs#L928
pub type HandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame);
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
// Duplicate
// pub type PageFaultHandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(InterruptStackFrame) -> !;
pub type DivergingHandlerFuncWithErrCode =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64) -> !;
pub type GeneralHandlerFunc = fn(InterruptStackFrame, index: u8, error_code: Option<u64>);

macro_rules! impl_idfn_trait {
    ($intfn: ty) => {
        impl IDFfn for $intfn {
            fn to_offsets(&self) -> (u16, u16, u32) {
                unsafe {
                    let ptr: u64 = core::mem::transmute(self);
                    let ptr = ToplevelG::<u64, u8, 8> { word: ptr };

                    let offset1: u16 = {
                        let offset1 = ToplevelG::<_, _, 2> {
                            component: [ptr.component[0], ptr.component[1]],
                        };
                        offset1.word
                    };
                    let offset2: u16 = {
                        let offset1 = ToplevelG::<_, _, 2> {
                            component: [ptr.component[2], ptr.component[3]],
                        };
                        offset1.word
                    };
                    let offset3: u32 = {
                        let offset1 = ToplevelG::<_, _, 4> {
                            component: [
                                ptr.component[4],
                                ptr.component[5],
                                ptr.component[6],
                                ptr.component[7],
                            ],
                        };
                        offset1.word
                    };
                    (offset1, offset2, offset3)
                }
            }
        }
    };
}

impl_idfn_trait!(HandlerFunc);
impl_idfn_trait!(HandlerFuncWithErrCode);
impl_idfn_trait!(DivergingHandlerFunc);
impl_idfn_trait!(DivergingHandlerFuncWithErrCode);
impl_idfn_trait!(GeneralHandlerFunc);

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();



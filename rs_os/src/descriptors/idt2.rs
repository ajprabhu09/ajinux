

use bitfield_struct::bitfield;
use core::fmt;
use core::marker::PhantomData;

use crate::utils::asm::lidt;

use super::idt::{DescriptorPointer, GateType, ExcptionStackFrame};
use super::segmentation::{GetReg, CS};



#[derive(Clone, Debug)]
#[repr(C)]
#[repr(align(16))]
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
    coprocessor_segment_overrun: Entry<HandlerFunc>,
    pub invalid_tss: Entry<HandlerFuncWithErrCode>,
    pub segment_not_present: Entry<HandlerFuncWithErrCode>,
    pub stack_segment_fault: Entry<HandlerFuncWithErrCode>,
    pub general_protection_fault: Entry<HandlerFuncWithErrCode>,
    pub page_fault: Entry<PageFaultHandlerFunc>,
    reserved_1: Entry<HandlerFunc>,
    pub x87_floating_point: Entry<HandlerFunc>,
    pub alignment_check: Entry<HandlerFuncWithErrCode>,
    pub machine_check: Entry<DivergingHandlerFunc>, // DivergingHandlerFunc
    pub simd_floating_point: Entry<HandlerFunc>,
    pub virtualization: Entry<HandlerFunc>,
    reserved_2: [Entry<HandlerFunc>; 8],
    pub vmm_communication_exception: Entry<HandlerFuncWithErrCode>,
    pub security_exception: Entry<HandlerFuncWithErrCode>,
    reserved_3: Entry<HandlerFunc>,
    interrupts: [Entry<HandlerFunc>; 256 - 32],
}

impl InterruptDescriptorTable {
    pub const fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            divide_error: Entry::missing(),
            debug: Entry::missing(),
            non_maskable_interrupt: Entry::missing(),
            breakpoint: Entry::missing(),
            overflow: Entry::missing(),
            bound_range_exceeded: Entry::missing(),
            invalid_opcode: Entry::missing(),
            device_not_available: Entry::missing(),
            double_fault: Entry::missing(),
            coprocessor_segment_overrun: Entry::missing(),
            invalid_tss: Entry::missing(),
            segment_not_present: Entry::missing(),
            stack_segment_fault: Entry::missing(),
            general_protection_fault: Entry::missing(),
            page_fault: Entry::missing(),
            reserved_1: Entry::missing(),
            x87_floating_point: Entry::missing(),
            alignment_check: Entry::missing(),
            machine_check: Entry::missing(),
            simd_floating_point: Entry::missing(),
            virtualization: Entry::missing(),
            reserved_2: [Entry::missing(); 8],
            vmm_communication_exception: Entry::missing(),
            security_exception: Entry::missing(),
            reserved_3: Entry::missing(),
            interrupts: [Entry::missing(); 256 - 32],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn load(&'static self) {
        unsafe { self.load_unsafe() }
    }

    pub unsafe fn load_unsafe(&self) {
        unsafe {
            lidt(&self.pointer());
        }
    }

    fn pointer(&self) -> DescriptorPointer {
        use core::mem::size_of;
        DescriptorPointer {
            offset: self as *const _ as u64,
            size: (size_of::<Self>() - 1) as u16,
        }
    }
}

#[bitfield(u16)]
pub struct IdtSettings {
    #[bits(3)]
    interrupt_stack_table: u8,

    #[bits(5)]
    _reserved: u8,

    #[bits(4)]
    gate_type: GateType,

    #[bits(1)]
    _reserved2: u8, // always zero

    #[bits(2)]
    privilege_level: u8,

    #[bits(1)]
    present: bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Entry<F> {
    pointer_low: u16,
    gdt_selector: u16,
    options: IdtSettings,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}

impl<T> fmt::Debug for Entry<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("handler_addr", &format_args!("{:#x}", self.handler_addr()))
            .field("gdt_selector", &self.gdt_selector)
            .field("options", &self.options)
            .finish()
    }
}

pub type HandlerFunc = extern "x86-interrupt" fn(ExcptionStackFrame);
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(ExcptionStackFrame, error_code: u64);
pub type PageFaultHandlerFunc = HandlerFunc;
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(ExcptionStackFrame) -> !;
pub type DivergingHandlerFuncWithErrCode =
    extern "x86-interrupt" fn(ExcptionStackFrame, error_code: u64) -> !;
pub type GeneralHandlerFunc = fn(ExcptionStackFrame, index: u8, error_code: Option<u64>);

impl<F> Entry<F> {
    pub const fn missing() -> Self {
        Entry {
            gdt_selector: 0,
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: IdtSettings(0b1110_0000_0000),
            reserved: 0,
            phantom: PhantomData,
        }
    }
    

    pub fn handler_addr(&self) -> u64 {
        let addr = self.pointer_low as u64
            | (self.pointer_middle as u64) << 16
            | (self.pointer_high as u64) << 32;
        addr
    }
}

macro_rules! impl_set_handler_fn {
    ($h:ty) => {
        impl Entry<$h> {
            pub fn set_handler_fn(&mut self, handler: $h) {
                let addr = handler as u64;
                self.pointer_low = addr as u16;
                self.pointer_middle = (addr >> 16) as u16;
                self.pointer_high = (addr >> 32) as u32;
                self.gdt_selector = CS::get_reg();
                self.options.set_present(true);
            }
        }
    };
}

impl_set_handler_fn!(HandlerFunc);
impl_set_handler_fn!(HandlerFuncWithErrCode);
impl_set_handler_fn!(DivergingHandlerFunc);
impl_set_handler_fn!(DivergingHandlerFuncWithErrCode);

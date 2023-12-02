use super::reg::{GetReg, CS};
use crate::utils::asm::lidt;
use bitfield_struct::bitfield;
use core::fmt;
use core::marker::PhantomData;

#[repr(C, packed(2))]
#[derive(Debug)]
pub struct DescriptorPointer {
    pub size: u16,
    pub offset: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}
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
    pub interrupts: [Entry<HandlerFunc>; 256 - 32],
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
    pub fn load(&'static self) {
        unsafe {
            let ptr = DescriptorPointer {
                offset: self as *const _ as u64,
                size: 4095,
            };
            lidt(&ptr);
        }
    }
}

pub enum GateType {
    TrapGate = 0xE,
    InterruptGate = 0xF,
}

impl GateType {
    pub const fn from_bits(val: u16) -> Self {
        let _v1 = 0xE << 8;
        let _v2 = 0xF << 8;
        match val {
            _v1 => Self::TrapGate,
            _v2 => Self::InterruptGate,
            _ => panic!("invalid gate type"),
        }
    }
    pub const fn into_bits(self) -> u16 {
        match self {
            Self::TrapGate => 0xE,
            Self::InterruptGate => 0xF,
        }
    }
}

impl core::fmt::Debug for GateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TrapGate => write!(f, "TrapGate"),
            Self::InterruptGate => write!(f, "InterruptGate"),
        }
    }
}
#[bitfield(u16)]
pub struct IdtSettings {
    #[bits(3)]
    pub interrupt_stack_table: u8,

    #[bits(5)]
    _reserved: u8,

    #[bits(4)]
    pub gate_type: GateType,

    #[bits(1)]
    _reserved2: u8, // always zero

    #[bits(2)]
    pub privilege_level: u8,

    #[bits(1)]
    pub present: bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Entry<F> {
    pointer_low: u16,
    gdt_selector: u16,
    pub options: IdtSettings,
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

pub type HandlerFunc = extern "x86-interrupt" fn(ExceptionStackFrame);
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(ExceptionStackFrame, error_code: u64);
pub type PageFaultHandlerFunc = HandlerFunc;
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(ExceptionStackFrame) -> !;
pub type DivergingHandlerFuncWithErrCode =
    extern "x86-interrupt" fn(ExceptionStackFrame, error_code: u64) -> !;
pub type GeneralHandlerFunc = fn(ExceptionStackFrame, index: u8, error_code: Option<u64>);

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
            pub fn set_handler_fn(&mut self, handler: $h) -> &mut Self {
                let addr = handler as u64;
                self.pointer_low = addr as u16;
                self.pointer_middle = (addr >> 16) as u16;
                self.pointer_high = (addr >> 32) as u32;
                self.gdt_selector = CS::get_reg();
                self.options.set_present(true);
                self
            }
        }
    };
}

impl_set_handler_fn!(HandlerFunc);
impl_set_handler_fn!(HandlerFuncWithErrCode);
impl_set_handler_fn!(DivergingHandlerFunc);
impl_set_handler_fn!(DivergingHandlerFuncWithErrCode);

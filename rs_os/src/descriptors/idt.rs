use core::{fmt::Debug, marker::PhantomData};

use bitfield_struct::bitfield;

use crate::{addr::CanonicalAddr, utils::asm::lidt};

use super::segmentation::{GetReg, CS};

pub enum GateType {
    TrapGate = 0xE,
    InterruptGate = 0xF,
}

impl GateType {
    pub const fn from_bits(val: u16) -> Self {
        let v1 = (0xE << 8);
        let v2 = (0xF << 8);
        match val {
            v1 => Self::TrapGate,
            v2 => Self::InterruptGate,
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

impl Debug for GateType {
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
pub struct Entry<T> {
    offset_low: u16,
    segment_selector: u16,
    settings: IdtSettings,
    offset_middle: u16,
    offset_high: u32,
    _reserved: u32,
    _pd: PhantomData<T>,
}
impl<T> Entry<T> {
    pub const fn empty() -> Self {
        Self {
            offset_low: 0,
            segment_selector: 0,
            settings: IdtSettings::new().with_gate_type(GateType::TrapGate),
            offset_middle: 0,
            offset_high: 0,
            _reserved: 0,
            _pd: PhantomData,
        }
    }
}

impl<T> Debug for Entry<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Entry")
            .field(
                "addr",
                &CanonicalAddr(
                    self.offset_low as u64
                        | ((self.offset_middle as u64) << 16)
                        | ((self.offset_high as u64) << 32),
                ),
            )
            .field("segment_selector", &self.segment_selector)
            .field("settings", &self.settings)
            .field("_reserved", &self._reserved)
            .field("_pd", &self._pd)
            .finish()
    }
}

macro_rules! impl_entry_builder {
    ($funct:ty) => {
        impl Entry<$funct> {
            pub fn set_handler_func(&mut self, func: $funct) {
                let ptr = func as u64;
                let lower = ptr as u16;
                let middle = (ptr >> 16) as u16;
                let upper = (ptr >> 32) as u32;
                (self.offset_low, self.offset_middle, self.offset_high) = (lower, middle, upper);

                self.offset_high = (0xFFFF << 16) | self.offset_high;
                self.settings = self
                    .settings
                    .with_present(true)
                    .with_privilege_level(0)
                    .with_gate_type(GateType::TrapGate);

                self.segment_selector = CS::get_reg();
            }
        }
    };
}

#[test_case]
pub fn test_idt_entry_size() {
    assert_eq!(core::mem::size_of::<Entry<HandlerFunc>>(), 16);
}

#[test_case]
pub fn test_idt_size() {
    assert_eq!(core::mem::size_of::<IDT>(), 16 * 256);
}
#[derive(Debug, Clone, Copy)]
pub struct ExcptionStackFrame {
    pub instruction_pointer: u64,

    pub code_segment: u64,

    pub cpu_flags: u64,

    pub stack_pointer: u64,

    pub stack_segment: u64,
}

pub type HandlerFunc = extern "x86-interrupt" fn(ExcptionStackFrame);
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(ExcptionStackFrame, error_code: u64);
pub type PageFaultHandlerFunc = extern "x86-interrupt" fn(ExcptionStackFrame, error_code: u64);
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(ExcptionStackFrame) -> !;
pub type DivergingHandlerFuncWithErrCode =
    extern "x86-interrupt" fn(ExcptionStackFrame, error_code: u64) -> !;
pub type GeneralHandlerFunc = fn(ExcptionStackFrame, index: u8, error_code: Option<u64>);

impl_entry_builder!(HandlerFunc);
impl_entry_builder!(HandlerFuncWithErrCode);
// impl_entry_builder!(PageFaultHandlerFunc);
impl_entry_builder!(DivergingHandlerFunc);
impl_entry_builder!(DivergingHandlerFuncWithErrCode);
// pub struct IDT {
//     entries: [IdtEntry<HandlerFn>; 256],
// }

#[derive(Clone, Debug)]
#[repr(C)]
#[repr(align(16))]
pub struct IDT {
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

    pub cp_protection_exception: Entry<HandlerFuncWithErrCode>,

    reserved_2: [Entry<HandlerFunc>; 6],

    pub hv_injection_exception: Entry<HandlerFunc>,

    pub vmm_communication_exception: Entry<HandlerFuncWithErrCode>,

    pub security_exception: Entry<HandlerFuncWithErrCode>,

    reserved_3: Entry<HandlerFunc>,

    interrupts: [Entry<HandlerFunc>; 256 - 32],
}

pub struct IDTPointer {
    size: u16,
    offset: *const IDT,
}

impl IDT {
    pub const fn default() -> Self {
        Self {
            divide_error: Entry::empty(),
            debug: Entry::empty(),
            non_maskable_interrupt: Entry::empty(),
            breakpoint: Entry::empty(),
            overflow: Entry::empty(),
            bound_range_exceeded: Entry::empty(),
            invalid_opcode: Entry::empty(),
            device_not_available: Entry::empty(),
            double_fault: Entry::empty(),
            coprocessor_segment_overrun: Entry::empty(),
            invalid_tss: Entry::empty(),
            segment_not_present: Entry::empty(),
            stack_segment_fault: Entry::empty(),
            general_protection_fault: Entry::empty(),
            page_fault: Entry::empty(),
            reserved_1: Entry::empty(),
            x87_floating_point: Entry::empty(),
            alignment_check: Entry::empty(),
            machine_check: Entry::empty(),
            simd_floating_point: Entry::empty(),
            virtualization: Entry::empty(),
            cp_protection_exception: Entry::empty(),
            reserved_2: [Entry::empty(); 6],
            hv_injection_exception: Entry::empty(),
            vmm_communication_exception: Entry::empty(),
            security_exception: Entry::empty(),
            reserved_3: Entry::empty(),
            interrupts: [Entry::empty(); 256 - 32],
        }
    }
    pub fn load(&self) {
        let ptr = IDTPointer {
            size: core::mem::size_of::<IDT>() as u16 - 1,
            offset: self as *const _,
        };

        unsafe { lidt(&ptr) };
    }
}

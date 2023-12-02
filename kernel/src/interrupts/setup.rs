use core::f32::consts::PI;

use crate::devices::keyboard::Keyboard;
use crate::devices::pic8259::*;
use crate::devices::pit::PIT;
use crate::interrupts::keyboard::keyboard_interrupt;
use crate::interrupts::timer::timer_interrupt;
use crate::io::reader::READER;
use crate::{descriptors::idt::*, kprintln, sync::shitlock::Racy};
use crate::{error, info};
use lazy_static::lazy_static;

lazy_static! {
    static ref _IDT: Racy<InterruptDescriptorTable> = Racy::from(InterruptDescriptorTable::new());
}

pub const PIC: Pic8259 = Pic8259::new();

extern "x86-interrupt" fn double_fault_handler(frame: ExceptionStackFrame, err: u64) -> ! {
    error!("Double fault");
    loop {}
}

extern "x86-interrupt" fn breakpoint_handler(frame: ExceptionStackFrame) {}

extern "x86-interrupt" fn segment_not_present_handler(frame: ExceptionStackFrame, err_code: u64) {
    kprintln!("Segment not present error");
    // PIC.eoi(1);
}
extern "x86-interrupt" fn page_fault_handler(frame: ExceptionStackFrame) {
    kprintln!("Segment not present error");
    // PIC.eoi(1);
}

pub fn interrupt_setup() {
    PIC.remap(0x20, 0x28);

    _IDT.take().interrupts[0]
        .set_handler_fn(timer_interrupt)
        .options
        .set_gate_type(GateType::InterruptGate); // timer

    _IDT.take().interrupts[1]
        .set_handler_fn(keyboard_interrupt)
        .options
        .set_gate_type(GateType::TrapGate); // timer

    _IDT.take().breakpoint.set_handler_fn(breakpoint_handler);
    _IDT.take()
        .double_fault
        .set_handler_fn(double_fault_handler);
    _IDT.take()
        .segment_not_present
        .set_handler_fn(segment_not_present_handler);

    _IDT.take().page_fault.set_handler_fn(page_fault_handler);

    _IDT.take_static().load();
}

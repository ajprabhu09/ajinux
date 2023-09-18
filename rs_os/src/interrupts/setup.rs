use core::f32::consts::PI;

use crate::devices::keyboard::Keyboard;
use crate::devices::pic8259::*;
use lazy_static::lazy_static;

use crate::info;
use crate::{descriptors::idt::*, println, sync::shitlock::Racy};

lazy_static! {
    static ref _IDT: Racy<InterruptDescriptorTable> = Racy::from(InterruptDescriptorTable::new());
}

const PIC: Pic8259 = Pic8259::new();
lazy_static! {
    static ref KEYBOARD: Racy<Keyboard> = Racy::from(Keyboard::default());
}



extern "x86-interrupt" fn double_fault_handler(frame: ExceptionStackFrame, err: u64) -> ! {
    println!("Double fault!! {:#?}", frame);
    loop {}
}

extern "x86-interrupt" fn breakpoint_handler(frame: ExceptionStackFrame) {
    println!("Breakpoint {:#?}", frame);
}

extern "x86-interrupt" fn timer_interrupt(frame: ExceptionStackFrame) {
    let ptr = frame.instruction_pointer as *const u64;
    PIC.eoi(0);
}

extern "x86-interrupt" fn keyboard_interrupt(frame: ExceptionStackFrame) {
    let scan_code = KEYBOARD.take().read_key();
    info!("Scanned code: {:?}", scan_code);
    PIC.eoi(1);
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
    // _IDT.take().double_fault.set_handler_fn(double_fault_handler);

    _IDT.take_static().load();
}

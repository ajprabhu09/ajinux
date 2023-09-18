use lazy_static::lazy_static;
use crate::interrupts::pic8259::*;

use crate::{descriptors::idt::*, println, sync::shitlock::Racy};

lazy_static! {
    static ref _IDT: Racy<InterruptDescriptorTable> = Racy::from(InterruptDescriptorTable::new());
}

extern "x86-interrupt" fn double_fault_handler(frame: ExceptionStackFrame, err: u64) -> ! {
    println!("Double fault!! {:#?}", frame);
    loop {}
}

extern "x86-interrupt" fn breakpoint_handler(frame: ExceptionStackFrame) {
    println!("Breakpoint {:#?}", frame);
}

extern "x86-interrupt" fn timer_interrupt(frame: ExceptionStackFrame) {
    println!("Timer {:#?}", frame);

    Pic8259::new().eoi(0);

    let ptr = frame.instruction_pointer as *const u64;
}


pub fn interrupt_setup() {
    let pic = Pic8259::new();

    pic.remap(0x20, 0x28);


    _IDT.take().interrupts[0].set_handler_fn(timer_interrupt);

    _IDT.take().breakpoint.set_handler_fn(breakpoint_handler);
    // _IDT.take().double_fault.set_handler_fn(double_fault_handler);
    
    _IDT.take_static().load();
}

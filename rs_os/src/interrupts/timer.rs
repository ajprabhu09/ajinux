use crate::{descriptors::idt::InterruptStackFrame, println};

pub extern "x86-interrupt" fn timer_interrupt_handler(stackframe : InterruptStackFrame) {
    println!("Hello");
}


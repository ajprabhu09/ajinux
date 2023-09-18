use crate::{descriptors::{idt::*}, println};
 


extern "x86-interrupt" fn bkpt_handle2(frame: ExceptionStackFrame) {
    println!("{:#?}", frame);
}
static mut _IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn interrupt_setup() {
    unsafe {
        _IDT.breakpoint.set_handler_fn(bkpt_handle2);
        _IDT.load();
    };
}

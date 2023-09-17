use crate::{
    descriptors::idt::{ExcptionStackFrame, IDT},
    println, error,
};

static mut _IDT: IDT = IDT::default();
extern "x86-interrupt" fn bkpt_handle(frame: ExcptionStackFrame) {}

extern "x86-interrupt" fn dble_fault(frame: ExcptionStackFrame, err: u64) -> !{

    error!("Double fault");
    loop {}
    
}

extern "x86-interrupt" fn gpf(esf: ExcptionStackFrame, error_code: u64) {
    error!("General ")
}
pub fn interrupt_setup() {
    unsafe {
        _IDT.breakpoint.set_handler_func(bkpt_handle);
        _IDT.double_fault.set_handler_func(dble_fault);
        _IDT.general_protection_fault.set_handler_func(gpf);
        println!("{:?}", _IDT.breakpoint);
        _IDT.load();
    };
}

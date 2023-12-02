use crate::{descriptors::idt::ExceptionStackFrame, io::reader::READER};

use super::setup::PIC;

pub extern "x86-interrupt" fn keyboard_interrupt(_frame: ExceptionStackFrame) {
    let _scan_code = READER.take().input.read_into_buf();
    // info!("Scanned code: {:?}", scan_code);
    // println!("jerer");
    PIC.eoi(1);
}

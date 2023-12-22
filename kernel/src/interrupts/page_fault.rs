use crate::{
    addr::VirtAddr,
    descriptors::{
        idt::ExceptionStackFrame,
        reg::{GetReg, CR2, CR3},
    },
    paging::Table,
    serial_debug,
};

pub extern "x86-interrupt" fn page_fault_handler(_frame: ExceptionStackFrame) {
    let reg_val = VirtAddr::as_canonical(CR2::get_reg());
    let cr3_ = VirtAddr::as_canonical(CR3::get_reg());
    let mut table = Table::from_addr::<512>(cr3_);
    serial_debug!("PAGE FAULT!");
    reg_val.map_addr(&mut table);

    // PIC.
}

use crate::{
    addr::VirtAddr,
    descriptors::{
        idt::ExceptionStackFrame,
        reg::{self, GetReg, CR2, CR3},
    },
    paging::Table,
    serial_info,
};

pub extern "x86-interrupt" fn page_fault_handler(_frame: ExceptionStackFrame) {
    let reg_val = VirtAddr::as_canonical(CR2::get_reg());
    let cr3_ = VirtAddr::as_canonical(CR3::get_reg());
    let mut table = Table::from_addr::<512>(cr3_);
    reg_val.map_addr(&mut table);
    serial_info!("page faulty faulting address: {:?}", reg_val);

    // PIC.
}

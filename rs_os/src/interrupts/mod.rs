pub mod timer;

use crate::{devices::pic::pic_init, info};

const X86_PIC_MASTER_IRQ_BASE: u8 = 0x20;
/** @brief Default location of the slave  PIC's interrupts in the IDT */
const X86_PIC_SLAVE_IRQ_BASE: u8 = 0x28;

pub fn interrupt_setup() {
    info!("Setting up PIC");
    pic_init(X86_PIC_MASTER_IRQ_BASE, X86_PIC_SLAVE_IRQ_BASE);
}

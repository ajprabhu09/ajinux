pub mod timer;

use crate::{
    descriptors::idt::IDT,
    descriptors::idt::{Entry, InterruptIdx, HandlerFunc},
    devices::pic::pic_init,
    info, interrupts::timer::timer_interrupt_handler, utils::asm,
};

const X86_PIC_MASTER_IRQ_BASE: u8 = 0x20;
/** @brief Default location of the slave  PIC's interrupts in the IDT */
const X86_PIC_SLAVE_IRQ_BASE: u8 = 0x28;

pub fn interrupt_setup() {
    info!("Setting up PIC");
    unsafe { IDT.interrupts[InterruptIdx::Timer as usize] = Entry::nil().with_entry_fn(timer_interrupt_handler as HandlerFunc) };
    unsafe { IDT.load(); }
    pic_init(X86_PIC_MASTER_IRQ_BASE, X86_PIC_SLAVE_IRQ_BASE);

}

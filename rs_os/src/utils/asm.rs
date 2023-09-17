use core::arch::asm;


pub unsafe fn outb(port: u16, val: u8) {
    let dx: u16;
    let al: u8;

    asm!(
        "movw {0:x}, %dx",
        "movb {1}, %al",
        "out %al, %dx",
        in(reg) port,
        in(reg_byte) val,
        out("dx") dx,
        out("al") al,
        options(att_syntax)
    );
}

pub unsafe fn iodelay() {
    asm!(
        "pushq %rax",
        "inb $0x08,  %al",
        "inb $0x08,  %al",
        "popq %rax",
        options(att_syntax, nostack),
    )
}

pub unsafe fn inb(port: u16) -> u8 {
    let dx: u16;
    let al: u8;
    asm!(
        "xorq %rax, %rax",
        "movw {0:x}, %dx",
        "in %dx, %al",
        in(reg) port,
        out("dx") dx,
        out("al") al,
        options(att_syntax, nostack),
    );
    return al;
}

pub unsafe fn disable_interrupts() {
    asm!("cli")
}
pub unsafe fn enable_interrupts() {
    asm!("sti")
}

// pub unsafe fn lidt(idt: &InterruptDescriptorPointer) {
//     asm!("lidt ({})", in(reg) idt,  options(att_syntax, nostack));
// }

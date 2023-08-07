use core::arch::asm;

/// Example
pub fn add(a: i64) -> i64 {
    let o: i64;
    unsafe {
        asm!(
            "movq {i1}, {o1}",
            "addq $5, {o1}",
            o1 = out(reg) o,
            i1 = in(reg) a,
            options(att_syntax)
        );
    }
    return o;
}
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
    // .global inb
    // inb:
    //     xorl %eax, %eax
    //     movw 4(%esp), %dx
    //     in %dx, %al
    //     ret
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

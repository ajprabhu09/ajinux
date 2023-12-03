use core::arch::asm;

// use x86_64::{structures::DescriptorTablePointer, VirtAddr};

use crate::{
    descriptors::{gdt::GdtPointer, idt::DescriptorPointer},
    info, serial_info,
};

pub unsafe fn outb(port: u16, val: u8) {
    let _dx: u16;
    let _al: u8;

    asm!(
        "movw {0:x}, %dx",
        "movb {1}, %al",
        "out %al, %dx",
        in(reg) port,
        in(reg_byte) val,
        out("dx") _dx,
        out("al") _al,
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
    let _dx: u16;
    let al: u8;
    asm!(
        "xorq %rax, %rax",
        "movw {0:x}, %dx",
        "in %dx, %al",
        in(reg) port,
        out("dx") _dx,
        out("al") al,
        options(att_syntax, nostack),
    );
    return al;
}

#[allow(dead_code)]
pub unsafe fn disable_interrupts() {
    asm!("cli")
}
#[allow(dead_code)]
pub unsafe fn enable_interrupts() {
    asm!("sti")
}

#[allow(dead_code)]
pub unsafe fn lgdt(gdt_p: &GdtPointer) {
    asm!("lgdt [{}]", in(reg) gdt_p, options(readonly, nostack, preserves_flags));
}

#[allow(dead_code)]
#[inline]
pub unsafe fn lidt(idt_p: &DescriptorPointer) {
    asm!("lidt [{}]", in(reg) idt_p, options(readonly, nostack, preserves_flags));
}

#[allow(dead_code)]
pub unsafe fn sgdt() -> GdtPointer {
    let pointer: GdtPointer = GdtPointer {
        size: 0,
        offset: 0 as *const _,
    };
    asm!("sgdt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags));
    serial_info!(" Pointer {:#?}", pointer);
    return pointer;
}

pub unsafe fn int3() {
    asm!("int3")
}
#[allow(dead_code)]
pub unsafe fn sidt() -> DescriptorPointer {
    let pointer: DescriptorPointer = DescriptorPointer { size: 0, offset: 0 };
    asm!("sidt [{}]", in(reg) &pointer, options(readonly, nostack, preserves_flags));
    info!(" Pointer {:#?}", pointer);
    return pointer;
}

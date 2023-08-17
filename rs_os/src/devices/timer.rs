use crate::utils::asm;
use crate::utils::bytes::{Component, TopLevel};
/// Timer
const TIMER_RATE: u32 = 1193182;
const TIMER_IDT_ENTRY: usize = 0x20;
const TIMER_PERIOD_IO_PORT: u16 = 0x40;
const TIMER_MODE_IO_PORT: u16 = 0x43;
const TIMER_SQUARE_WAVE: u8 = 0x36;
const TIMER_ONE_SHOT: usize = 0x30;

pub fn setup_timer(interval_time_s: u32) {
    unsafe {
        asm::outb(TIMER_MODE_IO_PORT, TIMER_SQUARE_WAVE);
        let value = (interval_time_s * TIMER_RATE) as u16;
        let tplvl = TopLevel { word: value };
        asm::outb(TIMER_PERIOD_IO_PORT, tplvl.component.lsb);
        asm::outb(TIMER_PERIOD_IO_PORT, tplvl.component.msb);
    }
}

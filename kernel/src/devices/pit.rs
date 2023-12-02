#![allow(dead_code)]
#![allow(unused)]
use crate::{info, kprintln};

use super::port::Port;

const TIMER_RATE: u32 = 1193182;
const TIMER_IDT_ENTRY: usize = 0x20;
const TIMER_PERIOD_IO_PORT: u16 = 0x40;
const TIMER_MODE_IO_PORT: u16 = 0x43;
const TIMER_SQUARE_WAVE: u8 = 0x36;
const TIMER_ONE_SHOT: u8 = 0x30;

#[derive(Clone, Copy)]
pub struct PitTimerEvent;

#[allow(unused)]
pub struct PIT {
    ch0: Port,
    ch1: Port,
    ch2: Port,
    cmd: Port,
}

impl PIT {
    pub const fn new() -> Self {
        Self {
            ch0: Port(TIMER_MODE_IO_PORT),
            ch1: Port(TIMER_MODE_IO_PORT + 1),
            ch2: Port(TIMER_MODE_IO_PORT + 2),
            cmd: Port(TIMER_MODE_IO_PORT + 3),
        }
    }

    pub fn setup(&self, interval_ms: u32) {
        let count = (TIMER_RATE * 2 * interval_ms) / 1000;
        kprintln!("Setting count to {:?}", count);
        let lsb = (count) as u8;
        info!("lsb :{:?}", lsb);
        let msb = (count >> 8) as u8;
        info!("msb :{:?}", msb);

        self.cmd.send_byte(TIMER_SQUARE_WAVE);
        self.ch0.send_byte(lsb);
        self.ch0.send_byte(msb);
    }
}

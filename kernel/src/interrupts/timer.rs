use core::sync::atomic::{AtomicI64, AtomicIsize};

use crate::descriptors::idt::ExceptionStackFrame;
use crate::devices::pit::PIT;
use crate::interrupts::setup::PIC;

pub static mut PIT_: PIT = PIT::new();

pub struct TimerEvents {
    pub count: AtomicIsize,
}
impl TimerEvents {
    pub fn new(&mut self) {
        self.count
            .fetch_add(1, core::sync::atomic::Ordering::AcqRel);
    }
    pub const fn default() -> Self {
        Self {
            count: AtomicIsize::new(0),
        }
    }
}

pub static mut TIMER_EVENTS: TimerEvents = TimerEvents::default();

pub extern "x86-interrupt" fn timer_interrupt(frame: ExceptionStackFrame) {
    // let ptr = frame.instruction_pointer as *const u64;
    unsafe { TIMER_EVENTS.new() };
    PIC.eoi(0);
}


/// Timer
const TIMER_RATE: usize = 1193182;
const TIMER_IDT_ENTRY: usize = 0x20;
const TIMER_PERIOD_IO_PORT: usize = 0x40;
const TIMER_MODE_IO_PORT: usize = 0x43;
const TIMER_SQUARE_WAVE: usize = 0x36;
const TIMER_ONE_SHOT: usize = 0x30;
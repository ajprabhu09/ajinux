use core::{fmt, borrow::BorrowMut};

use crate::writer::WRITER;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::mystd::vga_print::_print(format_args!($($arg)*)));
}


#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::mystd::vga_print::print!("{}\n", format_args!($($arg)*));
    }};
}


#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.display.borrow_mut().write_fmt(args);
}
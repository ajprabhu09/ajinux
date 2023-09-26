use crate::io::writer::WRITER;
use core::{borrow::BorrowMut, fmt};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::logging::vga_print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER
        .take()
        .borrow_mut()
        .write_fmt(args)
        .expect("failed to write during _print");
}


#[macro_export]
macro_rules! info {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::mystd::vga_print::print!("[INFO] {}\n", format_args!($($arg)*));
    }};
}

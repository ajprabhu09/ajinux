

#[macro_export]
macro_rules! info {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("[INFO] {}\n", format_args!($($arg)*));
    }};
}

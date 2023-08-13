#[macro_export]
macro_rules! info {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("[INFO] {}\n", format_args!($($arg)*));
    }};
}
#[macro_export]
macro_rules! debug {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("[DEBUG] {}\n", format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::print!("[ERROR] {}\n", format_args!($($arg)*));
    }};
}

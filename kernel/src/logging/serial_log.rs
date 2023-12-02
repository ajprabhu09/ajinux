#[macro_export]
macro_rules! serial_info {
    () => {
        $crate::ksprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::ksprint!("[INFO] {}\n", format_args!($($arg)*));
    }};
}
#[macro_export]
macro_rules! serial_debug {
    () => {
        $crate::ksprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::ksprint!("[DEBUG] {}\n", format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! serial_error {
    () => {
        $crate::ksprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::ksprint!("[ERROR] {}\n", format_args!($($arg)*));
    }};
}

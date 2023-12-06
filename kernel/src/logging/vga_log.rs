#[macro_export]
macro_rules! info {
    () => {
        $crate::kprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::kprint!("[INFO] {}\n", format_args!($($arg)*));
    }};
}
#[macro_export]
macro_rules! debug {
    () => {
        $crate::kprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::kprint!("[DEBUG] {}\n", format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    () => {
        $crate::kprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::kprint!("[ERROR] {}\n", format_args!($($arg)*));
    }};
}

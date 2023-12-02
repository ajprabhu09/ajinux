use crate::{io::writer::{SERIAL_WRITER}, devices::serial::{COM1, SerialCom}};
use core::{fmt};


const SERIAL: SerialCom = SerialCom::new(COM1);

#[macro_export]
macro_rules! ksprint {
    ($($arg:tt)*) => ($crate::logging::serial_print::_sprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ksprintln {
    () => {
        $crate::ksprint!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::ksprint!("{}\n", format_args!($($arg)*));
    }};
}

#[doc(hidden)]
pub fn _sprint(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_WRITER.take().write_fmt(args).unwrap();
}

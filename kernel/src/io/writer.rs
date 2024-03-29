use crate::devices::serial::{SerialCom, COM1};
use crate::devices::vga::{ConsoleDisplay, VGADisplay};
use crate::sync::shitlock::Racy;
pub struct Writer<T: ConsoleDisplay> {
    pub display: T,
}

lazy_static::lazy_static! {
    pub static ref WRITER: Racy<Writer<VGADisplay>> = Racy::from(Writer{
        display: VGADisplay::default(),
    });
    pub static ref SERIAL_WRITER: Racy<Writer<SerialCom>> = Racy::from(Writer{
        display: SerialCom::new(COM1).connect().unwrap(),
    });

}

pub fn set_color(color: u8) {
    WRITER.take().display.set_term_color(color);
}

impl<T> core::fmt::Write for Writer<T>
where
    T: ConsoleDisplay,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.display.put_bytes(s.as_bytes()).map_err(|err| {
            panic!("{:?}", err);
        })
    }
}

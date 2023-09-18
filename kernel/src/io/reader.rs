use crate::{devices::keyboard::{ConsoleInput, Keyboard}, sync::shitlock::Racy};

pub struct Reader<T: ConsoleInput> {
    pub input: T
}
lazy_static::lazy_static! {
    pub static ref READER: Racy<Reader<Keyboard>> = Racy::from(Reader {
        input: Keyboard::default()
    });
}
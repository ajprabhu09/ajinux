// FIXME: This not crossplatform
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Component {
    pub lsb: u8,
    pub msb: u8,
}
#[repr(C)]
pub union TopLevel {
    pub word: u16,
    pub component: Component,
}

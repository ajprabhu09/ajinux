use core::fmt::{write, Debug};

#[repr(transparent)]
pub struct CanonicalAddr(pub u64);

impl Debug for CanonicalAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("CanonicalAdd(")?;
        write!(f, "{:#08x}", self.0)?;
        f.write_str(")")
    }
}



use user_tests::bytes::*;

pub struct Program(pub *const u8);

pub trait CodeLoader {
    fn load(name: &str) -> &[u8];
}

pub struct UserTestLoader;

impl CodeLoader for UserTestLoader {
    fn load(name: &str) -> &[u8] {
        match name {
            "simple" => simple,
            _ => panic!("unknown test"),
        }
    }
}

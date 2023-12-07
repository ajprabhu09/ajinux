
use core::ptr::NonNull;

use user_tests::bytes::*;


pub struct Program(pub *const u8);

pub trait CodeLoader {
    fn load(name: &str) -> Program;
}

struct UserTestLoader;

impl CodeLoader for UserTestLoader {
    fn load(name: &str) -> Program {
        match name {
            "simple" => Program(simple.as_ptr()),
            _ => panic!("unknown test")
        }
    }
}

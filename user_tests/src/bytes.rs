macro_rules! user_mode_test {
    ($test:ident) => {
        pub const $test: &[u8] = include_bytes!(concat!("../../user/tests/progs/",stringify!($test), ".o"));
    }
}

user_mode_test!(simple);
user_mode_test!(other_simple);
pub const asdasd: usize = 1;
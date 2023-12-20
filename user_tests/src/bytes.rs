macro_rules! user_mode_test {
    ($test:ident) => {
        pub const $test: &[u8] = include_bytes!(concat!(
            "../../user/tests/progs/",
            stringify!($test),
            ".out"
        ));
    };
}

user_mode_test!(simple);
pub const asdasd: usize = 1;

fn main() {
    // This does not work yet!
    cc::Build::new()
        .flag("--target=x86_64-unknown-none")
        .file("cc/test.c")
        .compile("foo");
}

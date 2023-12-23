fn main() {
    // This does not work yet!
    // TODO: Change this to /opt/homebrew/Cellar/x86_64-unknown-linux-gnu/7.2.0/bin/x86_64-unknown-linux-gnu-gcc
    // println!("cargo:rerun-if-changed=cc/src/*.c");
    // println!("cargo:rerun-if-changed=cc/inc/*.h");
    // println!("cargo:rerun-if-changed=cc/asm/*.s");
    // let include_path = "cc/inc/";
    // let mut build = cc::Build::new();

    // for entry in glob::glob("cc/src/*.c").expect("invalid path cc/src/ ") {
    //     if let Ok(path) = entry {
    //         build
    //             .flag("--target=x86_64-unknown-none")
    //             .file(path.display().to_string())
    //             .include(include_path);
    //     }
    // }
    // for entry in glob::glob("cc/asm/*.s").expect("invalid path cc/asm/ ") {
    //     if let Ok(path) = entry {
    //         build
    //             .flag("--target=x86_64-unknown-none")
    //             .file(path.display().to_string())
    //             .include(include_path);
    //     }
    // }
    // build.compile("ajinuxcc");
}

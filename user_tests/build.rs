use std::process::Command;

fn main() {
    // This does not work yet!
    println!("cargo:rerun-if-changed=../user/tests/progs/*.c");
    // println!("cargo:rerun-if-changed=cc/inc/*.h");
    // println!("cargo:rerun-if-changed=cc/asm/*.s");
    // let x: i32 = "1asd".parse().unwrap();
    // panic!("hehehe");

    let _x = Command::new("/usr/bin/make")
        .arg("-f")
        .arg("../user/tests/Makefile")
        .arg("clean")
        .output()
        .expect("failed to build user tests");

    let _x = Command::new("make")
        .arg("-f")
        .arg("../user/tests/Makefile")
        .output()
        .expect("failed to build user tests");
    // build.compile("ajinuxcc");
}

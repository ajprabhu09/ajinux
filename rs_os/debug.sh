qemu-system-x86_64 -S -s -drive format=raw,file=target/x86_target/debug/bootimage-rs_os.bin -d int -no-reboot &

rust-lldb target/x86_target/debug/rs_os -s startup.lldb



qemu-system-x86_64 -S -s -drive  format=raw,file=target/x86_target/debug/bootimage-os.bin -d int -no-reboot -serial stdio &

rust-lldb target/x86_target/debug/os -s startup.lldb



# An x86-64 bit kernel written in rust


## Debugging

Run `qemu-system-x86_64 -S -s -drive format=raw,file=target/x86_target/debug/bootimage-rs_os.bin -d int -no-reboot` in one terminal and
`rust-lldb target/x86_target/debug/rs_os` in another terminal and in the lldb terminal run 

`gdb-remote 1234`

the gdb session can be reused for each run

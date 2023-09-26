use crate::{interrupts, utils::asm, info};

#[test_case]
fn test_breakpoint() {
    interrupts::setup::interrupt_setup();
    // unsafe { utils::asm::enable_interrupts() }; // this fails if no handler is installed
    unsafe { asm::int3() };
    info!("Breakpoint interrupt tested");
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::println;

    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

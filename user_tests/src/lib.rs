#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(allocator_api)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#![allow(clippy::empty_loop)]
#![allow(clippy::needless_return)]
#![feature(let_chains)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

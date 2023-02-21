#![no_std]
#![feature(const_trait_impl)]

extern crate minrt_sys;
extern crate seven_sys;

pub mod sync;
pub mod panic;
pub mod irq;


#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

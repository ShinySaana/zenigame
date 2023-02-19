#![no_std]

extern crate minrt_sys;
extern crate seven_sys;

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

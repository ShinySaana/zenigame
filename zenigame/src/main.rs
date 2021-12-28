#![no_std]
#![no_main]
#![feature(asm)]

use core::{hint::unreachable_unchecked};

#[panic_handler]
#[allow(unused)]
unsafe fn panic(info: &core::panic::PanicInfo) -> ! {
    asm!(
        "swi 0"
    );

    unreachable_unchecked()
}


#[no_mangle]
fn main() -> ! {
    unsafe {
        let p = 0x04000000 as *mut u16;
        p.write_volatile(0x0);

        let p = 0x05000000 as *mut u16;
        p.write_volatile(0x3E0);
    }

    loop {}
}

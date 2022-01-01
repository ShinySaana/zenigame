#![no_std]
#![no_main]

use core::{hint::unreachable_unchecked};

use seven_sys::{irq::{svcVBlankIntrWait, irqInitDefault, irqEnable, IRQ_VBLANK}, log::{logInit, logOutput, LogInterface, logInitInterface, LogLevel}};
use zenigame::{irq::CriticalSection, write_reg16, log::{initialize_logger, self, enable_logging}};

#[panic_handler]
#[allow(unused)]
unsafe fn panic(info: &core::panic::PanicInfo) -> ! {
    core::arch::asm!(
        "swi 0"
    );

    unreachable_unchecked()
}

#[no_mangle]
fn main() -> ! {
    unsafe {
        write_reg16(0x04000000, 0x0);
        write_reg16(0x05000000, 0x3E0);

        irqInitDefault();
        irqEnable(IRQ_VBLANK);

        write_reg16(0x04000004, 0b_0000_0000_0000_1000)
    }

    initialize_logger();
    // log::log(LogLevel::Debug, b"YOLO\0".as_ptr());

    let mut counter = 0;

    loop {
        counter += 1;
        unsafe {
            write_reg16(0x05000000, counter & 0x7FFF);
            svcVBlankIntrWait();
        }
    }
}

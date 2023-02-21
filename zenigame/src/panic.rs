use core::{panic::PanicInfo};

use crate::sync::{Swappable, SyncedU8};

static mut PANICKED: SyncedU8 = SyncedU8::new(0);

pub type PanicCallback = fn();
static mut PANIC_CALLBACK: Option<PanicCallback> = None;

pub fn has_panicked() -> bool {
    unsafe {
        PANICKED.get() == 1
    }
}

pub fn set_panic_callback(cb: PanicCallback) {
    unsafe {
        PANIC_CALLBACK = Some(cb);
    }
}

#[panic_handler]
#[instruction_set(arm::a32)]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        let has_already_panicked = Swappable::swap(&mut PANICKED, 1);

        if has_already_panicked == 1 {
            double_panic();
        }
    
        match PANIC_CALLBACK {
            Some(cb) => {
                cb()
            }
            None => {}
        }
    }

    loop {}
}

fn double_panic() -> ! {
    unsafe {
        seven_sys::addresses::REG_IME.write(0);
        seven_sys::bindings::logOutput(
            seven_sys::bindings::LogLevel::LOG_FATAL as u8,
            b"panic!() called twice. This is wrong. Cya!\0".as_ptr().cast()
        );
        
        loop {}
    }
}

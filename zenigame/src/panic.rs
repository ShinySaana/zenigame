use core::panic::PanicInfo;


pub(crate) static mut PANICKED: bool = false;

pub type PanicCallback = fn();
static mut PANIC_CALLBACK: Option<PanicCallback> = None;

pub fn has_panicked() -> bool {
    return unsafe {
        PANICKED
    }
}

pub fn set_panic_callback(cb: PanicCallback) {
    unsafe {
        PANIC_CALLBACK = Some(cb);
    }
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        if PANICKED {
            double_panic();
        }

        PANICKED = true;
    
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

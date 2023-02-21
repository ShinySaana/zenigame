use core::{panic::PanicInfo, arch::asm};


static mut PANICKED: u8 = 0;

pub type PanicCallback = fn();
static mut PANIC_CALLBACK: Option<PanicCallback> = None;

pub fn has_panicked() -> bool {
    unsafe {
        PANICKED == 1
    }
}

pub fn set_panic_callback(cb: PanicCallback) {
    unsafe {
        PANIC_CALLBACK = Some(cb);
    }
}

#[instruction_set(arm::a32)]
#[inline]
unsafe fn arm_exchange_u8(dst: *mut u8, i: u8) -> u8 {
    let out;
    asm!("swpb {2}, {1}, [{0}]", in(reg) dst, in(reg) i, lateout(reg) out);
    out
}


#[instruction_set(arm::a32)]
#[inline]
unsafe fn arm_exchange_u32(dst: *mut u32, i: u32) -> u32 {
    let out;
    asm!("swp {2}, {1}, [{0}]", in(reg) dst, in(reg) i, lateout(reg) out);
    out
}

#[panic_handler]
#[instruction_set(arm::a32)]
pub fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        let has_already_panicked = arm_exchange_u8(&mut PANICKED, 1); 
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

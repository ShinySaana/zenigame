#![no_std]
#![feature(start)]

use core::panic;

use zenigame::sync::SyncedPtr;
use zenigame::sync::Swappable;

extern crate zenigame;

static mut MESSAGE: Option<SyncedPtr<u8>> = None;
static mut AA: *const u8 = b"AA\0".as_ptr();
static mut BB: *const u8 = b"BB\0".as_ptr();

fn print_message(_irqs: u16) {
    unsafe {
        let msg = MESSAGE.as_ref().unwrap().get();
        seven_sys::bindings::logOutput(5, msg.cast());
    }
}




fn vblank_callback(_irqs: u16) {
    use seven_sys::addresses::BG_PALETTE_COLOR;

    unsafe {
        BG_PALETTE_COLOR.index_mut(0, BG_PALETTE_COLOR.index(0) + 1);
    }
}


fn print_a(_irqs: u16) {
    unsafe {
        seven_sys::bindings::logOutput(5, b"AAAA\0".as_ptr().cast());
    }
}

fn print_z(_irqs: u16) {
    unsafe {
        seven_sys::bindings::logOutput(5, b"ZZZZ\0".as_ptr().cast());
    }
}


#[start]
fn main(_argc: isize, _argv: *const *const u8 ) -> isize {
    unsafe {
        MESSAGE = Some(SyncedPtr::new(AA.cast_mut()));

        seven_sys::bindings::irqInitDefault();
        seven_sys::bindings::irqEnable(seven_sys::bindings::IRQFlag::IRQ_VBLANK as u16);
        
        zenigame::irq::set_vblank_callback(print_message);
        zenigame::panic::set_panic_callback(|| {
            seven_sys::bindings::logOutput(5, b"panicked\0".as_ptr().cast());
            panic!();
        });
        
        seven_sys::addresses::REG_DISPCNT.write(1);

        seven_sys::bindings::logInit();
        seven_sys::bindings::logSetMaxLevel(5);
        seven_sys::bindings::logOutput(5, b"hello\0".as_ptr().cast());

        let mut i: u8 = 0;
        loop {
            if i % 2 == 0 {
                MESSAGE.as_mut().unwrap().swap(AA.cast_mut());
            } else {
                MESSAGE.as_mut().unwrap().swap(BB.cast_mut());
            }

            if i > 30 {
                panic!();
            }
            i = i + 1;
            seven_sys::bindings::biosVBlankIntrWait();
        }
    }
}

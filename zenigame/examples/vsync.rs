#![no_std]
#![feature(start)]

extern crate zenigame;

fn vblank_callback(_irqs: u16) {
    use seven_sys::addresses::BG_PALETTE_COLOR;

    unsafe {
        BG_PALETTE_COLOR.index_mut(0, BG_PALETTE_COLOR.index(0) + 1);
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8 ) -> isize {
    unsafe {
        seven_sys::bindings::irqInitDefault();
        seven_sys::bindings::irqEnableFull(seven_sys::bindings::IRQ::IRQ_VBLANK as u16);
        
        zenigame::irq::set_vblank_callback(vblank_callback);
        
        seven_sys::addresses::REG_DISPCNT.write(1);

        seven_sys::bindings::logInit();
        seven_sys::bindings::logSetMaxLevel(5);
        seven_sys::bindings::logOutput(5, b"hello\0".as_ptr().cast());

        loop {
            seven_sys::bindings::biosVBlankIntrWait();
        }
    }
}

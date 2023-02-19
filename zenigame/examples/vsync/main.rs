#![no_std]
#![feature(start)]

extern crate zenigame;

extern "C" fn vblank_callback(_irqs: u16) {
    use seven_sys::addresses::BG_PALETTE;

    unsafe {
        BG_PALETTE.set(0, BG_PALETTE.get(0) + 1)
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8 ) -> isize {
    unsafe {
        seven_sys::bindings::irqInitDefault();
        seven_sys::bindings::irqEnableFull(seven_sys::bindings::IRQ::IRQ_VBLANK as u16);
        seven_sys::bindings::irqHandlerSet(seven_sys::bindings::IRQ::IRQ_VBLANK as u16, Some(vblank_callback));

        seven_sys::addresses::REG_DISPCNT.write(1);

        loop {
            seven_sys::bindings::biosVBlankIntrWait();
        }
    }
}

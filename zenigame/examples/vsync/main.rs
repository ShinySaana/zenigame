#![no_std]
#![feature(start)]

extern crate zenigame;

extern "C" fn vblank_callback(_irqs: u16) {
    let bg_palette_addr: usize = 0x0500_0000;
    let bg_palette_ptr = bg_palette_addr as *mut u16;
    unsafe {
        *bg_palette_ptr = *bg_palette_ptr + 1;
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8 ) -> isize {
    unsafe {
        seven_sys::bindings::irqInitDefault();
        seven_sys::bindings::irqEnableFull(seven_sys::bindings::IRQ::IRQ_VBLANK as u16);
        seven_sys::bindings::irqHandlerSet(seven_sys::bindings::IRQ::IRQ_VBLANK as u16, Some(vblank_callback));

        seven_sys::addresses::REG_DISPCNT.write(1);

        seven_sys::bindings::logInit();
        seven_sys::bindings::logSetMaxLevel(5);

        seven_sys::addresses::REG_BGCNT.set(1, 32);
        let value = seven_sys::addresses::REG_BGCNT.get(1);

        let mut buffer = [0u8; 256];
        buffer[0] = value as u8 + b'0';
       
        loop {
            seven_sys::bindings::logOutput(5, buffer.as_ptr().cast());
            seven_sys::bindings::biosVBlankIntrWait();
        }
    }
}

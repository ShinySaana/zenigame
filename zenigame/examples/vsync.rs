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
        
        seven_sys::bindings::irqEnableFull(seven_sys::bindings::IRQ_IRQ_VBLANK as u16);
        
        seven_sys::bindings::irqHandlerSet(seven_sys::bindings::IRQ_IRQ_VBLANK as u16, Some(vblank_callback));

        let reg_dispcnt_addr: usize = 0x0400_0000;
        let reg_dispcnt_ptr = reg_dispcnt_addr as *mut u16;
        reg_dispcnt_ptr.write_volatile(0);

        loop {
            seven_sys::bindings::biosVBlankIntrWait();
        }
    }
}

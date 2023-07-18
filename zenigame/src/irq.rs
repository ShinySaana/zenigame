use crate::panic::has_panicked;

pub type IrqHandlerFn = fn(arg: u16);

static mut VBLANK_CALLBACK: Option<IrqHandlerFn> = None;

extern "C" fn vblank(arg: u16) {
    unsafe {
        match VBLANK_CALLBACK {
            Some(cb) => {
                if !has_panicked() {
                    cb(arg)
                }
            },
            None => {}
        }
    }
}

pub fn set_vblank_callback(cb: IrqHandlerFn) {
    unsafe {
        VBLANK_CALLBACK = Some(cb);
        seven_sys::bindings::irqHandlerSet(seven_sys::bindings::IRQFlag::IRQ_VBLANK as u16, Some(vblank));
    }

}

pub fn swap_vblank_callback(cb: IrqHandlerFn) -> Option<IrqHandlerFn> {
    unsafe {
        let old = VBLANK_CALLBACK.replace(cb);
        seven_sys::bindings::irqHandlerSet(seven_sys::bindings::IRQFlag::IRQ_VBLANK as u16, Some(vblank));
        old
    }

}

pub fn unset_vblank_callback() -> Option<IrqHandlerFn> {
    unsafe {
        VBLANK_CALLBACK.take()
    }
}

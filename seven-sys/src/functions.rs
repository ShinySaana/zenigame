use core::ffi::c_void;

pub const fn DMA_LEN(length: u16, flags: u16) -> u16 {
    if (flags & (crate::bindings::DMAControl::DMA_32BIT as u16)) != 0 {
        length >> 2
    } else {
        length >> 1
    }
}

pub const fn DMA_BUILD(
    src: *const c_void,
    dst: *mut c_void,
    len: u16,
    flags: u16
) -> crate::bindings::DMA {
    crate::bindings::DMA {
        src,
        dst,
        len: DMA_LEN(len, flags),
        cnt: flags
    }
}


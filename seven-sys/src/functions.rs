use core::ffi::c_void;
use crate::bitfields::{COLOR_B, COLOR_G, COLOR_R};

// seven/hw/dma.h
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

// seven/hw/video/bg_bitmap.h
pub const fn M4PX(l: u8, h: u8) -> u16 {
    ((l as u16) << 8u16) | (h as u16)
}


// seven/hw/video/color.h
pub const fn RGB5(r: u8, g: u8, b: u8) -> u16 {
    (COLOR_R(r as u32) | COLOR_G(g as u32) | COLOR_B(b as u32)) as u16
}

pub const fn RGB8(r: u8, g: u8, b: u8) -> u16 {
    RGB5(r >> 3, g >> 3, b >> 3)
}


// seven/hw/video/memory.h
pub const fn GFX_BASE_ADDR(n: u32) -> *mut c_void {
    (crate::addresses::MEM_VRAM_ADDR + (n << 14) as usize) as *mut c_void 
}

pub const fn MAP_BASE_ADDR(n: u32) -> *mut c_void {
    (crate::addresses::MEM_VRAM_ADDR + (n << 11) as usize) as *mut c_void 
}

// seven/hw/video/window.h
pub const fn WINDOW_DIM(l: u8, h: u8) -> u16 {
    (((l as u16) & 255) << 8) | (h & 255) as u16
}

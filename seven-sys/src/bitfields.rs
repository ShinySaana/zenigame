use core::ffi::c_void;

macro_rules! BIT {
    ($n:expr) => {
        1 << $n
    };
}


macro_rules! BITS {
    ($n:expr) => {
        BIT!($n) - 1
    };
}

macro_rules! BITFIELD {
    ($name:ident, $value:expr) => {
        (($value) & BITS!(concat_idents!(BF_, $name, _LENGTH))) << concat_idents!(BF_, $name, _OFFSET)
    };
}

macro_rules! BITFIELD_FN {
    ($name:ident) => {
        pub const fn $name(n: u32) -> usize {
            use $crate::bindings::*;

            (BITFIELD!($name, n)) as usize
        }
    };
}

// seven/hw/cpu.h
BITFIELD_FN!(PSR_MODE);
BITFIELD_FN!(PSR_CONTROL_BITS);
BITFIELD_FN!(PSR_FLAGS);

// seven/hw/dma.h
BITFIELD_FN!(DMA_DST);
BITFIELD_FN!(DMA_SRC);
BITFIELD_FN!(DMA_START);


// seven/hw/video.h
BITFIELD_FN!(VIDEO_MODE);
BITFIELD_FN!(LCD_TARGET_VCOUNT);
BITFIELD_FN!(BG_PRIORITY);
BITFIELD_FN!(BG_GFX_BASE);
BITFIELD_FN!(BG_MAP_BASE);
BITFIELD_FN!(BG_SIZE);
BITFIELD_FN!(BG_AFFINE_SIZE);

// seven/hw/video/memory.h
pub const fn GFX_BASE_ADDR(n: u32) -> *mut c_void {
    (crate::addresses::MEM_VRAM_ADDR + (n << 14) as usize) as *mut c_void 
}

pub const fn MAP_BASE_ADDR(n: u32) -> *mut c_void {
    (crate::addresses::MEM_VRAM_ADDR + (n << 11) as usize) as *mut c_void 
}
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
        (($value) & BITS!(concat_idents!(BF_, $name, _LEN))) << concat_idents!(BF_, $name, _OFF)
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

// seven/cpu.h
BITFIELD_FN!(PSR_MODE);
BITFIELD_FN!(PSR_CONTROL_BITS);
BITFIELD_FN!(PSR_FLAGS);

// seven/dma.h
BITFIELD_FN!(DMA_DST);
BITFIELD_FN!(DMA_SRC);
BITFIELD_FN!(DMA_START);

// seven/serial.h
BITFIELD_FN!(SIO_MODE);
BITFIELD_FN!(R_MODE);

// seven/timer.h
BITFIELD_FN!(TIMER_FREQ);

// seven/video.h
// BITFIELD_FN!(VIDEO_MODE); VIDEO_MODE disappeared?
BITFIELD_FN!(LCD_TARGET_VCOUNT);
BITFIELD_FN!(BG_PRIORITY);
BITFIELD_FN!(BG_GFX_BASE);
BITFIELD_FN!(BG_MAP_BASE);
BITFIELD_FN!(BG_SIZE);
BITFIELD_FN!(BG_AFFINE_SIZE);

// seven/waitstate.h
BITFIELD_FN!(WAIT_SRAM);
BITFIELD_FN!(WAIT_ROM_N);
BITFIELD_FN!(WAIT_ROM_S);

// seven/serial/multiplayer.h
BITFIELD_FN!(MULTI_BAUD);
BITFIELD_FN!(MULTI_CLIENT_ID);

// seven/serial/spi.h
BITFIELD_FN!(SPI_CLOCK);

// seven/serial/uart.h
BITFIELD_FN!(UART_BAUD);

// seven/video/gb_tiled.h
BITFIELD_FN!(TILE_NUMBER);
BITFIELD_FN!(TILE_PALETTE);

// seven/video/blend.h
BITFIELD_FN!(BLEND_MODE);

// seven/video/color.h
BITFIELD_FN!(COLOR_R);
BITFIELD_FN!(COLOR_G);
BITFIELD_FN!(COLOR_B);

// seven/video/mosaic.h
BITFIELD_FN!(MOSAIC_BG_H);
BITFIELD_FN!(MOSAIC_BG_V);
BITFIELD_FN!(MOSAIC_OBJ_H);
BITFIELD_FN!(MOSAIC_OBJ_V);

// seven/video/object.h
BITFIELD_FN!(OBJ_Y_POS);
BITFIELD_FN!(OBJ_MODE);
BITFIELD_FN!(OBJ_TYPE);
BITFIELD_FN!(OBJ_SHAPE);

BITFIELD_FN!(OBJ_X_POS);
BITFIELD_FN!(OBJ_AFFINE_MATRIX);
BITFIELD_FN!(OBJ_SIZE);

BITFIELD_FN!(OBJ_TILE_NUMBER);
BITFIELD_FN!(OBJ_PRIORITY);
BITFIELD_FN!(OBJ_PALETTE_NUMBER);

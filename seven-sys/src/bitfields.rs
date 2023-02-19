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

// seven/hw/serial.h
BITFIELD_FN!(SIO_MODE);
BITFIELD_FN!(R_MODE);

// seven/hw/timer.h
BITFIELD_FN!(TIMER_FREQ);

// seven/hw/video.h
BITFIELD_FN!(VIDEO_MODE);
BITFIELD_FN!(LCD_TARGET_VCOUNT);
BITFIELD_FN!(BG_PRIORITY);
BITFIELD_FN!(BG_GFX_BASE);
BITFIELD_FN!(BG_MAP_BASE);
BITFIELD_FN!(BG_SIZE);
BITFIELD_FN!(BG_AFFINE_SIZE);

// seven/hw/waitstate.h
BITFIELD_FN!(WAIT_SRAM);
BITFIELD_FN!(WAIT_ROM_N);
BITFIELD_FN!(WAIT_ROM_S);

// seven/hw/serial/multiplayer.h
BITFIELD_FN!(MULTI_BAUD);
BITFIELD_FN!(MULTI_CLIENT_ID);

// seven/hw/serial/spi.h
BITFIELD_FN!(SPI_CLOCK);

// seven/hw/serial/uart.h
BITFIELD_FN!(UART_BAUD);

// seven/hw/video/gb_tiled.h
BITFIELD_FN!(TILE_NUMBER);
BITFIELD_FN!(TILE_PALETTE);

// seven/hw/video/blend.h
BITFIELD_FN!(BLEND_MODE);

// seven/hw/video/color.h
BITFIELD_FN!(COLOR_R);
BITFIELD_FN!(COLOR_G);
BITFIELD_FN!(COLOR_B);

// seven/hw/video/mosaic.h
BITFIELD_FN!(MOSAIC_BG_H);
BITFIELD_FN!(MOSAIC_BG_V);
BITFIELD_FN!(MOSAIC_OBJ_H);
BITFIELD_FN!(MOSAIC_OBJ_V);

// seven/hw/video/object.h
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

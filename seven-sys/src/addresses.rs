use core::{marker::PhantomData, ffi::c_void};

pub struct SevenVolAddr<T: Copy> (usize, PhantomData<T>);

impl <T: Copy> SevenVolAddr<T> {
    pub fn address(&self) -> *mut T {
        return self.0 as *mut T
    }

    pub unsafe fn read(&self) -> T {
        self.address().read_volatile()
    }

    pub unsafe fn write(&self, val: T) {
        self.address().write_volatile(val)
    }
}

pub struct SevenVolArray<T: Copy, const Size: usize> (usize, PhantomData<T>);

impl <T: Copy, const Size: usize> SevenVolArray <T, Size> {
    pub fn address(&self) -> *mut [T; Size] {
        return self.0 as *mut [T; Size]
    }

    pub unsafe fn read(&self) -> [T; Size] {
        self.address().read_volatile()
    }

    pub unsafe fn get(&self, idx: usize) -> T {
        self.read()[idx]
    }

    pub unsafe fn write(&self, val: [T; Size]) {
        self.address().write_volatile(val)
    }

    pub unsafe fn set(&self, idx: usize, val: T) {
        let mut temp = self.read();
        temp[idx] = val;
        self.write(temp);
    }
}

pub struct SevenMemAddr<T: Copy> (usize, PhantomData<T>);

impl <T: Copy> SevenMemAddr<T> {
    pub fn address(&self) -> *mut T {
        return self.0 as *mut T
    }

    pub unsafe fn read(&self) -> T {
        self.address().read()
    }

    pub unsafe fn write(&self, val: T) {
        self.address().write(val)
    }
}

pub struct SevenMemArray<T: Copy, const Size: usize> (usize, PhantomData<T>);

impl <T: Copy, const Size: usize> SevenMemArray <T, Size> {
    pub fn address(&self) -> *mut [T; Size] {
        return self.0 as *mut [T; Size]
    }

    pub unsafe fn read(&self) -> [T; Size] {
        self.address().read()
    }

    pub unsafe fn get(&self, idx: usize) -> T {
        self.read()[idx]
    }

    pub unsafe fn write(&self, val: [T; Size]) {
        self.address().write(val)
    }

    pub unsafe fn set(&self, idx: usize, val: T) {
        let mut temp = self.read();
        temp[idx] = val;
        self.write(temp);
    }


}


// seven/hw/memory.h
pub const MEM_BIOS_ADDR:    usize           = 0x00000000;
pub const MEM_BIOS:         *const c_void   = MEM_BIOS_ADDR as *const c_void;
pub const MEM_BIOS_SIZE:    usize           = 0x4000;

pub const MEM_EWRAW_ADDR:   usize           = 0x02000000;
pub const MEM_EWRAM:        *mut c_void     = MEM_EWRAW_ADDR as *mut c_void;
pub const MEM_EWRAM_SIZE:   usize           = 0x40000;

pub const MEM_IWRAM_ADDR:   usize           = 0x03000000;
pub const MEM_IWRAM:        *mut c_void     = MEM_IWRAM_ADDR as *mut c_void;
pub const MEM_IWRAM_SIZE:   usize           = 0x8000;

pub const REG_BASE_ADRR:    usize           = 0x04000000;
pub const REG_BASE:         *mut c_void     = REG_BASE_ADRR as *mut c_void;
pub const MEM_IO:           *mut c_void     = REG_BASE_ADRR as *mut c_void;
pub const MEM_IO_SIZE:      usize           = 0x20C;

pub const MEM_PALETTE_ADRR: usize           = 0x05000000;
pub const MEM_PALETTE:      *mut c_void     = MEM_PALETTE_ADRR as *mut c_void;
pub const MEM_PALETTE_SIZE: usize           = 0x400;

pub const MEM_VRAM_ADDR:    usize           = 0x06000000;
pub const MEM_VRAM:         *mut c_void     = MEM_VRAM_ADDR as *mut c_void;
pub const MEM_VRAM_SIZE:    usize           = 0x18000;

pub const MEM_OAM_ADDR:     usize           = 0x07000000;
pub const MEM_OAM:          *mut c_void     = MEM_OAM_ADDR as *mut c_void;
pub const MEM_OAM_SIZE:     usize           = 0x400;

pub const MEM_ROM_ADDR:     usize           = 0x08000000;
pub const MEM_ROM:          *const c_void   = MEM_ROM_ADDR as *const c_void;
pub const MEM_ROM_SIZE:     usize           = 0x02000000;

pub const MEM_SRAM_ADDR:    usize           = 0x0E000000;
pub const MEM_SRAM:         *mut c_void     = MEM_SRAM_ADDR as *mut c_void;
pub const MEM_SRAM_SIZE:    usize           = 0x8000;


macro_rules! voladdr {
    ($($name:ident, $address:expr, $type:ty)*) => {
        $(
            pub const $name: SevenVolAddr<$type> = SevenVolAddr($address, PhantomData);
        )*
    };
}

voladdr! {
    // seven/hw/cartridge.h
    REG_GPIODAT,    0x080000C4, u16
    REG_GPIODIR,    0x080000C6, u16
    REG_GPIOCNT,    0x080000C8, u16
    
    // seven/bw/dma.h
    REG_DMA0SRC,    0x040000B0, *const c_void
    REG_DMA0DST,    0x040000B4, *mut c_void
    REG_DMA0LEN,    0x040000B8, u16
    REG_DMA0CNT,    0x040000BA, u16

    REG_DMA1SRC,    0x040000BC, *const c_void
    REG_DMA1DST,    0x040000C0, *mut c_void
    REG_DMA1LEN,    0x040000C4, u16
    REG_DMA1CNT,    0x040000C6, u16

    REG_DMA2SRC,    0x040000C8, *const c_void
    REG_DMA2DST,    0x040000CC, *mut c_void
    REG_DMA2LEN,    0x040000D0, u16
    REG_DMA2CNT,    0x040000D2, u16

    REG_DMA3SRC,    0x040000D4, *const c_void
    REG_DMA3DST,    0x040000D8, *mut c_void
    REG_DMA3LEN,    0x040000DC, u16
    REG_DMA3CNT,    0x040000DE, u16

    // seven/hw/input.h
    REG_KEYINPUT,   0x04000130, /* const */ u16
    REG_KEYCNT,     0x04000132, u16

    // seven/hw/irq.h
    REG_IE,         0x04000200, u16
    REG_IF,         0x04000202, u16
    REG_IME,        0x04000208, u16

    // seven/hw/serial.h
    REG_SIOCNT,     0x04000128, u16
    REG_RCNT,       0x04000134, u16

    REG_SIODATA8,   0x0400012A, u8
    REG_SIODATA32,  0x04000120, u32

    REG_MULTISEND,  0x0400012A, u16
    REG_MULTIRECV0, 0x04000120, u16
    REG_MULTIRECV1, 0x04000122, u16
    REG_MULTIRECV2, 0x04000124, u16
    REG_MULTIRECV3, 0x04000126, u16

    REG_JOYCNT,     0x04000140, u16
    REG_JOYRECV,    0x04000150, u32
    REG_JOYSEND,    0x04000154, u32
    REG_JOYSTAT,    0x04000158, u16

    // seven/hw/timer.h
    REG_TM0VAL,     0x04000100, u16
    REG_TM0CNT,     0x04000102, u16

    REG_TM1VAL,     0x04000104, u16
    REG_TM1CNT,     0x04000106, u16

    REG_TM2VAL,     0x04000108, u16
    REG_TM2CNT,     0x0400010A, u16

    REG_TM3VAL,     0x0400010C, u16
    REG_TM3CNT,     0x0400010E, u16

    // seven/hw/video.h
    REG_DISPCNT,    0x04000000, u16
    REG_DISPSTAT,   0x04000004, u16
    REG_VCOUNT,     0x04000006, /* const */ u16
    REG_BG0CNT,     0x04000008, u16
    REG_BG1CNT,     0x0400000A, u16
    REG_BG2CNT,     0x0400000C, u16
    REG_BG3CNT,     0x0400000E, u16

    // seven/hw/waitstate
    REG_WAITCNT,    0x04000204, u16

    // seven/hw/video/bg_scroll.h
    REG_BG0HOFS,    0x04000010, u16
    REG_BG0VOFS,    0x04000012, u16
    REG_BG1HOFS,    0x04000014, u16
    REG_BG1VOFS,    0x04000016, u16
    REG_BG2HOFS,    0x04000018, u16
    REG_BG2VOFS,    0x0400001A, u16
    REG_BG3HOFS,    0x0400001C, u16
    REG_BG3VOFS,    0x0400001E, u16

    // seven/hw/video/bg_transform
    REG_BG2PA,      0x04000020, u16
    REG_BG2PB,      0x04000022, u16
    REG_BG2PC,      0x04000024, u16
    REG_BG2PD,      0x04000026, u16

    REG_BG2MAT,     0x04000020, crate::bindings::Matrix

    REG_BG2X,       0x04000028, u32
    REG_BG2Y,       0x0400002C, u32

    REG_BG3PA,      0x04000030, u16
    REG_BG3PB,      0x04000032, u16
    REG_BG3PC,      0x04000034, u16
    REG_BG3PD,      0x04000036, u16

    REG_BG3MAT,     0x04000030, crate::bindings::Matrix

    REG_BG3X,       0x04000038, u32
    REG_BG3Y,       0x0400003C, u32

    // seven/hw/video/blend.h
    REG_BLDCNT,     0x04000050, u16
    REG_BLDALPHA,   0x04000052, u16
    REG_BLDY,       0x04000054, u16

    // seven/hw/video/mosaic.h
    REG_MOSAIC,     0x0400004C, u16

    // seven/hw/video/window.h
    REG_WIN0H,      0x04000040, u16
    REG_WIN1H,      0x04000042, u16
    REG_WIN0V,      0x04000044, u16
    REG_WIN1V,      0x04000046, u16

    REG_WIN0IN,     0x04000048, u8
    REG_WIN1IN,     0x04000049, u8
    REG_WIN0OUT,    0x0400004A, u8
    REG_WIN1OUT,    0x0400004B, u8
}

macro_rules! volarray {
    ($($name:ident, $address:expr, $type:ty, $size:literal)*) => {
        $(
            pub const $name: SevenVolArray<$type, $size> = SevenVolArray($address, PhantomData);
        )*
    };
}

volarray! {
    // seven/hw/video.b
    REG_BGCNT,  0x0400_0008, u16, 4
}

macro_rules! memaddr {
    ($($name:ident, $address:expr, $type:ty)*) => {
        $(
            pub const $name: SevenMemAddr<$type> = SevenMemAddr($address, PhantomData);
        )*
    };
}

memaddr! {
    // seven/hw/irq.h
    MEM_ISR,        0x03FFFFFC,     *mut crate::bindings::IsrFn

    // seven/hw/video/bg_bitmap.h
    MODE3_FRAME,    MEM_VRAM_ADDR,  crate::bindings::Mode3Frame

    MODE4_FRAME_0,  MEM_VRAM_ADDR, crate::bindings::Mode4Frame
    MODE4_FRAME_1,  MEM_VRAM_ADDR + 0xA000, crate::bindings::Mode4Frame

    MODE5_FRAME_0,  MEM_VRAM_ADDR, crate::bindings::Mode5Frame
    MODE5_FRAME_1,  MEM_VRAM_ADDR + 0xA000, crate::bindings::Mode5Frame

    // seven/hw/video/memory.h
    BG_PALETTE,     MEM_PALETTE_ADRR, crate::bindings::Palette
    OBJ_PALETTE,    MEM_PALETTE_ADRR + 512, crate::bindings::Palette
}

macro_rules! memarray {
    ($($name:ident, $address:expr, $type:ty, $size:literal)*) => {
        $(
            pub const $name: SevenMemArray<$type, $size> = SevenMemArray($address, PhantomData);
        )*
    };
}

memarray! {
    // seven/hw/video/memory.h
    BG_PALETTE_BANK, MEM_PALETTE_ADRR, crate::bindings::PaletteBank, 16
    OBJ_PALETTE_BANK, MEM_PALETTE_ADRR + 512, crate::bindings::PaletteBank, 16

    OAM_OBJS,       MEM_OAM_ADDR, crate::bindings::Object, 128
}


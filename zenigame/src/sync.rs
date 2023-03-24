use core::arch::asm;

pub trait Swappable<T> {
    unsafe fn get(&self) -> T;
    unsafe fn swap(&mut self, i: T) -> T;
}

#[repr(transparent)]
pub struct SyncedU8 (u8);

impl SyncedU8 {
    pub const fn new(i: u8) -> Self {
        Self(i)
    }
}

#[repr(transparent)]
pub struct SyncedU32 (u32);

impl SyncedU32 {
    pub const fn new(i: u32) -> Self {
        Self(i)
    }
}

pub struct SyncedPtr<T> (*mut T);

impl <T> SyncedPtr<T> {
    pub const fn new(i: *mut T) -> Self {
        Self(i)
    }
}


impl Swappable<u8> for SyncedU8 {

    unsafe fn get(&self) -> u8 {
        self.0
    }
 
    #[instruction_set(arm::a32)]
    #[inline]
    unsafe fn swap(&mut self, i: u8) -> u8 {
        let out;
        let dst = self;
        asm!("swpb {2}, {1}, [{0}]", in(reg) dst, in(reg) i, lateout(reg) out);
        out
    }
}

impl Swappable<u32> for SyncedU32 {
    unsafe fn get(&self) -> u32 {
        self.0
    } 

    #[instruction_set(arm::a32)]
    #[inline]
    unsafe fn swap(&mut self, i: u32) -> u32 {
        let out;
        let dst = self;
        asm!("swp {2}, {1}, [{0}]", in(reg) dst, in(reg) i, lateout(reg) out);
        out
    }
}


impl <T> Swappable<*mut T> for SyncedPtr<T> {
    unsafe fn get(&self) -> *mut T {
        return self.0
    }

    #[instruction_set(arm::a32)]
    #[inline]
    unsafe fn swap(&mut self, i: *mut T) -> *mut T {
        let out;
        let dst = self;
        asm!("swp {2}, {1}, [{0}]", in(reg) dst, in(reg) i, lateout(reg) out);
        out 
    }
}

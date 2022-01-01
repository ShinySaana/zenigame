#![no_std]

pub mod irq;
pub mod log;

#[inline]
pub unsafe fn read_reg16(address: u32) -> u16 {
    let p = address as *mut u16;
    p.read_volatile()
}

#[inline]
pub unsafe fn write_reg16(address: u32, value: u16) {
    let p = address as *mut u16;
    p.write_volatile(value);
}



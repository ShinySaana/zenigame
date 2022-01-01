use seven_sys::irq::*;

use crate::{write_reg16, read_reg16};

pub struct CriticalSection (());

impl CriticalSection {
    #[must_use]
    pub fn enter() -> Self {
        unsafe {
            irqCriticalSectionEnter();
        }

        CriticalSection(())
    }

    pub fn exit(self) {}
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        unsafe {
            irqCriticalSectionExit()
        }
    }
}

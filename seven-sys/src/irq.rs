extern "C" {
    pub fn irqCriticalSectionEnter();
    pub fn irqCriticalSectionExit();
    pub fn svcVBlankIntrWait();
    pub fn irqInitDefault();
    pub fn irqEnable(intr_flags: u16) -> u16;

}

pub const IRQ_VBLANK: u16 = 0b_0000_0000_0000_0001;
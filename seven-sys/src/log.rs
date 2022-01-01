#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum LogLevel {
    Off = 0,
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogInterface {
    None = 0,
    MGBA,
    NoCash,
    VBA,
    Custom = u8::MAX 
}

pub type LogCustomOutputFunction = extern fn(u8, *const u8) -> ();

extern "C" {
    pub fn logInit() -> u8;
    pub fn logInitInterface(interface: u8);
    pub fn logInitCustom(f: LogCustomOutputFunction);
    pub fn logGetInterface() -> u8;
    pub fn logGetInterfaceName() -> *const u8;
    pub fn logSetMaxLevel(level: u8);
    pub fn logGetMaxLevel() -> u8;
    pub fn logOutput(level: u8, message: *const u8);
}

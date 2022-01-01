use core::{fmt::{Arguments, write}};

use arrayvec::ArrayString;
use seven_sys::log::{logInit, LogInterface, logInitInterface, LogLevel, logOutput, logSetMaxLevel};

static mut ACTUALLY_LOG: bool = true;
static mut LOGGER_INITIALIZED: bool = false;

#[inline]
pub fn enable_logging(enable: bool) {
    unsafe {
        ACTUALLY_LOG = enable;
    }
}

pub fn set_max_level(level: LogLevel) {
    unsafe {
        logSetMaxLevel(level as u8);
    }
}

pub fn initialize_logger() {
    unsafe {
        let initialized = logInit();

        if initialized != LogInterface::None as u8 {
            LOGGER_INITIALIZED = true;
        }
    }

    set_max_level(LogLevel::Trace);
}

pub fn initialize_logger_interface(interface: LogInterface) {
    // Check against LogInterface::Custom is done in libseven.
    unsafe {
        logInitInterface(interface as u8);

        if interface != LogInterface::None {
            LOGGER_INITIALIZED = true;
        }
    }

    set_max_level(LogLevel::Trace);
}

pub unsafe fn raw_log(level: LogLevel, message: *const u8) {
    let do_log = ACTUALLY_LOG && LOGGER_INITIALIZED;
    if do_log {
        logOutput(level as u8, message);
    }
}

pub fn log(level: LogLevel, message: Arguments) {
    let mut buffer = ArrayString::<256>::new();

    match write(&mut buffer, message) {
        Ok(_) => {
            // logOutputLength(level as u8, buffer.as_bytes().as_ptr(), buffer.len());
        },
        Err(_) => unsafe {
            raw_log(LogLevel::Warn, "An error occurred while logging with log::log. Was the message too long?\0".as_ptr())
        },
    };
}

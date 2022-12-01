#![no_std]
#[macro_use]
extern crate bitflags;

pub mod uart;

pub struct Device {
    uart0: uart::Controller,
    uart1: uart::Controller,
}

impl Device {
    /// Creates a global device instance.
    /// # Safety
    /// If you attempt to create and use more than one device at a time, the devices may clash in unpredictable ways.
    pub unsafe fn new() -> Self {
        Self {
            uart0: uart::Controller::new(0x00FF000000),
            uart1: uart::Controller::new(0x00FF010000),
        }
    }

    pub fn uart0(&mut self) -> &mut uart::Controller {
        &mut self.uart0
    }

    pub fn uart1(&mut self) -> &mut uart::Controller {
        &mut self.uart1
    }
}

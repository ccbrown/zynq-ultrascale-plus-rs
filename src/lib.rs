#![no_std]
#[macro_use]
extern crate tock_registers;

pub mod iou_scntrs;
pub mod uart;

pub struct Device {
    iou_scntrs: iou_scntrs::Module,
    uart0: uart::Module,
    uart1: uart::Module,
}

impl Device {
    /// Creates a global device instance.
    ///
    /// # Safety
    ///
    /// Things will blow up spectacularly if this is called on anything other than a Zynq UltraScale+ device.
    ///
    /// If you create more than one device, it's up to you to ensure that they don't simultaneously access the same resources.
    pub unsafe fn new() -> Self {
        Self {
            iou_scntrs: iou_scntrs::Module::new(0x00FF260000),
            uart0: uart::Module::new(0x00FF000000),
            uart1: uart::Module::new(0x00FF010000),
        }
    }

    pub fn iou_scntrs(&mut self) -> &mut iou_scntrs::Module {
        &mut self.iou_scntrs
    }

    pub fn uart0(&mut self) -> &mut uart::Module {
        &mut self.uart0
    }

    pub fn uart1(&mut self) -> &mut uart::Module {
        &mut self.uart1
    }
}

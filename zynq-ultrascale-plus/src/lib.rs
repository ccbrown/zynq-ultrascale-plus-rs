#![no_std]

use zynq_ultrascale_plus_modules as mods;

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
            iou_scntrs: iou_scntrs::Module::new(&mut *mods::iou_scntrs::IOU_SCNTRS),
            uart0: uart::Module::new(&mut *mods::uart::UART0),
            uart1: uart::Module::new(&mut *mods::uart::UART1),
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

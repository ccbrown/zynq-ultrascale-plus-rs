use tock_registers::interfaces::{Readable, Writeable};
use zynq_ultrascale_plus_modules::iou_scntrs::*;

pub struct Module {
    registers: &'static mut Registers,
}

impl Module {
    /// Creates a new system timestamp generator.
    pub fn new(registers: &'static mut Registers) -> Self {
        Self { registers }
    }

    /// Provides raw access to the registers.
    ///
    /// # Safety
    /// Refer to the module's reference material to understand what is and isn't safe.
    pub unsafe fn registers(&mut self) -> &mut Registers {
        self.registers
    }

    pub fn is_enabled(&self) -> bool {
        self.registers.counter_control.is_set(CounterControlR::EN)
    }

    pub fn enable(&mut self) {
        self.registers
            .counter_control
            .write(CounterControlW::EN::SET)
    }

    pub fn disable(&mut self) {
        self.registers
            .counter_control
            .write(CounterControlW::EN::CLEAR)
    }

    pub fn frequency(&self) -> u32 {
        self.registers.base_frequency_id.get()
    }
}

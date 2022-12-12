use tock_registers::interfaces::{Readable, Writeable};
use zynq_ultrascale_plus_modules::crf_apb::*;

pub struct Controller {
    registers: &'static Registers,
}

impl Controller {
    /// Initiatizes and returns the CRF_APB controller.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn crf_apb() -> Self {
        Self::new(&mut *CRF_APB)
    }

    /// Creates a new controller.
    pub fn new(registers: &'static Registers) -> Self {
        Self { registers }
    }

    /// Provides raw access to the registers.
    ///
    /// # Safety
    /// Refer to the module's reference material to understand what is and isn't safe.
    pub unsafe fn registers(&mut self) -> &Registers {
        self.registers
    }

    pub fn core0(&mut self) -> Core {
        Core {
            controller: self,
            index: 0,
        }
    }

    pub fn core1(&mut self) -> Core {
        Core {
            controller: self,
            index: 1,
        }
    }

    pub fn core2(&mut self) -> Core {
        Core {
            controller: self,
            index: 2,
        }
    }

    pub fn core3(&mut self) -> Core {
        Core {
            controller: self,
            index: 3,
        }
    }
}

pub struct Core<'a> {
    controller: &'a mut Controller,
    index: u32,
}

impl<'a> Core<'a> {
    pub fn is_reset(&self) -> bool {
        self.controller.registers.rst_fpd_apu.get() & (0x401 << self.index) != 0
    }

    /// Sends the core into the reset state.
    ///
    /// # Safety
    /// Any code the core is running will be terminated without warning and without cleanup. It is
    /// very easy to leave things in an inconsistent state if you're not careful about when you do
    /// this.
    pub unsafe fn reset(&mut self) {
        let v = self.controller.registers.rst_fpd_apu.get() | (1 << self.index);
        self.controller.registers.rst_fpd_apu.set(v);
    }

    pub fn release_reset(&mut self) {
        let v = self.controller.registers.rst_fpd_apu.get() & !(1 << self.index);
        self.controller.registers.rst_fpd_apu.set(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller() {
        let mut controller = unsafe { Controller::crf_apb() };
        assert!(!controller.core0().is_reset());
        assert!(controller.core1().is_reset());
        assert!(controller.core2().is_reset());
        assert!(controller.core3().is_reset());
    }
}

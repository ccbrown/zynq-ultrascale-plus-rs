use crate::interrupt_vector_table;
use aarch64_cpu::registers::{self, SCR_EL3};
use tock_registers::interfaces::Writeable;
use zynq_ultrascale_plus_modules::gic400::*;

/// Routes interrupts to EL3.
pub fn take_to_el3() {
    SCR_EL3.write(SCR_EL3::IRQ::Taken + SCR_EL3::FIQ::Taken + SCR_EL3::EA::Taken);
}

pub struct ControllerInstaller {
    registers: &'static mut Registers,
    interrupt_handler: Option<&'static (dyn Fn(u16) + Send)>,
}

impl ControllerInstaller {
    /// Configures a function to be called whenever an IRQ or FIQ is triggered.
    ///
    /// The interrupt is called with the id of the triggered interrupt.
    pub fn set_interrupt_handler(&mut self, f: &'static (dyn Fn(u16) + Send)) -> &mut Self {
        self.interrupt_handler = Some(f);
        self
    }

    /// Configures the vector table and enables the distributor.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn install(self) -> Controller {
        self.registers.gicd_ctlr.set(0);

        if let Some(f) = self.interrupt_handler {
            interrupt_vector_table::IRQ_HANDLER = Some(f);
            interrupt_vector_table::FIQ_HANDLER = Some(f);
        }

        self.registers.gicd_isenabler0.set(0xffffffff);
        self.registers.gicd_isenabler1.set(0xffffffff);
        self.registers.gicd_isenabler2.set(0xffffffff);
        self.registers.gicd_isenabler3.set(0xffffffff);
        self.registers.gicd_isenabler4.set(0xffffffff);
        self.registers.gicd_isenabler5.set(0xffffffff);

        let table = unsafe { interrupt_vector_table::apu_vector_table.as_ptr() as u64 };
        registers::VBAR_EL3.set(table);
        registers::VBAR_EL2.set(table);
        registers::VBAR_EL1.set(table);

        self.registers.gicd_ctlr.set(3);

        Controller {
            registers: self.registers,
        }
    }
}

pub struct Controller {
    registers: &'static mut Registers,
}

#[derive(Clone, Copy, Debug)]
pub enum SgiTarget {
    ThisCpu,
    OtherCpus,
}

impl Controller {
    /// Returns an installer for the APU interrupt controller.
    pub fn apu() -> ControllerInstaller {
        ControllerInstaller {
            registers: unsafe { &mut *ACPU_GIC },
            interrupt_handler: None,
        }
    }

    /// Enables CPU signaling on this processing element.
    pub fn enable_cpu_signaling(&mut self) {
        self.registers.gicc_ctlr.set(11);
        self.registers.gicc_pmr.set(0xff);
    }

    /// Creates a software-generated interrupt.
    pub fn generate_interrupt(&mut self, id: u16, target: SgiTarget) {
        let id = (id & 0xf) as u32;
        self.registers.gicd_sgir.set(match target {
            SgiTarget::OtherCpus => (1 << 24) | id,
            SgiTarget::ThisCpu => (2 << 24) | id,
        });
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        self.registers.gicd_icenabler0.set(0xffffffff);
        self.registers.gicd_icenabler1.set(0xffffffff);
        self.registers.gicd_icenabler2.set(0xffffffff);
        self.registers.gicd_icenabler3.set(0xffffffff);
        self.registers.gicd_icenabler4.set(0xffffffff);
        self.registers.gicd_icenabler5.set(0xffffffff);
        self.registers.gicd_ctlr.set(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::{
        sync::atomic::{AtomicBool, Ordering},
        time::Duration,
    };

    const SGI_ID: u16 = 5;
    static SGI_TRIGGERED: AtomicBool = AtomicBool::new(false);

    fn sgi_handler(id: u16) {
        assert_eq!(id, SGI_ID);
        SGI_TRIGGERED.store(true, Ordering::SeqCst);
    }

    #[test]
    fn test_controller() {
        let mut controller = Controller::apu();
        controller.set_interrupt_handler(&sgi_handler);
        let mut controller = unsafe { controller.install() };
        controller.enable_cpu_signaling();
        registers::DAIF.set(0);
        take_to_el3();
        controller.generate_interrupt(SGI_ID, SgiTarget::ThisCpu);

        let mut triggered = false;
        while !triggered {
            aarch64_std::thread::sleep(Duration::from_millis(20));
            triggered = SGI_TRIGGERED.load(Ordering::SeqCst);
        }
        assert!(triggered);
    }
}

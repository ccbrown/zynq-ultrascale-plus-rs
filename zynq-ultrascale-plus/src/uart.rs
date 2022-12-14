use tock_registers::interfaces::{Readable, Writeable};
use zynq_ultrascale_plus_modules::uart::*;

pub struct Controller {
    registers: &'static Registers,
}

impl Controller {
    /// Initiatizes and returns the UART 0 controller.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn uart0() -> Self {
        Self::new(&mut *UART0)
    }

    /// Initiatizes and returns the UART 1 controller.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn uart1() -> Self {
        Self::new(&mut *UART1)
    }

    /// Creates a new UART controller.
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

    fn is_transmit_full(&self) -> bool {
        self.registers.channel_sts.is_set(ChannelSts::TFUL)
    }

    pub fn send_byte(&mut self, b: u8) {
        while self.is_transmit_full() {}
        self.registers
            .tx_rx_fifo
            .write(TxRxFifoW::FIFO.val(b as u32));
        while self.is_transmit_full() {}
    }

    pub fn send_bytes<T: AsRef<[u8]>>(&mut self, buf: T) {
        for b in buf.as_ref() {
            self.send_byte(*b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller() {
        let uart = unsafe { Controller::uart0() };
        assert!(!uart.is_transmit_full());
    }
}

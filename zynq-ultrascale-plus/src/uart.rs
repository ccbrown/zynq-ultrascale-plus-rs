use tock_registers::interfaces::{Readable, Writeable};
use zynq_ultrascale_plus_modules::uart::*;

pub struct Module {
    registers: &'static mut Registers,
}

impl Module {
    /// Creates a new UART controller.
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

    fn is_transmit_full(&self) -> bool {
        self.registers.channel_sts.is_set(ChannelSts::TFUL)
    }

    pub fn send_byte(&mut self, b: u8) {
        while self.is_transmit_full() {}
        self.registers
            .tx_rx_fifo
            .write(TxRxFifoW::FIFO.val(b as u32))
    }

    pub fn send_bytes<T: AsRef<[u8]>>(&mut self, buf: T) {
        for b in buf.as_ref() {
            self.send_byte(*b);
        }
    }
}

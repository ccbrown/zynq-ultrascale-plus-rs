use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::ReadWrite,
};

pub struct Module {
    registers: &'static mut Registers,
}

register_structs! {
    pub Registers {
        // TODO: implement remaining registers
        (0x0000000000 => _not_implemented),
        (0x000000002C => channel_status: ReadWrite<u32, ChannelStatus::Register>),
        (0x0000000030 => tx_rx_fifo: ReadWrite<u32, TxRxFifo::Register>),
        (0x0000000034 => _not_implemented_2),
        (0x000000004C => @END),
    }
}

register_bitfields! [
    u32,
    ChannelStatus [
        TNFUL 9,
        TTRIG 8,
        FDELT 7,
        TACTIVE 6,
        RACTIVE 5,
        TFUL 4,
        TEMPTY 3,
        RFUL 2,
        REMPTY 1,
        RTRIG 0,
    ],
    TxRxFifo [
        FIFO OFFSET(0) NUMBITS(8) []
    ]
];

impl Module {
    /// Creates a new UART controller.
    ///
    /// # Safety
    /// `base_address` must be the base address for a UART controller that is not being used elsewhere.
    pub unsafe fn new(base_address: usize) -> Self {
        Self {
            registers: &mut *(base_address as *mut Registers),
        }
    }

    /// Provides raw access to the registers.
    ///
    /// # Safety
    /// Refer to the module's reference material to understand what is and isn't safe.
    pub unsafe fn registers(&mut self) -> &mut Registers {
        self.registers
    }

    fn is_transmit_full(&self) -> bool {
        self.registers.channel_status.is_set(ChannelStatus::TFUL)
    }

    pub fn send_byte(&mut self, b: u8) {
        while self.is_transmit_full() {}
        self.registers
            .tx_rx_fifo
            .write(TxRxFifo::FIFO.val(b as u32))
    }

    pub fn send_bytes<T: AsRef<[u8]>>(&mut self, buf: T) {
        for b in buf.as_ref() {
            self.send_byte(*b);
        }
    }
}

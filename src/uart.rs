use core::ptr::{read_volatile, write_volatile};

pub struct Controller {
    base_address: usize,
}

bitflags! {
    struct ChannelStatus: u32 {
        const RTRIG = (1 << 0);
        const REMPTY = (1 << 1);
        const RFUL = (1 << 2);
        const TEMPTY = (1 << 3);
        const TFUL = (1 << 4);
        const RACTIVE = (1 << 10);
        const TACTIVE = (1 << 11);
        const FDELT = (1 << 12);
        const TTRIG = (1 << 13);
        const TNFUL = (1 << 14);
    }
}

impl Controller {
    /// Creates a new UART controller.
    ///
    /// # Safety
    /// `base_address` must be the base address for a UART controller that is not being used elsewhere.
    pub unsafe fn new(base_address: usize) -> Self {
        Self { base_address }
    }

    fn read_channel_status(&self) -> ChannelStatus {
        unsafe {
            ChannelStatus::from_bits_unchecked(read_volatile(
                &mut *((self.base_address + 0x000000002C) as *mut u32),
            ))
        }
    }

    fn is_transmit_full(&self) -> bool {
        self.read_channel_status().contains(ChannelStatus::TFUL)
    }

    pub fn send_byte(&mut self, b: u8) {
        while self.is_transmit_full() {}
        unsafe {
            write_volatile(
                &mut *((self.base_address + 0x0000000030) as *mut u32),
                b as u32,
            );
        }
    }

    pub fn send_bytes<T: AsRef<[u8]>>(&mut self, buf: T) {
        for b in buf.as_ref() {
            self.send_byte(*b);
        }
    }
}

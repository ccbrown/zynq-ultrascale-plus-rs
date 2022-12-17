use tock_registers::interfaces::{Readable, Writeable};
use zynq_ultrascale_plus_modules::rtc::*;

pub struct Clock {
    registers: &'static Registers,
}

impl Clock {
    /// Initiatizes and returns the real time clock.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple clocks at once.
    pub unsafe fn rtc() -> Self {
        Self::new(&mut *RTC)
    }

    /// Creates a new real time clock.
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

    /// Sets the clock's time in seconds.
    pub fn set_time(&mut self, t: u32) {
        self.registers.set_time_write.set(t);
        self.registers.rtc_int_status.set(0xff);
    }

    /// Returns the clock's time in seconds.
    pub fn time(&self) -> u32 {
        if self.registers.rtc_int_status.is_set(RtcIntStatus::SECONDS) {
            self.registers.set_time_read.get()
        } else {
            self.registers.current_time.get()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock() {
        let mut clock = unsafe { Clock::rtc() };
        clock.set_time(12345);
        let t = clock.time();
        assert!(t == 12345 || t == 12346);
    }
}

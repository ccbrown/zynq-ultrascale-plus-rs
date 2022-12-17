use tock_registers::interfaces::{Readable, Writeable};
use zynq_ultrascale_plus_modules::rtc::*;

pub struct Clock {
    registers: &'static Registers,
}

#[derive(Clone, Debug)]
pub struct Calibration {
    /// This value represents the number of cycles at which the RTC gets one clock cycle ahead of
    /// the real time due to the cumulative fractional inaccuracy of the Oscillator.
    pub fractional_compensation: Option<u8>,

    /// This value multiplied by the period of the oscillator should equal to 1 second.
    pub max_tick: u16,
}

impl Calibration {
    pub const fn with_frequency(freq: u16) -> Self {
        Self {
            max_tick: freq - 1,
            fractional_compensation: None,
        }
    }

    fn to_register(&self) -> u32 {
        self.max_tick as u32
            | match self.fractional_compensation {
                Some(c) => ((0x10 | (c & 0xf)) as u32) << 16,
                None => 0,
            }
    }
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

    /// Calibrates the clock.
    pub fn calibrate(&mut self, calibration: Calibration) {
        self.registers.calib_write.set(calibration.to_register());
    }

    /// Sets the clock's time in seconds.
    pub fn set_time(&mut self, t: u32) {
        // Rewrite the calibration so the next tick happens exactly one second from now.
        self.registers
            .calib_write
            .set(self.registers.calib_read.get());
        // Write t + 1 because this value won't be persisted until exactly 1 second later.
        self.registers.set_time_write.set(t + 1);
        self.registers.rtc_int_status.set(0xff);
    }

    /// Returns the clock's time in seconds.
    pub fn time(&self) -> u32 {
        if self.registers.rtc_int_status.is_set(RtcIntStatus::SECONDS) {
            self.registers.current_time.get()
        } else {
            self.registers.set_time_read.get() - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;

    // The qemu RTC implementation doesn't match the hardware at all. It appears to function by
    // just offsetting the host clock, so we can't really test it in qemu.
    //
    // See: https://github.com/Xilinx/qemu/blob/959afcfa82b7e281b7232402ae7ffb6741cccabf/hw/rtc/xlnx-zynqmp-rtc.c#L182
    #[test(hw_only)]
    fn test_clock() {
        let mut clock = unsafe { Clock::rtc() };
        clock.calibrate(Calibration::with_frequency(0x8000));
        clock.set_time(12345);
        let t = clock.time();
        assert!(t == 12345);
        aarch64_std::thread::sleep(Duration::from_millis(1200));
        assert_eq!(clock.time(), t + 1);
        aarch64_std::thread::sleep(Duration::from_millis(1000));
        assert_eq!(clock.time(), t + 2);
    }
}

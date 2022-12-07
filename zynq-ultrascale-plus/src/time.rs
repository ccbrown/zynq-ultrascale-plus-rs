use aarch64_cpu::registers;
use core::time::Duration;
use tock_registers::interfaces::Readable;

#[derive(Clone, Copy, Debug, Hash)]
pub struct Instant {
    ticks: u64,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            ticks: registers::CNTVCT_EL0.get(),
        }
    }

    pub fn duration_since(&self, earlier: Self) -> Duration {
        let freq = registers::CNTFRQ_EL0.get();
        if freq == 0 {
            panic!("the physical timer is not configured");
        }
        let ticks = self.ticks - earlier.ticks;
        Duration::from_secs_f64(ticks as f64 / freq as f64)
    }

    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thread::sleep;

    #[test_case]
    fn test_instant() {
        let start = Instant::now();
        const SLEEP_DURATION: Duration = Duration::from_millis(500);
        sleep(SLEEP_DURATION);
        assert!(start.elapsed() >= SLEEP_DURATION);
    }
}

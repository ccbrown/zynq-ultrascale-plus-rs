use aarch64_cpu::registers;
use core::time::Duration;
use tock_registers::interfaces::Readable;

pub fn sleep(d: Duration) {
    let freq = registers::CNTFRQ_EL0.get();
    if freq == 0 {
        panic!("the physical timer is not configured");
    }
    let start = registers::CNTVCT_EL0.get();
    let end = start + (d.as_secs_f64() * freq as f64) as u64;
    // TODO: set a timer instead
    loop {
        if registers::CNTVCT_EL0.get() >= end {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_sleep() {
        sleep(Duration::from_secs(1));
    }
}

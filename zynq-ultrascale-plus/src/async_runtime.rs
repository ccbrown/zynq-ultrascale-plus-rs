use core::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

static SEV_WAKER_V_TABLE: RawWakerVTable = RawWakerVTable::new(
    sev_waker_clone,
    sev_waker_wake,
    sev_waker_wake_by_ref,
    sev_waker_drop,
);

unsafe fn sev_waker_clone(_: *const ()) -> RawWaker {
    RawWaker::new(ptr::null(), &SEV_WAKER_V_TABLE)
}

unsafe fn sev_waker_wake(_: *const ()) {
    aarch64_cpu::asm::sev();
}

unsafe fn sev_waker_wake_by_ref(data: *const ()) {
    sev_waker_wake(data)
}

unsafe fn sev_waker_drop(_: *const ()) {}

/// Runs a future to completion on the current thread.
pub fn block_on<F: Future>(mut future: F) -> F::Output {
    let mut future = unsafe { Pin::new_unchecked(&mut future) };
    let waker = unsafe { Waker::from_raw(RawWaker::new(ptr::null(), &SEV_WAKER_V_TABLE)) };
    let mut ctx = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut ctx) {
            Poll::Ready(ret) => return ret,
            Poll::Pending => aarch64_cpu::asm::wfe(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_on() {
        assert_eq!(block_on(async { 123 }), 123);
    }
}

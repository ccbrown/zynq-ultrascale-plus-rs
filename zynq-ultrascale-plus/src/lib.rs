#![no_std]
#![cfg_attr(test, no_main)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::tests::runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, feature(default_alloc_error_handler))]
// asm_sym is stabilized, but not yet released. this should be very temporary
#![feature(asm_sym)]

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(test, macro_use)]
extern crate alloc;

#[cfg(test)]
#[cfg_attr(test, macro_use)]
extern crate test_macro;

#[cfg(test)]
pub(crate) static PRINT_LOCK: aarch64_std::sync::Mutex<()> = aarch64_std::sync::Mutex::new(());

#[cfg(not(test))]
#[allow(unused_macros)]
macro_rules! debug {
    ($($args:expr),*) => {};
}

#[cfg(test)]
macro_rules! debug {
    ($($args:expr),*) => {
        {
            let _lock = $crate::PRINT_LOCK.lock().unwrap();
            let mut uart = unsafe { $crate::uart::Controller::uart0() };
            uart.send_bytes(format!($($args),*));
            uart.send_byte('\n' as _);
        }
    }
}

pub mod apu;
pub mod async_runtime;
pub mod crf_apb;
pub mod gem;
pub mod interrupt;
mod interrupt_vector_table;
pub mod net;
pub mod uart;

pub use zynq_ultrascale_plus_modules as modules;

#[cfg(test)]
mod tests {
    use super::*;
    use core::arch::global_asm;
    use qemu_exit::QEMUExit;

    global_asm!(
        r#"
.global _Reset
_Reset:
    ldr x30, =stack_top
    mov sp, x30
    bl main
    b .
"#
    );

    use linked_list_allocator::LockedHeap;

    #[global_allocator]
    static ALLOCATOR: LockedHeap = LockedHeap::empty();

    extern "C" {
        static mut _heap_start: u32;
        static mut _heap_end: u32;
    }

    fn init_heap() {
        unsafe {
            let heap_start_ptr = &mut _heap_start as *mut u32;
            let heap_end_ptr = &mut _heap_end as *mut u32;
            ALLOCATOR.lock().init(
                heap_start_ptr as *mut u8,
                heap_end_ptr as usize - heap_start_ptr as usize,
            );
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn main() {
        init_heap();
        crate::test_main();
        qemu_exit::AArch64::new().exit(0);
    }

    pub fn runner(tests: &[&dyn Fn()]) {
        debug!("running {} tests", tests.len());
        for test in tests {
            test();
        }
    }

    #[panic_handler]
    fn panic_handler(info: &core::panic::PanicInfo) -> ! {
        let mut uart = unsafe { uart::Controller::uart0() };
        uart.send_bytes(format!("panic: {}\n", info));
        qemu_exit::AArch64::new().exit(1);
    }
}

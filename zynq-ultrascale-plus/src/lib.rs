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
            let mut uart = unsafe { if $crate::tests::is_qemu() { $crate::uart::Controller::uart0() } else { $crate::uart::Controller::uart1() } };
            uart.send_bytes(format!($($args),*));
            uart.send_bytes("\r\n");
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
        .global _vector_table
        _vector_table:
            b _Reset
            nop
            nop
            nop
            nop
            eret
        .global _Reset
        _Reset:
            ldr x30, =__stack_top__
            mov sp, x30
            bl main
            b .
        "#
    );

    use linked_list_allocator::LockedHeap;

    #[global_allocator]
    static ALLOCATOR: LockedHeap = LockedHeap::empty();

    extern "C" {
        static mut __bss_start__: u32;
        static mut __bss_end__: u32;
        static mut __heap_start__: u32;
        static mut __heap_end__: u32;
    }

    unsafe fn init_bss() {
        let bss_start_ptr = &mut __bss_start__ as *mut u32;
        let bss_end_ptr = &mut __bss_end__ as *mut u32;
        let mut dest = bss_start_ptr;
        while dest != bss_end_ptr {
            *dest = 0;
            dest = dest.offset(1);
        }
    }

    unsafe fn init_heap() {
        let heap_start_ptr = &mut __heap_start__ as *mut u32;
        let heap_end_ptr = &mut __heap_end__ as *mut u32;
        ALLOCATOR.lock().init(
            heap_start_ptr as *mut u8,
            heap_end_ptr as usize - heap_start_ptr as usize,
        );
    }

    #[no_mangle]
    pub unsafe extern "C" fn main() {
        init_bss();
        init_heap();
        crate::test_main();
        exit(0);
    }

    /// Returns true if we're running in QEMU.
    pub fn is_qemu() -> bool {
        use tock_registers::interfaces::Readable;
        let csu = unsafe { &mut *modules::csu::CSU };
        let jtag_sec = csu.jtag_sec.get();
        (jtag_sec & 0x1ff) != 0x1ff
    }

    pub fn exit(status: u32) -> ! {
        if is_qemu() {
            qemu_exit::AArch64::new().exit(status);
        } else {
            loop {}
        }
    }

    pub fn runner(tests: &[&dyn Fn()]) {
        is_qemu();
        debug!("running {} tests", tests.len());
        for test in tests {
            test();
        }
        debug!("all tests passed");
    }

    #[panic_handler]
    fn panic_handler(info: &core::panic::PanicInfo) -> ! {
        let mut uart = unsafe {
            if is_qemu() {
                uart::Controller::uart0()
            } else {
                uart::Controller::uart1()
            }
        };
        uart.send_bytes(format!("panic: {}\r\n", info));
        exit(1);
    }
}

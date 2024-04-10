#![no_std]
#![cfg_attr(test, no_main)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::tests::runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

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
            use aarch64_cpu::registers::{DBGDTRTX_EL0, MDCCSR_EL0};
            use tock_registers::interfaces::{Readable, Writeable};

            let _lock = $crate::PRINT_LOCK.lock().unwrap();
            #[allow(unused_unsafe)]
            let mut uart = unsafe { if $crate::tests::is_qemu() { None } else { Some($crate::uart::Controller::uart1()) } };
            let s = format!($($args),*);
            for c in s.as_bytes().iter().chain(&['\n' as u8]) {
                if let Some(uart) = &mut uart {
                    uart.send_byte(*c);
                }
                while MDCCSR_EL0.is_set(MDCCSR_EL0::TXfull) {}
                DBGDTRTX_EL0.set(*c as u64);
            }
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
pub mod rtc;
pub mod sdio;
pub mod uart;

pub use zynq_ultrascale_plus_modules as modules;

#[cfg(test)]
mod tests {
    use super::*;
    use core::{arch::global_asm, ptr::addr_of_mut};
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
        let bss_start_ptr = addr_of_mut!(__bss_start__);
        let bss_end_ptr = addr_of_mut!(__bss_end__);
        let mut dest = bss_start_ptr;
        while dest != bss_end_ptr {
            *dest = 0;
            dest = dest.offset(1);
        }
    }

    unsafe fn init_heap() {
        let heap_start_ptr = addr_of_mut!(__heap_start__);
        let heap_end_ptr = addr_of_mut!(__heap_end__);
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
        uart.send_bytes(format!("panic: {}\n", info));
        exit(1);
    }
}

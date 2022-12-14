use crate::crf_apb;
use core::arch::global_asm;
use tock_registers::{interfaces::Writeable, registers::InMemoryRegister};
use zynq_ultrascale_plus_modules::apu::*;

pub struct APU {
    registers: &'static Registers,
}

impl APU {
    /// Initiatizes and returns the APU.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple instances at once.
    pub unsafe fn apu() -> Self {
        Self::new(&mut *APU)
    }

    /// Creates a new APU instance.
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

    pub fn core0(&mut self) -> Core {
        Core {
            apu: self,
            index: 0,
        }
    }

    pub fn core1(&mut self) -> Core {
        Core {
            apu: self,
            index: 1,
        }
    }

    pub fn core2(&mut self) -> Core {
        Core {
            apu: self,
            index: 2,
        }
    }

    pub fn core3(&mut self) -> Core {
        Core {
            apu: self,
            index: 3,
        }
    }
}

pub struct Core<'a> {
    apu: &'a mut APU,
    index: u32,
}

#[repr(C)]
struct CoreEntryData {
    entry_point: InMemoryRegister<u64>,
    user_data: InMemoryRegister<u64>,
    stack_top: InMemoryRegister<u64>,
    // XXX: if the size of this struct changes, _ApuCoreReset must also be updated
}

impl CoreEntryData {
    const fn new() -> Self {
        Self {
            entry_point: InMemoryRegister::new(0),
            user_data: InMemoryRegister::new(0),
            stack_top: InMemoryRegister::new(0),
        }
    }
}

unsafe impl Sync for CoreEntryData {}

static mut CORE_ENTRY_DATA: [CoreEntryData; 4] = [
    CoreEntryData::new(),
    CoreEntryData::new(),
    CoreEntryData::new(),
    CoreEntryData::new(),
];

global_asm!(
    r#"
.global _ApuCoreReset
_ApuCoreReset:
    mrs x30, MPIDR_EL1
    and x30, x30, #3
    mov x29, #24
    mul x30, x30, x29
    ldr x29, ={}
    add x30, x30, x29
    ldr x10, [x30, #0]
    ldr x0, [x30, #8]
    ldr x12, [x30, #16]
    mov sp, x12
    blr x10
    1:
    wfe
    b 1b
"#,
    sym CORE_ENTRY_DATA,
);

extern "C" {
    static _ApuCoreReset: u64;
}

#[cfg(feature = "alloc")]
struct SevOnDrop;

#[cfg(feature = "alloc")]
impl Drop for SevOnDrop {
    fn drop(&mut self) {
        aarch64_cpu::asm::sev();
    }
}

#[cfg(feature = "alloc")]
pub struct JoinHandle<T> {
    result: alloc::sync::Arc<core::sync::atomic::AtomicU64>,
    index: u32,
    _data: core::marker::PhantomData<T>,
}

#[cfg(feature = "alloc")]
impl<T> JoinHandle<T> {
    /// Waits for the function to return, resets the core it ran on, and provides the return value.
    pub fn join(self, reset_controller: &mut crf_apb::Controller) -> T {
        use alloc::{boxed::Box, sync::Arc};
        use core::sync::atomic::Ordering;

        let mut reset_controller_core = match self.index {
            0 => reset_controller.core0(),
            1 => reset_controller.core1(),
            2 => reset_controller.core2(),
            3 => reset_controller.core3(),
            _ => unreachable!(),
        };

        loop {
            let v = self.result.load(Ordering::SeqCst);
            if v == 0 {
                if Arc::strong_count(&self.result) == 1 {
                    unsafe { reset_controller_core.reset() };
                    panic!("core panicked");
                } else {
                    aarch64_cpu::asm::wfe();
                }
            } else {
                let mut v: Box<Option<T>> = unsafe { Box::from_raw(v as *mut _) };
                unsafe { reset_controller_core.reset() };
                return v.take().unwrap();
            }
        }
    }
}

impl<'a> Core<'a> {
    /// Launches a function on a reset core.
    ///
    /// # Panics
    /// This function panics if the core is not reset.
    ///
    /// # Safety
    /// It is up to the user to ensure that the stack is big enough. No stack overflow detection is
    /// provided.
    pub unsafe fn start(
        &mut self,
        reset_controller: &mut crf_apb::Controller,
        f: extern "C" fn(u64) -> !,
        user_data: u64,
        stack: &'static mut [u8],
    ) {
        let mut reset_controller_core = match self.index {
            0 => reset_controller.core0(),
            1 => reset_controller.core1(),
            2 => reset_controller.core2(),
            3 => reset_controller.core3(),
            _ => unreachable!(),
        };
        if !reset_controller_core.is_reset() {
            panic!("attempted to start already started core");
        }
        unsafe {
            let data: &mut CoreEntryData = &mut CORE_ENTRY_DATA[self.index as usize];
            data.entry_point.set(f as _);
            data.user_data.set(user_data);
            data.stack_top
                .set(stack.as_mut_ptr().offset(stack.len() as _) as _);
        }
        let addr = &_ApuCoreReset as *const _ as u64;
        match self.index {
            0 => {
                self.apu.registers.rvbaraddr0l.set(addr as u32);
                self.apu.registers.rvbaraddr0h.set((addr >> 32) as u32);
            }
            1 => {
                self.apu.registers.rvbaraddr1l.set(addr as u32);
                self.apu.registers.rvbaraddr1h.set((addr >> 32) as u32);
            }
            2 => {
                self.apu.registers.rvbaraddr2l.set(addr as u32);
                self.apu.registers.rvbaraddr2h.set((addr >> 32) as u32);
            }
            3 => {
                self.apu.registers.rvbaraddr3l.set(addr as u32);
                self.apu.registers.rvbaraddr3h.set((addr >> 32) as u32);
            }
            _ => unreachable!(),
        }
        reset_controller_core.release_reset();
    }

    /// Launches a function on a reset core and waits for it to return.
    ///
    /// # Panics
    /// This function panics if the core is not reset.
    ///
    /// # Safety
    /// It is up to the user to ensure that the stack is big enough. No stack overflow detection is
    /// provided.
    #[cfg(feature = "alloc")]
    pub unsafe fn spawn<T: Send, F: FnOnce() -> T + Send + 'static>(
        &mut self,
        reset_controller: &mut crf_apb::Controller,
        f: F,
        stack: &mut [u8],
    ) -> JoinHandle<T> {
        use alloc::{boxed::Box, sync::Arc};
        use core::{
            marker::PhantomData,
            mem,
            sync::atomic::{AtomicU64, Ordering},
        };

        let result = Arc::new(AtomicU64::new(0));

        type FnBox = Box<dyn FnOnce() + Send>;

        extern "C" fn entry(user_data: u64) -> ! {
            let f: Box<FnBox> = unsafe { Box::from_raw(user_data as _) };
            f();
            loop {
                aarch64_cpu::asm::wfe();
            }
        }

        let f: Box<FnBox> = Box::new(Box::new({
            let result = result.clone();
            move || {
                let _sev = SevOnDrop;
                let r: Box<Option<T>> = Box::new(Some(f()));
                result.store(Box::into_raw(r) as u64, Ordering::SeqCst);
            }
        }));

        let stack = mem::transmute::<&mut [u8], &'static mut [u8]>(stack);
        self.start(reset_controller, entry, Box::into_raw(f) as _, stack);

        {
            JoinHandle {
                result,
                index: self.index,
                _data: PhantomData,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test(qemu_only)]
    fn test_apu_core_block_on() {
        let mut apu = unsafe { APU::apu() };
        let mut reset_controller = unsafe { crf_apb::Controller::crf_apb() };
        let mut stack = vec![0u8; 8 * 1024];

        assert_eq!(123, unsafe {
            apu.core1()
                .spawn(&mut reset_controller, || 123, &mut stack)
                .join(&mut reset_controller)
        });

        assert_eq!(456, unsafe {
            apu.core1()
                .spawn(&mut reset_controller, || 456, &mut stack)
                .join(&mut reset_controller)
        });
    }
}

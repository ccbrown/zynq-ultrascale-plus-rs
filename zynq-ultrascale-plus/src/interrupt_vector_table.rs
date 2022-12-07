use core::arch::global_asm;
use tock_registers::interfaces::{Readable, Writeable};
use zynq_ultrascale_plus_modules::gic400::*;

extern "C" {
    pub static apu_vector_table: [u32; 512];
}

pub static mut IRQ_HANDLER: Option<&(dyn Fn(u16) + Send)> = None;
pub static mut FIQ_HANDLER: Option<&(dyn Fn(u16) + Send)> = None;

global_asm!(
    r#"
.global apu_vector_table
.balign 0x800
apu_vector_table:
    b synchronous_exception_entry
.balign 0x80
    b irq_entry
.balign 0x80
    b fiq_entry
.balign 0x80
    b serror_entry
.balign 0x80
    b synchronous_exception_entry
.balign 0x80
    b irq_entry
.balign 0x80
    b fiq_entry
.balign 0x80
    b serror_entry
.balign 0x80
    b synchronous_exception_entry
.balign 0x80
    b irq_entry
.balign 0x80
    b fiq_entry
.balign 0x80
    b serror_entry
.balign 0x80
    b synchronous_exception_entry
.balign 0x80
    b irq_entry
.balign 0x80
    b fiq_entry
.balign 0x80
    b serror_entry

synchronous_exception_entry:
    sub sp, sp, #176
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x29, [sp, #144]
    stp x30, xzr, [sp, #160]

    mov x0, sp
    bl apu_synchronous_exception_handler

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x29, [sp, #144]
    ldp x30, xzr, [sp, #160]
    add sp, sp, #176
    eret

irq_entry:
    sub sp, sp, #176
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x29, [sp, #144]
    stp x30, xzr, [sp, #160]

    mov x0, sp
    bl apu_irq_handler

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x29, [sp, #144]
    ldp x30, xzr, [sp, #160]
    add sp, sp, #176
    eret

fiq_entry:
    sub sp, sp, #176
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x29, [sp, #144]
    stp x30, xzr, [sp, #160]

    mov x0, sp
    bl apu_fiq_handler

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x29, [sp, #144]
    ldp x30, xzr, [sp, #160]
    add sp, sp, #176
    eret

serror_entry:
    sub sp, sp, #176
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x29, [sp, #144]
    stp x30, xzr, [sp, #160]

    mov x0, sp
    bl apu_serror_handler

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x29, [sp, #144]
    ldp x30, xzr, [sp, #160]
    add sp, sp, #176
    eret
"#
);

#[repr(C)]
struct State {
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
    x9: u64,
    x10: u64,
    x11: u64,
    x12: u64,
    x13: u64,
    x14: u64,
    x15: u64,
    x16: u64,
    x17: u64,
    x18: u64,
    fp: u64,
    lr: u64,
    xzr: u64,
}

#[no_mangle]
unsafe extern "C" fn apu_synchronous_exception_handler(_state: *mut State) {
    panic!("exception encountered!");
}

#[no_mangle]
unsafe extern "C" fn apu_irq_handler(_state: *mut State) {
    let interrupt = (*ACPU_GIC).gicc_iar.get();
    let id = (interrupt & 0x3f) as u16;
    if let Some(f) = IRQ_HANDLER {
        f(id);
    }
    (*ACPU_GIC).gicc_eoir.set(interrupt);
}

#[no_mangle]
unsafe extern "C" fn apu_fiq_handler(_state: *mut State) {
    let interrupt = (*ACPU_GIC).gicc_iar.get();
    let id = (interrupt & 0x3f) as u16;
    if let Some(f) = FIQ_HANDLER {
        f(id);
    }
    (*ACPU_GIC).gicc_eoir.set(interrupt);
}

#[no_mangle]
unsafe extern "C" fn apu_serror_handler(_state: *mut State) {
    panic!("serror encountered!");
}

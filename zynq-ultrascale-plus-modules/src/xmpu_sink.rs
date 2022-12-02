// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// XMPU Protection Unit Sink, XMPU Sink Configuration
pub static mut FPD_XMPU_SINK: *mut Registers = 0xfd4f0000 as *mut Registers;
register_structs! {
    pub Registers {
        (0x00000000 => _padding0),
        /// Access Violation Error Status.
        (0x0000ff00 => pub err_status: ReadWrite<u32, ErrStatus::Register>),
        (0x0000ff04 => _padding65284),
        /// Interrupt Status and Clear.
        (0x0000ff10 => pub isr: ReadWrite<u8, Isr::Register>),
        (0x0000ff11 => _padding65297),
        /// Interrupt Mask.
        (0x0000ff14 => pub imr: ReadOnly<u8, Imr::Register>),
        (0x0000ff15 => _padding65301),
        /// Interrupt Enable.
        (0x0000ff18 => pub ier: WriteOnly<u8, Ier::Register>),
        (0x0000ff19 => _padding65305),
        /// Interrupt Disable.
        (0x0000ff1c => pub idr: WriteOnly<u8, Idr::Register>),
        (0x0000ff1d => _padding65309),
        /// Error Signal Control. APB slave error signal.
        (0x0000ffec => pub err_ctrl: ReadWrite<u32, ErrCtrl::Register>),
        (0x0000fff0 => @END),
    }
}
register_bitfields! [
    u32,
    pub ErrStatus [
        RDWR OFFSET(31) NUMBITS(1) [],
        ADDR OFFSET(0) NUMBITS(12) [],
    ]
];
register_bitfields! [
    u8,
    pub Isr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Imr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Ier [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Idr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ErrCtrl [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        PSLVERR OFFSET(0) NUMBITS(1) [],
    ]
];

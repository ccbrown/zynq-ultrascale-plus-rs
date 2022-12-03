// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// XPPU Protection Unit Sink, XPPU Sink Controller
pub static mut LPD_XPPU_SINK: *mut Registers = 0xff9c0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 65280],
    /// Access Violation Error Status.
    pub err_status: ReadWrite<u32, ErrStatus::Register>,
    _padding65284: [u8; 12],
    /// Interrupt Status and Clear.
    pub isr: ReadWrite<u8, Isr::Register>,
    _padding65297: [u8; 3],
    /// Interrupt Mask.
    pub imr: ReadOnly<u8, Imr::Register>,
    _padding65301: [u8; 3],
    /// Interrupt Enable.
    pub ier: WriteOnly<u8, Ier::Register>,
    _padding65305: [u8; 3],
    /// Interrupt Disable.
    pub idr: WriteOnly<u8, Idr::Register>,
    _padding65309: [u8; 207],
    /// Error Control. APB error signal SLVERR.
    pub err_ctrl: ReadWrite<u32, ErrCtrl::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub ErrStatus [
        RDWR OFFSET(31) NUMBITS(1) [],
        ADDR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Isr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Imr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Ier [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Idr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrCtrl [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        PSLVERR OFFSET(0) NUMBITS(1) [],
    ]
];

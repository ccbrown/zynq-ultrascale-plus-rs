// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Secure Low-power Domain SLCR, Low-power Domain System-level Control Registers - Secure
pub static mut LPD_SLCR_SECURE: *mut Registers = 0xff4b0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 4],
    /// General control register for the LP SLCR
    pub ctrl: ReadWrite<u8, Ctrl::Register>,
    _padding5: [u8; 3],
    /// Interrupt Status Register
    pub isr: ReadWrite<u8, Isr::Register>,
    _padding9: [u8; 3],
    /// Interrupt Mask Register
    pub imr: ReadOnly<u8, Imr::Register>,
    _padding13: [u8; 3],
    /// Interrupt Enable Register
    pub ier: WriteOnly<u8, Ier::Register>,
    _padding17: [u8; 3],
    /// Interrupt Disable Register
    pub idr: WriteOnly<u8, Idr::Register>,
    _padding21: [u8; 3],
    /// Interrupt Trigger Register
    pub itr: WriteOnly<u8, Itr::Register>,
    _padding25: [u8; 7],
    /// RPU TrustZone settings
    pub slcr_rpu: ReadWrite<u8, SlcrRpu::Register>,
    _padding33: [u8; 3],
    /// LPD DMA TrustZone setting.
    pub slcr_adma: ReadWrite<u8, SlcrAdma::Register>,
    _padding37: [u8; 11],
    /// Safety endpoint connectivity check.
    pub safety_chk: ReadWrite<u32>,
    /// USB TrustZone settings.
    pub slcr_usb: ReadWrite<u8, SlcrUsb::Register>,
}
tock_registers::register_bitfields! [
    u8,
    pub Ctrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
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
    u8,
    pub Itr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub SlcrRpu [
        TZ_R5_1 OFFSET(1) NUMBITS(1) [],
        TZ_R5_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub SlcrAdma [
        TZ OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub SlcrUsb [
        TZ_USB3_1 OFFSET(1) NUMBITS(1) [],
        TZ_USB3_0 OFFSET(0) NUMBITS(1) [],
    ]
];

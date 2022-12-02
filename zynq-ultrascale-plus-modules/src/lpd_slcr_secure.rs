// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Secure Low-power Domain SLCR, Low-power Domain System-level Control Registers - Secure
pub static mut LPD_SLCR_SECURE: *mut Registers = 0xff4b0000 as *mut Registers;
register_structs! {
    pub Registers {
        (0x00000000 => _padding0),
        /// General control register for the LP SLCR
        (0x00000004 => pub ctrl: ReadWrite<u8, Ctrl::Register>),
        (0x00000005 => _padding5),
        /// Interrupt Status Register
        (0x00000008 => pub isr: ReadWrite<u8, Isr::Register>),
        (0x00000009 => _padding9),
        /// Interrupt Mask Register
        (0x0000000c => pub imr: ReadOnly<u8, Imr::Register>),
        (0x0000000d => _padding13),
        /// Interrupt Enable Register
        (0x00000010 => pub ier: WriteOnly<u8, Ier::Register>),
        (0x00000011 => _padding17),
        /// Interrupt Disable Register
        (0x00000014 => pub idr: WriteOnly<u8, Idr::Register>),
        (0x00000015 => _padding21),
        /// Interrupt Trigger Register
        (0x00000018 => pub itr: WriteOnly<u8, Itr::Register>),
        (0x00000019 => _padding25),
        /// RPU TrustZone settings
        (0x00000020 => pub slcr_rpu: ReadWrite<u8, SlcrRpu::Register>),
        (0x00000021 => _padding33),
        /// LPD DMA TrustZone setting.
        (0x00000024 => pub slcr_adma: ReadWrite<u8, SlcrAdma::Register>),
        (0x00000025 => _padding37),
        /// Safety endpoint connectivity check.
        (0x00000030 => pub safety_chk: ReadWrite<u32>),
        /// USB TrustZone settings.
        (0x00000034 => pub slcr_usb: ReadWrite<u8, SlcrUsb::Register>),
        (0x00000035 => @END),
    }
}
register_bitfields! [
    u8,
    pub Ctrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
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
    u8,
    pub Itr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub SlcrRpu [
        TZ_R5_1 OFFSET(1) NUMBITS(1) [],
        TZ_R5_0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub SlcrAdma [
        TZ OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub SlcrUsb [
        TZ_USB3_1 OFFSET(1) NUMBITS(1) [],
        TZ_USB3_0 OFFSET(0) NUMBITS(1) [],
    ]
];

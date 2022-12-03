// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Battery Backed RAM, Battery-backed RAM control
pub static mut BBRAM: *mut Registers = 0xffcd0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// BBRAM Status
    pub bbram_status: ReadOnly<u32, BbramStatus::Register>,
    /// BBRAM Control
    pub bbram_ctrl: WriteOnly<u32, BbramCtrl::Register>,
    /// BBRAM Program Mode
    pub pgm_mode: ReadWrite<u32>,
    /// BBRAM AES Key Integrity Check
    pub bbram_aes_crc: WriteOnly<u32>,
    /// BBRAM Data 0
    pub bbram_0: WriteOnly<u32>,
    /// BBRAM Data 1
    pub bbram_1: WriteOnly<u32>,
    /// BBRAM Data 2
    pub bbram_2: WriteOnly<u32>,
    /// BBRAM Data 3
    pub bbram_3: WriteOnly<u32>,
    /// BBRAM Data 4
    pub bbram_4: WriteOnly<u32>,
    /// BBRAM Data 5
    pub bbram_5: WriteOnly<u32>,
    /// BBRAM Data 6
    pub bbram_6: WriteOnly<u32>,
    /// BBRAM Data 7
    pub bbram_7: WriteOnly<u32>,
    /// BBRAM Data 8
    pub bbram_8: WriteOnly<u32>,
    /// BBRAM Slave Error Control
    pub bbram_slverr: ReadWrite<u32, BbramSlverr::Register>,
    /// BBRAM Interrupt Status
    pub bbram_isr: ReadWrite<u32, BbramIsr::Register>,
    /// BBRAM Interrupt Mask
    pub bbram_imr: ReadOnly<u32, BbramImr::Register>,
    /// BBRAM Interrupt Enable
    pub bbram_ier: WriteOnly<u32, BbramIer::Register>,
    /// BBRAM Interrupt Disable
    pub bbram_idr: WriteOnly<u32, BbramIdr::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub BbramStatus [
        AES_CRC_PASS OFFSET(9) NUMBITS(1) [],
        AES_CRC_DONE OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(3) [],
        BBRAM_ZEROIZED OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(3) [],
        PGM_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BbramCtrl [
        ZEROIZE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BbramSlverr [
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BbramIsr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BbramImr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BbramIer [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BbramIdr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];

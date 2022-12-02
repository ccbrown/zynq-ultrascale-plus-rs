// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Battery Backed RAM, Battery-backed RAM control
pub static mut BBRAM: *mut Registers = 0xffcd0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// BBRAM Status
        (0x00000000 => pub bbram_status: ReadOnly<u32, BbramStatus::Register>),
        /// BBRAM Control
        (0x00000004 => pub bbram_ctrl: WriteOnly<u32, BbramCtrl::Register>),
        /// BBRAM Program Mode
        (0x00000008 => pub pgm_mode: ReadWrite<u32>),
        /// BBRAM AES Key Integrity Check
        (0x0000000c => pub bbram_aes_crc: WriteOnly<u32>),
        /// BBRAM Data 0
        (0x00000010 => pub bbram_0: WriteOnly<u32>),
        /// BBRAM Data 1
        (0x00000014 => pub bbram_1: WriteOnly<u32>),
        /// BBRAM Data 2
        (0x00000018 => pub bbram_2: WriteOnly<u32>),
        /// BBRAM Data 3
        (0x0000001c => pub bbram_3: WriteOnly<u32>),
        /// BBRAM Data 4
        (0x00000020 => pub bbram_4: WriteOnly<u32>),
        /// BBRAM Data 5
        (0x00000024 => pub bbram_5: WriteOnly<u32>),
        /// BBRAM Data 6
        (0x00000028 => pub bbram_6: WriteOnly<u32>),
        /// BBRAM Data 7
        (0x0000002c => pub bbram_7: WriteOnly<u32>),
        /// BBRAM Data 8
        (0x00000030 => pub bbram_8: WriteOnly<u32>),
        /// BBRAM Slave Error Control
        (0x00000034 => pub bbram_slverr: ReadWrite<u32, BbramSlverr::Register>),
        /// BBRAM Interrupt Status
        (0x00000038 => pub bbram_isr: ReadWrite<u32, BbramIsr::Register>),
        /// BBRAM Interrupt Mask
        (0x0000003c => pub bbram_imr: ReadOnly<u32, BbramImr::Register>),
        /// BBRAM Interrupt Enable
        (0x00000040 => pub bbram_ier: WriteOnly<u32, BbramIer::Register>),
        /// BBRAM Interrupt Disable
        (0x00000044 => pub bbram_idr: WriteOnly<u32, BbramIdr::Register>),
        (0x00000048 => @END),
    }
}
register_bitfields! [
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
register_bitfields! [
    u32,
    pub BbramCtrl [
        ZEROIZE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub BbramSlverr [
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub BbramIsr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub BbramImr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub BbramIer [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub BbramIdr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];

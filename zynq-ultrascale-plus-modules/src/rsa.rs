// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// RSA Data and Configuration, RSA Data and Configuration
pub static mut RSA: *mut Registers = 0xffce002c as *mut Registers;
register_structs! {
    pub Registers {
        /// Write Data 0
        (0x00000000 => pub wr_data_0: WriteOnly<u32>),
        /// Write Data 1
        (0x00000004 => pub wr_data_1: WriteOnly<u32>),
        /// Write Data 2
        (0x00000008 => pub wr_data_2: WriteOnly<u32>),
        /// Write Data 3
        (0x0000000c => pub wr_data_3: WriteOnly<u32>),
        /// Write Data 4
        (0x00000010 => pub wr_data_4: WriteOnly<u32>),
        /// Write Data 5
        (0x00000014 => pub wr_data_5: WriteOnly<u32>),
        /// Write Address
        (0x00000018 => pub wr_addr: WriteOnly<u32>),
        /// Read Data 0
        (0x0000001c => pub rd_data_0: ReadOnly<u32>),
        /// Read Data 1
        (0x00000020 => pub rd_data_1: ReadOnly<u32>),
        /// Read Data 2
        (0x00000024 => pub rd_data_2: ReadOnly<u32>),
        /// Read Data 3
        (0x00000028 => pub rd_data_3: ReadOnly<u32>),
        /// Read Data 4
        (0x0000002c => pub rd_data_4: ReadOnly<u32>),
        /// Read Data 5
        (0x00000030 => pub rd_data_5: ReadOnly<u32>),
        /// Read Address
        (0x00000034 => pub rd_addr: WriteOnly<u32>),
        /// RSA Control
        (0x00000038 => pub rsa_cfg: ReadWrite<u32, RsaCfg::Register>),
        /// RSA Interrupt Status
        (0x0000003c => pub rsa_isr: ReadWrite<u32, RsaIsr::Register>),
        /// RSA Interrupt Mask
        (0x00000040 => pub rsa_imr: ReadOnly<u32, RsaImr::Register>),
        /// RSA Interrupt Enable
        (0x00000044 => pub rsa_ier: WriteOnly<u32, RsaIer::Register>),
        /// RSA Interrupt Disable
        (0x00000048 => pub rsa_idr: WriteOnly<u32, RsaIdr::Register>),
        (0x0000004c => @END),
    }
}
register_bitfields! [
    u32,
    pub RsaCfg [
        RD_ENDIANNESS OFFSET(2) NUMBITS(1) [],
        WR_ENDIANNESS OFFSET(1) NUMBITS(1) [],
        SLVERR_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub RsaIsr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub RsaImr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub RsaIer [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub RsaIdr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];

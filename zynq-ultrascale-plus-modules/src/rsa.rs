// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// RSA Data and Configuration, RSA Data and Configuration
pub static mut RSA: *mut Registers = 0xffce002c as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Write Data 0
    pub wr_data_0: WriteOnly<u32>,
    /// Write Data 1
    pub wr_data_1: WriteOnly<u32>,
    /// Write Data 2
    pub wr_data_2: WriteOnly<u32>,
    /// Write Data 3
    pub wr_data_3: WriteOnly<u32>,
    /// Write Data 4
    pub wr_data_4: WriteOnly<u32>,
    /// Write Data 5
    pub wr_data_5: WriteOnly<u32>,
    /// Write Address
    pub wr_addr: WriteOnly<u32>,
    /// Read Data 0
    pub rd_data_0: ReadOnly<u32>,
    /// Read Data 1
    pub rd_data_1: ReadOnly<u32>,
    /// Read Data 2
    pub rd_data_2: ReadOnly<u32>,
    /// Read Data 3
    pub rd_data_3: ReadOnly<u32>,
    /// Read Data 4
    pub rd_data_4: ReadOnly<u32>,
    /// Read Data 5
    pub rd_data_5: ReadOnly<u32>,
    /// Read Address
    pub rd_addr: WriteOnly<u32>,
    /// RSA Control
    pub rsa_cfg: ReadWrite<u32, RsaCfg::Register>,
    /// RSA Interrupt Status
    pub rsa_isr: ReadWrite<u32, RsaIsr::Register>,
    /// RSA Interrupt Mask
    pub rsa_imr: ReadOnly<u32, RsaImr::Register>,
    /// RSA Interrupt Enable
    pub rsa_ier: WriteOnly<u32, RsaIer::Register>,
    /// RSA Interrupt Disable
    pub rsa_idr: WriteOnly<u32, RsaIdr::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub RsaCfg [
        RD_ENDIANNESS OFFSET(2) NUMBITS(1) [],
        WR_ENDIANNESS OFFSET(1) NUMBITS(1) [],
        SLVERR_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RsaIsr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RsaImr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RsaIer [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RsaIdr [
        APB_SLVERR OFFSET(0) NUMBITS(1) [],
    ]
];

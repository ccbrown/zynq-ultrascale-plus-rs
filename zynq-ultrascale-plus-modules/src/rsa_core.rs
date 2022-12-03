// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, WriteOnly};
/// RSA Core Data and Configuration, RSA Core Data and Configuration
pub static mut RSA_CORE: *mut Registers = 0xffce0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Write Data
    pub rsa_wr_data: WriteOnly<u32, RsaWrData::Register>,
    /// Write Address
    pub rsa_wr_addr: WriteOnly<u32, RsaWrAddr::Register>,
    /// Read Data
    pub rsa_rd_data: ReadOnly<u32, RsaRdData::Register>,
    /// Read Address
    pub rsa_rd_addr: WriteOnly<u32, RsaRdAddr::Register>,
    /// RSA Control
    pub ctrl: WriteOnly<u32, Ctrl::Register>,
    /// RSA Status
    pub status: ReadOnly<u32, Status::Register>,
    /// RSA MINV0
    pub minv0: WriteOnly<u32, Minv0::Register>,
    /// RSA MINV1
    pub minv1: WriteOnly<u32, Minv1::Register>,
    /// RSA MINV2
    pub minv2: WriteOnly<u32, Minv2::Register>,
    /// RSA MINV3
    pub minv3: WriteOnly<u32, Minv3::Register>,
    /// RSA Zero
    pub zero: WriteOnly<u32, Zero::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub RsaWrData [
        WR_DATA OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RsaWrAddr [
        WR_ADDR OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RsaRdData [
        RD_DATA OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RsaRdAddr [
        RD_ADDR OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctrl [
        LEN_CODE OFFSET(4) NUMBITS(4) [],
        DONE_CLR_ABORT OFFSET(3) NUMBITS(1) [],
        OPCODE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Status [
        PROG_CNT OFFSET(3) NUMBITS(5) [],
        ERROR OFFSET(2) NUMBITS(1) [],
        BUSY OFFSET(1) NUMBITS(1) [],
        DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Minv0 [
        MINV0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Minv1 [
        MINV1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Minv2 [
        MINV2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Minv3 [
        MINV2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zero [
        ZERO OFFSET(0) NUMBITS(1) [],
    ]
];

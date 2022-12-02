// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, WriteOnly};
/// RSA Core Data and Configuration, RSA Core Data and Configuration
pub static mut RSA_CORE: *mut Registers = 0xffce0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Write Data
        (0x00000000 => pub rsa_wr_data: WriteOnly<u32, RsaWrData::Register>),
        /// Write Address
        (0x00000004 => pub rsa_wr_addr: WriteOnly<u32, RsaWrAddr::Register>),
        /// Read Data
        (0x00000008 => pub rsa_rd_data: ReadOnly<u32, RsaRdData::Register>),
        /// Read Address
        (0x0000000c => pub rsa_rd_addr: WriteOnly<u32, RsaRdAddr::Register>),
        /// RSA Control
        (0x00000010 => pub ctrl: WriteOnly<u32, Ctrl::Register>),
        /// RSA Status
        (0x00000014 => pub status: ReadOnly<u32, Status::Register>),
        /// RSA MINV0
        (0x00000018 => pub minv0: WriteOnly<u32, Minv0::Register>),
        /// RSA MINV1
        (0x0000001c => pub minv1: WriteOnly<u32, Minv1::Register>),
        /// RSA MINV2
        (0x00000020 => pub minv2: WriteOnly<u32, Minv2::Register>),
        /// RSA MINV3
        (0x00000024 => pub minv3: WriteOnly<u32, Minv3::Register>),
        /// RSA Zero
        (0x00000028 => pub zero: WriteOnly<u32, Zero::Register>),
        (0x0000002c => @END),
    }
}
register_bitfields! [
    u32,
    pub RsaWrData [
        WR_DATA OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub RsaWrAddr [
        WR_ADDR OFFSET(0) NUMBITS(7) [],
    ]
];
register_bitfields! [
    u32,
    pub RsaRdData [
        RD_DATA OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub RsaRdAddr [
        RD_ADDR OFFSET(0) NUMBITS(7) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctrl [
        LEN_CODE OFFSET(4) NUMBITS(4) [],
        DONE_CLR_ABORT OFFSET(3) NUMBITS(1) [],
        OPCODE OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub Status [
        PROG_CNT OFFSET(3) NUMBITS(5) [],
        ERROR OFFSET(2) NUMBITS(1) [],
        BUSY OFFSET(1) NUMBITS(1) [],
        DONE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Minv0 [
        MINV0 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Minv1 [
        MINV1 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Minv2 [
        MINV2 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Minv3 [
        MINV2 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Zero [
        ZERO OFFSET(0) NUMBITS(1) [],
    ]
];

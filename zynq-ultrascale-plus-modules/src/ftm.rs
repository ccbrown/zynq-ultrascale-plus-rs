// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Fabric Trigger Macrocell, Fabric Trigger Macrocell interface from PL to ECT
pub static mut CORESIGHT_SOC_FTM: *mut Registers = 0xfe9d0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 16],
    /// General Purpose Input
    pub gpi: ReadOnly<u32>,
    _padding20: [u8; 12],
    /// General Purpose Output
    pub gpo: ReadWrite<u32>,
    _padding36: [u8; 3756],
    /// Trigger Output Register
    pub ittrigout: WriteOnly<u8, Ittrigout::Register>,
    _padding3793: [u8; 3],
    /// Trigger Output Acknowledge Register
    pub ittrigoutack: ReadOnly<u8, Ittrigoutack::Register>,
    _padding3797: [u8; 3],
    /// Trigger Input Register
    pub ittrigin: ReadOnly<u8, Ittrigin::Register>,
    _padding3801: [u8; 3],
    /// Trigger Input Acknowledge Register
    pub ittriginack: WriteOnly<u8, Ittriginack::Register>,
    _padding3805: [u8; 35],
    /// Integration Control Register
    pub itctrl: ReadWrite<u8, Itctrl::Register>,
    _padding3841: [u8; 159],
    /// Claim Tag Set Register
    pub claimset: ReadWrite<u8, Claimset::Register>,
    _padding4001: [u8; 3],
    /// Claim Tag Clear Register
    pub claimclr: ReadWrite<u8, Claimclr::Register>,
    _padding4005: [u8; 11],
    /// Lock Access Register
    pub lar: WriteOnly<u32>,
    /// Lock Status Register
    pub lsr: ReadOnly<u8, Lsr::Register>,
    _padding4021: [u8; 3],
    /// Authentication Status Register
    pub authstatus: ReadOnly<u8, Authstatus::Register>,
    _padding4025: [u8; 15],
    /// Device ID
    pub devid: ReadOnly<u8, Devid::Register>,
    _padding4041: [u8; 3],
    /// Device Type
    pub devtype: ReadOnly<u8, Devtype::Register>,
    _padding4045: [u8; 3],
    /// Peripheral ID4
    pub pidr4: ReadOnly<u8, Pidr4::Register>,
    _padding4049: [u8; 3],
    /// Peripheral ID5
    pub pidr5: ReadOnly<u8, Pidr5::Register>,
    _padding4053: [u8; 3],
    /// Peripheral ID6
    pub pidr6: ReadOnly<u8, Pidr6::Register>,
    _padding4057: [u8; 3],
    /// Peripheral ID7
    pub pidr7: ReadOnly<u8, Pidr7::Register>,
    _padding4061: [u8; 3],
    /// Peripheral ID0
    pub pidr0: ReadOnly<u8, Pidr0::Register>,
    _padding4065: [u8; 3],
    /// Peripheral ID1
    pub pidr1: ReadOnly<u8, Pidr1::Register>,
    _padding4069: [u8; 3],
    /// Peripheral ID2
    pub pidr2: ReadOnly<u8, Pidr2::Register>,
    _padding4073: [u8; 3],
    /// Peripheral ID3
    pub pidr3: ReadOnly<u8, Pidr3::Register>,
    _padding4077: [u8; 3],
    /// Component ID0
    pub cidr0: ReadOnly<u8, Cidr0::Register>,
    _padding4081: [u8; 3],
    /// Component ID1
    pub cidr1: ReadOnly<u8, Cidr1::Register>,
    _padding4085: [u8; 3],
    /// Component ID2
    pub cidr2: ReadOnly<u8, Cidr2::Register>,
    _padding4089: [u8; 3],
    /// Component ID3
    pub cidr3: ReadOnly<u8, Cidr3::Register>,
}
tock_registers::register_bitfields! [
    u8,
    pub Ittrigout [
        TRIGOUT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Ittrigoutack [
        TRIGOUTACK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Ittrigin [
        TRIGIN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Ittriginack [
        TRIGINACK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Itctrl [
        INTEGRATION OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Claimset [
        SET OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Claimclr [
        CLEAR OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Lsr [
        _8BIT OFFSET(2) NUMBITS(1) [],
        STATUS OFFSET(1) NUMBITS(1) [],
        IMP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Authstatus [
        SNI OFFSET(6) NUMBITS(2) [],
        SI OFFSET(4) NUMBITS(2) [],
        NSNI OFFSET(2) NUMBITS(2) [],
        NSI OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Devid [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Devtype [
        TYPE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Pidr4 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Pidr5 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Pidr6 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Pidr7 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Pidr0 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Pidr1 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Pidr2 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Pidr3 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Cidr0 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Cidr1 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Cidr2 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Cidr3 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];

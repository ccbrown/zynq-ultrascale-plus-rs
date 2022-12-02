// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Fabric Trigger Macrocell, Fabric Trigger Macrocell interface from PL to ECT
pub static mut CORESIGHT_SOC_FTM: *mut Registers = 0xfe9d0000 as *mut Registers;
register_structs! {
    pub Registers {
        (0x00000000 => _padding0),
        /// General Purpose Input
        (0x00000010 => pub gpi: ReadOnly<u32>),
        (0x00000014 => _padding20),
        /// General Purpose Output
        (0x00000020 => pub gpo: ReadWrite<u32>),
        (0x00000024 => _padding36),
        /// Trigger Output Register
        (0x00000ed0 => pub ittrigout: WriteOnly<u8, Ittrigout::Register>),
        (0x00000ed1 => _padding3793),
        /// Trigger Output Acknowledge Register
        (0x00000ed4 => pub ittrigoutack: ReadOnly<u8, Ittrigoutack::Register>),
        (0x00000ed5 => _padding3797),
        /// Trigger Input Register
        (0x00000ed8 => pub ittrigin: ReadOnly<u8, Ittrigin::Register>),
        (0x00000ed9 => _padding3801),
        /// Trigger Input Acknowledge Register
        (0x00000edc => pub ittriginack: WriteOnly<u8, Ittriginack::Register>),
        (0x00000edd => _padding3805),
        /// Integration Control Register
        (0x00000f00 => pub itctrl: ReadWrite<u8, Itctrl::Register>),
        (0x00000f01 => _padding3841),
        /// Claim Tag Set Register
        (0x00000fa0 => pub claimset: ReadWrite<u8, Claimset::Register>),
        (0x00000fa1 => _padding4001),
        /// Claim Tag Clear Register
        (0x00000fa4 => pub claimclr: ReadWrite<u8, Claimclr::Register>),
        (0x00000fa5 => _padding4005),
        /// Lock Access Register
        (0x00000fb0 => pub lar: WriteOnly<u32>),
        /// Lock Status Register
        (0x00000fb4 => pub lsr: ReadOnly<u8, Lsr::Register>),
        (0x00000fb5 => _padding4021),
        /// Authentication Status Register
        (0x00000fb8 => pub authstatus: ReadOnly<u8, Authstatus::Register>),
        (0x00000fb9 => _padding4025),
        /// Device ID
        (0x00000fc8 => pub devid: ReadOnly<u8, Devid::Register>),
        (0x00000fc9 => _padding4041),
        /// Device Type
        (0x00000fcc => pub devtype: ReadOnly<u8, Devtype::Register>),
        (0x00000fcd => _padding4045),
        /// Peripheral ID4
        (0x00000fd0 => pub pidr4: ReadOnly<u8, Pidr4::Register>),
        (0x00000fd1 => _padding4049),
        /// Peripheral ID5
        (0x00000fd4 => pub pidr5: ReadOnly<u8, Pidr5::Register>),
        (0x00000fd5 => _padding4053),
        /// Peripheral ID6
        (0x00000fd8 => pub pidr6: ReadOnly<u8, Pidr6::Register>),
        (0x00000fd9 => _padding4057),
        /// Peripheral ID7
        (0x00000fdc => pub pidr7: ReadOnly<u8, Pidr7::Register>),
        (0x00000fdd => _padding4061),
        /// Peripheral ID0
        (0x00000fe0 => pub pidr0: ReadOnly<u8, Pidr0::Register>),
        (0x00000fe1 => _padding4065),
        /// Peripheral ID1
        (0x00000fe4 => pub pidr1: ReadOnly<u8, Pidr1::Register>),
        (0x00000fe5 => _padding4069),
        /// Peripheral ID2
        (0x00000fe8 => pub pidr2: ReadOnly<u8, Pidr2::Register>),
        (0x00000fe9 => _padding4073),
        /// Peripheral ID3
        (0x00000fec => pub pidr3: ReadOnly<u8, Pidr3::Register>),
        (0x00000fed => _padding4077),
        /// Component ID0
        (0x00000ff0 => pub cidr0: ReadOnly<u8, Cidr0::Register>),
        (0x00000ff1 => _padding4081),
        /// Component ID1
        (0x00000ff4 => pub cidr1: ReadOnly<u8, Cidr1::Register>),
        (0x00000ff5 => _padding4085),
        /// Component ID2
        (0x00000ff8 => pub cidr2: ReadOnly<u8, Cidr2::Register>),
        (0x00000ff9 => _padding4089),
        /// Component ID3
        (0x00000ffc => pub cidr3: ReadOnly<u8, Cidr3::Register>),
        (0x00000ffd => @END),
    }
}
register_bitfields! [
    u8,
    pub Ittrigout [
        TRIGOUT OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Ittrigoutack [
        TRIGOUTACK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Ittrigin [
        TRIGIN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Ittriginack [
        TRIGINACK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Itctrl [
        INTEGRATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Claimset [
        SET OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u8,
    pub Claimclr [
        CLEAR OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u8,
    pub Lsr [
        _8BIT OFFSET(2) NUMBITS(1) [],
        STATUS OFFSET(1) NUMBITS(1) [],
        IMP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Authstatus [
        SNI OFFSET(6) NUMBITS(2) [],
        SI OFFSET(4) NUMBITS(2) [],
        NSNI OFFSET(2) NUMBITS(2) [],
        NSI OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u8,
    pub Devid [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Devtype [
        TYPE OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Pidr4 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Pidr5 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Pidr6 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Pidr7 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Pidr0 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Pidr1 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Pidr2 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Pidr3 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Cidr0 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Cidr1 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Cidr2 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub Cidr3 [
        ID OFFSET(0) NUMBITS(8) [],
    ]
];

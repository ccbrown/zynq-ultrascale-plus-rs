// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// Timestamp Generator, Master Timestamp generator
pub static mut CORESIGHT_SOC_TSGEN: *mut Registers = 0xfe900000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Controls counter operation by enabling or disabling or halting the counter
        (0x00000000 => pub cntcr: ReadWrite<u32, Cntcr::Register>),
        /// Counter status register
        (0x00000004 => pub cntsr: ReadOnly<u32, Cntsr::Register>),
        /// Current value of Counter. Lower 32-bits.
        (0x00000008 => pub cntcvl: ReadWrite<u32>),
        /// Current value of Counter. Upper 32-bits.
        (0x0000000c => pub cntcvu: ReadWrite<u32>),
        (0x00000010 => _padding16),
        /// Base Frequency ID
        (0x00000020 => pub cntfid0: ReadWrite<u32>),
        (0x00000024 => _padding36),
        /// This register indicates the capabilities.
        (0x00000fc8 => pub devid: ReadOnly<u32>),
        /// It provides a debugger with information about the component.
        (0x00000fcc => pub devtype: ReadOnly<u32, Devtype::Register>),
        /// Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.
        (0x00000fd0 => pub pidr4: ReadOnly<u32, Pidr4::Register>),
        /// Reserved
        (0x00000fd4 => pub pidr5: ReadWrite<u32>),
        /// Reserved
        (0x00000fd8 => pub pidr6: ReadWrite<u32>),
        /// Reserved
        (0x00000fdc => pub pidr7: ReadWrite<u32>),
        /// Part of the set of Peripheral Identification registers. Contains part of the designer specific part number.
        (0x00000fe0 => pub pidr0: ReadOnly<u32, Pidr0::Register>),
        /// Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity.
        (0x00000fe4 => pub pidr1: ReadOnly<u32, Pidr1::Register>),
        /// Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.
        (0x00000fe8 => pub pidr2: ReadOnly<u32, Pidr2::Register>),
        /// Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields.
        (0x00000fec => pub pidr3: ReadOnly<u32, Pidr3::Register>),
        /// A component identification register, that indicates that the identification registers are present.
        (0x00000ff0 => pub cidr0: ReadOnly<u32, Cidr0::Register>),
        /// A component identification register, that indicates that the identification registers are present. This register also indicates the component class.
        (0x00000ff4 => pub cidr1: ReadOnly<u32, Cidr1::Register>),
        /// A component identification register, that indicates that the identification registers are present.
        (0x00000ff8 => pub cidr2: ReadOnly<u32, Cidr2::Register>),
        /// A component identification register, that indicates that the identification registers are present.
        (0x00000ffc => pub cidr3: ReadOnly<u32, Cidr3::Register>),
        (0x00001000 => @END),
    }
}
register_bitfields! [
    u32,
    pub Cntcr [
        CNTCR_HDBG OFFSET(1) NUMBITS(1) [],
        CNTCR_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Cntsr [
        CNTSR_DBGH OFFSET(1) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Devtype [
        SUB_TYPE OFFSET(4) NUMBITS(4) [],
        MAJOR_TYPE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr4 [
        SIZE OFFSET(4) NUMBITS(4) [],
        DES_2 OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr0 [
        PART_0 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr1 [
        DES_0 OFFSET(4) NUMBITS(4) [],
        PART_1 OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC OFFSET(3) NUMBITS(1) [],
        DES_1 OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        CMOD OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr0 [
        PRMBL_0 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr1 [
        CLASS OFFSET(4) NUMBITS(4) [],
        PRMBL_1 OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr2 [
        PRMBL_2 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr3 [
        PRMBL_3 OFFSET(0) NUMBITS(8) [],
    ]
];

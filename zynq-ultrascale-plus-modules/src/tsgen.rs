// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// Timestamp Generator, Master Timestamp generator
pub static mut CORESIGHT_SOC_TSGEN: *mut Registers = 0xfe900000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Controls counter operation by enabling or disabling or halting the counter
    pub cntcr: ReadWrite<u32, Cntcr::Register>,
    /// Counter status register
    pub cntsr: ReadOnly<u32, Cntsr::Register>,
    /// Current value of Counter. Lower 32-bits.
    pub cntcvl: ReadWrite<u32>,
    /// Current value of Counter. Upper 32-bits.
    pub cntcvu: ReadWrite<u32>,
    _padding16: [u8; 16],
    /// Base Frequency ID
    pub cntfid0: ReadWrite<u32>,
    _padding36: [u8; 4004],
    /// This register indicates the capabilities.
    pub devid: ReadOnly<u32>,
    /// It provides a debugger with information about the component.
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.
    pub pidr4: ReadOnly<u32, Pidr4::Register>,
    /// Reserved
    pub pidr5: ReadWrite<u32>,
    /// Reserved
    pub pidr6: ReadWrite<u32>,
    /// Reserved
    pub pidr7: ReadWrite<u32>,
    /// Part of the set of Peripheral Identification registers. Contains part of the designer specific part number.
    pub pidr0: ReadOnly<u32, Pidr0::Register>,
    /// Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity.
    pub pidr1: ReadOnly<u32, Pidr1::Register>,
    /// Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.
    pub pidr2: ReadOnly<u32, Pidr2::Register>,
    /// Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields.
    pub pidr3: ReadOnly<u32, Pidr3::Register>,
    /// A component identification register, that indicates that the identification registers are present.
    pub cidr0: ReadOnly<u32, Cidr0::Register>,
    /// A component identification register, that indicates that the identification registers are present. This register also indicates the component class.
    pub cidr1: ReadOnly<u32, Cidr1::Register>,
    /// A component identification register, that indicates that the identification registers are present.
    pub cidr2: ReadOnly<u32, Cidr2::Register>,
    /// A component identification register, that indicates that the identification registers are present.
    pub cidr3: ReadOnly<u32, Cidr3::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Cntcr [
        CNTCR_HDBG OFFSET(1) NUMBITS(1) [],
        CNTCR_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntsr [
        CNTSR_DBGH OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devtype [
        SUB_TYPE OFFSET(4) NUMBITS(4) [],
        MAJOR_TYPE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr4 [
        SIZE OFFSET(4) NUMBITS(4) [],
        DES_2 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr0 [
        PART_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr1 [
        DES_0 OFFSET(4) NUMBITS(4) [],
        PART_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC OFFSET(3) NUMBITS(1) [],
        DES_1 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        CMOD OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr0 [
        PRMBL_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr1 [
        CLASS OFFSET(4) NUMBITS(4) [],
        PRMBL_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr2 [
        PRMBL_2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr3 [
        PRMBL_3 OFFSET(0) NUMBITS(8) [],
    ]
];

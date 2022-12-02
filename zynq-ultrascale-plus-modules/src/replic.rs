// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Replicator forks ATB data to multiple streams, Replicator forks ATB data to multiple streams
pub static mut CORESIGHT_SOC_REPLIC: *mut Registers = 0xfe960000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Allows the setting of ID filter for Master 0.
        (0x00000000 => pub idfilter0: ReadWrite<u32, Idfilter0::Register>),
        /// Allows the setting of ID filter for Master 1.
        (0x00000004 => pub idfilter1: ReadWrite<u32, Idfilter1::Register>),
        (0x00000008 => _padding8),
        /// Returns the value of the ATREADYM0, ATREADYM1 and ATVALIDS inputs in integration mode.
        (0x00000ef8 => pub itatbctr1: ReadOnly<u32, Itatbctr1::Register>),
        /// Controls the value of the ATVALIDM0, ATVALIDM1 and ATREADYS outputs in integration mode.
        (0x00000efc => pub itatbctr0: WriteOnly<u32, Itatbctr0::Register>),
        /// Used to enable topology detection. See the CoreSight Architecture Specification for more information. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for integration testing and topology solving. Note: When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection.
        (0x00000f00 => pub itctrl: ReadWrite<u32, Itctrl::Register>),
        (0x00000f04 => _padding3844),
        /// This is used in conjunction with Claim Tag Clear Register, CLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read.
        (0x00000fa0 => pub claimset: ReadWrite<u32, Claimset::Register>),
        /// This register is used in conjunction with Claim Tag Set Register, CLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read.
        (0x00000fa4 => pub claimclr: ReadWrite<u32, Claimclr::Register>),
        (0x00000fa8 => _padding4008),
        /// This is used to enable write access to device registers.
        (0x00000fb0 => pub lar: WriteOnly<u32>),
        /// This indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, must block write accesses to any control register, except the Lock Access Register. For most components this covers all registers except for the Lock Access Register, 0xFB0.
        (0x00000fb4 => pub lsr: ReadOnly<u32, Lsr::Register>),
        /// Reports the required security level and current status of those enables. Where functionality changes on a given security level then this change in status must be reported in this register
        (0x00000fb8 => pub authstatus: ReadOnly<u32, Authstatus::Register>),
        (0x00000fbc => _padding4028),
        /// Indicates the capabilities of the CoreSight Replicator.
        (0x00000fc8 => pub devid: ReadOnly<u32, Devid::Register>),
        /// Provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information.
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
    pub Idfilter0 [
        ID0_70_7F OFFSET(7) NUMBITS(1) [],
        ID0_60_6F OFFSET(6) NUMBITS(1) [],
        ID0_50_5F OFFSET(5) NUMBITS(1) [],
        ID0_40_4F OFFSET(4) NUMBITS(1) [],
        ID0_30_3F OFFSET(3) NUMBITS(1) [],
        ID0_20_2F OFFSET(2) NUMBITS(1) [],
        ID0_10_1F OFFSET(1) NUMBITS(1) [],
        ID0_0_F OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Idfilter1 [
        ID1_70_7F OFFSET(7) NUMBITS(1) [],
        ID1_60_6F OFFSET(6) NUMBITS(1) [],
        ID1_50_5F OFFSET(5) NUMBITS(1) [],
        ID1_40_4F OFFSET(4) NUMBITS(1) [],
        ID1_30_3F OFFSET(3) NUMBITS(1) [],
        ID1_20_2F OFFSET(2) NUMBITS(1) [],
        ID1_10_1F OFFSET(1) NUMBITS(1) [],
        ID1_0_F OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Itatbctr1 [
        ATVALIDS_R OFFSET(3) NUMBITS(1) [],
        ATREADYM1_R OFFSET(1) NUMBITS(1) [],
        ATREADYM0_R OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Itatbctr0 [
        ATREADYS_W OFFSET(4) NUMBITS(1) [],
        ATVALIDM1_W OFFSET(2) NUMBITS(1) [],
        ATVALIDM0_W OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Itctrl [
        INTEGRATION_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Claimset [
        CLAIMSET OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Claimclr [
        CLAIMCLR OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Lsr [
        LOCKTYPE OFFSET(2) NUMBITS(1) [],
        LOCKGRANT OFFSET(1) NUMBITS(1) [],
        LOCKEXIST OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Authstatus [
        SNID OFFSET(6) NUMBITS(2) [],
        SID OFFSET(4) NUMBITS(2) [],
        NSNID OFFSET(2) NUMBITS(2) [],
        NSID OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub Devid [
        PORTNUM OFFSET(0) NUMBITS(4) [],
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
        FOURKB_COUNT OFFSET(4) NUMBITS(4) [],
        JEP106_CONT OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr0 [
        PART_NUMBER_BITS7TO0 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr1 [
        JEP106_BITS3TO0 OFFSET(4) NUMBITS(4) [],
        PART_NUMBER_BITS11TO8 OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC OFFSET(3) NUMBITS(1) [],
        JEP106_BITS6TO4 OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        CUSTOMER_MODIFIED OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr0 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr1 [
        CLASS OFFSET(4) NUMBITS(4) [],
        PREAMBLE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr2 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr3 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];

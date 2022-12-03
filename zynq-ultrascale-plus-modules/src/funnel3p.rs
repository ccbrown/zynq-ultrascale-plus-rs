// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Three-Port CoreSight Funnel, Funnel for multiple trace streams to single ATB
pub static mut CORESIGHT_SOC_FUNN_0: *mut Registers = 0xfe910000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Soc_dbug_lpdFunnel Control
    pub ctrl_reg: ReadWrite<u32, CtrlReg::Register>,
    /// The Priority Control Register defines the order in which inputs are selected. Each 3-bit field represents a priority for each particular slave interface. Location 0 has the priority value for the first slave port. Location 1 is the priority value for the second slave port, Location 2 is the third, down to location 7, which has the priority value of the eighth slave port. Values represent the priority value for each port number. If you want to give highest priority to a particular slave port, the corresponding port must be programmed with the lowest value. Typically this is likely to be a port that has more important data or that has a small FIFO and is therefore likely to overflow.If you want to give lowest priority to a particular slave port, the corresponding slave port must be programmed with the highest value. Typically this is likely to be a device that has a large FIFO that is less likely to overflow or a source that has information that is of lower importance.
    pub priority_ctrl_reg: ReadWrite<u32, PriorityCtrlReg::Register>,
    _padding8: [u8; 3812],
    /// The Integration Test ATB Data 0 Register performs different functions depending on whether the access is a read or a write: A write outputs data on byte boundaries of ATDATAM.A read returns the data from ATDATASn, where n is defined by the status of the Funnel Control register at 0x000. The read data is only valid when ATVALIDSn is HIGH.
    pub itatbdata0: ReadWrite<u32, Itatbdata0::Register>,
    /// The Integration Test ATB Control 2 Register performs different functions depending on whether the access is a read or a write:* A write outputs data on atreadysn, where n is defined by the status of the ATB Funnel Control Register at 0x000* A read returns the data from atreadym.
    pub itatbctr2: ReadWrite<u32, Itatbctr2::Register>,
    /// The Integration Test ATB Control 1 Register performs different functions depending on whether the access is a read or a write:* a write sets the value of the atidm.* a read returns the value of the atidmn signals, where n is defined by the status of the Control register at 0x000.
    pub itatbctr1: ReadWrite<u32, Itatbctr1::Register>,
    /// The Integration Test ATB Control 0 Register performs different functions depending on whether the access is a read or a write:* a write sets the value of the atvalidm.* a read returns the value of the atvalidsn signals, where n is defined by the status of the Control register at 0x000.
    pub itatbctr0: ReadWrite<u32, Itatbctr0::Register>,
    _padding3836: [u8; 4],
    /// This register is used to enable topology detection. For more information see the CoreSight Architecture Specification. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purpose of integration testing and topology solvingNoteWhen a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection.
    pub itctrl: ReadWrite<u32, Itctrl::Register>,
    _padding3844: [u8; 156],
    /// This is used in conjunction with Claim Tag Clear Register, CLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read.
    pub claimset: ReadWrite<u32, Claimset::Register>,
    /// This register is used in conjunction with Claim Tag Set Register, CLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read.
    pub claimclr: ReadWrite<u32, Claimclr::Register>,
    _padding4008: [u8; 8],
    /// This is used to enable write access to device registers.
    pub lar: WriteOnly<u32>,
    /// This indicates the status of the Lock control mechanism. This lock prevents accidental writes by code under debug. This register must always be present although there might not be any lock-access control mechanism. The lock mechanism, where present and locked, must block write accesses to any control register, except the Lock Access Register. For most components this covers all registers except for the Lock Access Register 0xFB0
    pub lsr: ReadOnly<u32, Lsr::Register>,
    /// Reports the required security level and current status of those enables. Where functionality changes on a given security level then this change in status must be reported in this register
    pub authstatus: ReadOnly<u32, Authstatus::Register>,
    _padding4028: [u8; 12],
    /// This indicates the capabilities of the CoreSight Funnel.
    pub devid: ReadOnly<u32, Devid::Register>,
    /// It provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information.
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.
    pub pidr4: ReadOnly<u32, Pidr4::Register>,
    /// Reserved
    pub pidr5: ReadOnly<u32>,
    /// Reserved
    pub pidr6: ReadOnly<u32>,
    /// Reserved
    pub pidr7: ReadOnly<u32>,
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
    pub CtrlReg [
        HT OFFSET(8) NUMBITS(4) [],
        ENS2 OFFSET(2) NUMBITS(1) [],
        ENS1 OFFSET(1) NUMBITS(1) [],
        ENS0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PriorityCtrlReg [
        PRIPORT2 OFFSET(6) NUMBITS(3) [],
        PRIPORT1 OFFSET(3) NUMBITS(3) [],
        PRIPORT0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbdata0 [
        ATDATA31 OFFSET(4) NUMBITS(1) [],
        ATDATAM23 OFFSET(3) NUMBITS(1) [],
        ATDATA15 OFFSET(2) NUMBITS(1) [],
        ATDATA7 OFFSET(1) NUMBITS(1) [],
        ATDATA0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbctr2 [
        AFVALID OFFSET(1) NUMBITS(1) [],
        ATREADY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbctr1 [
        ATID OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbctr0 [
        ATBYTES OFFSET(8) NUMBITS(2) [],
        AFREADY OFFSET(1) NUMBITS(1) [],
        ATVALID OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itctrl [
        INTEGRATION_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Claimset [
        CLAIMSET OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Claimclr [
        CLAIMCLR OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Lsr [
        NTT OFFSET(2) NUMBITS(1) [],
        SLK OFFSET(1) NUMBITS(1) [],
        SLI OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Authstatus [
        SNID OFFSET(6) NUMBITS(2) [],
        SID OFFSET(4) NUMBITS(2) [],
        NSNID OFFSET(2) NUMBITS(2) [],
        NSID OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devid [
        SCHEME OFFSET(4) NUMBITS(4) [],
        PORTCOUNT OFFSET(0) NUMBITS(4) [],
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
        FOURKB_COUNT OFFSET(4) NUMBITS(4) [],
        JEP106_CONT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr0 [
        PART_NUMBER_BITS7TO0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr1 [
        JEP106_BITS3TO0 OFFSET(4) NUMBITS(4) [],
        PART_NUMBER_BITS11TO8 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC OFFSET(3) NUMBITS(1) [],
        JEP106_BITS6TO4 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        CUSTOMER_MODIFIED OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr0 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr1 [
        CLASS OFFSET(4) NUMBITS(4) [],
        PREAMBLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr2 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr3 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];

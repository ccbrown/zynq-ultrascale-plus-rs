// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// SATA AHCI HBA Spec, SATA AHCI HBA Spec
pub static mut SATA_AHCI_HBA: *mut Registers = 0xfd0c0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// HBA Capabilities
    pub cap: ReadOnly<u32, Cap::Register>,
    /// Global HBA Control
    pub ghc: Aliased<u32, GhcR::Register, GhcW::Register>,
    /// Interrupt Status
    pub is: ReadWrite<u32>,
    /// Ports Implemented
    pub pi: ReadOnly<u32>,
    /// AHCI Version
    pub vs: ReadOnly<u32, Vs::Register>,
    /// Command Completion Coalescing Control
    pub ccc_ctl: Aliased<u32, CccCtlR::Register, CccCtlW::Register>,
    /// Command Completion Coalescing Ports.
    pub ccc_ports: ReadWrite<u32>,
    /// Enclosure Management Location
    pub em_loc: ReadOnly<u32, EmLoc::Register>,
    /// Enclosure Management Control.
    pub em_ctl: Aliased<u32, EmCtlR::Register, EmCtlW::Register>,
    /// HBA Capabilities Extended
    pub cap2: ReadOnly<u32, Cap2::Register>,
    /// BIOS/OS Handoff Control and Status
    pub bohc: Aliased<u32, BohcR::Register, BohcW::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Cap [
        S64A OFFSET(31) NUMBITS(1) [],
        SNCQ OFFSET(30) NUMBITS(1) [],
        SSNTF OFFSET(29) NUMBITS(1) [],
        SMPS OFFSET(28) NUMBITS(1) [],
        SSS OFFSET(27) NUMBITS(1) [],
        SALP OFFSET(26) NUMBITS(1) [],
        SAL OFFSET(25) NUMBITS(1) [],
        SCLO OFFSET(24) NUMBITS(1) [],
        ISS OFFSET(20) NUMBITS(4) [],
        RESERVED0 OFFSET(19) NUMBITS(1) [],
        SAM OFFSET(18) NUMBITS(1) [],
        SPM OFFSET(17) NUMBITS(1) [],
        FBSS OFFSET(16) NUMBITS(1) [],
        PMD OFFSET(15) NUMBITS(1) [],
        SSC OFFSET(14) NUMBITS(1) [],
        PSC OFFSET(13) NUMBITS(1) [],
        NCS OFFSET(8) NUMBITS(5) [],
        CCCS OFFSET(7) NUMBITS(1) [],
        EMS OFFSET(6) NUMBITS(1) [],
        SXS OFFSET(5) NUMBITS(1) [],
        NP OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GhcR [
        AE OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(28) [],
        MRSM OFFSET(2) NUMBITS(1) [],
        IE OFFSET(1) NUMBITS(1) [],
        HR OFFSET(0) NUMBITS(1) [],
    ],
    pub GhcW [
        AE OFFSET(31) NUMBITS(1) [],
        IE OFFSET(1) NUMBITS(1) [],
        HR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vs [
        MJR OFFSET(16) NUMBITS(16) [],
        MNR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CccCtlR [
        TV OFFSET(16) NUMBITS(16) [],
        CC OFFSET(8) NUMBITS(8) [],
        INT OFFSET(3) NUMBITS(5) [],
        RESERVED0 OFFSET(1) NUMBITS(2) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub CccCtlW [
        TV OFFSET(16) NUMBITS(16) [],
        CC OFFSET(8) NUMBITS(8) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EmLoc [
        OFST OFFSET(16) NUMBITS(16) [],
        SZ OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EmCtlR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        ATTR_PM OFFSET(27) NUMBITS(1) [],
        ATTR_ALHD OFFSET(26) NUMBITS(1) [],
        ATTR_XMT OFFSET(25) NUMBITS(1) [],
        ATTR_SMB OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        SUPP_SGPIO OFFSET(19) NUMBITS(1) [],
        SUPP_SES2 OFFSET(18) NUMBITS(1) [],
        SUPP_SAFTE OFFSET(17) NUMBITS(1) [],
        SUPP_LED OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        CTL_RST OFFSET(9) NUMBITS(1) [],
        CTL_TM OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        STS_MR OFFSET(0) NUMBITS(1) [],
    ],
    pub EmCtlW [
        CTL_RST OFFSET(9) NUMBITS(1) [],
        CTL_TM OFFSET(8) NUMBITS(1) [],
        STS_MR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cap2 [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DESO OFFSET(5) NUMBITS(1) [],
        SADM OFFSET(4) NUMBITS(1) [],
        SDS OFFSET(3) NUMBITS(1) [],
        APST OFFSET(2) NUMBITS(1) [],
        NVMP OFFSET(1) NUMBITS(1) [],
        BOH OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BohcR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        BB OFFSET(4) NUMBITS(1) [],
        OOC OFFSET(3) NUMBITS(1) [],
        SOOE OFFSET(2) NUMBITS(1) [],
        OOS OFFSET(1) NUMBITS(1) [],
        BOS OFFSET(0) NUMBITS(1) [],
    ],
    pub BohcW [
        BB OFFSET(4) NUMBITS(1) [],
        OOC OFFSET(3) NUMBITS(1) [],
        SOOE OFFSET(2) NUMBITS(1) [],
        OOS OFFSET(1) NUMBITS(1) [],
        BOS OFFSET(0) NUMBITS(1) [],
    ]
];

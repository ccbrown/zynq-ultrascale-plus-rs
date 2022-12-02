// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// PL-PS AXI Channel Configuration, QoS and FIFO Configuration; S_AXI_HPC0_FPD
pub static mut AFIFM0: *mut Registers = 0xfd360000 as *mut Registers;
/// PL-PS AXI Channel Configuration, QoS and FIFO Configuration; S_AXI_HPC1_FPD
pub static mut AFIFM1: *mut Registers = 0xfd370000 as *mut Registers;
/// PL-PS AXI Channel Configuration, QoS and FIFO Configuration; S_AXI_HP0_FPD
pub static mut AFIFM2: *mut Registers = 0xfd380000 as *mut Registers;
/// PL-PS AXI Channel Configuration, QoS and FIFO Configuration; S_AXI_HP1_FPD
pub static mut AFIFM3: *mut Registers = 0xfd390000 as *mut Registers;
/// PL-PS AXI Channel Configuration, QoS and FIFO Configuration; S_AXI_HP2_FPD
pub static mut AFIFM4: *mut Registers = 0xfd3a0000 as *mut Registers;
/// PL-PS AXI Channel Configuration, QoS and FIFO Configuration; S_AXI_HP3_FPD
pub static mut AFIFM5: *mut Registers = 0xfd3b0000 as *mut Registers;
/// PL-PS AXI Channel Configuration, QoS and FIFO Configuration; S_AXI_LPD
pub static mut AFIFM6: *mut Registers = 0xff9b0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Read Channel Control Register
        (0x00000000 => pub rdctrl: Aliased<u32, RdctrlR::Register, RdctrlW::Register>),
        /// Read Issuing Capability Register
        (0x00000004 => pub rdissue: Aliased<u32, RdissueR::Register, RdissueW::Register>),
        /// QoS Read Channel Register
        (0x00000008 => pub rdqos: Aliased<u32, RdqosR::Register, RdqosW::Register>),
        (0x0000000c => _padding12),
        /// Read Channel Debug Register
        (0x00000010 => pub rddebug: ReadOnly<u32, Rddebug::Register>),
        /// Write Channel Control Register
        (0x00000014 => pub wrctrl: Aliased<u32, WrctrlR::Register, WrctrlW::Register>),
        /// Write Issuing Capability Register
        (0x00000018 => pub wrissue: Aliased<u32, WrissueR::Register, WrissueW::Register>),
        /// QoS Write Channel Register
        (0x0000001c => pub wrqos: Aliased<u32, WrqosR::Register, WrqosW::Register>),
        (0x00000020 => _padding32),
        /// Interrupt Status Register
        (0x00000e00 => pub i_sts: Aliased<u32, IStsR::Register, IStsW::Register>),
        /// Interrupt Enable
        (0x00000e04 => pub i_en: Aliased<u32, IEnR::Register, IEnW::Register>),
        /// Interrupt Disable
        (0x00000e08 => pub i_dis: Aliased<u32, IDisR::Register, IDisW::Register>),
        /// Interrupt Mask
        (0x00000e0c => pub i_mask: ReadOnly<u32, IMask::Register>),
        (0x00000e10 => _padding3600),
        /// General Control Register
        (0x00000f04 => pub control: Aliased<u32, ControlR::Register, ControlW::Register>),
        (0x00000f08 => _padding3848),
        /// Safety endpoint connectivity check Register
        (0x00000f0c => pub safety_chk: ReadWrite<u32>),
        (0x00000f10 => @END),
    }
}
register_bitfields! [
    u32,
    pub RdctrlR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        RESERVED1 OFFSET(8) NUMBITS(3) [],
        RESERVED2 OFFSET(7) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(3) [],
        PAUSE OFFSET(3) NUMBITS(1) [],
        FABRIC_QOS_EN OFFSET(2) NUMBITS(1) [],
        FABRIC_WIDTH OFFSET(0) NUMBITS(2) [],
    ],
    pub RdctrlW [
        RESERVED0 OFFSET(8) NUMBITS(3) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(3) [],
        PAUSE OFFSET(3) NUMBITS(1) [],
        FABRIC_QOS_EN OFFSET(2) NUMBITS(1) [],
        FABRIC_WIDTH OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub RdissueR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        CAPABILITY OFFSET(0) NUMBITS(4) [],
    ],
    pub RdissueW [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
        CAPABILITY OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub RdqosR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub RdqosW [
        VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Rddebug [
        AFI_VERSION OFFSET(30) NUMBITS(2) [],
        RESERVED0 OFFSET(6) NUMBITS(24) [],
        RESERVED1 OFFSET(1) NUMBITS(5) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub WrctrlR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        WR_RELEASE_MODE OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        RESERVED2 OFFSET(8) NUMBITS(3) [],
        RESERVED3 OFFSET(7) NUMBITS(1) [],
        RESERVED4 OFFSET(4) NUMBITS(3) [],
        PAUSE OFFSET(3) NUMBITS(1) [],
        FABRIC_QOS_EN OFFSET(2) NUMBITS(1) [],
        FABRIC_WIDTH OFFSET(0) NUMBITS(2) [],
    ],
    pub WrctrlW [
        WR_RELEASE_MODE OFFSET(12) NUMBITS(1) [],
        RESERVED0 OFFSET(8) NUMBITS(3) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(3) [],
        PAUSE OFFSET(3) NUMBITS(1) [],
        FABRIC_QOS_EN OFFSET(2) NUMBITS(1) [],
        FABRIC_WIDTH OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub WrissueR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        CAPABILITY OFFSET(0) NUMBITS(4) [],
    ],
    pub WrissueW [
        CAPABILITY OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub WrqosR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub WrqosW [
        VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub IStsR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        INVALID_APB OFFSET(0) NUMBITS(1) [],
    ],
    pub IStsW [
        INVALID_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IEnR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub IEnW [
        INVALID_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IDisR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub IDisW [
        INVALID_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IMask [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        INVALID_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ControlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        APB_ERR_RESP OFFSET(0) NUMBITS(1) [],
    ],
    pub ControlW [
        APB_ERR_RESP OFFSET(0) NUMBITS(1) [],
    ]
];

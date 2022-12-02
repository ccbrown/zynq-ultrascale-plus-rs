// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// APU 1 Performance Monitor Unit profiler, A53 Performance Monitor Unit profiler
pub static mut CORESIGHT_A53_PMU_1: *mut Registers = 0xfed30000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Performance Monitors Event Count Registers
        (0x00000000 => pub evcntr0_el0: ReadWrite<u32>),
        (0x00000004 => _padding4),
        /// Performance Monitors Event Count Registers
        (0x00000008 => pub evcntr1_el0: ReadWrite<u32>),
        (0x0000000c => _padding12),
        /// Performance Monitors Event Count Registers
        (0x00000010 => pub evcntr2_el0: ReadWrite<u32>),
        (0x00000014 => _padding20),
        /// Performance Monitors Event Count Registers
        (0x00000018 => pub evcntr3_el0: ReadWrite<u32>),
        (0x0000001c => _padding28),
        /// Performance Monitors Event Count Registers
        (0x00000020 => pub evcntr4_el0: ReadWrite<u32>),
        (0x00000024 => _padding36),
        /// Performance Monitors Event Count Registers
        (0x00000028 => pub evcntr5_el0: ReadWrite<u32>),
        (0x0000002c => _padding44),
        /// Performance Monitors Cycle Counter (low word)
        (0x000000f8 => pub ccntr_el0_31to0: ReadWrite<u32>),
        /// Performance Monitors Cycle Counter (high word)
        (0x000000fc => pub ccntr_el0_63to32: ReadWrite<u32>),
        (0x00000100 => _padding256),
        /// Performance Monitors Event Type Registers
        (0x00000400 => pub evtyper0_el0: ReadWrite<u32, Evtyper0El0::Register>),
        /// Performance Monitors Event Type Registers
        (0x00000404 => pub evtyper1_el0: ReadWrite<u32, Evtyper1El0::Register>),
        /// Performance Monitors Event Type Registers
        (0x00000408 => pub evtyper2_el0: ReadWrite<u32, Evtyper2El0::Register>),
        /// Performance Monitors Event Type Registers
        (0x0000040c => pub evtyper3_el0: ReadWrite<u32, Evtyper3El0::Register>),
        /// Performance Monitors Event Type Registers
        (0x00000410 => pub evtyper4_el0: ReadWrite<u32, Evtyper4El0::Register>),
        /// Performance Monitors Event Type Registers
        (0x00000414 => pub evtyper5_el0: ReadWrite<u32, Evtyper5El0::Register>),
        (0x00000418 => _padding1048),
        /// Performance Monitors Cycle Counter Filter Register
        (0x0000047c => pub ccfiltr_el0: ReadWrite<u32, CcfiltrEl0::Register>),
        (0x00000480 => _padding1152),
        /// Performance Monitors Count Enable Set Register
        (0x00000c00 => pub cntenset_el0: ReadWrite<u32, CntensetEl0::Register>),
        (0x00000c04 => _padding3076),
        /// Performance Monitors Count Enable Clear Register
        (0x00000c20 => pub cntenclr_el0: ReadWrite<u32, CntenclrEl0::Register>),
        (0x00000c24 => _padding3108),
        /// Performance Monitors Interrupt Enable Set Register
        (0x00000c40 => pub intenset_el1: ReadWrite<u32, IntensetEl1::Register>),
        (0x00000c44 => _padding3140),
        /// Performance Monitors Interrupt Enable Clear Register
        (0x00000c60 => pub intenclr_el1: ReadWrite<u32, IntenclrEl1::Register>),
        (0x00000c64 => _padding3172),
        /// Performance Monitors Overflow Flag Status Clear Register
        (0x00000c80 => pub ovsclr_el0: ReadWrite<u32, OvsclrEl0::Register>),
        (0x00000c84 => _padding3204),
        /// Performance Monitors Software Increment Register
        (0x00000ca0 => pub swinc_el0: WriteOnly<u32, SwincEl0::Register>),
        (0x00000ca4 => _padding3236),
        /// Performance Monitors Overflow Flag Status Set Register
        (0x00000cc0 => pub ovsset_el0: ReadWrite<u32, OvssetEl0::Register>),
        (0x00000cc4 => _padding3268),
        /// Performance Monitors Configuration Register
        (0x00000e00 => pub cfgr: ReadOnly<u32, Cfgr::Register>),
        /// Performance Monitors Control Register
        (0x00000e04 => pub cr_el0: ReadWrite<u32, CrEl0::Register>),
        (0x00000e08 => _padding3592),
        /// Performance Monitors Common Event Identification Register 0
        (0x00000e20 => pub ceid0_el0: ReadOnly<u32>),
        /// Performance Monitors Common Event Identification Register 1
        (0x00000e24 => pub ceid1_el0: ReadOnly<u32, Ceid1El0::Register>),
        (0x00000e28 => _padding3624),
        /// Performance Monitors Integration mode Control Register
        (0x00000f00 => pub itctrl: ReadWrite<u32, Itctrl::Register>),
        (0x00000f04 => _padding3844),
        /// Performance Monitors Device Affinity Register 0
        (0x00000fa8 => pub devaff0: ReadOnly<u32>),
        /// Performance Monitors Device Affinity Register 1
        (0x00000fac => pub devaff1: ReadOnly<u32>),
        /// Performance Monitors Lock Access Register
        (0x00000fb0 => pub lar: WriteOnly<u32>),
        /// Performance Monitors Lock Status Register
        (0x00000fb4 => pub lsr: ReadOnly<u32, Lsr::Register>),
        /// Performance Monitors Authentication Status Register
        (0x00000fb8 => pub authstatus: ReadOnly<u32, Authstatus::Register>),
        /// Performance Monitors Device Architecture Register
        (0x00000fbc => pub devarch: ReadOnly<u32, Devarch::Register>),
        (0x00000fc0 => _padding4032),
        /// Device ID Register
        (0x00000fc8 => pub devid: ReadOnly<u32>),
        /// Performance Monitors Device Type Register
        (0x00000fcc => pub devtype: ReadOnly<u32, Devtype::Register>),
        /// Performance Monitors Peripheral Identification Register 4
        (0x00000fd0 => pub pidr4: ReadOnly<u32, Pidr4::Register>),
        /// Performance Monitors Peripheral Identification Register 4
        (0x00000fd4 => pub pidr5: ReadOnly<u32>),
        /// Performance Monitors Peripheral Identification Register 4
        (0x00000fd8 => pub pidr6: ReadOnly<u32>),
        /// Performance Monitors Peripheral Identification Register 4
        (0x00000fdc => pub pidr7: ReadOnly<u32>),
        /// Performance Monitors Peripheral Identification Register 0
        (0x00000fe0 => pub pidr0: ReadOnly<u32, Pidr0::Register>),
        /// Performance Monitors Peripheral Identification Register 1
        (0x00000fe4 => pub pidr1: ReadOnly<u32, Pidr1::Register>),
        /// Performance Monitors Peripheral Identification Register 2
        (0x00000fe8 => pub pidr2: ReadOnly<u32, Pidr2::Register>),
        /// Performance Monitors Peripheral Identification Register 3
        (0x00000fec => pub pidr3: ReadOnly<u32, Pidr3::Register>),
        /// Performance Monitors Component Identification Register 0
        (0x00000ff0 => pub cidr0: ReadOnly<u32, Cidr0::Register>),
        /// Performance Monitors Component Identification Register 1
        (0x00000ff4 => pub cidr1: ReadOnly<u32, Cidr1::Register>),
        /// Performance Monitors Component Identification Register 2
        (0x00000ff8 => pub cidr2: ReadOnly<u32, Cidr2::Register>),
        /// Performance Monitors Component Identification Register 3
        (0x00000ffc => pub cidr3: ReadOnly<u32, Cidr3::Register>),
        (0x00001000 => @END),
    }
}
register_bitfields! [
    u32,
    pub Evtyper0El0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSK OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        NSH OFFSET(27) NUMBITS(1) [],
        M OFFSET(26) NUMBITS(1) [],
        EVTCOUNT OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u32,
    pub Evtyper1El0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSK OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        NSH OFFSET(27) NUMBITS(1) [],
        M OFFSET(26) NUMBITS(1) [],
        EVTCOUNT OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u32,
    pub Evtyper2El0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSK OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        NSH OFFSET(27) NUMBITS(1) [],
        M OFFSET(26) NUMBITS(1) [],
        EVTCOUNT OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u32,
    pub Evtyper3El0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSK OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        NSH OFFSET(27) NUMBITS(1) [],
        M OFFSET(26) NUMBITS(1) [],
        EVTCOUNT OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u32,
    pub Evtyper4El0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSK OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        NSH OFFSET(27) NUMBITS(1) [],
        M OFFSET(26) NUMBITS(1) [],
        EVTCOUNT OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u32,
    pub Evtyper5El0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSK OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        NSH OFFSET(27) NUMBITS(1) [],
        M OFFSET(26) NUMBITS(1) [],
        EVTCOUNT OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u32,
    pub CcfiltrEl0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSK OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        NSH OFFSET(27) NUMBITS(1) [],
        M OFFSET(26) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub CntensetEl0 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
register_bitfields! [
    u32,
    pub CntenclrEl0 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
register_bitfields! [
    u32,
    pub IntensetEl1 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
register_bitfields! [
    u32,
    pub IntenclrEl1 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
register_bitfields! [
    u32,
    pub OvsclrEl0 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
register_bitfields! [
    u32,
    pub SwincEl0 [
        P OFFSET(0) NUMBITS(31) [],
    ]
];
register_bitfields! [
    u32,
    pub OvssetEl0 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
register_bitfields! [
    u32,
    pub Cfgr [
        UEN OFFSET(19) NUMBITS(1) [],
        WT OFFSET(18) NUMBITS(1) [],
        NA OFFSET(17) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub CrEl0 [
        LC OFFSET(6) NUMBITS(1) [],
        DP OFFSET(5) NUMBITS(1) [],
        X OFFSET(4) NUMBITS(1) [],
        D OFFSET(3) NUMBITS(1) [],
        C OFFSET(2) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Ceid1El0 [
        CE_32 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Itctrl [
        IME OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Lsr [
        NTT OFFSET(2) NUMBITS(1) [],
        SLK OFFSET(1) NUMBITS(1) [],
        SLI OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Authstatus [
        SNID OFFSET(6) NUMBITS(2) [],
        NSNID OFFSET(2) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub Devarch [
        ARCHITECT OFFSET(21) NUMBITS(11) [],
        PRESENT OFFSET(20) NUMBITS(1) [],
        REVISION OFFSET(16) NUMBITS(4) [],
        ARCHID OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Devtype [
        SUB OFFSET(4) NUMBITS(4) [],
        MAJOR OFFSET(0) NUMBITS(4) [],
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

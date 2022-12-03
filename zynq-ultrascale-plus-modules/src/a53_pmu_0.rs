// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// APU 0 Performance Monitor Unit profiler, A53 Performance Monitor Unit profiler
pub static mut CORESIGHT_A53_PMU_0: *mut Registers = 0xfec30000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Performance Monitors Event Count Registers
    pub evcntr0_el0: ReadWrite<u32>,
    _padding4: [u8; 4],
    /// Performance Monitors Event Count Registers
    pub evcntr1_el0: ReadWrite<u32>,
    _padding12: [u8; 4],
    /// Performance Monitors Event Count Registers
    pub evcntr2_el0: ReadWrite<u32>,
    _padding20: [u8; 4],
    /// Performance Monitors Event Count Registers
    pub evcntr3_el0: ReadWrite<u32>,
    _padding28: [u8; 4],
    /// Performance Monitors Event Count Registers
    pub evcntr4_el0: ReadWrite<u32>,
    _padding36: [u8; 4],
    /// Performance Monitors Event Count Registers
    pub evcntr5_el0: ReadWrite<u32>,
    _padding44: [u8; 204],
    /// Performance Monitors Cycle Counter (low word)
    pub ccntr_el0_31to0: ReadWrite<u32>,
    /// Performance Monitors Cycle Counter (high word)
    pub ccntr_el0_63to32: ReadWrite<u32>,
    _padding256: [u8; 768],
    /// Performance Monitors Event Type Registers
    pub evtyper0_el0: ReadWrite<u32, Evtyper0El0::Register>,
    /// Performance Monitors Event Type Registers
    pub evtyper1_el0: ReadWrite<u32, Evtyper1El0::Register>,
    /// Performance Monitors Event Type Registers
    pub evtyper2_el0: ReadWrite<u32, Evtyper2El0::Register>,
    /// Performance Monitors Event Type Registers
    pub evtyper3_el0: ReadWrite<u32, Evtyper3El0::Register>,
    /// Performance Monitors Event Type Registers
    pub evtyper4_el0: ReadWrite<u32, Evtyper4El0::Register>,
    /// Performance Monitors Event Type Registers
    pub evtyper5_el0: ReadWrite<u32, Evtyper5El0::Register>,
    _padding1048: [u8; 100],
    /// Performance Monitors Cycle Counter Filter Register
    pub ccfiltr_el0: ReadWrite<u32, CcfiltrEl0::Register>,
    _padding1152: [u8; 1920],
    /// Performance Monitors Count Enable Set Register
    pub cntenset_el0: ReadWrite<u32, CntensetEl0::Register>,
    _padding3076: [u8; 28],
    /// Performance Monitors Count Enable Clear Register
    pub cntenclr_el0: ReadWrite<u32, CntenclrEl0::Register>,
    _padding3108: [u8; 28],
    /// Performance Monitors Interrupt Enable Set Register
    pub intenset_el1: ReadWrite<u32, IntensetEl1::Register>,
    _padding3140: [u8; 28],
    /// Performance Monitors Interrupt Enable Clear Register
    pub intenclr_el1: ReadWrite<u32, IntenclrEl1::Register>,
    _padding3172: [u8; 28],
    /// Performance Monitors Overflow Flag Status Clear Register
    pub ovsclr_el0: ReadWrite<u32, OvsclrEl0::Register>,
    _padding3204: [u8; 28],
    /// Performance Monitors Software Increment Register
    pub swinc_el0: WriteOnly<u32, SwincEl0::Register>,
    _padding3236: [u8; 28],
    /// Performance Monitors Overflow Flag Status Set Register
    pub ovsset_el0: ReadWrite<u32, OvssetEl0::Register>,
    _padding3268: [u8; 316],
    /// Performance Monitors Configuration Register
    pub cfgr: ReadOnly<u32, Cfgr::Register>,
    /// Performance Monitors Control Register
    pub cr_el0: ReadWrite<u32, CrEl0::Register>,
    _padding3592: [u8; 24],
    /// Performance Monitors Common Event Identification Register 0
    pub ceid0_el0: ReadOnly<u32>,
    /// Performance Monitors Common Event Identification Register 1
    pub ceid1_el0: ReadOnly<u32, Ceid1El0::Register>,
    _padding3624: [u8; 216],
    /// Performance Monitors Integration mode Control Register
    pub itctrl: ReadWrite<u32, Itctrl::Register>,
    _padding3844: [u8; 164],
    /// Performance Monitors Device Affinity Register 0
    pub devaff0: ReadOnly<u32>,
    /// Performance Monitors Device Affinity Register 1
    pub devaff1: ReadOnly<u32>,
    /// Performance Monitors Lock Access Register
    pub lar: WriteOnly<u32>,
    /// Performance Monitors Lock Status Register
    pub lsr: ReadOnly<u32, Lsr::Register>,
    /// Performance Monitors Authentication Status Register
    pub authstatus: ReadOnly<u32, Authstatus::Register>,
    /// Performance Monitors Device Architecture Register
    pub devarch: ReadOnly<u32, Devarch::Register>,
    _padding4032: [u8; 8],
    /// Device ID Register
    pub devid: ReadOnly<u32>,
    /// Performance Monitors Device Type Register
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// Performance Monitors Peripheral Identification Register 4
    pub pidr4: ReadOnly<u32, Pidr4::Register>,
    /// Performance Monitors Peripheral Identification Register 4
    pub pidr5: ReadOnly<u32>,
    /// Performance Monitors Peripheral Identification Register 4
    pub pidr6: ReadOnly<u32>,
    /// Performance Monitors Peripheral Identification Register 4
    pub pidr7: ReadOnly<u32>,
    /// Performance Monitors Peripheral Identification Register 0
    pub pidr0: ReadOnly<u32, Pidr0::Register>,
    /// Performance Monitors Peripheral Identification Register 1
    pub pidr1: ReadOnly<u32, Pidr1::Register>,
    /// Performance Monitors Peripheral Identification Register 2
    pub pidr2: ReadOnly<u32, Pidr2::Register>,
    /// Performance Monitors Peripheral Identification Register 3
    pub pidr3: ReadOnly<u32, Pidr3::Register>,
    /// Performance Monitors Component Identification Register 0
    pub cidr0: ReadOnly<u32, Cidr0::Register>,
    /// Performance Monitors Component Identification Register 1
    pub cidr1: ReadOnly<u32, Cidr1::Register>,
    /// Performance Monitors Component Identification Register 2
    pub cidr2: ReadOnly<u32, Cidr2::Register>,
    /// Performance Monitors Component Identification Register 3
    pub cidr3: ReadOnly<u32, Cidr3::Register>,
}
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub CntensetEl0 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CntenclrEl0 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntensetEl1 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntenclrEl1 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OvsclrEl0 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SwincEl0 [
        P OFFSET(0) NUMBITS(31) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OvssetEl0 [
        C OFFSET(31) NUMBITS(1) [],
        P OFFSET(0) NUMBITS(31) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub Ceid1El0 [
        CE_32 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itctrl [
        IME OFFSET(0) NUMBITS(1) [],
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
        NSNID OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devarch [
        ARCHITECT OFFSET(21) NUMBITS(11) [],
        PRESENT OFFSET(20) NUMBITS(1) [],
        REVISION OFFSET(16) NUMBITS(4) [],
        ARCHID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devtype [
        SUB OFFSET(4) NUMBITS(4) [],
        MAJOR OFFSET(0) NUMBITS(4) [],
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

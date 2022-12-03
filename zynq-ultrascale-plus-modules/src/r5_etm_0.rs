// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// RPU 0 Embedded Trace Macrocell, R5 Embedded Trace Macrocell
pub static mut CORESIGHT_R5_ETM_0: *mut Registers = 0xfebfc000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Main Control Register
    pub cr: ReadWrite<u32, Cr::Register>,
    /// Configuration Code Register
    pub ccr: ReadOnly<u32, Ccr::Register>,
    /// Trigger Event Register
    pub trigger: ReadWrite<u32, Trigger::Register>,
    _padding12: [u8; 4],
    /// Status Register
    pub sr: Aliased<u32, SrR::Register, SrW::Register>,
    /// System Configruation Register
    pub scr: ReadOnly<u32, Scr::Register>,
    /// TraceEnable Start/Stop Control Register
    pub tsscr: ReadWrite<u32, Tsscr::Register>,
    /// TraceEnable Control 2 Register
    pub tecr2: ReadWrite<u32, Tecr2::Register>,
    /// TraceEnable Event Register
    pub teevr: ReadWrite<u32, Teevr::Register>,
    /// TraceEnable Control1 Register
    pub tecr1: ReadWrite<u32, Tecr1::Register>,
    _padding40: [u8; 4],
    /// FIFOFULL Level Register
    pub fflr: ReadWrite<u32, Fflr::Register>,
    /// ViewData Event Register
    pub vdevr: ReadWrite<u32, Vdevr::Register>,
    /// ViewData Control 1 Register
    pub vdcr1: ReadWrite<u32, Vdcr1::Register>,
    _padding56: [u8; 4],
    /// ViewData Control 3 Register
    pub vdcr3: ReadWrite<u32, Vdcr3::Register>,
    /// Address Comparator Value 1 Register
    pub acvr1: ReadWrite<u32>,
    /// Address Comparator Value 2 Register
    pub acvr2: ReadWrite<u32>,
    /// Address Comparator Value 3 Register
    pub acvr3: ReadWrite<u32>,
    /// Address Comparator Value 4 Register
    pub acvr4: ReadWrite<u32>,
    /// Address Comparator Value 5 Register
    pub acvr5: ReadWrite<u32>,
    /// Address Comparator Value 6 Register
    pub acvr6: ReadWrite<u32>,
    /// Address Comparator Value 7 Register
    pub acvr7: ReadWrite<u32>,
    /// Address Comparator Value 8 Register
    pub acvr8: ReadWrite<u32>,
    _padding96: [u8; 32],
    /// Address Comparator Access Type 1 Register
    pub actr1: ReadWrite<u32, Actr1::Register>,
    /// Address Comparator Access Type 2 Register
    pub actr2: ReadWrite<u32, Actr2::Register>,
    /// Address Comparator Access Type 3 Register
    pub actr3: ReadWrite<u32, Actr3::Register>,
    /// Address Comparator Access Type 4 Register
    pub actr4: ReadWrite<u32, Actr4::Register>,
    /// Address Comparator Access Type 5 Register
    pub actr5: ReadWrite<u32, Actr5::Register>,
    /// Address Comparator Access Type 6 Register
    pub actr6: ReadWrite<u32, Actr6::Register>,
    /// Address Comparator Access Type 7 Register
    pub actr7: ReadWrite<u32, Actr7::Register>,
    /// Address Comparator Access Type 8 Register
    pub actr8: ReadWrite<u32, Actr8::Register>,
    _padding160: [u8; 32],
    /// Data Comparator Value 1 Register
    pub dcvr1: ReadWrite<u32>,
    _padding196: [u8; 4],
    /// Data Comparator Value 3 Register
    pub dcvr3: ReadWrite<u32>,
    _padding204: [u8; 52],
    /// Data Comparator Mask 1 Register
    pub dcmr1: ReadWrite<u32>,
    _padding260: [u8; 4],
    /// Data Comparator Mask 3 Register
    pub dcmr3: ReadWrite<u32>,
    _padding268: [u8; 52],
    /// Counter Reload Value 1 Register
    pub cntrldvr1: ReadWrite<u32, Cntrldvr1::Register>,
    /// Counter Reload Value 2 Register
    pub cntrldvr2: ReadWrite<u32, Cntrldvr2::Register>,
    _padding328: [u8; 8],
    /// Counter Enable Event 1 Register
    pub cntenr1: ReadWrite<u32, Cntenr1::Register>,
    /// Counter Enable Event 2 Register
    pub cntenr2: ReadWrite<u32, Cntenr2::Register>,
    _padding344: [u8; 8],
    /// Counter Reload Event 1 Register
    pub cntrldevr1: ReadWrite<u32, Cntrldevr1::Register>,
    /// Counter Reload Event 2 Register
    pub cntrldevr2: ReadWrite<u32, Cntrldevr2::Register>,
    _padding360: [u8; 8],
    /// Counter Value 1 Register
    pub cntvr1: ReadWrite<u32, Cntvr1::Register>,
    /// Counter Value 2 Register
    pub cntvr2: ReadWrite<u32, Cntvr2::Register>,
    _padding376: [u8; 8],
    /// Sequencer State Transition Event 1 Register
    pub sqevr1: ReadWrite<u32, Sqevr1::Register>,
    /// Sequencer State Transition Event 2 Register
    pub sqevr2: ReadWrite<u32, Sqevr2::Register>,
    /// Sequencer State Transition Event 3 Register
    pub sqevr3: ReadWrite<u32, Sqevr3::Register>,
    /// Sequencer State Transition Event 4 Register
    pub sqevr4: ReadWrite<u32, Sqevr4::Register>,
    /// Sequencer State Transition Event 5 Register
    pub sqevr5: ReadWrite<u32, Sqevr5::Register>,
    /// Sequencer State Transition Event 6 Register
    pub sqevr6: ReadWrite<u32, Sqevr6::Register>,
    _padding408: [u8; 4],
    /// Current Sequencer State Register
    pub sqr: ReadWrite<u32, Sqr::Register>,
    /// External Output Event 1 Register
    pub extoutevr1: ReadWrite<u32, Extoutevr1::Register>,
    /// External Output Event 2 Register
    pub extoutevr2: ReadWrite<u32, Extoutevr2::Register>,
    _padding424: [u8; 8],
    /// Context ID Comparator Value Register
    pub cidcvr: ReadWrite<u32>,
    _padding436: [u8; 8],
    /// Context ID Comparator Mask Register
    pub cidcmr: ReadWrite<u32>,
    _padding448: [u8; 32],
    /// Synchronization Frequency Regsiter
    pub syncfr: ReadWrite<u32, Syncfr::Register>,
    /// ETM ID Register
    pub idr: ReadOnly<u32, Idr::Register>,
    /// Configuration Code Extension Register
    pub ccer: ReadOnly<u32, Ccer::Register>,
    /// Extended External Input Selection Register
    pub extinselr: ReadWrite<u32, Extinselr::Register>,
    _padding496: [u8; 16],
    /// CoreSight Trace ID Register
    pub traceidr: ReadWrite<u32, Traceidr::Register>,
    _padding516: [u8; 272],
    /// Power-Down Status Register
    pub pdsr: ReadWrite<u32, Pdsr::Register>,
    _padding792: [u8; 3008],
    /// Processor-ETM Interface Register
    pub etmif: ReadOnly<u32, Etmif::Register>,
    /// Miscellaneous Outputs Register
    pub miscout: ReadWrite<u32, Miscout::Register>,
    /// Miscellaneous Inputs Register
    pub miscin: ReadOnly<u32, Miscin::Register>,
    /// Trigger Acknowledge Regsiter
    pub triggerack: ReadOnly<u32, Triggerack::Register>,
    /// Trigger Request Regsiter
    pub triggerreq: ReadWrite<u32, Triggerreq::Register>,
    /// ATB Data Register 0
    pub atbdata0: ReadWrite<u32, Atbdata0::Register>,
    /// ATB Control Register 2
    pub atbctr2: ReadOnly<u32, Atbctr2::Register>,
    /// ATB Control Register 1
    pub atbctr1: ReadWrite<u32, Atbctr1::Register>,
    /// ATB Control Register 0
    pub atbctr0: ReadWrite<u32, Atbctr0::Register>,
    _padding3836: [u8; 4],
    /// Integration Mode Control Register
    pub itctrl: ReadWrite<u32, Itctrl::Register>,
    _padding3844: [u8; 156],
    /// Claim Tag Set Register
    pub claimset: ReadWrite<u32, Claimset::Register>,
    /// Claim Tag Clear Register
    pub claimclr: ReadWrite<u32, Claimclr::Register>,
    _padding4008: [u8; 8],
    /// Lock Access Register
    pub lar: WriteOnly<u32>,
    /// Lock Status Register
    pub lsr: ReadOnly<u32, Lsr::Register>,
    /// Authentication Status Register
    pub authstatus: ReadOnly<u32, Authstatus::Register>,
    _padding4028: [u8; 12],
    /// Device Indentifier
    pub devid: ReadOnly<u32>,
    /// Device Type Register
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// Peripheral ID Register 4
    pub pidr4: ReadOnly<u32, Pidr4::Register>,
    /// Peripheral ID Register 5
    pub pidr5: ReadOnly<u32>,
    /// Peripheral ID Register 6
    pub pidr6: ReadOnly<u32>,
    /// Peripheral ID Register 7
    pub pidr7: ReadOnly<u32>,
    /// Peripheral ID Register 0
    pub pidr0: ReadOnly<u32, Pidr0::Register>,
    /// Peripheral ID Register 1
    pub pidr1: ReadOnly<u32, Pidr1::Register>,
    /// Peripheral ID Register 2
    pub pidr2: ReadOnly<u32, Pidr2::Register>,
    /// Peripheral ID Register 3
    pub pidr3: ReadOnly<u32, Pidr3::Register>,
    /// Component ID Register 0
    pub cidr0: ReadOnly<u32, Cidr0::Register>,
    /// Component ID Register 1
    pub cidr1: ReadOnly<u32, Cidr1::Register>,
    /// Component ID Register 2
    pub cidr2: ReadOnly<u32, Cidr2::Register>,
    /// Component ID Register 3
    pub cidr3: ReadOnly<u32, Cidr3::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Cr [
        CORE_SELECT OFFSET(25) NUMBITS(3) [],
        PORT_SIZE_3 OFFSET(21) NUMBITS(1) [],
        DATA_ONLY OFFSET(20) NUMBITS(1) [],
        FILTER OFFSET(19) NUMBITS(1) [],
        SUPPRESS_DATA OFFSET(18) NUMBITS(1) [],
        PORT_MODE_1_0 OFFSET(16) NUMBITS(2) [],
        CONTEXT_ID_SIZE OFFSET(14) NUMBITS(2) [],
        PORT_MODE_2 OFFSET(13) NUMBITS(1) [],
        CYCLE_ACCURATE OFFSET(12) NUMBITS(1) [],
        PROGRAMMING OFFSET(10) NUMBITS(1) [],
        DEBUG_REQUEST OFFSET(9) NUMBITS(1) [],
        BRANCH_OUTPUT OFFSET(8) NUMBITS(1) [],
        PORT_SIZE_2_0 OFFSET(4) NUMBITS(3) [],
        DATA_ACCESS OFFSET(2) NUMBITS(2) [],
        MONITORCPRT OFFSET(1) NUMBITS(1) [],
        POWER_DOWN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ccr [
        IDR_PRESENT OFFSET(31) NUMBITS(1) [],
        SW_ACCESS OFFSET(27) NUMBITS(1) [],
        TRACE_PRESENT OFFSET(26) NUMBITS(1) [],
        NO_OF_CIDCMP OFFSET(24) NUMBITS(2) [],
        FIFOFULL_PRESENT OFFSET(23) NUMBITS(1) [],
        NO_OF_EXTOUT OFFSET(20) NUMBITS(2) [],
        NO_OF_EXTIN OFFSET(17) NUMBITS(3) [],
        SEQUENCER_PRESENT OFFSET(16) NUMBITS(1) [],
        NO_OF_COUNTERS OFFSET(13) NUMBITS(3) [],
        NO_OF_MMAPS OFFSET(8) NUMBITS(5) [],
        NO_OF_DATACMP OFFSET(4) NUMBITS(4) [],
        NO_OF_ADDRCMPPAIR OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Trigger [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SrR [
        TRIGGER OFFSET(3) NUMBITS(1) [],
        STATUS OFFSET(2) NUMBITS(1) [],
        PROGRAMMING OFFSET(1) NUMBITS(1) [],
        OVERFLOW OFFSET(0) NUMBITS(1) [],
    ],
    pub SrW [
        TRIGGER OFFSET(3) NUMBITS(1) [],
        STATUS OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Scr [
        NO_FETCH_CMP OFFSET(17) NUMBITS(1) [],
        NO_OF_SUP_CORES OFFSET(12) NUMBITS(3) [],
        PORT_MODE OFFSET(11) NUMBITS(1) [],
        PORT_SIZE OFFSET(10) NUMBITS(1) [],
        MAX_PORT_SIZE_3 OFFSET(9) NUMBITS(1) [],
        FIFOFULL_SUPPORT OFFSET(8) NUMBITS(1) [],
        FULL_RATE OFFSET(4) NUMBITS(1) [],
        HALF_RATE OFFSET(3) NUMBITS(1) [],
        MAX_PORT_SIZE_2_0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Tsscr [
        STOP_SELECT OFFSET(16) NUMBITS(16) [],
        START_SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Tecr2 [
        ADDRCMP_SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Teevr [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Tecr1 [
        START_STOP_ENABLE OFFSET(25) NUMBITS(1) [],
        INC_EXC_CONTROL OFFSET(24) NUMBITS(1) [],
        MMAP_SELECT OFFSET(8) NUMBITS(16) [],
        RANGECMP_SELECT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Fflr [
        LEVEL OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vdevr [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vdcr1 [
        EXCLUDE_SELECT OFFSET(16) NUMBITS(16) [],
        INCLUDE_SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vdcr3 [
        EXCLUDE_SELECT OFFSET(16) NUMBITS(16) [],
        INCLUDE_SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Actr1 [
        SECURE_LEVEL OFFSET(10) NUMBITS(2) [],
        CONTEXT_CMP_CTRL OFFSET(8) NUMBITS(2) [],
        EXACT_MATCH OFFSET(7) NUMBITS(1) [],
        DATA_CMP_CTRL OFFSET(5) NUMBITS(2) [],
        CMP_ACCESS_SIZE OFFSET(3) NUMBITS(2) [],
        ACCESS_TYPE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Actr2 [
        SECURE_LEVEL OFFSET(10) NUMBITS(2) [],
        CONTEXT_CMP_CTRL OFFSET(8) NUMBITS(2) [],
        EXACT_MATCH OFFSET(7) NUMBITS(1) [],
        DATA_CMP_CTRL OFFSET(5) NUMBITS(2) [],
        CMP_ACCESS_SIZE OFFSET(3) NUMBITS(2) [],
        ACCESS_TYPE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Actr3 [
        SECURE_LEVEL OFFSET(10) NUMBITS(2) [],
        CONTEXT_CMP_CTRL OFFSET(8) NUMBITS(2) [],
        EXACT_MATCH OFFSET(7) NUMBITS(1) [],
        DATA_CMP_CTRL OFFSET(5) NUMBITS(2) [],
        CMP_ACCESS_SIZE OFFSET(3) NUMBITS(2) [],
        ACCESS_TYPE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Actr4 [
        SECURE_LEVEL OFFSET(10) NUMBITS(2) [],
        CONTEXT_CMP_CTRL OFFSET(8) NUMBITS(2) [],
        EXACT_MATCH OFFSET(7) NUMBITS(1) [],
        DATA_CMP_CTRL OFFSET(5) NUMBITS(2) [],
        CMP_ACCESS_SIZE OFFSET(3) NUMBITS(2) [],
        ACCESS_TYPE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Actr5 [
        SECURE_LEVEL OFFSET(10) NUMBITS(2) [],
        CONTEXT_CMP_CTRL OFFSET(8) NUMBITS(2) [],
        EXACT_MATCH OFFSET(7) NUMBITS(1) [],
        DATA_CMP_CTRL OFFSET(5) NUMBITS(2) [],
        CMP_ACCESS_SIZE OFFSET(3) NUMBITS(2) [],
        ACCESS_TYPE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Actr6 [
        SECURE_LEVEL OFFSET(10) NUMBITS(2) [],
        CONTEXT_CMP_CTRL OFFSET(8) NUMBITS(2) [],
        EXACT_MATCH OFFSET(7) NUMBITS(1) [],
        DATA_CMP_CTRL OFFSET(5) NUMBITS(2) [],
        CMP_ACCESS_SIZE OFFSET(3) NUMBITS(2) [],
        ACCESS_TYPE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Actr7 [
        SECURE_LEVEL OFFSET(10) NUMBITS(2) [],
        CONTEXT_CMP_CTRL OFFSET(8) NUMBITS(2) [],
        EXACT_MATCH OFFSET(7) NUMBITS(1) [],
        DATA_CMP_CTRL OFFSET(5) NUMBITS(2) [],
        CMP_ACCESS_SIZE OFFSET(3) NUMBITS(2) [],
        ACCESS_TYPE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Actr8 [
        SECURE_LEVEL OFFSET(10) NUMBITS(2) [],
        CONTEXT_CMP_CTRL OFFSET(8) NUMBITS(2) [],
        EXACT_MATCH OFFSET(7) NUMBITS(1) [],
        DATA_CMP_CTRL OFFSET(5) NUMBITS(2) [],
        CMP_ACCESS_SIZE OFFSET(3) NUMBITS(2) [],
        ACCESS_TYPE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntrldvr1 [
        INITIAL OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntrldvr2 [
        INITIAL OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntenr1 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntenr2 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntrldevr1 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntrldevr2 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntvr1 [
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntvr2 [
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sqevr1 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sqevr2 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sqevr3 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sqevr4 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sqevr5 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sqevr6 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sqr [
        STATE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Extoutevr1 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Extoutevr2 [
        EVENT OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Syncfr [
        COUNT OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr [
        IMPLEMENTER OFFSET(24) NUMBITS(8) [],
        SECURITY OFFSET(19) NUMBITS(1) [],
        THUMB2 OFFSET(18) NUMBITS(1) [],
        LOAD_PC_FIRST OFFSET(16) NUMBITS(1) [],
        ARM_FAMILY OFFSET(12) NUMBITS(4) [],
        ARCH_MAJOR OFFSET(8) NUMBITS(4) [],
        ARCH_MINOR OFFSET(4) NUMBITS(4) [],
        REVISION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ccer [
        ALL_READABLE OFFSET(11) NUMBITS(1) [],
        SIZE_OF_EEXTIN OFFSET(3) NUMBITS(8) [],
        NO_OF_EEXTIN_SEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Extinselr [
        SEL_2ND_EXTIN OFFSET(8) NUMBITS(6) [],
        SEL_1ST_EXTIN OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Traceidr [
        ID OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pdsr [
        STICKY_REG OFFSET(1) NUMBITS(1) [],
        POWERED_UP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Etmif [
        EVNTBUS_54 OFFSET(13) NUMBITS(1) [],
        EVNTBUS_0 OFFSET(12) NUMBITS(1) [],
        ETMCID_31 OFFSET(11) NUMBITS(1) [],
        ETMCID_0 OFFSET(10) NUMBITS(1) [],
        ETMDD_63 OFFSET(9) NUMBITS(1) [],
        ETMDD_0 OFFSET(8) NUMBITS(1) [],
        ETMDA_31 OFFSET(7) NUMBITS(1) [],
        ETMDA_0 OFFSET(6) NUMBITS(1) [],
        ETMDCTL_11 OFFSET(5) NUMBITS(1) [],
        ETMDCTL_0 OFFSET(4) NUMBITS(1) [],
        ETMIA_31 OFFSET(3) NUMBITS(1) [],
        ETMIA_1 OFFSET(2) NUMBITS(1) [],
        ETMICTL_13 OFFSET(1) NUMBITS(1) [],
        ETMICTL_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Miscout [
        EXTOUT OFFSET(8) NUMBITS(2) [],
        ETMWFIREADY OFFSET(5) NUMBITS(1) [],
        ETMDBGRQ OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Miscin [
        ETMWFIPENDING OFFSET(5) NUMBITS(1) [],
        DBGACK OFFSET(4) NUMBITS(1) [],
        EXTIN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Triggerack [
        TRIGGERACK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Triggerreq [
        TRIGGER OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Atbdata0 [
        ATDATA OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Atbctr2 [
        AFVALID OFFSET(1) NUMBITS(1) [],
        ATREADY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Atbctr1 [
        ATID OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Atbctr0 [
        ATBYTES OFFSET(8) NUMBITS(2) [],
        AFREADY OFFSET(1) NUMBITS(1) [],
        ATVALID OFFSET(0) NUMBITS(1) [],
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
    pub Claimset [
        CLAIM OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Claimclr [
        CLAIM OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Lsr [
        TT OFFSET(2) NUMBITS(1) [],
        SLK OFFSET(1) NUMBITS(1) [],
        SLI OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Authstatus [
        SNID_IMP OFFSET(7) NUMBITS(1) [],
        SNID_EN OFFSET(6) NUMBITS(1) [],
        SID_IMP OFFSET(5) NUMBITS(1) [],
        SID_EN OFFSET(4) NUMBITS(1) [],
        NSNID_IMP OFFSET(3) NUMBITS(1) [],
        NSNID_EN OFFSET(2) NUMBITS(1) [],
        NSID_IMP OFFSET(1) NUMBITS(1) [],
        NSID_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devtype [
        SUBTYPE OFFSET(4) NUMBITS(4) [],
        MAIN_CLASS OFFSET(0) NUMBITS(4) [],
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

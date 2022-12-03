// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// System Trace Macrocell, System Trace Macrocell with multiple SW and HW stimulus ports for MIPI STPv2 traces
pub static mut CORESIGHT_SOC_STM: *mut Registers = 0xfe9c0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 3076],
    /// Start DMA Transfer.
    pub dmastartr: WriteOnly<u32, Dmastartr::Register>,
    /// Stop DMA Transfer.
    pub dmastopr: WriteOnly<u32, Dmastopr::Register>,
    /// DMA Transfer Status.
    pub dmastatr: ReadOnly<u32, Dmastatr::Register>,
    /// Controls the DMA transfer request mechanism.
    pub dmactlr: ReadWrite<u32, Dmactlr::Register>,
    _padding3092: [u8; 232],
    /// DMA features of the STM (read-only).
    pub dmaidr: ReadOnly<u32, Dmaidr::Register>,
    /// Enable Hardware Events for Trace
    pub heer: ReadWrite<u32>,
    _padding3332: [u8; 28],
    /// Enable Trigger Generation on Hardware Events.
    pub heter: ReadWrite<u32>,
    _padding3364: [u8; 60],
    /// Select the Hardware Event bank.
    pub hebsr: ReadWrite<u32, Hebsr::Register>,
    /// Control the primary functions of Hardware Event tracing.
    pub hemcr: Aliased<u32, HemcrR::Register, HemcrW::Register>,
    /// Control hardware event multiplexors external to STM.
    pub heextmuxr: ReadWrite<u32, Heextmuxr::Register>,
    _padding3436: [u8; 136],
    /// Master Number in Event Trace
    pub hemastr: ReadOnly<u32, Hemastr::Register>,
    /// Read the features of the STM.
    pub hefeat1r: ReadOnly<u32, Hefeat1r::Register>,
    /// Read the features of hardware event tracing in STM.
    pub heidr: ReadOnly<u32, Heidr::Register>,
    /// Enable Stimulus Registers to Generate Trace.
    pub sper: ReadWrite<u32>,
    _padding3588: [u8; 28],
    /// Enable Trigger Generation on writes to enabled stimulus port registers.
    pub spter: ReadWrite<u32>,
    _padding3620: [u8; 60],
    /// Enable a debugger to program which stimulus ports the STMSPER and STMSPTER apply to.
    pub spscr: ReadWrite<u32, Spscr::Register>,
    /// Enable a debugger to program which masters the STMSPSCR applies to.
    pub spmscr: ReadWrite<u32, Spmscr::Register>,
    /// Enable a debugger to override various features of the STM.
    pub spoverrider: ReadWrite<u32, Spoverrider::Register>,
    /// Enables a debugger to choose which masters the STMSPOVERRIDERR applies to.
    pub spmoverrider: ReadWrite<u32, Spmoverrider::Register>,
    /// Control the STM triggers caused by STMSPTER.
    pub sptrigcsr: Aliased<u32, SptrigcsrR::Register, SptrigcsrW::Register>,
    _padding3700: [u8; 12],
    /// Controls the STM settings.
    pub tcsr: Aliased<u32, TcsrR::Register, TcsrW::Register>,
    /// Force Timestamp Output.
    pub tsstimr: WriteOnly<u32, Tsstimr::Register>,
    _padding3720: [u8; 4],
    /// Timestamp Counter Frequency.
    pub tsfreqr: ReadWrite<u32>,
    /// Interval Between Synchronization Packets.
    pub syncr: ReadWrite<u32, Syncr::Register>,
    /// Implementation Defined STM controls.
    pub auxcr: ReadWrite<u32, Auxcr::Register>,
    _padding3736: [u8; 8],
    /// Read the features of the STM.
    pub feat1r: ReadOnly<u32, Feat1r::Register>,
    /// Read the features of the STM.
    pub feat2r: ReadOnly<u32, Feat2r::Register>,
    /// Indicates the features of the STM.
    pub feat3r: ReadOnly<u32, Feat3r::Register>,
    _padding3756: [u8; 60],
    /// Integration Test for Cross-Trigger Outputs.
    pub ittrigger: WriteOnly<u32, Ittrigger::Register>,
    /// Control the value of the ATDATAM outputs in integration mode.
    pub itatbdata0: WriteOnly<u32, Itatbdata0::Register>,
    /// Returnvalue of the ATREADYM and AFVALIDM inputs in integration mode.
    pub itatbctr2: ReadOnly<u32, Itatbctr2::Register>,
    /// Control value of the ATIDM output in integration mode.
    pub itatbid: WriteOnly<u32, Itatbid::Register>,
    /// Control value of the ATVALIDM, AFREADYM, and ATBYTESM outputs in integration mode.
    pub itatbctr0: WriteOnly<u32, Itatbctr0::Register>,
    _padding3836: [u8; 4],
    /// Enable Topology Detection.
    pub itctrl: ReadWrite<u32, Itctrl::Register>,
    _padding3844: [u8; 156],
    /// Claim TagSet.
    pub claimset: WriteOnly<u32, Claimset::Register>,
    /// Claim Tag Clear.
    pub claimclr: WriteOnly<u32, Claimclr::Register>,
    _padding4008: [u8; 8],
    /// Enables write access to device registers.
    pub lar: WriteOnly<u32>,
    /// Status of Lock Control Mechanism.
    pub lsr: ReadOnly<u32, Lsr::Register>,
    /// Reports the required security level and current status of the authentication interface.
    pub authstatus: ReadOnly<u32, Authstatus::Register>,
    /// Indicates the architect and architecture of the STM. For the STM-500, the architect is Arm, and the architecture is STMv1.1
    pub devarch: ReadOnly<u32, Devarch::Register>,
    _padding4032: [u8; 8],
    /// Indicates the capabilities of the CoreSight STM.
    pub devid: ReadOnly<u32, Devid::Register>,
    /// Type Classification.
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// PID - Designer Identity and Memory Footprint.
    pub pidr4: ReadOnly<u32, Pidr4::Register>,
    /// Reserved
    pub pidr5: ReadOnly<u32>,
    /// Reserved
    pub pidr6: ReadOnly<u32>,
    /// Reserved
    pub pidr7: ReadOnly<u32>,
    /// PID - Designer Part Number
    pub pidr0: ReadOnly<u32, Pidr0::Register>,
    /// PID - Part Number and Designer Identify.
    pub pidr1: ReadOnly<u32, Pidr1::Register>,
    /// PID - Design Identity and Product Revision.
    pub pidr2: ReadOnly<u32, Pidr2::Register>,
    /// PID - RevAnd and Customer-modified Bit Fields.
    pub pidr3: ReadOnly<u32, Pidr3::Register>,
    /// CID - Indentification Registers Present.
    pub cidr0: ReadOnly<u32, Cidr0::Register>,
    /// CID - Indentification Registers Present and Component Class.
    pub cidr1: ReadOnly<u32, Cidr1::Register>,
    /// CID - Indentification Registers Present.
    pub cidr2: ReadOnly<u32, Cidr2::Register>,
    /// CID - Indentification Registers Present.
    pub cidr3: ReadOnly<u32, Cidr3::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Dmastartr [
        START OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dmastopr [
        STOP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dmastatr [
        STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dmactlr [
        SENS OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dmaidr [
        VENDSPEC OFFSET(8) NUMBITS(4) [],
        CLASSREV OFFSET(4) NUMBITS(4) [],
        CLASS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Hebsr [
        HEBS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub HemcrR [
        ATBTRIGEN OFFSET(7) NUMBITS(1) [],
        TRIGSTATUS OFFSET(5) NUMBITS(1) [],
        TRIGCTL OFFSET(4) NUMBITS(1) [],
        ERRDETECT OFFSET(2) NUMBITS(1) [],
        COMPEN OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub HemcrW [
        ATBTRIGEN OFFSET(7) NUMBITS(1) [],
        TRIGCLEAR OFFSET(6) NUMBITS(1) [],
        TRIGCTL OFFSET(4) NUMBITS(1) [],
        COMPEN OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Heextmuxr [
        EXTMUX OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Hemastr [
        MASTER OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Hefeat1r [
        HEEXTMUXSIZE OFFSET(28) NUMBITS(3) [],
        NUMHE OFFSET(15) NUMBITS(9) [],
        HECOMP OFFSET(4) NUMBITS(2) [],
        HEMASTR OFFSET(3) NUMBITS(1) [],
        HEERR OFFSET(2) NUMBITS(1) [],
        HETER OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Heidr [
        VENDSPEC OFFSET(8) NUMBITS(4) [],
        CLASSREV OFFSET(4) NUMBITS(4) [],
        CLASS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Spscr [
        PORTSEL OFFSET(20) NUMBITS(12) [],
        PORTCTL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Spmscr [
        MASTSEL OFFSET(15) NUMBITS(8) [],
        MASTCTL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Spoverrider [
        PORTSEL OFFSET(15) NUMBITS(17) [],
        OVERTS OFFSET(2) NUMBITS(1) [],
        OVERCTL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Spmoverrider [
        MASTSEL OFFSET(15) NUMBITS(8) [],
        MASTCTL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SptrigcsrR [
        ATBTRIGEN_DIR OFFSET(4) NUMBITS(1) [],
        ATBTRIGEN_TE OFFSET(3) NUMBITS(1) [],
        TRIGSTATUS OFFSET(1) NUMBITS(1) [],
        TRIGCTL OFFSET(0) NUMBITS(1) [],
    ],
    pub SptrigcsrW [
        ATBTRIGEN_DIR OFFSET(4) NUMBITS(1) [],
        ATBTRIGEN_TE OFFSET(3) NUMBITS(1) [],
        TRIGCLEAR OFFSET(2) NUMBITS(1) [],
        TRIGCTL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TcsrR [
        BUSY OFFSET(23) NUMBITS(1) [],
        TRACEID OFFSET(16) NUMBITS(7) [],
        COMPEN OFFSET(5) NUMBITS(1) [],
        SYNCEN OFFSET(2) NUMBITS(1) [],
        TSEN OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub TcsrW [
        TRACEID OFFSET(16) NUMBITS(7) [],
        COMPEN OFFSET(5) NUMBITS(1) [],
        TSEN OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Tsstimr [
        FORCETS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Syncr [
        MODE OFFSET(12) NUMBITS(1) [],
        COUNT OFFSET(3) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Auxcr [
        QHWEVOVERRIDE OFFSET(7) NUMBITS(1) [],
        PRIORINVDIS OFFSET(2) NUMBITS(1) [],
        ASYNCPE OFFSET(1) NUMBITS(1) [],
        FIFOAF OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Feat1r [
        SWOEN OFFSET(22) NUMBITS(2) [],
        SYNCEN OFFSET(20) NUMBITS(2) [],
        HWTEN OFFSET(18) NUMBITS(2) [],
        TSPRESCALE OFFSET(16) NUMBITS(2) [],
        TRIGCTL OFFSET(14) NUMBITS(2) [],
        TRACEBUS OFFSET(10) NUMBITS(4) [],
        SYNC OFFSET(8) NUMBITS(2) [],
        FORCETS OFFSET(7) NUMBITS(1) [],
        TSFREQ OFFSET(6) NUMBITS(1) [],
        TS OFFSET(4) NUMBITS(2) [],
        PROT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Feat2r [
        SPTYPE OFFSET(16) NUMBITS(2) [],
        DSIZE OFFSET(12) NUMBITS(4) [],
        SPTRTYPE OFFSET(9) NUMBITS(2) [],
        PRIVMASK OFFSET(7) NUMBITS(2) [],
        SPOVERRIDE OFFSET(6) NUMBITS(1) [],
        SPCOMP OFFSET(4) NUMBITS(2) [],
        SPER OFFSET(2) NUMBITS(1) [],
        SPTER OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Feat3r [
        NUMMAST OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ittrigger [
        ASYNCOUT_W OFFSET(3) NUMBITS(1) [],
        TRIGOUTHETE_W OFFSET(2) NUMBITS(1) [],
        TRIGOUTSW_W OFFSET(1) NUMBITS(1) [],
        TRIGOUTSPTE_W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbdata0 [
        ATDATAM63_W OFFSET(8) NUMBITS(1) [],
        ATDATAM55_W OFFSET(7) NUMBITS(1) [],
        ATDATAM47_W OFFSET(6) NUMBITS(1) [],
        ATDATAM39_W OFFSET(5) NUMBITS(1) [],
        ATDATAM31_W OFFSET(4) NUMBITS(1) [],
        ATDATAM23_W OFFSET(3) NUMBITS(1) [],
        ATDATAM15_W OFFSET(2) NUMBITS(1) [],
        ATDATAM7_W OFFSET(1) NUMBITS(1) [],
        ATDATAM0_W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbctr2 [
        AFVALIDM_R OFFSET(1) NUMBITS(1) [],
        ATREADYM_R OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbid [
        ATIDM_W OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbctr0 [
        ATBYTESM_W OFFSET(8) NUMBITS(3) [],
        AFREADYM_W OFFSET(1) NUMBITS(1) [],
        ATVALIDM_W OFFSET(0) NUMBITS(1) [],
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
        SET_W OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Claimclr [
        CLR_W OFFSET(0) NUMBITS(4) [],
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
    pub Devarch [
        ARCHITECT OFFSET(21) NUMBITS(11) [],
        PRESENT OFFSET(20) NUMBITS(1) [],
        REVISION OFFSET(16) NUMBITS(4) [],
        ARCHID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devid [
        NUMSP OFFSET(0) NUMBITS(17) [],
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

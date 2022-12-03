// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// APU 1 Embedded Trace Macrocell, A53 Embedded Trace Macrocell
pub static mut CORESIGHT_A53_ETM_1: *mut Registers = 0xfed40000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 4],
    /// Programming Control Register
    pub prgctlr: ReadWrite<u32, Prgctlr::Register>,
    _padding8: [u8; 4],
    /// Status Register
    pub statr: ReadOnly<u32, Statr::Register>,
    /// Trace Configuration Register
    pub configr: ReadWrite<u32, Configr::Register>,
    _padding20: [u8; 4],
    /// Auxiliary Control Register
    pub auxctlr: ReadWrite<u32, Auxctlr::Register>,
    _padding28: [u8; 4],
    /// Event Control 0 Register
    pub eventctl0r: ReadWrite<u32, Eventctl0r::Register>,
    /// Event Control 1 Register
    pub eventctl1r: ReadWrite<u32, Eventctl1r::Register>,
    _padding40: [u8; 4],
    /// Stall Control Register
    pub stallctlr: ReadWrite<u32, Stallctlr::Register>,
    /// Global Timestamp Control Register
    pub tsctlr: ReadWrite<u32, Tsctlr::Register>,
    /// Synchronization Period Register
    pub syncpr: ReadWrite<u32, Syncpr::Register>,
    /// Cycle Count Control Register
    pub ccctlr: ReadWrite<u32, Ccctlr::Register>,
    /// Branch Broadcast Control Register
    pub bbctlr: ReadWrite<u32, Bbctlr::Register>,
    /// Trace ID Register
    pub traceidr: ReadWrite<u32>,
    _padding68: [u8; 60],
    /// ViewInst Main Control Register
    pub victlr: ReadWrite<u32, Victlr::Register>,
    /// ViewInst Include-Exclude Control Register
    pub viiectlr: ReadWrite<u32, Viiectlr::Register>,
    /// ViewInst Start-Stop Control Register
    pub vissctlr: ReadWrite<u32, Vissctlr::Register>,
    _padding140: [u8; 116],
    /// Sequencer State Transition Control Registers 0
    pub seqevr0: ReadWrite<u32, Seqevr0::Register>,
    /// Sequencer State Transition Control Registers 1
    pub seqevr1: ReadWrite<u32, Seqevr1::Register>,
    /// Sequencer State Transition Control Registers 2
    pub seqevr2: ReadWrite<u32, Seqevr2::Register>,
    _padding268: [u8; 12],
    /// Sequencer Reset Control Register
    pub seqrstevr: ReadWrite<u32, Seqrstevr::Register>,
    /// Sequencer State Register
    pub seqstr: ReadWrite<u32, Seqstr::Register>,
    /// External Input Select Register
    pub extinselr: ReadWrite<u32, Extinselr::Register>,
    _padding292: [u8; 28],
    /// Counter Reload Value Registers 0
    pub cntrldvr0: ReadWrite<u32, Cntrldvr0::Register>,
    /// Counter Reload Value Registers 1
    pub cntrldvr1: ReadWrite<u32, Cntrldvr1::Register>,
    _padding328: [u8; 8],
    /// Counter Control Register 0
    pub cntctlr0: ReadWrite<u32, Cntctlr0::Register>,
    /// Counter Control Register 1
    pub cntctlr1: ReadWrite<u32, Cntctlr1::Register>,
    _padding344: [u8; 8],
    /// Counter Value Registers 0
    pub cntvr0: ReadWrite<u32, Cntvr0::Register>,
    /// Counter Value Registers 1
    pub cntvr1: ReadWrite<u32, Cntvr1::Register>,
    _padding360: [u8; 24],
    /// ID Register 8
    pub idr8: ReadOnly<u32>,
    /// ID Register 9
    pub idr9: ReadOnly<u32>,
    /// ID Register 10
    pub idr10: ReadOnly<u32>,
    /// ID Register 11
    pub idr11: ReadOnly<u32>,
    /// ID Register 12
    pub idr12: ReadOnly<u32>,
    /// ID Register 13
    pub idr13: ReadOnly<u32>,
    _padding408: [u8; 40],
    /// Implementation Specific Register 0
    pub imspec0: ReadWrite<u32, Imspec0::Register>,
    _padding452: [u8; 28],
    /// ID Register 0
    pub idr0: ReadOnly<u32, Idr0::Register>,
    /// ID Register 1
    pub idr1: ReadOnly<u32, Idr1::Register>,
    /// ID Register 2
    pub idr2: ReadOnly<u32, Idr2::Register>,
    /// ID Register 3
    pub idr3: ReadOnly<u32, Idr3::Register>,
    /// ID Register 4
    pub idr4: ReadOnly<u32, Idr4::Register>,
    /// ID Register 5
    pub idr5: ReadOnly<u32, Idr5::Register>,
    _padding504: [u8; 16],
    /// Resource Selection Control Registers 2
    pub rsctlr2: ReadWrite<u32, Rsctlr2::Register>,
    /// Resource Selection Control Registers 3
    pub rsctlr3: ReadWrite<u32, Rsctlr3::Register>,
    /// Resource Selection Control Registers 4
    pub rsctlr4: ReadWrite<u32, Rsctlr4::Register>,
    /// Resource Selection Control Registers 5
    pub rsctlr5: ReadWrite<u32, Rsctlr5::Register>,
    /// Resource Selection Control Registers 6
    pub rsctlr6: ReadWrite<u32, Rsctlr6::Register>,
    /// Resource Selection Control Registers 7
    pub rsctlr7: ReadWrite<u32, Rsctlr7::Register>,
    /// Resource Selection Control Registers 8
    pub rsctlr8: ReadWrite<u32, Rsctlr8::Register>,
    /// Resource Selection Control Registers 9
    pub rsctlr9: ReadWrite<u32, Rsctlr9::Register>,
    /// Resource Selection Control Registers 10
    pub rsctlr10: ReadWrite<u32, Rsctlr10::Register>,
    /// Resource Selection Control Registers 11
    pub rsctlr11: ReadWrite<u32, Rsctlr11::Register>,
    /// Resource Selection Control Registers 12
    pub rsctlr12: ReadWrite<u32, Rsctlr12::Register>,
    /// Resource Selection Control Registers 13
    pub rsctlr13: ReadWrite<u32, Rsctlr13::Register>,
    /// Resource Selection Control Registers 14
    pub rsctlr14: ReadWrite<u32, Rsctlr14::Register>,
    /// Resource Selection Control Registers 15
    pub rsctlr15: ReadWrite<u32, Rsctlr15::Register>,
    _padding576: [u8; 64],
    /// Single-Shot Comparator Control Register 0
    pub ssccr0: ReadWrite<u32, Ssccr0::Register>,
    _padding644: [u8; 28],
    /// Single-Shot Comparator Status Register 0
    pub sscsr0: Aliased<u32, Sscsr0R::Register, Sscsr0W::Register>,
    _padding676: [u8; 92],
    /// OS Lock Access Register
    pub oslar: WriteOnly<u32, Oslar::Register>,
    /// OS Lock Status Register
    pub oslsr: ReadOnly<u32, Oslsr::Register>,
    _padding776: [u8; 8],
    /// Power Down Control Register
    pub pdcr: ReadWrite<u32, Pdcr::Register>,
    /// Power Down Status Register
    pub pdsr: ReadOnly<u32, Pdsr::Register>,
    _padding792: [u8; 232],
    /// Address Comparator Value Registers 0
    pub acvrn0_31to0: ReadWrite<u32>,
    /// Address Comparator Value Registers 0
    pub acvrn0_63to32: ReadWrite<u32>,
    /// Address Comparator Value Registers 1
    pub acvrn1_31to0: ReadWrite<u32>,
    /// Address Comparator Value Registers 1
    pub acvrn1_63to32: ReadWrite<u32>,
    /// Address Comparator Value Registers 2
    pub acvrn2_31to0: ReadWrite<u32>,
    /// Address Comparator Value Registers 2
    pub acvrn2_63to32: ReadWrite<u32>,
    /// Address Comparator Value Registers 3
    pub acvrn3_31to0: ReadWrite<u32>,
    /// Address Comparator Value Registers 3
    pub acvrn3_63to32: ReadWrite<u32>,
    /// Address Comparator Value Registers 4
    pub acvrn4_31to0: ReadWrite<u32>,
    /// Address Comparator Value Registers 4
    pub acvrn4_63to32: ReadWrite<u32>,
    /// Address Comparator Value Registers 5
    pub acvrn5_31to0: ReadWrite<u32>,
    /// Address Comparator Value Registers 5
    pub acvrn5_63to32: ReadWrite<u32>,
    /// Address Comparator Value Registers 6
    pub acvrn6_31to0: ReadWrite<u32>,
    /// Address Comparator Value Registers 6
    pub acvrn6_63to32: ReadWrite<u32>,
    /// Address Comparator Value Registers 7
    pub acvrn7_31to0: ReadWrite<u32>,
    /// Address Comparator Value Registers 7
    pub acvrn7_63to32: ReadWrite<u32>,
    _padding1088: [u8; 64],
    /// Address Comparator Access Type Registers 0-7
    pub acatrn0: ReadWrite<u32, Acatrn0::Register>,
    _padding1156: [u8; 4],
    /// Address Comparator Access Type Registers 0-7
    pub acatrn1: ReadWrite<u32, Acatrn1::Register>,
    _padding1164: [u8; 4],
    /// Address Comparator Access Type Registers 0-7
    pub acatrn2: ReadWrite<u32, Acatrn2::Register>,
    _padding1172: [u8; 4],
    /// Address Comparator Access Type Registers 0-7
    pub acatrn3: ReadWrite<u32, Acatrn3::Register>,
    _padding1180: [u8; 4],
    /// Address Comparator Access Type Registers 0-7
    pub acatrn4: ReadWrite<u32, Acatrn4::Register>,
    _padding1188: [u8; 4],
    /// Address Comparator Access Type Registers 0-7
    pub acatrn5: ReadWrite<u32, Acatrn5::Register>,
    _padding1196: [u8; 4],
    /// Address Comparator Access Type Registers 0-7
    pub acatrn6: ReadWrite<u32, Acatrn6::Register>,
    _padding1204: [u8; 4],
    /// Address Comparator Access Type Registers 0-7
    pub acatrn7: ReadWrite<u32, Acatrn7::Register>,
    _padding1212: [u8; 324],
    /// Context ID Comparator Value Register 0
    pub cidcvr0: ReadWrite<u32>,
    _padding1540: [u8; 60],
    /// VMID Comparator Value Register 0
    pub vmidcvr0: ReadWrite<u32>,
    _padding1604: [u8; 60],
    /// Context ID Comparator Control Register 0
    pub cidcctlr0: ReadWrite<u32>,
    _padding1668: [u8; 2144],
    /// Integration ATB Identification Register
    pub itatbidr: ReadWrite<u32, Itatbidr::Register>,
    _padding3816: [u8; 4],
    /// Integration Instruction ATB Data Register
    pub itidatar: WriteOnly<u32, Itidatar::Register>,
    _padding3824: [u8; 4],
    /// Integration Instruction ATB In Register
    pub itiatbinr: ReadOnly<u32, Itiatbinr::Register>,
    _padding3832: [u8; 4],
    /// Integration Instruction ATB Out Register
    pub itiatboutr: WriteOnly<u32, Itiatboutr::Register>,
    /// Integration Mode Control Register
    pub itctrl: ReadWrite<u32, Itctrl::Register>,
    _padding3844: [u8; 156],
    /// Claim Tag Set Register
    pub claimset: ReadWrite<u32, Claimset::Register>,
    /// Claim Tag Clear Register
    pub claimclr: ReadWrite<u32, Claimclr::Register>,
    /// Device Affinity Register 0
    pub devaff0: ReadOnly<u32>,
    /// Device Affinity Register 1
    pub devaff1: ReadOnly<u32>,
    /// Software Lock Access Register
    pub lar: WriteOnly<u32>,
    /// Software Lock Status Register
    pub lsr: ReadOnly<u32, Lsr::Register>,
    /// Authentication Status Register
    pub authstatus: ReadOnly<u32, Authstatus::Register>,
    /// Device Architecture Register
    pub devarch: ReadOnly<u32, Devarch::Register>,
    _padding4032: [u8; 8],
    /// Device ID Register
    pub devid: ReadOnly<u32>,
    /// Device Type Register
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// Peripheral Identification Register 4
    pub pidr4: ReadOnly<u32, Pidr4::Register>,
    /// Peripheral Identification Register 5-7
    pub pidr5: ReadOnly<u32>,
    /// Peripheral Identification Register 6-7
    pub pidr6: ReadOnly<u32>,
    /// Peripheral Identification Register 7-7
    pub pidr7: ReadOnly<u32>,
    /// Peripheral Identification Register 0
    pub pidr0: ReadOnly<u32, Pidr0::Register>,
    /// Peripheral Identification Register 1
    pub pidr1: ReadOnly<u32, Pidr1::Register>,
    /// Peripheral Identification Register 2
    pub pidr2: ReadOnly<u32, Pidr2::Register>,
    /// Peripheral Identification Register 3
    pub pidr3: ReadOnly<u32, Pidr3::Register>,
    /// Component Identification Register 0
    pub cidr0: ReadOnly<u32, Cidr0::Register>,
    /// Component Identification Register 1
    pub cidr1: ReadOnly<u32, Cidr1::Register>,
    /// Component Identification Register 2
    pub cidr2: ReadOnly<u32, Cidr2::Register>,
    /// Component Identification Register 3
    pub cidr3: ReadOnly<u32, Cidr3::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Prgctlr [
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Statr [
        PMSTABLE OFFSET(1) NUMBITS(1) [],
        IDLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Configr [
        DV OFFSET(17) NUMBITS(1) [],
        DA OFFSET(16) NUMBITS(1) [],
        QE OFFSET(13) NUMBITS(2) [],
        RS OFFSET(12) NUMBITS(1) [],
        TS OFFSET(11) NUMBITS(1) [],
        COND OFFSET(8) NUMBITS(3) [],
        VMID OFFSET(7) NUMBITS(1) [],
        CID OFFSET(6) NUMBITS(1) [],
        CCI OFFSET(4) NUMBITS(1) [],
        BB OFFSET(3) NUMBITS(1) [],
        INSTP0 OFFSET(1) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Auxctlr [
        COREIFEN OFFSET(7) NUMBITS(1) [],
        AUTHNOFLUSH OFFSET(5) NUMBITS(1) [],
        TSNODELAY OFFSET(4) NUMBITS(1) [],
        SYNCDELAY OFFSET(3) NUMBITS(1) [],
        OVFLW OFFSET(2) NUMBITS(1) [],
        IDLEACK OFFSET(1) NUMBITS(1) [],
        AFREADY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eventctl0r [
        TYPE3 OFFSET(31) NUMBITS(1) [],
        SEL3 OFFSET(24) NUMBITS(4) [],
        TYPE2 OFFSET(23) NUMBITS(1) [],
        SEL2 OFFSET(16) NUMBITS(4) [],
        TYPE1 OFFSET(15) NUMBITS(1) [],
        SEL1 OFFSET(8) NUMBITS(4) [],
        TYPE0 OFFSET(7) NUMBITS(1) [],
        SEL0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eventctl1r [
        LPOVERRIDE OFFSET(12) NUMBITS(1) [],
        ATB OFFSET(11) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Stallctlr [
        ISTALL OFFSET(8) NUMBITS(1) [],
        LEVEL OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Tsctlr [
        EVENT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Syncpr [
        PERIOD OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ccctlr [
        THRESHOLD OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bbctlr [
        MODE OFFSET(8) NUMBITS(1) [],
        RANGE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Victlr [
        EXLEVEL_NS OFFSET(20) NUMBITS(4) [],
        EXLEVEL_S OFFSET(16) NUMBITS(4) [],
        TRCERR OFFSET(11) NUMBITS(1) [],
        TRCRESET OFFSET(10) NUMBITS(1) [],
        SSSTATUS OFFSET(9) NUMBITS(1) [],
        TYPE OFFSET(7) NUMBITS(1) [],
        SEL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Viiectlr [
        EXCLUDE OFFSET(16) NUMBITS(8) [],
        INCLUDE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vissctlr [
        STOP OFFSET(16) NUMBITS(16) [],
        START OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Seqevr0 [
        B OFFSET(8) NUMBITS(8) [],
        F OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Seqevr1 [
        B OFFSET(8) NUMBITS(8) [],
        F OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Seqevr2 [
        B OFFSET(8) NUMBITS(8) [],
        F OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Seqrstevr [
        RST OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Seqstr [
        STATE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Extinselr [
        SEL3 OFFSET(24) NUMBITS(5) [],
        SEL2 OFFSET(16) NUMBITS(5) [],
        SEL1 OFFSET(8) NUMBITS(5) [],
        SEL0 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntrldvr0 [
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntrldvr1 [
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntctlr0 [
        RLDSELF OFFSET(16) NUMBITS(1) [],
        RLDTYPE OFFSET(15) NUMBITS(1) [],
        RLDSEL OFFSET(8) NUMBITS(4) [],
        CNTTYPE OFFSET(7) NUMBITS(1) [],
        CNTSEL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntctlr1 [
        CNTCHAIN OFFSET(17) NUMBITS(1) [],
        RLDSELF OFFSET(16) NUMBITS(1) [],
        RLDEVENT OFFSET(8) NUMBITS(8) [],
        CNTEVENT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cntvr0 [
        VALUE OFFSET(0) NUMBITS(16) [],
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
    pub Imspec0 [
        SUPPORT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr0 [
        COMMOPT OFFSET(29) NUMBITS(1) [],
        TSSIZE OFFSET(24) NUMBITS(5) [],
        QSUPP OFFSET(15) NUMBITS(2) [],
        QFILT OFFSET(14) NUMBITS(1) [],
        CONDTYPE OFFSET(12) NUMBITS(2) [],
        NUMEVENT OFFSET(10) NUMBITS(2) [],
        RETSTACK OFFSET(9) NUMBITS(1) [],
        TRCCCI OFFSET(7) NUMBITS(1) [],
        TRCCOND OFFSET(6) NUMBITS(1) [],
        TRCBB OFFSET(5) NUMBITS(1) [],
        TRCDATA OFFSET(3) NUMBITS(2) [],
        INSTP0 OFFSET(1) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr1 [
        DESIGNER OFFSET(24) NUMBITS(8) [],
        TRCARCHMAJ OFFSET(8) NUMBITS(4) [],
        TRCARCHMIN OFFSET(4) NUMBITS(4) [],
        REVISION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr2 [
        CCSIZE OFFSET(25) NUMBITS(4) [],
        DVSIZE OFFSET(20) NUMBITS(5) [],
        DASIZE OFFSET(15) NUMBITS(5) [],
        VMIDSIZE OFFSET(10) NUMBITS(5) [],
        CIDSIZE OFFSET(5) NUMBITS(5) [],
        IASIZE OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr3 [
        NOOVERFLOW OFFSET(31) NUMBITS(1) [],
        NUMPROC OFFSET(28) NUMBITS(3) [],
        SYSSTALL OFFSET(27) NUMBITS(1) [],
        STALLCTL OFFSET(26) NUMBITS(1) [],
        SYNCPR OFFSET(25) NUMBITS(1) [],
        TRCERR OFFSET(24) NUMBITS(1) [],
        EXLEVEL_NS OFFSET(20) NUMBITS(4) [],
        EXLEVEL_S OFFSET(16) NUMBITS(4) [],
        CCITMIN OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr4 [
        NUMVMIDC OFFSET(28) NUMBITS(4) [],
        NUMCIDC OFFSET(24) NUMBITS(4) [],
        NUMSSCC OFFSET(20) NUMBITS(4) [],
        NUMRSPAIR OFFSET(16) NUMBITS(4) [],
        NUMPC OFFSET(12) NUMBITS(4) [],
        SUPPDAC OFFSET(8) NUMBITS(1) [],
        NUMDVC OFFSET(4) NUMBITS(4) [],
        NUMACPAIRS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr5 [
        REDFUNCNTR OFFSET(31) NUMBITS(1) [],
        NUMCNTR OFFSET(28) NUMBITS(3) [],
        NUMSEQSTATE OFFSET(25) NUMBITS(3) [],
        LPOVERRIDE OFFSET(23) NUMBITS(1) [],
        ATBTRIG OFFSET(22) NUMBITS(1) [],
        TRACEIDSIZE OFFSET(16) NUMBITS(6) [],
        NUMEXTINSEL OFFSET(9) NUMBITS(3) [],
        NUMEXTIN OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr2 [
        PAIRINV OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr3 [
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr4 [
        PAIRINV OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr5 [
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr6 [
        PAIRINV OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr7 [
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr8 [
        PAIRINV OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr9 [
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr10 [
        PAIRINV OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr11 [
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr12 [
        PAIRINV OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr13 [
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr14 [
        PAIRINV OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rsctlr15 [
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        INV OFFSET(20) NUMBITS(1) [],
        GROUP OFFSET(16) NUMBITS(4) [],
        SELECT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ssccr0 [
        RST OFFSET(24) NUMBITS(1) [],
        ARC OFFSET(16) NUMBITS(4) [],
        SAC OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sscsr0R [
        STATUS OFFSET(31) NUMBITS(1) [],
        DV OFFSET(2) NUMBITS(1) [],
        DA OFFSET(1) NUMBITS(1) [],
        INST OFFSET(0) NUMBITS(1) [],
    ],
    pub Sscsr0W [
        STATUS OFFSET(31) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Oslar [
        LOCK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Oslsr [
        PRESENT OFFSET(3) NUMBITS(1) [],
        BIT32 OFFSET(2) NUMBITS(1) [],
        LOCKED OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pdcr [
        PU OFFSET(3) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pdsr [
        LOCKED OFFSET(5) NUMBITS(1) [],
        STICKYPD OFFSET(1) NUMBITS(1) [],
        POWER OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acatrn0 [
        DTBM OFFSET(21) NUMBITS(1) [],
        DATARANGE OFFSET(20) NUMBITS(1) [],
        DATASIZE OFFSET(18) NUMBITS(2) [],
        DATAMATCH OFFSET(16) NUMBITS(2) [],
        EXLEVEL_NS OFFSET(12) NUMBITS(4) [],
        EXLEVEL_S OFFSET(8) NUMBITS(4) [],
        CONTEXT OFFSET(4) NUMBITS(3) [],
        CONTEXTTYPE OFFSET(2) NUMBITS(2) [],
        TYPE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acatrn1 [
        DTBM OFFSET(21) NUMBITS(1) [],
        DATARANGE OFFSET(20) NUMBITS(1) [],
        DATASIZE OFFSET(18) NUMBITS(2) [],
        DATAMATCH OFFSET(16) NUMBITS(2) [],
        EXLEVEL_NS OFFSET(12) NUMBITS(4) [],
        EXLEVEL_S OFFSET(8) NUMBITS(4) [],
        CONTEXT OFFSET(4) NUMBITS(3) [],
        CONTEXTTYPE OFFSET(2) NUMBITS(2) [],
        TYPE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acatrn2 [
        DTBM OFFSET(21) NUMBITS(1) [],
        DATARANGE OFFSET(20) NUMBITS(1) [],
        DATASIZE OFFSET(18) NUMBITS(2) [],
        DATAMATCH OFFSET(16) NUMBITS(2) [],
        EXLEVEL_NS OFFSET(12) NUMBITS(4) [],
        EXLEVEL_S OFFSET(8) NUMBITS(4) [],
        CONTEXT OFFSET(4) NUMBITS(3) [],
        CONTEXTTYPE OFFSET(2) NUMBITS(2) [],
        TYPE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acatrn3 [
        DTBM OFFSET(21) NUMBITS(1) [],
        DATARANGE OFFSET(20) NUMBITS(1) [],
        DATASIZE OFFSET(18) NUMBITS(2) [],
        DATAMATCH OFFSET(16) NUMBITS(2) [],
        EXLEVEL_NS OFFSET(12) NUMBITS(4) [],
        EXLEVEL_S OFFSET(8) NUMBITS(4) [],
        CONTEXT OFFSET(4) NUMBITS(3) [],
        CONTEXTTYPE OFFSET(2) NUMBITS(2) [],
        TYPE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acatrn4 [
        DTBM OFFSET(21) NUMBITS(1) [],
        DATARANGE OFFSET(20) NUMBITS(1) [],
        DATASIZE OFFSET(18) NUMBITS(2) [],
        DATAMATCH OFFSET(16) NUMBITS(2) [],
        EXLEVEL_NS OFFSET(12) NUMBITS(4) [],
        EXLEVEL_S OFFSET(8) NUMBITS(4) [],
        CONTEXT OFFSET(4) NUMBITS(3) [],
        CONTEXTTYPE OFFSET(2) NUMBITS(2) [],
        TYPE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acatrn5 [
        DTBM OFFSET(21) NUMBITS(1) [],
        DATARANGE OFFSET(20) NUMBITS(1) [],
        DATASIZE OFFSET(18) NUMBITS(2) [],
        DATAMATCH OFFSET(16) NUMBITS(2) [],
        EXLEVEL_NS OFFSET(12) NUMBITS(4) [],
        EXLEVEL_S OFFSET(8) NUMBITS(4) [],
        CONTEXT OFFSET(4) NUMBITS(3) [],
        CONTEXTTYPE OFFSET(2) NUMBITS(2) [],
        TYPE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acatrn6 [
        DTBM OFFSET(21) NUMBITS(1) [],
        DATARANGE OFFSET(20) NUMBITS(1) [],
        DATASIZE OFFSET(18) NUMBITS(2) [],
        DATAMATCH OFFSET(16) NUMBITS(2) [],
        EXLEVEL_NS OFFSET(12) NUMBITS(4) [],
        EXLEVEL_S OFFSET(8) NUMBITS(4) [],
        CONTEXT OFFSET(4) NUMBITS(3) [],
        CONTEXTTYPE OFFSET(2) NUMBITS(2) [],
        TYPE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acatrn7 [
        DTBM OFFSET(21) NUMBITS(1) [],
        DATARANGE OFFSET(20) NUMBITS(1) [],
        DATASIZE OFFSET(18) NUMBITS(2) [],
        DATAMATCH OFFSET(16) NUMBITS(2) [],
        EXLEVEL_NS OFFSET(12) NUMBITS(4) [],
        EXLEVEL_S OFFSET(8) NUMBITS(4) [],
        CONTEXT OFFSET(4) NUMBITS(3) [],
        CONTEXTTYPE OFFSET(2) NUMBITS(2) [],
        TYPE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbidr [
        ID OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itidatar [
        ATDATAM_31 OFFSET(4) NUMBITS(1) [],
        ATDATAM_23 OFFSET(3) NUMBITS(1) [],
        ATDATAM_15 OFFSET(2) NUMBITS(1) [],
        ATDATAM_7 OFFSET(1) NUMBITS(1) [],
        ATDATAM_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itiatbinr [
        AFVALIDM OFFSET(1) NUMBITS(1) [],
        ATREADYM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itiatboutr [
        BYTES OFFSET(8) NUMBITS(2) [],
        AFREADY OFFSET(1) NUMBITS(1) [],
        ATVALID OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itctrl [
        ITEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Claimset [
        SET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Claimclr [
        CLR OFFSET(0) NUMBITS(8) [],
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
        ARCHID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devtype [
        SUB OFFSET(4) NUMBITS(4) [],
        MAIN OFFSET(0) NUMBITS(4) [],
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

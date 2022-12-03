// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// APU 0 built-in debug logic
pub static mut CORESIGHT_A53_DBG_0: *mut Registers = 0xfec10000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 32],
    /// External Debug Event Status Register
    pub edesr: ReadWrite<u32, Edesr::Register>,
    /// External Debug Execution Control Register
    pub edecr: ReadWrite<u32, Edecr::Register>,
    _padding40: [u8; 8],
    /// External Debug Watchpoint Address Register (low word)
    pub edwar_31to0: ReadOnly<u32>,
    /// External Debug Watchpoint Address Register (high word)
    pub edwar_63to32: ReadOnly<u32>,
    _padding56: [u8; 72],
    /// Debug Data Transfer Register Receive
    pub dbgdtrrx_el0: ReadWrite<u32>,
    /// External Debug Instruction Transfer Register
    pub editr: WriteOnly<u32, Editr::Register>,
    /// External Debug Status and Control Register
    pub edscr: ReadWrite<u32, Edscr::Register>,
    /// Debug Data Transfer Register Transmit
    pub dbgdtrtx_el0: ReadWrite<u32>,
    /// External Debug Reserve Control Register
    pub edrcr: WriteOnly<u32, Edrcr::Register>,
    _padding148: [u8; 4],
    /// External Debug Exception Catch Control Register
    pub edeccr: ReadWrite<u32, Edeccr::Register>,
    _padding156: [u8; 4],
    /// External Debug Program Counter Sample Register (low word)
    pub edpcsr_31to0: ReadOnly<u32>,
    /// External Debug Context ID Sample Register
    pub edcidsr: ReadOnly<u32>,
    /// External Debug Virtual Context Sample Register
    pub edvidsr: ReadOnly<u32, Edvidsr::Register>,
    /// External Debug Program Counter Sample Register (high word)
    pub edpcsr_63to32: ReadOnly<u32>,
    _padding176: [u8; 592],
    /// OS Lock Access Register
    pub oslar_el1: WriteOnly<u32, OslarEl1::Register>,
    _padding772: [u8; 12],
    /// External Debug Power/Reset Control Register
    pub edprcr: ReadWrite<u32, Edprcr::Register>,
    /// External Debug Processor Status Register
    pub edprsr: ReadOnly<u32, Edprsr::Register>,
    _padding792: [u8; 232],
    /// Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr0_el1_31to0: ReadWrite<u32, Dbgbvr0El131to0::Register>,
    /// Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr0_el1_63to32: ReadWrite<u32, Dbgbvr0El163to32::Register>,
    /// Debug Breakpoint Control Registers
    pub dbgbcr0_el1: ReadWrite<u32, Dbgbcr0El1::Register>,
    _padding1036: [u8; 4],
    /// Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr1_el1_31to0: ReadWrite<u32, Dbgbvr1El131to0::Register>,
    /// Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr1_el1_63to32: ReadWrite<u32, Dbgbvr1El163to32::Register>,
    /// Debug Breakpoint Control Registers
    pub dbgbcr1_el1: ReadWrite<u32, Dbgbcr1El1::Register>,
    _padding1052: [u8; 4],
    /// Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr2_el1_31to0: ReadWrite<u32, Dbgbvr2El131to0::Register>,
    /// Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr2_el1_63to32: ReadWrite<u32, Dbgbvr2El163to32::Register>,
    /// Debug Breakpoint Control Registers
    pub dbgbcr2_el1: ReadWrite<u32, Dbgbcr2El1::Register>,
    _padding1068: [u8; 4],
    /// Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr3_el1_31to0: ReadWrite<u32, Dbgbvr3El131to0::Register>,
    /// Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr3_el1_63to32: ReadWrite<u32, Dbgbvr3El163to32::Register>,
    /// Debug Breakpoint Control Registers
    pub dbgbcr3_el1: ReadWrite<u32, Dbgbcr3El1::Register>,
    _padding1084: [u8; 4],
    /// Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr4_el1_31to0: ReadWrite<u32, Dbgbvr4El131to0::Register>,
    /// Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr4_el1_63to32: ReadWrite<u32, Dbgbvr4El163to32::Register>,
    /// Debug Breakpoint Control Registers
    pub dbgbcr4_el1: ReadWrite<u32, Dbgbcr4El1::Register>,
    _padding1100: [u8; 4],
    /// Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr5_el1_31to0: ReadWrite<u32, Dbgbvr5El131to0::Register>,
    /// Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR<n>_EL1 where n is 0 to 15. Multiple uses of this register refer to Armv8
    pub dbgbvr5_el1_63to32: ReadWrite<u32, Dbgbvr5El163to32::Register>,
    /// Debug Breakpoint Control Registers
    pub dbgbcr5_el1: ReadWrite<u32, Dbgbcr5El1::Register>,
    _padding1116: [u8; 932],
    /// Debug Watchpoint Value Registers
    pub dbgwvr0_el1_31to0: ReadWrite<u32, Dbgwvr0El131to0::Register>,
    /// Debug Watchpoint Extended Value Registers
    pub dbgwvr0_el1_63to32: ReadWrite<u32, Dbgwvr0El163to32::Register>,
    /// Debug Watchpoint Control Registers
    pub dbgwcr0_el1: ReadWrite<u32, Dbgwcr0El1::Register>,
    _padding2060: [u8; 4],
    /// Debug Watchpoint Value Registers
    pub dbgwvr1_el1_31to0: ReadWrite<u32, Dbgwvr1El131to0::Register>,
    /// Debug Watchpoint Extended Value Registers
    pub dbgwvr1_el1_63to32: ReadWrite<u32, Dbgwvr1El163to32::Register>,
    /// Debug Watchpoint Control Registers
    pub dbgwcr1_el1: ReadWrite<u32, Dbgwcr1El1::Register>,
    _padding2076: [u8; 4],
    /// Debug Watchpoint Value Registers
    pub dbgwvr2_el1_31to0: ReadWrite<u32, Dbgwvr2El131to0::Register>,
    /// Debug Watchpoint Extended Value Registers
    pub dbgwvr2_el1_63to32: ReadWrite<u32, Dbgwvr2El163to32::Register>,
    /// Debug Watchpoint Control Registers
    pub dbgwcr2_el1: ReadWrite<u32, Dbgwcr2El1::Register>,
    _padding2092: [u8; 4],
    /// Debug Watchpoint Value Registers
    pub dbgwvr3_el1_31to0: ReadWrite<u32, Dbgwvr3El131to0::Register>,
    /// Debug Watchpoint Extended Value Registers
    pub dbgwvr3_el1_63to32: ReadWrite<u32, Dbgwvr3El163to32::Register>,
    /// Debug Watchpoint Control Registers
    pub dbgwcr3_el1: ReadWrite<u32, Dbgwcr3El1::Register>,
    _padding2108: [u8; 4],
    /// Debug Watchpoint Value Registers
    pub dbgwvr4_el1_31to0: ReadWrite<u32, Dbgwvr4El131to0::Register>,
    /// Debug Watchpoint Extended Value Registers
    pub dbgwvr4_el1_63to32: ReadWrite<u32, Dbgwvr4El163to32::Register>,
    /// Debug Watchpoint Control Registers
    pub dbgwcr4_el1: ReadWrite<u32, Dbgwcr4El1::Register>,
    _padding2124: [u8; 4],
    /// Debug Watchpoint Value Registers
    pub dbgwvr5_el1_31to0: ReadWrite<u32, Dbgwvr5El131to0::Register>,
    /// Debug Watchpoint Extended Value Registers
    pub dbgwvr5_el1_63to32: ReadWrite<u32, Dbgwvr5El163to32::Register>,
    /// Debug Watchpoint Control Registers
    pub dbgwcr5_el1: ReadWrite<u32, Dbgwcr5El1::Register>,
    _padding2140: [u8; 1188],
    /// Main ID Register
    pub midr_el1: ReadOnly<u32, MidrEl1::Register>,
    _padding3332: [u8; 28],
    /// Processor Feature Register 0 (low word)
    pub id_aa64pfr0_el1_31to0: ReadOnly<u32, IdAa64pfr0El131to0::Register>,
    /// Processor Feature Register 0 (high word)
    pub id_aa64pfr0_el1_63to32: ReadOnly<u32>,
    /// Debug Feature Register 0 (low word)
    pub id_aa64dfr0_el1_31to0: ReadOnly<u32, IdAa64dfr0El131to0::Register>,
    /// Debug Feature Register 0 (high word)
    pub id_aa64dfr0_el1_63to32: ReadOnly<u32>,
    /// Instruction Set Attribute Register 0 (low word)
    pub id_aa64isar0_el1_31to0: ReadOnly<u32, IdAa64isar0El131to0::Register>,
    /// Instruction Set Attribute Register 0 (high word)
    pub id_aa64isar0_el1_63to32: ReadOnly<u32>,
    /// Memory Model Feature Register 0 (low word)
    pub id_aa64mmfr0_el1_31to0: ReadOnly<u32, IdAa64mmfr0El131to0::Register>,
    /// Memory Model Feature Register 0 (high word)
    pub id_aa64mmfr0_el1_63to32: ReadOnly<u32>,
    /// Processor Feature Register 1 (low word)
    pub id_aa64pfr1_el1_31to0: ReadOnly<u32>,
    /// Processor Feature Register 1 (high word)
    pub id_aa64pfr1_el1_63to32: ReadOnly<u32>,
    /// Auxiliary Feature Register 1 (low word)
    pub id_aa64dfr1_el1_31to0: ReadOnly<u32>,
    /// Auxiliary Feature Register 1 (high word)
    pub id_aa64dfr1_el1_63to32: ReadOnly<u32>,
    /// Instruction Set Attribute Register 1 (low word)
    pub id_aa64isar1_el1_31to0: ReadOnly<u32>,
    /// Instruction Set Attribute Register 1 (high word)
    pub id_aa64isar1_el1_63to32: ReadOnly<u32>,
    /// Memory Model Feature Register 1 (low word)
    pub id_aa64mmfr1_el1_31to0: ReadOnly<u32>,
    /// Memory Model Feature Register 1 (high word)
    pub id_aa64mmfr1_el1_63to32: ReadOnly<u32>,
    _padding3424: [u8; 416],
    /// External Debug Integration mode Control Register
    pub itctrl: ReadWrite<u32, Itctrl::Register>,
    _padding3844: [u8; 156],
    /// Debug Claim Tag Set Register
    pub claimset: ReadWrite<u32, Claimset::Register>,
    /// Debug Claim Tag Clear Register
    pub claimclr: ReadWrite<u32, Claimclr::Register>,
    /// External Debug Device Affinity Register 0
    pub devaff0: ReadOnly<u32>,
    /// External Debug Device Affinity Register 1
    pub devaff1: ReadOnly<u32>,
    /// External Debug Lock Access Register
    pub lar: WriteOnly<u32>,
    /// External Debug Lock Status Register
    pub lsr: ReadOnly<u32, Lsr::Register>,
    /// Debug Authentication Status register
    pub authstatus: ReadOnly<u32, Authstatus::Register>,
    /// External Debug Device Architecture Register
    pub devarch: ReadOnly<u32, Devarch::Register>,
    /// External Debug Device ID Register 2
    pub devid2: ReadOnly<u32>,
    /// External Debug Device ID Register 1
    pub devid1: ReadOnly<u32, Devid1::Register>,
    /// External Debug Device ID Register 0
    pub devid: ReadOnly<u32, Devid::Register>,
    /// External Debug Device Type Register
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// External Debug Peripheral Identification Register 4
    pub pidr4: ReadOnly<u32, Pidr4::Register>,
    /// CTI Peripheral Identification Register 5
    pub pidr5: ReadOnly<u32>,
    /// CTI Peripheral Identification Register 6
    pub pidr6: ReadOnly<u32>,
    /// CTI Peripheral Identification Register 7
    pub pidr7: ReadOnly<u32>,
    /// External Debug Peripheral Identification Register 0
    pub pidr0: ReadOnly<u32, Pidr0::Register>,
    /// External Debug Peripheral Identification Register 1
    pub pidr1: ReadOnly<u32, Pidr1::Register>,
    /// External Debug Peripheral Identification Register 2
    pub pidr2: ReadOnly<u32, Pidr2::Register>,
    /// External Debug Peripheral Identification Register 3
    pub pidr3: ReadOnly<u32, Pidr3::Register>,
    /// External Debug Component Identification Register 0
    pub cidr0: ReadOnly<u32, Cidr0::Register>,
    /// External Debug Component Identification Register 1
    pub cidr1: ReadOnly<u32, Cidr1::Register>,
    /// External Debug Component Identification Register 2
    pub cidr2: ReadOnly<u32, Cidr2::Register>,
    /// External Debug Component Identification Register 3
    pub cidr3: ReadOnly<u32, Cidr3::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Edesr [
        SS OFFSET(2) NUMBITS(1) [],
        RC OFFSET(1) NUMBITS(1) [],
        OSUC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Edecr [
        SS OFFSET(2) NUMBITS(1) [],
        RCE OFFSET(1) NUMBITS(1) [],
        OSUCE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Editr [
        T32SECOND OFFSET(16) NUMBITS(16) [],
        EDITR OFFSET(0) NUMBITS(32) [],
        T32FIRST OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Edscr [
        RXFULL OFFSET(30) NUMBITS(1) [],
        TXFULL OFFSET(29) NUMBITS(1) [],
        ITO OFFSET(28) NUMBITS(1) [],
        RXO OFFSET(27) NUMBITS(1) [],
        TXU OFFSET(26) NUMBITS(1) [],
        PIPEADV OFFSET(25) NUMBITS(1) [],
        ITE OFFSET(24) NUMBITS(1) [],
        INTDIS OFFSET(22) NUMBITS(2) [],
        TDA OFFSET(21) NUMBITS(1) [],
        MA OFFSET(20) NUMBITS(1) [],
        NS OFFSET(18) NUMBITS(1) [],
        SDD OFFSET(16) NUMBITS(1) [],
        HDE OFFSET(14) NUMBITS(1) [],
        RW OFFSET(10) NUMBITS(4) [],
        EL OFFSET(8) NUMBITS(2) [],
        A OFFSET(7) NUMBITS(1) [],
        ERR OFFSET(6) NUMBITS(1) [],
        STATUS OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Edrcr [
        CBRRQ OFFSET(4) NUMBITS(1) [],
        CSPA OFFSET(3) NUMBITS(1) [],
        CSE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Edeccr [
        NSE OFFSET(4) NUMBITS(4) [],
        SE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Edvidsr [
        NS OFFSET(31) NUMBITS(1) [],
        E2 OFFSET(30) NUMBITS(1) [],
        E3 OFFSET(29) NUMBITS(1) [],
        HV OFFSET(28) NUMBITS(1) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OslarEl1 [
        OSLK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Edprcr [
        COREPURQ OFFSET(3) NUMBITS(1) [],
        CWRR OFFSET(1) NUMBITS(1) [],
        CORENPDRQ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Edprsr [
        SDR OFFSET(11) NUMBITS(1) [],
        SPMAD OFFSET(10) NUMBITS(1) [],
        EPMAD OFFSET(9) NUMBITS(1) [],
        SDAD OFFSET(8) NUMBITS(1) [],
        EDAD OFFSET(7) NUMBITS(1) [],
        DLK OFFSET(6) NUMBITS(1) [],
        OSLK OFFSET(5) NUMBITS(1) [],
        HALTED OFFSET(4) NUMBITS(1) [],
        SR OFFSET(3) NUMBITS(1) [],
        R OFFSET(2) NUMBITS(1) [],
        SPD OFFSET(1) NUMBITS(1) [],
        PU OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr0El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr0El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbcr0El1 [
        BT OFFSET(20) NUMBITS(4) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(4) [],
        PMC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr1El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr1El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbcr1El1 [
        BT OFFSET(20) NUMBITS(4) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(4) [],
        PMC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr2El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr2El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbcr2El1 [
        BT OFFSET(20) NUMBITS(4) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(4) [],
        PMC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr3El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr3El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbcr3El1 [
        BT OFFSET(20) NUMBITS(4) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(4) [],
        PMC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr4El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr4El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbcr4El1 [
        BT OFFSET(20) NUMBITS(4) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(4) [],
        PMC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr5El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbvr5El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgbcr5El1 [
        BT OFFSET(20) NUMBITS(4) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(4) [],
        PMC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr0El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr0El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwcr0El1 [
        MASK OFFSET(24) NUMBITS(5) [],
        WT OFFSET(20) NUMBITS(1) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(8) [],
        LSC OFFSET(3) NUMBITS(2) [],
        PAC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr1El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr1El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwcr1El1 [
        MASK OFFSET(24) NUMBITS(5) [],
        WT OFFSET(20) NUMBITS(1) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(8) [],
        LSC OFFSET(3) NUMBITS(2) [],
        PAC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr2El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr2El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwcr2El1 [
        MASK OFFSET(24) NUMBITS(5) [],
        WT OFFSET(20) NUMBITS(1) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(8) [],
        LSC OFFSET(3) NUMBITS(2) [],
        PAC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr3El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr3El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwcr3El1 [
        MASK OFFSET(24) NUMBITS(5) [],
        WT OFFSET(20) NUMBITS(1) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(8) [],
        LSC OFFSET(3) NUMBITS(2) [],
        PAC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr4El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr4El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwcr4El1 [
        MASK OFFSET(24) NUMBITS(5) [],
        WT OFFSET(20) NUMBITS(1) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(8) [],
        LSC OFFSET(3) NUMBITS(2) [],
        PAC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr5El131to0 [
        VA OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwvr5El163to32 [
        RESS OFFSET(17) NUMBITS(15) [],
        VA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgwcr5El1 [
        MASK OFFSET(24) NUMBITS(5) [],
        WT OFFSET(20) NUMBITS(1) [],
        LBN OFFSET(16) NUMBITS(4) [],
        SSC OFFSET(14) NUMBITS(2) [],
        HMC OFFSET(13) NUMBITS(1) [],
        BAS OFFSET(5) NUMBITS(8) [],
        LSC OFFSET(3) NUMBITS(2) [],
        PAC OFFSET(1) NUMBITS(2) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MidrEl1 [
        IMPLEMENTER OFFSET(24) NUMBITS(8) [],
        VARIANT OFFSET(20) NUMBITS(4) [],
        ARCHITECTURE OFFSET(16) NUMBITS(4) [],
        PARTNUM OFFSET(4) NUMBITS(12) [],
        REVISION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdAa64pfr0El131to0 [
        GIC OFFSET(24) NUMBITS(4) [],
        ADVSIMD OFFSET(20) NUMBITS(4) [],
        FP OFFSET(16) NUMBITS(4) [],
        EL3 OFFSET(12) NUMBITS(4) [],
        EL2 OFFSET(8) NUMBITS(4) [],
        EL1 OFFSET(4) NUMBITS(4) [],
        EL0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdAa64dfr0El131to0 [
        CTX_CMPS OFFSET(28) NUMBITS(4) [],
        WRPS OFFSET(20) NUMBITS(4) [],
        BRPS OFFSET(12) NUMBITS(4) [],
        PMUVER OFFSET(8) NUMBITS(4) [],
        TRACEVER OFFSET(4) NUMBITS(4) [],
        DEBUGVER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdAa64isar0El131to0 [
        CRC32 OFFSET(16) NUMBITS(4) [],
        SHA2 OFFSET(12) NUMBITS(4) [],
        SHA1 OFFSET(8) NUMBITS(4) [],
        AES OFFSET(4) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdAa64mmfr0El131to0 [
        TGRAN4 OFFSET(28) NUMBITS(4) [],
        TGRAN64 OFFSET(24) NUMBITS(4) [],
        TGRAN16 OFFSET(20) NUMBITS(4) [],
        BIGENDEL0 OFFSET(16) NUMBITS(4) [],
        SNSMEM OFFSET(12) NUMBITS(4) [],
        BIGEND OFFSET(8) NUMBITS(4) [],
        ASIDBITS OFFSET(4) NUMBITS(4) [],
        PARANGE OFFSET(0) NUMBITS(4) [],
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
    pub Devid1 [
        PCSROFFSET OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devid [
        AUXREGS OFFSET(24) NUMBITS(4) [],
        PCSAMPLE OFFSET(0) NUMBITS(4) [],
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

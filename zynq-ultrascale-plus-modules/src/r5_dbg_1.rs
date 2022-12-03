// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// RPU 1 Built-in Debug Logic, R5 Debug Logic
pub static mut CORESIGHT_R5_DBG_1: *mut Registers = 0xfebf2000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Debug ID register
    pub didr: ReadOnly<u32, Didr::Register>,
    _padding4: [u8; 20],
    /// The Watchpoint Fault Address Register
    pub wfar: ReadWrite<u32, Wfar::Register>,
    /// Vector Catch Register
    pub vcr: ReadWrite<u32, Vcr::Register>,
    _padding32: [u8; 8],
    /// Debug State Cache Control Register
    pub dsccr: ReadWrite<u32, Dsccr::Register>,
    _padding44: [u8; 84],
    /// Read Data Transfer Register
    pub dtrrxext: ReadOnly<u32>,
    /// Instruction Transfer Register
    pub itr: WriteOnly<u32>,
    /// Debug Status and Control Register
    pub dscrext: Aliased<u32, DscrextR::Register, DscrextW::Register>,
    /// Write Data Transfer Register
    pub dtrtxext: ReadWrite<u32>,
    /// Debug Run Control Register
    pub drcr: WriteOnly<u32, Drcr::Register>,
    _padding148: [u8; 108],
    /// Breakpoint Value Register 0
    pub bvr0: ReadWrite<u32>,
    /// Breakpoint Value Register 1
    pub bvr1: ReadWrite<u32>,
    /// Breakpoint Value Register 2
    pub bvr2: ReadWrite<u32>,
    /// Breakpoint Value Register 3
    pub bvr3: ReadWrite<u32>,
    /// Breakpoint Value Register 4
    pub bvr4: ReadWrite<u32>,
    /// Breakpoint Value Register 5
    pub bvr5: ReadWrite<u32>,
    /// Breakpoint Value Register 6
    pub bvr6: ReadWrite<u32>,
    /// Breakpoint Value Register 7
    pub bvr7: ReadWrite<u32>,
    _padding288: [u8; 32],
    /// Breakpoint Control Register 0
    pub bcr0: ReadWrite<u32, Bcr0::Register>,
    /// Breakpoint Control Register 1
    pub bcr1: ReadWrite<u32, Bcr1::Register>,
    /// Breakpoint Control Register 2
    pub bcr2: ReadWrite<u32, Bcr2::Register>,
    /// Breakpoint Control Register 3
    pub bcr3: ReadWrite<u32, Bcr3::Register>,
    /// Breakpoint Control Register 4
    pub bcr4: ReadWrite<u32, Bcr4::Register>,
    /// Breakpoint Control Register 5
    pub bcr5: ReadWrite<u32, Bcr5::Register>,
    /// Breakpoint Control Register 6
    pub bcr6: ReadWrite<u32, Bcr6::Register>,
    /// Breakpoint Control Register 7
    pub bcr7: ReadWrite<u32, Bcr7::Register>,
    _padding352: [u8; 32],
    /// Watchpoint Value Register 0
    pub wvr0: ReadWrite<u32, Wvr0::Register>,
    /// Watchpoint Value Register 1
    pub wvr1: ReadWrite<u32, Wvr1::Register>,
    /// Watchpoint Value Register 2
    pub wvr2: ReadWrite<u32, Wvr2::Register>,
    /// Watchpoint Value Register 3
    pub wvr3: ReadWrite<u32, Wvr3::Register>,
    /// Watchpoint Value Register 4
    pub wvr4: ReadWrite<u32, Wvr4::Register>,
    /// Watchpoint Value Register 5
    pub wvr5: ReadWrite<u32, Wvr5::Register>,
    /// Watchpoint Value Register 6
    pub wvr6: ReadWrite<u32, Wvr6::Register>,
    /// Watchpoint Value Register 7
    pub wvr7: ReadWrite<u32, Wvr7::Register>,
    _padding416: [u8; 32],
    /// Watchpoint Control Register 0
    pub wcr0: ReadWrite<u32, Wcr0::Register>,
    /// Watchpoint Control Register 1
    pub wcr1: ReadWrite<u32, Wcr1::Register>,
    /// Watchpoint Control Register 2
    pub wcr2: ReadWrite<u32, Wcr2::Register>,
    /// Watchpoint Control Register 3
    pub wcr3: ReadWrite<u32, Wcr3::Register>,
    /// Watchpoint Control Register 4
    pub wcr4: ReadWrite<u32, Wcr4::Register>,
    /// Watchpoint Control Register 5
    pub wcr5: ReadWrite<u32, Wcr5::Register>,
    /// Watchpoint Control Register 6
    pub wcr6: ReadWrite<u32, Wcr6::Register>,
    /// Watchpoint Control Register 7
    pub wcr7: ReadWrite<u32, Wcr7::Register>,
    _padding480: [u8; 292],
    /// Operating System Lock Status Register
    pub oslsr: ReadOnly<u32>,
    _padding776: [u8; 8],
    /// Device Powerdown and Reset Control Register
    pub prcr: ReadWrite<u32, Prcr::Register>,
    /// Device Powerdown and Reset Status Register
    pub prsr: ReadWrite<u32, Prsr::Register>,
    _padding792: [u8; 2536],
    /// Main ID Register
    pub midr: ReadOnly<u32, Midr::Register>,
    /// Cache Type Register
    pub ctr: ReadOnly<u32, Ctr::Register>,
    /// TCM Type Register
    pub tcmtr: ReadOnly<u32, Tcmtr::Register>,
    _padding3340: [u8; 4],
    /// MPU Type Register
    pub mpuir: ReadOnly<u32, Mpuir::Register>,
    /// Multiprocessor Affinity Register
    pub mpidr: ReadOnly<u32, Mpidr::Register>,
    _padding3352: [u8; 8],
    /// Processor Feature Register 0
    pub id_pfr0: ReadOnly<u32, IdPfr0::Register>,
    /// Processor Feature Register 1
    pub id_pfr1: ReadOnly<u32, IdPfr1::Register>,
    /// Debug Feature Register 0
    pub id_dfr0: ReadOnly<u32, IdDfr0::Register>,
    /// Auxiliary Feature Register 0
    pub id_afr0: ReadOnly<u32>,
    /// Memory Model Feature Register 0
    pub id_mmfr0: ReadOnly<u32, IdMmfr0::Register>,
    /// Memory Model Feature Register 1
    pub id_mmfr1: ReadOnly<u32, IdMmfr1::Register>,
    /// Memory Model Feature Register 2
    pub id_mmfr2: ReadOnly<u32, IdMmfr2::Register>,
    /// Memory Model Feature Register 3
    pub id_mmfr3: ReadOnly<u32, IdMmfr3::Register>,
    /// ISA Feature Register 0
    pub id_isar0: ReadOnly<u32, IdIsar0::Register>,
    /// ISA Feature Register 1
    pub id_isar1: ReadOnly<u32, IdIsar1::Register>,
    /// ISA Feature Register 2
    pub id_isar2: ReadOnly<u32, IdIsar2::Register>,
    /// ISA Feature Register 3
    pub id_isar3: ReadOnly<u32, IdIsar3::Register>,
    /// ISA Feature Register 4
    pub id_isar4: ReadOnly<u32, IdIsar4::Register>,
    /// ISA Feature Register 5
    pub id_isar5: ReadOnly<u32>,
    _padding3416: [u8; 384],
    /// ETM Interface Integration Register
    pub etmif: WriteOnly<u32, Etmif::Register>,
    _padding3804: [u8; 28],
    /// Miscellaneous Outputs Integration Register
    pub miscout: WriteOnly<u32, Miscout::Register>,
    /// Miscellaneous Inputs Integration Register
    pub miscin: ReadOnly<u32, Miscin::Register>,
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
    pub Didr [
        WRP OFFSET(28) NUMBITS(4) [],
        BRP OFFSET(24) NUMBITS(4) [],
        CONTEXT OFFSET(20) NUMBITS(4) [],
        ARCH_VER OFFSET(16) NUMBITS(4) [],
        DEVID_IMP OFFSET(15) NUMBITS(1) [],
        VARIANT OFFSET(4) NUMBITS(4) [],
        REVISION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wfar [
        ADDRESS OFFSET(1) NUMBITS(31) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vcr [
        FIQ OFFSET(7) NUMBITS(1) [],
        IRQ OFFSET(6) NUMBITS(1) [],
        DATA_ABORT OFFSET(4) NUMBITS(1) [],
        PREFETCH_ABORT OFFSET(3) NUMBITS(1) [],
        SVC OFFSET(2) NUMBITS(1) [],
        UNDEFINED OFFSET(1) NUMBITS(1) [],
        RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dsccr [
        NWT OFFSET(2) NUMBITS(1) [],
        NIL OFFSET(1) NUMBITS(1) [],
        NDL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DscrextR [
        RXFULL OFFSET(30) NUMBITS(1) [],
        TXFULL OFFSET(29) NUMBITS(1) [],
        PIPEADV OFFSET(25) NUMBITS(1) [],
        INSTRCOMPL_L OFFSET(24) NUMBITS(1) [],
        EXTDCCMODE OFFSET(20) NUMBITS(2) [],
        ADADISCARD OFFSET(19) NUMBITS(1) [],
        NS OFFSET(18) NUMBITS(1) [],
        SPNIDDIS OFFSET(17) NUMBITS(1) [],
        SPIDDIS OFFSET(16) NUMBITS(1) [],
        MDBGEN OFFSET(15) NUMBITS(1) [],
        HDBGEN OFFSET(14) NUMBITS(1) [],
        ITREN OFFSET(13) NUMBITS(1) [],
        UDCCDIS OFFSET(12) NUMBITS(1) [],
        INTDIS OFFSET(11) NUMBITS(1) [],
        DBGACK OFFSET(10) NUMBITS(1) [],
        UND_I OFFSET(8) NUMBITS(1) [],
        ADABORT_I OFFSET(7) NUMBITS(1) [],
        SDABORT_I OFFSET(6) NUMBITS(1) [],
        MOE OFFSET(2) NUMBITS(4) [],
        RESTARTED OFFSET(1) NUMBITS(1) [],
        HALTED OFFSET(0) NUMBITS(1) [],
    ],
    pub DscrextW [
        EXTDCCMODE OFFSET(20) NUMBITS(2) [],
        MDBGEN OFFSET(15) NUMBITS(1) [],
        HDBGEN OFFSET(14) NUMBITS(1) [],
        ITREN OFFSET(13) NUMBITS(1) [],
        UDCCDIS OFFSET(12) NUMBITS(1) [],
        INTDIS OFFSET(11) NUMBITS(1) [],
        DBGACK OFFSET(10) NUMBITS(1) [],
        MOE OFFSET(2) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Drcr [
        CANCEL_MEMORY_REQUESTS OFFSET(4) NUMBITS(1) [],
        CLEAR_STICKY_PIPELINE_ADVANCE OFFSET(3) NUMBITS(1) [],
        CLEAR_STICKY_EXCEPTIONS OFFSET(2) NUMBITS(1) [],
        RESTART_REQUEST OFFSET(1) NUMBITS(1) [],
        HALT_REQUEST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bcr0 [
        BREAKPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        M OFFSET(20) NUMBITS(3) [],
        LINKED_BRP_NUMBER OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(4) [],
        S OFFSET(1) NUMBITS(2) [],
        B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bcr1 [
        BREAKPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        M OFFSET(20) NUMBITS(3) [],
        LINKED_BRP_NUMBER OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(4) [],
        S OFFSET(1) NUMBITS(2) [],
        B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bcr2 [
        BREAKPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        M OFFSET(20) NUMBITS(3) [],
        LINKED_BRP_NUMBER OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(4) [],
        S OFFSET(1) NUMBITS(2) [],
        B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bcr3 [
        BREAKPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        M OFFSET(20) NUMBITS(3) [],
        LINKED_BRP_NUMBER OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(4) [],
        S OFFSET(1) NUMBITS(2) [],
        B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bcr4 [
        BREAKPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        M OFFSET(20) NUMBITS(3) [],
        LINKED_BRP_NUMBER OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(4) [],
        S OFFSET(1) NUMBITS(2) [],
        B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bcr5 [
        BREAKPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        M OFFSET(20) NUMBITS(3) [],
        LINKED_BRP_NUMBER OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(4) [],
        S OFFSET(1) NUMBITS(2) [],
        B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bcr6 [
        BREAKPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        M OFFSET(20) NUMBITS(3) [],
        LINKED_BRP_NUMBER OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(4) [],
        S OFFSET(1) NUMBITS(2) [],
        B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bcr7 [
        BREAKPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        M OFFSET(20) NUMBITS(3) [],
        LINKED_BRP_NUMBER OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(4) [],
        S OFFSET(1) NUMBITS(2) [],
        B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wvr0 [
        WATCHPOINT_ADDRESS OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wvr1 [
        WATCHPOINT_ADDRESS OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wvr2 [
        WATCHPOINT_ADDRESS OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wvr3 [
        WATCHPOINT_ADDRESS OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wvr4 [
        WATCHPOINT_ADDRESS OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wvr5 [
        WATCHPOINT_ADDRESS OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wvr6 [
        WATCHPOINT_ADDRESS OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wvr7 [
        WATCHPOINT_ADDRESS OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wcr0 [
        WATCHPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        E OFFSET(20) NUMBITS(1) [],
        LINKED_BRP OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(8) [],
        LS OFFSET(3) NUMBITS(2) [],
        S OFFSET(1) NUMBITS(2) [],
        W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wcr1 [
        WATCHPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        E OFFSET(20) NUMBITS(1) [],
        LINKED_BRP OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(8) [],
        LS OFFSET(3) NUMBITS(2) [],
        S OFFSET(1) NUMBITS(2) [],
        W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wcr2 [
        WATCHPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        E OFFSET(20) NUMBITS(1) [],
        LINKED_BRP OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(8) [],
        LS OFFSET(3) NUMBITS(2) [],
        S OFFSET(1) NUMBITS(2) [],
        W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wcr3 [
        WATCHPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        E OFFSET(20) NUMBITS(1) [],
        LINKED_BRP OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(8) [],
        LS OFFSET(3) NUMBITS(2) [],
        S OFFSET(1) NUMBITS(2) [],
        W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wcr4 [
        WATCHPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        E OFFSET(20) NUMBITS(1) [],
        LINKED_BRP OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(8) [],
        LS OFFSET(3) NUMBITS(2) [],
        S OFFSET(1) NUMBITS(2) [],
        W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wcr5 [
        WATCHPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        E OFFSET(20) NUMBITS(1) [],
        LINKED_BRP OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(8) [],
        LS OFFSET(3) NUMBITS(2) [],
        S OFFSET(1) NUMBITS(2) [],
        W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wcr6 [
        WATCHPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        E OFFSET(20) NUMBITS(1) [],
        LINKED_BRP OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(8) [],
        LS OFFSET(3) NUMBITS(2) [],
        S OFFSET(1) NUMBITS(2) [],
        W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Wcr7 [
        WATCHPOINT_ADDRESS_MASK OFFSET(24) NUMBITS(5) [],
        E OFFSET(20) NUMBITS(1) [],
        LINKED_BRP OFFSET(16) NUMBITS(4) [],
        BYTE_ADDRESS_SELECT OFFSET(5) NUMBITS(8) [],
        LS OFFSET(3) NUMBITS(2) [],
        S OFFSET(1) NUMBITS(2) [],
        W OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Prcr [
        HOLD_INTERNAL_RESET OFFSET(2) NUMBITS(1) [],
        FORCE_INTERNAL_RESET OFFSET(1) NUMBITS(1) [],
        NO_POWER_DOWN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Prsr [
        STICKY_RESET_STATUS OFFSET(3) NUMBITS(1) [],
        RESET_STATUS OFFSET(2) NUMBITS(1) [],
        STICKY_POWER_DOWN_STATUS OFFSET(1) NUMBITS(1) [],
        POWER_DOWN_STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Midr [
        IMPLEMENTER OFFSET(24) NUMBITS(8) [],
        VARIANT OFFSET(20) NUMBITS(4) [],
        ARCHITECTUR OFFSET(16) NUMBITS(4) [],
        PRIMARY_PART_NUMBER OFFSET(4) NUMBITS(12) [],
        REVISION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        CWG OFFSET(24) NUMBITS(4) [],
        ERG OFFSET(20) NUMBITS(4) [],
        DMINLINE OFFSET(16) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        RESERVED2 OFFSET(4) NUMBITS(10) [],
        IMINLINE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Tcmtr [
        FORMAT OFFSET(29) NUMBITS(3) [],
        BTCM OFFSET(16) NUMBITS(3) [],
        ATCM OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mpuir [
        DREGION OFFSET(8) NUMBITS(8) [],
        S OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mpidr [
        EXTENSION OFFSET(30) NUMBITS(2) [],
        AFF2 OFFSET(16) NUMBITS(8) [],
        AFF1 OFFSET(8) NUMBITS(8) [],
        AFF0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdPfr0 [
        STATE3 OFFSET(12) NUMBITS(4) [],
        STATE2 OFFSET(8) NUMBITS(4) [],
        STATE1 OFFSET(4) NUMBITS(4) [],
        STATE0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdPfr1 [
        UCTLR_PROG_MODEL OFFSET(8) NUMBITS(4) [],
        SECURITY_EXTENSION OFFSET(4) NUMBITS(4) [],
        ARMV4_PROG_MODEL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdDfr0 [
        UCTLR_DEBUG_MODEL_MMAP OFFSET(20) NUMBITS(4) [],
        TRACE_DEBUG_MODEL_MMAP OFFSET(16) NUMBITS(4) [],
        TRACE_DEBUG_MODEL_CP OFFSET(12) NUMBITS(4) [],
        CORE_DEBUG_MODEL_MMAP OFFSET(8) NUMBITS(4) [],
        SECURE_DEBUG_MODEL OFFSET(4) NUMBITS(4) [],
        CORE_DEBUG_MODEL_CP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdMmfr0 [
        INNERMOST_SHAREABILITY OFFSET(28) NUMBITS(4) [],
        FCSE OFFSET(24) NUMBITS(4) [],
        AUXILIARY_REGISTERS OFFSET(20) NUMBITS(4) [],
        TCM_SUPPORT OFFSET(16) NUMBITS(4) [],
        SHAREABILITY_LEVELS OFFSET(12) NUMBITS(4) [],
        OUTERMOST_SHAREABILITY OFFSET(8) NUMBITS(4) [],
        PMSA OFFSET(4) NUMBITS(4) [],
        VMSA OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdMmfr1 [
        BRANCH_PREDICTOR OFFSET(28) NUMBITS(4) [],
        L1_TEST_CLEAN_OP OFFSET(24) NUMBITS(4) [],
        L1_CACHE_MAINT_OP_UNI OFFSET(20) NUMBITS(4) [],
        L1_CACHE_MAINT_OP_HAR OFFSET(16) NUMBITS(4) [],
        L1_CACHE_LINE_MAINT_OP_SNW_UNI OFFSET(12) NUMBITS(4) [],
        L1_CACHE_LINE_MAINT_OP_SNW_HAR OFFSET(8) NUMBITS(4) [],
        L1_CACHE_LINE_MAINT_OP_MVA_UNI OFFSET(4) NUMBITS(4) [],
        L1_CACHE_LINE_MAINT_OP_MVA_HAR OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdMmfr2 [
        HW_ACCESS_FLAG OFFSET(28) NUMBITS(4) [],
        WFI OFFSET(24) NUMBITS(4) [],
        MEMORY_BARRIER OFFSET(20) NUMBITS(4) [],
        TLB_MAINT_OP_UNI OFFSET(16) NUMBITS(4) [],
        TLB_MAINT_OP_HAR OFFSET(12) NUMBITS(4) [],
        L1_CACHE_MAINT_RANGE_OP_HAR OFFSET(8) NUMBITS(4) [],
        L1_BGND_PREFETCH_CACHE_OP OFFSET(4) NUMBITS(4) [],
        L1_FGND_PREFETCH_CACHE_OP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdMmfr3 [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        MAINT_BROADCAST OFFSET(12) NUMBITS(4) [],
        BRANCH_PREDICTOR_MAINT_OP OFFSET(8) NUMBITS(4) [],
        HIER_CACHE_MAINT_BY_SNW OFFSET(4) NUMBITS(4) [],
        HIER_CACHE_MAINT_BY_MVA OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdIsar0 [
        DIVIDE OFFSET(24) NUMBITS(4) [],
        DEBUG OFFSET(20) NUMBITS(4) [],
        COPROCESSOR OFFSET(16) NUMBITS(4) [],
        COMPARE_AND_BRANCH OFFSET(12) NUMBITS(4) [],
        BITFIELD OFFSET(8) NUMBITS(4) [],
        BIT_COUNTING OFFSET(4) NUMBITS(4) [],
        ATOMIC OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdIsar1 [
        JAZELLE OFFSET(28) NUMBITS(4) [],
        INTERWORKING OFFSET(24) NUMBITS(4) [],
        IMMEDIATE OFFSET(20) NUMBITS(4) [],
        ITE OFFSET(16) NUMBITS(4) [],
        EXTEND OFFSET(12) NUMBITS(4) [],
        EXCEPTION_2 OFFSET(8) NUMBITS(4) [],
        EXCEPTION_1 OFFSET(4) NUMBITS(4) [],
        ENDIAN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdIsar2 [
        REVERSAL OFFSET(28) NUMBITS(4) [],
        PSR OFFSET(24) NUMBITS(4) [],
        UNSIGNED_MULTIPLY OFFSET(20) NUMBITS(4) [],
        SIGNED_MULTIPLY OFFSET(16) NUMBITS(4) [],
        MULTIPLY OFFSET(12) NUMBITS(4) [],
        INTERRUPTIBLE OFFSET(8) NUMBITS(4) [],
        MEMORY_HINT OFFSET(4) NUMBITS(4) [],
        LOAD_STORE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdIsar3 [
        THUMB_EE_EXTENSION OFFSET(28) NUMBITS(4) [],
        TRUE_NOP OFFSET(24) NUMBITS(4) [],
        THUMB_COPY OFFSET(20) NUMBITS(4) [],
        TABLE_BRANCH OFFSET(16) NUMBITS(4) [],
        SYNC_PRIMITIVE OFFSET(12) NUMBITS(4) [],
        SVC OFFSET(8) NUMBITS(4) [],
        SIMD OFFSET(4) NUMBITS(4) [],
        SATURATE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdIsar4 [
        SWP_FRAC OFFSET(28) NUMBITS(4) [],
        PSR_M OFFSET(24) NUMBITS(4) [],
        EXCLUSIVE OFFSET(20) NUMBITS(4) [],
        BARRIER OFFSET(16) NUMBITS(4) [],
        SMC OFFSET(12) NUMBITS(4) [],
        WRITE_BACK OFFSET(8) NUMBITS(4) [],
        WITH_SHIFT OFFSET(4) NUMBITS(4) [],
        UNPRIVILEGED OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Etmif [
        EVNTBUSM_54 OFFSET(13) NUMBITS(1) [],
        EVNTBUSM_0 OFFSET(12) NUMBITS(1) [],
        ETMCIDM_31 OFFSET(11) NUMBITS(1) [],
        ETMCIDM_0 OFFSET(10) NUMBITS(1) [],
        ETMDDM_63 OFFSET(9) NUMBITS(1) [],
        ETMDDM_0 OFFSET(8) NUMBITS(1) [],
        ETMDAM_31 OFFSET(7) NUMBITS(1) [],
        ETMDAM_0 OFFSET(6) NUMBITS(1) [],
        ETMDCTLM_11 OFFSET(5) NUMBITS(1) [],
        ETMDCTLM_0 OFFSET(4) NUMBITS(1) [],
        ETMIAM_31 OFFSET(3) NUMBITS(1) [],
        ETMIAM_1 OFFSET(2) NUMBITS(1) [],
        ETMICTLM_13 OFFSET(1) NUMBITS(1) [],
        ETMICTLM_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Miscout [
        DBGRESTARTEDM OFFSET(9) NUMBITS(1) [],
        DBGTRIGGERM OFFSET(8) NUMBITS(1) [],
        ETMWFIPENDINGM OFFSET(5) NUMBITS(1) [],
        NPMUIRQM OFFSET(4) NUMBITS(1) [],
        COMMTXM OFFSET(2) NUMBITS(1) [],
        COMMRXM OFFSET(1) NUMBITS(1) [],
        DBGACKM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Miscin [
        DBGRESTARTM OFFSET(11) NUMBITS(1) [],
        ETMEXTOUTM OFFSET(8) NUMBITS(2) [],
        NETMWFIREADYM OFFSET(5) NUMBITS(1) [],
        NFIQM OFFSET(2) NUMBITS(1) [],
        NIRQM OFFSET(1) NUMBITS(1) [],
        EDBGRQM OFFSET(0) NUMBITS(1) [],
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

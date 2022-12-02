// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, WriteOnly};
/// PMU MicroBlaze Processor I/O Module, PMU IO Module
pub static mut PMU_IOMODULE: *mut Registers = 0xffd40000 as *mut Registers;
register_structs! {
    pub Registers {
        (0x00000000 => _padding0),
        /// Interrupt Mode Register
        (0x0000000c => pub irq_mode: WriteOnly<u32>),
        /// IOModule Misc Control Register (GPO0)
        (0x00000010 => pub gpo0: WriteOnly<u32, Gpo0::Register>),
        /// PMU to MIO Signals (GPO1)
        (0x00000014 => pub gpo1: Aliased<u32, Gpo1R::Register, Gpo1W::Register>),
        /// PMU Acknowlegements (GPO2)
        (0x00000018 => pub gpo2: Aliased<u32, Gpo2R::Register, Gpo2W::Register>),
        /// PMU to PL Signals (GPO3)
        (0x0000001c => pub gpo3: WriteOnly<u32, Gpo3::Register>),
        /// Fault Tolerance Status Register (GPI0)
        (0x00000020 => pub gpi0: ReadOnly<u32, Gpi0::Register>),
        /// General Purpose Input Register 1
        (0x00000024 => pub gpi1: ReadOnly<u32, Gpi1::Register>),
        /// General Purpose Input Register 2
        (0x00000028 => pub gpi2: ReadOnly<u32, Gpi2::Register>),
        /// General Purpose Input Register 3
        (0x0000002c => pub gpi3: ReadOnly<u32, Gpi3::Register>),
        /// Interrupt Status Register
        (0x00000030 => pub irq_status: ReadOnly<u32, IrqStatus::Register>),
        /// Interrupt Pending Register
        (0x00000034 => pub irq_pending: ReadOnly<u32, IrqPending::Register>),
        /// Interrupt Enable Register
        (0x00000038 => pub irq_enable: Aliased<u32, IrqEnableR::Register, IrqEnableW::Register>),
        /// Interrupt Acknowledge Register
        (0x0000003c => pub irq_ack: Aliased<u32, IrqAckR::Register, IrqAckW::Register>),
        /// PIT0 Preload Register
        (0x00000040 => pub pit0_preload: ReadOnly<u32>),
        /// PIT0 Counter Register
        (0x00000044 => pub pit0_counter: ReadOnly<u32>),
        /// PIT0 Control Register
        (0x00000048 => pub pit0_control: Aliased<u32, Pit0ControlR::Register, Pit0ControlW::Register>),
        (0x0000004c => _padding76),
        /// PIT1 Preload Register
        (0x00000050 => pub pit1_preload: ReadOnly<u32>),
        /// PIT1 Counter Register
        (0x00000054 => pub pit1_counter: ReadOnly<u32>),
        /// PIT1 Control Register
        (0x00000058 => pub pit1_control: Aliased<u32, Pit1ControlR::Register, Pit1ControlW::Register>),
        (0x0000005c => _padding92),
        /// PIT2 Preload Register
        (0x00000060 => pub pit2_preload: ReadOnly<u32>),
        /// PIT2 Counter Register
        (0x00000064 => pub pit2_counter: ReadOnly<u32>),
        /// PIT2 Control Register
        (0x00000068 => pub pit2_control: Aliased<u32, Pit2ControlR::Register, Pit2ControlW::Register>),
        (0x0000006c => _padding108),
        /// PIT3 Preload Register
        (0x00000070 => pub pit3_preload: ReadOnly<u32>),
        /// PIT3 Counter Register
        (0x00000074 => pub pit3_counter: ReadOnly<u32>),
        /// PIT3 Control Register
        (0x00000078 => pub pit3_control: Aliased<u32, Pit3ControlR::Register, Pit3ControlW::Register>),
        (0x0000007c => _padding124),
        /// Instruction Injection Address (IOModule_1.GPO1)
        (0x00001014 => pub instruction_inject_addr: WriteOnly<u32>),
        /// Instruction Injection (IOModule_1.GPO2)
        (0x00001018 => pub instruction_inject: WriteOnly<u32>),
        (0x0000101c => @END),
    }
}
register_bitfields! [
    u32,
    pub Gpo0 [
        MAGIC_WORD_1 OFFSET(24) NUMBITS(8) [],
        MAGIC_WORD_2 OFFSET(16) NUMBITS(8) [],
        FT_INJECT_FAILURE OFFSET(13) NUMBITS(3) [],
        DISABLE_RST_FTSM OFFSET(12) NUMBITS(1) [],
        RST_FTSM OFFSET(11) NUMBITS(1) [],
        CLR_FTSTS OFFSET(10) NUMBITS(1) [],
        RST_ON_SLEEP OFFSET(9) NUMBITS(1) [],
        DISABLE_TRACE_COMP OFFSET(8) NUMBITS(1) [],
        PIT3_PRESCALE OFFSET(7) NUMBITS(1) [],
        PIT2_PRESCALE OFFSET(5) NUMBITS(2) [],
        PIT1_PRESCALE OFFSET(3) NUMBITS(2) [],
        PIT0_PRESCALE OFFSET(1) NUMBITS(2) [],
        DEBUG_REMAP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpo1R [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        RESERVED1 OFFSET(10) NUMBITS(1) [],
        RESERVED2 OFFSET(6) NUMBITS(4) [],
    ],
    pub Gpo1W [
        MIO_5 OFFSET(5) NUMBITS(1) [],
        MIO_4 OFFSET(4) NUMBITS(1) [],
        MIO_3 OFFSET(3) NUMBITS(1) [],
        MIO_2 OFFSET(2) NUMBITS(1) [],
        MIO_1 OFFSET(1) NUMBITS(1) [],
        MIO_0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpo2R [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        RESERVED1 OFFSET(0) NUMBITS(6) [],
    ],
    pub Gpo2W [
        DAP_RPU_WAKE_ACK OFFSET(9) NUMBITS(1) [],
        DAP_FP_WAKE_ACK OFFSET(8) NUMBITS(1) [],
        PS_STATUS OFFSET(7) NUMBITS(1) [],
        PCAP_EN OFFSET(6) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpo3 [
        PL_GPO_31 OFFSET(31) NUMBITS(1) [],
        PL_GPO_30 OFFSET(30) NUMBITS(1) [],
        PL_GPO_29 OFFSET(29) NUMBITS(1) [],
        PL_GPO_28 OFFSET(28) NUMBITS(1) [],
        PL_GPO_27 OFFSET(27) NUMBITS(1) [],
        PL_GPO_26 OFFSET(26) NUMBITS(1) [],
        PL_GPO_25 OFFSET(25) NUMBITS(1) [],
        PL_GPO_24 OFFSET(24) NUMBITS(1) [],
        PL_GPO_23 OFFSET(23) NUMBITS(1) [],
        PL_GPO_22 OFFSET(22) NUMBITS(1) [],
        PL_GPO_21 OFFSET(21) NUMBITS(1) [],
        PL_GPO_20 OFFSET(20) NUMBITS(1) [],
        PL_GPO_19 OFFSET(19) NUMBITS(1) [],
        PL_GPO_18 OFFSET(18) NUMBITS(1) [],
        PL_GPO_17 OFFSET(17) NUMBITS(1) [],
        PL_GPO_16 OFFSET(16) NUMBITS(1) [],
        PL_GPO_15 OFFSET(15) NUMBITS(1) [],
        PL_GPO_14 OFFSET(14) NUMBITS(1) [],
        PL_GPO_13 OFFSET(13) NUMBITS(1) [],
        PL_GPO_12 OFFSET(12) NUMBITS(1) [],
        PL_GPO_11 OFFSET(11) NUMBITS(1) [],
        PL_GPO_10 OFFSET(10) NUMBITS(1) [],
        PL_GPO_9 OFFSET(9) NUMBITS(1) [],
        PL_GPO_8 OFFSET(8) NUMBITS(1) [],
        PL_GPO_7 OFFSET(7) NUMBITS(1) [],
        PL_GPO_6 OFFSET(6) NUMBITS(1) [],
        PL_GPO_5 OFFSET(5) NUMBITS(1) [],
        PL_GPO_4 OFFSET(4) NUMBITS(1) [],
        PL_GPO_3 OFFSET(3) NUMBITS(1) [],
        PL_GPO_2 OFFSET(2) NUMBITS(1) [],
        PL_GPO_1 OFFSET(1) NUMBITS(1) [],
        PL_GPO_0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpi0 [
        RFT_ECC_FATAL_ERR OFFSET(31) NUMBITS(1) [],
        RFT_VOTER_ERR OFFSET(30) NUMBITS(1) [],
        RFT_COMPARE_ERR_23 OFFSET(29) NUMBITS(1) [],
        RFT_COMPARE_ERR_13 OFFSET(28) NUMBITS(1) [],
        RFT_COMPARE_ERR_12 OFFSET(27) NUMBITS(1) [],
        RFT_LS_MISMATCH_23_B OFFSET(26) NUMBITS(1) [],
        RFT_LS_MISMATCH_13_B OFFSET(25) NUMBITS(1) [],
        RFT_LS_MISMATCH_12_B OFFSET(24) NUMBITS(1) [],
        RFT_MISMATCH_STATE OFFSET(23) NUMBITS(1) [],
        RFT_MISMATCH_CPU OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        RFT_SLEEP_RESET OFFSET(19) NUMBITS(1) [],
        RFT_LS_MISMATCH_23_A OFFSET(18) NUMBITS(1) [],
        RFT_LS_MISMATCH_13_A OFFSET(17) NUMBITS(1) [],
        RFT_LS_MISMATCH_12_A OFFSET(16) NUMBITS(1) [],
        NFT_ECC_FATAL_ERR OFFSET(15) NUMBITS(1) [],
        NFT_VOTER_ERR OFFSET(14) NUMBITS(1) [],
        NFT_COMPARE_ERR_23 OFFSET(13) NUMBITS(1) [],
        NFT_COMPARE_ERR_13 OFFSET(12) NUMBITS(1) [],
        NFT_COMPARE_ERR_12 OFFSET(11) NUMBITS(1) [],
        NFT_LS_MISMATCH_23_B OFFSET(10) NUMBITS(1) [],
        NFT_LS_MISMATCH_13_B OFFSET(9) NUMBITS(1) [],
        NFT_LS_MISMATCH_12_B OFFSET(8) NUMBITS(1) [],
        NFT_MISMATCH_STATE OFFSET(7) NUMBITS(1) [],
        NFT_MISMATCH_CPU OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        NFT_SLEEP_RESET OFFSET(3) NUMBITS(1) [],
        NFT_LS_MISMATCH_23_A OFFSET(2) NUMBITS(1) [],
        NFT_LS_MISMATCH_13_A OFFSET(1) NUMBITS(1) [],
        NFT_LS_MISMATCH_12_A OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpi1 [
        APB_AIB_ERROR OFFSET(31) NUMBITS(1) [],
        AXI_AIB_ERROR OFFSET(30) NUMBITS(1) [],
        ERROR_2 OFFSET(29) NUMBITS(1) [],
        ERROR_1 OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        ACPU_3_DBG_PWRUP OFFSET(23) NUMBITS(1) [],
        ACPU_2_DBG_PWRUP OFFSET(22) NUMBITS(1) [],
        ACPU_1_DBG_PWRUP OFFSET(21) NUMBITS(1) [],
        ACPU_0_DBG_PWRUP OFFSET(20) NUMBITS(1) [],
        RESERVED1 OFFSET(17) NUMBITS(3) [],
        FPD_WAKE_GIC_PROXY OFFSET(16) NUMBITS(1) [],
        MIO_WAKE_5 OFFSET(15) NUMBITS(1) [],
        MIO_WAKE_4 OFFSET(14) NUMBITS(1) [],
        MIO_WAKE_3 OFFSET(13) NUMBITS(1) [],
        MIO_WAKE_2 OFFSET(12) NUMBITS(1) [],
        MIO_WAKE_1 OFFSET(11) NUMBITS(1) [],
        MIO_WAKE_0 OFFSET(10) NUMBITS(1) [],
        DAP_RPU_WAKE OFFSET(9) NUMBITS(1) [],
        DAP_FPD_WAKE OFFSET(8) NUMBITS(1) [],
        USB_1_WAKE OFFSET(7) NUMBITS(1) [],
        USB_0_WAKE OFFSET(6) NUMBITS(1) [],
        R5_1_WAKE OFFSET(5) NUMBITS(1) [],
        R5_0_WAKE OFFSET(4) NUMBITS(1) [],
        ACPU_3_WAKE OFFSET(3) NUMBITS(1) [],
        ACPU_2_WAKE OFFSET(2) NUMBITS(1) [],
        ACPU_1_WAKE OFFSET(1) NUMBITS(1) [],
        ACPU_0_WAKE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpi2 [
        VCC_INT_FP_DISCONNECT OFFSET(31) NUMBITS(1) [],
        VCC_INT_DISCONNECT OFFSET(30) NUMBITS(1) [],
        VCC_AUX_DISCONNECT OFFSET(29) NUMBITS(1) [],
        RESERVED0 OFFSET(24) NUMBITS(5) [],
        DBG_ACPU3_RST_REQ OFFSET(23) NUMBITS(1) [],
        DBG_ACPU2_RST_REQ OFFSET(22) NUMBITS(1) [],
        DBG_ACPU1_RST_REQ OFFSET(21) NUMBITS(1) [],
        DBG_ACPU0_RST_REQ OFFSET(20) NUMBITS(1) [],
        CP_ACPU3_RST_REQ OFFSET(19) NUMBITS(1) [],
        CP_ACPU2_RST_REQ OFFSET(18) NUMBITS(1) [],
        CP_ACPU1_RST_REQ OFFSET(17) NUMBITS(1) [],
        CP_ACPU0_RST_REQ OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        DBG_RCPU1_RST_REQ OFFSET(9) NUMBITS(1) [],
        DBG_RCPU0_RST_REQ OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(6) NUMBITS(2) [],
        R5_1_SLEEP OFFSET(5) NUMBITS(1) [],
        R5_0_SLEEP OFFSET(4) NUMBITS(1) [],
        ACPU_3_SLEEP OFFSET(3) NUMBITS(1) [],
        ACPU_2_SLEEP OFFSET(2) NUMBITS(1) [],
        ACPU_1_SLEEP OFFSET(1) NUMBITS(1) [],
        ACPU_0_SLEEP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpi3 [
        PL_GPI_31 OFFSET(31) NUMBITS(1) [],
        PL_GPI_30 OFFSET(30) NUMBITS(1) [],
        PL_GPI_29 OFFSET(29) NUMBITS(1) [],
        PL_GPI_28 OFFSET(28) NUMBITS(1) [],
        PL_GPI_27 OFFSET(27) NUMBITS(1) [],
        PL_GPI_26 OFFSET(26) NUMBITS(1) [],
        PL_GPI_25 OFFSET(25) NUMBITS(1) [],
        PL_GPI_24 OFFSET(24) NUMBITS(1) [],
        PL_GPI_23 OFFSET(23) NUMBITS(1) [],
        PL_GPI_22 OFFSET(22) NUMBITS(1) [],
        PL_GPI_21 OFFSET(21) NUMBITS(1) [],
        PL_GPI_20 OFFSET(20) NUMBITS(1) [],
        PL_GPI_19 OFFSET(19) NUMBITS(1) [],
        PL_GPI_18 OFFSET(18) NUMBITS(1) [],
        PL_GPI_17 OFFSET(17) NUMBITS(1) [],
        PL_GPI_16 OFFSET(16) NUMBITS(1) [],
        PL_GPI_15 OFFSET(15) NUMBITS(1) [],
        PL_GPI_14 OFFSET(14) NUMBITS(1) [],
        PL_GPI_13 OFFSET(13) NUMBITS(1) [],
        PL_GPI_12 OFFSET(12) NUMBITS(1) [],
        PL_GPI_11 OFFSET(11) NUMBITS(1) [],
        PL_GPI_10 OFFSET(10) NUMBITS(1) [],
        PL_GPI_9 OFFSET(9) NUMBITS(1) [],
        PL_GPI_8 OFFSET(8) NUMBITS(1) [],
        PL_GPI_7 OFFSET(7) NUMBITS(1) [],
        PL_GPI_6 OFFSET(6) NUMBITS(1) [],
        PL_GPI_5 OFFSET(5) NUMBITS(1) [],
        PL_GPI_4 OFFSET(4) NUMBITS(1) [],
        PL_GPI_3 OFFSET(3) NUMBITS(1) [],
        PL_GPI_2 OFFSET(2) NUMBITS(1) [],
        PL_GPI_1 OFFSET(1) NUMBITS(1) [],
        PL_GPI_0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrqStatus [
        CSU_PMU_SEC_LOCK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(30) NUMBITS(1) [],
        INV_ADDR OFFSET(29) NUMBITS(1) [],
        PWR_DN_REQ OFFSET(28) NUMBITS(1) [],
        PWR_UP_REQ OFFSET(27) NUMBITS(1) [],
        SW_RST_REQ OFFSET(26) NUMBITS(1) [],
        HW_RST_REQ OFFSET(25) NUMBITS(1) [],
        ISO_REQ OFFSET(24) NUMBITS(1) [],
        FW_REQ OFFSET(23) NUMBITS(1) [],
        IPI3 OFFSET(22) NUMBITS(1) [],
        IPI2 OFFSET(21) NUMBITS(1) [],
        IPI1 OFFSET(20) NUMBITS(1) [],
        IPI0 OFFSET(19) NUMBITS(1) [],
        RTC_ALARM OFFSET(18) NUMBITS(1) [],
        RTC_EVERY_SECOND OFFSET(17) NUMBITS(1) [],
        CORRECTABLE_ECC OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(15) NUMBITS(1) [],
        GPI3 OFFSET(14) NUMBITS(1) [],
        GPI2 OFFSET(13) NUMBITS(1) [],
        GPI1 OFFSET(12) NUMBITS(1) [],
        GPI0 OFFSET(11) NUMBITS(1) [],
        RESERVED2 OFFSET(7) NUMBITS(4) [],
        PIT3 OFFSET(6) NUMBITS(1) [],
        PIT2 OFFSET(5) NUMBITS(1) [],
        PIT1 OFFSET(4) NUMBITS(1) [],
        PIT0 OFFSET(3) NUMBITS(1) [],
        RESERVED3 OFFSET(2) NUMBITS(1) [],
        RESERVED4 OFFSET(1) NUMBITS(1) [],
        RESERVED5 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrqPending [
        CSU_PMU_SEC_LOCK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(30) NUMBITS(1) [],
        INV_ADDR OFFSET(29) NUMBITS(1) [],
        PWR_DN_REQ OFFSET(28) NUMBITS(1) [],
        PWR_UP_REQ OFFSET(27) NUMBITS(1) [],
        SW_RST_REQ OFFSET(26) NUMBITS(1) [],
        HW_RST_REQ OFFSET(25) NUMBITS(1) [],
        ISO_REQ OFFSET(24) NUMBITS(1) [],
        FW_REQ OFFSET(23) NUMBITS(1) [],
        IPI3 OFFSET(22) NUMBITS(1) [],
        IPI2 OFFSET(21) NUMBITS(1) [],
        IPI1 OFFSET(20) NUMBITS(1) [],
        IPI0 OFFSET(19) NUMBITS(1) [],
        RTC_ALARM OFFSET(18) NUMBITS(1) [],
        RTC_EVERY_SECOND OFFSET(17) NUMBITS(1) [],
        CORRECTABLE_ECC OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(15) NUMBITS(1) [],
        GPI3 OFFSET(14) NUMBITS(1) [],
        GPI2 OFFSET(13) NUMBITS(1) [],
        GPI1 OFFSET(12) NUMBITS(1) [],
        GPI0 OFFSET(11) NUMBITS(1) [],
        RESERVED2 OFFSET(7) NUMBITS(4) [],
        PIT3 OFFSET(6) NUMBITS(1) [],
        PIT2 OFFSET(5) NUMBITS(1) [],
        PIT1 OFFSET(4) NUMBITS(1) [],
        PIT0 OFFSET(3) NUMBITS(1) [],
        RESERVED3 OFFSET(2) NUMBITS(1) [],
        RESERVED4 OFFSET(1) NUMBITS(1) [],
        RESERVED5 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrqEnableR [
        INV_ADDR OFFSET(29) NUMBITS(1) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        GPI3 OFFSET(14) NUMBITS(1) [],
        GPI2 OFFSET(13) NUMBITS(1) [],
        GPI1 OFFSET(12) NUMBITS(1) [],
        GPI0 OFFSET(11) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(4) [],
        RESERVED2 OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(1) [],
        RESERVED4 OFFSET(0) NUMBITS(1) [],
    ],
    pub IrqEnableW [
        CSU_PMU_SEC_LOCK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(30) NUMBITS(1) [],
        PWR_DN_REQ OFFSET(28) NUMBITS(1) [],
        PWR_UP_REQ OFFSET(27) NUMBITS(1) [],
        SW_RST_REQ OFFSET(26) NUMBITS(1) [],
        HW_RST_REQ OFFSET(25) NUMBITS(1) [],
        ISO_REQ OFFSET(24) NUMBITS(1) [],
        FW_REQ OFFSET(23) NUMBITS(1) [],
        IPI3 OFFSET(22) NUMBITS(1) [],
        IPI2 OFFSET(21) NUMBITS(1) [],
        IPI1 OFFSET(20) NUMBITS(1) [],
        IPI0 OFFSET(19) NUMBITS(1) [],
        RTC_ALARM OFFSET(18) NUMBITS(1) [],
        RTC_EVERY_SECOND OFFSET(17) NUMBITS(1) [],
        CORRECTABLE_ECC OFFSET(16) NUMBITS(1) [],
        PIT3 OFFSET(6) NUMBITS(1) [],
        PIT2 OFFSET(5) NUMBITS(1) [],
        PIT1 OFFSET(4) NUMBITS(1) [],
        PIT0 OFFSET(3) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrqAckR [
        INV_ADDR OFFSET(29) NUMBITS(1) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(4) [],
        RESERVED2 OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(1) [],
        RESERVED4 OFFSET(0) NUMBITS(1) [],
    ],
    pub IrqAckW [
        CSU_PMU_SEC_LOCK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(30) NUMBITS(1) [],
        PWR_DN_REQ OFFSET(28) NUMBITS(1) [],
        PWR_UP_REQ OFFSET(27) NUMBITS(1) [],
        SW_RST_REQ OFFSET(26) NUMBITS(1) [],
        HW_RST_REQ OFFSET(25) NUMBITS(1) [],
        ISO_REQ OFFSET(24) NUMBITS(1) [],
        FW_REQ OFFSET(23) NUMBITS(1) [],
        IPI3 OFFSET(22) NUMBITS(1) [],
        IPI2 OFFSET(21) NUMBITS(1) [],
        IPI1 OFFSET(20) NUMBITS(1) [],
        IPI0 OFFSET(19) NUMBITS(1) [],
        RTC_ALARM OFFSET(18) NUMBITS(1) [],
        RTC_EVERY_SECOND OFFSET(17) NUMBITS(1) [],
        CORRECTABLE_ECC OFFSET(16) NUMBITS(1) [],
        GPI3 OFFSET(14) NUMBITS(1) [],
        GPI2 OFFSET(13) NUMBITS(1) [],
        GPI1 OFFSET(12) NUMBITS(1) [],
        GPI0 OFFSET(11) NUMBITS(1) [],
        PIT3 OFFSET(6) NUMBITS(1) [],
        PIT2 OFFSET(5) NUMBITS(1) [],
        PIT1 OFFSET(4) NUMBITS(1) [],
        PIT0 OFFSET(3) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pit0ControlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub Pit0ControlW [
        PRELOAD OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pit1ControlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub Pit1ControlW [
        PRELOAD OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pit2ControlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub Pit2ControlW [
        PRELOAD OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pit3ControlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub Pit3ControlW [
        PRELOAD OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];

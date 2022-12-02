// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadWrite};
/// FPD System Memory Management Unit, SMMU Configuration and Event Control
pub static mut SMMU_REG: *mut Registers = 0xfd5f0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Controls for the register block.
        (0x00000000 => pub misc_ctrl: ReadWrite<u8, MiscCtrl::Register>),
        (0x00000001 => _padding1),
        /// Interrupt Status Register. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
        (0x00000010 => pub isr_0: Aliased<u32, Isr0R::Register, Isr0W::Register>),
        /// Interrupt Mask Register. This is a read-only location and can be atomically altered by either the IDR or the IER.
        (0x00000014 => pub imr_0: Aliased<u32, Imr0R::Register, Imr0W::Register>),
        /// Interrupt Enable Register. A write of 1 to this location will unmask the interrupt
        (0x00000018 => pub ier_0: Aliased<u32, Ier0R::Register, Ier0W::Register>),
        /// Interrupt Disable Register. A write of 1 to this location will mask the interrupt
        (0x0000001c => pub idr_0: Aliased<u32, Idr0R::Register, Idr0W::Register>),
        (0x00000020 => _padding32),
        /// Low Power Signals for TBU
        (0x00000040 => pub qreqn: ReadWrite<u32, Qreqn::Register>),
        (0x00000044 => _padding68),
        /// Miscellaneous signals
        (0x00000054 => pub misc: Aliased<u32, MiscR::Register, MiscW::Register>),
        /// Miscellaneous signals
        (0x00000058 => pub config_signals: ReadWrite<u32, ConfigSignals::Register>),
        (0x0000005c => @END),
    }
}
register_bitfields! [
    u8,
    pub MiscCtrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Isr0R [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(26) [],
        GBL_FLT_IRPT_NS OFFSET(4) NUMBITS(1) [],
        GBL_FLT_IRPT_S OFFSET(3) NUMBITS(1) [],
        COMB_PERF_IRPT_TBU OFFSET(2) NUMBITS(1) [],
        COMB_IRPT_S OFFSET(1) NUMBITS(1) [],
        COMB_IRPT_NS OFFSET(0) NUMBITS(1) [],
    ],
    pub Isr0W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        GBL_FLT_IRPT_NS OFFSET(4) NUMBITS(1) [],
        GBL_FLT_IRPT_S OFFSET(3) NUMBITS(1) [],
        COMB_PERF_IRPT_TBU OFFSET(2) NUMBITS(1) [],
        COMB_IRPT_S OFFSET(1) NUMBITS(1) [],
        COMB_IRPT_NS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Imr0R [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(26) [],
        GBL_FLT_IRPT_NS OFFSET(4) NUMBITS(1) [],
        GBL_FLT_IRPT_S OFFSET(3) NUMBITS(1) [],
        COMB_PERF_IRPT_TBU OFFSET(2) NUMBITS(1) [],
        COMB_IRPT_S OFFSET(1) NUMBITS(1) [],
        COMB_IRPT_NS OFFSET(0) NUMBITS(1) [],
    ],
    pub Imr0W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Ier0R [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(26) [],
    ],
    pub Ier0W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        GBL_FLT_IRPT_NS OFFSET(4) NUMBITS(1) [],
        GBL_FLT_IRPT_S OFFSET(3) NUMBITS(1) [],
        COMB_PERF_IRPT_TBU OFFSET(2) NUMBITS(1) [],
        COMB_IRPT_S OFFSET(1) NUMBITS(1) [],
        COMB_IRPT_NS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Idr0R [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(26) [],
    ],
    pub Idr0W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        GBL_FLT_IRPT_NS OFFSET(4) NUMBITS(1) [],
        GBL_FLT_IRPT_S OFFSET(3) NUMBITS(1) [],
        COMB_PERF_IRPT_TBU OFFSET(2) NUMBITS(1) [],
        COMB_IRPT_S OFFSET(1) NUMBITS(1) [],
        COMB_IRPT_NS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Qreqn [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        TBU_TBU5_5_CG OFFSET(14) NUMBITS(1) [],
        TBU_TBU5_5_PD OFFSET(13) NUMBITS(1) [],
        TBU_TBU4_4_CG OFFSET(12) NUMBITS(1) [],
        TBU_TBU4_4_PD OFFSET(11) NUMBITS(1) [],
        TBU_TBU3_3_CG OFFSET(10) NUMBITS(1) [],
        TBU_TBU3_3_PD OFFSET(9) NUMBITS(1) [],
        PD_MST_BR_TBU2_2 OFFSET(8) NUMBITS(1) [],
        PD_SLV_BR_TBU2_2 OFFSET(7) NUMBITS(1) [],
        TBU_TBU2_2_CG OFFSET(6) NUMBITS(1) [],
        TBU_TBU2_2_PD OFFSET(5) NUMBITS(1) [],
        TBU_TBU1_1_CG OFFSET(4) NUMBITS(1) [],
        TBU_TBU1_1_PD OFFSET(3) NUMBITS(1) [],
        TBU_TBU0_0_CG OFFSET(2) NUMBITS(1) [],
        TBU_TBU0_0_PD OFFSET(1) NUMBITS(1) [],
        TCU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub MiscR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        SPNIDEN OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(4) [],
        AWAKEUP_PROG OFFSET(7) NUMBITS(1) [],
        RESERVED2 OFFSET(6) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(2) [],
        RESERVED4 OFFSET(1) NUMBITS(3) [],
        RESERVED5 OFFSET(0) NUMBITS(1) [],
    ],
    pub MiscW [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        SPNIDEN OFFSET(12) NUMBITS(1) [],
        AWAKEUP_PROG OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(2) [],
        RESERVED3 OFFSET(1) NUMBITS(3) [],
        RESERVED4 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ConfigSignals [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        CFG_NORMALIZE OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];

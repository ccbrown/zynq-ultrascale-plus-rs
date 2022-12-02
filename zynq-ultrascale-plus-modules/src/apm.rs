// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// AXI Performance Monitor, CCI APM Control and Configuration
pub static mut APM_CCI_INTC: *mut Registers = 0xfd490000 as *mut Registers;
/// AXI Performance Monitor, OCM Performance Monitor
pub static mut APM_INTC_OCM: *mut Registers = 0xffa00000 as *mut Registers;
/// AXI Performance Monitor, LPD Performance Monitor
pub static mut APM_LPD_FPD: *mut Registers = 0xffa10000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Global Clock Counter
        (0x00000000 => pub gccr_h: ReadOnly<u32>),
        /// Global Clock Counter
        (0x00000004 => pub gccr_l: ReadOnly<u32>),
        (0x00000008 => _padding8),
        /// Sample Interval Time Configuration
        (0x00000024 => pub sir: ReadWrite<u32>),
        /// Sample Interval Control
        (0x00000028 => pub sicr: Aliased<u32, SicrR::Register, SicrW::Register>),
        /// Sample Interval Sample. Reading this initiates sampling of Metric Counters data to the Sample Metric Counters.
        (0x0000002c => pub sisr: ReadWrite<u32>),
        /// Global Interrupt Enable Register
        (0x00000030 => pub gier: Aliased<u32, GierR::Register, GierW::Register>),
        /// Interrupt Enable
        (0x00000034 => pub ier: Aliased<u32, IerR::Register, IerW::Register>),
        /// Interrupt Status
        (0x00000038 => pub isr: Aliased<u32, IsrR::Register, IsrW::Register>),
        (0x0000003c => _padding60),
        /// Metric Selector, Counters 0, 1, 2, and 3
        (0x00000044 => pub msr_0: ReadWrite<u32, Msr0::Register>),
        /// Metric Selector, Counters 4, 5, 6, and 7
        (0x00000048 => pub msr_1: ReadWrite<u32, Msr1::Register>),
        (0x0000004c => _padding76),
        /// Incrementer
        (0x00000104 => pub ir_0: ReadOnly<u32>),
        /// Range
        (0x00000108 => pub rr_0: ReadWrite<u32, Rr0::Register>),
        /// Metric Count Log Enable
        (0x0000010c => pub mcler_0: ReadWrite<u32>),
        (0x00000110 => _padding272),
        /// Incrementer
        (0x00000114 => pub ir_1: ReadOnly<u32>),
        /// Range
        (0x00000118 => pub rr_1: ReadWrite<u32, Rr1::Register>),
        /// Metric Count Log Enable
        (0x0000011c => pub mcler_1: ReadWrite<u32>),
        (0x00000120 => _padding288),
        /// Incrementer
        (0x00000124 => pub ir_2: ReadOnly<u32>),
        /// Range
        (0x00000128 => pub rr_2: ReadWrite<u32, Rr2::Register>),
        /// Metric Count Log Enable
        (0x0000012c => pub mcler_2: ReadWrite<u32>),
        (0x00000130 => _padding304),
        /// Incrementer
        (0x00000134 => pub ir_3: ReadOnly<u32>),
        /// Range
        (0x00000138 => pub rr_3: ReadWrite<u32, Rr3::Register>),
        /// Metric Count Log Enable
        (0x0000013c => pub mcler_3: ReadWrite<u32>),
        (0x00000140 => _padding320),
        /// Incrementer
        (0x00000144 => pub ir_4: ReadOnly<u32>),
        /// Range
        (0x00000148 => pub rr_4: ReadWrite<u32, Rr4::Register>),
        /// Metric Count Log Enable
        (0x0000014c => pub mcler_4: ReadWrite<u32>),
        (0x00000150 => _padding336),
        /// Incrementer
        (0x00000154 => pub ir_5: ReadOnly<u32>),
        /// Range
        (0x00000158 => pub rr_5: ReadWrite<u32, Rr5::Register>),
        /// Metric Count Log Enable
        (0x0000015c => pub mcler_5: ReadWrite<u32>),
        (0x00000160 => _padding352),
        /// Incrementer
        (0x00000164 => pub ir_6: ReadOnly<u32>),
        /// Range
        (0x00000168 => pub rr_6: ReadWrite<u32, Rr6::Register>),
        /// Metric Count Log Enable
        (0x0000016c => pub mcler_6: ReadWrite<u32>),
        (0x00000170 => _padding368),
        /// Incrementer
        (0x00000174 => pub ir_7: ReadOnly<u32>),
        /// Range
        (0x00000178 => pub rr_7: ReadWrite<u32, Rr7::Register>),
        /// Metric Count Log Enable
        (0x0000017c => pub mcler_7: ReadWrite<u32>),
        (0x00000180 => _padding384),
        /// Sampled Metric Counter
        (0x00000200 => pub smcr_0: ReadOnly<u32>),
        /// Sampled Incrementer
        (0x00000204 => pub sir_0: ReadOnly<u32>),
        (0x00000208 => _padding520),
        /// Sampled Metric Counter
        (0x00000210 => pub smcr_1: ReadOnly<u32>),
        /// Sampled Incrementer
        (0x00000214 => pub sir_1: ReadOnly<u32>),
        (0x00000218 => _padding536),
        /// Sampled Metric Counter
        (0x00000220 => pub smcr_2: ReadOnly<u32>),
        /// Sampled Incrementer
        (0x00000224 => pub sir_2: ReadOnly<u32>),
        (0x00000228 => _padding552),
        /// Sampled Metric Counter
        (0x00000230 => pub smcr_3: ReadOnly<u32>),
        /// Sampled Incrementer
        (0x00000234 => pub sir_3: ReadOnly<u32>),
        (0x00000238 => _padding568),
        /// Sampled Metric Counter
        (0x00000240 => pub smcr_4: ReadOnly<u32>),
        /// Sampled Incrementer
        (0x00000244 => pub sir_4: ReadOnly<u32>),
        (0x00000248 => _padding584),
        /// Sampled Metric Counter
        (0x00000250 => pub smcr_5: ReadOnly<u32>),
        /// Sampled Incrementer
        (0x00000254 => pub sir_5: ReadOnly<u32>),
        (0x00000258 => _padding600),
        /// Sampled Metric Counter
        (0x00000260 => pub smcr_6: ReadOnly<u32>),
        /// Sampled Incrementer
        (0x00000264 => pub sir_6: ReadOnly<u32>),
        (0x00000268 => _padding616),
        /// Sampled Metric Counter
        (0x00000270 => pub smcr_7: ReadOnly<u32>),
        /// Sampled Incrementer
        (0x00000274 => pub sir_7: ReadOnly<u32>),
        (0x00000278 => _padding632),
        /// Control
        (0x00000300 => pub cr: Aliased<u32, CrR::Register, CrW::Register>),
        /// WID
        (0x00000304 => pub widr: ReadWrite<u32>),
        /// WID Mask
        (0x00000308 => pub widmr: ReadWrite<u32>),
        /// RID
        (0x0000030c => pub ridr: ReadWrite<u32>),
        /// RID Mask
        (0x00000310 => pub ridmr: ReadWrite<u32>),
        (0x00000314 => _padding788),
        /// Flag Enable
        (0x00000400 => pub fecr: Aliased<u32, FecrR::Register, FecrW::Register>),
        (0x00000404 => @END),
    }
}
register_bitfields! [
    u32,
    pub SicrR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MET_CNT_RST OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(6) [],
        LOAD OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub SicrW [
        MET_CNT_RST OFFSET(8) NUMBITS(1) [],
        LOAD OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub GierR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GIE OFFSET(0) NUMBITS(1) [],
    ],
    pub GierW [
        GIE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IerR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        RESERVED1 OFFSET(12) NUMBITS(1) [],
        RESERVED2 OFFSET(11) NUMBITS(1) [],
        MET_CT7_OVFLINT_EN OFFSET(10) NUMBITS(1) [],
        MET_CT6_OVFLINT_EN OFFSET(9) NUMBITS(1) [],
        MET_CT5_OVFLINT_EN OFFSET(8) NUMBITS(1) [],
        MET_CT4_OVFLINT_EN OFFSET(7) NUMBITS(1) [],
        MET_CT3_OVFLINT_EN OFFSET(6) NUMBITS(1) [],
        MET_CT2_OVFLINT_EN OFFSET(5) NUMBITS(1) [],
        MET_CT1_OVFLINT_EN OFFSET(4) NUMBITS(1) [],
        MET_CT0_OVFLINT_EN OFFSET(3) NUMBITS(1) [],
        RESERVED3 OFFSET(2) NUMBITS(1) [],
        SMPL_INTRVL_OVFLINT_EN OFFSET(1) NUMBITS(1) [],
        GLBCLKCNT_OVFLINT_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub IerW [
        RESERVED0 OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        MET_CT7_OVFLINT_EN OFFSET(10) NUMBITS(1) [],
        MET_CT6_OVFLINT_EN OFFSET(9) NUMBITS(1) [],
        MET_CT5_OVFLINT_EN OFFSET(8) NUMBITS(1) [],
        MET_CT4_OVFLINT_EN OFFSET(7) NUMBITS(1) [],
        MET_CT3_OVFLINT_EN OFFSET(6) NUMBITS(1) [],
        MET_CT2_OVFLINT_EN OFFSET(5) NUMBITS(1) [],
        MET_CT1_OVFLINT_EN OFFSET(4) NUMBITS(1) [],
        MET_CT0_OVFLINT_EN OFFSET(3) NUMBITS(1) [],
        RESERVED2 OFFSET(2) NUMBITS(1) [],
        SMPL_INTRVL_OVFLINT_EN OFFSET(1) NUMBITS(1) [],
        GLBCLKCNT_OVFLINT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IsrR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        RESERVED1 OFFSET(12) NUMBITS(1) [],
        RESERVED2 OFFSET(11) NUMBITS(1) [],
        MET_CT7_OVFLINT OFFSET(10) NUMBITS(1) [],
        MET_CT6_OVFLINT OFFSET(9) NUMBITS(1) [],
        MET_CT5_OVFLINT OFFSET(8) NUMBITS(1) [],
        MET_CT4_OVFLINT OFFSET(7) NUMBITS(1) [],
        MET_CT3_OVFLINT OFFSET(6) NUMBITS(1) [],
        MET_CT2_OVFLINT OFFSET(5) NUMBITS(1) [],
        MET_CT1_OVFLINT OFFSET(4) NUMBITS(1) [],
        MET_CT0_OVFLINT OFFSET(3) NUMBITS(1) [],
        RESERVED3 OFFSET(2) NUMBITS(1) [],
        SMPL_INTRVL_OVFLINT OFFSET(1) NUMBITS(1) [],
        GLBCLKCNT_OVFLINT OFFSET(0) NUMBITS(1) [],
    ],
    pub IsrW [
        RESERVED0 OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        MET_CT7_OVFLINT OFFSET(10) NUMBITS(1) [],
        MET_CT6_OVFLINT OFFSET(9) NUMBITS(1) [],
        MET_CT5_OVFLINT OFFSET(8) NUMBITS(1) [],
        MET_CT4_OVFLINT OFFSET(7) NUMBITS(1) [],
        MET_CT3_OVFLINT OFFSET(6) NUMBITS(1) [],
        MET_CT2_OVFLINT OFFSET(5) NUMBITS(1) [],
        MET_CT1_OVFLINT OFFSET(4) NUMBITS(1) [],
        MET_CT0_OVFLINT OFFSET(3) NUMBITS(1) [],
        RESERVED2 OFFSET(2) NUMBITS(1) [],
        SMPL_INTRVL_OVFLINT OFFSET(1) NUMBITS(1) [],
        GLBCLKCNT_OVFLINT OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Msr0 [
        MET_CT3_SLOT OFFSET(29) NUMBITS(3) [],
        MET_CT3_SEL OFFSET(24) NUMBITS(5) [],
        MET_CT2_SLOT OFFSET(21) NUMBITS(3) [],
        MET_CT2_SEL OFFSET(16) NUMBITS(5) [],
        MET_CT1_SLOT OFFSET(13) NUMBITS(3) [],
        MET_CT1_SEL OFFSET(8) NUMBITS(5) [],
        MET_CT0_SLOT OFFSET(5) NUMBITS(3) [],
        MET_CT0_SEL OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u32,
    pub Msr1 [
        MET_CT7_SLOT OFFSET(29) NUMBITS(3) [],
        MET_CT7_SEL OFFSET(24) NUMBITS(5) [],
        MET_CT6_SLOT OFFSET(21) NUMBITS(3) [],
        MET_CT6_SEL OFFSET(16) NUMBITS(5) [],
        MET_CT5_SLOT OFFSET(13) NUMBITS(3) [],
        MET_CT5_SEL OFFSET(8) NUMBITS(5) [],
        MET_CT4_SLOT OFFSET(5) NUMBITS(3) [],
        MET_CT4_SEL OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u32,
    pub Rr0 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Rr1 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Rr2 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Rr3 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Rr4 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Rr5 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Rr6 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Rr7 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub CrR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        STR_FIFO_RST OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(1) [],
        RESERVED2 OFFSET(18) NUMBITS(6) [],
        GCCR_RST OFFSET(17) NUMBITS(1) [],
        GCCR_EN OFFSET(16) NUMBITS(1) [],
        RESERVED3 OFFSET(10) NUMBITS(6) [],
        RESERVED4 OFFSET(9) NUMBITS(1) [],
        RESERVED5 OFFSET(8) NUMBITS(1) [],
        LATENCY_READ_END OFFSET(7) NUMBITS(1) [],
        LATENCY_READ_START OFFSET(6) NUMBITS(1) [],
        LATENCY_WRITE_END OFFSET(5) NUMBITS(1) [],
        LATENCY_WRITE_START OFFSET(4) NUMBITS(1) [],
        ID_MASKING_EN OFFSET(3) NUMBITS(1) [],
        RESERVED6 OFFSET(2) NUMBITS(1) [],
        MET_CNT_RST OFFSET(1) NUMBITS(1) [],
        MET_CNT_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub CrW [
        STR_FIFO_RST OFFSET(25) NUMBITS(1) [],
        GCCR_RST OFFSET(17) NUMBITS(1) [],
        GCCR_EN OFFSET(16) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        LATENCY_READ_END OFFSET(7) NUMBITS(1) [],
        LATENCY_READ_START OFFSET(6) NUMBITS(1) [],
        LATENCY_WRITE_END OFFSET(5) NUMBITS(1) [],
        LATENCY_WRITE_START OFFSET(4) NUMBITS(1) [],
        ID_MASKING_EN OFFSET(3) NUMBITS(1) [],
        RESERVED2 OFFSET(2) NUMBITS(1) [],
        MET_CNT_RST OFFSET(1) NUMBITS(1) [],
        MET_CNT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub FecrR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        RESERVED4 OFFSET(27) NUMBITS(1) [],
        RESERVED5 OFFSET(26) NUMBITS(1) [],
        RESERVED6 OFFSET(25) NUMBITS(1) [],
        RESERVED7 OFFSET(24) NUMBITS(1) [],
        RESERVED8 OFFSET(23) NUMBITS(1) [],
        RESERVED9 OFFSET(22) NUMBITS(1) [],
        SMP_CNT_LAPSE_FLG OFFSET(21) NUMBITS(1) [],
        GCC_OFVL_FLG OFFSET(20) NUMBITS(1) [],
        RESERVED10 OFFSET(19) NUMBITS(1) [],
        RESERVED11 OFFSET(18) NUMBITS(1) [],
        RESERVED12 OFFSET(17) NUMBITS(1) [],
        SFT_DATA_FLG_EN OFFSET(16) NUMBITS(1) [],
        RESERVED13 OFFSET(7) NUMBITS(9) [],
        LAST_READ_FLG OFFSET(6) NUMBITS(1) [],
        FIRST_READ_FLG OFFSET(5) NUMBITS(1) [],
        READ_ADDR_FLG OFFSET(4) NUMBITS(1) [],
        RESPONSE_FLG OFFSET(3) NUMBITS(1) [],
        LAST_WRITE_FLG OFFSET(2) NUMBITS(1) [],
        FIRST_WRITE_FLG OFFSET(1) NUMBITS(1) [],
        WRITE_ADDR_FLG OFFSET(0) NUMBITS(1) [],
    ],
    pub FecrW [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        RESERVED4 OFFSET(27) NUMBITS(1) [],
        RESERVED5 OFFSET(26) NUMBITS(1) [],
        RESERVED6 OFFSET(25) NUMBITS(1) [],
        RESERVED7 OFFSET(24) NUMBITS(1) [],
        RESERVED8 OFFSET(23) NUMBITS(1) [],
        RESERVED9 OFFSET(22) NUMBITS(1) [],
        SMP_CNT_LAPSE_FLG OFFSET(21) NUMBITS(1) [],
        GCC_OFVL_FLG OFFSET(20) NUMBITS(1) [],
        RESERVED10 OFFSET(19) NUMBITS(1) [],
        RESERVED11 OFFSET(18) NUMBITS(1) [],
        RESERVED12 OFFSET(17) NUMBITS(1) [],
        SFT_DATA_FLG_EN OFFSET(16) NUMBITS(1) [],
        LAST_READ_FLG OFFSET(6) NUMBITS(1) [],
        FIRST_READ_FLG OFFSET(5) NUMBITS(1) [],
        READ_ADDR_FLG OFFSET(4) NUMBITS(1) [],
        RESPONSE_FLG OFFSET(3) NUMBITS(1) [],
        LAST_WRITE_FLG OFFSET(2) NUMBITS(1) [],
        FIRST_WRITE_FLG OFFSET(1) NUMBITS(1) [],
        WRITE_ADDR_FLG OFFSET(0) NUMBITS(1) [],
    ]
];

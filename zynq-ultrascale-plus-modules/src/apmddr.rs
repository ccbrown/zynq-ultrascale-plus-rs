// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// DDR AXI Performance Monitor, AXI DDR Performance Monitor
pub static mut APM_DDR: *mut Registers = 0xfd0b0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Global Clock
    pub gccr_h: ReadOnly<u32>,
    /// Global Clock
    pub gccr_l: ReadOnly<u32>,
    _padding8: [u8; 28],
    /// Sample Interval Time Configuration
    pub sir: ReadWrite<u32>,
    /// Sample Interval Control
    pub sicr: Aliased<u32, SicrR::Register, SicrW::Register>,
    /// Sample Interval Sample
    pub sisr: ReadWrite<u32>,
    /// Global Interrupt Enable
    pub gier: Aliased<u32, GierR::Register, GierW::Register>,
    /// Interrupt Enable
    pub ier: Aliased<u32, IerR::Register, IerW::Register>,
    /// Interrupt Status
    pub isr: Aliased<u32, IsrR::Register, IsrW::Register>,
    _padding60: [u8; 8],
    /// Metric Selector, Counters 0, 1, 2, and 3
    pub msr_0: ReadWrite<u32, Msr0::Register>,
    /// Metric Selector, Counters 4, 5, 6, and 7
    pub msr_1: ReadWrite<u32, Msr1::Register>,
    /// Metric Selector, Counters 8 and 9
    pub msr_2: Aliased<u32, Msr2R::Register, Msr2W::Register>,
    _padding80: [u8; 176],
    /// Metric Counter
    pub mcr_0: ReadOnly<u32>,
    /// Incrementer
    pub ir_0: ReadOnly<u32>,
    /// Range
    pub rr_0: ReadWrite<u32, Rr0::Register>,
    _padding268: [u8; 4],
    /// Metric Counter
    pub mcr_1: ReadOnly<u32>,
    /// Incrementer
    pub ir_1: ReadOnly<u32>,
    /// Range
    pub rr_1: ReadWrite<u32, Rr1::Register>,
    _padding284: [u8; 4],
    /// Metric Counter
    pub mcr_2: ReadOnly<u32>,
    /// Incrementer
    pub ir_2: ReadOnly<u32>,
    /// Range
    pub rr_2: ReadWrite<u32, Rr2::Register>,
    _padding300: [u8; 4],
    /// Metric Counter
    pub mcr_3: ReadOnly<u32>,
    /// Incrementer
    pub ir_3: ReadOnly<u32>,
    /// Range
    pub rr_3: ReadWrite<u32, Rr3::Register>,
    _padding316: [u8; 4],
    /// Metric Counter
    pub mcr_4: ReadOnly<u32>,
    /// Incrementer
    pub ir_4: ReadOnly<u32>,
    /// Range
    pub rr_4: ReadWrite<u32, Rr4::Register>,
    _padding332: [u8; 4],
    /// Metric Counter
    pub mcr_5: ReadOnly<u32>,
    /// Incrementer
    pub ir_5: ReadOnly<u32>,
    /// Range
    pub rr_5: ReadWrite<u32, Rr5::Register>,
    _padding348: [u8; 4],
    /// Metric Counter
    pub mcr_6: ReadOnly<u32>,
    /// Incrementer
    pub ir_6: ReadOnly<u32>,
    /// Range
    pub rr_6: ReadWrite<u32, Rr6::Register>,
    _padding364: [u8; 4],
    /// Metric Counter
    pub mcr_7: ReadOnly<u32>,
    /// Incrementer
    pub ir_7: ReadOnly<u32>,
    /// Range
    pub rr_7: ReadWrite<u32, Rr7::Register>,
    _padding380: [u8; 4],
    /// Metric Counter
    pub mcr_8: ReadOnly<u32>,
    /// Incrementer
    pub ir_8: ReadOnly<u32>,
    /// Range
    pub rr_8: ReadWrite<u32, Rr8::Register>,
    _padding396: [u8; 4],
    /// Metric Counter
    pub mcr_9: ReadOnly<u32>,
    /// Incrementer
    pub ir_9: ReadOnly<u32>,
    /// Range
    pub rr_9: ReadWrite<u32, Rr9::Register>,
    _padding412: [u8; 100],
    /// Sampled Metric Counter
    pub smcr_0: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_0: ReadOnly<u32>,
    _padding520: [u8; 8],
    /// Sampled Metric Counter
    pub smcr_1: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_1: ReadOnly<u32>,
    _padding536: [u8; 8],
    /// Sampled Metric Counter
    pub smcr_2: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_2: ReadOnly<u32>,
    _padding552: [u8; 8],
    /// Sampled Metric Counter
    pub smcr_3: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_3: ReadOnly<u32>,
    _padding568: [u8; 8],
    /// Sampled Metric Counter
    pub smcr_4: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_4: ReadOnly<u32>,
    _padding584: [u8; 8],
    /// Sampled Metric Counter
    pub smcr_5: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_5: ReadOnly<u32>,
    _padding600: [u8; 8],
    /// Sampled Metric Counter
    pub smcr_6: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_6: ReadOnly<u32>,
    _padding616: [u8; 8],
    /// Sampled Metric Counter
    pub smcr_7: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_7: ReadOnly<u32>,
    _padding632: [u8; 8],
    /// Sampled Metric Counter
    pub smcr_8: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_8: ReadOnly<u32>,
    _padding648: [u8; 8],
    /// Sampled Metric Counter
    pub smcr_9: ReadOnly<u32>,
    /// Sampled Incrementer
    pub sir_9: ReadOnly<u32>,
    _padding664: [u8; 104],
    /// Control
    pub cr: Aliased<u32, CrR::Register, CrW::Register>,
    /// Write ID Filter.
    pub widr: ReadWrite<u32>,
    /// Write ID Mask.
    pub widmr: ReadWrite<u32>,
    /// Read ID Filter.
    pub ridr: ReadWrite<u32>,
    /// Read ID Mask.
    pub ridmr: ReadWrite<u32>,
    _padding788: [u8; 236],
    /// Flag Enable.
    pub fecr: Aliased<u32, FecrR::Register, FecrW::Register>,
}
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub GierR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GIE OFFSET(0) NUMBITS(1) [],
    ],
    pub GierW [
        GIE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IerR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        MET_CT9_OVFLINT_EN OFFSET(12) NUMBITS(1) [],
        MET_CT8_OVFLINT_EN OFFSET(11) NUMBITS(1) [],
        MET_CT7_OVFLINT_EN OFFSET(10) NUMBITS(1) [],
        MET_CT6_OVFLINT_EN OFFSET(9) NUMBITS(1) [],
        MET_CT5_OVFLINT_EN OFFSET(8) NUMBITS(1) [],
        MET_CT4_OVFLINT_EN OFFSET(7) NUMBITS(1) [],
        MET_CT3_OVFLINT_EN OFFSET(6) NUMBITS(1) [],
        MET_CT2_OVFLINT_EN OFFSET(5) NUMBITS(1) [],
        MET_CT1_OVFLINT_EN OFFSET(4) NUMBITS(1) [],
        MET_CT0_OVFLINT_EN OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        SMPL_INTRVL_OVFLINT_EN OFFSET(1) NUMBITS(1) [],
        GLBCLKCNT_OVFLINT_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub IerW [
        MET_CT9_OVFLINT_EN OFFSET(12) NUMBITS(1) [],
        MET_CT8_OVFLINT_EN OFFSET(11) NUMBITS(1) [],
        MET_CT7_OVFLINT_EN OFFSET(10) NUMBITS(1) [],
        MET_CT6_OVFLINT_EN OFFSET(9) NUMBITS(1) [],
        MET_CT5_OVFLINT_EN OFFSET(8) NUMBITS(1) [],
        MET_CT4_OVFLINT_EN OFFSET(7) NUMBITS(1) [],
        MET_CT3_OVFLINT_EN OFFSET(6) NUMBITS(1) [],
        MET_CT2_OVFLINT_EN OFFSET(5) NUMBITS(1) [],
        MET_CT1_OVFLINT_EN OFFSET(4) NUMBITS(1) [],
        MET_CT0_OVFLINT_EN OFFSET(3) NUMBITS(1) [],
        RESERVED0 OFFSET(2) NUMBITS(1) [],
        SMPL_INTRVL_OVFLINT_EN OFFSET(1) NUMBITS(1) [],
        GLBCLKCNT_OVFLINT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IsrR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        MET_CT9_OVFLINT OFFSET(12) NUMBITS(1) [],
        MET_CT8_OVFLINT OFFSET(11) NUMBITS(1) [],
        MET_CT7_OVFLINT OFFSET(10) NUMBITS(1) [],
        MET_CT6_OVFLINT OFFSET(9) NUMBITS(1) [],
        MET_CT5_OVFLINT OFFSET(8) NUMBITS(1) [],
        MET_CT4_OVFLINT OFFSET(7) NUMBITS(1) [],
        MET_CT3_OVFLINT OFFSET(6) NUMBITS(1) [],
        MET_CT2_OVFLINT OFFSET(5) NUMBITS(1) [],
        MET_CT1_OVFLINT OFFSET(4) NUMBITS(1) [],
        MET_CT0_OVFLINT OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        SMPL_INTRVL_OVFLINT OFFSET(1) NUMBITS(1) [],
        GLBCLKCNT_OVFLINT OFFSET(0) NUMBITS(1) [],
    ],
    pub IsrW [
        MET_CT9_OVFLINT OFFSET(12) NUMBITS(1) [],
        MET_CT8_OVFLINT OFFSET(11) NUMBITS(1) [],
        MET_CT7_OVFLINT OFFSET(10) NUMBITS(1) [],
        MET_CT6_OVFLINT OFFSET(9) NUMBITS(1) [],
        MET_CT5_OVFLINT OFFSET(8) NUMBITS(1) [],
        MET_CT4_OVFLINT OFFSET(7) NUMBITS(1) [],
        MET_CT3_OVFLINT OFFSET(6) NUMBITS(1) [],
        MET_CT2_OVFLINT OFFSET(5) NUMBITS(1) [],
        MET_CT1_OVFLINT OFFSET(4) NUMBITS(1) [],
        MET_CT0_OVFLINT OFFSET(3) NUMBITS(1) [],
        RESERVED0 OFFSET(2) NUMBITS(1) [],
        SMPL_INTRVL_OVFLINT OFFSET(1) NUMBITS(1) [],
        GLBCLKCNT_OVFLINT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub Msr2R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        MET_CT9_SLOT OFFSET(13) NUMBITS(3) [],
        MET_CT9_SEL OFFSET(8) NUMBITS(5) [],
        MET_CT8_SLOT OFFSET(5) NUMBITS(3) [],
        MET_CT8_SEL OFFSET(0) NUMBITS(5) [],
    ],
    pub Msr2W [
        MET_CT9_SLOT OFFSET(13) NUMBITS(3) [],
        MET_CT9_SEL OFFSET(8) NUMBITS(5) [],
        MET_CT8_SLOT OFFSET(5) NUMBITS(3) [],
        MET_CT8_SEL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr0 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr1 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr2 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr3 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr4 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr5 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr6 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr7 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr8 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rr9 [
        RANGE_HIGH OFFSET(16) NUMBITS(16) [],
        RANGE_LOW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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

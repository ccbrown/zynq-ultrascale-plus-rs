// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// General Purpose I/O Controller, GPIO Controller
pub static mut GPIO: *mut Registers = 0xff0a0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Maskable Output Data (GPIO Bank0, MIO, Lower 16bits)
    pub mask_data_0_lsw: Aliased<u32, MaskData0LswR::Register, MaskData0LswW::Register>,
    /// Maskable Output Data (GPIO Bank0, MIO, Upper 10bits)
    pub mask_data_0_msw: Aliased<u32, MaskData0MswR::Register, MaskData0MswW::Register>,
    /// Maskable Output Data (GPIO Bank1, MIO, Lower 16bits)
    pub mask_data_1_lsw: Aliased<u32, MaskData1LswR::Register, MaskData1LswW::Register>,
    /// Maskable Output Data (GPIO Bank1, MIO, Upper 10 bits)
    pub mask_data_1_msw: Aliased<u32, MaskData1MswR::Register, MaskData1MswW::Register>,
    /// Maskable Output Data (GPIO Bank2, MIO, Lower 16 bits)
    pub mask_data_2_lsw: Aliased<u32, MaskData2LswR::Register, MaskData2LswW::Register>,
    /// Maskable Output Data (GPIO Bank2, MIO, Upper 10 bits)
    pub mask_data_2_msw: Aliased<u32, MaskData2MswR::Register, MaskData2MswW::Register>,
    /// Maskable Output Data (GPIO Bank3, EMIO, Lower 16bits)
    pub mask_data_3_lsw: Aliased<u32, MaskData3LswR::Register, MaskData3LswW::Register>,
    /// Maskable Output Data (GPIO Bank3, EMIO, Upper 16bits)
    pub mask_data_3_msw: Aliased<u32, MaskData3MswR::Register, MaskData3MswW::Register>,
    /// Maskable Output Data (GPIO Bank4, EMIO, Lower 16bits)
    pub mask_data_4_lsw: Aliased<u32, MaskData4LswR::Register, MaskData4LswW::Register>,
    /// Maskable Output Data (GPIO Bank4, EMIO, Upper 16bits)
    pub mask_data_4_msw: Aliased<u32, MaskData4MswR::Register, MaskData4MswW::Register>,
    /// Maskable Output Data (GPIO Bank5, EMIO, Lower 16bits)
    pub mask_data_5_lsw: Aliased<u32, MaskData5LswR::Register, MaskData5LswW::Register>,
    /// Maskable Output Data (GPIO Bank5, EMIO, Upper 16bits)
    pub mask_data_5_msw: Aliased<u32, MaskData5MswR::Register, MaskData5MswW::Register>,
    _padding48: [u8; 16],
    /// Output Data (GPIO Bank0, MIO)
    pub data_0: Aliased<u32, Data0R::Register, Data0W::Register>,
    /// Output Data (GPIO Bank1, MIO)
    pub data_1: Aliased<u32, Data1R::Register, Data1W::Register>,
    /// Output Data (GPIO Bank2, MIO)
    pub data_2: Aliased<u32, Data2R::Register, Data2W::Register>,
    /// Output Data (GPIO Bank3, EMIO)
    pub data_3: ReadWrite<u32>,
    /// Output Data (GPIO Bank4, EMIO)
    pub data_4: ReadWrite<u32>,
    /// Output Data (GPIO Bank5, EMIO)
    pub data_5: ReadWrite<u32>,
    _padding88: [u8; 8],
    /// Input Data (GPIO Bank0, MIO)
    pub data_0_ro: ReadOnly<u32, Data0Ro::Register>,
    /// Input Data (GPIO Bank1, MIO)
    pub data_1_ro: ReadOnly<u32, Data1Ro::Register>,
    /// Input Data (GPIO Bank2, MIO)
    pub data_2_ro: ReadOnly<u32, Data2Ro::Register>,
    /// Input Data (GPIO Bank3, EMIO)
    pub data_3_ro: ReadOnly<u32>,
    /// Input Data (GPIO Bank4, EMIO)
    pub data_4_ro: ReadOnly<u32>,
    /// Input Data (GPIO Bank5, EMIO)
    pub data_5_ro: ReadOnly<u32>,
    _padding120: [u8; 396],
    /// Direction mode (GPIO Bank0, MIO)
    pub dirm_0: Aliased<u32, Dirm0R::Register, Dirm0W::Register>,
    /// Output enable (GPIO Bank0, MIO)
    pub oen_0: Aliased<u32, Oen0R::Register, Oen0W::Register>,
    /// Interrupt Mask Status (GPIO Bank0, MIO)
    pub int_mask_0: ReadOnly<u32, IntMask0::Register>,
    /// Interrupt Enable/Unmask (GPIO Bank0, MIO)
    pub int_en_0: Aliased<u32, IntEn0R::Register, IntEn0W::Register>,
    /// Interrupt Disable/Mask (GPIO Bank0, MIO)
    pub int_dis_0: Aliased<u32, IntDis0R::Register, IntDis0W::Register>,
    /// Interrupt Status (GPIO Bank0, MIO)
    pub int_stat_0: Aliased<u32, IntStat0R::Register, IntStat0W::Register>,
    /// Interrupt Type (GPIO Bank0, MIO)
    pub int_type_0: Aliased<u32, IntType0R::Register, IntType0W::Register>,
    /// Interrupt Polarity (GPIO Bank0, MIO)
    pub int_polarity_0: Aliased<u32, IntPolarity0R::Register, IntPolarity0W::Register>,
    /// Interrupt Any Edge Sensitive (GPIO Bank0, MIO)
    pub int_any_0: Aliased<u32, IntAny0R::Register, IntAny0W::Register>,
    _padding552: [u8; 28],
    /// Direction mode (GPIO Bank1, MIO)
    pub dirm_1: Aliased<u32, Dirm1R::Register, Dirm1W::Register>,
    /// Output enable (GPIO Bank1, MIO)
    pub oen_1: Aliased<u32, Oen1R::Register, Oen1W::Register>,
    /// Interrupt Mask Status (GPIO Bank1, MIO)
    pub int_mask_1: ReadOnly<u32, IntMask1::Register>,
    /// Interrupt Enable/Unmask (GPIO Bank1, MIO)
    pub int_en_1: Aliased<u32, IntEn1R::Register, IntEn1W::Register>,
    /// Interrupt Disable/Mask (GPIO Bank1, MIO)
    pub int_dis_1: Aliased<u32, IntDis1R::Register, IntDis1W::Register>,
    /// Interrupt Status (GPIO Bank1, MIO)
    pub int_stat_1: Aliased<u32, IntStat1R::Register, IntStat1W::Register>,
    /// Interrupt Type (GPIO Bank1, MIO)
    pub int_type_1: Aliased<u32, IntType1R::Register, IntType1W::Register>,
    /// Interrupt Polarity (GPIO Bank1, MIO)
    pub int_polarity_1: Aliased<u32, IntPolarity1R::Register, IntPolarity1W::Register>,
    /// Interrupt Any Edge Sensitive (GPIO Bank1, MIO)
    pub int_any_1: Aliased<u32, IntAny1R::Register, IntAny1W::Register>,
    _padding616: [u8; 28],
    /// Direction mode (GPIO Bank2, MIO)
    pub dirm_2: Aliased<u32, Dirm2R::Register, Dirm2W::Register>,
    /// Output enable (GPIO Bank2, MIO)
    pub oen_2: Aliased<u32, Oen2R::Register, Oen2W::Register>,
    /// Interrupt Mask Status (GPIO Bank2, MIO)
    pub int_mask_2: ReadOnly<u32, IntMask2::Register>,
    /// Interrupt Enable/Unmask (GPIO Bank2, MIO)
    pub int_en_2: Aliased<u32, IntEn2R::Register, IntEn2W::Register>,
    /// Interrupt Disable/Mask (GPIO Bank2, MIO)
    pub int_dis_2: Aliased<u32, IntDis2R::Register, IntDis2W::Register>,
    /// Interrupt Status (GPIO Bank2, MIO)
    pub int_stat_2: Aliased<u32, IntStat2R::Register, IntStat2W::Register>,
    /// Interrupt Type (GPIO Bank2, MIO)
    pub int_type_2: Aliased<u32, IntType2R::Register, IntType2W::Register>,
    /// Interrupt Polarity (GPIO Bank2, MIO)
    pub int_polarity_2: Aliased<u32, IntPolarity2R::Register, IntPolarity2W::Register>,
    /// Interrupt Any Edge Sensitive (GPIO Bank2, MIO)
    pub int_any_2: Aliased<u32, IntAny2R::Register, IntAny2W::Register>,
    _padding680: [u8; 28],
    /// Direction mode (GPIO Bank3, EMIO Bank0)
    pub dirm_3: ReadWrite<u32>,
    /// Output enable (GPIO Bank3, EMIO Bank0)
    pub oen_3: ReadWrite<u32>,
    /// Interrupt Mask Status (GPIO Bank3, EMIO Bank0)
    pub int_mask_3: ReadOnly<u32>,
    /// Interrupt Enable/Unmask (GPIO Bank3, EMIO Bank0)
    pub int_en_3: WriteOnly<u32>,
    /// Interrupt Disable/Mask (GPIO Bank3, EMIO Bank0)
    pub int_dis_3: WriteOnly<u32>,
    /// Interrupt Status (GPIO Bank3, EMIO Bank0)
    pub int_stat_3: ReadWrite<u32>,
    /// Interrupt Type (GPIO Bank3, EMIO Bank0)
    pub int_type_3: ReadWrite<u32>,
    /// Interrupt Polarity (GPIO Bank3, EMIO Bank0)
    pub int_polarity_3: ReadWrite<u32>,
    /// Interrupt Any Edge Sensitive (GPIO Bank3, EMIO Bank0)
    pub int_any_3: ReadWrite<u32>,
    _padding744: [u8; 28],
    /// Direction mode (GPIO Bank4, EMIO Bank1)
    pub dirm_4: ReadWrite<u32>,
    /// Output enable (GPIO Bank4, EMIO Bank1)
    pub oen_4: ReadWrite<u32>,
    /// Interrupt Mask Status (GPIO Bank4, EMIO Bank1)
    pub int_mask_4: ReadOnly<u32>,
    /// Interrupt Enable/Unmask (GPIO Bank4, EMIO Bank1)
    pub int_en_4: WriteOnly<u32>,
    /// Interrupt Disable/Mask (GPIO Bank4, EMIO Bank1)
    pub int_dis_4: WriteOnly<u32>,
    /// Interrupt Status (GPIO Bank4, EMIO Bank1)
    pub int_stat_4: ReadWrite<u32>,
    /// Interrupt Type (GPIO Bank4, EMIO Bank1)
    pub int_type_4: ReadWrite<u32>,
    /// Interrupt Polarity (GPIO Bank4, EMIO Bank1)
    pub int_polarity_4: ReadWrite<u32>,
    /// Interrupt Any Edge Sensitive (GPIO Bank4, EMIO Bank1)
    pub int_any_4: ReadWrite<u32>,
    _padding808: [u8; 28],
    /// Direction mode (GPIO Bank5, EMIO Bank2)
    pub dirm_5: ReadWrite<u32>,
    /// Output enable (GPIO Bank5, EMIO Bank2)
    pub oen_5: ReadWrite<u32>,
    /// Interrupt Mask Status (GPIO Bank5, EMIO Bank2)
    pub int_mask_5: ReadOnly<u32>,
    /// Interrupt Enable/Unmask (GPIO Bank5, EMIO Bank2)
    pub int_en_5: WriteOnly<u32>,
    /// Interrupt Disable/Mask (GPIO Bank5, EMIO Bank2)
    pub int_dis_5: WriteOnly<u32>,
    /// Interrupt Status (GPIO Bank5, EMIO Bank2)
    pub int_stat_5: ReadWrite<u32>,
    /// Interrupt Type (GPIO Bank5, EMIO Bank2)
    pub int_type_5: ReadWrite<u32>,
    /// Interrupt Polarity (GPIO Bank5, EMIO Bank2)
    pub int_polarity_5: ReadWrite<u32>,
    /// Interrupt Any Edge Sensitive (GPIO Bank5, EMIO Bank2)
    pub int_any_5: ReadWrite<u32>,
}
tock_registers::register_bitfields! [
    u32,
    pub MaskData0LswR [
        DATA_0_LSW OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskData0LswW [
        MASK_0_LSW OFFSET(16) NUMBITS(16) [],
        DATA_0_LSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData0MswR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        DATA_0_MSW OFFSET(0) NUMBITS(10) [],
    ],
    pub MaskData0MswW [
        MASK_0_MSW OFFSET(16) NUMBITS(10) [],
        DATA_0_MSW OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData1LswR [
        DATA_1_LSW OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskData1LswW [
        MASK_1_LSW OFFSET(16) NUMBITS(16) [],
        DATA_1_LSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData1MswR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        DATA_1_MSW OFFSET(0) NUMBITS(10) [],
    ],
    pub MaskData1MswW [
        MASK_1_MSW OFFSET(16) NUMBITS(10) [],
        DATA_1_MSW OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData2LswR [
        DATA_2_LSW OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskData2LswW [
        MASK_2_LSW OFFSET(16) NUMBITS(16) [],
        DATA_2_LSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData2MswR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        DATA_2_MSW OFFSET(0) NUMBITS(10) [],
    ],
    pub MaskData2MswW [
        MASK_2_MSW OFFSET(16) NUMBITS(10) [],
        DATA_2_MSW OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData3LswR [
        DATA_3_LSW OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskData3LswW [
        MASK_3_LSW OFFSET(16) NUMBITS(16) [],
        DATA_3_LSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData3MswR [
        DATA_3_MSW OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskData3MswW [
        MASK_3_MSW OFFSET(16) NUMBITS(16) [],
        DATA_3_MSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData4LswR [
        DATA_4_LSW OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskData4LswW [
        MASK_4_LSW OFFSET(16) NUMBITS(16) [],
        DATA_4_LSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData4MswR [
        DATA_4_MSW OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskData4MswW [
        MASK_4_MSW OFFSET(16) NUMBITS(16) [],
        DATA_4_MSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData5LswR [
        DATA_5_LSW OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskData5LswW [
        MASK_5_LSW OFFSET(16) NUMBITS(16) [],
        DATA_5_LSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskData5MswR [
        DATA_5_MSW OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskData5MswW [
        MASK_5_MSW OFFSET(16) NUMBITS(16) [],
        DATA_5_MSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Data0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        DATA_0 OFFSET(0) NUMBITS(26) [],
    ],
    pub Data0W [
        DATA_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Data1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        DATA_1 OFFSET(0) NUMBITS(26) [],
    ],
    pub Data1W [
        DATA_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Data2R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        DATA_2 OFFSET(0) NUMBITS(26) [],
    ],
    pub Data2W [
        DATA_2 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Data0Ro [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        DATA_0_RO OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Data1Ro [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        DATA_1_RO OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Data2Ro [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        DATA_2_RO OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dirm0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        DIRECTION_0 OFFSET(0) NUMBITS(26) [],
    ],
    pub Dirm0W [
        DIRECTION_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Oen0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        OP_ENABLE_0 OFFSET(0) NUMBITS(26) [],
    ],
    pub Oen0W [
        OP_ENABLE_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntMask0 [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_MASK_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntEn0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
    ],
    pub IntEn0W [
        INT_ENABLE_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntDis0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
    ],
    pub IntDis0W [
        INT_DISABLE_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntStat0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_STATUS_0 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntStat0W [
        INT_STATUS_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntType0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_TYPE_0 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntType0W [
        INT_TYPE_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntPolarity0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_POL_0 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntPolarity0W [
        INT_POL_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntAny0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_ON_ANY_0 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntAny0W [
        INT_ON_ANY_0 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dirm1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        DIRECTION_1 OFFSET(0) NUMBITS(26) [],
    ],
    pub Dirm1W [
        DIRECTION_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Oen1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        OP_ENABLE_1 OFFSET(0) NUMBITS(26) [],
    ],
    pub Oen1W [
        OP_ENABLE_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntMask1 [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_MASK_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntEn1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
    ],
    pub IntEn1W [
        INT_ENABLE_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntDis1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
    ],
    pub IntDis1W [
        INT_DISABLE_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntStat1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_STATUS_1 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntStat1W [
        INT_STATUS_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntType1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_TYPE_1 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntType1W [
        INT_TYPE_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntPolarity1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_POL_1 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntPolarity1W [
        INT_POL_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntAny1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_ON_ANY_1 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntAny1W [
        INT_ON_ANY_1 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dirm2R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        DIRECTION_2 OFFSET(0) NUMBITS(26) [],
    ],
    pub Dirm2W [
        DIRECTION_2 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Oen2R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        OP_ENABLE_2 OFFSET(0) NUMBITS(26) [],
    ],
    pub Oen2W [
        OP_ENABLE_2 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntMask2 [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_MASK_2 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntEn2R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
    ],
    pub IntEn2W [
        INT_ENABLE_2 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntDis2R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
    ],
    pub IntDis2W [
        INT_DISABLE_2 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntStat2R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_STATUS_2 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntStat2W [
        INT_STATUS_2 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntType2R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_TYPE_2 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntType2W [
        INT_TYPE_2 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntPolarity2R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_POL_2 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntPolarity2W [
        INT_POL_2 OFFSET(0) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntAny2R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        INT_ON_ANY_2 OFFSET(0) NUMBITS(26) [],
    ],
    pub IntAny2W [
        INT_ON_ANY_2 OFFSET(0) NUMBITS(26) [],
    ]
];

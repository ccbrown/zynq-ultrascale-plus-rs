// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, WriteOnly};
/// PMU RAM, PMU Local Memory Bus RAM
pub static mut PMU_LMB_RAM: *mut Registers = 0xffd50000 as *mut Registers;
register_structs! {
    pub Registers {
        /// ECC Status
        (0x00000000 => pub ecc_status: Aliased<u32, EccStatusR::Register, EccStatusW::Register>),
        /// ECC Enable Interrupt
        (0x00000004 => pub ecc_en_irq: Aliased<u32, EccEnIrqR::Register, EccEnIrqW::Register>),
        /// ECC On-Off Control
        (0x00000008 => pub ecc_onoff: Aliased<u32, EccOnoffR::Register, EccOnoffW::Register>),
        /// Correctable Error Counter
        (0x0000000c => pub ce_cnt: Aliased<u32, CeCntR::Register, CeCntW::Register>),
        (0x00000010 => _padding16),
        /// Correctable Error First Failing Data
        (0x00000100 => pub ce_ffd: ReadOnly<u32>),
        (0x00000104 => _padding260),
        /// Correctable Error First Failing ECC
        (0x00000180 => pub ce_ffe: ReadOnly<u32, CeFfe::Register>),
        (0x00000184 => _padding388),
        /// Correctable Error First Failing Address
        (0x000001c0 => pub ce_ffa: ReadOnly<u32>),
        (0x000001c4 => _padding452),
        /// Uncorrectable Error First Failing Data
        (0x00000200 => pub ue_ffd: ReadOnly<u32>),
        (0x00000204 => _padding516),
        /// Uncorrectable Error First Failing ECC
        (0x00000280 => pub ue_ffe: ReadOnly<u32, UeFfe::Register>),
        (0x00000284 => _padding644),
        /// Uncorrectable Error First Failing Address Register
        (0x000002c0 => pub ue_ffa: ReadOnly<u32>),
        (0x000002c4 => _padding708),
        /// Fault Inject Data
        (0x00000300 => pub fi_d: WriteOnly<u32>),
        (0x00000304 => _padding772),
        /// Fault Inject ECC
        (0x00000380 => pub fi_ecc: Aliased<u32, FiEccR::Register, FiEccW::Register>),
        (0x00000384 => @END),
    }
}
register_bitfields! [
    u32,
    pub EccStatusR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        CE_STATUS OFFSET(1) NUMBITS(1) [],
        UE_STATUS OFFSET(0) NUMBITS(1) [],
    ],
    pub EccStatusW [
        CE_STATUS OFFSET(1) NUMBITS(1) [],
        UE_STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EccEnIrqR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        CE_EN_IRQ OFFSET(1) NUMBITS(1) [],
        UE_EN_IRQ OFFSET(0) NUMBITS(1) [],
    ],
    pub EccEnIrqW [
        CE_EN_IRQ OFFSET(1) NUMBITS(1) [],
        UE_EN_IRQ OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EccOnoffR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ECC_ONOFF OFFSET(0) NUMBITS(1) [],
    ],
    pub EccOnoffW [
        ECC_ONOFF OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub CeCntR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CE_CNT OFFSET(0) NUMBITS(16) [],
    ],
    pub CeCntW [
        CE_CNT OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub CeFfe [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        CE_FFE OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub UeFfe [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        UE_FFE OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub FiEccR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
    ],
    pub FiEccW [
        FI_ECC OFFSET(0) NUMBITS(8) [],
    ]
];

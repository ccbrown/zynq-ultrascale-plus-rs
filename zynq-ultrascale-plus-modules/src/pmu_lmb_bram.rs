// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, WriteOnly};
/// PMU RAM, PMU Local Memory Bus RAM
pub static mut PMU_LMB_RAM: *mut Registers = 0xffd50000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// ECC Status
    pub ecc_status: Aliased<u32, EccStatusR::Register, EccStatusW::Register>,
    /// ECC Enable Interrupt
    pub ecc_en_irq: Aliased<u32, EccEnIrqR::Register, EccEnIrqW::Register>,
    /// ECC On-Off Control
    pub ecc_onoff: Aliased<u32, EccOnoffR::Register, EccOnoffW::Register>,
    /// Correctable Error Counter
    pub ce_cnt: Aliased<u32, CeCntR::Register, CeCntW::Register>,
    _padding16: [u8; 240],
    /// Correctable Error First Failing Data
    pub ce_ffd: ReadOnly<u32>,
    _padding260: [u8; 124],
    /// Correctable Error First Failing ECC
    pub ce_ffe: ReadOnly<u32, CeFfe::Register>,
    _padding388: [u8; 60],
    /// Correctable Error First Failing Address
    pub ce_ffa: ReadOnly<u32>,
    _padding452: [u8; 60],
    /// Uncorrectable Error First Failing Data
    pub ue_ffd: ReadOnly<u32>,
    _padding516: [u8; 124],
    /// Uncorrectable Error First Failing ECC
    pub ue_ffe: ReadOnly<u32, UeFfe::Register>,
    _padding644: [u8; 60],
    /// Uncorrectable Error First Failing Address Register
    pub ue_ffa: ReadOnly<u32>,
    _padding708: [u8; 60],
    /// Fault Inject Data
    pub fi_d: WriteOnly<u32>,
    _padding772: [u8; 124],
    /// Fault Inject ECC
    pub fi_ecc: Aliased<u32, FiEccR::Register, FiEccW::Register>,
}
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub EccOnoffR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ECC_ONOFF OFFSET(0) NUMBITS(1) [],
    ],
    pub EccOnoffW [
        ECC_ONOFF OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CeCntR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CE_CNT OFFSET(0) NUMBITS(16) [],
    ],
    pub CeCntW [
        CE_CNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CeFfe [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        CE_FFE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub UeFfe [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        UE_FFE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub FiEccR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
    ],
    pub FiEccW [
        FI_ECC OFFSET(0) NUMBITS(8) [],
    ]
];

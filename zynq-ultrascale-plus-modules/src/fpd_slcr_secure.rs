// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// FPD System-level Control Registers, FPD System-level Control - Secure
pub static mut FPD_SLCR_SECURE: *mut Registers = 0xfd690000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 4],
    /// General control register for the LPD SLCR SECURE.
    pub ctrl: ReadWrite<u8, Ctrl::Register>,
    _padding5: [u8; 3],
    /// Interrupt Status and clear.
    pub isr: ReadWrite<u8, Isr::Register>,
    _padding9: [u8; 3],
    /// Interrupt Mask
    pub imr: ReadOnly<u8, Imr::Register>,
    _padding13: [u8; 3],
    /// Interrupt Enable
    pub ier: WriteOnly<u8, Ier::Register>,
    _padding17: [u8; 3],
    /// Interrupt Disable
    pub idr: WriteOnly<u8, Idr::Register>,
    _padding21: [u8; 3],
    /// Interrupt Trigger
    pub itr: WriteOnly<u8, Itr::Register>,
    _padding25: [u8; 7],
    /// SATA TrustZone settings.
    pub slcr_sata: Aliased<u32, SlcrSataR::Register, SlcrSataW::Register>,
    _padding36: [u8; 12],
    /// PCIe TrustZone settings.
    pub slcr_pcie: Aliased<u32, SlcrPcieR::Register, SlcrPcieW::Register>,
    _padding52: [u8; 12],
    /// DisplayPort DMA TrustZone Setting.
    pub slcr_dpdma: Aliased<u32, SlcrDpdmaR::Register, SlcrDpdmaW::Register>,
    _padding68: [u8; 12],
    /// FPD DMA Trustzone Settings.
    pub slcr_gdma: ReadWrite<u8, SlcrGdma::Register>,
    _padding81: [u8; 15],
    /// APU GIC setting.
    pub slcr_gic: ReadWrite<u8, SlcrGic::Register>,
}
tock_registers::register_bitfields! [
    u8,
    pub Ctrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Isr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Imr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Ier [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Idr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Itr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SlcrSataR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        TZ_AXIMDMA1 OFFSET(3) NUMBITS(1) [],
        TZ_AXIMDMA0 OFFSET(2) NUMBITS(1) [],
        TZ_AXIS OFFSET(1) NUMBITS(1) [],
        TZ_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub SlcrSataW [
        TZ_AXIMDMA1 OFFSET(3) NUMBITS(1) [],
        TZ_AXIMDMA0 OFFSET(2) NUMBITS(1) [],
        TZ_AXIS OFFSET(1) NUMBITS(1) [],
        TZ_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SlcrPcieR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TZ_DMA_3 OFFSET(24) NUMBITS(1) [],
        TZ_DMA_2 OFFSET(23) NUMBITS(1) [],
        TZ_DMA_1 OFFSET(22) NUMBITS(1) [],
        TZ_DMA_0 OFFSET(21) NUMBITS(1) [],
        TZ_AT_INGR_7 OFFSET(20) NUMBITS(1) [],
        TZ_AT_INGR_6 OFFSET(19) NUMBITS(1) [],
        TZ_AT_INGR_5 OFFSET(18) NUMBITS(1) [],
        TZ_AT_INGR_4 OFFSET(17) NUMBITS(1) [],
        TZ_AT_INGR_3 OFFSET(16) NUMBITS(1) [],
        TZ_AT_INGR_2 OFFSET(15) NUMBITS(1) [],
        TZ_AT_INGR_1 OFFSET(14) NUMBITS(1) [],
        TZ_AT_INGR_0 OFFSET(13) NUMBITS(1) [],
        TZ_AT_EGR_7 OFFSET(12) NUMBITS(1) [],
        TZ_AT_EGR_6 OFFSET(11) NUMBITS(1) [],
        TZ_AT_EGR_5 OFFSET(10) NUMBITS(1) [],
        TZ_AT_EGR_4 OFFSET(9) NUMBITS(1) [],
        TZ_AT_EGR_3 OFFSET(8) NUMBITS(1) [],
        TZ_AT_EGR_2 OFFSET(7) NUMBITS(1) [],
        TZ_AT_EGR_1 OFFSET(6) NUMBITS(1) [],
        TZ_AT_EGR_0 OFFSET(5) NUMBITS(1) [],
        TZ_DMA_REGS OFFSET(4) NUMBITS(1) [],
        TZ_MSIX_PBA OFFSET(3) NUMBITS(1) [],
        TZ_MSIX_TABLE OFFSET(2) NUMBITS(1) [],
        TZ_ECAM OFFSET(1) NUMBITS(1) [],
        TZ_BRIDGE_REGS OFFSET(0) NUMBITS(1) [],
    ],
    pub SlcrPcieW [
        TZ_DMA_3 OFFSET(24) NUMBITS(1) [],
        TZ_DMA_2 OFFSET(23) NUMBITS(1) [],
        TZ_DMA_1 OFFSET(22) NUMBITS(1) [],
        TZ_DMA_0 OFFSET(21) NUMBITS(1) [],
        TZ_AT_INGR_7 OFFSET(20) NUMBITS(1) [],
        TZ_AT_INGR_6 OFFSET(19) NUMBITS(1) [],
        TZ_AT_INGR_5 OFFSET(18) NUMBITS(1) [],
        TZ_AT_INGR_4 OFFSET(17) NUMBITS(1) [],
        TZ_AT_INGR_3 OFFSET(16) NUMBITS(1) [],
        TZ_AT_INGR_2 OFFSET(15) NUMBITS(1) [],
        TZ_AT_INGR_1 OFFSET(14) NUMBITS(1) [],
        TZ_AT_INGR_0 OFFSET(13) NUMBITS(1) [],
        TZ_AT_EGR_7 OFFSET(12) NUMBITS(1) [],
        TZ_AT_EGR_6 OFFSET(11) NUMBITS(1) [],
        TZ_AT_EGR_5 OFFSET(10) NUMBITS(1) [],
        TZ_AT_EGR_4 OFFSET(9) NUMBITS(1) [],
        TZ_AT_EGR_3 OFFSET(8) NUMBITS(1) [],
        TZ_AT_EGR_2 OFFSET(7) NUMBITS(1) [],
        TZ_AT_EGR_1 OFFSET(6) NUMBITS(1) [],
        TZ_AT_EGR_0 OFFSET(5) NUMBITS(1) [],
        TZ_DMA_REGS OFFSET(4) NUMBITS(1) [],
        TZ_MSIX_PBA OFFSET(3) NUMBITS(1) [],
        TZ_MSIX_TABLE OFFSET(2) NUMBITS(1) [],
        TZ_ECAM OFFSET(1) NUMBITS(1) [],
        TZ_BRIDGE_REGS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SlcrDpdmaR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        TZ OFFSET(0) NUMBITS(1) [],
    ],
    pub SlcrDpdmaW [
        TZ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub SlcrGdma [
        TZ OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub SlcrGic [
        CFG_DISABLE OFFSET(0) NUMBITS(1) [],
    ]
];

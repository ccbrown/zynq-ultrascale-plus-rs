// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// FPD System-level Control Registers, FPD System-level Control - Secure
pub static mut FPD_SLCR_SECURE: *mut Registers = 0xfd690000 as *mut Registers;
register_structs! {
    pub Registers {
        (0x00000000 => _padding0),
        /// General control register for the LPD SLCR SECURE.
        (0x00000004 => pub ctrl: ReadWrite<u8, Ctrl::Register>),
        (0x00000005 => _padding5),
        /// Interrupt Status and clear.
        (0x00000008 => pub isr: ReadWrite<u8, Isr::Register>),
        (0x00000009 => _padding9),
        /// Interrupt Mask
        (0x0000000c => pub imr: ReadOnly<u8, Imr::Register>),
        (0x0000000d => _padding13),
        /// Interrupt Enable
        (0x00000010 => pub ier: WriteOnly<u8, Ier::Register>),
        (0x00000011 => _padding17),
        /// Interrupt Disable
        (0x00000014 => pub idr: WriteOnly<u8, Idr::Register>),
        (0x00000015 => _padding21),
        /// Interrupt Trigger
        (0x00000018 => pub itr: WriteOnly<u8, Itr::Register>),
        (0x00000019 => _padding25),
        /// SATA TrustZone settings.
        (0x00000020 => pub slcr_sata: Aliased<u32, SlcrSataR::Register, SlcrSataW::Register>),
        (0x00000024 => _padding36),
        /// PCIe TrustZone settings.
        (0x00000030 => pub slcr_pcie: Aliased<u32, SlcrPcieR::Register, SlcrPcieW::Register>),
        (0x00000034 => _padding52),
        /// DisplayPort DMA TrustZone Setting.
        (0x00000040 => pub slcr_dpdma: Aliased<u32, SlcrDpdmaR::Register, SlcrDpdmaW::Register>),
        (0x00000044 => _padding68),
        /// FPD DMA Trustzone Settings.
        (0x00000050 => pub slcr_gdma: ReadWrite<u8, SlcrGdma::Register>),
        (0x00000051 => _padding81),
        /// APU GIC setting.
        (0x00000060 => pub slcr_gic: ReadWrite<u8, SlcrGic::Register>),
        (0x00000061 => @END),
    }
}
register_bitfields! [
    u8,
    pub Ctrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Isr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Imr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Ier [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Idr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Itr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
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
register_bitfields! [
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
register_bitfields! [
    u32,
    pub SlcrDpdmaR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        TZ OFFSET(0) NUMBITS(1) [],
    ],
    pub SlcrDpdmaW [
        TZ OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub SlcrGdma [
        TZ OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub SlcrGic [
        CFG_DISABLE OFFSET(0) NUMBITS(1) [],
    ]
];

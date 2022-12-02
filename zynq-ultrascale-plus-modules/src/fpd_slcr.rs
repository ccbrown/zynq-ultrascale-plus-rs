// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// FPD System-level Control Registers, FPD System-level Control
pub static mut FPD_SLCR: *mut Registers = 0xfd610000 as *mut Registers;
register_structs! {
    pub Registers {
        /// FP Domain SLCR Write protection register
        (0x00000000 => pub wprot0: ReadWrite<u8, Wprot0::Register>),
        (0x00000001 => _padding1),
        /// General control register for the FP Domain SLCR
        (0x00000004 => pub ctrl: ReadWrite<u8, Ctrl::Register>),
        (0x00000005 => _padding5),
        /// Interrupt Status Register
        (0x00000008 => pub isr: ReadWrite<u8, Isr::Register>),
        (0x00000009 => _padding9),
        /// Interrupt Mask Register
        (0x0000000c => pub imr: ReadOnly<u8, Imr::Register>),
        (0x0000000d => _padding13),
        /// Interrupt Enable Register
        (0x00000010 => pub ier: WriteOnly<u8, Ier::Register>),
        (0x00000011 => _padding17),
        /// Interrupt Disable Register
        (0x00000014 => pub idr: WriteOnly<u8, Idr::Register>),
        (0x00000015 => _padding21),
        /// Interrupt Trigger Register
        (0x00000018 => pub itr: WriteOnly<u8, Itr::Register>),
        (0x00000019 => _padding25),
        /// FPD SWDT clock source select (WDT)
        (0x00000100 => pub wdt_clk_sel: Aliased<u32, WdtClkSelR::Register, WdtClkSelW::Register>),
        (0x00000104 => _padding260),
        /// Interconnect Clock Source Select
        (0x00000200 => pub int_fpd: Aliased<u32, IntFpdR::Register, IntFpdW::Register>),
        (0x00000204 => _padding516),
        /// GPU Idle status Register
        (0x0000100c => pub gpu: Aliased<u32, GpuR::Register, GpuW::Register>),
        (0x00001010 => _padding4112),
        /// GDMA RF2 Configuation
        (0x00003000 => pub gdma_cfg: ReadOnly<u8, GdmaCfg::Register>),
        (0x00003001 => _padding12289),
        /// RAM control register
        (0x00003010 => pub gdma_ram: Aliased<u16, GdmaRamR::Register, GdmaRamW::Register>),
        (0x00003012 => _padding12306),
        /// afi fs SLCR control register. This register is static and should not be modified during operation.
        (0x00005000 => pub afi_fs: Aliased<u32, AfiFsR::Register, AfiFsW::Register>),
        (0x00005004 => _padding20484),
        /// Interrupt Status.
        (0x00006000 => pub err_atb_isr: Aliased<u32, ErrAtbIsrR::Register, ErrAtbIsrW::Register>),
        /// Interrupt Mask.
        (0x00006004 => pub err_atb_imr: ReadOnly<u32, ErrAtbImr::Register>),
        /// Interrupt Enable.
        (0x00006008 => pub err_atb_ier: Aliased<u32, ErrAtbIerR::Register, ErrAtbIerW::Register>),
        /// Interrupt Disable.
        (0x0000600c => pub err_atb_idr: Aliased<u32, ErrAtbIdrR::Register, ErrAtbIdrW::Register>),
        /// Timeout Enable.
        (0x00006010 => pub atb_cmd_store_en: Aliased<u32, AtbCmdStoreEnR::Register, AtbCmdStoreEnW::Register>),
        /// AXI Response Enables.
        (0x00006014 => pub atb_resp_en: Aliased<u32, AtbRespEnR::Register, AtbRespEnW::Register>),
        /// Timed-out AXI Response Selection.
        (0x00006018 => pub atb_resp_type: Aliased<u32, AtbRespTypeR::Register, AtbRespTypeW::Register>),
        (0x0000601c => _padding24604),
        /// Prescalar and Enable
        (0x00006020 => pub atb_prescale: Aliased<u32, AtbPrescaleR::Register, AtbPrescaleW::Register>),
        (0x00006024 => @END),
    }
}
register_bitfields! [
    u8,
    pub Wprot0 [
        ACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
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
    pub WdtClkSelR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SELECT OFFSET(0) NUMBITS(1) [],
    ],
    pub WdtClkSelW [
        SELECT OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IntFpdR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GFM_SEL OFFSET(0) NUMBITS(1) [],
    ],
    pub IntFpdW [
        GFM_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub GpuR [
        ARCACHE OFFSET(7) NUMBITS(4) [],
        AWCACHE OFFSET(3) NUMBITS(4) [],
        PP1_IDLE OFFSET(2) NUMBITS(1) [],
        PP0_IDLE OFFSET(1) NUMBITS(1) [],
        IDLE OFFSET(0) NUMBITS(1) [],
    ],
    pub GpuW [
        ARCACHE OFFSET(7) NUMBITS(4) [],
        AWCACHE OFFSET(3) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u8,
    pub GdmaCfg [
        BUS_WIDTH OFFSET(5) NUMBITS(2) [],
        NUM_CH OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u16,
    pub GdmaRamR [
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(12) NUMBITS(3) [],
        RESERVED2 OFFSET(11) NUMBITS(1) [],
        RESERVED3 OFFSET(8) NUMBITS(3) [],
        RESERVED4 OFFSET(7) NUMBITS(1) [],
        RESERVED5 OFFSET(4) NUMBITS(3) [],
        RESERVED6 OFFSET(3) NUMBITS(1) [],
        RESERVED7 OFFSET(0) NUMBITS(3) [],
    ],
    pub GdmaRamW [
        RESERVED0 OFFSET(12) NUMBITS(3) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        RESERVED2 OFFSET(8) NUMBITS(3) [],
        RESERVED3 OFFSET(7) NUMBITS(1) [],
        RESERVED4 OFFSET(4) NUMBITS(3) [],
        RESERVED5 OFFSET(3) NUMBITS(1) [],
        RESERVED6 OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub AfiFsR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        DW_SS1_SEL OFFSET(10) NUMBITS(2) [],
        DW_SS0_SEL OFFSET(8) NUMBITS(2) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ],
    pub AfiFsW [
        DW_SS1_SEL OFFSET(10) NUMBITS(2) [],
        DW_SS0_SEL OFFSET(8) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub ErrAtbIsrR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ],
    pub ErrAtbIsrW [
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ErrAtbImr [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ErrAtbIerR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
    ],
    pub ErrAtbIerW [
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ErrAtbIdrR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
    ],
    pub ErrAtbIdrW [
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub AtbCmdStoreEnR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ],
    pub AtbCmdStoreEnW [
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub AtbRespEnR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ],
    pub AtbRespEnW [
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub AtbRespTypeR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ],
    pub AtbRespTypeW [
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub AtbPrescaleR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        ENABLE OFFSET(16) NUMBITS(1) [],
        VALUE OFFSET(0) NUMBITS(16) [],
    ],
    pub AtbPrescaleW [
        ENABLE OFFSET(16) NUMBITS(1) [],
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];

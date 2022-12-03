// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// FPD System-level Control Registers, FPD System-level Control
pub static mut FPD_SLCR: *mut Registers = 0xfd610000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// FP Domain SLCR Write protection register
    pub wprot0: ReadWrite<u8, Wprot0::Register>,
    _padding1: [u8; 3],
    /// General control register for the FP Domain SLCR
    pub ctrl: ReadWrite<u8, Ctrl::Register>,
    _padding5: [u8; 3],
    /// Interrupt Status Register
    pub isr: ReadWrite<u8, Isr::Register>,
    _padding9: [u8; 3],
    /// Interrupt Mask Register
    pub imr: ReadOnly<u8, Imr::Register>,
    _padding13: [u8; 3],
    /// Interrupt Enable Register
    pub ier: WriteOnly<u8, Ier::Register>,
    _padding17: [u8; 3],
    /// Interrupt Disable Register
    pub idr: WriteOnly<u8, Idr::Register>,
    _padding21: [u8; 3],
    /// Interrupt Trigger Register
    pub itr: WriteOnly<u8, Itr::Register>,
    _padding25: [u8; 231],
    /// FPD SWDT clock source select (WDT)
    pub wdt_clk_sel: Aliased<u32, WdtClkSelR::Register, WdtClkSelW::Register>,
    _padding260: [u8; 252],
    /// Interconnect Clock Source Select
    pub int_fpd: Aliased<u32, IntFpdR::Register, IntFpdW::Register>,
    _padding516: [u8; 3592],
    /// GPU Idle status Register
    pub gpu: Aliased<u32, GpuR::Register, GpuW::Register>,
    _padding4112: [u8; 8176],
    /// GDMA RF2 Configuation
    pub gdma_cfg: ReadOnly<u8, GdmaCfg::Register>,
    _padding12289: [u8; 15],
    /// RAM control register
    pub gdma_ram: Aliased<u16, GdmaRamR::Register, GdmaRamW::Register>,
    _padding12306: [u8; 8174],
    /// afi fs SLCR control register. This register is static and should not be modified during operation.
    pub afi_fs: Aliased<u32, AfiFsR::Register, AfiFsW::Register>,
    _padding20484: [u8; 4092],
    /// Interrupt Status.
    pub err_atb_isr: Aliased<u32, ErrAtbIsrR::Register, ErrAtbIsrW::Register>,
    /// Interrupt Mask.
    pub err_atb_imr: ReadOnly<u32, ErrAtbImr::Register>,
    /// Interrupt Enable.
    pub err_atb_ier: Aliased<u32, ErrAtbIerR::Register, ErrAtbIerW::Register>,
    /// Interrupt Disable.
    pub err_atb_idr: Aliased<u32, ErrAtbIdrR::Register, ErrAtbIdrW::Register>,
    /// Timeout Enable.
    pub atb_cmd_store_en: Aliased<u32, AtbCmdStoreEnR::Register, AtbCmdStoreEnW::Register>,
    /// AXI Response Enables.
    pub atb_resp_en: Aliased<u32, AtbRespEnR::Register, AtbRespEnW::Register>,
    /// Timed-out AXI Response Selection.
    pub atb_resp_type: Aliased<u32, AtbRespTypeR::Register, AtbRespTypeW::Register>,
    _padding24604: [u8; 4],
    /// Prescalar and Enable
    pub atb_prescale: Aliased<u32, AtbPrescaleR::Register, AtbPrescaleW::Register>,
}
tock_registers::register_bitfields! [
    u8,
    pub Wprot0 [
        ACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
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
    pub WdtClkSelR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SELECT OFFSET(0) NUMBITS(1) [],
    ],
    pub WdtClkSelW [
        SELECT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntFpdR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GFM_SEL OFFSET(0) NUMBITS(1) [],
    ],
    pub IntFpdW [
        GFM_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u8,
    pub GdmaCfg [
        BUS_WIDTH OFFSET(5) NUMBITS(2) [],
        NUM_CH OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub ErrAtbImr [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        AFIFS1 OFFSET(2) NUMBITS(1) [],
        AFIFS0 OFFSET(1) NUMBITS(1) [],
        FPDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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

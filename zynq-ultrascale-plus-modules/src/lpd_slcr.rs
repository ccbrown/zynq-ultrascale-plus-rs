// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// Low-power Domain SLCR, Low-power Domain System-level Control Registers
pub static mut LPD_SLCR: *mut Registers = 0xff410000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 4],
    /// General control register for the LP SLCR
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
    _padding25: [u8; 7],
    /// This register is never reset, even through POR.
    pub persistent0: ReadWrite<u32>,
    /// This register is never reset, even through POR.
    pub persistent1: ReadWrite<u32>,
    /// This register is never reset, even through POR.
    pub persistent2: ReadWrite<u32>,
    /// This register is never reset, even through POR.
    pub persistent3: ReadWrite<u32>,
    /// This register is never reset, even through POR.
    pub persistent4: ReadWrite<u32>,
    /// This register is never reset, even through POR.
    pub persistent5: ReadWrite<u32>,
    /// This register is never reset, even through POR.
    pub persistent6: ReadWrite<u32>,
    /// This register is never reset, even through POR.
    pub persistent7: ReadWrite<u32>,
    /// Safety endpoint connectivity check Register
    pub safety_chk0: ReadWrite<u32>,
    /// Safety endpoint connectivity check Register
    pub safety_chk1: ReadWrite<u32>,
    /// Safety endpoint connectivity check Register
    pub safety_chk2: ReadWrite<u32>,
    /// Safety endpoint connectivity check Register
    pub safety_chk3: ReadWrite<u32>,
    /// SWDT clock source select
    pub csupmu_wdt_clk_sel: Aliased<u32, CsupmuWdtClkSelR::Register, CsupmuWdtClkSelW::Register>,
    _padding84: [u8; 8120],
    /// GDMA RF2 Configuation
    pub adma_cfg: ReadOnly<u8, AdmaCfg::Register>,
    _padding8205: [u8; 3],
    /// RAM control register
    pub adma_ram: Aliased<u16, AdmaRamR::Register, AdmaRamW::Register>,
    _padding8210: [u8; 12270],
    /// afi fs SLCR control register. Do not change the bits durin
    pub afi_fs: Aliased<u16, AfiFsR::Register, AfiFsW::Register>,
    _padding20482: [u8; 4094],
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub err_atb_isr: Aliased<u32, ErrAtbIsrR::Register, ErrAtbIsrW::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub err_atb_imr: ReadOnly<u32, ErrAtbImr::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub err_atb_ier: Aliased<u32, ErrAtbIerR::Register, ErrAtbIerW::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub err_atb_idr: Aliased<u32, ErrAtbIdrR::Register, ErrAtbIdrW::Register>,
    /// ATB Sideband Signals
    pub atb_cmd_store_en: Aliased<u32, AtbCmdStoreEnR::Register, AtbCmdStoreEnW::Register>,
    /// ATB Sideband Signals
    pub atb_resp_en: Aliased<u32, AtbRespEnR::Register, AtbRespEnW::Register>,
    /// ATB Sideband Signals
    pub atb_resp_type: Aliased<u32, AtbRespTypeR::Register, AtbRespTypeW::Register>,
    _padding24604: [u8; 4],
    /// ATB Sideband Signals
    pub atb_prescale: Aliased<u32, AtbPrescaleR::Register, AtbPrescaleW::Register>,
    _padding24612: [u8; 8156],
    /// GIC Proxy Interrupt Status (1/2)
    pub gicp0_irq_status: ReadWrite<u32, Gicp0IrqStatus::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub gicp0_irq_mask: ReadOnly<u32, Gicp0IrqMask::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub gicp0_irq_enable: WriteOnly<u32, Gicp0IrqEnable::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub gicp0_irq_disable: WriteOnly<u32, Gicp0IrqDisable::Register>,
    /// Interrupt Trigger Register. A write of one to this location will set the interrupt status register related to this interrupt.
    pub gicp0_irq_trigger: WriteOnly<u32, Gicp0IrqTrigger::Register>,
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub gicp1_irq_status: ReadWrite<u32, Gicp1IrqStatus::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub gicp1_irq_mask: ReadOnly<u32, Gicp1IrqMask::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub gicp1_irq_enable: WriteOnly<u32, Gicp1IrqEnable::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub gicp1_irq_disable: WriteOnly<u32, Gicp1IrqDisable::Register>,
    /// Interrupt Trigger Register. A write of one to this location will set the interrupt status register related to this interrupt.
    pub gicp1_irq_trigger: WriteOnly<u32, Gicp1IrqTrigger::Register>,
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub gicp2_irq_status: ReadWrite<u32, Gicp2IrqStatus::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub gicp2_irq_mask: ReadOnly<u32, Gicp2IrqMask::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub gicp2_irq_enable: WriteOnly<u32, Gicp2IrqEnable::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub gicp2_irq_disable: WriteOnly<u32, Gicp2IrqDisable::Register>,
    /// Interrupt Trigger Register. A write of one to this location will set the interrupt status register related to this interrupt.
    pub gicp2_irq_trigger: WriteOnly<u32, Gicp2IrqTrigger::Register>,
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub gicp3_irq_status: ReadWrite<u32, Gicp3IrqStatus::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub gicp3_irq_mask: ReadOnly<u32, Gicp3IrqMask::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub gicp3_irq_enable: WriteOnly<u32, Gicp3IrqEnable::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub gicp3_irq_disable: WriteOnly<u32, Gicp3IrqDisable::Register>,
    /// Interrupt Trigger Register. A write of one to this location will set the interrupt status register related to this interrupt.
    pub gicp3_irq_trigger: WriteOnly<u32, Gicp3IrqTrigger::Register>,
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub gicp4_irq_status: ReadWrite<u32, Gicp4IrqStatus::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub gicp4_irq_mask: ReadOnly<u32, Gicp4IrqMask::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub gicp4_irq_enable: WriteOnly<u32, Gicp4IrqEnable::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub gicp4_irq_disable: WriteOnly<u32, Gicp4IrqDisable::Register>,
    /// Interrupt Trigger Register. A write of one to this location will set the interrupt status register related to this interrupt.
    pub gicp4_irq_trigger: WriteOnly<u32, Gicp4IrqTrigger::Register>,
    _padding32868: [u8; 60],
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub gicp_pmu_irq_status: Aliased<u32, GicpPmuIrqStatusR::Register, GicpPmuIrqStatusW::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub gicp_pmu_irq_mask: ReadOnly<u32, GicpPmuIrqMask::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub gicp_pmu_irq_enable: Aliased<u32, GicpPmuIrqEnableR::Register, GicpPmuIrqEnableW::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub gicp_pmu_irq_disable:
        Aliased<u32, GicpPmuIrqDisableR::Register, GicpPmuIrqDisableW::Register>,
    /// Interrupt Trigger Register. A write of one to this location will set the interrupt status register related to this interrupt.
    pub gicp_pmu_irq_trigger:
        Aliased<u32, GicpPmuIrqTriggerR::Register, GicpPmuIrqTriggerW::Register>,
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
    pub CsupmuWdtClkSelR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SELECT OFFSET(0) NUMBITS(1) [],
    ],
    pub CsupmuWdtClkSelW [
        SELECT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub AdmaCfg [
        BUS_WIDTH OFFSET(5) NUMBITS(2) [],
        NUM_CH OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub AdmaRamR [
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(12) NUMBITS(3) [],
        RESERVED2 OFFSET(11) NUMBITS(1) [],
        RESERVED3 OFFSET(8) NUMBITS(3) [],
        RESERVED4 OFFSET(7) NUMBITS(1) [],
        RESERVED5 OFFSET(4) NUMBITS(3) [],
        RESERVED6 OFFSET(3) NUMBITS(1) [],
        RESERVED7 OFFSET(0) NUMBITS(3) [],
    ],
    pub AdmaRamW [
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
    u16,
    pub AfiFsR [
        RESERVED0 OFFSET(10) NUMBITS(6) [],
        DW_SS2_SEL OFFSET(8) NUMBITS(2) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ],
    pub AfiFsW [
        DW_SS2_SEL OFFSET(8) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrAtbIsrR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ],
    pub ErrAtbIsrW [
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrAtbImr [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrAtbIerR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub ErrAtbIerW [
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrAtbIdrR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub ErrAtbIdrW [
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AtbCmdStoreEnR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ],
    pub AtbCmdStoreEnW [
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AtbRespEnR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ],
    pub AtbRespEnW [
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AtbRespTypeR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
    ],
    pub AtbRespTypeW [
        AFIFS2 OFFSET(1) NUMBITS(1) [],
        LPDM OFFSET(0) NUMBITS(1) [],
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
tock_registers::register_bitfields! [
    u32,
    pub Gicp0IrqStatus [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        RESERVED7 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp0IrqMask [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        RESERVED7 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp0IrqEnable [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        RESERVED7 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp0IrqDisable [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        RESERVED7 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp0IrqTrigger [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        RESERVED7 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp1IrqStatus [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp1IrqMask [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp1IrqEnable [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp1IrqDisable [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp1IrqTrigger [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp2IrqStatus [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp2IrqMask [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp2IrqEnable [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp2IrqDisable [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp2IrqTrigger [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp3IrqStatus [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp3IrqMask [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp3IrqEnable [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp3IrqDisable [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp3IrqTrigger [
        SRC31 OFFSET(31) NUMBITS(1) [],
        SRC30 OFFSET(30) NUMBITS(1) [],
        SRC29 OFFSET(29) NUMBITS(1) [],
        SRC28 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(2) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp4IrqStatus [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp4IrqMask [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp4IrqEnable [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp4IrqDisable [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gicp4IrqTrigger [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        SRC27 OFFSET(27) NUMBITS(1) [],
        SRC26 OFFSET(26) NUMBITS(1) [],
        SRC25 OFFSET(25) NUMBITS(1) [],
        SRC24 OFFSET(24) NUMBITS(1) [],
        SRC23 OFFSET(23) NUMBITS(1) [],
        SRC22 OFFSET(22) NUMBITS(1) [],
        SRC21 OFFSET(21) NUMBITS(1) [],
        SRC20 OFFSET(20) NUMBITS(1) [],
        SRC19 OFFSET(19) NUMBITS(1) [],
        SRC18 OFFSET(18) NUMBITS(1) [],
        SRC17 OFFSET(17) NUMBITS(1) [],
        SRC16 OFFSET(16) NUMBITS(1) [],
        SRC15 OFFSET(15) NUMBITS(1) [],
        SRC14 OFFSET(14) NUMBITS(1) [],
        SRC13 OFFSET(13) NUMBITS(1) [],
        SRC12 OFFSET(12) NUMBITS(1) [],
        SRC11 OFFSET(11) NUMBITS(1) [],
        SRC10 OFFSET(10) NUMBITS(1) [],
        SRC9 OFFSET(9) NUMBITS(1) [],
        SRC8 OFFSET(8) NUMBITS(1) [],
        SRC7 OFFSET(7) NUMBITS(1) [],
        SRC6 OFFSET(6) NUMBITS(1) [],
        SRC5 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GicpPmuIrqStatusR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        RESERVED2 OFFSET(6) NUMBITS(1) [],
        RESERVED3 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ],
    pub GicpPmuIrqStatusW [
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GicpPmuIrqMask [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        RESERVED2 OFFSET(6) NUMBITS(1) [],
        RESERVED3 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GicpPmuIrqEnableR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
    ],
    pub GicpPmuIrqEnableW [
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GicpPmuIrqDisableR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
    ],
    pub GicpPmuIrqDisableW [
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GicpPmuIrqTriggerR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
    ],
    pub GicpPmuIrqTriggerW [
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        SRC4 OFFSET(4) NUMBITS(1) [],
        SRC3 OFFSET(3) NUMBITS(1) [],
        SRC2 OFFSET(2) NUMBITS(1) [],
        SRC1 OFFSET(1) NUMBITS(1) [],
        SRC0 OFFSET(0) NUMBITS(1) [],
    ]
];

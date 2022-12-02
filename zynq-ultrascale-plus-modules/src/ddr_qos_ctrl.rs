// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly};
/// DDR Quality of Service (QoS), DDR QoS Control
pub static mut DDR_QOS_CTRL: *mut Registers = 0xfd090000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Set Port Type Register
        (0x00000000 => pub port_type: Aliased<u32, PortTypeR::Register, PortTypeW::Register>),
        /// Set Port Type Register
        (0x00000004 => pub qos_ctrl: Aliased<u32, QosCtrlR::Register, QosCtrlW::Register>),
        /// Set Value for Read HPR (High Priority Read) CAM Threshold
        (0x00000008 => pub rd_hpr_thrsld: Aliased<u32, RdHprThrsldR::Register, RdHprThrsldW::Register>),
        /// Set Value for Read LPR (Low Priority Read) CAM Threshold
        (0x0000000c => pub rd_lpr_thrsld: Aliased<u32, RdLprThrsldR::Register, RdLprThrsldW::Register>),
        /// Set Value for Write CAM Threshold
        (0x00000010 => pub wr_thrsld: Aliased<u32, WrThrsldR::Register, WrThrsldW::Register>),
        /// ZQCS Control Register 0
        (0x00000014 => pub zqcs_ctrl0: Aliased<u32, ZqcsCtrl0R::Register, ZqcsCtrl0W::Register>),
        /// ZQCS Control Register 2
        (0x00000018 => pub zqcs_ctrl1: Aliased<u32, ZqcsCtrl1R::Register, ZqcsCtrl1W::Register>),
        /// ZQCS Status Register
        (0x0000001c => pub zqcs_status: ReadOnly<u32, ZqcsStatus::Register>),
        /// DDRC External Refresh Control Register
        (0x00000020 => pub ddrc_ext_refresh: Aliased<u32, DdrcExtRefreshR::Register, DdrcExtRefreshW::Register>),
        (0x00000024 => _padding36),
        /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
        (0x00000200 => pub qos_irq_status: Aliased<u32, QosIrqStatusR::Register, QosIrqStatusW::Register>),
        /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
        (0x00000204 => pub qos_irq_mask: ReadOnly<u32, QosIrqMask::Register>),
        /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
        (0x00000208 => pub qos_irq_enable: Aliased<u32, QosIrqEnableR::Register, QosIrqEnableW::Register>),
        /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
        (0x0000020c => pub qos_irq_disable: Aliased<u32, QosIrqDisableR::Register, QosIrqDisableW::Register>),
        (0x00000210 => _padding528),
        /// DDRC URGENT Sideband signal control register
        (0x00000510 => pub ddrc_urgent: Aliased<u32, DdrcUrgentR::Register, DdrcUrgentW::Register>),
        /// DDRC QVN Control register
        (0x00000514 => pub ddrc_qvn_ctrl: Aliased<u32, DdrcQvnCtrlR::Register, DdrcQvnCtrlW::Register>),
        /// DDRC MRR Register Status
        (0x00000518 => pub ddrc_mrr_status: ReadOnly<u32, DdrcMrrStatus::Register>),
        /// DDRC MRR Register Data
        (0x0000051c => pub ddrc_mrr_data0: ReadOnly<u32>),
        /// DDRC MRR Register Data
        (0x00000520 => pub ddrc_mrr_data1: ReadOnly<u32>),
        /// DDRC MRR Register Data
        (0x00000524 => pub ddrc_mrr_data2: ReadOnly<u32, DdrcMrrData2::Register>),
        /// DDRC MRR Register Data
        (0x00000528 => pub ddrc_mrr_data3: ReadOnly<u32>),
        /// DDRC MRR Register Data
        (0x0000052c => pub ddrc_mrr_data4: ReadOnly<u32>),
        /// DDRC MRR Register Data
        (0x00000530 => pub ddrc_mrr_data5: ReadOnly<u32, DdrcMrrData5::Register>),
        /// DDRC MRR Register Data
        (0x00000534 => pub ddrc_mrr_data6: ReadOnly<u32>),
        /// DDRC MRR Register Data
        (0x00000538 => pub ddrc_mrr_data7: ReadOnly<u32>),
        /// DDRC MRR Register Data
        (0x0000053c => pub ddrc_mrr_data8: ReadOnly<u32, DdrcMrrData8::Register>),
        /// DDRC MRR Register Data
        (0x00000540 => pub ddrc_mrr_data9: ReadOnly<u32>),
        /// DDRC MRR Register Data
        (0x00000544 => pub ddrc_mrr_data10: ReadOnly<u32>),
        /// DDRC MRR Register Data
        (0x00000548 => pub ddrc_mrr_data11: ReadOnly<u32, DdrcMrrData11::Register>),
        (0x0000054c => _padding1356),
        /// DDR Sub system clock control
        (0x00000700 => pub ddr_clk_ctrl: Aliased<u32, DdrClkCtrlR::Register, DdrClkCtrlW::Register>),
        (0x00000704 => @END),
    }
}
register_bitfields! [
    u32,
    pub PortTypeR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        PORT5_TYPE OFFSET(14) NUMBITS(2) [],
        PORT4_TYPE OFFSET(12) NUMBITS(2) [],
        PORT3_TYPE OFFSET(10) NUMBITS(2) [],
        PORT2B_TYPE OFFSET(8) NUMBITS(2) [],
        PORT2R_TYPE OFFSET(6) NUMBITS(2) [],
        PORT1B_TYPE OFFSET(4) NUMBITS(2) [],
        PORT1R_TYPE OFFSET(2) NUMBITS(2) [],
        PORT0_TYPE OFFSET(0) NUMBITS(2) [],
    ],
    pub PortTypeW [
        PORT5_TYPE OFFSET(14) NUMBITS(2) [],
        PORT4_TYPE OFFSET(12) NUMBITS(2) [],
        PORT3_TYPE OFFSET(10) NUMBITS(2) [],
        PORT2B_TYPE OFFSET(8) NUMBITS(2) [],
        PORT2R_TYPE OFFSET(6) NUMBITS(2) [],
        PORT1B_TYPE OFFSET(4) NUMBITS(2) [],
        PORT1R_TYPE OFFSET(2) NUMBITS(2) [],
        PORT0_TYPE OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub QosCtrlR [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        APB_ERR_RES OFFSET(22) NUMBITS(1) [],
        PORT5_WR_CTRL OFFSET(21) NUMBITS(1) [],
        PORT5_HPR_CTRL OFFSET(20) NUMBITS(1) [],
        PORT5_LPR_CTRL OFFSET(19) NUMBITS(1) [],
        PORT4_WR_CTRL OFFSET(18) NUMBITS(1) [],
        PORT4_HPR_CTRL OFFSET(17) NUMBITS(1) [],
        PORT4_LPR_CTRL OFFSET(16) NUMBITS(1) [],
        PORT3_WR_CTRL OFFSET(15) NUMBITS(1) [],
        PORT3_HPR_CTRL OFFSET(14) NUMBITS(1) [],
        PORT3_LPR_CTRL OFFSET(13) NUMBITS(1) [],
        PORT2_WR_CTRL OFFSET(12) NUMBITS(1) [],
        PORT2B_HPR_CTRL OFFSET(11) NUMBITS(1) [],
        PORT2B_LPR_CTRL OFFSET(10) NUMBITS(1) [],
        PORT2R_HPR_CTRL OFFSET(9) NUMBITS(1) [],
        PORT2R_LPR_CTRL OFFSET(8) NUMBITS(1) [],
        PORT1_WR_CTRL OFFSET(7) NUMBITS(1) [],
        PORT1B_HPR_CTRL OFFSET(6) NUMBITS(1) [],
        PORT1B_LPR_CTRL OFFSET(5) NUMBITS(1) [],
        PORT1R_HPR_CTRL OFFSET(4) NUMBITS(1) [],
        PORT1R_LPR_CTRL OFFSET(3) NUMBITS(1) [],
        PORT0_WR_CTRL OFFSET(2) NUMBITS(1) [],
        PORT0_HPR_CTRL OFFSET(1) NUMBITS(1) [],
        PORT0_LPR_CTRL OFFSET(0) NUMBITS(1) [],
    ],
    pub QosCtrlW [
        APB_ERR_RES OFFSET(22) NUMBITS(1) [],
        PORT5_WR_CTRL OFFSET(21) NUMBITS(1) [],
        PORT5_HPR_CTRL OFFSET(20) NUMBITS(1) [],
        PORT5_LPR_CTRL OFFSET(19) NUMBITS(1) [],
        PORT4_WR_CTRL OFFSET(18) NUMBITS(1) [],
        PORT4_HPR_CTRL OFFSET(17) NUMBITS(1) [],
        PORT4_LPR_CTRL OFFSET(16) NUMBITS(1) [],
        PORT3_WR_CTRL OFFSET(15) NUMBITS(1) [],
        PORT3_HPR_CTRL OFFSET(14) NUMBITS(1) [],
        PORT3_LPR_CTRL OFFSET(13) NUMBITS(1) [],
        PORT2_WR_CTRL OFFSET(12) NUMBITS(1) [],
        PORT2B_HPR_CTRL OFFSET(11) NUMBITS(1) [],
        PORT2B_LPR_CTRL OFFSET(10) NUMBITS(1) [],
        PORT2R_HPR_CTRL OFFSET(9) NUMBITS(1) [],
        PORT2R_LPR_CTRL OFFSET(8) NUMBITS(1) [],
        PORT1_WR_CTRL OFFSET(7) NUMBITS(1) [],
        PORT1B_HPR_CTRL OFFSET(6) NUMBITS(1) [],
        PORT1B_LPR_CTRL OFFSET(5) NUMBITS(1) [],
        PORT1R_HPR_CTRL OFFSET(4) NUMBITS(1) [],
        PORT1R_LPR_CTRL OFFSET(3) NUMBITS(1) [],
        PORT0_WR_CTRL OFFSET(2) NUMBITS(1) [],
        PORT0_HPR_CTRL OFFSET(1) NUMBITS(1) [],
        PORT0_LPR_CTRL OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub RdHprThrsldR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        VALUE OFFSET(0) NUMBITS(7) [],
    ],
    pub RdHprThrsldW [
        VALUE OFFSET(0) NUMBITS(7) [],
    ]
];
register_bitfields! [
    u32,
    pub RdLprThrsldR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        VALUE OFFSET(0) NUMBITS(7) [],
    ],
    pub RdLprThrsldW [
        VALUE OFFSET(0) NUMBITS(7) [],
    ]
];
register_bitfields! [
    u32,
    pub WrThrsldR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        VALUE OFFSET(0) NUMBITS(7) [],
    ],
    pub WrThrsldW [
        VALUE OFFSET(0) NUMBITS(7) [],
    ]
];
register_bitfields! [
    u32,
    pub ZqcsCtrl0R [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub ZqcsCtrl0W [
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ZqcsCtrl1R [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        VSYNC_CNT OFFSET(0) NUMBITS(4) [],
    ],
    pub ZqcsCtrl1W [
        VSYNC_CNT OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub ZqcsStatus [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        BUSY OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrcExtRefreshR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub DdrcExtRefreshW [
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub QosIrqStatusR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        DDRC_WR_POISON OFFSET(10) NUMBITS(1) [],
        DDRC_RD_POISON OFFSET(9) NUMBITS(1) [],
        MRR_DATA_VALID OFFSET(8) NUMBITS(1) [],
        PC_COPY_DONE OFFSET(7) NUMBITS(1) [],
        DFI_ALT_ERR OFFSET(6) NUMBITS(1) [],
        DFI_ALT_ERR_MAX OFFSET(5) NUMBITS(1) [],
        DFI_ALT_ERR_FTL OFFSET(4) NUMBITS(1) [],
        DFI_INIT_COMP OFFSET(3) NUMBITS(1) [],
        DDRECC_UNCRERR OFFSET(2) NUMBITS(1) [],
        DDRECC_CORERR OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ],
    pub QosIrqStatusW [
        DDRC_WR_POISON OFFSET(10) NUMBITS(1) [],
        DDRC_RD_POISON OFFSET(9) NUMBITS(1) [],
        MRR_DATA_VALID OFFSET(8) NUMBITS(1) [],
        PC_COPY_DONE OFFSET(7) NUMBITS(1) [],
        DFI_ALT_ERR OFFSET(6) NUMBITS(1) [],
        DFI_ALT_ERR_MAX OFFSET(5) NUMBITS(1) [],
        DFI_ALT_ERR_FTL OFFSET(4) NUMBITS(1) [],
        DFI_INIT_COMP OFFSET(3) NUMBITS(1) [],
        DDRECC_UNCRERR OFFSET(2) NUMBITS(1) [],
        DDRECC_CORERR OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub QosIrqMask [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        DDRC_WR_POISON OFFSET(10) NUMBITS(1) [],
        DDRC_RD_POISON OFFSET(9) NUMBITS(1) [],
        MRR_DATA_VALID OFFSET(8) NUMBITS(1) [],
        PC_COPY_DONE OFFSET(7) NUMBITS(1) [],
        DFI_ALT_ERR OFFSET(6) NUMBITS(1) [],
        DFI_ALT_ERR_MAX OFFSET(5) NUMBITS(1) [],
        DFI_ALT_ERR_FTL OFFSET(4) NUMBITS(1) [],
        DFI_INIT_COMP OFFSET(3) NUMBITS(1) [],
        DDRECC_UNCRERR OFFSET(2) NUMBITS(1) [],
        DDRECC_CORERR OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub QosIrqEnableR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
    ],
    pub QosIrqEnableW [
        DDRC_WR_POISON OFFSET(10) NUMBITS(1) [],
        DDRC_RD_POISON OFFSET(9) NUMBITS(1) [],
        MRR_DATA_VALID OFFSET(8) NUMBITS(1) [],
        PC_COPY_DONE OFFSET(7) NUMBITS(1) [],
        DFI_ALT_ERR OFFSET(6) NUMBITS(1) [],
        DFI_ALT_ERR_MAX OFFSET(5) NUMBITS(1) [],
        DFI_ALT_ERR_FTL OFFSET(4) NUMBITS(1) [],
        DFI_INIT_COMP OFFSET(3) NUMBITS(1) [],
        DDRECC_UNCRERR OFFSET(2) NUMBITS(1) [],
        DDRECC_CORERR OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub QosIrqDisableR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
    ],
    pub QosIrqDisableW [
        DDRC_WR_POISON OFFSET(10) NUMBITS(1) [],
        DDRC_RD_POISON OFFSET(9) NUMBITS(1) [],
        MRR_DATA_VALID OFFSET(8) NUMBITS(1) [],
        PC_COPY_DONE OFFSET(7) NUMBITS(1) [],
        DFI_ALT_ERR OFFSET(6) NUMBITS(1) [],
        DFI_ALT_ERR_MAX OFFSET(5) NUMBITS(1) [],
        DFI_ALT_ERR_FTL OFFSET(4) NUMBITS(1) [],
        DFI_INIT_COMP OFFSET(3) NUMBITS(1) [],
        DDRECC_UNCRERR OFFSET(2) NUMBITS(1) [],
        DDRECC_CORERR OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrcUrgentR [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        ARURGENT_5 OFFSET(13) NUMBITS(1) [],
        AWURGENT_5 OFFSET(12) NUMBITS(1) [],
        ARURGENT_4 OFFSET(11) NUMBITS(1) [],
        AWURGENT_4 OFFSET(10) NUMBITS(1) [],
        ARURGENT_3 OFFSET(9) NUMBITS(1) [],
        AWURGENT_3 OFFSET(8) NUMBITS(1) [],
        ARURGENTR_2 OFFSET(7) NUMBITS(1) [],
        ARURGENTB_2 OFFSET(6) NUMBITS(1) [],
        AWURGENT_2 OFFSET(5) NUMBITS(1) [],
        ARURGENTR_1 OFFSET(4) NUMBITS(1) [],
        ARURGENTB_1 OFFSET(3) NUMBITS(1) [],
        AWURGENT_1 OFFSET(2) NUMBITS(1) [],
        ARURGENT_0 OFFSET(1) NUMBITS(1) [],
        AWURGENT_0 OFFSET(0) NUMBITS(1) [],
    ],
    pub DdrcUrgentW [
        ARURGENT_5 OFFSET(13) NUMBITS(1) [],
        AWURGENT_5 OFFSET(12) NUMBITS(1) [],
        ARURGENT_4 OFFSET(11) NUMBITS(1) [],
        AWURGENT_4 OFFSET(10) NUMBITS(1) [],
        ARURGENT_3 OFFSET(9) NUMBITS(1) [],
        AWURGENT_3 OFFSET(8) NUMBITS(1) [],
        ARURGENTR_2 OFFSET(7) NUMBITS(1) [],
        ARURGENTB_2 OFFSET(6) NUMBITS(1) [],
        AWURGENT_2 OFFSET(5) NUMBITS(1) [],
        ARURGENTR_1 OFFSET(4) NUMBITS(1) [],
        ARURGENTB_1 OFFSET(3) NUMBITS(1) [],
        AWURGENT_1 OFFSET(2) NUMBITS(1) [],
        ARURGENT_0 OFFSET(1) NUMBITS(1) [],
        AWURGENT_0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrcQvnCtrlR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        PREALLOC_P2 OFFSET(4) NUMBITS(2) [],
        PREALLOC_P1 OFFSET(2) NUMBITS(2) [],
        EN_P2 OFFSET(1) NUMBITS(1) [],
        EN_P1 OFFSET(0) NUMBITS(1) [],
    ],
    pub DdrcQvnCtrlW [
        PREALLOC_P2 OFFSET(4) NUMBITS(2) [],
        PREALLOC_P1 OFFSET(2) NUMBITS(2) [],
        EN_P2 OFFSET(1) NUMBITS(1) [],
        EN_P1 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrcMrrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        VALID_CNT OFFSET(1) NUMBITS(3) [],
        VALID OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrcMrrData2 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ECC OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrcMrrData5 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ECC OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrcMrrData8 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ECC OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrcMrrData11 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ECC OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrClkCtrlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        CLKACT OFFSET(0) NUMBITS(1) [],
    ],
    pub DdrClkCtrlW [
        CLKACT OFFSET(0) NUMBITS(1) [],
    ]
];

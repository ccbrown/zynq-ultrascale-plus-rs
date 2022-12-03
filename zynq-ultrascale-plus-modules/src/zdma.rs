// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// PS General Purpose DMA, LPD DMA Channel 0
pub static mut ADMA_CH0: *mut Registers = 0xffa80000 as *mut Registers;
/// PS General Purpose DMA, LPD DMA Channel 1
pub static mut ADMA_CH1: *mut Registers = 0xffa90000 as *mut Registers;
/// PS General Purpose DMA, LPD DMA Channel 2
pub static mut ADMA_CH2: *mut Registers = 0xffaa0000 as *mut Registers;
/// PS General Purpose DMA, LPD DMA Channel 3
pub static mut ADMA_CH3: *mut Registers = 0xffab0000 as *mut Registers;
/// PS General Purpose DMA, LPD DMA Channel 4
pub static mut ADMA_CH4: *mut Registers = 0xffac0000 as *mut Registers;
/// PS General Purpose DMA, LPD DMA Channel 5
pub static mut ADMA_CH5: *mut Registers = 0xffad0000 as *mut Registers;
/// PS General Purpose DMA, LPD DMA Channel 6
pub static mut ADMA_CH6: *mut Registers = 0xffae0000 as *mut Registers;
/// PS General Purpose DMA, LPD DMA Channel 7
pub static mut ADMA_CH7: *mut Registers = 0xffaf0000 as *mut Registers;
/// PS General Purpose DMA, FPD DMA Channel 0
pub static mut GDMA_CH0: *mut Registers = 0xfd500000 as *mut Registers;
/// PS General Purpose DMA, FPD DMA Channel 1
pub static mut GDMA_CH1: *mut Registers = 0xfd510000 as *mut Registers;
/// PS General Purpose DMA, FPD DMA Channel 2
pub static mut GDMA_CH2: *mut Registers = 0xfd520000 as *mut Registers;
/// PS General Purpose DMA, FPD DMA Channel 3
pub static mut GDMA_CH3: *mut Registers = 0xfd530000 as *mut Registers;
/// PS General Purpose DMA, FPD DMA Channel 4
pub static mut GDMA_CH4: *mut Registers = 0xfd540000 as *mut Registers;
/// PS General Purpose DMA, FPD DMA Channel 5
pub static mut GDMA_CH5: *mut Registers = 0xfd550000 as *mut Registers;
/// PS General Purpose DMA, FPD DMA Channel 6
pub static mut GDMA_CH6: *mut Registers = 0xfd560000 as *mut Registers;
/// PS General Purpose DMA, FPD DMA Channel 7
pub static mut GDMA_CH7: *mut Registers = 0xfd570000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Enable/Disable a error response
    pub zdma_err_ctrl: Aliased<u32, ZdmaErrCtrlR::Register, ZdmaErrCtrlW::Register>,
    _padding4: [u8; 252],
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub zdma_ch_isr: Aliased<u32, ZdmaChIsrR::Register, ZdmaChIsrW::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub zdma_ch_imr: ReadOnly<u32, ZdmaChImr::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub zdma_ch_ien: Aliased<u32, ZdmaChIenR::Register, ZdmaChIenW::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub zdma_ch_ids: Aliased<u32, ZdmaChIdsR::Register, ZdmaChIdsW::Register>,
    /// Channel Control Register 0
    pub zdma_ch_ctrl0: Aliased<u32, ZdmaChCtrl0R::Register, ZdmaChCtrl0W::Register>,
    /// Channel Control Register 1
    pub zdma_ch_ctrl1: Aliased<u32, ZdmaChCtrl1R::Register, ZdmaChCtrl1W::Register>,
    /// Channel Flow Control Register
    pub zdma_ch_fci: Aliased<u32, ZdmaChFciR::Register, ZdmaChFciW::Register>,
    /// Channel Status Register
    pub zdma_ch_status: ReadOnly<u32, ZdmaChStatus::Register>,
    /// Channel DATA AXI parameter Register
    pub zdma_ch_data_attr: Aliased<u32, ZdmaChDataAttrR::Register, ZdmaChDataAttrW::Register>,
    /// Channel DSCR AXI parameter Register
    pub zdma_ch_dscr_attr: Aliased<u32, ZdmaChDscrAttrR::Register, ZdmaChDscrAttrW::Register>,
    /// SRC DSCR Word 0
    pub zdma_ch_src_dscr_word0: ReadWrite<u32>,
    /// SRC DSCR Word 1
    pub zdma_ch_src_dscr_word1:
        Aliased<u32, ZdmaChSrcDscrWord1R::Register, ZdmaChSrcDscrWord1W::Register>,
    /// SRC DSCR Word 2
    pub zdma_ch_src_dscr_word2:
        Aliased<u32, ZdmaChSrcDscrWord2R::Register, ZdmaChSrcDscrWord2W::Register>,
    /// SRC DSCR Word 3
    pub zdma_ch_src_dscr_word3:
        Aliased<u32, ZdmaChSrcDscrWord3R::Register, ZdmaChSrcDscrWord3W::Register>,
    /// DST DSCR Word 0
    pub zdma_ch_dst_dscr_word0: ReadWrite<u32>,
    /// DST DSCR Word 1
    pub zdma_ch_dst_dscr_word1:
        Aliased<u32, ZdmaChDstDscrWord1R::Register, ZdmaChDstDscrWord1W::Register>,
    /// DST DSCR Word 2
    pub zdma_ch_dst_dscr_word2:
        Aliased<u32, ZdmaChDstDscrWord2R::Register, ZdmaChDstDscrWord2W::Register>,
    /// DST DSCR Word 3
    pub zdma_ch_dst_dscr_word3:
        Aliased<u32, ZdmaChDstDscrWord3R::Register, ZdmaChDstDscrWord3W::Register>,
    /// Write Only Data Word 0
    pub zdma_ch_wr_only_word0: ReadWrite<u32>,
    /// Write Only Data Word 1
    pub zdma_ch_wr_only_word1: ReadWrite<u32>,
    /// Write Only Data Word 2
    pub zdma_ch_wr_only_word2: ReadWrite<u32>,
    /// Write Only Data Word 3
    pub zdma_ch_wr_only_word3: ReadWrite<u32>,
    /// SRC DSCR Start Address LSB Register
    pub zdma_ch_src_start_lsb: ReadWrite<u32>,
    /// SRC DSCR Start Address MSB Register
    pub zdma_ch_src_start_msb:
        Aliased<u32, ZdmaChSrcStartMsbR::Register, ZdmaChSrcStartMsbW::Register>,
    /// DST DSCR Start Address LSB Register
    pub zdma_ch_dst_start_lsb: ReadWrite<u32>,
    /// DST DSCR Start Address MSB Register
    pub zdma_ch_dst_start_msb:
        Aliased<u32, ZdmaChDstStartMsbR::Register, ZdmaChDstStartMsbW::Register>,
    _padding360: [u8; 32],
    /// Total Bytes Transferred Register
    pub zdma_ch_total_byte: ReadWrite<u32>,
    /// Rate Control Count Register
    pub zdma_ch_rate_ctrl: Aliased<u32, ZdmaChRateCtrlR::Register, ZdmaChRateCtrlW::Register>,
    /// SRC Interrupt Account Count Register
    pub zdma_ch_irq_src_acct: ReadOnly<u32, ZdmaChIrqSrcAcct::Register>,
    /// DST Interrupt Account Count Register
    pub zdma_ch_irq_dst_acct: ReadOnly<u32, ZdmaChIrqDstAcct::Register>,
    _padding408: [u8; 104],
    /// zDMA Control Register 2
    pub zdma_ch_ctrl2: Aliased<u32, ZdmaChCtrl2R::Register, ZdmaChCtrl2W::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub ZdmaErrCtrlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ],
    pub ZdmaErrCtrlW [
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChIsrR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        DMA_PAUSE OFFSET(11) NUMBITS(1) [],
        DMA_DONE OFFSET(10) NUMBITS(1) [],
        AXI_WR_DATA OFFSET(9) NUMBITS(1) [],
        AXI_RD_DATA OFFSET(8) NUMBITS(1) [],
        AXI_RD_DST_DSCR OFFSET(7) NUMBITS(1) [],
        AXI_RD_SRC_DSCR OFFSET(6) NUMBITS(1) [],
        IRQ_DST_ACCT_ERR OFFSET(5) NUMBITS(1) [],
        IRQ_SRC_ACCT_ERR OFFSET(4) NUMBITS(1) [],
        BYTE_CNT_OVRFL OFFSET(3) NUMBITS(1) [],
        DST_DSCR_DONE OFFSET(2) NUMBITS(1) [],
        SRC_DSCR_DONE OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ],
    pub ZdmaChIsrW [
        DMA_PAUSE OFFSET(11) NUMBITS(1) [],
        DMA_DONE OFFSET(10) NUMBITS(1) [],
        AXI_WR_DATA OFFSET(9) NUMBITS(1) [],
        AXI_RD_DATA OFFSET(8) NUMBITS(1) [],
        AXI_RD_DST_DSCR OFFSET(7) NUMBITS(1) [],
        AXI_RD_SRC_DSCR OFFSET(6) NUMBITS(1) [],
        IRQ_DST_ACCT_ERR OFFSET(5) NUMBITS(1) [],
        IRQ_SRC_ACCT_ERR OFFSET(4) NUMBITS(1) [],
        BYTE_CNT_OVRFL OFFSET(3) NUMBITS(1) [],
        DST_DSCR_DONE OFFSET(2) NUMBITS(1) [],
        SRC_DSCR_DONE OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChImr [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        DMA_PAUSE OFFSET(11) NUMBITS(1) [],
        DMA_DONE OFFSET(10) NUMBITS(1) [],
        AXI_WR_DATA OFFSET(9) NUMBITS(1) [],
        AXI_RD_DATA OFFSET(8) NUMBITS(1) [],
        AXI_RD_DST_DSCR OFFSET(7) NUMBITS(1) [],
        AXI_RD_SRC_DSCR OFFSET(6) NUMBITS(1) [],
        IRQ_DST_ACCT_ERR OFFSET(5) NUMBITS(1) [],
        IRQ_SRC_ACCT_ERR OFFSET(4) NUMBITS(1) [],
        BYTE_CNT_OVRFL OFFSET(3) NUMBITS(1) [],
        DST_DSCR_DONE OFFSET(2) NUMBITS(1) [],
        SRC_DSCR_DONE OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChIenR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
    ],
    pub ZdmaChIenW [
        DMA_PAUSE OFFSET(11) NUMBITS(1) [],
        DMA_DONE OFFSET(10) NUMBITS(1) [],
        AXI_WR_DATA OFFSET(9) NUMBITS(1) [],
        AXI_RD_DATA OFFSET(8) NUMBITS(1) [],
        AXI_RD_DST_DSCR OFFSET(7) NUMBITS(1) [],
        AXI_RD_SRC_DSCR OFFSET(6) NUMBITS(1) [],
        IRQ_DST_ACCT_ERR OFFSET(5) NUMBITS(1) [],
        IRQ_SRC_ACCT_ERR OFFSET(4) NUMBITS(1) [],
        BYTE_CNT_OVRFL OFFSET(3) NUMBITS(1) [],
        DST_DSCR_DONE OFFSET(2) NUMBITS(1) [],
        SRC_DSCR_DONE OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChIdsR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
    ],
    pub ZdmaChIdsW [
        DMA_PAUSE OFFSET(11) NUMBITS(1) [],
        DMA_DONE OFFSET(10) NUMBITS(1) [],
        AXI_WR_DATA OFFSET(9) NUMBITS(1) [],
        AXI_RD_DATA OFFSET(8) NUMBITS(1) [],
        AXI_RD_DST_DSCR OFFSET(7) NUMBITS(1) [],
        AXI_RD_SRC_DSCR OFFSET(6) NUMBITS(1) [],
        IRQ_DST_ACCT_ERR OFFSET(5) NUMBITS(1) [],
        IRQ_SRC_ACCT_ERR OFFSET(4) NUMBITS(1) [],
        BYTE_CNT_OVRFL OFFSET(3) NUMBITS(1) [],
        DST_DSCR_DONE OFFSET(2) NUMBITS(1) [],
        SRC_DSCR_DONE OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChCtrl0R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        OVR_FETCH OFFSET(7) NUMBITS(1) [],
        POINT_TYPE OFFSET(6) NUMBITS(1) [],
        MODE OFFSET(4) NUMBITS(2) [],
        RATE_CTRL OFFSET(3) NUMBITS(1) [],
        CONT_ADDR OFFSET(2) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ],
    pub ZdmaChCtrl0W [
        OVR_FETCH OFFSET(7) NUMBITS(1) [],
        POINT_TYPE OFFSET(6) NUMBITS(1) [],
        MODE OFFSET(4) NUMBITS(2) [],
        RATE_CTRL OFFSET(3) NUMBITS(1) [],
        CONT_ADDR OFFSET(2) NUMBITS(1) [],
        CONT OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChCtrl1R [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        DST_ISSUE OFFSET(5) NUMBITS(5) [],
        SRC_ISSUE OFFSET(0) NUMBITS(5) [],
    ],
    pub ZdmaChCtrl1W [
        DST_ISSUE OFFSET(5) NUMBITS(5) [],
        SRC_ISSUE OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChFciR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        PROG_CELL_CNT OFFSET(2) NUMBITS(2) [],
        SIDE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub ZdmaChFciW [
        PROG_CELL_CNT OFFSET(2) NUMBITS(2) [],
        SIDE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChStatus [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        STATE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChDataAttrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        ARBURST OFFSET(26) NUMBITS(2) [],
        ARCACHE OFFSET(22) NUMBITS(4) [],
        ARQOS OFFSET(18) NUMBITS(4) [],
        ARLEN OFFSET(14) NUMBITS(4) [],
        AWBURST OFFSET(12) NUMBITS(2) [],
        AWCACHE OFFSET(8) NUMBITS(4) [],
        AWQOS OFFSET(4) NUMBITS(4) [],
        AWLEN OFFSET(0) NUMBITS(4) [],
    ],
    pub ZdmaChDataAttrW [
        ARBURST OFFSET(26) NUMBITS(2) [],
        ARCACHE OFFSET(22) NUMBITS(4) [],
        ARQOS OFFSET(18) NUMBITS(4) [],
        ARLEN OFFSET(14) NUMBITS(4) [],
        AWBURST OFFSET(12) NUMBITS(2) [],
        AWCACHE OFFSET(8) NUMBITS(4) [],
        AWQOS OFFSET(4) NUMBITS(4) [],
        AWLEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChDscrAttrR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        AXCOHRNT OFFSET(8) NUMBITS(1) [],
        AXCACHE OFFSET(4) NUMBITS(4) [],
        AXQOS OFFSET(0) NUMBITS(4) [],
    ],
    pub ZdmaChDscrAttrW [
        AXCOHRNT OFFSET(8) NUMBITS(1) [],
        AXCACHE OFFSET(4) NUMBITS(4) [],
        AXQOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChSrcDscrWord1R [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        MSB OFFSET(0) NUMBITS(17) [],
    ],
    pub ZdmaChSrcDscrWord1W [
        MSB OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChSrcDscrWord2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        SIZE OFFSET(0) NUMBITS(30) [],
    ],
    pub ZdmaChSrcDscrWord2W [
        SIZE OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChSrcDscrWord3R [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        CMD OFFSET(3) NUMBITS(2) [],
        INTR OFFSET(2) NUMBITS(1) [],
        TYPE OFFSET(1) NUMBITS(1) [],
        COHRNT OFFSET(0) NUMBITS(1) [],
    ],
    pub ZdmaChSrcDscrWord3W [
        CMD OFFSET(3) NUMBITS(2) [],
        INTR OFFSET(2) NUMBITS(1) [],
        TYPE OFFSET(1) NUMBITS(1) [],
        COHRNT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChDstDscrWord1R [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        MSB OFFSET(0) NUMBITS(17) [],
    ],
    pub ZdmaChDstDscrWord1W [
        MSB OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChDstDscrWord2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        SIZE OFFSET(0) NUMBITS(30) [],
    ],
    pub ZdmaChDstDscrWord2W [
        SIZE OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChDstDscrWord3R [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        INTR OFFSET(2) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(1) [],
        COHRNT OFFSET(0) NUMBITS(1) [],
    ],
    pub ZdmaChDstDscrWord3W [
        INTR OFFSET(2) NUMBITS(1) [],
        COHRNT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChSrcStartMsbR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        ADDR OFFSET(0) NUMBITS(17) [],
    ],
    pub ZdmaChSrcStartMsbW [
        ADDR OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChDstStartMsbR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        ADDR OFFSET(0) NUMBITS(17) [],
    ],
    pub ZdmaChDstStartMsbW [
        ADDR OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChRateCtrlR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        CNT OFFSET(0) NUMBITS(12) [],
    ],
    pub ZdmaChRateCtrlW [
        CNT OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChIrqSrcAcct [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        CNT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChIrqDstAcct [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        CNT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZdmaChCtrl2R [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub ZdmaChCtrl2W [
        EN OFFSET(0) NUMBITS(1) [],
    ]
];

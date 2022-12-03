// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// CSU Module DMA Controller, CSU DMA Control
pub static mut CSUDMA: *mut Registers = 0xffc80000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Source mem address (lsbs) for DMA memory->stream data transfer
    pub csudma_src_addr: Aliased<u32, CsudmaSrcAddrR::Register, CsudmaSrcAddrW::Register>,
    /// DMA transfer payload for DMA memory-> stream data transfer
    pub csudma_src_size: Aliased<u32, CsudmaSrcSizeR::Register, CsudmaSrcSizeW::Register>,
    /// General SRC DMA Status
    pub csudma_src_sts: Aliased<u32, CsudmaSrcStsR::Register, CsudmaSrcStsW::Register>,
    /// General SRC DMA Control Register 1
    pub csudma_src_ctrl: Aliased<u32, CsudmaSrcCtrlR::Register, CsudmaSrcCtrlW::Register>,
    /// SRC DMA Pseudo CRC
    pub csudma_src_crc: ReadWrite<u32>,
    /// SRC DMA Interrupt Status Register
    pub csudma_src_i_sts: Aliased<u32, CsudmaSrcIStsR::Register, CsudmaSrcIStsW::Register>,
    /// SRC DMA Interrupt Enable
    pub csudma_src_i_en: Aliased<u32, CsudmaSrcIEnR::Register, CsudmaSrcIEnW::Register>,
    /// SRC DMA Interrupt Disable
    pub csudma_src_i_dis: Aliased<u32, CsudmaSrcIDisR::Register, CsudmaSrcIDisW::Register>,
    /// SRC DMA Interrupt Mask
    pub csudma_src_i_mask: ReadOnly<u32, CsudmaSrcIMask::Register>,
    /// General SRC DMA Control Register 2
    pub csudma_src_ctrl2: Aliased<u32, CsudmaSrcCtrl2R::Register, CsudmaSrcCtrl2W::Register>,
    /// Source mem address (msbs) for DMA memory->stream data transfer
    pub csudma_src_addr_msb: Aliased<u32, CsudmaSrcAddrMsbR::Register, CsudmaSrcAddrMsbW::Register>,
    _padding44: [u8; 2004],
    /// Destination mem address (lsbs) for DMA stream->memory data transfer
    pub csudma_dst_addr: Aliased<u32, CsudmaDstAddrR::Register, CsudmaDstAddrW::Register>,
    /// DMA transfer payload for DMA stream-> memory data transfer
    pub csudma_dst_size: Aliased<u32, CsudmaDstSizeR::Register, CsudmaDstSizeW::Register>,
    /// General DST DMA Status
    pub csudma_dst_sts: Aliased<u32, CsudmaDstStsR::Register, CsudmaDstStsW::Register>,
    /// General DST DMA Control
    pub csudma_dst_ctrl: ReadWrite<u32, CsudmaDstCtrl::Register>,
    _padding2064: [u8; 4],
    /// DST DMA Interrupt Status Register
    pub csudma_dst_i_sts: Aliased<u32, CsudmaDstIStsR::Register, CsudmaDstIStsW::Register>,
    /// DST DMA Interrupt Enable
    pub csudma_dst_i_en: Aliased<u32, CsudmaDstIEnR::Register, CsudmaDstIEnW::Register>,
    /// DST DMA Interrupt Disable
    pub csudma_dst_i_dis: Aliased<u32, CsudmaDstIDisR::Register, CsudmaDstIDisW::Register>,
    /// DST DMA Interrupt Mask
    pub csudma_dst_i_mask: ReadOnly<u32, CsudmaDstIMask::Register>,
    /// General DST DMA Control Register 2
    pub csudma_dst_ctrl2: Aliased<u32, CsudmaDstCtrl2R::Register, CsudmaDstCtrl2W::Register>,
    /// Destination mem address (msbs) for DMA stream->memory data transfer
    pub csudma_dst_addr_msb: Aliased<u32, CsudmaDstAddrMsbR::Register, CsudmaDstAddrMsbW::Register>,
    _padding2092: [u8; 1996],
    /// Safety endpoint connectivity check Register
    pub csudma_safety_chk: ReadWrite<u32>,
}
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcAddrR [
        ADDR OFFSET(2) NUMBITS(30) [],
        RESERVED0 OFFSET(0) NUMBITS(2) [],
    ],
    pub CsudmaSrcAddrW [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcSizeR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SIZE OFFSET(2) NUMBITS(27) [],
        RESERVED1 OFFSET(1) NUMBITS(1) [],
        LAST_WORD OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaSrcSizeW [
        SIZE OFFSET(2) NUMBITS(27) [],
        LAST_WORD OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcStsR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        DONE_CNT OFFSET(13) NUMBITS(3) [],
        SRC_FIFO_LEVEL OFFSET(5) NUMBITS(8) [],
        RD_OUTSTANDING OFFSET(1) NUMBITS(4) [],
        BUSY OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaSrcStsW [
        DONE_CNT OFFSET(13) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcCtrlR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        APB_ERR_RESP OFFSET(24) NUMBITS(1) [],
        ENDIANNESS OFFSET(23) NUMBITS(1) [],
        AXI_BRST_TYPE OFFSET(22) NUMBITS(1) [],
        TIMEOUT_VAL OFFSET(10) NUMBITS(12) [],
        FIFO_THRESH OFFSET(2) NUMBITS(8) [],
        PAUSE_STRM OFFSET(1) NUMBITS(1) [],
        PAUSE_MEM OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaSrcCtrlW [
        APB_ERR_RESP OFFSET(24) NUMBITS(1) [],
        ENDIANNESS OFFSET(23) NUMBITS(1) [],
        AXI_BRST_TYPE OFFSET(22) NUMBITS(1) [],
        TIMEOUT_VAL OFFSET(10) NUMBITS(12) [],
        FIFO_THRESH OFFSET(2) NUMBITS(8) [],
        PAUSE_STRM OFFSET(1) NUMBITS(1) [],
        PAUSE_MEM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcIStsR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_RDERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        MEM_DONE OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaSrcIStsW [
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_RDERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        MEM_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcIEnR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_RDERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        MEM_DONE OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaSrcIEnW [
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_RDERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        MEM_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcIDisR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_RDERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        MEM_DONE OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaSrcIDisW [
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_RDERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        MEM_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcIMask [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_RDERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        MEM_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcCtrl2R [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(27) NUMBITS(1) [],
        ARCACHE OFFSET(24) NUMBITS(3) [],
        ROUTE_BIT OFFSET(23) NUMBITS(1) [],
        TIMEOUT_EN OFFSET(22) NUMBITS(1) [],
        RESERVED2 OFFSET(19) NUMBITS(3) [],
        RESERVED3 OFFSET(16) NUMBITS(3) [],
        TIMEOUT_PRE OFFSET(4) NUMBITS(12) [],
        MAX_OUTS_CMDS OFFSET(0) NUMBITS(4) [],
    ],
    pub CsudmaSrcCtrl2W [
        RESERVED0 OFFSET(27) NUMBITS(1) [],
        ARCACHE OFFSET(24) NUMBITS(3) [],
        ROUTE_BIT OFFSET(23) NUMBITS(1) [],
        TIMEOUT_EN OFFSET(22) NUMBITS(1) [],
        RESERVED1 OFFSET(19) NUMBITS(3) [],
        RESERVED2 OFFSET(16) NUMBITS(3) [],
        TIMEOUT_PRE OFFSET(4) NUMBITS(12) [],
        MAX_OUTS_CMDS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaSrcAddrMsbR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        ADDR_MSB OFFSET(0) NUMBITS(17) [],
    ],
    pub CsudmaSrcAddrMsbW [
        ADDR_MSB OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstAddrR [
        ADDR OFFSET(2) NUMBITS(30) [],
        RESERVED0 OFFSET(0) NUMBITS(2) [],
    ],
    pub CsudmaDstAddrW [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstSizeR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SIZE OFFSET(2) NUMBITS(27) [],
        RESERVED1 OFFSET(0) NUMBITS(2) [],
    ],
    pub CsudmaDstSizeW [
        SIZE OFFSET(2) NUMBITS(27) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstStsR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        DONE_CNT OFFSET(13) NUMBITS(3) [],
        DST_FIFO_LEVEL OFFSET(5) NUMBITS(8) [],
        WR_OUTSTANDING OFFSET(1) NUMBITS(4) [],
        BUSY OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaDstStsW [
        DONE_CNT OFFSET(13) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstCtrl [
        SSS_FIFOTHRESH OFFSET(25) NUMBITS(7) [],
        APB_ERR_RESP OFFSET(24) NUMBITS(1) [],
        ENDIANNESS OFFSET(23) NUMBITS(1) [],
        AXI_BRST_TYPE OFFSET(22) NUMBITS(1) [],
        TIMEOUT_VAL OFFSET(10) NUMBITS(12) [],
        FIFO_THRESH OFFSET(2) NUMBITS(8) [],
        PAUSE_STRM OFFSET(1) NUMBITS(1) [],
        PAUSE_MEM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstIStsR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        FIFO_OVERFLOW OFFSET(7) NUMBITS(1) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_BRESP_ERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaDstIStsW [
        FIFO_OVERFLOW OFFSET(7) NUMBITS(1) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_BRESP_ERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstIEnR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        FIFO_OVERFLOW OFFSET(7) NUMBITS(1) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_BRESP_ERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaDstIEnW [
        FIFO_OVERFLOW OFFSET(7) NUMBITS(1) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_BRESP_ERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstIDisR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        FIFO_OVERFLOW OFFSET(7) NUMBITS(1) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_BRESP_ERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ],
    pub CsudmaDstIDisW [
        FIFO_OVERFLOW OFFSET(7) NUMBITS(1) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_BRESP_ERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstIMask [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        FIFO_OVERFLOW OFFSET(7) NUMBITS(1) [],
        INVALID_APB OFFSET(6) NUMBITS(1) [],
        THRESH_HIT OFFSET(5) NUMBITS(1) [],
        TIMEOUT_MEM OFFSET(4) NUMBITS(1) [],
        TIMEOUT_STRM OFFSET(3) NUMBITS(1) [],
        AXI_BRESP_ERR OFFSET(2) NUMBITS(1) [],
        DONE OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstCtrl2R [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(27) NUMBITS(1) [],
        AWCACHE OFFSET(24) NUMBITS(3) [],
        ROUTE_BIT OFFSET(23) NUMBITS(1) [],
        TIMEOUT_EN OFFSET(22) NUMBITS(1) [],
        RESERVED2 OFFSET(19) NUMBITS(3) [],
        RESERVED3 OFFSET(16) NUMBITS(3) [],
        TIMEOUT_PRE OFFSET(4) NUMBITS(12) [],
        MAX_OUTS_CMDS OFFSET(0) NUMBITS(4) [],
    ],
    pub CsudmaDstCtrl2W [
        RESERVED0 OFFSET(27) NUMBITS(1) [],
        AWCACHE OFFSET(24) NUMBITS(3) [],
        ROUTE_BIT OFFSET(23) NUMBITS(1) [],
        TIMEOUT_EN OFFSET(22) NUMBITS(1) [],
        RESERVED1 OFFSET(19) NUMBITS(3) [],
        RESERVED2 OFFSET(16) NUMBITS(3) [],
        TIMEOUT_PRE OFFSET(4) NUMBITS(12) [],
        MAX_OUTS_CMDS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsudmaDstAddrMsbR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        ADDR_MSB OFFSET(0) NUMBITS(17) [],
    ],
    pub CsudmaDstAddrMsbW [
        ADDR_MSB OFFSET(0) NUMBITS(17) [],
    ]
];

// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// Quad-SPI Registers, Quad SPI Controller
pub static mut QSPI: *mut Registers = 0xff0f0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// QSPI configuration register
    pub config: Aliased<u32, ConfigR::Register, ConfigW::Register>,
    /// Interrupt Status
    pub isr: Aliased<u32, IsrR::Register, IsrW::Register>,
    /// Interrupt Enable
    pub ier: Aliased<u32, IerR::Register, IerW::Register>,
    /// Interrupt Disable
    pub idr: Aliased<u32, IdrR::Register, IdrW::Register>,
    /// Interrupt Un-Mask (enabled)
    pub imr: ReadOnly<u32, Imr::Register>,
    /// LQSPI Enable
    pub lqspi_en: Aliased<u32, LqspiEnR::Register, LqspiEnW::Register>,
    /// Timing Control Delay
    pub delay: ReadWrite<u32, Delay::Register>,
    /// Transmit Data, 4 Bytes
    pub txd0: WriteOnly<u32>,
    /// Receive Data in RX FIFO
    pub rx_data: ReadOnly<u32>,
    /// Slave Idle Count
    pub slave_idle_count: Aliased<u32, SlaveIdleCountR::Register, SlaveIdleCountW::Register>,
    /// TX FIFO Threshold
    pub tx_thres: ReadWrite<u32>,
    /// RX FIFO Threshold
    pub rx_thres: ReadWrite<u32>,
    /// Write Protection Output
    pub gpio: ReadWrite<u32, Gpio::Register>,
    _padding52: [u8; 4],
    /// Loopback Master Clock Delay Adjustment
    pub lpbk_dly_adj: ReadWrite<u32, LpbkDlyAdj::Register>,
    _padding60: [u8; 68],
    /// Transmit Data, 1 Byte
    pub txd1: WriteOnly<u32>,
    /// Transmit Data, 2 Byte
    pub txd2: WriteOnly<u32>,
    /// Transmit Data, 3 Bytes
    pub txd3: WriteOnly<u32>,
    _padding140: [u8; 20],
    /// Configuration
    pub lqspi_cfg: ReadWrite<u32, LqspiCfg::Register>,
    /// Status
    pub lqspi_sts: ReadOnly<u16, LqspiSts::Register>,
    _padding166: [u8; 26],
    /// Command control
    pub command: Aliased<u32, CommandR::Register, CommandW::Register>,
    /// Transfer Size
    pub transfer_size: Aliased<u32, TransferSizeR::Register, TransferSizeW::Register>,
    /// Dummy Cycles Enable
    pub dummy_cycle_en: Aliased<u32, DummyCycleEnR::Register, DummyCycleEnW::Register>,
    _padding204: [u8; 48],
    /// Module Identification
    pub mod_id: ReadWrite<u32>,
    /// GQSPI Configuration
    pub gqspi_cfg: Aliased<u32, GqspiCfgR::Register, GqspiCfgW::Register>,
    /// Generic QSPI Interrupt Status
    pub gqspi_isr: Aliased<u32, GqspiIsrR::Register, GqspiIsrW::Register>,
    /// GQSPI Interrupt Enable
    pub gqspi_ier: Aliased<u32, GqspiIerR::Register, GqspiIerW::Register>,
    /// GQSPI Interrupt disable
    pub gqspi_idr: Aliased<u32, GqspiIdrR::Register, GqspiIdrW::Register>,
    /// GQSPI Interrupt Mask
    pub gqspi_imr: ReadOnly<u32, GqspiImr::Register>,
    /// GQSPI_Enable
    pub gqspi_en: Aliased<u32, GqspiEnR::Register, GqspiEnW::Register>,
    _padding280: [u8; 4],
    /// GQSPI Transmit Data
    pub gqspi_txd: WriteOnly<u32>,
    /// GQSPI Receive Data
    pub gqspi_rxd: ReadOnly<u32>,
    _padding292: [u8; 4],
    /// GQSPI TX FIFO Threshold Level
    pub gqspi_tx_thresh: Aliased<u32, GqspiTxThreshR::Register, GqspiTxThreshW::Register>,
    /// GQSPI RX FIFO Threshold Level
    pub gqspi_rx_thresh: Aliased<u32, GqspiRxThreshR::Register, GqspiRxThreshW::Register>,
    /// GQSPI GPIO for Write Protect
    pub gqspi_gpio: Aliased<u32, GqspiGpioR::Register, GqspiGpioW::Register>,
    _padding308: [u8; 4],
    /// GQSPI Loopback clock delay adjustment Register
    pub gqspi_lpbk_dly_adj: Aliased<u32, GqspiLpbkDlyAdjR::Register, GqspiLpbkDlyAdjW::Register>,
    _padding316: [u8; 4],
    /// GQSPI Generic FIFO Configuration
    pub gqspi_gen_fifo: Aliased<u32, GqspiGenFifoR::Register, GqspiGenFifoW::Register>,
    /// GQSPI Select
    pub gqspi_sel: Aliased<u32, GqspiSelR::Register, GqspiSelW::Register>,
    _padding328: [u8; 4],
    /// GQSPI FIFO Control
    pub gqspi_fifo_ctrl: Aliased<u32, GqspiFifoCtrlR::Register, GqspiFifoCtrlW::Register>,
    /// GQSPI Generic FIFO ThresholdLevel
    pub gqspi_gf_thresh: Aliased<u32, GqspiGfThreshR::Register, GqspiGfThreshW::Register>,
    /// GQSPI Poll Configuration Register
    pub gqspi_poll_cfg: Aliased<u32, GqspiPollCfgR::Register, GqspiPollCfgW::Register>,
    /// GQSPI Poll Time out
    pub gqspi_p_timeout: ReadWrite<u32>,
    /// GQSPI Transfer Status
    pub gqspi_xfer_sts: ReadOnly<u32>,
    _padding352: [u8; 4],
    /// GQSPI Receive Data Copy
    pub gqspi_rx_copy: ReadOnly<u32, GqspiRxCopy::Register>,
    _padding360: [u8; 144],
    /// QSPI RX Data Delay
    pub qspi_data_dly_adj: ReadWrite<u32, QspiDataDlyAdj::Register>,
    /// GQSPI Module Identification register
    pub gqspi_mod_id: ReadWrite<u32>,
    _padding512: [u8; 1536],
    /// DMA destination memory address
    pub qspidma_dst_addr: Aliased<u32, QspidmaDstAddrR::Register, QspidmaDstAddrW::Register>,
    /// DMA transfer payload
    pub qspidma_dst_size: Aliased<u32, QspidmaDstSizeR::Register, QspidmaDstSizeW::Register>,
    /// General DST DMA Status
    pub qspidma_dst_sts: Aliased<u32, QspidmaDstStsR::Register, QspidmaDstStsW::Register>,
    /// General DST DMA Control
    pub qspidma_dst_ctrl: ReadWrite<u32, QspidmaDstCtrl::Register>,
    _padding2064: [u8; 4],
    /// DST DMA Interrupt Status
    pub qspidma_dst_i_sts: Aliased<u32, QspidmaDstIStsR::Register, QspidmaDstIStsW::Register>,
    /// DST DMA Interrupt Enable
    pub qspidma_dst_i_en: Aliased<u32, QspidmaDstIEnR::Register, QspidmaDstIEnW::Register>,
    /// DST DMA Interrupt Disable
    pub qspidma_dst_i_dis: Aliased<u32, QspidmaDstIDisR::Register, QspidmaDstIDisW::Register>,
    /// DST DMA Interrupt Mask
    pub qspidma_dst_i_mask: ReadOnly<u32, QspidmaDstIMask::Register>,
    /// General DST DMA Control Reg 2
    pub qspidma_dst_ctrl2: Aliased<u32, QspidmaDstCtrl2R::Register, QspidmaDstCtrl2W::Register>,
    /// DMA destination memory address (MSBs)
    pub qspidma_dst_addr_msb:
        Aliased<u32, QspidmaDstAddrMsbR::Register, QspidmaDstAddrMsbW::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub ConfigR [
        LEG_FLSH OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(27) NUMBITS(4) [],
        ENDIAN OFFSET(26) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(6) [],
        HOLDB_DR OFFSET(19) NUMBITS(1) [],
        RESERVED2 OFFSET(17) NUMBITS(2) [],
        MAN_START_EN OFFSET(15) NUMBITS(1) [],
        MANUAL_CS OFFSET(14) NUMBITS(1) [],
        RESERVED3 OFFSET(11) NUMBITS(3) [],
        PCS OFFSET(10) NUMBITS(1) [],
        RESERVED4 OFFSET(9) NUMBITS(1) [],
        REF_CLK OFFSET(8) NUMBITS(1) [],
        FIFO_WIDTH OFFSET(6) NUMBITS(2) [],
        BAUD_RATE_DIV OFFSET(3) NUMBITS(3) [],
        CLK_PH OFFSET(2) NUMBITS(1) [],
        CLK_POL OFFSET(1) NUMBITS(1) [],
        MODE_SEL OFFSET(0) NUMBITS(1) [],
    ],
    pub ConfigW [
        LEG_FLSH OFFSET(31) NUMBITS(1) [],
        ENDIAN OFFSET(26) NUMBITS(1) [],
        HOLDB_DR OFFSET(19) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(2) [],
        MAN_START_COM OFFSET(16) NUMBITS(1) [],
        MAN_START_EN OFFSET(15) NUMBITS(1) [],
        MANUAL_CS OFFSET(14) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(3) [],
        PCS OFFSET(10) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        REF_CLK OFFSET(8) NUMBITS(1) [],
        FIFO_WIDTH OFFSET(6) NUMBITS(2) [],
        BAUD_RATE_DIV OFFSET(3) NUMBITS(3) [],
        CLK_PH OFFSET(2) NUMBITS(1) [],
        CLK_POL OFFSET(1) NUMBITS(1) [],
        MODE_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IsrR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        TXFIFO_EMPTY OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ],
    pub IsrW [
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IerR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
    ],
    pub IerW [
        TXFIFO_EMPTY OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdrR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
    ],
    pub IdrW [
        TXFIFO_EMPTY OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imr [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        TXFIFO_EMPTY OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub LqspiEnR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SPI_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub LqspiEnW [
        SPI_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Delay [
        D_NSS OFFSET(24) NUMBITS(8) [],
        D_BTWN OFFSET(16) NUMBITS(8) [],
        D_AFTER OFFSET(8) NUMBITS(8) [],
        D_INT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SlaveIdleCountR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ],
    pub SlaveIdleCountW [
        RESERVED0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gpio [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        WP_N OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub LpbkDlyAdj [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        USE_LPBK OFFSET(5) NUMBITS(1) [],
        DLY1 OFFSET(3) NUMBITS(2) [],
        DLY0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub LqspiCfg [
        LQ_MODE OFFSET(31) NUMBITS(1) [],
        TWO_MEM OFFSET(30) NUMBITS(1) [],
        SEP_BUS OFFSET(29) NUMBITS(1) [],
        U_PAGE OFFSET(28) NUMBITS(1) [],
        ADDR_32BIT OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(1) [],
        MODE_EN OFFSET(25) NUMBITS(1) [],
        MODE_ON OFFSET(24) NUMBITS(1) [],
        MODE_BITS OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(11) NUMBITS(5) [],
        DUMMY_BYTE OFFSET(8) NUMBITS(3) [],
        INST_CODE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub LqspiSts [
        RESERVED0 OFFSET(3) NUMBITS(6) [],
        D_FSM_ERR OFFSET(2) NUMBITS(1) [],
        WR_RECVD OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CommandR [
        RESERVED0 OFFSET(21) NUMBITS(11) [],
        RXFIFO_DRAIN_STATUS OFFSET(19) NUMBITS(1) [],
        PARTIAL_BYTE_LEN OFFSET(16) NUMBITS(3) [],
        RESERVED1 OFFSET(15) NUMBITS(1) [],
        RX_DISCARD_REG OFFSET(8) NUMBITS(7) [],
        DUMMY_CYCLES OFFSET(2) NUMBITS(6) [],
        DMA_EN OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub CommandW [
        RXFIFO_DRAIN OFFSET(20) NUMBITS(1) [],
        PARTIAL_BYTE_LEN OFFSET(16) NUMBITS(3) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        RX_DISCARD_REG OFFSET(8) NUMBITS(7) [],
        DUMMY_CYCLES OFFSET(2) NUMBITS(6) [],
        DMA_EN OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TransferSizeR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        RESERVED1 OFFSET(0) NUMBITS(2) [],
    ],
    pub TransferSizeW [
        SIZE OFFSET(2) NUMBITS(27) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DummyCycleEnR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        DUMMY_CYCLE_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DummyCycleEnW [
        DUMMY_CYCLE_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiCfgR [
        MODE_EN OFFSET(30) NUMBITS(2) [],
        GEN_FIFO_START_MODE OFFSET(29) NUMBITS(1) [],
        RESERVED0 OFFSET(27) NUMBITS(1) [],
        ENDIAN OFFSET(26) NUMBITS(1) [],
        RESERVED1 OFFSET(21) NUMBITS(5) [],
        EN_POLL_TIMEOUT OFFSET(20) NUMBITS(1) [],
        WP_HOLD OFFSET(19) NUMBITS(1) [],
        RESERVED2 OFFSET(6) NUMBITS(13) [],
        BAUD_RATE_DIV OFFSET(3) NUMBITS(3) [],
        CLK_PH OFFSET(2) NUMBITS(1) [],
        CLK_POL OFFSET(1) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub GqspiCfgW [
        MODE_EN OFFSET(30) NUMBITS(2) [],
        GEN_FIFO_START_MODE OFFSET(29) NUMBITS(1) [],
        START_GEN_FIFO OFFSET(28) NUMBITS(1) [],
        ENDIAN OFFSET(26) NUMBITS(1) [],
        EN_POLL_TIMEOUT OFFSET(20) NUMBITS(1) [],
        WP_HOLD OFFSET(19) NUMBITS(1) [],
        BAUD_RATE_DIV OFFSET(3) NUMBITS(3) [],
        CLK_PH OFFSET(2) NUMBITS(1) [],
        CLK_POL OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiIsrR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        RX_FIFO_EMPTY OFFSET(11) NUMBITS(1) [],
        GEN_FIFO_FULL OFFSET(10) NUMBITS(1) [],
        GEN_FIFO_NOT_FULL OFFSET(9) NUMBITS(1) [],
        TX_FIFO_EMPTY OFFSET(8) NUMBITS(1) [],
        GEN_FIFO_EMPTY OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        POLL_TIME_EXPIRE OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub GqspiIsrW [
        POLL_TIME_EXPIRE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiIerR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub GqspiIerW [
        RX_FIFO_EMPTY OFFSET(11) NUMBITS(1) [],
        GEN_FIFO_FULL OFFSET(10) NUMBITS(1) [],
        GEN_FIFO_NOT_FULL OFFSET(9) NUMBITS(1) [],
        TX_FIFO_EMPTY OFFSET(8) NUMBITS(1) [],
        GEN_FIFO_EMPTY OFFSET(7) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        POLL_TIME_EXPIRE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiIdrR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub GqspiIdrW [
        RX_FIFO_EMPTY OFFSET(11) NUMBITS(1) [],
        GEN_FIFO_FULL OFFSET(10) NUMBITS(1) [],
        GEN_FIFO_NOT_FULL OFFSET(9) NUMBITS(1) [],
        TX_FIFO_EMPTY OFFSET(8) NUMBITS(1) [],
        GEN_FIFO_EMPTY OFFSET(7) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        POLL_TIME_EXPIRE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiImr [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        RX_FIFO_EMPTY OFFSET(11) NUMBITS(1) [],
        GEN_FIFO_FULL OFFSET(10) NUMBITS(1) [],
        GEN_FIFO_NOT_FULL OFFSET(9) NUMBITS(1) [],
        TX_FIFO_EMPTY OFFSET(8) NUMBITS(1) [],
        GEN_FIFO_EMPTY OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        POLL_TIME_EXPIRE OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiEnR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GQSPI_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub GqspiEnW [
        GQSPI_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiTxThreshR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        LEVEL_TX_FIFO OFFSET(0) NUMBITS(6) [],
    ],
    pub GqspiTxThreshW [
        LEVEL_TX_FIFO OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiRxThreshR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        LEVEL_RX_FIFO OFFSET(0) NUMBITS(6) [],
    ],
    pub GqspiRxThreshW [
        LEVEL_RX_FIFO OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiGpioR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        WP_N OFFSET(0) NUMBITS(1) [],
    ],
    pub GqspiGpioW [
        WP_N OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiLpbkDlyAdjR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        USE_LPBK OFFSET(5) NUMBITS(1) [],
        DLY1 OFFSET(3) NUMBITS(2) [],
        DLY0 OFFSET(0) NUMBITS(3) [],
    ],
    pub GqspiLpbkDlyAdjW [
        USE_LPBK OFFSET(5) NUMBITS(1) [],
        DLY1 OFFSET(3) NUMBITS(2) [],
        DLY0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiGenFifoR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
    ],
    pub GqspiGenFifoW [
        GEN_DATA OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiSelR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GENERIC_QSPI_SEL OFFSET(0) NUMBITS(1) [],
    ],
    pub GqspiSelW [
        GENERIC_QSPI_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiFifoCtrlR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
    ],
    pub GqspiFifoCtrlW [
        RST_RX_FIFO OFFSET(2) NUMBITS(1) [],
        RST_TX_FIFO OFFSET(1) NUMBITS(1) [],
        RST_GEN_FIFO OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiGfThreshR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        LEVEL_GF_FIFO OFFSET(0) NUMBITS(5) [],
    ],
    pub GqspiGfThreshW [
        LEVEL_GF_FIFO OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiPollCfgR [
        EN_MASK_UPPER OFFSET(31) NUMBITS(1) [],
        EN_MASK_LOWER OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(14) [],
        MASK_EN OFFSET(8) NUMBITS(8) [],
        DATA_VALUE OFFSET(0) NUMBITS(8) [],
    ],
    pub GqspiPollCfgW [
        EN_MASK_UPPER OFFSET(31) NUMBITS(1) [],
        EN_MASK_LOWER OFFSET(30) NUMBITS(1) [],
        MASK_EN OFFSET(8) NUMBITS(8) [],
        DATA_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GqspiRxCopy [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RESERVED1 OFFSET(8) NUMBITS(8) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspiDataDlyAdj [
        USE_DATA_DLY OFFSET(31) NUMBITS(1) [],
        DATA_DLY_ADJ OFFSET(28) NUMBITS(3) [],
        RESERVED0 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspidmaDstAddrR [
        RESERVED0 OFFSET(0) NUMBITS(2) [],
    ],
    pub QspidmaDstAddrW [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspidmaDstSizeR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        RESERVED1 OFFSET(0) NUMBITS(2) [],
    ],
    pub QspidmaDstSizeW [
        SIZE OFFSET(2) NUMBITS(27) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspidmaDstStsR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        DONE_CNT OFFSET(13) NUMBITS(3) [],
        RESERVED1 OFFSET(5) NUMBITS(8) [],
        RESERVED2 OFFSET(1) NUMBITS(4) [],
        BUSY OFFSET(0) NUMBITS(1) [],
    ],
    pub QspidmaDstStsW [
        DONE_CNT OFFSET(13) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspidmaDstCtrl [
        FIFO_LVL_HIT_THRESH OFFSET(25) NUMBITS(7) [],
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
    pub QspidmaDstIStsR [
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
    pub QspidmaDstIStsW [
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
    pub QspidmaDstIEnR [
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
    pub QspidmaDstIEnW [
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
    pub QspidmaDstIDisR [
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
    pub QspidmaDstIDisW [
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
    pub QspidmaDstIMask [
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
    pub QspidmaDstCtrl2R [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(27) NUMBITS(1) [],
        AWCACHE OFFSET(24) NUMBITS(3) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        TIMEOUT_EN OFFSET(22) NUMBITS(1) [],
        RESERVED3 OFFSET(19) NUMBITS(3) [],
        RESERVED4 OFFSET(16) NUMBITS(3) [],
        TIMEOUT_PRE OFFSET(4) NUMBITS(12) [],
        MAX_OUTS_CMDS OFFSET(0) NUMBITS(4) [],
    ],
    pub QspidmaDstCtrl2W [
        RESERVED0 OFFSET(27) NUMBITS(1) [],
        AWCACHE OFFSET(24) NUMBITS(3) [],
        TIMEOUT_EN OFFSET(22) NUMBITS(1) [],
        RESERVED1 OFFSET(19) NUMBITS(3) [],
        RESERVED2 OFFSET(16) NUMBITS(3) [],
        TIMEOUT_PRE OFFSET(4) NUMBITS(12) [],
        MAX_OUTS_CMDS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspidmaDstAddrMsbR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
    ],
    pub QspidmaDstAddrMsbW [
        ADDR_MSB OFFSET(0) NUMBITS(12) [],
    ]
];

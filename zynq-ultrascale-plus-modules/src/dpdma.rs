// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// DisplayPort DMA, DisplayPort DMA
pub static mut DPDMA: *mut Registers = 0xfd4c0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Enable/Disable a error response
        (0x00000000 => pub dpdma_err_ctrl: Aliased<u32, DpdmaErrCtrlR::Register, DpdmaErrCtrlW::Register>),
        /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
        (0x00000004 => pub dpdma_isr: ReadWrite<u32, DpdmaIsr::Register>),
        /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
        (0x00000008 => pub dpdma_imr: ReadOnly<u32, DpdmaImr::Register>),
        /// Interrupt Enable Register. A write of 1 to this location will unmask the interrupt. (IMR: 0)
        (0x0000000c => pub dpdma_ien: WriteOnly<u32, DpdmaIen::Register>),
        /// Interrupt Disable Register. A write of 1 one to this location will mask the interrupt. (IMR: 1)
        (0x00000010 => pub dpdma_ids: WriteOnly<u32, DpdmaIds::Register>),
        /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
        (0x00000014 => pub dpdma_eisr: ReadWrite<u32, DpdmaEisr::Register>),
        /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
        (0x00000018 => pub dpdma_eimr: ReadOnly<u32, DpdmaEimr::Register>),
        /// Interrupt Enable Register. A write of 1 to this location will unmask the interrupt. (IMR: 0)
        (0x0000001c => pub dpdma_eien: WriteOnly<u32, DpdmaEien::Register>),
        /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
        (0x00000020 => pub dpdma_eids: WriteOnly<u32, DpdmaEids::Register>),
        (0x00000024 => _padding36),
        /// DPDMA Global control register, holds fields which control all 6 channels
        (0x00000100 => pub dpdma_cntl: ReadOnly<u32>),
        /// Global control register provides control to start or redirect any channel
        (0x00000104 => pub dpdma_gbl: Aliased<u32, DpdmaGblR::Register, DpdmaGblW::Register>),
        /// Global control register provides control to start or redirect any channel
        (0x00000108 => pub dpdma_alc0_cntl: Aliased<u32, DpdmaAlc0CntlR::Register, DpdmaAlc0CntlW::Register>),
        /// Status Register
        (0x0000010c => pub dpdma_alc0_status: ReadOnly<u32, DpdmaAlc0Status::Register>),
        /// ALC0 Max latency Register
        (0x00000110 => pub dpdma_alc0_max: ReadOnly<u32, DpdmaAlc0Max::Register>),
        /// ALC0 Min Latency Register
        (0x00000114 => pub dpdma_alc0_min: ReadOnly<u32, DpdmaAlc0Min::Register>),
        /// ALC0 Accumulated Transaction Latency Register
        (0x00000118 => pub dpdma_alc0_acc: ReadOnly<u32>),
        /// ALC1 Accumulated Transaction Count Register
        (0x0000011c => pub dpdma_alc0_acc_tran: ReadOnly<u32>),
        /// Global control register provides control to start or redirect any channel
        (0x00000120 => pub dpdma_alc1_cntl: Aliased<u32, DpdmaAlc1CntlR::Register, DpdmaAlc1CntlW::Register>),
        /// Status Register
        (0x00000124 => pub dpdma_alc1_status: ReadOnly<u32, DpdmaAlc1Status::Register>),
        /// ALC1 Max latency Register
        (0x00000128 => pub dpdma_alc1_max: ReadOnly<u32, DpdmaAlc1Max::Register>),
        /// ALC1 Min Latency Register
        (0x0000012c => pub dpdma_alc1_min: ReadOnly<u32, DpdmaAlc1Min::Register>),
        /// ALC1 Accumulated Transaction Latency Register
        (0x00000130 => pub dpdma_alc1_acc: ReadOnly<u32>),
        /// ALC1 Accumulated Transaction Count Register
        (0x00000134 => pub dpdma_alc1_acc_tran: ReadOnly<u32>),
        (0x00000138 => _padding312),
        /// Descriptor Start Address Extension Register
        (0x00000200 => pub dpdma_ch0_dscr_strt_addre: Aliased<u32, DpdmaCh0DscrStrtAddreR::Register, DpdmaCh0DscrStrtAddreW::Register>),
        /// Descriptor Start Address Register
        (0x00000204 => pub dpdma_ch0_dscr_strt_addr: ReadWrite<u32>),
        (0x00000208 => _padding520),
        /// Channel 0 Control Register
        (0x00000218 => pub dpdma_ch0_cntl: Aliased<u32, DpdmaCh0CntlR::Register, DpdmaCh0CntlW::Register>),
        (0x0000021c => _padding540),
        /// Descriptor Start Address Extension Register
        (0x00000300 => pub dpdma_ch1_dscr_strt_addre: Aliased<u32, DpdmaCh1DscrStrtAddreR::Register, DpdmaCh1DscrStrtAddreW::Register>),
        /// Descriptor Start Address Register
        (0x00000304 => pub dpdma_ch1_dscr_strt_addr: ReadWrite<u32>),
        (0x00000308 => _padding776),
        /// Channel 1 Control Register
        (0x00000318 => pub dpdma_ch1_cntl: Aliased<u32, DpdmaCh1CntlR::Register, DpdmaCh1CntlW::Register>),
        (0x0000031c => _padding796),
        /// Descriptor Start Address Extension Register
        (0x00000400 => pub dpdma_ch2_dscr_strt_addre: Aliased<u32, DpdmaCh2DscrStrtAddreR::Register, DpdmaCh2DscrStrtAddreW::Register>),
        /// Descriptor Start Address Register
        (0x00000404 => pub dpdma_ch2_dscr_strt_addr: ReadWrite<u32>),
        (0x00000408 => _padding1032),
        /// Channel 2 Control Register
        (0x00000418 => pub dpdma_ch2_cntl: Aliased<u32, DpdmaCh2CntlR::Register, DpdmaCh2CntlW::Register>),
        (0x0000041c => _padding1052),
        /// Descriptor Start Address Extension Register
        (0x00000500 => pub dpdma_ch3_dscr_strt_addre: Aliased<u32, DpdmaCh3DscrStrtAddreR::Register, DpdmaCh3DscrStrtAddreW::Register>),
        /// Descriptor Start Address Register
        (0x00000504 => pub dpdma_ch3_dscr_strt_addr: ReadWrite<u32>),
        (0x00000508 => _padding1288),
        /// Channel 3 Control Register
        (0x00000518 => pub dpdma_ch3_cntl: Aliased<u32, DpdmaCh3CntlR::Register, DpdmaCh3CntlW::Register>),
        (0x0000051c => _padding1308),
        /// Descriptor Start Address Extension Register
        (0x00000600 => pub dpdma_ch4_dscr_strt_addre: Aliased<u32, DpdmaCh4DscrStrtAddreR::Register, DpdmaCh4DscrStrtAddreW::Register>),
        /// Descriptor Start Address Register
        (0x00000604 => pub dpdma_ch4_dscr_strt_addr: ReadWrite<u32>),
        (0x00000608 => _padding1544),
        /// Channel 4 Control Register
        (0x00000618 => pub dpdma_ch4_cntl: Aliased<u32, DpdmaCh4CntlR::Register, DpdmaCh4CntlW::Register>),
        (0x0000061c => _padding1564),
        /// Descriptor Start Address Extension Register
        (0x00000700 => pub dpdma_ch5_dscr_strt_addre: Aliased<u32, DpdmaCh5DscrStrtAddreR::Register, DpdmaCh5DscrStrtAddreW::Register>),
        /// Descriptor Start Address Register
        (0x00000704 => pub dpdma_ch5_dscr_strt_addr: ReadWrite<u32>),
        (0x00000708 => _padding1800),
        /// Channel 4 Control Register
        (0x00000718 => pub dpdma_ch5_cntl: Aliased<u32, DpdmaCh5CntlR::Register, DpdmaCh5CntlW::Register>),
        (0x0000071c => @END),
    }
}
register_bitfields! [
    u32,
    pub DpdmaErrCtrlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ],
    pub DpdmaErrCtrlW [
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaIsr [
        VSYNC_INT OFFSET(27) NUMBITS(1) [],
        AXI_RD_4K_CROSS OFFSET(26) NUMBITS(1) [],
        WR_DATA_FIFO_FULL OFFSET(25) NUMBITS(1) [],
        WR_CMD_FIFO_FULL OFFSET(24) NUMBITS(1) [],
        DSCR_ERR5 OFFSET(23) NUMBITS(1) [],
        DSCR_ERR4 OFFSET(22) NUMBITS(1) [],
        DSCR_ERR3 OFFSET(21) NUMBITS(1) [],
        DSCR_ERR2 OFFSET(20) NUMBITS(1) [],
        DSCR_ERR1 OFFSET(19) NUMBITS(1) [],
        DSCR_ERR0 OFFSET(18) NUMBITS(1) [],
        DATA_AXI_ERR5 OFFSET(17) NUMBITS(1) [],
        DATA_AXI_ERR4 OFFSET(16) NUMBITS(1) [],
        DATA_AXI_ERR3 OFFSET(15) NUMBITS(1) [],
        DATA_AXI_ERR2 OFFSET(14) NUMBITS(1) [],
        DATA_AXI_ERR1 OFFSET(13) NUMBITS(1) [],
        DATA_AXI_ERR0 OFFSET(12) NUMBITS(1) [],
        NO_OSTAND_TRAN5 OFFSET(11) NUMBITS(1) [],
        NO_OSTAND_TRAN4 OFFSET(10) NUMBITS(1) [],
        NO_OSTAND_TRAN3 OFFSET(9) NUMBITS(1) [],
        NO_OSTAND_TRAN2 OFFSET(8) NUMBITS(1) [],
        NO_OSTAND_TRAN1 OFFSET(7) NUMBITS(1) [],
        NO_OSTAND_TRAN0 OFFSET(6) NUMBITS(1) [],
        DSCR_DONE5 OFFSET(5) NUMBITS(1) [],
        DSCR_DONE4 OFFSET(4) NUMBITS(1) [],
        DSCR_DONE3 OFFSET(3) NUMBITS(1) [],
        DSCR_DONE2 OFFSET(2) NUMBITS(1) [],
        DSCR_DONE1 OFFSET(1) NUMBITS(1) [],
        DSCR_DONE0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaImr [
        VSYNC_INT OFFSET(27) NUMBITS(1) [],
        AXI_RD_4K_CROSS OFFSET(26) NUMBITS(1) [],
        WR_DATA_FIFO_FULL OFFSET(25) NUMBITS(1) [],
        WR_CMD_FIFO_FULL OFFSET(24) NUMBITS(1) [],
        DSCR_ERR5 OFFSET(23) NUMBITS(1) [],
        DSCR_ERR4 OFFSET(22) NUMBITS(1) [],
        DSCR_ERR3 OFFSET(21) NUMBITS(1) [],
        DSCR_ERR2 OFFSET(20) NUMBITS(1) [],
        DSCR_ERR1 OFFSET(19) NUMBITS(1) [],
        DSCR_ERR0 OFFSET(18) NUMBITS(1) [],
        DATA_AXI_ERR5 OFFSET(17) NUMBITS(1) [],
        DATA_AXI_ERR4 OFFSET(16) NUMBITS(1) [],
        DATA_AXI_ERR3 OFFSET(15) NUMBITS(1) [],
        DATA_AXI_ERR2 OFFSET(14) NUMBITS(1) [],
        DATA_AXI_ERR1 OFFSET(13) NUMBITS(1) [],
        DATA_AXI_ERR0 OFFSET(12) NUMBITS(1) [],
        NO_OSTAND_TRAN5 OFFSET(11) NUMBITS(1) [],
        NO_OSTAND_TRAN4 OFFSET(10) NUMBITS(1) [],
        NO_OSTAND_TRAN3 OFFSET(9) NUMBITS(1) [],
        NO_OSTAND_TRAN2 OFFSET(8) NUMBITS(1) [],
        NO_OSTAND_TRAN1 OFFSET(7) NUMBITS(1) [],
        NO_OSTAND_TRAN0 OFFSET(6) NUMBITS(1) [],
        DSCR_DONE5 OFFSET(5) NUMBITS(1) [],
        DSCR_DONE4 OFFSET(4) NUMBITS(1) [],
        DSCR_DONE3 OFFSET(3) NUMBITS(1) [],
        DSCR_DONE2 OFFSET(2) NUMBITS(1) [],
        DSCR_DONE1 OFFSET(1) NUMBITS(1) [],
        DSCR_DONE0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaIen [
        VSYNC_INT OFFSET(27) NUMBITS(1) [],
        AXI_RD_4K_CROSS OFFSET(26) NUMBITS(1) [],
        WR_DATA_FIFO_FULL OFFSET(25) NUMBITS(1) [],
        WR_CMD_FIFO_FULL OFFSET(24) NUMBITS(1) [],
        DSCR_ERR5 OFFSET(23) NUMBITS(1) [],
        DSCR_ERR4 OFFSET(22) NUMBITS(1) [],
        DSCR_ERR3 OFFSET(21) NUMBITS(1) [],
        DSCR_ERR2 OFFSET(20) NUMBITS(1) [],
        DSCR_ERR1 OFFSET(19) NUMBITS(1) [],
        DSCR_ERR0 OFFSET(18) NUMBITS(1) [],
        DATA_AXI_ERR5 OFFSET(17) NUMBITS(1) [],
        DATA_AXI_ERR4 OFFSET(16) NUMBITS(1) [],
        DATA_AXI_ERR3 OFFSET(15) NUMBITS(1) [],
        DATA_AXI_ERR2 OFFSET(14) NUMBITS(1) [],
        DATA_AXI_ERR1 OFFSET(13) NUMBITS(1) [],
        DATA_AXI_ERR0 OFFSET(12) NUMBITS(1) [],
        NO_OSTAND_TRAN5 OFFSET(11) NUMBITS(1) [],
        NO_OSTAND_TRAN4 OFFSET(10) NUMBITS(1) [],
        NO_OSTAND_TRAN3 OFFSET(9) NUMBITS(1) [],
        NO_OSTAND_TRAN2 OFFSET(8) NUMBITS(1) [],
        NO_OSTAND_TRAN1 OFFSET(7) NUMBITS(1) [],
        NO_OSTAND_TRAN0 OFFSET(6) NUMBITS(1) [],
        DSCR_DONE5 OFFSET(5) NUMBITS(1) [],
        DSCR_DONE4 OFFSET(4) NUMBITS(1) [],
        DSCR_DONE3 OFFSET(3) NUMBITS(1) [],
        DSCR_DONE2 OFFSET(2) NUMBITS(1) [],
        DSCR_DONE1 OFFSET(1) NUMBITS(1) [],
        DSCR_DONE0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaIds [
        VSYNC_INT OFFSET(27) NUMBITS(1) [],
        AXI_RD_4K_CROSS OFFSET(26) NUMBITS(1) [],
        WR_DATA_FIFO_FULL OFFSET(25) NUMBITS(1) [],
        WR_CMD_FIFO_FULL OFFSET(24) NUMBITS(1) [],
        DSCR_ERR5 OFFSET(23) NUMBITS(1) [],
        DSCR_ERR4 OFFSET(22) NUMBITS(1) [],
        DSCR_ERR3 OFFSET(21) NUMBITS(1) [],
        DSCR_ERR2 OFFSET(20) NUMBITS(1) [],
        DSCR_ERR1 OFFSET(19) NUMBITS(1) [],
        DSCR_ERR0 OFFSET(18) NUMBITS(1) [],
        DATA_AXI_ERR5 OFFSET(17) NUMBITS(1) [],
        DATA_AXI_ERR4 OFFSET(16) NUMBITS(1) [],
        DATA_AXI_ERR3 OFFSET(15) NUMBITS(1) [],
        DATA_AXI_ERR2 OFFSET(14) NUMBITS(1) [],
        DATA_AXI_ERR1 OFFSET(13) NUMBITS(1) [],
        DATA_AXI_ERR0 OFFSET(12) NUMBITS(1) [],
        NO_OSTAND_TRAN5 OFFSET(11) NUMBITS(1) [],
        NO_OSTAND_TRAN4 OFFSET(10) NUMBITS(1) [],
        NO_OSTAND_TRAN3 OFFSET(9) NUMBITS(1) [],
        NO_OSTAND_TRAN2 OFFSET(8) NUMBITS(1) [],
        NO_OSTAND_TRAN1 OFFSET(7) NUMBITS(1) [],
        NO_OSTAND_TRAN0 OFFSET(6) NUMBITS(1) [],
        DSCR_DONE5 OFFSET(5) NUMBITS(1) [],
        DSCR_DONE4 OFFSET(4) NUMBITS(1) [],
        DSCR_DONE3 OFFSET(3) NUMBITS(1) [],
        DSCR_DONE2 OFFSET(2) NUMBITS(1) [],
        DSCR_DONE1 OFFSET(1) NUMBITS(1) [],
        DSCR_DONE0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaEisr [
        RD_CMD_FIFO_FULL OFFSET(31) NUMBITS(1) [],
        DSCR_DONE_ERR5 OFFSET(30) NUMBITS(1) [],
        DSCR_DONE_ERR4 OFFSET(29) NUMBITS(1) [],
        DSCR_DONE_ERR3 OFFSET(28) NUMBITS(1) [],
        DSCR_DONE_ERR2 OFFSET(27) NUMBITS(1) [],
        DSCR_DONE_ERR1 OFFSET(26) NUMBITS(1) [],
        DSCR_DONE_ERR0 OFFSET(25) NUMBITS(1) [],
        DSCR_WR_AXI_ERR5 OFFSET(24) NUMBITS(1) [],
        DSCR_WR_AXI_ERR4 OFFSET(23) NUMBITS(1) [],
        DSCR_WR_AXI_ERR3 OFFSET(22) NUMBITS(1) [],
        DSCR_WR_AXI_ERR2 OFFSET(21) NUMBITS(1) [],
        DSCR_WR_AXI_ERR1 OFFSET(20) NUMBITS(1) [],
        DSCR_WR_AXI_ERR0 OFFSET(19) NUMBITS(1) [],
        DSCR_CRC_ERR5 OFFSET(18) NUMBITS(1) [],
        DSCR_CRC_ERR4 OFFSET(17) NUMBITS(1) [],
        DSCR_CRC_ERR3 OFFSET(16) NUMBITS(1) [],
        DSCR_CRC_ERR2 OFFSET(15) NUMBITS(1) [],
        DSCR_CRC_ERR1 OFFSET(14) NUMBITS(1) [],
        DSCR_CRC_ERR0 OFFSET(13) NUMBITS(1) [],
        DSCR_PRE_ERR5 OFFSET(12) NUMBITS(1) [],
        DSCR_PRE_ERR4 OFFSET(11) NUMBITS(1) [],
        DSCR_PRE_ERR3 OFFSET(10) NUMBITS(1) [],
        DSCR_PRE_ERR2 OFFSET(9) NUMBITS(1) [],
        DSCR_PRE_ERR1 OFFSET(8) NUMBITS(1) [],
        DSCR_PRE_ERR0 OFFSET(7) NUMBITS(1) [],
        DSCR_RD_AXI_ERR5 OFFSET(6) NUMBITS(1) [],
        DSCR_RD_AXI_ERR4 OFFSET(5) NUMBITS(1) [],
        DSCR_RD_AXI_ERR3 OFFSET(4) NUMBITS(1) [],
        DSCR_RD_AXI_ERR2 OFFSET(3) NUMBITS(1) [],
        DSCR_RD_AXI_ERR1 OFFSET(2) NUMBITS(1) [],
        DSCR_RD_AXI_ERR0 OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaEimr [
        RD_CMD_FIFO_FULL OFFSET(31) NUMBITS(1) [],
        DSCR_DONE_ERR5 OFFSET(30) NUMBITS(1) [],
        DSCR_DONE_ERR4 OFFSET(29) NUMBITS(1) [],
        DSCR_DONE_ERR3 OFFSET(28) NUMBITS(1) [],
        DSCR_DONE_ERR2 OFFSET(27) NUMBITS(1) [],
        DSCR_DONE_ERR1 OFFSET(26) NUMBITS(1) [],
        DSCR_DONE_ERR0 OFFSET(25) NUMBITS(1) [],
        DSCR_WR_AXI_ERR5 OFFSET(24) NUMBITS(1) [],
        DSCR_WR_AXI_ERR4 OFFSET(23) NUMBITS(1) [],
        DSCR_WR_AXI_ERR3 OFFSET(22) NUMBITS(1) [],
        DSCR_WR_AXI_ERR2 OFFSET(21) NUMBITS(1) [],
        DSCR_WR_AXI_ERR1 OFFSET(20) NUMBITS(1) [],
        DSCR_WR_AXI_ERR0 OFFSET(19) NUMBITS(1) [],
        DSCR_CRC_ERR5 OFFSET(18) NUMBITS(1) [],
        DSCR_CRC_ERR4 OFFSET(17) NUMBITS(1) [],
        DSCR_CRC_ERR3 OFFSET(16) NUMBITS(1) [],
        DSCR_CRC_ERR2 OFFSET(15) NUMBITS(1) [],
        DSCR_CRC_ERR1 OFFSET(14) NUMBITS(1) [],
        DSCR_CRC_ERR0 OFFSET(13) NUMBITS(1) [],
        DSCR_PRE_ERR5 OFFSET(12) NUMBITS(1) [],
        DSCR_PRE_ERR4 OFFSET(11) NUMBITS(1) [],
        DSCR_PRE_ERR3 OFFSET(10) NUMBITS(1) [],
        DSCR_PRE_ERR2 OFFSET(9) NUMBITS(1) [],
        DSCR_PRE_ERR1 OFFSET(8) NUMBITS(1) [],
        DSCR_PRE_ERR0 OFFSET(7) NUMBITS(1) [],
        DSCR_RD_AXI_ERR5 OFFSET(6) NUMBITS(1) [],
        DSCR_RD_AXI_ERR4 OFFSET(5) NUMBITS(1) [],
        DSCR_RD_AXI_ERR3 OFFSET(4) NUMBITS(1) [],
        DSCR_RD_AXI_ERR2 OFFSET(3) NUMBITS(1) [],
        DSCR_RD_AXI_ERR1 OFFSET(2) NUMBITS(1) [],
        DSCR_RD_AXI_ERR0 OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaEien [
        RD_CMD_FIFO_FULL OFFSET(31) NUMBITS(1) [],
        DSCR_DONE_ERR5 OFFSET(30) NUMBITS(1) [],
        DSCR_DONE_ERR4 OFFSET(29) NUMBITS(1) [],
        DSCR_DONE_ERR3 OFFSET(28) NUMBITS(1) [],
        DSCR_DONE_ERR2 OFFSET(27) NUMBITS(1) [],
        DSCR_DONE_ERR1 OFFSET(26) NUMBITS(1) [],
        DSCR_DONE_ERR0 OFFSET(25) NUMBITS(1) [],
        DSCR_WR_AXI_ERR5 OFFSET(24) NUMBITS(1) [],
        DSCR_WR_AXI_ERR4 OFFSET(23) NUMBITS(1) [],
        DSCR_WR_AXI_ERR3 OFFSET(22) NUMBITS(1) [],
        DSCR_WR_AXI_ERR2 OFFSET(21) NUMBITS(1) [],
        DSCR_WR_AXI_ERR1 OFFSET(20) NUMBITS(1) [],
        DSCR_WR_AXI_ERR0 OFFSET(19) NUMBITS(1) [],
        DSCR_CRC_ERR5 OFFSET(18) NUMBITS(1) [],
        DSCR_CRC_ERR4 OFFSET(17) NUMBITS(1) [],
        DSCR_CRC_ERR3 OFFSET(16) NUMBITS(1) [],
        DSCR_CRC_ERR2 OFFSET(15) NUMBITS(1) [],
        DSCR_CRC_ERR1 OFFSET(14) NUMBITS(1) [],
        DSCR_CRC_ERR0 OFFSET(13) NUMBITS(1) [],
        DSCR_PRE_ERR5 OFFSET(12) NUMBITS(1) [],
        DSCR_PRE_ERR4 OFFSET(11) NUMBITS(1) [],
        DSCR_PRE_ERR3 OFFSET(10) NUMBITS(1) [],
        DSCR_PRE_ERR2 OFFSET(9) NUMBITS(1) [],
        DSCR_PRE_ERR1 OFFSET(8) NUMBITS(1) [],
        DSCR_PRE_ERR0 OFFSET(7) NUMBITS(1) [],
        DSCR_RD_AXI_ERR5 OFFSET(6) NUMBITS(1) [],
        DSCR_RD_AXI_ERR4 OFFSET(5) NUMBITS(1) [],
        DSCR_RD_AXI_ERR3 OFFSET(4) NUMBITS(1) [],
        DSCR_RD_AXI_ERR2 OFFSET(3) NUMBITS(1) [],
        DSCR_RD_AXI_ERR1 OFFSET(2) NUMBITS(1) [],
        DSCR_RD_AXI_ERR0 OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaEids [
        RD_CMD_FIFO_FULL OFFSET(31) NUMBITS(1) [],
        DSCR_DONE_ERR5 OFFSET(30) NUMBITS(1) [],
        DSCR_DONE_ERR4 OFFSET(29) NUMBITS(1) [],
        DSCR_DONE_ERR3 OFFSET(28) NUMBITS(1) [],
        DSCR_DONE_ERR2 OFFSET(27) NUMBITS(1) [],
        DSCR_DONE_ERR1 OFFSET(26) NUMBITS(1) [],
        DSCR_DONE_ERR0 OFFSET(25) NUMBITS(1) [],
        DSCR_WR_AXI_ERR5 OFFSET(24) NUMBITS(1) [],
        DSCR_WR_AXI_ERR4 OFFSET(23) NUMBITS(1) [],
        DSCR_WR_AXI_ERR3 OFFSET(22) NUMBITS(1) [],
        DSCR_WR_AXI_ERR2 OFFSET(21) NUMBITS(1) [],
        DSCR_WR_AXI_ERR1 OFFSET(20) NUMBITS(1) [],
        DSCR_WR_AXI_ERR0 OFFSET(19) NUMBITS(1) [],
        DSCR_CRC_ERR5 OFFSET(18) NUMBITS(1) [],
        DSCR_CRC_ERR4 OFFSET(17) NUMBITS(1) [],
        DSCR_CRC_ERR3 OFFSET(16) NUMBITS(1) [],
        DSCR_CRC_ERR2 OFFSET(15) NUMBITS(1) [],
        DSCR_CRC_ERR1 OFFSET(14) NUMBITS(1) [],
        DSCR_CRC_ERR0 OFFSET(13) NUMBITS(1) [],
        DSCR_PRE_ERR5 OFFSET(12) NUMBITS(1) [],
        DSCR_PRE_ERR4 OFFSET(11) NUMBITS(1) [],
        DSCR_PRE_ERR3 OFFSET(10) NUMBITS(1) [],
        DSCR_PRE_ERR2 OFFSET(9) NUMBITS(1) [],
        DSCR_PRE_ERR1 OFFSET(8) NUMBITS(1) [],
        DSCR_PRE_ERR0 OFFSET(7) NUMBITS(1) [],
        DSCR_RD_AXI_ERR5 OFFSET(6) NUMBITS(1) [],
        DSCR_RD_AXI_ERR4 OFFSET(5) NUMBITS(1) [],
        DSCR_RD_AXI_ERR3 OFFSET(4) NUMBITS(1) [],
        DSCR_RD_AXI_ERR2 OFFSET(3) NUMBITS(1) [],
        DSCR_RD_AXI_ERR1 OFFSET(2) NUMBITS(1) [],
        DSCR_RD_AXI_ERR0 OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaGblR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
    ],
    pub DpdmaGblW [
        RTRG_CH5 OFFSET(11) NUMBITS(1) [],
        RTRG_CH4 OFFSET(10) NUMBITS(1) [],
        RTRG_CH3 OFFSET(9) NUMBITS(1) [],
        RTRG_CH2 OFFSET(8) NUMBITS(1) [],
        RTRG_CH1 OFFSET(7) NUMBITS(1) [],
        RTRG_CH0 OFFSET(6) NUMBITS(1) [],
        TRG_CH5 OFFSET(5) NUMBITS(1) [],
        TRG_CH4 OFFSET(4) NUMBITS(1) [],
        TRG_CH3 OFFSET(3) NUMBITS(1) [],
        TRG_CH2 OFFSET(2) NUMBITS(1) [],
        TRG_CH1 OFFSET(1) NUMBITS(1) [],
        TRG_CH0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaAlc0CntlR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        MON_ID OFFSET(2) NUMBITS(4) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpdmaAlc0CntlW [
        MON_ID OFFSET(2) NUMBITS(4) [],
        CLEAR OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaAlc0Status [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        OFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaAlc0Max [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        LATENCY OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaAlc0Min [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        LATENCY OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaAlc1CntlR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        MON_ID OFFSET(2) NUMBITS(4) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpdmaAlc1CntlW [
        MON_ID OFFSET(2) NUMBITS(4) [],
        CLEAR OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaAlc1Status [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        OFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaAlc1Max [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        LATENCY OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaAlc1Min [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        LATENCY OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh0DscrStrtAddreR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        MSB OFFSET(0) NUMBITS(16) [],
    ],
    pub DpdmaCh0DscrStrtAddreW [
        MSB OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh0CntlR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSCR_DLY_CNT OFFSET(20) NUMBITS(10) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpdmaCh0CntlW [
        DSCR_DLY_CNT OFFSET(20) NUMBITS(10) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh1DscrStrtAddreR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        MSB OFFSET(0) NUMBITS(16) [],
    ],
    pub DpdmaCh1DscrStrtAddreW [
        MSB OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh1CntlR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSCR_DLY_CNT OFFSET(20) NUMBITS(10) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpdmaCh1CntlW [
        DSCR_DLY_CNT OFFSET(20) NUMBITS(10) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh2DscrStrtAddreR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        MSB OFFSET(0) NUMBITS(16) [],
    ],
    pub DpdmaCh2DscrStrtAddreW [
        MSB OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh2CntlR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSCR_DLY_CNT OFFSET(20) NUMBITS(10) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpdmaCh2CntlW [
        DSCR_DLY_CNT OFFSET(20) NUMBITS(10) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh3DscrStrtAddreR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        MSB OFFSET(0) NUMBITS(16) [],
    ],
    pub DpdmaCh3DscrStrtAddreW [
        MSB OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh3CntlR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSCR_DLY_CNT OFFSET(20) NUMBITS(10) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpdmaCh3CntlW [
        DSCR_DLY_CNT OFFSET(20) NUMBITS(10) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh4DscrStrtAddreR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        MSB OFFSET(0) NUMBITS(16) [],
    ],
    pub DpdmaCh4DscrStrtAddreW [
        MSB OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh4CntlR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpdmaCh4CntlW [
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh5DscrStrtAddreR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        MSB OFFSET(0) NUMBITS(16) [],
    ],
    pub DpdmaCh5DscrStrtAddreW [
        MSB OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaCh5CntlR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpdmaCh5CntlW [
        DSCR_AXCACHE OFFSET(16) NUMBITS(4) [],
        DSCR_AXPROT OFFSET(14) NUMBITS(2) [],
        QOS_DATA_RD OFFSET(10) NUMBITS(4) [],
        QOS_DSCR_RD OFFSET(6) NUMBITS(4) [],
        QOS_DSCR_WR OFFSET(2) NUMBITS(4) [],
        PAUSE OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];

// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// PCIe Bridge - Main Control and Status, PCIe Bridge - Main Control and Status
pub static mut AXIPCIE_MAIN: *mut Registers = 0xfd0e0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// PCI Express Receive Access and BAR Configuration
    pub bridge_core_cfg_pcie_rx0:
        Aliased<u32, BridgeCoreCfgPcieRx0R::Register, BridgeCoreCfgPcieRx0W::Register>,
    /// PCI Express Receive Transaction Attribute Handling
    pub bridge_core_cfg_pcie_rx1:
        Aliased<u32, BridgeCoreCfgPcieRx1R::Register, BridgeCoreCfgPcieRx1W::Register>,
    /// AXI Master Max Payload Size Configuration
    pub bridge_core_cfg_axi_master:
        Aliased<u32, BridgeCoreCfgAxiMasterR::Register, BridgeCoreCfgAxiMasterW::Register>,
    /// PCI Express Transmit Cut Through Configuration
    pub bridge_core_cfg_pcie_tx:
        Aliased<u32, BridgeCoreCfgPcieTxR::Register, BridgeCoreCfgPcieTxW::Register>,
    /// PCI Express Core Interrupt Routing Configuration
    pub bridge_core_cfg_interrupt:
        Aliased<u32, BridgeCoreCfgInterruptR::Register, BridgeCoreCfgInterruptW::Register>,
    /// ECC RAM 1-bit Error Correction Enable/Disable (designs with ECC support only)
    pub bridge_core_cfg_ram_disable0:
        Aliased<u32, BridgeCoreCfgRamDisable0R::Register, BridgeCoreCfgRamDisable0W::Register>,
    /// ECC RAM 2-bit Error Handling Enable/Disable (designs with ECC support only)
    pub bridge_core_cfg_ram_disable1:
        Aliased<u32, BridgeCoreCfgRamDisable1R::Register, BridgeCoreCfgRamDisable1W::Register>,
    /// PCI Express Receive Completion Ordering Configuration
    pub bridge_core_cfg_pcie_relaxed_order: Aliased<
        u32,
        BridgeCoreCfgPcieRelaxedOrderR::Register,
        BridgeCoreCfgPcieRelaxedOrderW::Register,
    >,
    /// PCI Express Receive Message Filtering Configuration
    pub bridge_core_cfg_pcie_rx_msg_filter: Aliased<
        u32,
        BridgeCoreCfgPcieRxMsgFilterR::Register,
        BridgeCoreCfgPcieRxMsgFilterW::Register,
    >,
    /// PCI Express and AXI Read Reorder Queue Completion Ordering Configuration
    pub bridge_core_cfg_rq_req_order:
        Aliased<u32, BridgeCoreCfgRqReqOrderR::Register, BridgeCoreCfgRqReqOrderW::Register>,
    /// PCI Express Transmit Completion Header and Data Credit Metering Configuration
    pub bridge_core_cfg_pcie_credit:
        Aliased<u32, BridgeCoreCfgPcieCreditR::Register, BridgeCoreCfgPcieCreditW::Register>,
    /// AXI Master Write Completion Timeout Configuration
    pub bridge_core_cfg_axi_m_w_tick_count: Aliased<
        u32,
        BridgeCoreCfgAxiMWTickCountR::Register,
        BridgeCoreCfgAxiMWTickCountW::Register,
    >,
    /// AXI Master Read Completion Timeout Configuration
    pub bridge_core_cfg_axi_m_r_tick_count: Aliased<
        u32,
        BridgeCoreCfgAxiMRTickCountR::Register,
        BridgeCoreCfgAxiMRTickCountW::Register,
    >,
    /// PCIe Configuration Write/Read Request CRS Replay Timeout Configuration
    pub bridge_core_cfg_crs_rpl_tick_count: Aliased<
        u32,
        BridgeCoreCfgCrsRplTickCountR::Register,
        BridgeCoreCfgCrsRplTickCountW::Register,
    >,
    _padding56: [u8; 456],
    /// Egress Bridge Register Translation - Capabilities
    pub e_breg_capabilities: ReadOnly<u32, EBregCapabilities::Register>,
    /// Egress Bridge Register Translation - Status
    pub e_breg_status: ReadOnly<u32, EBregStatus::Register>,
    /// Egress Bridge Register Translation - Control
    pub e_breg_control: Aliased<u32, EBregControlR::Register, EBregControlW::Register>,
    _padding524: [u8; 4],
    /// Egress Bridge Register Translation - Source Address Low
    pub e_breg_base_lo: Aliased<u32, EBregBaseLoR::Register, EBregBaseLoW::Register>,
    /// Egress Bridge Register Translation - Source Address High
    pub e_breg_base_hi: ReadWrite<u32>,
    _padding536: [u8; 8],
    /// Egress ECAM Translation - Capabilities
    pub e_ecam_capabilities: ReadOnly<u32, EEcamCapabilities::Register>,
    /// Egress ECAM Translation - Status
    pub e_ecam_status: ReadOnly<u32, EEcamStatus::Register>,
    /// Egress ECAM Translation - Control
    pub e_ecam_control: Aliased<u32, EEcamControlR::Register, EEcamControlW::Register>,
    _padding556: [u8; 4],
    /// Egress ECAM Translation - Source Address Low
    pub e_ecam_base_lo: Aliased<u32, EEcamBaseLoR::Register, EEcamBaseLoW::Register>,
    /// Egress ECAM Translation - Source Address High
    pub e_ecam_base_hi: ReadWrite<u32>,
    _padding568: [u8; 8],
    /// Egress MSI-X Table Translation - Capabilities
    pub e_msxt_capabilities: ReadOnly<u32, EMsxtCapabilities::Register>,
    /// Egress MSI-X Table Translation - Status
    pub e_msxt_status: ReadOnly<u32, EMsxtStatus::Register>,
    /// Egress MSI-X Table Translation - Control
    pub e_msxt_control: Aliased<u32, EMsxtControlR::Register, EMsxtControlW::Register>,
    _padding588: [u8; 4],
    /// Egress MSI-X Table Translation - Source Address Low
    pub e_msxt_base_lo: Aliased<u32, EMsxtBaseLoR::Register, EMsxtBaseLoW::Register>,
    /// Egress MSI-X Table Translation - Source Address High
    pub e_msxt_base_hi: ReadWrite<u32>,
    _padding600: [u8; 8],
    /// Egress MSI-X PBA Translation - Capabilities
    pub e_msxp_capabilities: ReadOnly<u32, EMsxpCapabilities::Register>,
    /// Egress MSI-X PBA Translation - Status
    pub e_msxp_status: ReadOnly<u32, EMsxpStatus::Register>,
    /// Egress MSI-X PBA Translation - Control
    pub e_msxp_control: Aliased<u32, EMsxpControlR::Register, EMsxpControlW::Register>,
    _padding620: [u8; 4],
    /// Egress MSI-X PBA Translation - Source Address Low
    pub e_msxp_base_lo: Aliased<u32, EMsxpBaseLoR::Register, EMsxpBaseLoW::Register>,
    /// Egress MSI-X PBA Translation - Source Address High
    pub e_msxp_base_hi: ReadWrite<u32>,
    _padding632: [u8; 8],
    /// Egress DMA Register Translation - Capabilities
    pub e_dreg_capabilities: ReadOnly<u32, EDregCapabilities::Register>,
    /// Egress DMA Register Translation - Status
    pub e_dreg_status: ReadOnly<u32, EDregStatus::Register>,
    /// Egress DMA Register Translation - Control
    pub e_dreg_control: Aliased<u32, EDregControlR::Register, EDregControlW::Register>,
    _padding652: [u8; 4],
    /// Egress DMA Register Translation - Source Address Low
    pub e_dreg_base_lo: Aliased<u32, EDregBaseLoR::Register, EDregBaseLoW::Register>,
    /// Egress DMA Register Translation - Source Address High
    pub e_dreg_base_hi: ReadWrite<u32>,
    _padding664: [u8; 72],
    /// Egress Subtractive Decode Translation - Capabilities
    pub e_esub_capabilities: ReadOnly<u32, EEsubCapabilities::Register>,
    /// Egress Subtractive Decode Translation - Status
    pub e_esub_status: ReadOnly<u32, EEsubStatus::Register>,
    /// Egress Subtractive Decode Translation - Control
    pub e_esub_control: Aliased<u32, EEsubControlR::Register, EEsubControlW::Register>,
    _padding748: [u8; 20],
    /// Ingress PCI Express Received MSI Interrupt Translation - Capabilities
    pub i_msii_capabilities: ReadOnly<u32, IMsiiCapabilities::Register>,
    _padding772: [u8; 4],
    /// Ingress PCI Express Received MSI Interrupt Translation - Control
    pub i_msii_control: Aliased<u32, IMsiiControlR::Register, IMsiiControlW::Register>,
    _padding780: [u8; 4],
    /// Ingress PCI Express Received MSI Interrupt Translation - Source Address Low
    pub i_msii_base_lo: Aliased<u32, IMsiiBaseLoR::Register, IMsiiBaseLoW::Register>,
    /// Ingress PCI Express Received MSI Interrupt Translation - Source Address High
    pub i_msii_base_hi: ReadWrite<u32>,
    _padding792: [u8; 8],
    /// Ingress PCI Express Received MSI-X Interrupt Translation - Capabilities
    pub i_msix_capabilities: ReadOnly<u32, IMsixCapabilities::Register>,
    _padding804: [u8; 4],
    /// Ingress PCI Express Received MSI-X Interrupt Translation - Control
    pub i_msix_control: Aliased<u32, IMsixControlR::Register, IMsixControlW::Register>,
    _padding812: [u8; 4],
    /// Ingress PCI Express Received MSI-X Interrupt Translation - Source Address Low
    pub i_msix_base_lo: Aliased<u32, IMsixBaseLoR::Register, IMsixBaseLoW::Register>,
    /// Ingress PCI Express Received MSI-X Interrupt Translation - Source Address High
    pub i_msix_base_hi: ReadWrite<u32>,
    _padding824: [u8; 168],
    /// Ingress Subtractive Decode Translation - Capabilities
    pub i_isub_capabilities: ReadOnly<u32, IIsubCapabilities::Register>,
    /// Ingress Subtractive Decode Translation - Status
    pub i_isub_status: ReadOnly<u32, IIsubStatus::Register>,
    /// Ingress Subtractive Decode Translation - Control
    pub i_isub_control: Aliased<u32, IIsubControlR::Register, IIsubControlW::Register>,
    _padding1004: [u8; 20],
    /// Received Interrupt and Message Controller - Miscellaneous Interrupt Status.
    pub msgf_misc_status: Aliased<u32, MsgfMiscStatusR::Register, MsgfMiscStatusW::Register>,
    /// Received Interrupt and Message Controller - Miscellaneous Interrupt Status.
    pub msgf_misc_mask: Aliased<u32, MsgfMiscMaskR::Register, MsgfMiscMaskW::Register>,
    /// Slave Error AXI ID.
    pub msgf_misc_slave_id: ReadOnly<u32, MsgfMiscSlaveId::Register>,
    /// Master Error AXI ID.
    pub msgf_misc_master_id: ReadOnly<u32, MsgfMiscMasterId::Register>,
    /// Ingress Error AXI ID.
    pub msgf_misc_ingress_id: ReadOnly<u32, MsgfMiscIngressId::Register>,
    /// Egress Error AXI ID.
    pub msgf_misc_egress_id: ReadOnly<u32, MsgfMiscEgressId::Register>,
    _padding1048: [u8; 8],
    /// Legacy Interrupt Status.
    pub msgf_leg_status: ReadOnly<u32, MsgfLegStatus::Register>,
    /// Legacy Interrupt Mask.
    pub msgf_leg_mask: Aliased<u32, MsgfLegMaskR::Register, MsgfLegMaskW::Register>,
    _padding1064: [u8; 24],
    /// MSI Interrupt Status[31:0].
    pub msgf_msi_status_lo: ReadWrite<u32>,
    /// MSI Interrupt Status[63:32].
    pub msgf_msi_status_hi: ReadWrite<u32>,
    /// MSI Interrupt Mask[31:0].
    pub msgf_msi_mask_lo: ReadWrite<u32>,
    /// MSI Interrupt Mask[63:32].
    pub msgf_msi_mask_hi: ReadWrite<u32>,
    _padding1104: [u8; 16],
    /// DMA Interrupt Status.
    pub msgf_dma_status: ReadOnly<u32, MsgfDmaStatus::Register>,
    /// DMA Interrupt Mask.
    pub msgf_dma_mask: Aliased<u32, MsgfDmaMaskR::Register, MsgfDmaMaskW::Register>,
    _padding1128: [u8; 24],
    /// Received Interrupt and Message FIFO - Level
    pub msgf_rx_fifo_level: ReadOnly<u32, MsgfRxFifoLevel::Register>,
    /// Received Interrupt and Message FIFO - Pop Element
    pub msgf_rx_fifo_pop: ReadOnly<u32, MsgfRxFifoPop::Register>,
    /// Received Interrupt and Message FIFO - Message/Interrupt Type
    pub msgf_rx_fifo_type: Aliased<u32, MsgfRxFifoTypeR::Register, MsgfRxFifoTypeW::Register>,
    /// Received Message Header
    pub msgf_rx_fifo_msg: ReadOnly<u32, MsgfRxFifoMsg::Register>,
    /// Received Message/Interrupt Address[31:0].
    pub msgf_rx_fifo_address_lo: ReadOnly<u32>,
    /// Received Message/Interrupt Address[63:32].
    pub msgf_rx_fifo_address_hi: ReadOnly<u32>,
    /// Received Message/Interrupt Data Payload[31:0].
    pub msgf_rx_fifo_data: ReadOnly<u32>,
    _padding1180: [u8; 388],
    /// PCIe Message Request Execution.
    pub tx_pcie_msg_execute: ReadOnly<u32, TxPcieMsgExecute::Register>,
    /// PCIe Message Request Execution - Control
    pub tx_pcie_msg_control: Aliased<u32, TxPcieMsgControlR::Register, TxPcieMsgControlW::Register>,
    /// PCIe Message Request Execution - Message Specific[31:0].
    pub tx_pcie_msg_specific_lo: ReadWrite<u32, TxPcieMsgSpecificLo::Register>,
    /// PCIe Message Request Execution - Message Specific[63:32].
    pub tx_pcie_msg_specific_hi: ReadWrite<u32, TxPcieMsgSpecificHi::Register>,
    /// PCIe Message Request Execution - Message Data Payload[31:0].
    pub tx_pcie_msg_data: ReadWrite<u32>,
}
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgPcieRx0R [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        CFG_DISABLE_PCIE_DMA_REG_ACCESS OFFSET(17) NUMBITS(1) [],
        CFG_DISABLE_PCIE_BRIDGE_REG_ACCESS OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(13) [],
        CFG_DMA_REG_BAR OFFSET(0) NUMBITS(3) [],
    ],
    pub BridgeCoreCfgPcieRx0W [
        CFG_DISABLE_PCIE_DMA_REG_ACCESS OFFSET(17) NUMBITS(1) [],
        CFG_DISABLE_PCIE_BRIDGE_REG_ACCESS OFFSET(16) NUMBITS(1) [],
        CFG_DMA_REG_BAR OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgPcieRx1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        CFG_RD_UR_IS_UR_OK1S_N OFFSET(8) NUMBITS(1) [],
        CFG_PCIE_RX_ARCACHE OFFSET(4) NUMBITS(4) [],
        CFG_PCIE_RX_AWCACHE OFFSET(0) NUMBITS(4) [],
    ],
    pub BridgeCoreCfgPcieRx1W [
        CFG_RD_UR_IS_UR_OK1S_N OFFSET(8) NUMBITS(1) [],
        CFG_PCIE_RX_ARCACHE OFFSET(4) NUMBITS(4) [],
        CFG_PCIE_RX_AWCACHE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgAxiMasterR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        CFG_M_MAX_RD_REQ_SIZE OFFSET(4) NUMBITS(3) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        CFG_M_MAX_WR_REQ_SIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub BridgeCoreCfgAxiMasterW [
        CFG_M_MAX_RD_REQ_SIZE OFFSET(4) NUMBITS(3) [],
        CFG_M_MAX_WR_REQ_SIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgPcieTxR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        CFG_PCIE_TX_CUT_THROUGH OFFSET(0) NUMBITS(1) [],
    ],
    pub BridgeCoreCfgPcieTxW [
        CFG_PCIE_TX_CUT_THROUGH OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgInterruptR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        CFG_PCIE_INT_AXI_PCIE_N OFFSET(0) NUMBITS(1) [],
    ],
    pub BridgeCoreCfgInterruptW [
        CFG_PCIE_INT_AXI_PCIE_N OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgRamDisable0R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        CFG_RAM_DMA_SGL_DST_DIS_COR OFFSET(14) NUMBITS(1) [],
        CFG_RAM_DMA_SGL_SRC_DIS_COR OFFSET(13) NUMBITS(1) [],
        CFG_RAM_DMA_CH_REG_DIS_COR OFFSET(12) NUMBITS(1) [],
        CFG_RAM_DMA_MSIX_TAB_DIS_COR OFFSET(11) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(2) [],
        CFG_RAM_DMA_AXI_S_W_DIS_COR OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(7) NUMBITS(1) [],
        CFG_RAM_DMA_AXI_M_R_DIS_COR OFFSET(6) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_CD_DIS_COR OFFSET(5) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_RA_DIS_COR OFFSET(4) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_W_DIS_COR OFFSET(3) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_WA_DIS_COR OFFSET(2) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_TX_W_DIS_COR OFFSET(1) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_M_R_DIS_COR OFFSET(0) NUMBITS(1) [],
    ],
    pub BridgeCoreCfgRamDisable0W [
        CFG_RAM_DMA_SGL_DST_DIS_COR OFFSET(14) NUMBITS(1) [],
        CFG_RAM_DMA_SGL_SRC_DIS_COR OFFSET(13) NUMBITS(1) [],
        CFG_RAM_DMA_CH_REG_DIS_COR OFFSET(12) NUMBITS(1) [],
        CFG_RAM_DMA_MSIX_TAB_DIS_COR OFFSET(11) NUMBITS(1) [],
        CFG_RAM_DMA_AXI_S_W_DIS_COR OFFSET(8) NUMBITS(1) [],
        CFG_RAM_DMA_AXI_M_R_DIS_COR OFFSET(6) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_CD_DIS_COR OFFSET(5) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_RA_DIS_COR OFFSET(4) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_W_DIS_COR OFFSET(3) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_WA_DIS_COR OFFSET(2) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_TX_W_DIS_COR OFFSET(1) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_M_R_DIS_COR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgRamDisable1R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        CFG_RAM_DMA_SGL_DST_DIS_ERR OFFSET(14) NUMBITS(1) [],
        CFG_RAM_DMA_SGL_SRC_DIS_ERR OFFSET(13) NUMBITS(1) [],
        CFG_RAM_DMA_CH_REG_DIS_ERR OFFSET(12) NUMBITS(1) [],
        CFG_RAM_DMA_MSIX_TAB_DIS_ERR OFFSET(11) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(2) [],
        CFG_RAM_DMA_AXI_S_W_DIS_ERR OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(7) NUMBITS(1) [],
        CFG_RAM_DMA_AXI_M_R_DIS_ERR OFFSET(6) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_CD_DIS_ERR OFFSET(5) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_RA_DIS_ERR OFFSET(4) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_W_DIS_ERR OFFSET(3) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_WA_DIS_ERR OFFSET(2) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_TX_W_DIS_ERR OFFSET(1) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_M_R_DIS_ERR OFFSET(0) NUMBITS(1) [],
    ],
    pub BridgeCoreCfgRamDisable1W [
        CFG_RAM_DMA_SGL_DST_DIS_ERR OFFSET(14) NUMBITS(1) [],
        CFG_RAM_DMA_SGL_SRC_DIS_ERR OFFSET(13) NUMBITS(1) [],
        CFG_RAM_DMA_CH_REG_DIS_ERR OFFSET(12) NUMBITS(1) [],
        CFG_RAM_DMA_MSIX_TAB_DIS_ERR OFFSET(11) NUMBITS(1) [],
        CFG_RAM_DMA_AXI_S_W_DIS_ERR OFFSET(8) NUMBITS(1) [],
        CFG_RAM_DMA_AXI_M_R_DIS_ERR OFFSET(6) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_CD_DIS_ERR OFFSET(5) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_RA_DIS_ERR OFFSET(4) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_W_DIS_ERR OFFSET(3) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_S_WA_DIS_ERR OFFSET(2) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_TX_W_DIS_ERR OFFSET(1) NUMBITS(1) [],
        CFG_RAM_DMA_PCIE_M_R_DIS_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgPcieRelaxedOrderR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        CFG_ENABLE_CFGIO_WR_RO OFFSET(1) NUMBITS(1) [],
        CFG_ENABLE_DMA_RO OFFSET(0) NUMBITS(1) [],
    ],
    pub BridgeCoreCfgPcieRelaxedOrderW [
        CFG_ENABLE_CFGIO_WR_RO OFFSET(1) NUMBITS(1) [],
        CFG_ENABLE_DMA_RO OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgPcieRxMsgFilterR [
        CFG_DESIRED_VEN_MSG_VEN_ID OFFSET(16) NUMBITS(16) [],
        CFG_DESIRED_VEN_MSG_VEN_INV OFFSET(15) NUMBITS(1) [],
        CFG_DESIRED_VEN_MSG_EN OFFSET(14) NUMBITS(1) [],
        CFG_ENABLE_OTH_MSG_FWD OFFSET(13) NUMBITS(1) [],
        RESERVED0 OFFSET(8) NUMBITS(5) [],
        CFG_ENABLE_VEN_MSG_FWD OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        CFG_ENABLE_SLT_MSG_FWD OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        CFG_ENABLE_ERR_MSG_FWD OFFSET(3) NUMBITS(1) [],
        CFG_ENABLE_INT_MSG_FWD OFFSET(2) NUMBITS(1) [],
        CFG_ENABLE_PM_MSG_FWD OFFSET(1) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub BridgeCoreCfgPcieRxMsgFilterW [
        CFG_DESIRED_VEN_MSG_VEN_ID OFFSET(16) NUMBITS(16) [],
        CFG_DESIRED_VEN_MSG_VEN_INV OFFSET(15) NUMBITS(1) [],
        CFG_DESIRED_VEN_MSG_EN OFFSET(14) NUMBITS(1) [],
        CFG_ENABLE_OTH_MSG_FWD OFFSET(13) NUMBITS(1) [],
        CFG_ENABLE_VEN_MSG_FWD OFFSET(7) NUMBITS(1) [],
        CFG_ENABLE_SLT_MSG_FWD OFFSET(5) NUMBITS(1) [],
        CFG_ENABLE_ERR_MSG_FWD OFFSET(3) NUMBITS(1) [],
        CFG_ENABLE_INT_MSG_FWD OFFSET(2) NUMBITS(1) [],
        CFG_ENABLE_PM_MSG_FWD OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgRqReqOrderR [
        CFG_PCIE_REQ_ORDER_STRICT OFFSET(31) NUMBITS(1) [],
        CFG_AXI_REQ_ORDER_STRICT OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(14) [],
        CFG_AXI_REQ_ORDER_ID_MASK OFFSET(0) NUMBITS(16) [],
    ],
    pub BridgeCoreCfgRqReqOrderW [
        CFG_PCIE_REQ_ORDER_STRICT OFFSET(31) NUMBITS(1) [],
        CFG_AXI_REQ_ORDER_STRICT OFFSET(30) NUMBITS(1) [],
        CFG_AXI_REQ_ORDER_ID_MASK OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgPcieCreditR [
        CFG_PCIE_CREDIT_EN OFFSET(31) NUMBITS(1) [],
        CFG_PCIE_CREDIT_CH_INF OFFSET(30) NUMBITS(1) [],
        CFG_PCIE_CREDIT_CD_INF OFFSET(29) NUMBITS(1) [],
        RESERVED0 OFFSET(24) NUMBITS(5) [],
        CFG_PCIE_CREDIT_CH_VAL OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        CFG_PCIE_CREDIT_CD_VAL OFFSET(0) NUMBITS(12) [],
    ],
    pub BridgeCoreCfgPcieCreditW [
        CFG_PCIE_CREDIT_EN OFFSET(31) NUMBITS(1) [],
        CFG_PCIE_CREDIT_CH_INF OFFSET(30) NUMBITS(1) [],
        CFG_PCIE_CREDIT_CD_INF OFFSET(29) NUMBITS(1) [],
        CFG_PCIE_CREDIT_CH_VAL OFFSET(16) NUMBITS(8) [],
        CFG_PCIE_CREDIT_CD_VAL OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgAxiMWTickCountR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        AXI_M_W_TICK_COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub BridgeCoreCfgAxiMWTickCountW [
        AXI_M_W_TICK_COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgAxiMRTickCountR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        AXI_M_R_TICK_COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub BridgeCoreCfgAxiMRTickCountW [
        AXI_M_R_TICK_COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BridgeCoreCfgCrsRplTickCountR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CRS_RPL_TICK_COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub BridgeCoreCfgCrsRplTickCountW [
        CRS_RPL_TICK_COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EBregCapabilities [
        BREG_SIZE_MAX OFFSET(24) NUMBITS(8) [],
        BREG_SIZE_OFFSET OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        BREG_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EBregStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        WR_PENDING_CTR OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        RD_PENDING_CTR OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EBregControlR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        BREG_SIZE OFFSET(16) NUMBITS(2) [],
        RESERVED1 OFFSET(3) NUMBITS(13) [],
        BREG_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        BREG_ENABLE_FORCE OFFSET(1) NUMBITS(1) [],
        BREG_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub EBregControlW [
        BREG_SIZE OFFSET(16) NUMBITS(2) [],
        BREG_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        BREG_ENABLE_FORCE OFFSET(1) NUMBITS(1) [],
        BREG_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EBregBaseLoR [
        BREG_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub EBregBaseLoW [
        BREG_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EEcamCapabilities [
        ECAM_SIZE_MAX OFFSET(24) NUMBITS(8) [],
        ECAM_SIZE_OFFSET OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        ECAM_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EEcamStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        WR_PENDING_CTR OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        RD_PENDING_CTR OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EEcamControlR [
        RESERVED0 OFFSET(21) NUMBITS(11) [],
        ECAM_SIZE OFFSET(16) NUMBITS(5) [],
        RESERVED1 OFFSET(3) NUMBITS(13) [],
        ECAM_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        ECAM_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub EEcamControlW [
        ECAM_SIZE OFFSET(16) NUMBITS(5) [],
        ECAM_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        ECAM_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EEcamBaseLoR [
        ECAM_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub EEcamBaseLoW [
        ECAM_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EMsxtCapabilities [
        MSXT_SIZE_MAX OFFSET(24) NUMBITS(8) [],
        MSXT_SIZE_OFFSET OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        MSXT_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EMsxtStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        WR_PENDING_CTR OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        RD_PENDING_CTR OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EMsxtControlR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        MSXT_SIZE OFFSET(16) NUMBITS(2) [],
        RESERVED1 OFFSET(3) NUMBITS(13) [],
        MSXT_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        MSXT_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub EMsxtControlW [
        MSXT_SIZE OFFSET(16) NUMBITS(2) [],
        MSXT_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        MSXT_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EMsxtBaseLoR [
        MSXT_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub EMsxtBaseLoW [
        MSXT_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EMsxpCapabilities [
        MSXP_SIZE_MAX OFFSET(24) NUMBITS(8) [],
        MSXP_SIZE_OFFSET OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        MSXP_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EMsxpStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        WR_PENDING_CTR OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        RD_PENDING_CTR OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EMsxpControlR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        MSXP_SIZE OFFSET(16) NUMBITS(2) [],
        RESERVED1 OFFSET(3) NUMBITS(13) [],
        MSXP_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        MSXP_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub EMsxpControlW [
        MSXP_SIZE OFFSET(16) NUMBITS(2) [],
        MSXP_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        MSXP_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EMsxpBaseLoR [
        MSXP_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub EMsxpBaseLoW [
        MSXP_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EDregCapabilities [
        DMA_SIZE_MAX OFFSET(24) NUMBITS(8) [],
        DMA_SIZE_OFFSET OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        DMA_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EDregStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        WR_PENDING_CTR OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        RD_PENDING_CTR OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EDregControlR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        DMA_SIZE OFFSET(16) NUMBITS(2) [],
        RESERVED1 OFFSET(3) NUMBITS(13) [],
        DMA_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        DMA_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub EDregControlW [
        DMA_SIZE OFFSET(16) NUMBITS(2) [],
        DMA_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        DMA_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EDregBaseLoR [
        DMA_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub EDregBaseLoW [
        DMA_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EEsubCapabilities [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SUBTRACTIVE_DECODE_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EEsubStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        WR_PENDING_CTR OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        RD_PENDING_CTR OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EEsubControlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        EGRESS_SUB_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub EEsubControlW [
        EGRESS_SUB_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IMsiiCapabilities [
        I_MSII_SIZE_MAX OFFSET(24) NUMBITS(8) [],
        I_MSII_SIZE_OFFSET OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        I_MSII_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IMsiiControlR [
        RESERVED0 OFFSET(21) NUMBITS(11) [],
        I_MSII_SIZE OFFSET(16) NUMBITS(5) [],
        I_MSII_STATUS_ENABLE OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(14) [],
        I_MSII_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub IMsiiControlW [
        I_MSII_SIZE OFFSET(16) NUMBITS(5) [],
        I_MSII_STATUS_ENABLE OFFSET(15) NUMBITS(1) [],
        I_MSII_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IMsiiBaseLoR [
        I_MSII_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub IMsiiBaseLoW [
        I_MSII_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IMsixCapabilities [
        I_MSIX_SIZE_MAX OFFSET(24) NUMBITS(8) [],
        I_MSIX_SIZE_OFFSET OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        I_MSIX_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IMsixControlR [
        RESERVED0 OFFSET(21) NUMBITS(11) [],
        I_MSIX_SIZE OFFSET(16) NUMBITS(5) [],
        RESERVED1 OFFSET(1) NUMBITS(15) [],
        I_MSIX_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub IMsixControlW [
        I_MSIX_SIZE OFFSET(16) NUMBITS(5) [],
        I_MSIX_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IMsixBaseLoR [
        I_MSIX_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub IMsixBaseLoW [
        I_MSIX_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IIsubCapabilities [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SUBTRACTIVE_DECODE_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IIsubStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        WR_PENDING_CTR OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        RD_PENDING_CTR OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IIsubControlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        INGRESS_SUB_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub IIsubControlW [
        INGRESS_SUB_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfMiscStatusR [
        PCIE_CORE_EVENT OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        EGRESS_ADDRESS_TRANSLATION_ERROR OFFSET(7) NUMBITS(1) [],
        INGRESS_ADDRESS_TRANSLATION_ERROR OFFSET(6) NUMBITS(1) [],
        MASTER_ERROR OFFSET(5) NUMBITS(1) [],
        SLAVE_ERROR OFFSET(4) NUMBITS(1) [],
        UNCORRECTABLE_WRITE_ERROR OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        RX_MSG_OVERFLOW OFFSET(1) NUMBITS(1) [],
        RX_MSG_AVAIL OFFSET(0) NUMBITS(1) [],
    ],
    pub MsgfMiscStatusW [
        PCIE_CORE_EVENT OFFSET(16) NUMBITS(16) [],
        EGRESS_ADDRESS_TRANSLATION_ERROR OFFSET(7) NUMBITS(1) [],
        INGRESS_ADDRESS_TRANSLATION_ERROR OFFSET(6) NUMBITS(1) [],
        MASTER_ERROR OFFSET(5) NUMBITS(1) [],
        SLAVE_ERROR OFFSET(4) NUMBITS(1) [],
        UNCORRECTABLE_WRITE_ERROR OFFSET(3) NUMBITS(1) [],
        RX_MSG_OVERFLOW OFFSET(1) NUMBITS(1) [],
        RX_MSG_AVAIL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfMiscMaskR [
        PCIE_CORE_EVENT_MASK OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        EGRESS_ADDRESS_TRANSLATION_ERROR_MASK OFFSET(7) NUMBITS(1) [],
        INGRESS_ADDRESS_TRANSLATION_ERROR_MASK OFFSET(6) NUMBITS(1) [],
        MASTER_ERROR_MASK OFFSET(5) NUMBITS(1) [],
        SLAVE_ERROR_MASK OFFSET(4) NUMBITS(1) [],
        UNCORRECTABLE_WRITE_ERROR_MASK OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        RX_MSG_OVERFLOW_MASK OFFSET(1) NUMBITS(1) [],
        RX_MSG_AVAIL_MASK OFFSET(0) NUMBITS(1) [],
    ],
    pub MsgfMiscMaskW [
        PCIE_CORE_EVENT_MASK OFFSET(16) NUMBITS(16) [],
        EGRESS_ADDRESS_TRANSLATION_ERROR_MASK OFFSET(7) NUMBITS(1) [],
        INGRESS_ADDRESS_TRANSLATION_ERROR_MASK OFFSET(6) NUMBITS(1) [],
        MASTER_ERROR_MASK OFFSET(5) NUMBITS(1) [],
        SLAVE_ERROR_MASK OFFSET(4) NUMBITS(1) [],
        UNCORRECTABLE_WRITE_ERROR_MASK OFFSET(3) NUMBITS(1) [],
        RX_MSG_OVERFLOW_MASK OFFSET(1) NUMBITS(1) [],
        RX_MSG_AVAIL_MASK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfMiscSlaveId [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        SLAVE_ERROR_ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfMiscMasterId [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        MASTER_ERROR_ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfMiscIngressId [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        INGRESS_ERROR_ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfMiscEgressId [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        EGRESS_ERROR_ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfLegStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        MSGF_LEG_STATUS_INTD OFFSET(3) NUMBITS(1) [],
        MSGF_LEG_STATUS_INTC OFFSET(2) NUMBITS(1) [],
        MSGF_LEG_STATUS_INTB OFFSET(1) NUMBITS(1) [],
        MSGF_LEG_STATUS_INTA OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfLegMaskR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        MSGF_LEG_MASK_INTD OFFSET(3) NUMBITS(1) [],
        MSGF_LEG_MASK_INTC OFFSET(2) NUMBITS(1) [],
        MSGF_LEG_MASK_INTB OFFSET(1) NUMBITS(1) [],
        MSGF_LEG_MASK_INTA OFFSET(0) NUMBITS(1) [],
    ],
    pub MsgfLegMaskW [
        MSGF_LEG_MASK_INTD OFFSET(3) NUMBITS(1) [],
        MSGF_LEG_MASK_INTC OFFSET(2) NUMBITS(1) [],
        MSGF_LEG_MASK_INTB OFFSET(1) NUMBITS(1) [],
        MSGF_LEG_MASK_INTA OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfDmaStatus [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        MSGF_DMA_STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfDmaMaskR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        MSGF_DMA_MASK OFFSET(0) NUMBITS(1) [],
    ],
    pub MsgfDmaMaskW [
        MSGF_DMA_MASK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfRxFifoLevel [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        LEVEL OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfRxFifoPop [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        POP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfRxFifoTypeR [
        REQUESTER_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(4) NUMBITS(12) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        RECEIVED_MSI_X_INTERRUPT OFFSET(2) NUMBITS(1) [],
        RECEIVED_MSI_INTERRUPT OFFSET(1) NUMBITS(1) [],
        RECEIVED_MESSAGE OFFSET(0) NUMBITS(1) [],
    ],
    pub MsgfRxFifoTypeW [
        RECEIVED_MSI_X_INTERRUPT OFFSET(2) NUMBITS(1) [],
        RECEIVED_MSI_INTERRUPT OFFSET(1) NUMBITS(1) [],
        RECEIVED_MESSAGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MsgfRxFifoMsg [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        MESSAGE_TAG OFFSET(16) NUMBITS(8) [],
        MESSAGE_CODE OFFSET(8) NUMBITS(8) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        MESSAGE_PAYLOAD_PRESENT OFFSET(3) NUMBITS(1) [],
        MESSAGE_ROUTING OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxPcieMsgExecute [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MSG_DONE_STATUS OFFSET(24) NUMBITS(2) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        MSG_DONE OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        MSG_BUSY OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        MSG_EXECUTE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxPcieMsgControlR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        MSG_HAS_DATA OFFSET(24) NUMBITS(1) [],
        MSG_TAG OFFSET(16) NUMBITS(8) [],
        MSG_CODE OFFSET(8) NUMBITS(8) [],
        MSG_FMT_TYPE OFFSET(0) NUMBITS(8) [],
    ],
    pub TxPcieMsgControlW [
        MSG_HAS_DATA OFFSET(24) NUMBITS(1) [],
        MSG_TAG OFFSET(16) NUMBITS(8) [],
        MSG_CODE OFFSET(8) NUMBITS(8) [],
        MSG_FMT_TYPE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxPcieMsgSpecificLo [
        MSG_TLP_HDR11 OFFSET(24) NUMBITS(8) [],
        MSG_TLP_HDR10 OFFSET(16) NUMBITS(8) [],
        MSG_TLP_HDR9 OFFSET(8) NUMBITS(8) [],
        MSG_TLP_HDR8 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxPcieMsgSpecificHi [
        MSG_TLP_HDR15 OFFSET(24) NUMBITS(8) [],
        MSG_TLP_HDR14 OFFSET(16) NUMBITS(8) [],
        MSG_TLP_HDR13 OFFSET(8) NUMBITS(8) [],
        MSG_TLP_HDR12 OFFSET(0) NUMBITS(8) [],
    ]
];

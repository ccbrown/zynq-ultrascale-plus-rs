// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// Gigabit Ethernet Controller, Gigabit Ethernet; GEM 0 Controller
pub static mut GEM0: *mut Registers = 0xff0b0000 as *mut Registers;
/// Gigabit Ethernet Controller, Gigabit Ethernet; GEM 1 Controller
pub static mut GEM1: *mut Registers = 0xff0c0000 as *mut Registers;
/// Gigabit Ethernet Controller, Gigabit Ethernet; GEM 2 Controller
pub static mut GEM2: *mut Registers = 0xff0d0000 as *mut Registers;
/// Gigabit Ethernet Controller, Gigabit Ethernet; GEM 3 Controller
pub static mut GEM3: *mut Registers = 0xff0e0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// The network control register contains general MAC control functions for both receiver and transmitter.
    pub network_control: Aliased<u32, NetworkControlR::Register, NetworkControlW::Register>,
    /// The network configuration register contains functions for setting the mode of operation for the Gigabit Ethernet MAC.
    pub network_config: ReadWrite<u32, NetworkConfig::Register>,
    /// The network status register returns status information with respect to the PHY management interface.
    pub network_status: ReadOnly<u32, NetworkStatus::Register>,
    _padding12: [u8; 4],
    /// DMA Configuration Register
    pub dma_config: Aliased<u32, DmaConfigR::Register, DmaConfigW::Register>,
    /// This register, when read, provides details of the status of a transmit. Once read, individual bits may be cleared by writing 1 to them. It is not possible to set a bit to 1 by writing to the register.
    pub transmit_status: Aliased<u32, TransmitStatusR::Register, TransmitStatusW::Register>,
    /// Start address of the receive buffer queue (receive buffers descriptor list). The receive buffer queue base address must be initialized before receive is enabled through bit 2 of the network control register. Once reception is enabled, any write to the receive buffer queue base address register is ignored. Reading this register returns the location of the descriptor currently being accessed. This value increments as buffers are used. Software should not use this register for determining where to remove received frames from the queue as it constantly changes as new frames are received. Software should instead work its way through the buffer descriptor queue checking the used bits.In terms of system bus operation, the receive descriptors must be aligned at 64-bit boundaries for each pair of 32-bit descriptors.
    pub receive_q_ptr: Aliased<u32, ReceiveQPtrR::Register, ReceiveQPtrW::Register>,
    /// Start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Since the DMA handles two frames at once, this may not necessarily be pointing to the current frame being transmitted.In terms of system bus operation, the transmit descriptors must be aligned at 64-bit boundaries for each pair of 32-bit descriptors.
    pub transmit_q_ptr: Aliased<u32, TransmitQPtrR::Register, TransmitQPtrW::Register>,
    /// This register, when read provides details of the status of a receive. Once read, individual bits may be cleared by writing 1 to them. It is not possible to set a bit to 1 by writing to the register.
    pub receive_status: Aliased<u32, ReceiveStatusR::Register, ReceiveStatusW::Register>,
    /// Interrupt Status - non-priority queing.
    pub int_status: Aliased<u32, IntStatusR::Register, IntStatusW::Register>,
    /// At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero.
    pub int_enable: Aliased<u32, IntEnableR::Register, IntEnableW::Register>,
    /// Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero.
    pub int_disable: Aliased<u32, IntDisableR::Register, IntDisableW::Register>,
    /// The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register.
    pub int_mask: ReadOnly<u32, IntMask::Register>,
    /// PHY maintenance register is implemented as a shift register. Writing to the register starts a shift operation which is signalled as complete when bit-2 is set in the network status register. It takes about 125 LPD_LSBUS_CLK clock cycles to complete, when MDC is set for LPD_LSBUS_CLK divide by 2 in the network configuration register. An interrupt is generated upon completion.During this time, the MSB of the register is output on the MDIO pin and the LSB updated from the MDIO pin with each MDC cycle. This causes transmission of a PHY management frame on MDIO. See Section 22.2.4.5 of the IEEE 802.3 standard. Reading during the shift operation will return the current contents of the shift register. At the end of management operation, the bits will have shifted back to their original locations. For a read operation, the data bits will be updated with data read from the PHY. It is important to write the correct values to the register to ensure a valid PHY management frame is produced. The MDIO interface can read IEEE 802.3 clause 45 PHYs as well as clause 22 PHYs. To read clause 45 PHYs, bit 30 should be written with a 0 rather than a 1. For a description of MDC generation, see Network Configuration Register.
    pub phy_management: ReadWrite<u32, PhyManagement::Register>,
    /// Received Pause Quantum Register
    pub pause_time: ReadOnly<u32, PauseTime::Register>,
    /// Transmit Pause Quantum Register
    pub tx_pause_quantum: Aliased<u32, TxPauseQuantumR::Register, TxPauseQuantumW::Register>,
    /// Partial store and forward is only applicable when using the the DMA configured in SRAM based packet buffer mode. It is also not available when using multi buffer frames. TX Partial Store and Forward
    pub pbuf_txcutthru: Aliased<u32, PbufTxcutthruR::Register, PbufTxcutthruW::Register>,
    /// RX Partial Store and Forward
    pub pbuf_rxcutthru: Aliased<u32, PbufRxcutthruR::Register, PbufRxcutthruW::Register>,
    /// Maximum Jumbo Frame Size.
    pub jumbo_max_length: Aliased<u32, JumboMaxLengthR::Register, JumboMaxLengthW::Register>,
    /// External FIFO Interface Enable (only valid when gem_host_if_soft_select is defined)
    pub external_fifo_interface:
        Aliased<u32, ExternalFifoInterfaceR::Register, ExternalFifoInterfaceW::Register>,
    _padding80: [u8; 4],
    /// Used to set the maximum amnount of outstanding transactions on the AXI bus between AR / R channels and AW / W channels. Cannot be more than the depth of the configured AXI pipeline FIFO (defined in verilog defs.v).Note:Xilinx recommends to use the default setting for this register.
    pub axi_max_pipeline: Aliased<u32, AxiMaxPipelineR::Register, AxiMaxPipelineW::Register>,
    _padding88: [u8; 40],
    /// The unicast hash enable and the multicast hash enable bits in the network configuration register enable the reception of hash matched framesHash Register Bottom [31:0]
    pub hash_bottom: ReadWrite<u32>,
    /// Hash Register Top 63:32
    pub hash_top: ReadWrite<u32>,
    /// The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written.
    pub spec_add1_bottom: ReadWrite<u32>,
    /// Specific Address Top
    pub spec_add1_top: Aliased<u32, SpecAdd1TopR::Register, SpecAdd1TopW::Register>,
    /// The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written.
    pub spec_add2_bottom: ReadWrite<u32>,
    /// Specific Address Top
    pub spec_add2_top: Aliased<u32, SpecAdd2TopR::Register, SpecAdd2TopW::Register>,
    /// The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written.
    pub spec_add3_bottom: ReadWrite<u32>,
    /// Specific Address Top
    pub spec_add3_top: Aliased<u32, SpecAdd3TopR::Register, SpecAdd3TopW::Register>,
    /// The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written.
    pub spec_add4_bottom: ReadWrite<u32>,
    /// Specific Address Top
    pub spec_add4_top: Aliased<u32, SpecAdd4TopR::Register, SpecAdd4TopW::Register>,
    /// Type ID Match 1
    pub spec_type1: Aliased<u32, SpecType1R::Register, SpecType1W::Register>,
    /// Type ID Match 2
    pub spec_type2: Aliased<u32, SpecType2R::Register, SpecType2W::Register>,
    /// Type ID Match 3
    pub spec_type3: Aliased<u32, SpecType3R::Register, SpecType3W::Register>,
    /// Type ID Match 4
    pub spec_type4: Aliased<u32, SpecType4R::Register, SpecType4W::Register>,
    /// Wake on LAN Register
    pub wol: Aliased<u32, WolR::Register, WolW::Register>,
    /// IPG stretch register
    pub stretch_ratio: Aliased<u32, StretchRatioR::Register, StretchRatioW::Register>,
    /// Stacked VLAN Register
    pub stacked_vlan: Aliased<u32, StackedVlanR::Register, StackedVlanW::Register>,
    /// Transmit PFC Pause Register
    pub tx_pfc_pause: Aliased<u32, TxPfcPauseR::Register, TxPfcPauseW::Register>,
    /// Specific Address Mask 1 Bottom 31:0
    pub mask_add1_bottom: ReadWrite<u32>,
    /// Specific Address Mask 1 Top 47:32
    pub mask_add1_top: Aliased<u32, MaskAdd1TopR::Register, MaskAdd1TopW::Register>,
    /// Receive DMA Data Buffer Address Mask
    pub dma_addr_or_mask: Aliased<u32, DmaAddrOrMaskR::Register, DmaAddrOrMaskW::Register>,
    /// PTP RX unicast IP destination address
    pub rx_ptp_unicast: ReadWrite<u32>,
    /// PTP TX unicast IP destination address
    pub tx_ptp_unicast: ReadWrite<u32>,
    /// TSU timer comparison value nanoseconds
    pub tsu_nsec_cmp: Aliased<u32, TsuNsecCmpR::Register, TsuNsecCmpW::Register>,
    /// TSU timer comparison value seconds 31:0
    pub tsu_sec_cmp: ReadWrite<u32>,
    /// TSU timer comparison value seconds 47:32
    pub tsu_msb_sec_cmp: Aliased<u32, TsuMsbSecCmpR::Register, TsuMsbSecCmpW::Register>,
    /// PTP Event Frame Transmitted Seconds Register 47:32
    pub tsu_ptp_tx_msb_sec: ReadOnly<u32, TsuPtpTxMsbSec::Register>,
    /// PTP Event Frame Received Seconds Register 47:32
    pub tsu_ptp_rx_msb_sec: ReadOnly<u32, TsuPtpRxMsbSec::Register>,
    /// PTP Peer Event Frame Transmitted Seconds Register 47:32
    pub tsu_peer_tx_msb_sec: ReadOnly<u32, TsuPeerTxMsbSec::Register>,
    /// PTP Peer Event Frame Received Seconds Register 47:32
    pub tsu_peer_rx_msb_sec: ReadOnly<u32, TsuPeerRxMsbSec::Register>,
    /// The fill levels for the TX & RX packet buffers can be read using this register, including the fill level for each queue in the TX direction.
    pub dpram_fill_dbg: Aliased<u32, DpramFillDbgR::Register, DpramFillDbgW::Register>,
    /// This register indicates amodule identification number and module revision. The value of this register is read only as defined by `gem_revision_reg_value
    pub revision_reg: ReadOnly<u32, RevisionReg::Register>,
    /// These registers reset to zero on a read and stick at all ones when they count to their maximum value. They should be read frequently enough to prevent loss of data. In order to reduce overall design area, the statistics registers may be optionally removed in the configuration file if they are deemed unnecsessary for a particular design. The receive statistics registers are only incremented when the receive enable bit is set in the network control register. The statistics registers optionally have a snapshot capability which, when exercised, will simultaneously store and clear the current values of all the statistics registers into a snapshot register set in order to allow a consistent set of statistics to be read by the processor. The snapshot is controlled using bit 13 of the network control register. The read snapshot control indicated by bit 14 of the network control register determines whether the processor reads the snapshot registers (logic 1) or the incrementing registers (logic 0). The default GEM configuration does not support the snapshot capability. See Parameterization section under Implementation Application Notes for an explanation of how to enable this function. All the statistics registers are read only. For test purposes they may be written by setting bit 7 (Write Enable) in the network control register. Setting bit 6 (increment statistics) in the network control register causes all the statistics registers to increment by one, again for test purposes. Once a statistics register has been read, it is automatically cleared. When reading the octets transmitted and octets received registers, bits 31:0 should be read prior to bits 47:32 to ensure reliable operation. The statistics register block contains the following registers. Octets Transmitted [31:0]
    pub octets_txed_bottom: ReadWrite<u32>,
    /// Octets Transmitted 47:32
    pub octets_txed_top: Aliased<u32, OctetsTxedTopR::Register, OctetsTxedTopW::Register>,
    /// Frames Transmitted
    pub frames_txed_ok: ReadWrite<u32>,
    /// Broadcast Frames Transmitted
    pub broadcast_txed: ReadWrite<u32>,
    /// Multicast Frames Transmitted
    pub multicast_txed: ReadWrite<u32>,
    /// Pause Frames Transmitted
    pub pause_frames_txed: Aliased<u32, PauseFramesTxedR::Register, PauseFramesTxedW::Register>,
    /// 64 Byte Frames Transmitted
    pub frames_txed_64: ReadWrite<u32>,
    /// 65 to 127 Byte Frames Transmitted
    pub frames_txed_65: ReadWrite<u32>,
    /// 128 to 255 Byte Frames Transmitted
    pub frames_txed_128: ReadWrite<u32>,
    /// 256 to 511 Byte Frames Transmitted
    pub frames_txed_256: ReadWrite<u32>,
    /// 512 to 1023 Byte Frames Transmitted
    pub frames_txed_512: ReadWrite<u32>,
    /// 1024 to 1518 Byte Frames Transmitted
    pub frames_txed_1024: ReadWrite<u32>,
    /// Greater Than 1518 Byte Frames Transmitted
    pub frames_txed_1519: ReadWrite<u32>,
    /// Transmit Under Runs
    pub tx_underruns: Aliased<u32, TxUnderrunsR::Register, TxUnderrunsW::Register>,
    /// Single Collision Frames
    pub single_collisions: Aliased<u32, SingleCollisionsR::Register, SingleCollisionsW::Register>,
    /// Multiple Collision Frames
    pub multiple_collisions:
        Aliased<u32, MultipleCollisionsR::Register, MultipleCollisionsW::Register>,
    /// Excessive Collisions
    pub excessive_collisions:
        Aliased<u32, ExcessiveCollisionsR::Register, ExcessiveCollisionsW::Register>,
    /// Late Collisions
    pub late_collisions: Aliased<u32, LateCollisionsR::Register, LateCollisionsW::Register>,
    /// Deferred Transmission Frames
    pub deferred_frames: Aliased<u32, DeferredFramesR::Register, DeferredFramesW::Register>,
    /// Carrier Sense Errors
    pub crs_errors: Aliased<u32, CrsErrorsR::Register, CrsErrorsW::Register>,
    /// Octets Received 31:0
    pub octets_rxed_bottom: ReadWrite<u32>,
    /// Octets Received 47:32
    pub octets_rxed_top: Aliased<u32, OctetsRxedTopR::Register, OctetsRxedTopW::Register>,
    /// Frames Received
    pub frames_rxed_ok: ReadWrite<u32>,
    /// Broadcast Frames Received
    pub broadcast_rxed: ReadWrite<u32>,
    /// Multicast Frames Received
    pub multicast_rxed: ReadWrite<u32>,
    /// Pause Frames Received
    pub pause_frames_rxed: Aliased<u32, PauseFramesRxedR::Register, PauseFramesRxedW::Register>,
    /// 64 Byte Frames Received
    pub frames_rxed_64: ReadWrite<u32>,
    /// 65 to 127 Byte Frames Received
    pub frames_rxed_65: ReadWrite<u32>,
    /// 128 to 255 Byte Frames Received
    pub frames_rxed_128: ReadWrite<u32>,
    /// 256 to 511 Byte Frames Received
    pub frames_rxed_256: ReadWrite<u32>,
    /// 512 to 1023 Byte Frames Received
    pub frames_rxed_512: ReadWrite<u32>,
    /// 1024 to 1518 Byte Frames Received
    pub frames_rxed_1024: ReadWrite<u32>,
    /// 1519 to maximum Byte Frames Received
    pub frames_rxed_1519: ReadWrite<u32>,
    /// Undersized Frames Received
    pub undersize_frames: Aliased<u32, UndersizeFramesR::Register, UndersizeFramesW::Register>,
    /// Oversize Frames Received
    pub excessive_rx_length:
        Aliased<u32, ExcessiveRxLengthR::Register, ExcessiveRxLengthW::Register>,
    /// Jabbers Received
    pub rx_jabbers: Aliased<u32, RxJabbersR::Register, RxJabbersW::Register>,
    /// Frame Check Sequence Errors
    pub fcs_errors: Aliased<u32, FcsErrorsR::Register, FcsErrorsW::Register>,
    /// Length Field Frame Errors
    pub rx_length_errors: Aliased<u32, RxLengthErrorsR::Register, RxLengthErrorsW::Register>,
    /// Receive Symbol Errors
    pub rx_symbol_errors: Aliased<u32, RxSymbolErrorsR::Register, RxSymbolErrorsW::Register>,
    /// Alignment Errors
    pub alignment_errors: Aliased<u32, AlignmentErrorsR::Register, AlignmentErrorsW::Register>,
    /// Receive Resource Errors
    pub rx_resource_errors: Aliased<u32, RxResourceErrorsR::Register, RxResourceErrorsW::Register>,
    /// Receive Overruns
    pub rx_overruns: Aliased<u32, RxOverrunsR::Register, RxOverrunsW::Register>,
    /// IP Header Checksum Errors
    pub rx_ip_ck_errors: Aliased<u32, RxIpCkErrorsR::Register, RxIpCkErrorsW::Register>,
    /// TCP Checksum Errors
    pub rx_tcp_ck_errors: Aliased<u32, RxTcpCkErrorsR::Register, RxTcpCkErrorsW::Register>,
    /// UDP Checksum Errors
    pub rx_udp_ck_errors: Aliased<u32, RxUdpCkErrorsR::Register, RxUdpCkErrorsW::Register>,
    /// Receive DMA Flushed Packets
    pub auto_flushed_pkts: Aliased<u32, AutoFlushedPktsR::Register, AutoFlushedPktsW::Register>,
    _padding440: [u8; 4],
    /// 1588 Timer Increment Register sub nsec
    pub tsu_timer_incr_sub_nsec:
        Aliased<u32, TsuTimerIncrSubNsecR::Register, TsuTimerIncrSubNsecW::Register>,
    /// 1588 Timer Seconds Register 47:32
    pub tsu_timer_msb_sec: Aliased<u32, TsuTimerMsbSecR::Register, TsuTimerMsbSecW::Register>,
    /// 1588 Timer Sync Strobe Seconds Register 47:32
    pub tsu_strobe_msb_sec: ReadOnly<u32, TsuStrobeMsbSec::Register>,
    /// 1588 Timer Sync Strobe Seconds Register 31:0
    pub tsu_strobe_sec: ReadOnly<u32>,
    /// 1588 Timer Sync Strobe Nanoseconds Register
    pub tsu_strobe_nsec: ReadOnly<u32, TsuStrobeNsec::Register>,
    /// 1588 Timer Seconds Register 31:0
    pub tsu_timer_sec: ReadWrite<u32>,
    /// 1588 Timer Nanoseconds Register
    pub tsu_timer_nsec: Aliased<u32, TsuTimerNsecR::Register, TsuTimerNsecW::Register>,
    /// This register returns all zeroes when read.
    pub tsu_timer_adjust: Aliased<u32, TsuTimerAdjustR::Register, TsuTimerAdjustW::Register>,
    /// 1588 Timer Increment Register
    pub tsu_timer_incr: Aliased<u32, TsuTimerIncrR::Register, TsuTimerIncrW::Register>,
    /// PTP Event Frame Transmitted Seconds Register 31:0
    pub tsu_ptp_tx_sec: ReadOnly<u32>,
    /// PTP Event Frame Transmitted Nanoseconds Register
    pub tsu_ptp_tx_nsec: ReadOnly<u32, TsuPtpTxNsec::Register>,
    /// PTP Event Frame Received Seconds Register 31:0
    pub tsu_ptp_rx_sec: ReadOnly<u32>,
    /// PTP Event Frame Received Nanoseconds Register
    pub tsu_ptp_rx_nsec: ReadOnly<u32, TsuPtpRxNsec::Register>,
    /// PTP Peer Event Frame Transmitted Seconds Register 31:0
    pub tsu_peer_tx_sec: ReadOnly<u32>,
    /// PTP Peer Event Frame Transmitted Nanoseconds Register
    pub tsu_peer_tx_nsec: ReadOnly<u32, TsuPeerTxNsec::Register>,
    /// PTP Peer Event Frame Received Seconds Register 31:0
    pub tsu_peer_rx_sec: ReadOnly<u32>,
    /// PTP Peer Event Frame Received Nanoseconds Register
    pub tsu_peer_rx_nsec: ReadOnly<u32, TsuPeerRxNsec::Register>,
    /// Note:All PCS registers are defined in the IEEE 802.3 Standard. PCS Control RegisterThis register provides the main control functions with respect to the PCS.
    pub pcs_control: Aliased<u32, PcsControlR::Register, PcsControlW::Register>,
    /// This register indicates general status information concerning the PCS.
    pub pcs_status: ReadOnly<u32, PcsStatus::Register>,
    /// The value of this register indicates the upper 16-bits of the PHYs identification code. This is a read-only register with a value defined by `gem_phy_id_top
    pub pcs_phy_top_id: ReadOnly<u32, PcsPhyTopId::Register>,
    /// The value of this register indicates the lower 16-bits of the PHYs identification code. This is a read-only register with a value defined by `gem_phy_id_bot
    pub pcs_phy_bot_id: ReadOnly<u32, PcsPhyBotId::Register>,
    /// The value of this register is used to transmit the base page of the GEM PCS capabilities. Note this is only valid for the default configuration where SGMII is not included in the GEM. In this case the registers is reserved. When SGMII is included, this register returns fixed 0x00000001 when read. SGMII specifies that the transmit configuration information sent from the MAC to the PHY is fixed with bit 14 set to 1 to indicate acknowledge, bit 0 set to 1 to indicate SGMII and all other bits set to 0.
    pub pcs_an_adv: Aliased<u32, PcsAnAdvR::Register, PcsAnAdvW::Register>,
    /// For non SGMII (default) mode, the value of this register contains the link partners base page received information. This register is updated in the ABILITY_DETECT state of the PCS auto-negotiation state machine so bit 14 will only be set if the link partner is sending acknowledge while the PCS in this state. The register is not updated in the ACK_DETECT state. For SGMII mode, the contents of this register change to the one defined in the SGMII standard. The value of this register contains the link partners base page received information. In this case the link partner is the PHY connected by the SGMII.
    pub pcs_an_lp_base: ReadOnly<u32, PcsAnLpBase::Register>,
    /// This register contains auto-negotiation next page ability and page received information.
    pub pcs_an_exp: ReadOnly<u32, PcsAnExp::Register>,
    /// The value of this register is used to transmit the next page information for the GEM PCS. For next page exchange to work this register must be written within 10 ms of receiving a new page from the link partner. If the link partner is requesting next pages and the GEM has none or no more to send then this register should be written with the null message (0x2001). The value 0x0000 must not be written to this register.
    pub pcs_an_np_tx: Aliased<u32, PcsAnNpTxR::Register, PcsAnNpTxW::Register>,
    /// This value of this register contains the next page received information from the link partner.
    pub pcs_an_lp_np: ReadOnly<u32, PcsAnLpNp::Register>,
    _padding548: [u8; 24],
    /// This register contains PCS auto-negotiation extended status information.
    pub pcs_an_ext_status: ReadOnly<u32, PcsAnExtStatus::Register>,
    _padding576: [u8; 48],
    /// Received LPI transitions
    pub rx_lpi: Aliased<u32, RxLpiR::Register, RxLpiW::Register>,
    /// Received LPI time
    pub rx_lpi_time: ReadWrite<u32, RxLpiTime::Register>,
    /// Transmit LPI transitions
    pub tx_lpi: Aliased<u32, TxLpiR::Register, TxLpiW::Register>,
    /// Transmit LPI time
    pub tx_lpi_time: Aliased<u32, TxLpiTimeR::Register, TxLpiTimeW::Register>,
    /// The defined parameterized options to configure the IP are read here.
    pub designcfg_debug1: ReadOnly<u32, DesigncfgDebug1::Register>,
    /// Design Configuration Register 2
    pub designcfg_debug2: ReadOnly<u32, DesigncfgDebug2::Register>,
    /// Design Configuration Register 3
    pub designcfg_debug3: ReadOnly<u32, DesigncfgDebug3::Register>,
    /// Design Configuration Register 4
    pub designcfg_debug4: ReadOnly<u32>,
    /// Design Configuration Register 5
    pub designcfg_debug5: ReadOnly<u32, DesigncfgDebug5::Register>,
    /// Design Configuration Register 6
    pub designcfg_debug6: ReadOnly<u32, DesigncfgDebug6::Register>,
    /// Design Configuration Register 7
    pub designcfg_debug7: ReadOnly<u32, DesigncfgDebug7::Register>,
    /// Design Configuration Register 8
    pub designcfg_debug8: ReadOnly<u32, DesigncfgDebug8::Register>,
    /// Design Configuration Register 9
    pub designcfg_debug9: ReadOnly<u32, DesigncfgDebug9::Register>,
    /// Design Configuration Register 10
    pub designcfg_debug10: ReadOnly<u32, DesigncfgDebug10::Register>,
    _padding680: [u8; 344],
    /// Priority Queue Interrupt Status Register
    pub int_q1_status: Aliased<u32, IntQ1StatusR::Register, IntQ1StatusW::Register>,
    _padding1028: [u8; 60],
    /// Start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Since the DMA handles two frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of the system bus operation, the transmit descriptors must be aligned at 64-bit boundaries for each pair of 32-bit descriptors is read from memory using a single bus access.
    pub transmit_q1_ptr: Aliased<u32, TransmitQ1PtrR::Register, TransmitQ1PtrW::Register>,
    _padding1092: [u8; 60],
    /// Start address of the receive buffer queue (receive buffers descriptor list). The receive buffer queue base address must be initialized before receive is enabled through bit 2 of the network control register. Once reception is enabled, any write to the receive buffer queue base address register is ignored. Reading this register returns the location of the descriptor currently being accessed. This value increments as buffers are used. Software should not use this register for determining where to remove received frames from the queue as it constantly changes as new frames are received. Software should instead work its way through the buffer descriptor queue checking the used bits. In terms of the system bus operation, the receive descriptors must be aligned at 64-bit boundaries and each pair of 32-bit descriptors is written to using a single 64-bit bus access.
    pub receive_q1_ptr: Aliased<u32, ReceiveQ1PtrR::Register, ReceiveQ1PtrW::Register>,
    _padding1156: [u8; 28],
    /// Receive Buffer Queue Size
    pub dma_rxbuf_size_q1: Aliased<u32, DmaRxbufSizeQ1R::Register, DmaRxbufSizeQ1W::Register>,
    _padding1188: [u8; 24],
    /// The IdleSlope value is defined as the rate of change of credit when a packet is waiting to be sent. This must not exceed the portTransmitRate which is dependent on the speed of operation, eg, portTranmsitRate: 1Gb/sec= 32h07735940,100Mb/sec = 32h017D7840, 10Mb/sec= 32h002625A0. If 50% of bandwidth was to be allocated to a particular queue in 1Gb/sec mode then the IdleSlope value for that queue would be calculated as 32h07735940/2. Note: Credit-Based Shaping should be disabled prior to updating the IdleSlope values.
    pub cbs_control: Aliased<u32, CbsControlR::Register, CbsControlW::Register>,
    _padding1216: [u8; 8],
    /// Upper 32 bits of transmit buffer descriptor queue base address.
    pub upper_tx_q_base_addr: ReadWrite<u32>,
    /// TX BD control register
    pub tx_bd_control: Aliased<u32, TxBdControlR::Register, TxBdControlW::Register>,
    /// RX BD control register
    pub rx_bd_control: Aliased<u32, RxBdControlR::Register, RxBdControlW::Register>,
    /// Upper 32 bits of receive buffer descriptor queue base address.
    pub upper_rx_q_base_addr: ReadWrite<u32>,
    _padding1240: [u8; 40],
    /// Screening type 1 registers are used to allocate two priority queues to received frames based on certain IP or UDP fields of incoming frames. Firstly, when DS/TC match enable is set (bit 28), the DS (Differentiated Services) field of the received IPv4 header or TCfield (traffic class) of IPv6 headers are matched against bits 11:4. Secondly, when UDP port match enable is set (bit 29), the UDP Destination Port of the received UDP frame is matched against bits 27:12. Both UDP and DS/TC matching can be enabled simultaneously or individually. If a match is successful, then the queue value programmed in bits 3:0 is allocated to the frame. The required number of Type 1 screening registers is configured in the gem defines file. Up to 16 type 1 screening registers have been allocated APB address space between 0x500 and 0x53C.
    pub screening_type_1_register_0:
        Aliased<u32, ScreeningType1Register0R::Register, ScreeningType1Register0W::Register>,
    /// Screening type 1 registers are used to allocate two priority queues to received frames based on certain IP or UDP fields of incoming frames. Firstly, when DS/TC match enable is set (bit 28), the DS (Differentiated Services) field of the received IPv4 header or TCfield (traffic class) of IPv6 headers are matched against bits 11:4. Secondly, when UDP port match enable is set (bit 29), the UDP Destination Port of the received UDP frame is matched against bits 27:12. Both UDP and DS/TC matching can be enabled simultaneously or individually. If a match is successful, then the queue value programmed in bits 3:0 is allocated to the frame. The required number of Type 1 screening registers is configured in the gem defines file. Up to 16 type 1 screening registers have been allocated APB address space between 0x500 and 0x53C.
    pub screening_type_1_register_1:
        Aliased<u32, ScreeningType1Register1R::Register, ScreeningType1Register1W::Register>,
    /// Screening type 1 registers are used to allocate two priority queues to received frames based on certain IP or UDP fields of incoming frames. Firstly, when DS/TC match enable is set (bit 28), the DS (Differentiated Services) field of the received IPv4 header or TCfield (traffic class) of IPv6 headers are matched against bits 11:4. Secondly, when UDP port match enable is set (bit 29), the UDP Destination Port of the received UDP frame is matched against bits 27:12. Both UDP and DS/TC matching can be enabled simultaneously or individually. If a match is successful, then the queue value programmed in bits 3:0 is allocated to the frame. The required number of Type 1 screening registers is configured in the gem defines file. Up to 16 type 1 screening registers have been allocated APB address space between 0x500 and 0x53C.
    pub screening_type_1_register_2:
        Aliased<u32, ScreeningType1Register2R::Register, ScreeningType1Register2W::Register>,
    /// Screening type 1 registers are used to allocate two priority queues to received frames based on certain IP or UDP fields of incoming frames. Firstly, when DS/TC match enable is set (bit 28), the DS (Differentiated Services) field of the received IPv4 header or TCfield (traffic class) of IPv6 headers are matched against bits 11:4. Secondly, when UDP port match enable is set (bit 29), the UDP Destination Port of the received UDP frame is matched against bits 27:12. Both UDP and DS/TC matching can be enabled simultaneously or individually. If a match is successful, then the queue value programmed in bits 3:0 is allocated to the frame. The required number of Type 1 screening registers is configured in the gem defines file. Up to 16 type 1 screening registers have been allocated APB address space between 0x500 and 0x53C.
    pub screening_type_1_register_3:
        Aliased<u32, ScreeningType1Register3R::Register, ScreeningType1Register3W::Register>,
    _padding1296: [u8; 48],
    /// Screener Type 2 match registers allow a screen to be configured that is the combination of all or any of the following comparisons:1) An enabled VLAN Priority. A VLAN Priority match will be performed if the VLAN priority enable is set. The extracted priority field in the VLAN header is compared against 3 bits within the screener type 2 register itself.2) An enabled EtherType.3) An enabled Field Compare A. 4) An enabled Field Compare B. 5) An enabled Field Compare C. If a match is successful, then the queue value programmed in bits 3:0 is allocated to the frame. The required number of Type 2 screening registers is configured in the gem defines file. Up to 16 type 2 screening registers have been allocated APB address space between 0x540 and 0x57C.
    pub screening_type_2_register_0:
        Aliased<u32, ScreeningType2Register0R::Register, ScreeningType2Register0W::Register>,
    /// Screener Type 2 match registers allow a screen to be configured that is the combination of all or any of the following comparisons:1) An enabled VLAN Priority. A VLAN Priority match will be performed if the VLAN priority enable is set. The extracted priority field in the VLAN header is compared against 3 bits within the screener type 2 register itself.2) An enabled EtherType.3) An enabled Field Compare A. 4) An enabled Field Compare B. 5) An enabled Field Compare C. If a match is successful, then the queue value programmed in bits 3:0 is allocated to the frame. The required number of Type 2 screening registers is configured in the gem defines file. Up to 16 type 2 screening registers have been allocated APB address space between 0x540 and 0x57C.
    pub screening_type_2_register_1:
        Aliased<u32, ScreeningType2Register1R::Register, ScreeningType2Register1W::Register>,
    /// Screener Type 2 match registers allow a screen to be configured that is the combination of all or any of the following comparisons:1) An enabled VLAN Priority. A VLAN Priority match will be performed if the VLAN priority enable is set. The extracted priority field in the VLAN header is compared against 3 bits within the screener type 2 register itself.2) An enabled EtherType.3) An enabled Field Compare A. 4) An enabled Field Compare B. 5) An enabled Field Compare C. If a match is successful, then the queue value programmed in bits 3:0 is allocated to the frame. The required number of Type 2 screening registers is configured in the gem defines file. Up to 16 type 2 screening registers have been allocated APB address space between 0x540 and 0x57C.
    pub screening_type_2_register_2:
        Aliased<u32, ScreeningType2Register2R::Register, ScreeningType2Register2W::Register>,
    /// Screener Type 2 match registers allow a screen to be configured that is the combination of all or any of the following comparisons:1) An enabled VLAN Priority. A VLAN Priority match will be performed if the VLAN priority enable is set. The extracted priority field in the VLAN header is compared against 3 bits within the screener type 2 register itself.2) An enabled EtherType.3) An enabled Field Compare A. 4) An enabled Field Compare B. 5) An enabled Field Compare C. If a match is successful, then the queue value programmed in bits 3:0 is allocated to the frame. The required number of Type 2 screening registers is configured in the gem defines file. Up to 16 type 2 screening registers have been allocated APB address space between 0x540 and 0x57C.
    pub screening_type_2_register_3:
        Aliased<u32, ScreeningType2Register3R::Register, ScreeningType2Register3W::Register>,
    _padding1360: [u8; 176],
    /// At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero.
    pub int_q1_enable: Aliased<u32, IntQ1EnableR::Register, IntQ1EnableW::Register>,
    _padding1540: [u8; 28],
    /// Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero.
    pub int_q1_disable: Aliased<u32, IntQ1DisableR::Register, IntQ1DisableW::Register>,
    _padding1572: [u8; 28],
    /// The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register.
    pub int_q1_mask: ReadOnly<u32, IntQ1Mask::Register>,
    _padding1604: [u8; 156],
    /// Ethertype Register
    pub screening_type_2_ethertype_reg_0: Aliased<
        u32,
        ScreeningType2EthertypeReg0R::Register,
        ScreeningType2EthertypeReg0W::Register,
    >,
    /// Ethertype Register
    pub screening_type_2_ethertype_reg_1: Aliased<
        u32,
        ScreeningType2EthertypeReg1R::Register,
        ScreeningType2EthertypeReg1W::Register,
    >,
    /// Ethertype Register
    pub screening_type_2_ethertype_reg_2: Aliased<
        u32,
        ScreeningType2EthertypeReg2R::Register,
        ScreeningType2EthertypeReg2W::Register,
    >,
    /// Ethertype Register
    pub screening_type_2_ethertype_reg_3: Aliased<
        u32,
        ScreeningType2EthertypeReg3R::Register,
        ScreeningType2EthertypeReg3W::Register,
    >,
    _padding1776: [u8; 16],
    /// Compare A,B and C fields of the screener type 2 match register are pointers to a pool of up to 32 compare registers. If enabled the compare is true if the data at the OFFSET into the frame, ANDed with the MASK Value, is equal to the COMPARE Value. A 16 bit word comparison is done. The byte at the OFFSET number of bytes from the index start is compared thru bits 7:0 of the configured VALUE and MASK.The byte at the OFFSET number of bytes + 1 from the index start is compared thru bits 15:8 of the configured VALUE and MASK. The OFFSET can be configured to be from 0 to 127 bytes from either the start of the frame, the byte following the etherType field (last EtherType in the header if the frame is VLAN tagged), the byte following the IP header (IPv4 or IPv6) or from the byte following the start of the TCP/UDP header. The required number of Type 2 screening registers up to a maximum of 32 is configurable in the gem defines file and have been allocated APB address space between 0x700 and 0x7fc.Note. when using RX Partial Store and Forward mode and priority queues, the frame offset must be less than the Partial Store and Forward watermark. If the offset is higher than the watermark value its not possible to identify the priority queue before the frame is sent to the AMBA interface, and an incorrect priority queue may be used. The bit mapping for these registers is as follows:
    pub type2_compare_0_word_0: ReadWrite<u32, Type2Compare0Word0::Register>,
    /// Type2 Compare Word 1
    pub type2_compare_0_word_1:
        Aliased<u32, Type2Compare0Word1R::Register, Type2Compare0Word1W::Register>,
    /// Compare A,B and C fields of the screener type 2 match register are pointers to a pool of up to 32 compare registers. If enabled the compare is true if the data at the OFFSET into the frame, ANDed with the MASK Value, is equal to the COMPARE Value. A 16 bit word comparison is done. The byte at the OFFSET number of bytes from the index start is compared thru bits 7:0 of the configured VALUE and MASK.The byte at the OFFSET number of bytes + 1 from the index start is compared thru bits 15:8 of the configured VALUE and MASK. The OFFSET can be configured to be from 0 to 127 bytes from either the start of the frame, the byte following the etherType field (last EtherType in the header if the frame is VLAN tagged), the byte following the IP header (IPv4 or IPv6) or from the byte following the start of the TCP/UDP header. The required number of Type 2 screening registers up to a maximum of 32 is configurable in the gem defines file and have been allocated APB address space between 0x700 and 0x7fc.Note. when using RX Partial Store and Forward mode and priority queues, the frame offset must be less than the Partial Store and Forward watermark. If the offset is higher than the watermark value its not possible to identify the priority queue before the frame is sent to the AMBA interface, and an incorrect priority queue may be used. The bit mapping for these registers is as follows:
    pub type2_compare_1_word_0: ReadWrite<u32, Type2Compare1Word0::Register>,
    /// Type2 Compare Word 1
    pub type2_compare_1_word_1:
        Aliased<u32, Type2Compare1Word1R::Register, Type2Compare1Word1W::Register>,
    /// Compare A,B and C fields of the screener type 2 match register are pointers to a pool of up to 32 compare registers. If enabled the compare is true if the data at the OFFSET into the frame, ANDed with the MASK Value, is equal to the COMPARE Value. A 16 bit word comparison is done. The byte at the OFFSET number of bytes from the index start is compared thru bits 7:0 of the configured VALUE and MASK.The byte at the OFFSET number of bytes + 1 from the index start is compared thru bits 15:8 of the configured VALUE and MASK. The OFFSET can be configured to be from 0 to 127 bytes from either the start of the frame, the byte following the etherType field (last EtherType in the header if the frame is VLAN tagged), the byte following the IP header (IPv4 or IPv6) or from the byte following the start of the TCP/UDP header. The required number of Type 2 screening registers up to a maximum of 32 is configurable in the gem defines file and have been allocated APB address space between 0x700 and 0x7fc.Note. when using RX Partial Store and Forward mode and priority queues, the frame offset must be less than the Partial Store and Forward watermark. If the offset is higher than the watermark value its not possible to identify the priority queue before the frame is sent to the AMBA interface, and an incorrect priority queue may be used. The bit mapping for these registers is as follows:
    pub type2_compare_2_word_0: ReadWrite<u32, Type2Compare2Word0::Register>,
    /// Type2 Compare Word 1
    pub type2_compare_2_word_1:
        Aliased<u32, Type2Compare2Word1R::Register, Type2Compare2Word1W::Register>,
    /// Compare A,B and C fields of the screener type 2 match register are pointers to a pool of up to 32 compare registers. If enabled the compare is true if the data at the OFFSET into the frame, ANDed with the MASK Value, is equal to the COMPARE Value. A 16 bit word comparison is done. The byte at the OFFSET number of bytes from the index start is compared thru bits 7:0 of the configured VALUE and MASK.The byte at the OFFSET number of bytes + 1 from the index start is compared thru bits 15:8 of the configured VALUE and MASK. The OFFSET can be configured to be from 0 to 127 bytes from either the start of the frame, the byte following the etherType field (last EtherType in the header if the frame is VLAN tagged), the byte following the IP header (IPv4 or IPv6) or from the byte following the start of the TCP/UDP header. The required number of Type 2 screening registers up to a maximum of 32 is configurable in the gem defines file and have been allocated APB address space between 0x700 and 0x7fc.Note. when using RX Partial Store and Forward mode and priority queues, the frame offset must be less than the Partial Store and Forward watermark. If the offset is higher than the watermark value its not possible to identify the priority queue before the frame is sent to the AMBA interface, and an incorrect priority queue may be used. The bit mapping for these registers is as follows:
    pub type2_compare_3_word_0: ReadWrite<u32, Type2Compare3Word0::Register>,
    /// Type2 Compare Word 1
    pub type2_compare_3_word_1:
        Aliased<u32, Type2Compare3Word1R::Register, Type2Compare3Word1W::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub NetworkControlR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        RESERVED1 OFFSET(25) NUMBITS(1) [],
        ONE_STEP_SYNC_MODE OFFSET(24) NUMBITS(1) [],
        EXT_TSU_PORT_ENABLE OFFSET(23) NUMBITS(1) [],
        STORE_UDP_OFFSET OFFSET(22) NUMBITS(1) [],
        ALT_SGMII_MODE OFFSET(21) NUMBITS(1) [],
        PTP_UNICAST_ENA OFFSET(20) NUMBITS(1) [],
        TX_LPI_EN OFFSET(19) NUMBITS(1) [],
        FLUSH_RX_PKT_PCLK OFFSET(18) NUMBITS(1) [],
        PFC_ENABLE OFFSET(16) NUMBITS(1) [],
        STORE_RX_TS OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(2) [],
        TX_HALT_PCLK OFFSET(10) NUMBITS(1) [],
        TX_START_PCLK OFFSET(9) NUMBITS(1) [],
        BACK_PRESSURE OFFSET(8) NUMBITS(1) [],
        STATS_WRITE_EN OFFSET(7) NUMBITS(1) [],
        INC_ALL_STATS_REGS OFFSET(6) NUMBITS(1) [],
        CLEAR_ALL_STATS_REGS OFFSET(5) NUMBITS(1) [],
        MAN_PORT_EN OFFSET(4) NUMBITS(1) [],
        ENABLE_TRANSMIT OFFSET(3) NUMBITS(1) [],
        ENABLE_RECEIVE OFFSET(2) NUMBITS(1) [],
        LOOPBACK_LOCAL OFFSET(1) NUMBITS(1) [],
        LOOPBACK OFFSET(0) NUMBITS(1) [],
    ],
    pub NetworkControlW [
        RESERVED0 OFFSET(25) NUMBITS(1) [],
        ONE_STEP_SYNC_MODE OFFSET(24) NUMBITS(1) [],
        EXT_TSU_PORT_ENABLE OFFSET(23) NUMBITS(1) [],
        STORE_UDP_OFFSET OFFSET(22) NUMBITS(1) [],
        ALT_SGMII_MODE OFFSET(21) NUMBITS(1) [],
        PTP_UNICAST_ENA OFFSET(20) NUMBITS(1) [],
        TX_LPI_EN OFFSET(19) NUMBITS(1) [],
        FLUSH_RX_PKT_PCLK OFFSET(18) NUMBITS(1) [],
        TRANSMIT_PFC_PRIORITY_BASED_PAUSE_FRAME OFFSET(17) NUMBITS(1) [],
        PFC_ENABLE OFFSET(16) NUMBITS(1) [],
        STORE_RX_TS OFFSET(15) NUMBITS(1) [],
        TX_PAUSE_FRAME_ZERO OFFSET(12) NUMBITS(1) [],
        TX_PAUSE_FRAME_REQ OFFSET(11) NUMBITS(1) [],
        TX_HALT_PCLK OFFSET(10) NUMBITS(1) [],
        TX_START_PCLK OFFSET(9) NUMBITS(1) [],
        BACK_PRESSURE OFFSET(8) NUMBITS(1) [],
        STATS_WRITE_EN OFFSET(7) NUMBITS(1) [],
        INC_ALL_STATS_REGS OFFSET(6) NUMBITS(1) [],
        CLEAR_ALL_STATS_REGS OFFSET(5) NUMBITS(1) [],
        MAN_PORT_EN OFFSET(4) NUMBITS(1) [],
        ENABLE_TRANSMIT OFFSET(3) NUMBITS(1) [],
        ENABLE_RECEIVE OFFSET(2) NUMBITS(1) [],
        LOOPBACK_LOCAL OFFSET(1) NUMBITS(1) [],
        LOOPBACK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NetworkConfig [
        UNI_DIRECTION_ENABLE OFFSET(31) NUMBITS(1) [],
        IGNORE_IPG_RX_ER OFFSET(30) NUMBITS(1) [],
        NSP_CHANGE OFFSET(29) NUMBITS(1) [],
        IPG_STRETCH_ENABLE OFFSET(28) NUMBITS(1) [],
        SGMII_MODE_ENABLE OFFSET(27) NUMBITS(1) [],
        IGNORE_RX_FCS OFFSET(26) NUMBITS(1) [],
        EN_HALF_DUPLEX_RX OFFSET(25) NUMBITS(1) [],
        RECEIVE_CHECKSUM_OFFLOAD_ENABLE OFFSET(24) NUMBITS(1) [],
        DISABLE_COPY_OF_PAUSE_FRAMES OFFSET(23) NUMBITS(1) [],
        DATA_BUS_WIDTH OFFSET(21) NUMBITS(2) [],
        MDC_CLOCK_DIVISION OFFSET(18) NUMBITS(3) [],
        FCS_REMOVE OFFSET(17) NUMBITS(1) [],
        LENGTH_FIELD_ERROR_FRAME_DISCARD OFFSET(16) NUMBITS(1) [],
        RECEIVE_BUFFER_OFFSET OFFSET(14) NUMBITS(2) [],
        PAUSE_ENABLE OFFSET(13) NUMBITS(1) [],
        RETRY_TEST OFFSET(12) NUMBITS(1) [],
        PCS_SELECT OFFSET(11) NUMBITS(1) [],
        GIGABIT_MODE_ENABLE OFFSET(10) NUMBITS(1) [],
        EXTERNAL_ADDRESS_MATCH_ENABLE OFFSET(9) NUMBITS(1) [],
        RECEIVE_1536_BYTE_FRAMES OFFSET(8) NUMBITS(1) [],
        UNICAST_HASH_ENABLE OFFSET(7) NUMBITS(1) [],
        MULTICAST_HASH_ENABLE OFFSET(6) NUMBITS(1) [],
        NO_BROADCAST OFFSET(5) NUMBITS(1) [],
        COPY_ALL_FRAMES OFFSET(4) NUMBITS(1) [],
        JUMBO_FRAMES OFFSET(3) NUMBITS(1) [],
        DISCARD_NON_VLAN_FRAMES OFFSET(2) NUMBITS(1) [],
        FULL_DUPLEX OFFSET(1) NUMBITS(1) [],
        SPEED OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NetworkStatus [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        LPI_INDICATE_PCLK OFFSET(7) NUMBITS(1) [],
        PFC_NEGOTIATE_PCLK OFFSET(6) NUMBITS(1) [],
        MAC_PAUSE_TX_EN OFFSET(5) NUMBITS(1) [],
        MAC_PAUSE_RX_EN OFFSET(4) NUMBITS(1) [],
        MAC_FULL_DUPLEX OFFSET(3) NUMBITS(1) [],
        MAN_DONE OFFSET(2) NUMBITS(1) [],
        MDIO_IN OFFSET(1) NUMBITS(1) [],
        PCS_LINK_STATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DmaConfigR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        DMA_ADDR_BUS_WIDTH_1 OFFSET(30) NUMBITS(1) [],
        TX_BD_EXTENDED_MODE_EN OFFSET(29) NUMBITS(1) [],
        RX_BD_EXTENDED_MODE_EN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(27) NUMBITS(1) [],
        FORCE_MAX_AMBA_BURST_TX OFFSET(26) NUMBITS(1) [],
        FORCE_MAX_AMBA_BURST_RX OFFSET(25) NUMBITS(1) [],
        FORCE_DISCARD_ON_ERR OFFSET(24) NUMBITS(1) [],
        RX_BUF_SIZE OFFSET(16) NUMBITS(8) [],
        RESERVED2 OFFSET(12) NUMBITS(4) [],
        TX_PBUF_TCP_EN OFFSET(11) NUMBITS(1) [],
        TX_PBUF_SIZE OFFSET(10) NUMBITS(1) [],
        RX_PBUF_SIZE OFFSET(8) NUMBITS(2) [],
        ENDIAN_SWAP_PACKET OFFSET(7) NUMBITS(1) [],
        ENDIAN_SWAP_MANAGEMENT OFFSET(6) NUMBITS(1) [],
        RESERVED3 OFFSET(5) NUMBITS(1) [],
        AMBA_BURST_LENGTH OFFSET(0) NUMBITS(5) [],
    ],
    pub DmaConfigW [
        DMA_ADDR_BUS_WIDTH_1 OFFSET(30) NUMBITS(1) [],
        TX_BD_EXTENDED_MODE_EN OFFSET(29) NUMBITS(1) [],
        RX_BD_EXTENDED_MODE_EN OFFSET(28) NUMBITS(1) [],
        FORCE_MAX_AMBA_BURST_TX OFFSET(26) NUMBITS(1) [],
        FORCE_MAX_AMBA_BURST_RX OFFSET(25) NUMBITS(1) [],
        FORCE_DISCARD_ON_ERR OFFSET(24) NUMBITS(1) [],
        RX_BUF_SIZE OFFSET(16) NUMBITS(8) [],
        TX_PBUF_TCP_EN OFFSET(11) NUMBITS(1) [],
        TX_PBUF_SIZE OFFSET(10) NUMBITS(1) [],
        RX_PBUF_SIZE OFFSET(8) NUMBITS(2) [],
        ENDIAN_SWAP_PACKET OFFSET(7) NUMBITS(1) [],
        ENDIAN_SWAP_MANAGEMENT OFFSET(6) NUMBITS(1) [],
        AMBA_BURST_LENGTH OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TransmitStatusR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        RESP_NOT_OK OFFSET(8) NUMBITS(1) [],
        LATE_COLLISION_OCCURRED OFFSET(7) NUMBITS(1) [],
        TRANSMIT_UNDER_RUN OFFSET(6) NUMBITS(1) [],
        TRANSMIT_COMPLETE OFFSET(5) NUMBITS(1) [],
        AMBA_ERROR OFFSET(4) NUMBITS(1) [],
        TRANSMIT_GO OFFSET(3) NUMBITS(1) [],
        RETRY_LIMIT_EXCEEDED OFFSET(2) NUMBITS(1) [],
        COLLISION_OCCURRED OFFSET(1) NUMBITS(1) [],
        USED_BIT_READ OFFSET(0) NUMBITS(1) [],
    ],
    pub TransmitStatusW [
        RESP_NOT_OK OFFSET(8) NUMBITS(1) [],
        LATE_COLLISION_OCCURRED OFFSET(7) NUMBITS(1) [],
        TRANSMIT_UNDER_RUN OFFSET(6) NUMBITS(1) [],
        TRANSMIT_COMPLETE OFFSET(5) NUMBITS(1) [],
        AMBA_ERROR OFFSET(4) NUMBITS(1) [],
        RETRY_LIMIT_EXCEEDED OFFSET(2) NUMBITS(1) [],
        COLLISION_OCCURRED OFFSET(1) NUMBITS(1) [],
        USED_BIT_READ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReceiveQPtrR [
        DMA_RX_Q_PTR OFFSET(2) NUMBITS(30) [],
        RESERVED0 OFFSET(0) NUMBITS(2) [],
    ],
    pub ReceiveQPtrW [
        DMA_RX_Q_PTR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TransmitQPtrR [
        DMA_TX_Q_PTR OFFSET(2) NUMBITS(30) [],
        RESERVED0 OFFSET(0) NUMBITS(2) [],
    ],
    pub TransmitQPtrW [
        DMA_TX_Q_PTR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReceiveStatusR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        RESP_NOT_OK OFFSET(3) NUMBITS(1) [],
        RECEIVE_OVERRUN OFFSET(2) NUMBITS(1) [],
        FRAME_RECEIVED OFFSET(1) NUMBITS(1) [],
        BUFFER_NOT_AVAILABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub ReceiveStatusW [
        RESP_NOT_OK OFFSET(3) NUMBITS(1) [],
        RECEIVE_OVERRUN OFFSET(2) NUMBITS(1) [],
        FRAME_RECEIVED OFFSET(1) NUMBITS(1) [],
        BUFFER_NOT_AVAILABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntStatusR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        TSU_TIMER_COMPARISON_INTERRUPT OFFSET(29) NUMBITS(1) [],
        WOL_INTERRUPT OFFSET(28) NUMBITS(1) [],
        RECEIVE_LPI_INDICATION_STATUS_BIT_CHANGE OFFSET(27) NUMBITS(1) [],
        TSU_SECONDS_REGISTER_INCREMENT OFFSET(26) NUMBITS(1) [],
        PTP_PDELAY_RESP_FRAME_TRANSMITTED OFFSET(25) NUMBITS(1) [],
        PTP_PDELAY_REQ_FRAME_TRANSMITTED OFFSET(24) NUMBITS(1) [],
        PTP_PDELAY_RESP_FRAME_RECEIVED OFFSET(23) NUMBITS(1) [],
        PTP_PDELAY_REQ_FRAME_RECEIVED OFFSET(22) NUMBITS(1) [],
        PTP_SYNC_FRAME_TRANSMITTED OFFSET(21) NUMBITS(1) [],
        PTP_DELAY_REQ_FRAME_TRANSMITTED OFFSET(20) NUMBITS(1) [],
        PTP_SYNC_FRAME_RECEIVED OFFSET(19) NUMBITS(1) [],
        PTP_DELAY_REQ_FRAME_RECEIVED OFFSET(18) NUMBITS(1) [],
        PCS_LINK_PARTNER_PAGE_RECEIVED OFFSET(17) NUMBITS(1) [],
        PCS_AUTO_NEGOTIATION_COMPLETE OFFSET(16) NUMBITS(1) [],
        EXTERNAL_INTERRUPT OFFSET(15) NUMBITS(1) [],
        PAUSE_FRAME_TRANSMITTED OFFSET(14) NUMBITS(1) [],
        PAUSE_TIME_ELAPSED OFFSET(13) NUMBITS(1) [],
        PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_RECEIVED OFFSET(12) NUMBITS(1) [],
        RESP_NOT_OK OFFSET(11) NUMBITS(1) [],
        RECEIVE_OVERRUN OFFSET(10) NUMBITS(1) [],
        LINK_CHANGE OFFSET(9) NUMBITS(1) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        TRANSMIT_COMPLETE OFFSET(7) NUMBITS(1) [],
        AMBA_ERROR OFFSET(6) NUMBITS(1) [],
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION OFFSET(5) NUMBITS(1) [],
        TRANSMIT_UNDER_RUN OFFSET(4) NUMBITS(1) [],
        TX_USED_BIT_READ OFFSET(3) NUMBITS(1) [],
        RX_USED_BIT_READ OFFSET(2) NUMBITS(1) [],
        RECEIVE_COMPLETE OFFSET(1) NUMBITS(1) [],
        MANAGEMENT_FRAME_SENT OFFSET(0) NUMBITS(1) [],
    ],
    pub IntStatusW [
        TSU_TIMER_COMPARISON_INTERRUPT OFFSET(29) NUMBITS(1) [],
        WOL_INTERRUPT OFFSET(28) NUMBITS(1) [],
        RECEIVE_LPI_INDICATION_STATUS_BIT_CHANGE OFFSET(27) NUMBITS(1) [],
        TSU_SECONDS_REGISTER_INCREMENT OFFSET(26) NUMBITS(1) [],
        PTP_PDELAY_RESP_FRAME_TRANSMITTED OFFSET(25) NUMBITS(1) [],
        PTP_PDELAY_REQ_FRAME_TRANSMITTED OFFSET(24) NUMBITS(1) [],
        PTP_PDELAY_RESP_FRAME_RECEIVED OFFSET(23) NUMBITS(1) [],
        PTP_PDELAY_REQ_FRAME_RECEIVED OFFSET(22) NUMBITS(1) [],
        PTP_SYNC_FRAME_TRANSMITTED OFFSET(21) NUMBITS(1) [],
        PTP_DELAY_REQ_FRAME_TRANSMITTED OFFSET(20) NUMBITS(1) [],
        PTP_SYNC_FRAME_RECEIVED OFFSET(19) NUMBITS(1) [],
        PTP_DELAY_REQ_FRAME_RECEIVED OFFSET(18) NUMBITS(1) [],
        PCS_LINK_PARTNER_PAGE_RECEIVED OFFSET(17) NUMBITS(1) [],
        PCS_AUTO_NEGOTIATION_COMPLETE OFFSET(16) NUMBITS(1) [],
        EXTERNAL_INTERRUPT OFFSET(15) NUMBITS(1) [],
        PAUSE_FRAME_TRANSMITTED OFFSET(14) NUMBITS(1) [],
        PAUSE_TIME_ELAPSED OFFSET(13) NUMBITS(1) [],
        PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_RECEIVED OFFSET(12) NUMBITS(1) [],
        RESP_NOT_OK OFFSET(11) NUMBITS(1) [],
        RECEIVE_OVERRUN OFFSET(10) NUMBITS(1) [],
        LINK_CHANGE OFFSET(9) NUMBITS(1) [],
        TRANSMIT_COMPLETE OFFSET(7) NUMBITS(1) [],
        AMBA_ERROR OFFSET(6) NUMBITS(1) [],
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION OFFSET(5) NUMBITS(1) [],
        TRANSMIT_UNDER_RUN OFFSET(4) NUMBITS(1) [],
        TX_USED_BIT_READ OFFSET(3) NUMBITS(1) [],
        RX_USED_BIT_READ OFFSET(2) NUMBITS(1) [],
        RECEIVE_COMPLETE OFFSET(1) NUMBITS(1) [],
        MANAGEMENT_FRAME_SENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntEnableR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        NOT_USED OFFSET(8) NUMBITS(1) [],
    ],
    pub IntEnableW [
        ENABLE_TSU_TIMER_COMPARISON_INTERRUPT OFFSET(29) NUMBITS(1) [],
        ENABLE_WOL_EVENT_RECEIVED_INTERRUPT OFFSET(28) NUMBITS(1) [],
        ENABLE_RX_LPI_INDICATION_INTERRUPT OFFSET(27) NUMBITS(1) [],
        ENABLE_TSU_SECONDS_REGISTER_INCREMENT OFFSET(26) NUMBITS(1) [],
        ENABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED OFFSET(25) NUMBITS(1) [],
        ENABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED OFFSET(24) NUMBITS(1) [],
        ENABLE_PTP_PDELAY_RESP_FRAME_RECEIVED OFFSET(23) NUMBITS(1) [],
        ENABLE_PTP_PDELAY_REQ_FRAME_RECEIVED OFFSET(22) NUMBITS(1) [],
        ENABLE_PTP_SYNC_FRAME_TRANSMITTED OFFSET(21) NUMBITS(1) [],
        ENABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED OFFSET(20) NUMBITS(1) [],
        ENABLE_PTP_SYNC_FRAME_RECEIVED OFFSET(19) NUMBITS(1) [],
        ENABLE_PTP_DELAY_REQ_FRAME_RECEIVED OFFSET(18) NUMBITS(1) [],
        ENABLE_PCS_LINK_PARTNER_PAGE_RECEIVED OFFSET(17) NUMBITS(1) [],
        ENABLE_PCS_AUTO_NEGOTIATION_COMPLETE_INTERRUPT OFFSET(16) NUMBITS(1) [],
        ENABLE_EXTERNAL_INTERRUPT OFFSET(15) NUMBITS(1) [],
        ENABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT OFFSET(14) NUMBITS(1) [],
        ENABLE_PAUSE_TIME_ZERO_INTERRUPT OFFSET(13) NUMBITS(1) [],
        ENABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT OFFSET(12) NUMBITS(1) [],
        ENABLE_RESP_NOT_OK_INTERRUPT OFFSET(11) NUMBITS(1) [],
        ENABLE_RECEIVE_OVERRUN_INTERRUPT OFFSET(10) NUMBITS(1) [],
        ENABLE_LINK_CHANGE_INTERRUPT OFFSET(9) NUMBITS(1) [],
        ENABLE_TRANSMIT_COMPLETE_INTERRUPT OFFSET(7) NUMBITS(1) [],
        ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT OFFSET(6) NUMBITS(1) [],
        ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT OFFSET(5) NUMBITS(1) [],
        ENABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT OFFSET(4) NUMBITS(1) [],
        ENABLE_TRANSMIT_USED_BIT_READ_INTERRUPT OFFSET(3) NUMBITS(1) [],
        ENABLE_RECEIVE_USED_BIT_READ_INTERRUPT OFFSET(2) NUMBITS(1) [],
        ENABLE_RECEIVE_COMPLETE_INTERRUPT OFFSET(1) NUMBITS(1) [],
        ENABLE_MANAGEMENT_DONE_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntDisableR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        NOT_USED OFFSET(8) NUMBITS(1) [],
    ],
    pub IntDisableW [
        DISABLE_TSU_TIMER_COMPARISON_INTERRUPT OFFSET(29) NUMBITS(1) [],
        DISABLE_WOL_EVENT_RECEIVED_INTERRUPT OFFSET(28) NUMBITS(1) [],
        DISABLE_RX_LPI_INDICATION_INTERRUPT OFFSET(27) NUMBITS(1) [],
        DISABLE_TSU_SECONDS_REGISTER_INCREMENT OFFSET(26) NUMBITS(1) [],
        DISABLE_PTP_PDELAY_RESP_FRAME_TRANSMITTED OFFSET(25) NUMBITS(1) [],
        DISABLE_PTP_PDELAY_REQ_FRAME_TRANSMITTED OFFSET(24) NUMBITS(1) [],
        DISABLE_PTP_PDELAY_RESP_FRAME_RECEIVED OFFSET(23) NUMBITS(1) [],
        DISABLE_PTP_PDELAY_REQ_FRAME_RECEIVED OFFSET(22) NUMBITS(1) [],
        DISABLE_PTP_SYNC_FRAME_TRANSMITTED OFFSET(21) NUMBITS(1) [],
        DISABLE_PTP_DELAY_REQ_FRAME_TRANSMITTED OFFSET(20) NUMBITS(1) [],
        DISABLE_PTP_SYNC_FRAME_RECEIVED OFFSET(19) NUMBITS(1) [],
        DISABLE_PTP_DELAY_REQ_FRAME_RECEIVED OFFSET(18) NUMBITS(1) [],
        DISABLE_PCS_LINK_PARTNER_PAGE_RECEIVED OFFSET(17) NUMBITS(1) [],
        DISABLE_PCS_AUTO_NEGOTIATION_COMPLETE_INTERRUPT OFFSET(16) NUMBITS(1) [],
        DISABLE_EXTERNAL_INTERRUPT OFFSET(15) NUMBITS(1) [],
        DISABLE_PAUSE_FRAME_TRANSMITTED_INTERRUPT OFFSET(14) NUMBITS(1) [],
        DISABLE_PAUSE_TIME_ZERO_INTERRUPT OFFSET(13) NUMBITS(1) [],
        DISABLE_PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT OFFSET(12) NUMBITS(1) [],
        DISABLE_RESP_NOT_OK_INTERRUPT OFFSET(11) NUMBITS(1) [],
        DISABLE_RECEIVE_OVERRUN_INTERRUPT OFFSET(10) NUMBITS(1) [],
        DISABLE_LINK_CHANGE_INTERRUPT OFFSET(9) NUMBITS(1) [],
        DISABLE_TRANSMIT_COMPLETE_INTERRUPT OFFSET(7) NUMBITS(1) [],
        DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT OFFSET(6) NUMBITS(1) [],
        DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT OFFSET(5) NUMBITS(1) [],
        DISABLE_TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT OFFSET(4) NUMBITS(1) [],
        DISABLE_TRANSMIT_USED_BIT_READ_INTERRUPT OFFSET(3) NUMBITS(1) [],
        DISABLE_RECEIVE_USED_BIT_READ_INTERRUPT OFFSET(2) NUMBITS(1) [],
        DISABLE_RECEIVE_COMPLETE_INTERRUPT OFFSET(1) NUMBITS(1) [],
        DISABLE_MANAGEMENT_DONE_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntMask [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        TSU_TIMER_COMPARISON_MASK OFFSET(29) NUMBITS(1) [],
        WOL_EVENT_RECEIVED_MASK OFFSET(28) NUMBITS(1) [],
        RX_LPI_INDICATION_MASK OFFSET(27) NUMBITS(1) [],
        TSU_SECONDS_REGISTER_INCREMENT_MASK OFFSET(26) NUMBITS(1) [],
        PTP_PDELAY_RESP_FRAME_TRANSMITTED_MASK OFFSET(25) NUMBITS(1) [],
        PTP_PDELAY_REQ_FRAME_TRANSMITTED_MASK OFFSET(24) NUMBITS(1) [],
        PTP_PDELAY_RESP_FRAME_RECEIVED_MASK OFFSET(23) NUMBITS(1) [],
        PTP_PDELAY_REQ_FRAME_RECEIVED_MASK OFFSET(22) NUMBITS(1) [],
        PTP_SYNC_FRAME_TRANSMITTED_MASK OFFSET(21) NUMBITS(1) [],
        PTP_DELAY_REQ_FRAME_TRANSMITTED_MASK OFFSET(20) NUMBITS(1) [],
        PTP_SYNC_FRAME_RECEIVED_MASK OFFSET(19) NUMBITS(1) [],
        PTP_DELAY_REQ_FRAME_RECEIVED_MASK OFFSET(18) NUMBITS(1) [],
        PCS_LINK_PARTNER_PAGE_MASK OFFSET(17) NUMBITS(1) [],
        PCS_AUTO_NEGOTIATION_COMPLETE_INTERRUPT_MASK OFFSET(16) NUMBITS(1) [],
        EXTERNAL_INTERRUPT_MASK OFFSET(15) NUMBITS(1) [],
        PAUSE_FRAME_TRANSMITTED_INTERRUPT_MASK OFFSET(14) NUMBITS(1) [],
        PAUSE_TIME_ZERO_INTERRUPT_MASK OFFSET(13) NUMBITS(1) [],
        PAUSE_FRAME_WITH_NON_ZERO_PAUSE_QUANTUM_INTERRUPT_MASK OFFSET(12) NUMBITS(1) [],
        RESP_NOT_OK_INTERRUPT_MASK OFFSET(11) NUMBITS(1) [],
        RECEIVE_OVERRUN_INTERRUPT_MASK OFFSET(10) NUMBITS(1) [],
        LINK_CHANGE_INTERRUPT_MASK OFFSET(9) NUMBITS(1) [],
        NOT_USED OFFSET(8) NUMBITS(1) [],
        TRANSMIT_COMPLETE_INTERRUPT_MASK OFFSET(7) NUMBITS(1) [],
        AMBA_ERROR_INTERRUPT_MASK OFFSET(6) NUMBITS(1) [],
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION OFFSET(5) NUMBITS(1) [],
        TRANSMIT_BUFFER_UNDER_RUN_INTERRUPT_MASK OFFSET(4) NUMBITS(1) [],
        TRANSMIT_USED_BIT_READ_INTERRUPT_MASK OFFSET(3) NUMBITS(1) [],
        RECEIVE_USED_BIT_READ_INTERRUPT_MASK OFFSET(2) NUMBITS(1) [],
        RECEIVE_COMPLETE_INTERRUPT_MASK OFFSET(1) NUMBITS(1) [],
        MANAGEMENT_DONE_INTERRUPT_MASK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PhyManagement [
        WRITE0 OFFSET(31) NUMBITS(1) [],
        WRITE1 OFFSET(30) NUMBITS(1) [],
        OPERATION OFFSET(28) NUMBITS(2) [],
        PHY_ADDRESS OFFSET(23) NUMBITS(5) [],
        REGISTER_ADDRESS OFFSET(18) NUMBITS(5) [],
        WRITE10 OFFSET(16) NUMBITS(2) [],
        PHY_WRITE_READ_DATA OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PauseTime [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        QUANTUM OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxPauseQuantumR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        QUANTUM OFFSET(0) NUMBITS(16) [],
    ],
    pub TxPauseQuantumW [
        QUANTUM OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PbufTxcutthruR [
        DMA_TX_CUTTHRU OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(12) NUMBITS(19) [],
        DMA_TX_CUTTHRU_THRESHOLD OFFSET(0) NUMBITS(12) [],
    ],
    pub PbufTxcutthruW [
        DMA_TX_CUTTHRU OFFSET(31) NUMBITS(1) [],
        DMA_TX_CUTTHRU_THRESHOLD OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PbufRxcutthruR [
        DMA_RX_CUTTHRU OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(12) NUMBITS(19) [],
        DMA_RX_CUTTHRU_THRESHOLD OFFSET(0) NUMBITS(12) [],
    ],
    pub PbufRxcutthruW [
        DMA_RX_CUTTHRU OFFSET(31) NUMBITS(1) [],
        DMA_RX_CUTTHRU_THRESHOLD OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub JumboMaxLengthR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        JUMBO_MAX_LENGTH OFFSET(0) NUMBITS(16) [],
    ],
    pub JumboMaxLengthW [
        JUMBO_MAX_LENGTH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ExternalFifoInterfaceR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        EXTERNAL_FIFO_INTERFACE OFFSET(0) NUMBITS(1) [],
    ],
    pub ExternalFifoInterfaceW [
        EXTERNAL_FIFO_INTERFACE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AxiMaxPipelineR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        AW2W_MAX_PIPELINE OFFSET(8) NUMBITS(8) [],
        AR2R_MAX_PIPELINE OFFSET(0) NUMBITS(8) [],
    ],
    pub AxiMaxPipelineW [
        AW2W_MAX_PIPELINE OFFSET(8) NUMBITS(8) [],
        AR2R_MAX_PIPELINE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SpecAdd1TopR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        FILTER_TYPE OFFSET(16) NUMBITS(1) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ],
    pub SpecAdd1TopW [
        FILTER_TYPE OFFSET(16) NUMBITS(1) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SpecAdd2TopR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        FILTER_BYTE_MASK OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        FILTER_TYPE OFFSET(16) NUMBITS(1) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ],
    pub SpecAdd2TopW [
        FILTER_BYTE_MASK OFFSET(24) NUMBITS(6) [],
        FILTER_TYPE OFFSET(16) NUMBITS(1) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SpecAdd3TopR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        FILTER_BYTE_MASK OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        FILTER_TYPE OFFSET(16) NUMBITS(1) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ],
    pub SpecAdd3TopW [
        FILTER_BYTE_MASK OFFSET(24) NUMBITS(6) [],
        FILTER_TYPE OFFSET(16) NUMBITS(1) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SpecAdd4TopR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        FILTER_BYTE_MASK OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        FILTER_TYPE OFFSET(16) NUMBITS(1) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ],
    pub SpecAdd4TopW [
        FILTER_BYTE_MASK OFFSET(24) NUMBITS(6) [],
        FILTER_TYPE OFFSET(16) NUMBITS(1) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SpecType1R [
        ENABLE_COPY OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ],
    pub SpecType1W [
        ENABLE_COPY OFFSET(31) NUMBITS(1) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SpecType2R [
        ENABLE_COPY OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ],
    pub SpecType2W [
        ENABLE_COPY OFFSET(31) NUMBITS(1) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SpecType3R [
        ENABLE_COPY OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ],
    pub SpecType3W [
        ENABLE_COPY OFFSET(31) NUMBITS(1) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SpecType4R [
        ENABLE_COPY OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ],
    pub SpecType4W [
        ENABLE_COPY OFFSET(31) NUMBITS(1) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub WolR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        WOL_MASK_3 OFFSET(19) NUMBITS(1) [],
        WOL_MASK_2 OFFSET(18) NUMBITS(1) [],
        WOL_MASK_1 OFFSET(17) NUMBITS(1) [],
        WOL_MASK_0 OFFSET(16) NUMBITS(1) [],
        ADDR OFFSET(0) NUMBITS(16) [],
    ],
    pub WolW [
        WOL_MASK_3 OFFSET(19) NUMBITS(1) [],
        WOL_MASK_2 OFFSET(18) NUMBITS(1) [],
        WOL_MASK_1 OFFSET(17) NUMBITS(1) [],
        WOL_MASK_0 OFFSET(16) NUMBITS(1) [],
        ADDR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub StretchRatioR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        IPG_STRETCH OFFSET(0) NUMBITS(16) [],
    ],
    pub StretchRatioW [
        IPG_STRETCH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub StackedVlanR [
        ENABLE_PROCESSING OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ],
    pub StackedVlanW [
        ENABLE_PROCESSING OFFSET(31) NUMBITS(1) [],
        MATCH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxPfcPauseR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VECTOR OFFSET(8) NUMBITS(8) [],
        VECTOR_ENABLE OFFSET(0) NUMBITS(8) [],
    ],
    pub TxPfcPauseW [
        VECTOR OFFSET(8) NUMBITS(8) [],
        VECTOR_ENABLE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaskAdd1TopR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ADDRESS_MASK OFFSET(0) NUMBITS(16) [],
    ],
    pub MaskAdd1TopW [
        ADDRESS_MASK OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DmaAddrOrMaskR [
        MASK_VALUE OFFSET(28) NUMBITS(4) [],
        RESERVED0 OFFSET(4) NUMBITS(24) [],
        MASK_ENABLE OFFSET(0) NUMBITS(4) [],
    ],
    pub DmaAddrOrMaskW [
        MASK_VALUE OFFSET(28) NUMBITS(4) [],
        MASK_ENABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuNsecCmpR [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        COMPARISON_VALUE OFFSET(0) NUMBITS(22) [],
    ],
    pub TsuNsecCmpW [
        COMPARISON_VALUE OFFSET(0) NUMBITS(22) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuMsbSecCmpR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COMPARISON_VALUE OFFSET(0) NUMBITS(16) [],
    ],
    pub TsuMsbSecCmpW [
        COMPARISON_VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuPtpTxMsbSec [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        TIMER_SECONDS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuPtpRxMsbSec [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        TIMER_SECONDS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuPeerTxMsbSec [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        TIMER_SECONDS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuPeerRxMsbSec [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        TIMER_SECONDS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpramFillDbgR [
        DMA_TX_RX_FILL_LEVEL OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DMA_TX_Q_FILL_LEVEL_SELECT OFFSET(4) NUMBITS(4) [],
        RESERVED1 OFFSET(1) NUMBITS(3) [],
        DMA_TX_RX_FILL_LEVEL_SELECT OFFSET(0) NUMBITS(1) [],
    ],
    pub DpramFillDbgW [
        DMA_TX_Q_FILL_LEVEL_SELECT OFFSET(4) NUMBITS(4) [],
        DMA_TX_RX_FILL_LEVEL_SELECT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RevisionReg [
        FIX_NUMBER OFFSET(28) NUMBITS(4) [],
        MODULE_IDENTIFICATION_NUMBER OFFSET(16) NUMBITS(12) [],
        MODULE_REVISION OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OctetsTxedTopR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub OctetsTxedTopW [
        COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PauseFramesTxedR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub PauseFramesTxedW [
        COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxUnderrunsR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub TxUnderrunsW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SingleCollisionsR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        COUNT OFFSET(0) NUMBITS(18) [],
    ],
    pub SingleCollisionsW [
        COUNT OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MultipleCollisionsR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        COUNT OFFSET(0) NUMBITS(18) [],
    ],
    pub MultipleCollisionsW [
        COUNT OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ExcessiveCollisionsR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub ExcessiveCollisionsW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub LateCollisionsR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub LateCollisionsW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DeferredFramesR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        COUNT OFFSET(0) NUMBITS(18) [],
    ],
    pub DeferredFramesW [
        COUNT OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CrsErrorsR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub CrsErrorsW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OctetsRxedTopR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub OctetsRxedTopW [
        COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PauseFramesRxedR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub PauseFramesRxedW [
        COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub UndersizeFramesR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub UndersizeFramesW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ExcessiveRxLengthR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub ExcessiveRxLengthW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxJabbersR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub RxJabbersW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub FcsErrorsR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub FcsErrorsW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxLengthErrorsR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub RxLengthErrorsW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxSymbolErrorsR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub RxSymbolErrorsW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AlignmentErrorsR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub AlignmentErrorsW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxResourceErrorsR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        COUNT OFFSET(0) NUMBITS(18) [],
    ],
    pub RxResourceErrorsW [
        COUNT OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxOverrunsR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        COUNT OFFSET(0) NUMBITS(10) [],
    ],
    pub RxOverrunsW [
        COUNT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxIpCkErrorsR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        COUNT OFFSET(0) NUMBITS(8) [],
    ],
    pub RxIpCkErrorsW [
        COUNT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxTcpCkErrorsR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        COUNT OFFSET(0) NUMBITS(8) [],
    ],
    pub RxTcpCkErrorsW [
        COUNT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxUdpCkErrorsR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        COUNT OFFSET(0) NUMBITS(8) [],
    ],
    pub RxUdpCkErrorsW [
        COUNT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AutoFlushedPktsR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub AutoFlushedPktsW [
        COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuTimerIncrSubNsecR [
        SUB_NS_INCR_LSB OFFSET(24) NUMBITS(8) [],
        RESERVED0 OFFSET(16) NUMBITS(8) [],
        SUB_NS_INCR OFFSET(0) NUMBITS(16) [],
    ],
    pub TsuTimerIncrSubNsecW [
        SUB_NS_INCR_LSB OFFSET(24) NUMBITS(8) [],
        SUB_NS_INCR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuTimerMsbSecR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        TIMER OFFSET(0) NUMBITS(16) [],
    ],
    pub TsuTimerMsbSecW [
        TIMER OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuStrobeMsbSec [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        STROBE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuStrobeNsec [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        STROBE OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuTimerNsecR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        TIMER OFFSET(0) NUMBITS(30) [],
    ],
    pub TsuTimerNsecW [
        TIMER OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuTimerAdjustR [
        RESERVED0 OFFSET(30) NUMBITS(1) [],
    ],
    pub TsuTimerAdjustW [
        ADD_SUBTRACT OFFSET(31) NUMBITS(1) [],
        INCREMENT_VALUE OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuTimerIncrR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        NUM_INCS OFFSET(16) NUMBITS(8) [],
        ALT_NS_INCR OFFSET(8) NUMBITS(8) [],
        NS_INCREMENT OFFSET(0) NUMBITS(8) [],
    ],
    pub TsuTimerIncrW [
        NUM_INCS OFFSET(16) NUMBITS(8) [],
        ALT_NS_INCR OFFSET(8) NUMBITS(8) [],
        NS_INCREMENT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuPtpTxNsec [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        TIMER OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuPtpRxNsec [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        TIMER OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuPeerTxNsec [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        TIMER OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuPeerRxNsec [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        TIMER OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsControlR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        PCS_SOFTWARE_RESET OFFSET(15) NUMBITS(1) [],
        LOOPBACK_MODE OFFSET(14) NUMBITS(1) [],
        SPEED_SELECT_BIT_1 OFFSET(13) NUMBITS(1) [],
        ENABLE_AUTO_NEG OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(10) NUMBITS(2) [],
        RESTART_AUTO_NEG OFFSET(9) NUMBITS(1) [],
        MAC_DUPLEX_STATE OFFSET(8) NUMBITS(1) [],
        COLLISION_TEST OFFSET(7) NUMBITS(1) [],
        SPEED_SELECT_BIT_0 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(6) [],
    ],
    pub PcsControlW [
        PCS_SOFTWARE_RESET OFFSET(15) NUMBITS(1) [],
        LOOPBACK_MODE OFFSET(14) NUMBITS(1) [],
        ENABLE_AUTO_NEG OFFSET(12) NUMBITS(1) [],
        RESTART_AUTO_NEG OFFSET(9) NUMBITS(1) [],
        COLLISION_TEST OFFSET(7) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsStatus [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        BASE_100_T4 OFFSET(15) NUMBITS(1) [],
        BASE_100_X_FULL_DUPLEX OFFSET(14) NUMBITS(1) [],
        BASE_100_X_HALF_DUPLEX OFFSET(13) NUMBITS(1) [],
        MBPS_10_FULL_DUPLEX OFFSET(12) NUMBITS(1) [],
        MBPS_10_HALF_DUPLEX OFFSET(11) NUMBITS(1) [],
        BASE_100_T2_FULL_DUPLEX OFFSET(10) NUMBITS(1) [],
        BASE_100_T2_HALF_DUPLEX OFFSET(9) NUMBITS(1) [],
        EXTENDED_STATUS OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(2) [],
        AUTO_NEG_COMPLETE OFFSET(5) NUMBITS(1) [],
        REMOTE_FAULT OFFSET(4) NUMBITS(1) [],
        AUTO_NEG_ABILITY OFFSET(3) NUMBITS(1) [],
        LINK_STATUS OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        EXTENDED_CAPABILITIES OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsPhyTopId [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ID_CODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsPhyBotId [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ID_CODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsAnAdvR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        NEXT_PAGE OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(1) [],
        REMOTE_FAULT OFFSET(12) NUMBITS(2) [],
        RESERVED2 OFFSET(9) NUMBITS(3) [],
        PAUSE OFFSET(7) NUMBITS(2) [],
        HALF_DUPLEX OFFSET(6) NUMBITS(1) [],
        FULL_DUPLEX OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(5) [],
    ],
    pub PcsAnAdvW [
        NEXT_PAGE OFFSET(15) NUMBITS(1) [],
        REMOTE_FAULT OFFSET(12) NUMBITS(2) [],
        PAUSE OFFSET(7) NUMBITS(2) [],
        HALF_DUPLEX OFFSET(6) NUMBITS(1) [],
        FULL_DUPLEX OFFSET(5) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsAnLpBase [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        LINK_PARTNER_NEXT_PAGE_STATUS OFFSET(15) NUMBITS(1) [],
        LINK_PARTNER_ACKNOWLEDGE OFFSET(14) NUMBITS(1) [],
        LINK_PARTNER_REMOTE_FAULT_DUPLEX_MODE OFFSET(12) NUMBITS(2) [],
        SPEED_RESERVED OFFSET(9) NUMBITS(3) [],
        PAUSE OFFSET(7) NUMBITS(2) [],
        LINK_PARTNER_HALF_DUPLEX OFFSET(6) NUMBITS(1) [],
        LINK_PARTNER_FULL_DUPLEX OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsAnExp [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        NEXT_PAGE_CAPABILITY OFFSET(2) NUMBITS(1) [],
        PAGE_RECEIVED OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsAnNpTxR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        NEXT_PAGE_TO_TRANSMIT OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(1) [],
        MESSAGE_PAGE_INDICATOR OFFSET(13) NUMBITS(1) [],
        ACKNOWLEDGE_2 OFFSET(12) NUMBITS(1) [],
        RESERVED2 OFFSET(11) NUMBITS(1) [],
        MESSAGE OFFSET(0) NUMBITS(11) [],
    ],
    pub PcsAnNpTxW [
        NEXT_PAGE_TO_TRANSMIT OFFSET(15) NUMBITS(1) [],
        MESSAGE_PAGE_INDICATOR OFFSET(13) NUMBITS(1) [],
        ACKNOWLEDGE_2 OFFSET(12) NUMBITS(1) [],
        MESSAGE OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsAnLpNp [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        NEXT_PAGE_TO_RECEIVE OFFSET(15) NUMBITS(1) [],
        ACKNOWLEDGE OFFSET(14) NUMBITS(1) [],
        MESSAGE_PAGE_INDICATOR OFFSET(13) NUMBITS(1) [],
        ACKNOWLEDGE_2 OFFSET(12) NUMBITS(1) [],
        TOGGLE OFFSET(11) NUMBITS(1) [],
        MESSAGE OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcsAnExtStatus [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        FULL_DUPLEX_1000BASE_X OFFSET(15) NUMBITS(1) [],
        HALF_DUPLEX_1000BASE_X OFFSET(14) NUMBITS(1) [],
        FULL_DUPLEX_1000BASE_T OFFSET(13) NUMBITS(1) [],
        HALF_DUPLEX_1000BASE_T OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxLpiR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub RxLpiW [
        COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxLpiTime [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        LPI_TIME OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxLpiR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub TxLpiW [
        COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxLpiTimeR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        LPI_TIME OFFSET(0) NUMBITS(24) [],
    ],
    pub TxLpiTimeW [
        LPI_TIME OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DesigncfgDebug1 [
        AXI_CACHE_VALUE OFFSET(28) NUMBITS(4) [],
        DMA_BUS_WIDTH OFFSET(25) NUMBITS(3) [],
        RESERVED0 OFFSET(24) NUMBITS(1) [],
        IRQ_READ_CLEAR OFFSET(23) NUMBITS(1) [],
        NO_SNAPSHOT OFFSET(22) NUMBITS(1) [],
        NO_STATS OFFSET(21) NUMBITS(1) [],
        NO_SCAN_PINS OFFSET(20) NUMBITS(1) [],
        USER_IN_WIDTH OFFSET(15) NUMBITS(5) [],
        USER_OUT_WIDTH OFFSET(10) NUMBITS(5) [],
        USER_IO OFFSET(9) NUMBITS(1) [],
        APB_REV2 OFFSET(8) NUMBITS(1) [],
        APB_REV1 OFFSET(7) NUMBITS(1) [],
        EXT_FIFO_INTERFACE OFFSET(6) NUMBITS(1) [],
        NO_INT_LOOPBACK OFFSET(5) NUMBITS(1) [],
        INT_LOOPBACK OFFSET(4) NUMBITS(1) [],
        TDC_50 OFFSET(3) NUMBITS(1) [],
        RDC_50 OFFSET(2) NUMBITS(1) [],
        SERDES OFFSET(1) NUMBITS(1) [],
        NO_PCS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DesigncfgDebug2 [
        SPRAM OFFSET(31) NUMBITS(1) [],
        AXI OFFSET(30) NUMBITS(1) [],
        TX_PBUF_ADDR OFFSET(26) NUMBITS(4) [],
        RX_PBUF_ADDR OFFSET(22) NUMBITS(4) [],
        TX_PKT_BUFFER OFFSET(21) NUMBITS(1) [],
        RX_PKT_BUFFER OFFSET(20) NUMBITS(1) [],
        HPROT_VALUE OFFSET(16) NUMBITS(4) [],
        JUMBO_MAX_LENGTH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DesigncfgDebug3 [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        NUM_SPEC_ADD_FILTERS OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DesigncfgDebug5 [
        AXI_PROT_VALUE OFFSET(29) NUMBITS(3) [],
        TSU_CLK OFFSET(28) NUMBITS(1) [],
        RX_BUFFER_LENGTH_DEF OFFSET(20) NUMBITS(8) [],
        TX_PBUF_SIZE_DEF OFFSET(19) NUMBITS(1) [],
        RX_PBUF_SIZE_DEF OFFSET(17) NUMBITS(2) [],
        ENDIAN_SWAP_DEF OFFSET(15) NUMBITS(2) [],
        MDC_CLOCK_DIV OFFSET(12) NUMBITS(3) [],
        DMA_BUS_WIDTH_DEF OFFSET(10) NUMBITS(2) [],
        PHY_IDENT OFFSET(9) NUMBITS(1) [],
        TSU OFFSET(8) NUMBITS(1) [],
        TX_FIFO_CNT_WIDTH OFFSET(4) NUMBITS(4) [],
        RX_FIFO_CNT_WIDTH OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DesigncfgDebug6 [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        PBUF_CUTTHRU OFFSET(25) NUMBITS(1) [],
        PFC_MULTI_QUANTUM OFFSET(24) NUMBITS(1) [],
        DMA_ADDR_WIDTH_IS_64B OFFSET(23) NUMBITS(1) [],
        HOST_IF_SOFT_SELECT OFFSET(22) NUMBITS(1) [],
        TX_ADD_FIFO_IF OFFSET(21) NUMBITS(1) [],
        EXT_TSU_TIMER OFFSET(20) NUMBITS(1) [],
        TX_PBUF_QUEUE_SEGMENT_SIZE OFFSET(16) NUMBITS(4) [],
        DMA_PRIORITY_QUEUE15 OFFSET(15) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE14 OFFSET(14) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE13 OFFSET(13) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE12 OFFSET(12) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE11 OFFSET(11) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE10 OFFSET(10) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE9 OFFSET(9) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE8 OFFSET(8) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE7 OFFSET(7) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE6 OFFSET(6) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE5 OFFSET(5) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE4 OFFSET(4) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE3 OFFSET(3) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE2 OFFSET(2) NUMBITS(1) [],
        DMA_PRIORITY_QUEUE1 OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DesigncfgDebug7 [
        TX_PBUF_NUM_SEGMENTS_Q7 OFFSET(28) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q6 OFFSET(24) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q5 OFFSET(20) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q4 OFFSET(16) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q3 OFFSET(12) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q2 OFFSET(8) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q1 OFFSET(4) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DesigncfgDebug8 [
        NUM_TYPE1_SCREENERS OFFSET(24) NUMBITS(8) [],
        NUM_TYPE2_SCREENERS OFFSET(16) NUMBITS(8) [],
        NUM_SCR2_ETHTYPE_REGS OFFSET(8) NUMBITS(8) [],
        NUM_SCR2_COMPARE_REGS OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DesigncfgDebug9 [
        TX_PBUF_NUM_SEGMENTS_Q15 OFFSET(28) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q14 OFFSET(24) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q13 OFFSET(20) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q12 OFFSET(16) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q11 OFFSET(12) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q10 OFFSET(8) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q9 OFFSET(4) NUMBITS(4) [],
        TX_PBUF_NUM_SEGMENTS_Q8 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DesigncfgDebug10 [
        EMAC_BUS_WIDTH OFFSET(28) NUMBITS(4) [],
        TX_PBUF_DATA OFFSET(24) NUMBITS(4) [],
        RX_PBUF_DATA OFFSET(20) NUMBITS(4) [],
        AXI_ACCESS_PIPELINE_BITS OFFSET(16) NUMBITS(4) [],
        AXI_TX_DESCR_RD_BUFF_BITS OFFSET(12) NUMBITS(4) [],
        AXI_RX_DESCR_RD_BUFF_BITS OFFSET(8) NUMBITS(4) [],
        AXI_TX_DESCR_WR_BUFF_BITS OFFSET(4) NUMBITS(4) [],
        AXI_RX_DESCR_WR_BUFF_BITS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntQ1StatusR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        RESP_NOT_OK OFFSET(11) NUMBITS(1) [],
        RECEIVE_OVERRUN OFFSET(10) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(2) [],
        TRANSMIT_COMPLETE OFFSET(7) NUMBITS(1) [],
        AMBA_ERROR OFFSET(6) NUMBITS(1) [],
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(3) NUMBITS(2) [],
        RX_USED_BIT_READ OFFSET(2) NUMBITS(1) [],
        RECEIVE_COMPLETE OFFSET(1) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub IntQ1StatusW [
        RESP_NOT_OK OFFSET(11) NUMBITS(1) [],
        RECEIVE_OVERRUN OFFSET(10) NUMBITS(1) [],
        TRANSMIT_COMPLETE OFFSET(7) NUMBITS(1) [],
        AMBA_ERROR OFFSET(6) NUMBITS(1) [],
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION OFFSET(5) NUMBITS(1) [],
        RX_USED_BIT_READ OFFSET(2) NUMBITS(1) [],
        RECEIVE_COMPLETE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TransmitQ1PtrR [
        DMA_TX_Q_PTR OFFSET(2) NUMBITS(30) [],
        RESERVED0 OFFSET(0) NUMBITS(2) [],
    ],
    pub TransmitQ1PtrW [
        DMA_TX_Q_PTR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReceiveQ1PtrR [
        DMA_RX_Q_PTR OFFSET(2) NUMBITS(30) [],
        RESERVED0 OFFSET(0) NUMBITS(2) [],
    ],
    pub ReceiveQ1PtrW [
        DMA_RX_Q_PTR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DmaRxbufSizeQ1R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        DMA_RX_Q_BUF_SIZE OFFSET(0) NUMBITS(8) [],
    ],
    pub DmaRxbufSizeQ1W [
        DMA_RX_Q_BUF_SIZE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CbsControlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        CBS_ENABLE_QUEUE_B OFFSET(1) NUMBITS(1) [],
        CBS_ENABLE_QUEUE_A OFFSET(0) NUMBITS(1) [],
    ],
    pub CbsControlW [
        CBS_ENABLE_QUEUE_B OFFSET(1) NUMBITS(1) [],
        CBS_ENABLE_QUEUE_A OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxBdControlR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        TX_BD_TS_MODE OFFSET(4) NUMBITS(2) [],
        RESERVED1 OFFSET(0) NUMBITS(4) [],
    ],
    pub TxBdControlW [
        TX_BD_TS_MODE OFFSET(4) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxBdControlR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        RX_BD_TS_MODE OFFSET(4) NUMBITS(2) [],
        RESERVED1 OFFSET(0) NUMBITS(4) [],
    ],
    pub RxBdControlW [
        RX_BD_TS_MODE OFFSET(4) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType1Register0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        UDP_PORT_MATCH_ENABLE OFFSET(29) NUMBITS(1) [],
        DSTC_ENABLE OFFSET(28) NUMBITS(1) [],
        UDP_PORT_MATCH OFFSET(12) NUMBITS(16) [],
        DSTC_MATCH OFFSET(4) NUMBITS(8) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ],
    pub ScreeningType1Register0W [
        UDP_PORT_MATCH_ENABLE OFFSET(29) NUMBITS(1) [],
        DSTC_ENABLE OFFSET(28) NUMBITS(1) [],
        UDP_PORT_MATCH OFFSET(12) NUMBITS(16) [],
        DSTC_MATCH OFFSET(4) NUMBITS(8) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType1Register1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        UDP_PORT_MATCH_ENABLE OFFSET(29) NUMBITS(1) [],
        DSTC_ENABLE OFFSET(28) NUMBITS(1) [],
        UDP_PORT_MATCH OFFSET(12) NUMBITS(16) [],
        DSTC_MATCH OFFSET(4) NUMBITS(8) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ],
    pub ScreeningType1Register1W [
        UDP_PORT_MATCH_ENABLE OFFSET(29) NUMBITS(1) [],
        DSTC_ENABLE OFFSET(28) NUMBITS(1) [],
        UDP_PORT_MATCH OFFSET(12) NUMBITS(16) [],
        DSTC_MATCH OFFSET(4) NUMBITS(8) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType1Register2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        UDP_PORT_MATCH_ENABLE OFFSET(29) NUMBITS(1) [],
        DSTC_ENABLE OFFSET(28) NUMBITS(1) [],
        UDP_PORT_MATCH OFFSET(12) NUMBITS(16) [],
        DSTC_MATCH OFFSET(4) NUMBITS(8) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ],
    pub ScreeningType1Register2W [
        UDP_PORT_MATCH_ENABLE OFFSET(29) NUMBITS(1) [],
        DSTC_ENABLE OFFSET(28) NUMBITS(1) [],
        UDP_PORT_MATCH OFFSET(12) NUMBITS(16) [],
        DSTC_MATCH OFFSET(4) NUMBITS(8) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType1Register3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        UDP_PORT_MATCH_ENABLE OFFSET(29) NUMBITS(1) [],
        DSTC_ENABLE OFFSET(28) NUMBITS(1) [],
        UDP_PORT_MATCH OFFSET(12) NUMBITS(16) [],
        DSTC_MATCH OFFSET(4) NUMBITS(8) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ],
    pub ScreeningType1Register3W [
        UDP_PORT_MATCH_ENABLE OFFSET(29) NUMBITS(1) [],
        DSTC_ENABLE OFFSET(28) NUMBITS(1) [],
        UDP_PORT_MATCH OFFSET(12) NUMBITS(16) [],
        DSTC_MATCH OFFSET(4) NUMBITS(8) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType2Register0R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        COMPARE_C_ENABLE OFFSET(30) NUMBITS(1) [],
        COMPARE_C OFFSET(25) NUMBITS(5) [],
        COMPARE_B_ENABLE OFFSET(24) NUMBITS(1) [],
        COMPARE_B OFFSET(19) NUMBITS(5) [],
        COMPARE_A_ENABLE OFFSET(18) NUMBITS(1) [],
        COMPARE_A OFFSET(13) NUMBITS(5) [],
        ETHERTYPE_ENABLE OFFSET(12) NUMBITS(1) [],
        INDEX OFFSET(9) NUMBITS(3) [],
        VLAN_ENABLE OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        VLAN_PRIORITY OFFSET(4) NUMBITS(3) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ],
    pub ScreeningType2Register0W [
        COMPARE_C_ENABLE OFFSET(30) NUMBITS(1) [],
        COMPARE_C OFFSET(25) NUMBITS(5) [],
        COMPARE_B_ENABLE OFFSET(24) NUMBITS(1) [],
        COMPARE_B OFFSET(19) NUMBITS(5) [],
        COMPARE_A_ENABLE OFFSET(18) NUMBITS(1) [],
        COMPARE_A OFFSET(13) NUMBITS(5) [],
        ETHERTYPE_ENABLE OFFSET(12) NUMBITS(1) [],
        INDEX OFFSET(9) NUMBITS(3) [],
        VLAN_ENABLE OFFSET(8) NUMBITS(1) [],
        VLAN_PRIORITY OFFSET(4) NUMBITS(3) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType2Register1R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        COMPARE_C_ENABLE OFFSET(30) NUMBITS(1) [],
        COMPARE_C OFFSET(25) NUMBITS(5) [],
        COMPARE_B_ENABLE OFFSET(24) NUMBITS(1) [],
        COMPARE_B OFFSET(19) NUMBITS(5) [],
        COMPARE_A_ENABLE OFFSET(18) NUMBITS(1) [],
        COMPARE_A OFFSET(13) NUMBITS(5) [],
        ETHERTYPE_ENABLE OFFSET(12) NUMBITS(1) [],
        INDEX OFFSET(9) NUMBITS(3) [],
        VLAN_ENABLE OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        VLAN_PRIORITY OFFSET(4) NUMBITS(3) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ],
    pub ScreeningType2Register1W [
        COMPARE_C_ENABLE OFFSET(30) NUMBITS(1) [],
        COMPARE_C OFFSET(25) NUMBITS(5) [],
        COMPARE_B_ENABLE OFFSET(24) NUMBITS(1) [],
        COMPARE_B OFFSET(19) NUMBITS(5) [],
        COMPARE_A_ENABLE OFFSET(18) NUMBITS(1) [],
        COMPARE_A OFFSET(13) NUMBITS(5) [],
        ETHERTYPE_ENABLE OFFSET(12) NUMBITS(1) [],
        INDEX OFFSET(9) NUMBITS(3) [],
        VLAN_ENABLE OFFSET(8) NUMBITS(1) [],
        VLAN_PRIORITY OFFSET(4) NUMBITS(3) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType2Register2R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        COMPARE_C_ENABLE OFFSET(30) NUMBITS(1) [],
        COMPARE_C OFFSET(25) NUMBITS(5) [],
        COMPARE_B_ENABLE OFFSET(24) NUMBITS(1) [],
        COMPARE_B OFFSET(19) NUMBITS(5) [],
        COMPARE_A_ENABLE OFFSET(18) NUMBITS(1) [],
        COMPARE_A OFFSET(13) NUMBITS(5) [],
        ETHERTYPE_ENABLE OFFSET(12) NUMBITS(1) [],
        INDEX OFFSET(9) NUMBITS(3) [],
        VLAN_ENABLE OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        VLAN_PRIORITY OFFSET(4) NUMBITS(3) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ],
    pub ScreeningType2Register2W [
        COMPARE_C_ENABLE OFFSET(30) NUMBITS(1) [],
        COMPARE_C OFFSET(25) NUMBITS(5) [],
        COMPARE_B_ENABLE OFFSET(24) NUMBITS(1) [],
        COMPARE_B OFFSET(19) NUMBITS(5) [],
        COMPARE_A_ENABLE OFFSET(18) NUMBITS(1) [],
        COMPARE_A OFFSET(13) NUMBITS(5) [],
        ETHERTYPE_ENABLE OFFSET(12) NUMBITS(1) [],
        INDEX OFFSET(9) NUMBITS(3) [],
        VLAN_ENABLE OFFSET(8) NUMBITS(1) [],
        VLAN_PRIORITY OFFSET(4) NUMBITS(3) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType2Register3R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        COMPARE_C_ENABLE OFFSET(30) NUMBITS(1) [],
        COMPARE_C OFFSET(25) NUMBITS(5) [],
        COMPARE_B_ENABLE OFFSET(24) NUMBITS(1) [],
        COMPARE_B OFFSET(19) NUMBITS(5) [],
        COMPARE_A_ENABLE OFFSET(18) NUMBITS(1) [],
        COMPARE_A OFFSET(13) NUMBITS(5) [],
        ETHERTYPE_ENABLE OFFSET(12) NUMBITS(1) [],
        INDEX OFFSET(9) NUMBITS(3) [],
        VLAN_ENABLE OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        VLAN_PRIORITY OFFSET(4) NUMBITS(3) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ],
    pub ScreeningType2Register3W [
        COMPARE_C_ENABLE OFFSET(30) NUMBITS(1) [],
        COMPARE_C OFFSET(25) NUMBITS(5) [],
        COMPARE_B_ENABLE OFFSET(24) NUMBITS(1) [],
        COMPARE_B OFFSET(19) NUMBITS(5) [],
        COMPARE_A_ENABLE OFFSET(18) NUMBITS(1) [],
        COMPARE_A OFFSET(13) NUMBITS(5) [],
        ETHERTYPE_ENABLE OFFSET(12) NUMBITS(1) [],
        INDEX OFFSET(9) NUMBITS(3) [],
        VLAN_ENABLE OFFSET(8) NUMBITS(1) [],
        VLAN_PRIORITY OFFSET(4) NUMBITS(3) [],
        QUEUE_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntQ1EnableR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        RESERVED1 OFFSET(8) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub IntQ1EnableW [
        ENABLE_RESP_NOT_OK_INTERRUPT OFFSET(11) NUMBITS(1) [],
        ENABLE_RECEIVE_OVERRUN_INTERRUPT OFFSET(10) NUMBITS(1) [],
        ENABLE_TRANSMIT_COMPLETE_INTERRUPT OFFSET(7) NUMBITS(1) [],
        ENABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT OFFSET(6) NUMBITS(1) [],
        ENABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT OFFSET(5) NUMBITS(1) [],
        ENABLE_RX_USED_BIT_READ_INTERRUPT OFFSET(2) NUMBITS(1) [],
        ENABLE_RECEIVE_COMPLETE_INTERRUPT OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntQ1DisableR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        RESERVED1 OFFSET(8) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub IntQ1DisableW [
        DISABLE_RESP_NOT_OK_INTERRUPT OFFSET(11) NUMBITS(1) [],
        DISABLE_RECEIVE_OVERRUN_INTERRUPT OFFSET(10) NUMBITS(1) [],
        DISABLE_TRANSMIT_COMPLETE_INTERRUPT OFFSET(7) NUMBITS(1) [],
        DISABLE_TRANSMIT_FRAME_CORRUPTION_DUE_TO_AMBA_ERROR_INTERRUPT OFFSET(6) NUMBITS(1) [],
        DISABLE_RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT OFFSET(5) NUMBITS(1) [],
        DISABLE_RX_USED_BIT_READ_INTERRUPT OFFSET(2) NUMBITS(1) [],
        DISABLE_RECEIVE_COMPLETE_INTERRUPT OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntQ1Mask [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        RESP_NOT_OK_INTERRUPT_MASK OFFSET(11) NUMBITS(1) [],
        RECEIVE_OVERRUN_INTERRUPT_MASK OFFSET(10) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(2) [],
        TRANSMIT_COMPLETE_INTERRUPT_MASK OFFSET(7) NUMBITS(1) [],
        AMBA_ERROR_INTERRUPT_MASK OFFSET(6) NUMBITS(1) [],
        RETRY_LIMIT_EXCEEDED_OR_LATE_COLLISION_INTERRUPT_MASK OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(3) NUMBITS(2) [],
        RX_USED_INTERRUPT_MASK OFFSET(2) NUMBITS(1) [],
        RECEIVE_COMPLETE_INTERRUPT_MASK OFFSET(1) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType2EthertypeReg0R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COMPARE_VALUE OFFSET(0) NUMBITS(16) [],
    ],
    pub ScreeningType2EthertypeReg0W [
        COMPARE_VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType2EthertypeReg1R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COMPARE_VALUE OFFSET(0) NUMBITS(16) [],
    ],
    pub ScreeningType2EthertypeReg1W [
        COMPARE_VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType2EthertypeReg2R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COMPARE_VALUE OFFSET(0) NUMBITS(16) [],
    ],
    pub ScreeningType2EthertypeReg2W [
        COMPARE_VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ScreeningType2EthertypeReg3R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COMPARE_VALUE OFFSET(0) NUMBITS(16) [],
    ],
    pub ScreeningType2EthertypeReg3W [
        COMPARE_VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Type2Compare0Word0 [
        COMPARE_VALUE OFFSET(16) NUMBITS(16) [],
        MASK_VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Type2Compare0Word1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        COMPARE_OFFSET OFFSET(7) NUMBITS(2) [],
        OFFSET_VALUE OFFSET(0) NUMBITS(7) [],
    ],
    pub Type2Compare0Word1W [
        COMPARE_OFFSET OFFSET(7) NUMBITS(2) [],
        OFFSET_VALUE OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Type2Compare1Word0 [
        COMPARE_VALUE OFFSET(16) NUMBITS(16) [],
        MASK_VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Type2Compare1Word1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        COMPARE_OFFSET OFFSET(7) NUMBITS(2) [],
        OFFSET_VALUE OFFSET(0) NUMBITS(7) [],
    ],
    pub Type2Compare1Word1W [
        COMPARE_OFFSET OFFSET(7) NUMBITS(2) [],
        OFFSET_VALUE OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Type2Compare2Word0 [
        COMPARE_VALUE OFFSET(16) NUMBITS(16) [],
        MASK_VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Type2Compare2Word1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        COMPARE_OFFSET OFFSET(7) NUMBITS(2) [],
        OFFSET_VALUE OFFSET(0) NUMBITS(7) [],
    ],
    pub Type2Compare2Word1W [
        COMPARE_OFFSET OFFSET(7) NUMBITS(2) [],
        OFFSET_VALUE OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Type2Compare3Word0 [
        COMPARE_VALUE OFFSET(16) NUMBITS(16) [],
        MASK_VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Type2Compare3Word1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        COMPARE_OFFSET OFFSET(7) NUMBITS(2) [],
        OFFSET_VALUE OFFSET(0) NUMBITS(7) [],
    ],
    pub Type2Compare3Word1W [
        COMPARE_OFFSET OFFSET(7) NUMBITS(2) [],
        OFFSET_VALUE OFFSET(0) NUMBITS(7) [],
    ]
];

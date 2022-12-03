// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// DisplayPort Controller, DisplayPort Controller
pub static mut DISPLAY_PORT: *mut Registers = 0xfd4a0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Sets the value of the main link bandwidth for the sink device.
    pub dp_link_bw_set: Aliased<u32, DpLinkBwSetR::Register, DpLinkBwSetW::Register>,
    /// To set the lane count
    pub dp_lane_count_set: Aliased<u32, DpLaneCountSetR::Register, DpLaneCountSetW::Register>,
    /// To enable enhanced framing
    pub dp_enhanced_frame_en:
        Aliased<u32, DpEnhancedFrameEnR::Register, DpEnhancedFrameEnW::Register>,
    /// To force training pattern
    pub dp_training_pattern_set:
        Aliased<u32, DpTrainingPatternSetR::Register, DpTrainingPatternSetW::Register>,
    /// To transmit the link quality pattern
    pub dp_link_qual_pattern_set:
        Aliased<u32, DpLinkQualPatternSetR::Register, DpLinkQualPatternSetW::Register>,
    /// DP_SCRAMBLING_DISABLE
    pub dp_scrambling_disable:
        Aliased<u32, DpScramblingDisableR::Register, DpScramblingDisableW::Register>,
    /// For down-spreading control
    pub dp_downspread_ctrl: Aliased<u32, DpDownspreadCtrlR::Register, DpDownspreadCtrlW::Register>,
    /// Soft reset of DP Core
    pub dp_software_reset: Aliased<u32, DpSoftwareResetR::Register, DpSoftwareResetW::Register>,
    /// 32 bits of 80-bit custom pattern that is used for LINK quality test. These bits are valid when Bit 2 of DP_LINK_QUAL_PATTERN_SET 0x10 register is set to 1
    pub dp_comp_pattern_80bit_1: ReadWrite<u32>,
    /// Description same as DP_COMP_PATTERN_80BIT_1 (0x20)
    pub dp_comp_pattern_80bit_2: ReadWrite<u32>,
    /// Description same as DP_COMP_PATTERN_80BIT_1 (0x20)
    pub dp_comp_pattern_80bit_3:
        Aliased<u32, DpCompPattern80bit3R::Register, DpCompPattern80bit3W::Register>,
    _padding44: [u8; 84],
    /// Enable the basic operations of the transmitter.
    pub dp_transmitter_enable:
        Aliased<u32, DpTransmitterEnableR::Register, DpTransmitterEnableW::Register>,
    /// Enable the transmission of main link video information.
    pub dp_main_stream_enable:
        Aliased<u32, DpMainStreamEnableR::Register, DpMainStreamEnableW::Register>,
    _padding136: [u8; 56],
    /// Reads from this register always return 0x0.
    pub dp_force_scrambler_reset:
        Aliased<u32, DpForceScramblerResetR::Register, DpForceScramblerResetW::Register>,
    _padding196: [u8; 52],
    /// Core version register
    pub dp_version: ReadOnly<u32>,
    /// Returns the unique identification code of the core and the current revision level
    pub dp_core_id: ReadOnly<u32>,
    /// DP_AUX_COMMAND_REGISTER
    pub dp_aux_command: Aliased<u32, DpAuxCommandR::Register, DpAuxCommandW::Register>,
    /// . FIFO containing up to 16 bytes of write data for the current AUX channel command
    pub dp_aux_write_fifo: Aliased<u32, DpAuxWriteFifoR::Register, DpAuxWriteFifoW::Register>,
    /// Specifies the address for the current AUX channel command.
    pub dp_aux_address: Aliased<u32, DpAuxAddressR::Register, DpAuxAddressW::Register>,
    /// . Contains the clock divider value for generating the internal 1MHz clock from the APB host interface clock. The clock divider register provides integer division only and does not support fractional APB clock rates (for example, set to 75 for a 75 MHz APB clock).
    pub dp_aux_clock_divider:
        Aliased<u32, DpAuxClockDividerR::Register, DpAuxClockDividerW::Register>,
    /// . Indicates an overflow in the user FIFO. The event may occur if the video rate does not match the TU size programming.
    pub dp_tx_user_fifo_overflow: ReadOnly<u32, DpTxUserFifoOverflow::Register>,
    _padding276: [u8; 28],
    /// Contains the raw signal values for those conditions which may cause an interrupt.
    pub dp_interrupt_signal_state: ReadOnly<u32, DpInterruptSignalState::Register>,
    /// Maps to the internal FIFO which contains up to 16 bytes of information received during the AUX channel reply. Reply data is read from the FIFO starting with byte 0. The number of bytes in the FIFO corresponds to the number of bytes requested.
    pub dp_aux_reply_data: ReadOnly<u32, DpAuxReplyData::Register>,
    /// Reply code received from the most recent AUX Channel request. The AUX Reply Code corresponds to the code from the DisplayPort specification
    pub dp_aux_reply_code: ReadOnly<u32, DpAuxReplyCode::Register>,
    /// Provides an internal counter of the number of AUX reply transactions received on the AUX Channel. Writing to this register clears the count.
    pub dp_aux_reply_count: Aliased<u32, DpAuxReplyCountR::Register, DpAuxReplyCountW::Register>,
    _padding320: [u8; 8],
    /// Returns the total number of data bytes actually received during a transaction. This register does not use the length byte of the transaction header.
    pub dp_reply_data_count: Aliased<u32, DpReplyDataCountR::Register, DpReplyDataCountW::Register>,
    /// DP_REPLY_STATUS
    pub dp_reply_status: ReadOnly<u32, DpReplyStatus::Register>,
    /// DP_HPD_DURATION
    pub dp_hpd_duration: ReadOnly<u32, DpHpdDuration::Register>,
    _padding340: [u8; 44],
    /// Specifies the total number of clocks in the horizontal framing period for the main stream video signal.
    pub dp_main_stream_htotal:
        Aliased<u32, DpMainStreamHtotalR::Register, DpMainStreamHtotalW::Register>,
    /// Provides the total number of lines in the main stream video frame
    pub dp_main_stream_vtotal:
        Aliased<u32, DpMainStreamVtotalR::Register, DpMainStreamVtotalW::Register>,
    /// Provides the polarity values for the video sync signals
    pub dp_main_stream_polarity:
        Aliased<u32, DpMainStreamPolarityR::Register, DpMainStreamPolarityW::Register>,
    /// Sets the width of the horizontal sync pulse.
    pub dp_main_stream_hswidth:
        Aliased<u32, DpMainStreamHswidthR::Register, DpMainStreamHswidthW::Register>,
    /// Sets the width of the vertical sync pulse.
    pub dp_main_stream_vswidth:
        Aliased<u32, DpMainStreamVswidthR::Register, DpMainStreamVswidthW::Register>,
    /// Horizontal resolution of the main stream video source
    pub dp_main_stream_hres: Aliased<u32, DpMainStreamHresR::Register, DpMainStreamHresW::Register>,
    /// Vertical resolution of the main stream video source
    pub dp_main_stream_vres: Aliased<u32, DpMainStreamVresR::Register, DpMainStreamVresW::Register>,
    /// Number of clocks between the leading edge of the horizontal sync and the start of active data
    pub dp_main_stream_hstart:
        Aliased<u32, DpMainStreamHstartR::Register, DpMainStreamHstartW::Register>,
    /// Number of lines between the leading edge of the vertical sync and the first line of active data.
    pub dp_main_stream_vstart:
        Aliased<u32, DpMainStreamVstartR::Register, DpMainStreamVstartW::Register>,
    /// Miscellaneous stream attributes.Implements the attribute information contained in the DisplayPort MISC0 register described in section 2.2.4 of the standard.
    pub dp_main_stream_misc0:
        Aliased<u32, DpMainStreamMisc0R::Register, DpMainStreamMisc0W::Register>,
    /// MAIN_STREAM_MISC1. Miscellaneous stream attributes.Implements the attribute information contained in the DisplayPort MISC1 register described in section 2.2.4 of the standard.
    pub dp_main_stream_misc1:
        Aliased<u32, DpMainStreamMisc1R::Register, DpMainStreamMisc1W::Register>,
    /// M value for the video stream as computed by the source core. If synchronous clocking mode is used, this register must be written with the M value.
    pub dp_main_stream_m_vid:
        Aliased<u32, DpMainStreamMVidR::Register, DpMainStreamMVidW::Register>,
    /// Sets the size of a transfer unit in the framing logic On reset, transfer size is set to 64.
    pub dp_msa_transfer_unit_size:
        Aliased<u32, DpMsaTransferUnitSizeR::Register, DpMsaTransferUnitSizeW::Register>,
    /// N value for the video stream as computed by the source core. If synchronous clocking mode is used, this register must be written with the N value.
    pub dp_main_stream_n_vid:
        Aliased<u32, DpMainStreamNVidR::Register, DpMainStreamNVidW::Register>,
    /// User pixel width size
    pub dp_user_pix_width: ReadOnly<u32, DpUserPixWidth::Register>,
    /// This register is used to translate the number of pixels per line to the native internal 16-bit datapath.
    pub dp_user_data_count_per_lane:
        Aliased<u32, DpUserDataCountPerLaneR::Register, DpUserDataCountPerLaneW::Register>,
    _padding448: [u8; 4],
    /// Programs source to use MIN number of bytes per transfer unit. The calculation should be done based on the DisplayPort specification.
    pub dp_min_bytes_per_tu: Aliased<u32, DpMinBytesPerTuR::Register, DpMinBytesPerTuW::Register>,
    /// Calculating MIN bytes per TU will often not be a whole number.This register is used to hold the fractional component
    pub dp_frac_bytes_per_tu:
        Aliased<u32, DpFracBytesPerTuR::Register, DpFracBytesPerTuW::Register>,
    /// This register defines the number of initial wait cycles at the start of a new line by the Framing logic. This allows enough data to be buffered in the input FIFO.
    pub dp_init_wait: Aliased<u32, DpInitWaitR::Register, DpInitWaitW::Register>,
    _padding464: [u8; 48],
    /// Reset the transmitter PHY.
    pub dp_phy_reset: Aliased<u32, DpPhyResetR::Register, DpPhyResetW::Register>,
    _padding516: [u8; 44],
    /// Enable the pseudo random bit sequence 7 pattern transmission for link quality assessment. PRBS is generated by the DP transmit controller only. PRBS feature of Cadence GT is unused
    pub dp_transmit_prbs7: Aliased<u32, DpTransmitPrbs7R::Register, DpTransmitPrbs7W::Register>,
    /// Instructs the PHY PLL to generate the proper clock frequency for the required link rate
    pub dp_phy_clock_select: Aliased<u32, DpPhyClockSelectR::Register, DpPhyClockSelectW::Register>,
    /// Control PHY Power down
    pub dp_tx_phy_power_down:
        Aliased<u32, DpTxPhyPowerDownR::Register, DpTxPhyPowerDownW::Register>,
    _padding572: [u8; 16],
    /// Set the pre-cursor level(post cursor 1for cadence GT) for lane 0 of the DisplayPort link
    pub dp_phy_precursor_lane_0:
        Aliased<u32, DpPhyPrecursorLane0R::Register, DpPhyPrecursorLane0W::Register>,
    /// Set the pre-cursor level(post cursor 1 for Cadence GT) for lane 1 of the DisplayPort link
    pub dp_phy_precursor_lane_1:
        Aliased<u32, DpPhyPrecursorLane1R::Register, DpPhyPrecursorLane1W::Register>,
    _padding596: [u8; 44],
    /// Provides the current status from the PHY.
    pub dp_phy_status: ReadOnly<u32, DpPhyStatus::Register>,
    _padding644: [u8; 124],
    /// Enables audio stream packets in main link and provides buffer control.
    pub dp_tx_audio_control: Aliased<u32, DpTxAudioControlR::Register, DpTxAudioControlW::Register>,
    /// TX_AUDIO_CHANNELS. Used to input active channel count. Transmitter collects audio samples based on this information.
    pub dp_tx_audio_channels: ReadWrite<u32, DpTxAudioChannels::Register>,
    /// Word formatted as per CEA 861-C Info Frame.
    pub dp_tx_audio_info_data0: WriteOnly<u32>,
    /// Word formatted as per CEA 861-C Info Frame.
    pub dp_tx_audio_info_data1: WriteOnly<u32>,
    /// Word formatted as per CEA 861-C Info Frame.
    pub dp_tx_audio_info_data2: WriteOnly<u32>,
    /// Word formatted as per CEA 861-C Info Frame.
    pub dp_tx_audio_info_data3: WriteOnly<u32>,
    /// Word formatted as per CEA 861-C Info Frame.
    pub dp_tx_audio_info_data4: WriteOnly<u32>,
    /// Word formatted as per CEA 861-C Info Frame.
    pub dp_tx_audio_info_data5: WriteOnly<u32>,
    /// Word formatted as per CEA 861-C Info Frame.
    pub dp_tx_audio_info_data6: WriteOnly<u32>,
    /// Word formatted as per CEA 861-C Info Frame.
    pub dp_tx_audio_info_data7: WriteOnly<u32>,
    /// M value of audio stream as computed by transmitter
    pub dp_tx_m_aud: Aliased<u32, DpTxMAudR::Register, DpTxMAudW::Register>,
    /// TX_AUDIO_NAUD. N value of audio stream as computed by transmitter.
    pub dp_tx_n_aud: Aliased<u32, DpTxNAudR::Register, DpTxNAudW::Register>,
    /// Word formatted as per Extension packet described in protocol specification. Extended packet is fixed to 32 Bytes length. The controller has buffer space for only one extended packet.
    pub dp_tx_audio_ext_data0: WriteOnly<u32>,
    /// 2nd word of the 9 words of the extended packet
    pub dp_tx_audio_ext_data1: WriteOnly<u32>,
    /// 3rd word of the 9 words of the extended packet
    pub dp_tx_audio_ext_data2: WriteOnly<u32>,
    /// 4th word of the 9 words of the extended packet
    pub dp_tx_audio_ext_data3: WriteOnly<u32>,
    /// 5th word of the 9 words of the extended packet
    pub dp_tx_audio_ext_data4: WriteOnly<u32>,
    /// 6th word of the 9 words of the extended packet
    pub dp_tx_audio_ext_data5: WriteOnly<u32>,
    /// 7th word of the 9 words of the extended packet
    pub dp_tx_audio_ext_data6: WriteOnly<u32>,
    /// 8th word of the 9 words of the extended packet
    pub dp_tx_audio_ext_data7: WriteOnly<u32>,
    /// 9th word of the 9 words of the extended packet
    pub dp_tx_audio_ext_data8: WriteOnly<u32>,
    _padding852: [u8; 76],
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub dp_int_status: Aliased<u32, DpIntStatusR::Register, DpIntStatusW::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub dp_int_mask: ReadOnly<u32, DpIntMask::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub dp_int_en: Aliased<u32, DpIntEnR::Register, DpIntEnW::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub dp_int_ds: Aliased<u32, DpIntDsR::Register, DpIntDsW::Register>,
    _padding944: [u8; 40016],
    /// V_BLEND_BG_CLR_0: Sets background color of the layers
    pub v_blend_bg_clr_0: Aliased<u32, VBlendBgClr0R::Register, VBlendBgClr0W::Register>,
    /// V_BLEND_BG_CLR_1: Sets background color of the layers
    pub v_blend_bg_clr_1: Aliased<u32, VBlendBgClr1R::Register, VBlendBgClr1W::Register>,
    /// V_BLEND_BG_CLR_2: Sets background color of the layers.
    pub v_blend_bg_clr_2: Aliased<u32, VBlendBgClr2R::Register, VBlendBgClr2W::Register>,
    /// To set the global alpha
    pub v_blend_set_global_alpha_reg:
        Aliased<u32, VBlendSetGlobalAlphaRegR::Register, VBlendSetGlobalAlphaRegW::Register>,
    _padding40976: [u8; 4],
    /// V_BLEND_OUTPUT_VID_FORMAT:
    pub v_blend_output_vid_format:
        Aliased<u32, VBlendOutputVidFormatR::Register, VBlendOutputVidFormatW::Register>,
    /// V_BLEND_LAYER0_CONTROL:Layer 0 is always video pixel
    pub v_blend_layer0_control:
        Aliased<u32, VBlendLayer0ControlR::Register, VBlendLayer0ControlW::Register>,
    /// V_BLEND_LAYER1_CONTROL:Layer 1 is always Graphcis
    pub v_blend_layer1_control:
        Aliased<u32, VBlendLayer1ControlR::Register, VBlendLayer1ControlW::Register>,
    /// V_BLEND_RGB2YCBCR_COEFF0:Coefficient values from Matrix for output color space convertor. A total of 9 values are needed to form 3x3 matrix. The value is scaled by 2^12 and stored in 15-bit signed format.(1bit reserved). 12Bits out of the 15 represent fractional value and 2 bits for decimal value and one signed bit.The order of programming values is from v0 - v8
    pub v_blend_rgb2ycbcr_coeff0:
        Aliased<u32, VBlendRgb2ycbcrCoeff0R::Register, VBlendRgb2ycbcrCoeff0W::Register>,
    /// V_BLEND_RGB2YCBCR_COEFF1:Description same as V_BLEND_RGB2YCBCR_COEFF0
    pub v_blend_rgb2ycbcr_coeff1:
        Aliased<u32, VBlendRgb2ycbcrCoeff1R::Register, VBlendRgb2ycbcrCoeff1W::Register>,
    /// V_BLEND_RGB2YCBCR_COEFF2:Description same as V_BLEND_RGB2YCBCR_COEFF0
    pub v_blend_rgb2ycbcr_coeff2:
        Aliased<u32, VBlendRgb2ycbcrCoeff2R::Register, VBlendRgb2ycbcrCoeff2W::Register>,
    /// V_BLEND_RGB2YCBCR_COEFF3:Description same as V_BLEND_RGB2YCBCR_COEFF0
    pub v_blend_rgb2ycbcr_coeff3:
        Aliased<u32, VBlendRgb2ycbcrCoeff3R::Register, VBlendRgb2ycbcrCoeff3W::Register>,
    /// V_BLEND_RGB2YCBCR_COEFF4:Description same as V_BLEND_RGB2YCBCR_COEFF0
    pub v_blend_rgb2ycbcr_coeff4:
        Aliased<u32, VBlendRgb2ycbcrCoeff4R::Register, VBlendRgb2ycbcrCoeff4W::Register>,
    /// V_BLEND_RGB2YCBCR_COEFF5:Description same as V_BLEND_RGB2YCBCR_COEFF0
    pub v_blend_rgb2ycbcr_coeff5:
        Aliased<u32, VBlendRgb2ycbcrCoeff5R::Register, VBlendRgb2ycbcrCoeff5W::Register>,
    /// V_BLEND_RGB2YCBCR_COEFF6:Description same as V_BLEND_RGB2YCBCR_COEFF0
    pub v_blend_rgb2ycbcr_coeff6:
        Aliased<u32, VBlendRgb2ycbcrCoeff6R::Register, VBlendRgb2ycbcrCoeff6W::Register>,
    /// V_BLEND_RGB2YCBCR_COEFF7:Description same as V_BLEND_RGB2YCBCR_COEFF0
    pub v_blend_rgb2ycbcr_coeff7:
        Aliased<u32, VBlendRgb2ycbcrCoeff7R::Register, VBlendRgb2ycbcrCoeff7W::Register>,
    /// V_BLEND_RGB2YCBCR_COEFF8:Description same as V_BLEND_RGB2YCBCR_COEFF0
    pub v_blend_rgb2ycbcr_coeff8:
        Aliased<u32, VBlendRgb2ycbcrCoeff8R::Register, VBlendRgb2ycbcrCoeff8W::Register>,
    /// V_BLEND_IN1CSC_COEFF0:Coefficient values from Matrix for input color space convertor(video). A total of 9 values are needed to form 3x3 matrix. The value is scaled by 2^12 and stored in 15-bit signed format.(1bit reserved). 12Bits out of the 15 represent fractional value and 2 bits for decimal value and one signed bit.The order of programming values is from v0 - v8
    pub v_blend_in1csc_coeff0:
        Aliased<u32, VBlendIn1cscCoeff0R::Register, VBlendIn1cscCoeff0W::Register>,
    /// V_BLEND_IN1CSC_COEFF1:Description same as V_BLEND_IN1CSC_COEFF0
    pub v_blend_in1csc_coeff1:
        Aliased<u32, VBlendIn1cscCoeff1R::Register, VBlendIn1cscCoeff1W::Register>,
    /// V_BLEND_IN1CSC_COEFF2:Description same as V_BLEND_IN1CSC_COEFF0
    pub v_blend_in1csc_coeff2:
        Aliased<u32, VBlendIn1cscCoeff2R::Register, VBlendIn1cscCoeff2W::Register>,
    /// V_BLEND_IN1CSC_COEFF3:Description same as V_BLEND_IN1CSC_COEFF0
    pub v_blend_in1csc_coeff3:
        Aliased<u32, VBlendIn1cscCoeff3R::Register, VBlendIn1cscCoeff3W::Register>,
    /// V_BLEND_IN1CSC_COEFF4:Description same as V_BLEND_IN1CSC_COEFF0
    pub v_blend_in1csc_coeff4:
        Aliased<u32, VBlendIn1cscCoeff4R::Register, VBlendIn1cscCoeff4W::Register>,
    /// V_BLEND_IN1CSC_COEFF5:Description same as V_BLEND_IN1CSC_COEFF0
    pub v_blend_in1csc_coeff5:
        Aliased<u32, VBlendIn1cscCoeff5R::Register, VBlendIn1cscCoeff5W::Register>,
    /// V_BLEND_IN1CSC_COEFF6:Description same as V_BLEND_IN1CSC_COEFF0
    pub v_blend_in1csc_coeff6:
        Aliased<u32, VBlendIn1cscCoeff6R::Register, VBlendIn1cscCoeff6W::Register>,
    /// V_BLEND_IN1CSC_COEFF7:CDescription same as V_BLEND_IN1CSC_COEFF0
    pub v_blend_in1csc_coeff7:
        Aliased<u32, VBlendIn1cscCoeff7R::Register, VBlendIn1cscCoeff7W::Register>,
    /// V_BLEND_IN1CSC_COEFF8:Description same as V_BLEND_IN1CSC_COEFF0
    pub v_blend_in1csc_coeff8:
        Aliased<u32, VBlendIn1cscCoeff8R::Register, VBlendIn1cscCoeff8W::Register>,
    /// V_BLEND_LUMA_IN1CSC_OFFSET:Offset values for Y before and after matrix multiplication for input color space conversion
    pub v_blend_luma_in1csc_offset:
        Aliased<u32, VBlendLumaIn1cscOffsetR::Register, VBlendLumaIn1cscOffsetW::Register>,
    /// V_BLEND_CR_IN1CSC_OFFSET:Offset values for CR before and after matrix multiplication for input color space conversion
    pub v_blend_cr_in1csc_offset:
        Aliased<u32, VBlendCrIn1cscOffsetR::Register, VBlendCrIn1cscOffsetW::Register>,
    /// V_BLEND_CB_IN1CSC_OFFSET:Offset values for CB before and after matrix multiplication for input color space conversion
    pub v_blend_cb_in1csc_offset:
        Aliased<u32, VBlendCbIn1cscOffsetR::Register, VBlendCbIn1cscOffsetW::Register>,
    /// V_BLEND_LUMA_OUTCSC_OFFSET:Offset values for Y before and after matrix multiplication for output color space conversion
    pub v_blend_luma_outcsc_offset:
        Aliased<u32, VBlendLumaOutcscOffsetR::Register, VBlendLumaOutcscOffsetW::Register>,
    /// V_BLEND_CR_OUTCSC_OFFSET:Offset values for CR before and after matrix multiplication for output color space conversion
    pub v_blend_cr_outcsc_offset:
        Aliased<u32, VBlendCrOutcscOffsetR::Register, VBlendCrOutcscOffsetW::Register>,
    /// V_BLEND_CB_OUTCSC_OFFSET:Offset values for color CB before and after matrix multiplication for output color space conversion
    pub v_blend_cb_outcsc_offset:
        Aliased<u32, VBlendCbOutcscOffsetR::Register, VBlendCbOutcscOffsetW::Register>,
    /// V_BLEND_IN2CSC_COEFF0:Coefficient values from Matrix for input color space convertor(graphics). A total of 9 values are needed to form 3x3 matrix. The value is scaled by 2^12 and stored in 15-bit signed format.(1bit reserved). 12Bits out of the 15 represent fractional value and 2 bits for decimal value and one signed bit.The order of programming values is from v0 - v8
    pub v_blend_in2csc_coeff0:
        Aliased<u32, VBlendIn2cscCoeff0R::Register, VBlendIn2cscCoeff0W::Register>,
    /// V_BLEND_IN2CSC_COEFF1:Description same as V_BLEND_IN2CSC_COEFF0
    pub v_blend_in2csc_coeff1:
        Aliased<u32, VBlendIn2cscCoeff1R::Register, VBlendIn2cscCoeff1W::Register>,
    /// V_BLEND_IN2CSC_COEFF2:Description same as V_BLEND_IN2CSC_COEFF0
    pub v_blend_in2csc_coeff2:
        Aliased<u32, VBlendIn2cscCoeff2R::Register, VBlendIn2cscCoeff2W::Register>,
    /// V_BLEND_IN2CSC_COEFF3:Description same as V_BLEND_IN2CSC_COEFF0
    pub v_blend_in2csc_coeff3:
        Aliased<u32, VBlendIn2cscCoeff3R::Register, VBlendIn2cscCoeff3W::Register>,
    /// V_BLEND_IN2CSC_COEFF4:Description same as V_BLEND_IN2CSC_COEFF0
    pub v_blend_in2csc_coeff4:
        Aliased<u32, VBlendIn2cscCoeff4R::Register, VBlendIn2cscCoeff4W::Register>,
    /// V_BLEND_IN2CSC_COEFF5:Description same as V_BLEND_IN2CSC_COEFF0
    pub v_blend_in2csc_coeff5:
        Aliased<u32, VBlendIn2cscCoeff5R::Register, VBlendIn2cscCoeff5W::Register>,
    /// V_BLEND_IN2CSC_COEFF6:Description same as V_BLEND_IN2CSC_COEFF0
    pub v_blend_in2csc_coeff6:
        Aliased<u32, VBlendIn2cscCoeff6R::Register, VBlendIn2cscCoeff6W::Register>,
    /// V_BLEND_IN2CSC_COEFF7:Description same as V_BLEND_IN2CSC_COEFF0
    pub v_blend_in2csc_coeff7:
        Aliased<u32, VBlendIn2cscCoeff7R::Register, VBlendIn2cscCoeff7W::Register>,
    /// V_BLEND_IN2CSC_COEFF8:Description same as V_BLEND_IN2CSC_COEFF0
    pub v_blend_in2csc_coeff8:
        Aliased<u32, VBlendIn2cscCoeff8R::Register, VBlendIn2cscCoeff8W::Register>,
    /// V_BLEND_LUMA_IN2CSC_OFFSET:Offset values for Y before and after matrix multiplication for input color space conversion
    pub v_blend_luma_in2csc_offset:
        Aliased<u32, VBlendLumaIn2cscOffsetR::Register, VBlendLumaIn2cscOffsetW::Register>,
    /// V_BLEND_CR_IN2CSC_OFFSET:Offset values for CR before and after matrix multiplication for input color space conversion
    pub v_blend_cr_in2csc_offset:
        Aliased<u32, VBlendCrIn2cscOffsetR::Register, VBlendCrIn2cscOffsetW::Register>,
    /// V_BLEND_CB_IN2CSC_OFFSET:Offset values for CB before and after matrix multiplication for input color space conversion
    pub v_blend_cb_in2csc_offset:
        Aliased<u32, VBlendCbIn2cscOffsetR::Register, VBlendCbIn2cscOffsetW::Register>,
    _padding41136: [u8; 288],
    /// V_BLEND_CHROMA_KEY_ENABLE
    pub v_blend_chroma_key_enable:
        Aliased<u32, VBlendChromaKeyEnableR::Register, VBlendChromaKeyEnableW::Register>,
    /// V_BLEND_CHROMA_KEY_COMP1:
    pub v_blend_chroma_key_comp1:
        Aliased<u32, VBlendChromaKeyComp1R::Register, VBlendChromaKeyComp1W::Register>,
    /// V_BLEND_CHROMA_KEY_COMP2
    pub v_blend_chroma_key_comp2:
        Aliased<u32, VBlendChromaKeyComp2R::Register, VBlendChromaKeyComp2W::Register>,
    /// V_BLEND_CHROMA_KEY_COMP3:[11:0]: B component of the key minimum value[27:16]: B component of the key maximum value
    pub v_blend_chroma_key_comp3:
        Aliased<u32, VBlendChromaKeyComp3R::Register, VBlendChromaKeyComp3W::Register>,
    _padding41440: [u8; 3616],
    /// AV_BUF_FORMAT: This register should be programmed based on the Video/Graphics packing format in memory. DP unpacker works based on this
    pub av_buf_format: Aliased<u32, AvBufFormatR::Register, AvBufFormatW::Register>,
    _padding45060: [u8; 4],
    /// The memory fetch latency. This parameter is used to offset the early VTC. The max latency supported is 412. This should have a buffer of 35 pixel clocks than actual maximum latency expected in the system
    pub av_buf_non_live_latency:
        Aliased<u32, AvBufNonLiveLatencyR::Register, AvBufNonLiveLatencyW::Register>,
    _padding45068: [u8; 4],
    /// AV_CHBUF0: Channel Enable, flush and Burst length to be programmed based on video formats. Each channel can be programmed with independent BLChannel 0: must be always enabled for any video mode.Channel 1 and 2: should be enabled for planar modes.Channel 3: for graphics.Channel 4 and 5: for audio modes
    pub av_chbuf0: Aliased<u32, AvChbuf0R::Register, AvChbuf0W::Register>,
    /// AV_CHBUF1:Same as AV_CHBUF0
    pub av_chbuf1: Aliased<u32, AvChbuf1R::Register, AvChbuf1W::Register>,
    /// AV_CHBUF2:Same as AV_CHBUF0
    pub av_chbuf2: Aliased<u32, AvChbuf2R::Register, AvChbuf2W::Register>,
    /// AV_CHBUF3:Same as AV_CHBUF0
    pub av_chbuf3: Aliased<u32, AvChbuf3R::Register, AvChbuf3W::Register>,
    /// AV_CHBUF4
    pub av_chbuf4: Aliased<u32, AvChbuf4R::Register, AvChbuf4W::Register>,
    /// AV_CHBUF5: Same as AV_CHBUF4
    pub av_chbuf5: Aliased<u32, AvChbuf5R::Register, AvChbuf5W::Register>,
    _padding45096: [u8; 4],
    /// AV_BUF_STC_CONTROL:
    pub av_buf_stc_control: ReadWrite<u32, AvBufStcControl::Register>,
    /// AV_BUF_STC_INIT_VALUE0:
    pub av_buf_stc_init_value0: ReadWrite<u32>,
    /// AV_BUF_STC_INIT_VALUE1:
    pub av_buf_stc_init_value1:
        Aliased<u32, AvBufStcInitValue1R::Register, AvBufStcInitValue1W::Register>,
    /// AV_BUF_STC_ADJ: A write to this register triggersSTC adjust
    pub av_buf_stc_adj: ReadWrite<u32, AvBufStcAdj::Register>,
    /// AV_BUF_STC_VIDEO_VSYNC_TS_REG0: STC TS with VSYNC event
    pub av_buf_stc_video_vsync_ts_reg0: ReadOnly<u32>,
    /// AV_BUF_STC_VIDEO_VSYNC_TS_REG1: STC TS with VSYNC event
    pub av_buf_stc_video_vsync_ts_reg1: ReadOnly<u32, AvBufStcVideoVsyncTsReg1::Register>,
    /// AV_BUF_STC_EXT_VSYNC_TS_REG0: STC TS with external VSYNC event
    pub av_buf_stc_ext_vsync_ts_reg0: ReadOnly<u32>,
    /// AV_BUF_STC_EXT_VSYNC_TS_REG1: STC TS with VSYNC event
    pub av_buf_stc_ext_vsync_ts_reg1: ReadOnly<u32, AvBufStcExtVsyncTsReg1::Register>,
    /// AV_BUF_STC_CUSTOM_EVENT_TS_REG0: STC TS with custom event1
    pub av_buf_stc_custom_event_ts_reg0: ReadOnly<u32>,
    /// AV_BUF_STC_CUSTOM_EVENT_TS_REG1: STC TS with custom event1
    pub av_buf_stc_custom_event_ts_reg1: ReadOnly<u32, AvBufStcCustomEventTsReg1::Register>,
    /// AV_BUF_STC_CUSTOM_EVENT2_TS_REG0: STC TS with custom event 2 (can be audio TS)
    pub av_buf_stc_custom_event2_ts_reg0: ReadOnly<u32>,
    /// AV_BUF_STC_CUSTOM_EVENT2_TS_REG1: STC TS with custom event2
    pub av_buf_stc_custom_event2_ts_reg1: ReadOnly<u32, AvBufStcCustomEvent2TsReg1::Register>,
    _padding45148: [u8; 4],
    /// AV_BUF_STC_SNAPSHOT0
    pub av_buf_stc_snapshot0: ReadOnly<u32>,
    /// AV_BUF_STC_SNAPSHOT1
    pub av_buf_stc_snapshot1: ReadOnly<u32, AvBufStcSnapshot1::Register>,
    _padding45160: [u8; 8],
    /// AV_BUF_OUTPUT_AUDIO_VIDEO_SELECT: to select the buffer manager outputs
    pub av_buf_output_audio_video_select: Aliased<
        u32,
        AvBufOutputAudioVideoSelectR::Register,
        AvBufOutputAudioVideoSelectW::Register,
    >,
    /// AV_BUF_HCOUNT_VCOUNT_INT0: When the early VTC timing values(VCOUNT and HCOUNT) match the values programmed in this register and corresponding interrupt mask is enabled, an interrupt is generated
    pub av_buf_hcount_vcount_int0: ReadWrite<u32, AvBufHcountVcountInt0::Register>,
    /// AV_BUF_HCOUNT_VCOUNT_INT1: When the early VTC timing values(VCOUNT and HCOUNT) match the values programmed in this register and corresponding interrupt mask is enabled, an interrupt is generated
    pub av_buf_hcount_vcount_int1: ReadWrite<u32, AvBufHcountVcountInt1::Register>,
    /// This register is used for configuring dither functions
    pub av_buf_dither_config:
        Aliased<u32, AvBufDitherConfigR::Register, AvBufDitherConfigW::Register>,
    /// To set seed for LFSR0
    pub dither_config_seed0:
        Aliased<u32, DitherConfigSeed0R::Register, DitherConfigSeed0W::Register>,
    /// Description same as DITHER_CONFIG_SEED0
    pub dither_config_seed1:
        Aliased<u32, DitherConfigSeed1R::Register, DitherConfigSeed1W::Register>,
    /// Description same as DITHER_CONFIG_SEED0
    pub dither_config_seed2:
        Aliased<u32, DitherConfigSeed2R::Register, DitherConfigSeed2W::Register>,
    /// To set the max output value on video pixel (at the blender output towards DP )
    pub dither_config_max: Aliased<u32, DitherConfigMaxR::Register, DitherConfigMaxW::Register>,
    /// To set the min output value on video pixel (at the blender output towards DP )
    pub dither_config_min: Aliased<u32, DitherConfigMinR::Register, DitherConfigMinW::Register>,
    _padding45204: [u8; 108],
    /// PATTERN_GEN_SELECT:PATTERN_GEN_SELECT:
    pub pattern_gen_select: Aliased<u32, PatternGenSelectR::Register, PatternGenSelectW::Register>,
    /// AUD_CH1_PAT_SELECT
    pub aud_pattern_select1:
        Aliased<u32, AudPatternSelect1R::Register, AudPatternSelect1W::Register>,
    /// AUD_CH2_PAT_SELECT
    pub aud_pattern_select2:
        Aliased<u32, AudPatternSelect2R::Register, AudPatternSelect2W::Register>,
    _padding45324: [u8; 20],
    /// AV_BUF_AUD_VID_CLK_SOURCE: When live video from PL is absent, then the internal clock shall be video pipeline clock. If the live video is present, then clock from PL shall be the video pipe line clock. Similarly for the audio we can select from either PS or PL clock
    pub av_buf_aud_vid_clk_source:
        Aliased<u32, AvBufAudVidClkSourceR::Register, AvBufAudVidClkSourceW::Register>,
    /// AV_BUF_SRST_REG
    pub av_buf_srst_reg: Aliased<u32, AvBufSrstRegR::Register, AvBufSrstRegW::Register>,
    /// AV_BUF_AUDIO_RDY_INTERVAL. Debug register.
    pub av_buf_audio_rdy_interval: ReadWrite<u32, AvBufAudioRdyInterval::Register>,
    /// AV_BUF_AUDIO_CH_CONFIG
    pub av_buf_audio_ch_config:
        Aliased<u32, AvBufAudioChConfigR::Register, AvBufAudioChConfigW::Register>,
    _padding45360: [u8; 208],
    /// Scaling factor for graphics for component 0For 4-bits, scale factor will be 16/15*2^16 = 0x11111For 5-bits, scale factor will be 32/31*2^16 = 0x10842For 6-bits, scale factor will be 64/63*2^16 = 0x10410.For 8-bits, scale factor will be 256/255*2^16 = 0x10101For 10-bits, scale factor will be 1024/1023*2^16 = 0x10040For BPC =12, no scaling is done. This register is unused and can be default
    pub av_buf_graphics_comp0_scale_factor: Aliased<
        u32,
        AvBufGraphicsComp0ScaleFactorR::Register,
        AvBufGraphicsComp0ScaleFactorW::Register,
    >,
    /// Scaling factor for graphics for component1. Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_graphics_comp1_scale_factor: Aliased<
        u32,
        AvBufGraphicsComp1ScaleFactorR::Register,
        AvBufGraphicsComp1ScaleFactorW::Register,
    >,
    /// Scaling factor for graphics for component 2.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_graphics_comp2_scale_factor: Aliased<
        u32,
        AvBufGraphicsComp2ScaleFactorR::Register,
        AvBufGraphicsComp2ScaleFactorW::Register,
    >,
    /// Scaling factor for video color comp0.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_video_comp0_scale_factor:
        Aliased<u32, AvBufVideoComp0ScaleFactorR::Register, AvBufVideoComp0ScaleFactorW::Register>,
    /// Scaling factor for video color comp1.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_video_comp1_scale_factor:
        Aliased<u32, AvBufVideoComp1ScaleFactorR::Register, AvBufVideoComp1ScaleFactorW::Register>,
    /// Scaling factor for video color comp2.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_video_comp2_scale_factor:
        Aliased<u32, AvBufVideoComp2ScaleFactorR::Register, AvBufVideoComp2ScaleFactorW::Register>,
    /// Scaling factor for live video color comp0.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_live_video_comp0_sf:
        Aliased<u32, AvBufLiveVideoComp0SfR::Register, AvBufLiveVideoComp0SfW::Register>,
    /// Scaling factor for live video color comp1.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_live_video_comp1_sf:
        Aliased<u32, AvBufLiveVideoComp1SfR::Register, AvBufLiveVideoComp1SfW::Register>,
    /// Scaling factor for live video color comp2.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_live_video_comp2_sf:
        Aliased<u32, AvBufLiveVideoComp2SfR::Register, AvBufLiveVideoComp2SfW::Register>,
    /// Programmable option to configure Cb or Cr first, when YUV422 mode is enabled
    pub av_buf_live_vid_config:
        Aliased<u32, AvBufLiveVidConfigR::Register, AvBufLiveVidConfigW::Register>,
    /// Scaling factor for live graphics color comp0.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_live_gfx_comp0_sf:
        Aliased<u32, AvBufLiveGfxComp0SfR::Register, AvBufLiveGfxComp0SfW::Register>,
    /// Scaling factor for live graphics color comp1.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_live_gfx_comp1_sf:
        Aliased<u32, AvBufLiveGfxComp1SfR::Register, AvBufLiveGfxComp1SfW::Register>,
    /// Scaling factor for live graphics color comp2.Description same as AV_BUF_GRAPHICS_COMP0_SCALE_FACTOR
    pub av_buf_live_gfx_comp2_sf:
        Aliased<u32, AvBufLiveGfxComp2SfR::Register, AvBufLiveGfxComp2SfW::Register>,
    /// Programmable option to configure Cb or Cr first, when YUV422 mode is enabled
    pub av_buf_live_gfx_config:
        Aliased<u32, AvBufLiveGfxConfigR::Register, AvBufLiveGfxConfigW::Register>,
    _padding45624: [u8; 3528],
    /// AUDIO_MIXER_VOLUME_CONTROL:Setting value to 8192 means no volume change (1.0 scaling factor)
    pub audio_mixer_volume_control: ReadWrite<u32, AudioMixerVolumeControl::Register>,
    /// AUDIO_MIXER_META_DATA
    pub audio_mixer_meta_data:
        Aliased<u32, AudioMixerMetaDataR::Register, AudioMixerMetaDataW::Register>,
    /// AUD_CH_STATUS_REG0: Audio Channel status bits 31 to 0
    pub aud_ch_status_reg0: ReadWrite<u32>,
    /// AUD_CH_STATUS_REG1: Audio Channel status bits 63 to 32
    pub aud_ch_status_reg1: ReadWrite<u32>,
    /// AUD_CH_STATUS_REG2: Audio Channel status bits 95 to 64
    pub aud_ch_status_reg2: ReadWrite<u32>,
    /// AUD_CH_STATUS_REG3: Audio Channel status bits 127 to 96
    pub aud_ch_status_reg3: ReadWrite<u32>,
    /// AUD_CH_STATUS_REG4: Audio Channel status bits 159 to 128
    pub aud_ch_status_reg4: ReadWrite<u32>,
    /// AUD_CH_STATUS_REG5: Audio Channel status bits 191 to 160
    pub aud_ch_status_reg5: ReadWrite<u32>,
    /// AUD_CH_A_DATA_REG0: User data bits 31 to 0
    pub aud_ch_a_data_reg0: ReadWrite<u32>,
    /// AUD_CH_A_DATA_REG1: User data bits 63 to 32
    pub aud_ch_a_data_reg1: ReadWrite<u32>,
    /// AUD_CH_A_DATA_REG2: User data bits 95 to 64
    pub aud_ch_a_data_reg2: ReadWrite<u32>,
    /// AUD_CH_A_DATA_REG3: User data bits 127 to 96
    pub aud_ch_a_data_reg3: ReadWrite<u32>,
    /// AUD_CH_A_DATA_REG4: User data bits 159 to 128
    pub aud_ch_a_data_reg4: ReadWrite<u32>,
    /// AUD_CH_A_DATA_REG5: User data bits 191 to 160
    pub aud_ch_a_data_reg5: ReadWrite<u32>,
    /// AUD_CH_B_DATA_REG0: User data bits 31 to 0.
    pub aud_ch_b_data_reg0: ReadWrite<u32>,
    /// AUD_CH_B_DATA_REG1: User data bits 63 to 32.
    pub aud_ch_b_data_reg1: ReadWrite<u32>,
    /// AUD_CH_B_DATA_REG2: User data bits 95 to 64.
    pub aud_ch_b_data_reg2: ReadWrite<u32>,
    /// AUD_CH_B_DATA_REG3: User data bits 127 to 96.
    pub aud_ch_b_data_reg3: ReadWrite<u32>,
    /// AUD_CH_B_DATA_REG4: User data bits 159 to 128.
    pub aud_ch_b_data_reg4: ReadWrite<u32>,
    /// AUD_CH_B_DATA_REG5: User data bits 191 to 160.
    pub aud_ch_b_data_reg5: ReadWrite<u32>,
    _padding49232: [u8; 2992],
    /// Audio Soft reset reigster.
    pub audio_soft_reset: Aliased<u32, AudioSoftResetR::Register, AudioSoftResetW::Register>,
    _padding52228: [u8; 12],
    /// 16 bit CRC calculated on the first component of video output from Internal Test Pattern Generator
    pub patgen_crc_r: ReadOnly<u32, PatgenCrcR::Register>,
    /// 16 bit CRC calculated on the second component of video output from Internal Test Pattern Generator
    pub patgen_crc_g: ReadOnly<u32, PatgenCrcG::Register>,
    /// 16 bit CRC calculated on the third component of video output from Internal Test Pattern Generator
    pub patgen_crc_b: ReadOnly<u32, PatgenCrcB::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub DpLinkBwSetR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        BW OFFSET(0) NUMBITS(8) [],
    ],
    pub DpLinkBwSetW [
        BW OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpLaneCountSetR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        LANE_CNT OFFSET(0) NUMBITS(5) [],
    ],
    pub DpLaneCountSetW [
        LANE_CNT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpEnhancedFrameEnR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ENH_FRAMING_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpEnhancedFrameEnW [
        ENH_FRAMING_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpTrainingPatternSetR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        TP_SET OFFSET(0) NUMBITS(2) [],
    ],
    pub DpTrainingPatternSetW [
        TP_SET OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpLinkQualPatternSetR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        EXT OFFSET(2) NUMBITS(1) [],
        LNK_QUAL_PAT_SET OFFSET(0) NUMBITS(2) [],
    ],
    pub DpLinkQualPatternSetW [
        EXT OFFSET(2) NUMBITS(1) [],
        LNK_QUAL_PAT_SET OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpScramblingDisableR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SCR_DIS OFFSET(0) NUMBITS(1) [],
    ],
    pub DpScramblingDisableW [
        SCR_DIS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpDownspreadCtrlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        DWNSPRD_CTL OFFSET(0) NUMBITS(1) [],
    ],
    pub DpDownspreadCtrlW [
        DWNSPRD_CTL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpSoftwareResetR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub DpSoftwareResetW [
        SOFT_RST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpCompPattern80bit3R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        BITS_79_64 OFFSET(0) NUMBITS(16) [],
    ],
    pub DpCompPattern80bit3W [
        BITS_79_64 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpTransmitterEnableR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        TX_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub DpTransmitterEnableW [
        TX_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamEnableR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        MS_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub DpMainStreamEnableW [
        MS_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpForceScramblerResetR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub DpForceScramblerResetW [
        FORCE_SCR_RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpAuxCommandR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        ADDR_TRANSFER_EN OFFSET(12) NUMBITS(1) [],
        AUD_CH_COMMAND OFFSET(8) NUMBITS(4) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        NUM_OF_BYTES OFFSET(0) NUMBITS(4) [],
    ],
    pub DpAuxCommandW [
        ADDR_TRANSFER_EN OFFSET(12) NUMBITS(1) [],
        AUD_CH_COMMAND OFFSET(8) NUMBITS(4) [],
        NUM_OF_BYTES OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpAuxWriteFifoR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
    ],
    pub DpAuxWriteFifoW [
        AUX_WRITE_FIFO OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpAuxAddressR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        AUX_ADDRESS OFFSET(0) NUMBITS(20) [],
    ],
    pub DpAuxAddressW [
        AUX_ADDRESS OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpAuxClockDividerR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        AUX_SIGNAL_WIDTH_FILTER OFFSET(8) NUMBITS(8) [],
        CLK_DIV OFFSET(0) NUMBITS(8) [],
    ],
    pub DpAuxClockDividerW [
        AUX_SIGNAL_WIDTH_FILTER OFFSET(8) NUMBITS(8) [],
        CLK_DIV OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpTxUserFifoOverflow [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpInterruptSignalState [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        REPLY_TIMEOUT OFFSET(3) NUMBITS(1) [],
        REPLY_STATE OFFSET(2) NUMBITS(1) [],
        REQUEST_STATE OFFSET(1) NUMBITS(1) [],
        HPD_STATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpAuxReplyData [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        AUX_REPLY_DATA OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpAuxReplyCode [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        CODE1 OFFSET(2) NUMBITS(2) [],
        CODE0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpAuxReplyCountR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        AUX_REPLY_COUNT OFFSET(0) NUMBITS(8) [],
    ],
    pub DpAuxReplyCountW [
        AUX_REPLY_COUNT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpReplyDataCountR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        REPLY_DATA_COUNT OFFSET(0) NUMBITS(5) [],
    ],
    pub DpReplyDataCountW [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpReplyStatus [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        AUX_REPLY_STATE OFFSET(4) NUMBITS(8) [],
        REPLY_ERROR OFFSET(3) NUMBITS(1) [],
        REQUEST_IN_PROGRESS OFFSET(2) NUMBITS(1) [],
        REPLY_IN_PROGRESS OFFSET(1) NUMBITS(1) [],
        REPLY_RECEIVED OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpHpdDuration [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        HPD_DURATION OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamHtotalR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        HTOTAL OFFSET(0) NUMBITS(16) [],
    ],
    pub DpMainStreamHtotalW [
        HTOTAL OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamVtotalR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VTOTAL OFFSET(0) NUMBITS(16) [],
    ],
    pub DpMainStreamVtotalW [
        VTOTAL OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamPolarityR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        VSYNC_POLARITY OFFSET(1) NUMBITS(1) [],
        HSYNC_POLARITY OFFSET(0) NUMBITS(1) [],
    ],
    pub DpMainStreamPolarityW [
        VSYNC_POLARITY OFFSET(1) NUMBITS(1) [],
        HSYNC_POLARITY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamHswidthR [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        HSWIDTH OFFSET(0) NUMBITS(15) [],
    ],
    pub DpMainStreamHswidthW [
        HSWIDTH OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamVswidthR [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        VSWIDTH OFFSET(0) NUMBITS(15) [],
    ],
    pub DpMainStreamVswidthW [
        VSWIDTH OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamHresR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        HRES OFFSET(0) NUMBITS(16) [],
    ],
    pub DpMainStreamHresW [
        HRES OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamVresR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VRES OFFSET(0) NUMBITS(16) [],
    ],
    pub DpMainStreamVresW [
        VRES OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamHstartR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        HSTART OFFSET(0) NUMBITS(16) [],
    ],
    pub DpMainStreamHstartW [
        HSTART OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamVstartR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VSTART OFFSET(0) NUMBITS(16) [],
    ],
    pub DpMainStreamVstartW [
        VSTART OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamMisc0R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        BPC OFFSET(5) NUMBITS(3) [],
        YCBCR_COLR OFFSET(4) NUMBITS(1) [],
        DYNC_RANGE OFFSET(3) NUMBITS(1) [],
        COMP_FORMAT OFFSET(1) NUMBITS(2) [],
        SYNC_CLOCK OFFSET(0) NUMBITS(1) [],
    ],
    pub DpMainStreamMisc0W [
        BPC OFFSET(5) NUMBITS(3) [],
        YCBCR_COLR OFFSET(4) NUMBITS(1) [],
        DYNC_RANGE OFFSET(3) NUMBITS(1) [],
        COMP_FORMAT OFFSET(1) NUMBITS(2) [],
        SYNC_CLOCK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamMisc1R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        Y_ONLY_EN OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(4) [],
        STEREO_VID_ATTR OFFSET(1) NUMBITS(2) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub DpMainStreamMisc1W [
        Y_ONLY_EN OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(4) [],
        STEREO_VID_ATTR OFFSET(1) NUMBITS(2) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamMVidR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        M_VID OFFSET(0) NUMBITS(24) [],
    ],
    pub DpMainStreamMVidW [
        M_VID OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMsaTransferUnitSizeR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        TU_SIZE OFFSET(0) NUMBITS(7) [],
    ],
    pub DpMsaTransferUnitSizeW [
        TU_SIZE OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMainStreamNVidR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        N_VID OFFSET(0) NUMBITS(24) [],
    ],
    pub DpMainStreamNVidW [
        N_VID OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpUserPixWidth [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PIX_WIDTH OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpUserDataCountPerLaneR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        DATA_CNT_PER_LANE OFFSET(0) NUMBITS(18) [],
    ],
    pub DpUserDataCountPerLaneW [
        DATA_CNT_PER_LANE OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpMinBytesPerTuR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        MIN_BYTES_PER_TU OFFSET(0) NUMBITS(7) [],
    ],
    pub DpMinBytesPerTuW [
        MIN_BYTES_PER_TU OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpFracBytesPerTuR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        FRACT_BYTES_PER_TU OFFSET(0) NUMBITS(10) [],
    ],
    pub DpFracBytesPerTuW [
        FRACT_BYTES_PER_TU OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpInitWaitR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        INIT_WAIT OFFSET(0) NUMBITS(7) [],
    ],
    pub DpInitWaitW [
        INIT_WAIT OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpPhyResetR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        EN_8B_10B OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(14) [],
        GT_RESET OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub DpPhyResetW [
        EN_8B_10B OFFSET(16) NUMBITS(1) [],
        GT_RESET OFFSET(1) NUMBITS(1) [],
        RESERVED0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpTransmitPrbs7R [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        TX_PRBS7 OFFSET(0) NUMBITS(1) [],
    ],
    pub DpTransmitPrbs7W [
        TX_PRBS7 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpPhyClockSelectR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RESERVED1 OFFSET(3) NUMBITS(5) [],
        SEL OFFSET(0) NUMBITS(3) [],
    ],
    pub DpPhyClockSelectW [
        RESERVED0 OFFSET(3) NUMBITS(5) [],
        SEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpTxPhyPowerDownR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        POWER_DWN OFFSET(0) NUMBITS(4) [],
    ],
    pub DpTxPhyPowerDownW [
        POWER_DWN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpPhyPrecursorLane0R [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        PRECURSOR0 OFFSET(0) NUMBITS(5) [],
    ],
    pub DpPhyPrecursorLane0W [
        PRECURSOR0 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpPhyPrecursorLane1R [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        PRECURSOR1 OFFSET(0) NUMBITS(5) [],
    ],
    pub DpPhyPrecursorLane1W [
        PRECURSOR1 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpPhyStatus [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RESERVED1 OFFSET(6) NUMBITS(2) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        PLL_LOCKED OFFSET(4) NUMBITS(1) [],
        RATE_CHANGE_DONE_0_1 OFFSET(2) NUMBITS(2) [],
        RESET_LANES_0_1 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpTxAudioControlR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        RESERVED1 OFFSET(1) NUMBITS(3) [],
        TX_AUD_CTRL OFFSET(0) NUMBITS(1) [],
    ],
    pub DpTxAudioControlW [
        RESERVED0 OFFSET(1) NUMBITS(3) [],
        TX_AUD_CTRL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpTxAudioChannels [
        TX_AUD_CH OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpTxMAudR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        MAUD OFFSET(0) NUMBITS(24) [],
    ],
    pub DpTxMAudW [
        MAUD OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpTxNAudR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        NAUD OFFSET(0) NUMBITS(24) [],
    ],
    pub DpTxNAudW [
        NAUD OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntStatusR [
        VSYNC_TS OFFSET(31) NUMBITS(1) [],
        EXT_VSYNC_TS OFFSET(30) NUMBITS(1) [],
        CUST_TS OFFSET(29) NUMBITS(1) [],
        CUST_TS_2 OFFSET(28) NUMBITS(1) [],
        CHBUF0_OVERFLW OFFSET(27) NUMBITS(1) [],
        CHBUF1_OVERFLW OFFSET(26) NUMBITS(1) [],
        CHBUF2_OVERFLW OFFSET(25) NUMBITS(1) [],
        CHBUF3_OVERFLW OFFSET(24) NUMBITS(1) [],
        CHBUF4_OVERFLW OFFSET(23) NUMBITS(1) [],
        CHBUF5_OVERFLW OFFSET(22) NUMBITS(1) [],
        CHBUF0_UNDERFLW OFFSET(21) NUMBITS(1) [],
        CHBUF1_UNDERFLW OFFSET(20) NUMBITS(1) [],
        CHBUF2_UNDERFLW OFFSET(19) NUMBITS(1) [],
        CHBUF3_UNDERFLW OFFSET(18) NUMBITS(1) [],
        CHBUF4_UNDERFLW OFFSET(17) NUMBITS(1) [],
        CHBUF5_UNDERFLW OFFSET(16) NUMBITS(1) [],
        PIXEL0_MATCH OFFSET(15) NUMBITS(1) [],
        PIXEL1_MATCH OFFSET(14) NUMBITS(1) [],
        VBLNK_START OFFSET(13) NUMBITS(1) [],
        LIV_ABUF_UNDRFLW OFFSET(12) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(6) [],
        EXT_PKT_TXD OFFSET(5) NUMBITS(1) [],
        HPD_PULSE_DET OFFSET(4) NUMBITS(1) [],
        REPLY_TIMEOUT OFFSET(3) NUMBITS(1) [],
        REPLY_RECEIVED OFFSET(2) NUMBITS(1) [],
        HPD_EVENT OFFSET(1) NUMBITS(1) [],
        HPD_IRQ OFFSET(0) NUMBITS(1) [],
    ],
    pub DpIntStatusW [
        VSYNC_TS OFFSET(31) NUMBITS(1) [],
        EXT_VSYNC_TS OFFSET(30) NUMBITS(1) [],
        CUST_TS OFFSET(29) NUMBITS(1) [],
        CUST_TS_2 OFFSET(28) NUMBITS(1) [],
        CHBUF0_OVERFLW OFFSET(27) NUMBITS(1) [],
        CHBUF1_OVERFLW OFFSET(26) NUMBITS(1) [],
        CHBUF2_OVERFLW OFFSET(25) NUMBITS(1) [],
        CHBUF3_OVERFLW OFFSET(24) NUMBITS(1) [],
        CHBUF4_OVERFLW OFFSET(23) NUMBITS(1) [],
        CHBUF5_OVERFLW OFFSET(22) NUMBITS(1) [],
        CHBUF0_UNDERFLW OFFSET(21) NUMBITS(1) [],
        CHBUF1_UNDERFLW OFFSET(20) NUMBITS(1) [],
        CHBUF2_UNDERFLW OFFSET(19) NUMBITS(1) [],
        CHBUF3_UNDERFLW OFFSET(18) NUMBITS(1) [],
        CHBUF4_UNDERFLW OFFSET(17) NUMBITS(1) [],
        CHBUF5_UNDERFLW OFFSET(16) NUMBITS(1) [],
        PIXEL0_MATCH OFFSET(15) NUMBITS(1) [],
        PIXEL1_MATCH OFFSET(14) NUMBITS(1) [],
        VBLNK_START OFFSET(13) NUMBITS(1) [],
        LIV_ABUF_UNDRFLW OFFSET(12) NUMBITS(1) [],
        EXT_PKT_TXD OFFSET(5) NUMBITS(1) [],
        HPD_PULSE_DET OFFSET(4) NUMBITS(1) [],
        REPLY_TIMEOUT OFFSET(3) NUMBITS(1) [],
        REPLY_RECEIVED OFFSET(2) NUMBITS(1) [],
        HPD_EVENT OFFSET(1) NUMBITS(1) [],
        HPD_IRQ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntMask [
        VSYNC_TS OFFSET(31) NUMBITS(1) [],
        EXT_VSYNC_TS OFFSET(30) NUMBITS(1) [],
        CUST_TS OFFSET(29) NUMBITS(1) [],
        CUST_TS_2 OFFSET(28) NUMBITS(1) [],
        CHBUF0_OVERFLW OFFSET(27) NUMBITS(1) [],
        CHBUF1_OVERFLW OFFSET(26) NUMBITS(1) [],
        CHBUF2_OVERFLW OFFSET(25) NUMBITS(1) [],
        CHBUF3_OVERFLW OFFSET(24) NUMBITS(1) [],
        CHBUF4_OVERFLW OFFSET(23) NUMBITS(1) [],
        CHBUF5_OVERFLW OFFSET(22) NUMBITS(1) [],
        CHBUF0_UNDERFLW OFFSET(21) NUMBITS(1) [],
        CHBUF1_UNDERFLW OFFSET(20) NUMBITS(1) [],
        CHBUF2_UNDERFLW OFFSET(19) NUMBITS(1) [],
        CHBUF3_UNDERFLW OFFSET(18) NUMBITS(1) [],
        CHBUF4_UNDERFLW OFFSET(17) NUMBITS(1) [],
        CHBUF5_UNDERFLW OFFSET(16) NUMBITS(1) [],
        PIXEL0_MATCH OFFSET(15) NUMBITS(1) [],
        PIXEL1_MATCH OFFSET(14) NUMBITS(1) [],
        VBLNK_START OFFSET(13) NUMBITS(1) [],
        LIV_AUDBUF_UNDRFLW OFFSET(12) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(6) [],
        EXT_PKT_TXD OFFSET(5) NUMBITS(1) [],
        HPD_PULSE_DET OFFSET(4) NUMBITS(1) [],
        REPLY_TIMEOUT OFFSET(3) NUMBITS(1) [],
        REPLY_RECEIVED OFFSET(2) NUMBITS(1) [],
        HPD_EVENT OFFSET(1) NUMBITS(1) [],
        HPD_IRQ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntEnR [
        RESERVED0 OFFSET(6) NUMBITS(6) [],
    ],
    pub DpIntEnW [
        VSYNC_TS OFFSET(31) NUMBITS(1) [],
        EXT_VSYNC_TS OFFSET(30) NUMBITS(1) [],
        CUST_TS OFFSET(29) NUMBITS(1) [],
        CUST_TS_2 OFFSET(28) NUMBITS(1) [],
        CHBUF0_OVERFLW OFFSET(27) NUMBITS(1) [],
        CHBUF1_OVERFLW OFFSET(26) NUMBITS(1) [],
        CHBUF2_OVERFLW OFFSET(25) NUMBITS(1) [],
        CHBUF3_OVERFLW OFFSET(24) NUMBITS(1) [],
        CHBUF4_OVERFLW OFFSET(23) NUMBITS(1) [],
        CHBUF5_OVERFLW OFFSET(22) NUMBITS(1) [],
        CHBUF0_UNDERFLW OFFSET(21) NUMBITS(1) [],
        CHBUF1_UNDERFLW OFFSET(20) NUMBITS(1) [],
        CHBUF2_UNDERFLW OFFSET(19) NUMBITS(1) [],
        CHBUF3_UNDERFLW OFFSET(18) NUMBITS(1) [],
        CHBUF4_UNDERFLW OFFSET(17) NUMBITS(1) [],
        CHBUF5_UNDERFLW OFFSET(16) NUMBITS(1) [],
        PIXEL0_MATCH OFFSET(15) NUMBITS(1) [],
        PIXEL1_MATCH OFFSET(14) NUMBITS(1) [],
        VBLNK_START OFFSET(13) NUMBITS(1) [],
        LIV_AUDBUF_UNDRFLW OFFSET(12) NUMBITS(1) [],
        EXT_PKT_TXD OFFSET(5) NUMBITS(1) [],
        HPD_PULSE_DET OFFSET(4) NUMBITS(1) [],
        REPLY_TIMEOUT OFFSET(3) NUMBITS(1) [],
        REPLY_RECEIVED OFFSET(2) NUMBITS(1) [],
        HPD_EVENT OFFSET(1) NUMBITS(1) [],
        HPD_IRQ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntDsR [
        RESERVED0 OFFSET(6) NUMBITS(6) [],
    ],
    pub DpIntDsW [
        VSYNC_TS OFFSET(31) NUMBITS(1) [],
        EXT_VSYNC_TS OFFSET(30) NUMBITS(1) [],
        CUST_TS OFFSET(29) NUMBITS(1) [],
        CUST_TS_2 OFFSET(28) NUMBITS(1) [],
        CHBUF0_OVERFLW OFFSET(27) NUMBITS(1) [],
        CHBUF1_OVERFLW OFFSET(26) NUMBITS(1) [],
        CHBUF2_OVERFLW OFFSET(25) NUMBITS(1) [],
        CHBUF3_OVERFLW OFFSET(24) NUMBITS(1) [],
        CHBUF4_OVERFLW OFFSET(23) NUMBITS(1) [],
        CHBUF5_OVERFLW OFFSET(22) NUMBITS(1) [],
        CHBUF0_UNDERFLW OFFSET(21) NUMBITS(1) [],
        CHBUF1_UNDERFLW OFFSET(20) NUMBITS(1) [],
        CHBUF2_UNDERFLW OFFSET(19) NUMBITS(1) [],
        CHBUF3_UNDERFLW OFFSET(18) NUMBITS(1) [],
        CHBUF4_UNDERFLW OFFSET(17) NUMBITS(1) [],
        CHBUF5_UNDERFLW OFFSET(16) NUMBITS(1) [],
        PIXEL0_MATCH OFFSET(15) NUMBITS(1) [],
        PIXEL1_MATCH OFFSET(14) NUMBITS(1) [],
        VBLNK_START OFFSET(13) NUMBITS(1) [],
        LIV_AUDBUF_UNDRFLW OFFSET(12) NUMBITS(1) [],
        EXT_PKT_TXD OFFSET(5) NUMBITS(1) [],
        HPD_PULSE_DET OFFSET(4) NUMBITS(1) [],
        REPLY_TIMEOUT OFFSET(3) NUMBITS(1) [],
        REPLY_RECEIVED OFFSET(2) NUMBITS(1) [],
        HPD_EVENT OFFSET(1) NUMBITS(1) [],
        HPD_IRQ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendBgClr0R [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        CLR0 OFFSET(0) NUMBITS(12) [],
    ],
    pub VBlendBgClr0W [
        CLR0 OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendBgClr1R [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        CLR1 OFFSET(0) NUMBITS(12) [],
    ],
    pub VBlendBgClr1W [
        CLR1 OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendBgClr2R [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        CLR2 OFFSET(0) NUMBITS(12) [],
    ],
    pub VBlendBgClr2W [
        CLR2 OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendSetGlobalAlphaRegR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        VALUE OFFSET(1) NUMBITS(8) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub VBlendSetGlobalAlphaRegW [
        VALUE OFFSET(1) NUMBITS(8) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendOutputVidFormatR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        EN_DOWNSAMPLE OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        VID_FORMAT OFFSET(0) NUMBITS(3) [],
    ],
    pub VBlendOutputVidFormatW [
        EN_DOWNSAMPLE OFFSET(4) NUMBITS(1) [],
        VID_FORMAT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendLayer0ControlR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        BYPASS OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(6) [],
        RGB_MODE OFFSET(1) NUMBITS(1) [],
        EN_US OFFSET(0) NUMBITS(1) [],
    ],
    pub VBlendLayer0ControlW [
        BYPASS OFFSET(8) NUMBITS(1) [],
        RGB_MODE OFFSET(1) NUMBITS(1) [],
        EN_US OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendLayer1ControlR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        BYPASS OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(6) [],
        RGB_MODE OFFSET(1) NUMBITS(1) [],
        EN_US OFFSET(0) NUMBITS(1) [],
    ],
    pub VBlendLayer1ControlW [
        BYPASS OFFSET(8) NUMBITS(1) [],
        RGB_MODE OFFSET(1) NUMBITS(1) [],
        EN_US OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendRgb2ycbcrCoeff0R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        RGB2Y_C0 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendRgb2ycbcrCoeff0W [
        RGB2Y_C0 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendRgb2ycbcrCoeff1R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        RGB2Y_C1 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendRgb2ycbcrCoeff1W [
        RGB2Y_C1 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendRgb2ycbcrCoeff2R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        RGB2Y_C2 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendRgb2ycbcrCoeff2W [
        RGB2Y_C2 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendRgb2ycbcrCoeff3R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        RGB2Y_C3 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendRgb2ycbcrCoeff3W [
        RGB2Y_C3 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendRgb2ycbcrCoeff4R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        RGB2Y_C4 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendRgb2ycbcrCoeff4W [
        RGB2Y_C4 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendRgb2ycbcrCoeff5R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        RGB2Y_C5 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendRgb2ycbcrCoeff5W [
        RGB2Y_C5 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendRgb2ycbcrCoeff6R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        RGB2Y_C6 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendRgb2ycbcrCoeff6W [
        RGB2Y_C6 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendRgb2ycbcrCoeff7R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        RGB2Y_C7 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendRgb2ycbcrCoeff7W [
        RGB2Y_C7 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendRgb2ycbcrCoeff8R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        RGB2Y_C8 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendRgb2ycbcrCoeff8W [
        RGB2Y_C8 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn1cscCoeff0R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C0 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn1cscCoeff0W [
        Y2R_C0 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn1cscCoeff1R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C1 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn1cscCoeff1W [
        Y2R_C1 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn1cscCoeff2R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C2 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn1cscCoeff2W [
        Y2R_C2 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn1cscCoeff3R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C3 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn1cscCoeff3W [
        Y2R_C3 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn1cscCoeff4R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C4 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn1cscCoeff4W [
        Y2R_C4 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn1cscCoeff5R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C5 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn1cscCoeff5W [
        Y2R_C5 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn1cscCoeff6R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C6 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn1cscCoeff6W [
        Y2R_C6 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn1cscCoeff7R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C7 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn1cscCoeff7W [
        Y2R_C7 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn1cscCoeff8R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C8 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn1cscCoeff8W [
        Y2R_C8 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendLumaIn1cscOffsetR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ],
    pub VBlendLumaIn1cscOffsetW [
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendCrIn1cscOffsetR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ],
    pub VBlendCrIn1cscOffsetW [
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendCbIn1cscOffsetR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ],
    pub VBlendCbIn1cscOffsetW [
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendLumaOutcscOffsetR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ],
    pub VBlendLumaOutcscOffsetW [
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendCrOutcscOffsetR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ],
    pub VBlendCrOutcscOffsetW [
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendCbOutcscOffsetR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ],
    pub VBlendCbOutcscOffsetW [
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn2cscCoeff0R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C0 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn2cscCoeff0W [
        Y2R_C0 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn2cscCoeff1R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C1 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn2cscCoeff1W [
        Y2R_C1 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn2cscCoeff2R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C2 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn2cscCoeff2W [
        Y2R_C2 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn2cscCoeff3R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C3 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn2cscCoeff3W [
        Y2R_C3 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn2cscCoeff4R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C4 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn2cscCoeff4W [
        Y2R_C4 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn2cscCoeff5R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C5 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn2cscCoeff5W [
        Y2R_C5 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn2cscCoeff6R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C6 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn2cscCoeff6W [
        Y2R_C6 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn2cscCoeff7R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C7 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn2cscCoeff7W [
        Y2R_C7 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendIn2cscCoeff8R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        Y2R_C8 OFFSET(0) NUMBITS(15) [],
    ],
    pub VBlendIn2cscCoeff8W [
        Y2R_C8 OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendLumaIn2cscOffsetR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ],
    pub VBlendLumaIn2cscOffsetW [
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendCrIn2cscOffsetR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ],
    pub VBlendCrIn2cscOffsetW [
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendCbIn2cscOffsetR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ],
    pub VBlendCbIn2cscOffsetW [
        POST_OFFSET OFFSET(16) NUMBITS(13) [],
        PRE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendChromaKeyEnableR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        M_SEL OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub VBlendChromaKeyEnableW [
        M_SEL OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendChromaKeyComp1R [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        MAX OFFSET(16) NUMBITS(12) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        MIN OFFSET(0) NUMBITS(12) [],
    ],
    pub VBlendChromaKeyComp1W [
        MAX OFFSET(16) NUMBITS(12) [],
        MIN OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendChromaKeyComp2R [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        MAX OFFSET(16) NUMBITS(12) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        MIN OFFSET(0) NUMBITS(12) [],
    ],
    pub VBlendChromaKeyComp2W [
        MAX OFFSET(16) NUMBITS(12) [],
        MIN OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VBlendChromaKeyComp3R [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        MAX OFFSET(16) NUMBITS(12) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        MIN OFFSET(0) NUMBITS(12) [],
    ],
    pub VBlendChromaKeyComp3W [
        MAX OFFSET(16) NUMBITS(12) [],
        MIN OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufFormatR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        NL_GRAPHX_FORMAT OFFSET(8) NUMBITS(4) [],
        NL_VID_FORMAT OFFSET(0) NUMBITS(5) [],
    ],
    pub AvBufFormatW [
        NL_GRAPHX_FORMAT OFFSET(8) NUMBITS(4) [],
        NL_VID_FORMAT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufNonLiveLatencyR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        NL_LATENCY OFFSET(0) NUMBITS(10) [],
    ],
    pub AvBufNonLiveLatencyW [
        NL_LATENCY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvChbuf0R [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub AvChbuf0W [
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvChbuf1R [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub AvChbuf1W [
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvChbuf2R [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub AvChbuf2W [
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvChbuf3R [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub AvChbuf3W [
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvChbuf4R [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub AvChbuf4W [
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvChbuf5R [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub AvChbuf5W [
        BURST_LEN OFFSET(2) NUMBITS(5) [],
        FLUSH OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufStcControl [
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufStcInitValue1R [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        INIT_VALUE1 OFFSET(0) NUMBITS(10) [],
    ],
    pub AvBufStcInitValue1W [
        INIT_VALUE1 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufStcAdj [
        SIGN OFFSET(31) NUMBITS(1) [],
        VALUE OFFSET(0) NUMBITS(31) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufStcVideoVsyncTsReg1 [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        VSYNC_TS1 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufStcExtVsyncTsReg1 [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        EXT_VSYNC_TS1 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufStcCustomEventTsReg1 [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        CUST_EVENT_TS1 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufStcCustomEvent2TsReg1 [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        CUST_EVENT2_TS1 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufStcSnapshot1 [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        STC1 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufOutputAudioVideoSelectR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        AUD_STREAM2_SEL OFFSET(6) NUMBITS(1) [],
        AUD_STREAM1_SEL OFFSET(4) NUMBITS(2) [],
        VID_STREAM2_SEL OFFSET(2) NUMBITS(2) [],
        VID_STREAM1_SEL OFFSET(0) NUMBITS(2) [],
    ],
    pub AvBufOutputAudioVideoSelectW [
        AUD_STREAM2_SEL OFFSET(6) NUMBITS(1) [],
        AUD_STREAM1_SEL OFFSET(4) NUMBITS(2) [],
        VID_STREAM2_SEL OFFSET(2) NUMBITS(2) [],
        VID_STREAM1_SEL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufHcountVcountInt0 [
        HCOUNT OFFSET(16) NUMBITS(14) [],
        VCOUNT OFFSET(0) NUMBITS(14) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufHcountVcountInt1 [
        HCOUNT OFFSET(16) NUMBITS(14) [],
        VCOUNT OFFSET(0) NUMBITS(14) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufDitherConfigR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        TAP_MSB OFFSET(10) NUMBITS(1) [],
        DW_SEL OFFSET(9) NUMBITS(1) [],
        LD OFFSET(8) NUMBITS(1) [],
        TRUNC_PT OFFSET(5) NUMBITS(3) [],
        MODE OFFSET(3) NUMBITS(2) [],
        SIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub AvBufDitherConfigW [
        TAP_MSB OFFSET(10) NUMBITS(1) [],
        DW_SEL OFFSET(9) NUMBITS(1) [],
        LD OFFSET(8) NUMBITS(1) [],
        TRUNC_PT OFFSET(5) NUMBITS(3) [],
        MODE OFFSET(3) NUMBITS(2) [],
        SIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DitherConfigSeed0R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COLR0 OFFSET(0) NUMBITS(16) [],
    ],
    pub DitherConfigSeed0W [
        COLR0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DitherConfigSeed1R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COLR1 OFFSET(0) NUMBITS(16) [],
    ],
    pub DitherConfigSeed1W [
        COLR1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DitherConfigSeed2R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        COLR2 OFFSET(0) NUMBITS(16) [],
    ],
    pub DitherConfigSeed2W [
        COLR2 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DitherConfigMaxR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        COLR_MAX OFFSET(0) NUMBITS(12) [],
    ],
    pub DitherConfigMaxW [
        COLR_MAX OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DitherConfigMinR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        COLR_MIN OFFSET(0) NUMBITS(12) [],
    ],
    pub DitherConfigMinW [
        COLR_MIN OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PatternGenSelectR [
        OFFSET_EQ OFFSET(8) NUMBITS(24) [],
        RESERVED0 OFFSET(2) NUMBITS(6) [],
        AUD_RATE_SEL OFFSET(0) NUMBITS(2) [],
    ],
    pub PatternGenSelectW [
        OFFSET_EQ OFFSET(8) NUMBITS(24) [],
        AUD_RATE_SEL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AudPatternSelect1R [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PATTERN OFFSET(0) NUMBITS(2) [],
    ],
    pub AudPatternSelect1W [
        PATTERN OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AudPatternSelect2R [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PATTERN OFFSET(0) NUMBITS(2) [],
    ],
    pub AudPatternSelect2W [
        PATTERN OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufAudVidClkSourceR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        VID_TIMING_SRC OFFSET(2) NUMBITS(1) [],
        AUD_CLK_SRC OFFSET(1) NUMBITS(1) [],
        VID_CLK_SRC OFFSET(0) NUMBITS(1) [],
    ],
    pub AvBufAudVidClkSourceW [
        VID_TIMING_SRC OFFSET(2) NUMBITS(1) [],
        AUD_CLK_SRC OFFSET(1) NUMBITS(1) [],
        VID_CLK_SRC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufSrstRegR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        VID_RST OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ],
    pub AvBufSrstRegW [
        VID_RST OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufAudioRdyInterval [
        CH1_INT OFFSET(16) NUMBITS(16) [],
        CH0_INT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufAudioChConfigR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        AUD_CH_ID OFFSET(0) NUMBITS(2) [],
    ],
    pub AvBufAudioChConfigW [
        AUD_CH_ID OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufGraphicsComp0ScaleFactorR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        GRAPHICS_SCALE_FACTOR0 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufGraphicsComp0ScaleFactorW [
        GRAPHICS_SCALE_FACTOR0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufGraphicsComp1ScaleFactorR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        GRAPHICS_SCALE_FACTOR1 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufGraphicsComp1ScaleFactorW [
        GRAPHICS_SCALE_FACTOR1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufGraphicsComp2ScaleFactorR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        GRAPHICS_SCALE_FACTOR2 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufGraphicsComp2ScaleFactorW [
        GRAPHICS_SCALE_FACTOR2 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufVideoComp0ScaleFactorR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        VID_SCA_FACT0 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufVideoComp0ScaleFactorW [
        VID_SCA_FACT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufVideoComp1ScaleFactorR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        VID_SCA_FACT1 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufVideoComp1ScaleFactorW [
        VID_SCA_FACT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufVideoComp2ScaleFactorR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        VID_SCA_FACT2 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufVideoComp2ScaleFactorW [
        VID_SCA_FACT2 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufLiveVideoComp0SfR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        LIV_VID_SCA_FACT0 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufLiveVideoComp0SfW [
        LIV_VID_SCA_FACT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufLiveVideoComp1SfR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        LIV_VID_SCA_FACT1 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufLiveVideoComp1SfW [
        LIV_VID_SCA_FACT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufLiveVideoComp2SfR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        LIV_VID_SCA_FACT2 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufLiveVideoComp2SfW [
        LIV_VID_SCA_FACT2 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufLiveVidConfigR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        CB_FIRST OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(2) [],
        FORMAT OFFSET(4) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(1) [],
        BPC OFFSET(0) NUMBITS(3) [],
    ],
    pub AvBufLiveVidConfigW [
        CB_FIRST OFFSET(8) NUMBITS(1) [],
        FORMAT OFFSET(4) NUMBITS(2) [],
        BPC OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufLiveGfxComp0SfR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        LIV_VID_SCA_FACT0 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufLiveGfxComp0SfW [
        LIV_VID_SCA_FACT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufLiveGfxComp1SfR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        LIV_VID_SCA_FACT1 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufLiveGfxComp1SfW [
        LIV_VID_SCA_FACT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufLiveGfxComp2SfR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        LIV_VID_SCA_FACT2 OFFSET(0) NUMBITS(17) [],
    ],
    pub AvBufLiveGfxComp2SfW [
        LIV_VID_SCA_FACT2 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AvBufLiveGfxConfigR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        CB_FIRST OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(2) [],
        FORMAT OFFSET(4) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(1) [],
        BPC OFFSET(0) NUMBITS(3) [],
    ],
    pub AvBufLiveGfxConfigW [
        CB_FIRST OFFSET(8) NUMBITS(1) [],
        FORMAT OFFSET(4) NUMBITS(2) [],
        BPC OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AudioMixerVolumeControl [
        VOL_CTRL_CH1 OFFSET(16) NUMBITS(16) [],
        VOL_CTRL_CH0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AudioMixerMetaDataR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        AUD_META_DATA_SEL OFFSET(0) NUMBITS(1) [],
    ],
    pub AudioMixerMetaDataW [
        AUD_META_DATA_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AudioSoftResetR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        EXTRA_BS_CONTROL OFFSET(2) NUMBITS(1) [],
        LINE_RESET_DISABLE OFFSET(1) NUMBITS(1) [],
        AUDIO_SRST OFFSET(0) NUMBITS(1) [],
    ],
    pub AudioSoftResetW [
        EXTRA_BS_CONTROL OFFSET(2) NUMBITS(1) [],
        LINE_RESET_DISABLE OFFSET(1) NUMBITS(1) [],
        AUDIO_SRST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PatgenCrcR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CRC_R OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PatgenCrcG [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CRC_G OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PatgenCrcB [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CRC_B OFFSET(0) NUMBITS(16) [],
    ]
];

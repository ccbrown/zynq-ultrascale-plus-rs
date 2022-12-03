// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Graphics Processing Unit, Graphics Processing Unit
pub static mut GPU: *mut Registers = 0xfd4b0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// GP ControlRegister VSCL Start Address
    pub gp_contr_reg_vscl_start_addr: ReadWrite<u32, GpContrRegVsclStartAddr::Register>,
    /// GP Control Register VSCL End Address
    pub gp_contr_reg_vscl_end_addr: ReadWrite<u32, GpContrRegVsclEndAddr::Register>,
    /// GP Control Register PLBCL Start Address
    pub gp_contr_reg_plbcl_start_addr: ReadWrite<u32, GpContrRegPlbclStartAddr::Register>,
    /// GP Control Register PLBCL End Address
    pub gp_contr_reg_plbcl_end_addr: ReadWrite<u32, GpContrRegPlbclEndAddr::Register>,
    /// GP Control Register PLB Allocate Start Address
    pub gp_contr_reg_plb_alloc_start_addr: ReadWrite<u32, GpContrRegPlbAllocStartAddr::Register>,
    /// GP Control Register PLB Allocate End Address
    pub gp_contr_reg_plb_alloc_end_addr: ReadWrite<u32, GpContrRegPlbAllocEndAddr::Register>,
    _padding24: [u8; 8],
    /// GP Control Register Command
    pub gp_contr_reg_cmd: WriteOnly<u32, GpContrRegCmd::Register>,
    /// GP Control Register Interrupt Rawstat
    pub gp_contr_reg_int_rawstat: ReadWrite<u32, GpContrRegIntRawstat::Register>,
    /// GP Control Register Interrupt Clear
    pub gp_contr_reg_int_clear: WriteOnly<u32, GpContrRegIntClear::Register>,
    /// GP Control Register Interrupt Mask
    pub gp_contr_reg_int_mask: WriteOnly<u32, GpContrRegIntMask::Register>,
    /// GP Control Register Interrupt Status
    pub gp_contr_reg_int_stat: ReadOnly<u32, GpContrRegIntStat::Register>,
    /// GP Control Register Write Boundary Low
    pub gp_contr_reg_write_bound_low: ReadWrite<u32, GpContrRegWriteBoundLow::Register>,
    /// GP Control Register Write Boundary High
    pub gp_contr_reg_write_bound_high: ReadWrite<u32, GpContrRegWriteBoundHigh::Register>,
    /// GP Control Register Performance Counter 0 Enable
    pub gp_contr_reg_perf_cnt_0_enable: ReadWrite<u32, GpContrRegPerfCnt0Enable::Register>,
    /// GP Control Register Performance Counter 1 Enable
    pub gp_contr_reg_perf_cnt_1_enable: ReadWrite<u32, GpContrRegPerfCnt1Enable::Register>,
    /// GP Control Register Performance Counter 0 Source
    pub gp_contr_reg_perf_cnt_0_src: ReadWrite<u32, GpContrRegPerfCnt0Src::Register>,
    /// GP Control Register Performance Counter 1 Source
    pub gp_contr_reg_perf_cnt_1_src: ReadWrite<u32, GpContrRegPerfCnt1Src::Register>,
    /// GP Control Register Performance Counter 0 Value
    pub gp_contr_reg_perf_cnt_0_val: ReadOnly<u32>,
    /// GP Control Register Performance Counter 1 Value
    pub gp_contr_reg_perf_cnt_1_val: ReadOnly<u32>,
    /// GP Control Register Performance Counter 0 Limit
    pub gp_contr_reg_perf_cnt_0_limit: ReadWrite<u32>,
    /// GP Control Register Performance Control 1 Limit
    pub gp_contr_reg_perf_cnt_1_limit: ReadWrite<u32>,
    _padding92: [u8; 12],
    /// GP Control Register Status
    pub gp_contr_reg_status: ReadOnly<u32, GpContrRegStatus::Register>,
    /// GP Control Register VERSION
    pub gp_contr_reg_version: ReadOnly<u32, GpContrRegVersion::Register>,
    _padding112: [u8; 16],
    /// GP Control Register VSCL Initial Address
    pub gp_contr_reg_vscl_initial_addr: ReadOnly<u32, GpContrRegVsclInitialAddr::Register>,
    /// GP Control Register PLBCL Initial Address
    pub gp_contr_reg_plbcl_initial_addr: ReadOnly<u32, GpContrRegPlbclInitialAddr::Register>,
    /// GP Control Register Write Error Address
    pub gp_contr_reg_write_boundary_error_addr: ReadOnly<u32>,
    _padding140: [u8; 8],
    /// GP Control AXI Bus Error Status
    pub gp_contr_reg_axi_bus_error_stat: ReadOnly<u32, GpContrRegAxiBusErrorStat::Register>,
    _padding152: [u8; 8],
    /// GP Control Register Watchdog Disable
    pub gp_contr_reg_watchdog_disable: ReadWrite<u32, GpContrRegWatchdogDisable::Register>,
    /// GP Control Register Watchdog Timeout
    pub gp_contr_reg_watchdog_timeout: ReadWrite<u32>,
    _padding168: [u8; 3928],
    /// VERSION Register
    pub version: ReadOnly<u32, Version::Register>,
    /// SIZE Register
    pub size: ReadOnly<u32, Size::Register>,
    /// Status Register
    pub status: ReadOnly<u32, Status::Register>,
    _padding4108: [u8; 4],
    /// Command Register
    pub command: WriteOnly<u32, Command::Register>,
    /// Clear Page Register
    pub clear_page: WriteOnly<u32>,
    /// Maximum Reads Register
    pub max_reads: ReadWrite<u32, MaxReads::Register>,
    /// Enable Register
    pub enable: ReadWrite<u32, Enable::Register>,
    /// Performance Counter 0 Source Register
    pub perfcnt_src0: ReadWrite<u32, PerfcntSrc0::Register>,
    /// Performance Counter 0 Value Register
    pub perfcnt_val0: ReadWrite<u32>,
    /// Performance Counter 1 Source Register
    pub perfcnt_src1: ReadWrite<u32, PerfcntSrc1::Register>,
    /// Performance Counter 1 Value Register
    pub perfcnt_val1: ReadWrite<u32>,
    _padding4144: [u8; 8144],
    /// MMU Current Page Table Address Register
    pub gp_mmu_dte_addr: ReadWrite<u32>,
    /// MMU Status Register
    pub gp_mmu_status: ReadOnly<u32, GpMmuStatus::Register>,
    /// MMU Command Register
    pub gp_mmu_command: WriteOnly<u32, GpMmuCommand::Register>,
    /// MMU Logical Address
    pub gp_mmu_page_fault_addr: ReadOnly<u32>,
    /// MMU Zap Cache Line Register
    pub gp_mmu_zap_one_line: WriteOnly<u32>,
    /// MMU Raw Interrupt Status Register
    pub gp_mmu_int_rawstat: ReadWrite<u32, GpMmuIntRawstat::Register>,
    /// MMU Interrupt Clear Register
    pub gp_mmu_int_clear: WriteOnly<u32, GpMmuIntClear::Register>,
    /// MMU Interrupt Mask Register
    pub gp_mmu_int_mask: ReadWrite<u32, GpMmuIntMask::Register>,
    /// MMU Interrupt Status Register
    pub gp_mmu_int_status: ReadOnly<u32, GpMmuIntStatus::Register>,
    _padding12324: [u8; 4060],
    /// MMU Current Page Table Address Register
    pub pp0_mmu_dte_addr: ReadWrite<u32>,
    /// MMU Status Register
    pub pp0_mmu_status: ReadOnly<u32, Pp0MmuStatus::Register>,
    /// MMU Command Register
    pub pp0_mmu_command: WriteOnly<u32, Pp0MmuCommand::Register>,
    /// MMU Logical Address
    pub pp0_mmu_page_fault_addr: ReadOnly<u32>,
    /// MMU Zap Cache Line Register
    pub pp0_mmu_zap_one_line: WriteOnly<u32>,
    /// MMU Raw Interrupt Status Register
    pub pp0_mmu_int_rawstat: ReadWrite<u32, Pp0MmuIntRawstat::Register>,
    /// MMU Interrupt Clear Register
    pub pp0_mmu_int_clear: WriteOnly<u32, Pp0MmuIntClear::Register>,
    /// MMU Interrupt Mask Register
    pub pp0_mmu_int_mask: ReadWrite<u32, Pp0MmuIntMask::Register>,
    /// MMU Interrupt Status Register
    pub pp0_mmu_int_status: ReadOnly<u32, Pp0MmuIntStatus::Register>,
    _padding16420: [u8; 4060],
    /// MMU Current Page Table Address Register
    pub pp1_mmu_dte_addr: ReadWrite<u32>,
    /// MMU Status Register
    pub pp1_mmu_status: ReadOnly<u32, Pp1MmuStatus::Register>,
    /// MMU Command Register
    pub pp1_mmu_command: WriteOnly<u32, Pp1MmuCommand::Register>,
    /// MMU Logical Address
    pub pp1_mmu_page_fault_addr: ReadOnly<u32>,
    /// MMU Zap Cache Line Register
    pub pp1_mmu_zap_one_line: WriteOnly<u32>,
    /// MMU Raw Interrupt Status Register
    pub pp1_mmu_int_rawstat: ReadWrite<u32, Pp1MmuIntRawstat::Register>,
    /// MMU Interrupt Clear Register
    pub pp1_mmu_int_clear: WriteOnly<u32, Pp1MmuIntClear::Register>,
    /// MMU Interrupt Mask Register
    pub pp1_mmu_int_mask: ReadWrite<u32, Pp1MmuIntMask::Register>,
    /// MMU Interrupt Status Register
    pub pp1_mmu_int_status: ReadOnly<u32, Pp1MmuIntStatus::Register>,
    _padding20516: [u8; 12252],
    /// Renderer List Address Register
    pub pp0_rend_list_addr: ReadWrite<u32, Pp0RendListAddr::Register>,
    /// Renderer State Word Base Address Register
    pub pp0_rend_rsw_base: ReadWrite<u32, Pp0RendRswBase::Register>,
    /// Renderer Vertex Base Register
    pub pp0_rend_vertex_base: ReadWrite<u32, Pp0RendVertexBase::Register>,
    /// Feature Enable Register
    pub pp0_feature_enable: ReadWrite<u32, Pp0FeatureEnable::Register>,
    /// Z Clear Value Register
    pub pp0_z_clear_value: ReadWrite<u32, Pp0ZClearValue::Register>,
    /// Stencil Clear Value Register
    pub pp0_stencil_clear_value: ReadWrite<u32, Pp0StencilClearValue::Register>,
    /// ABGR Clear Value 0 Register
    pub pp0_abgr_clear_value_0: ReadWrite<u32, Pp0AbgrClearValue0::Register>,
    /// ABGR Clear Value 1 Register
    pub pp0_abgr_clear_value_1: ReadWrite<u32, Pp0AbgrClearValue1::Register>,
    /// ABGR Clear Value 2 Register
    pub pp0_abgr_clear_value_2: ReadWrite<u32, Pp0AbgrClearValue2::Register>,
    /// ABGR Clear Value 3 Register
    pub pp0_abgr_clear_value_3: ReadWrite<u32, Pp0AbgrClearValue3::Register>,
    /// Bounding Box Left Right Register
    pub pp0_bounding_box_left_right: ReadWrite<u32, Pp0BoundingBoxLeftRight::Register>,
    /// Bounding Box Bottom Register
    pub pp0_bounding_box_bottom: ReadWrite<u32, Pp0BoundingBoxBottom::Register>,
    /// FS Stack Address Register
    pub pp0_fs_stack_addr: ReadWrite<u32, Pp0FsStackAddr::Register>,
    /// FS Stack Size and Initial Value Register
    pub pp0_fs_stack_size_and_init_val: ReadWrite<u32, Pp0FsStackSizeAndInitVal::Register>,
    _padding32824: [u8; 8],
    /// Origin Offset X Register
    pub pp0_origin_offset_x: ReadWrite<u32, Pp0OriginOffsetX::Register>,
    /// Origin Offset Y Register
    pub pp0_origin_offset_y: ReadWrite<u32, Pp0OriginOffsetY::Register>,
    /// Subpixel Specifier Register
    pub pp0_subpixel_specifier: ReadWrite<u32, Pp0SubpixelSpecifier::Register>,
    /// Tiebreak mode Register
    pub pp0_tiebreak_mode: ReadWrite<u32, Pp0TiebreakMode::Register>,
    /// Polygon List Format Register
    pub pp0_plist_config: ReadWrite<u32, Pp0PlistConfig::Register>,
    /// Scaling Register
    pub pp0_scaling_config: ReadWrite<u32, Pp0ScalingConfig::Register>,
    /// Tilebuffer configuration Register
    pub pp0_tilebuffer_bits: ReadWrite<u32, Pp0TilebufferBits::Register>,
    _padding32860: [u8; 164],
    /// WB0 Source Select Register
    pub pp0_wb0_source_select: ReadWrite<u32, Pp0Wb0SourceSelect::Register>,
    /// WB0 Target Address Register
    pub pp0_wb0_target_addr: ReadWrite<u32, Pp0Wb0TargetAddr::Register>,
    /// WB0 Target Pixel Format Register
    pub pp0_wb0_target_pixel_format: ReadWrite<u32, Pp0Wb0TargetPixelFormat::Register>,
    /// WB0 Target AA Format Register
    pub pp0_wb0_target_aa_format: ReadWrite<u32, Pp0Wb0TargetAaFormat::Register>,
    /// WB0 Target Layout
    pub pp0_wb0_target_layout: ReadWrite<u32, Pp0Wb0TargetLayout::Register>,
    /// WB0 Target Scanline Length
    pub pp0_wb0_target_scanline_length: ReadWrite<u32, Pp0Wb0TargetScanlineLength::Register>,
    /// WB0 Target Flags Register
    pub pp0_wb0_target_flags: ReadWrite<u32, Pp0Wb0TargetFlags::Register>,
    /// WB0 MRT Enable Register
    pub pp0_wb0_mrt_enable: ReadWrite<u32, Pp0Wb0MrtEnable::Register>,
    /// WB0 MRT Offset Register
    pub pp0_wb0_mrt_offset: ReadWrite<u32, Pp0Wb0MrtOffset::Register>,
    /// WB0 Global Test Enable Register
    pub pp0_wb0_global_test_enable: ReadWrite<u32, Pp0Wb0GlobalTestEnable::Register>,
    /// WB0 Global Test Reference Value Register
    pub pp0_wb0_global_test_ref_value: ReadWrite<u32, Pp0Wb0GlobalTestRefValue::Register>,
    /// WB0 Global Test Compare Function Register
    pub pp0_wb0_global_test_cmp_func: ReadWrite<u32, Pp0Wb0GlobalTestCmpFunc::Register>,
    _padding33072: [u8; 208],
    /// WB1 Source Select Register
    pub pp0_wb1_source_select: ReadWrite<u32, Pp0Wb1SourceSelect::Register>,
    /// WB1 Target Address Register
    pub pp0_wb1_target_addr: ReadWrite<u32, Pp0Wb1TargetAddr::Register>,
    /// WB1 Target Pixel Format Register
    pub pp0_wb1_target_pixel_format: ReadWrite<u32, Pp0Wb1TargetPixelFormat::Register>,
    /// WB1 Target AA Format Register
    pub pp0_wb1_target_aa_format: ReadWrite<u32, Pp0Wb1TargetAaFormat::Register>,
    /// WB1 Target Layout
    pub pp0_wb1_target_layout: ReadWrite<u32, Pp0Wb1TargetLayout::Register>,
    /// WB1 Target Scanline Length
    pub pp0_wb1_target_scanline_length: ReadWrite<u32, Pp0Wb1TargetScanlineLength::Register>,
    /// WB1 Target Flags Register
    pub pp0_wb1_target_flags: ReadWrite<u32, Pp0Wb1TargetFlags::Register>,
    /// WB1 MRT Enable Register
    pub pp0_wb1_mrt_enable: ReadWrite<u32, Pp0Wb1MrtEnable::Register>,
    /// WB1 MRT Offset Register
    pub pp0_wb1_mrt_offset: ReadWrite<u32, Pp0Wb1MrtOffset::Register>,
    /// WB1 Global Test Enable Register
    pub pp0_wb1_global_test_enable: ReadWrite<u32, Pp0Wb1GlobalTestEnable::Register>,
    /// WB1 Global Test Reference Value Register
    pub pp0_wb1_global_test_ref_value: ReadWrite<u32, Pp0Wb1GlobalTestRefValue::Register>,
    /// WB1 Global Test Compare Function Register
    pub pp0_wb1_global_test_cmp_func: ReadWrite<u32, Pp0Wb1GlobalTestCmpFunc::Register>,
    _padding33328: [u8; 208],
    /// WB2 Source Select Register
    pub pp0_wb2_source_select: ReadWrite<u32, Pp0Wb2SourceSelect::Register>,
    /// WB2 Target Address Register
    pub pp0_wb2_target_addr: ReadWrite<u32, Pp0Wb2TargetAddr::Register>,
    /// WB2 Target Pixel Format Register
    pub pp0_wb2_target_pixel_format: ReadWrite<u32, Pp0Wb2TargetPixelFormat::Register>,
    /// WB2 Target AA Format Register
    pub pp0_wb2_target_aa_format: ReadWrite<u32, Pp0Wb2TargetAaFormat::Register>,
    /// WB2 Target Layout
    pub pp0_wb2_target_layout: ReadWrite<u32, Pp0Wb2TargetLayout::Register>,
    /// WB2 Target Scanline Length
    pub pp0_wb2_target_scanline_length: ReadWrite<u32, Pp0Wb2TargetScanlineLength::Register>,
    /// WB2 Target Flags Register
    pub pp0_wb2_target_flags: ReadWrite<u32, Pp0Wb2TargetFlags::Register>,
    /// WB2 MRT Enable Register
    pub pp0_wb2_mrt_enable: ReadWrite<u32, Pp0Wb2MrtEnable::Register>,
    /// WB2 MRT Offset Register
    pub pp0_wb2_mrt_offset: ReadWrite<u32, Pp0Wb2MrtOffset::Register>,
    /// WB2 Global Test Enable Register
    pub pp0_wb2_global_test_enable: ReadWrite<u32, Pp0Wb2GlobalTestEnable::Register>,
    /// WB2 Global Test Reference Value Register
    pub pp0_wb2_global_test_ref_value: ReadWrite<u32, Pp0Wb2GlobalTestRefValue::Register>,
    /// WB2 Global Test Compare Function Register
    pub pp0_wb2_global_test_cmp_func: ReadWrite<u32, Pp0Wb2GlobalTestCmpFunc::Register>,
    _padding33584: [u8; 3280],
    /// Version Register
    pub pp0_version: ReadOnly<u32, Pp0Version::Register>,
    /// Current Renderer List Address Register
    pub pp0_current_rend_list_addr: ReadWrite<u32, Pp0CurrentRendListAddr::Register>,
    /// Pixel Processor Status Register
    pub pp0_status: ReadWrite<u32, Pp0Status::Register>,
    /// Control Management Register
    pub pp0_ctrl_mgmt: WriteOnly<u32, Pp0CtrlMgmt::Register>,
    /// Last Tile Where Processing Started Register
    pub pp0_last_tile_pos_start: ReadOnly<u32, Pp0LastTilePosStart::Register>,
    /// Last Tile Where Processing Completed Register
    pub pp0_last_tile_pos_end: ReadOnly<u32, Pp0LastTilePosEnd::Register>,
    _padding36888: [u8; 8],
    /// Interrupt Rawstat Register
    pub pp0_int_rawstat: ReadWrite<u32, Pp0IntRawstat::Register>,
    /// Interrupt Clear Register
    pub pp0_int_clear: WriteOnly<u32, Pp0IntClear::Register>,
    /// Interrupt Mask Register
    pub pp0_int_mask: ReadWrite<u32, Pp0IntMask::Register>,
    /// Interrupt Status Register
    pub pp0_int_status: ReadWrite<u32, Pp0IntStatus::Register>,
    _padding36912: [u8; 16],
    /// Write Boundary Enable Register
    pub pp0_write_boundary_enable: ReadWrite<u32, Pp0WriteBoundaryEnable::Register>,
    /// Write Boundary Low Register
    pub pp0_write_boundary_low: ReadWrite<u32, Pp0WriteBoundaryLow::Register>,
    /// Write Boundary High Register
    pub pp0_write_boundary_high: ReadWrite<u32, Pp0WriteBoundaryHigh::Register>,
    /// Write Boundary Address Register
    pub pp0_write_boundary_address: ReadWrite<u32, Pp0WriteBoundaryAddress::Register>,
    /// Bus Error Status Register
    pub pp0_bus_error_status: ReadWrite<u32, Pp0BusErrorStatus::Register>,
    _padding36948: [u8; 12],
    /// Watchdog Disable Register
    pub pp0_watchdog_disable: ReadWrite<u32, Pp0WatchdogDisable::Register>,
    /// Watchdog Timeout Register
    pub pp0_watchdog_timeout: ReadWrite<u32>,
    _padding36968: [u8; 24],
    /// Performance Counter 0 Enable Register
    pub pp0_perf_cnt_0_enable: ReadWrite<u32, Pp0PerfCnt0Enable::Register>,
    /// Performance Counter 0 SRC Register
    pub pp0_perf_cnt_0_src: ReadWrite<u32, Pp0PerfCnt0Src::Register>,
    /// Performance Counter 0 Limit Register
    pub pp0_perf_cnt_0_limit: ReadWrite<u32>,
    /// Performance Counter 0 Value Register
    pub pp0_perf_cnt_0_value: ReadWrite<u32>,
    _padding37008: [u8; 16],
    /// Performance Counter 0 Enable Register
    pub pp0_perf_cnt_1_enable: ReadWrite<u32, Pp0PerfCnt1Enable::Register>,
    /// Performance Counter 1 SRC Register
    pub pp0_perf_cnt_1_src: ReadWrite<u32, Pp0PerfCnt1Src::Register>,
    /// Performance Counter 1 Limit Register
    pub pp0_perf_cnt_1_limit: ReadWrite<u32>,
    /// Performance Counter 1 Value Register
    pub pp0_perf_cnt_1_value: ReadWrite<u32, Pp0PerfCnt1Value::Register>,
    /// Performance Monitor Control Register
    pub pp0_perfmon_contr: ReadWrite<u32, Pp0PerfmonContr::Register>,
    /// Performance Monitor Base Address Register
    pub pp0_perfmon_base: ReadWrite<u32, Pp0PerfmonBase::Register>,
    _padding37048: [u8; 3912],
    /// Renderer List Address Register
    pub pp1_rend_list_addr: ReadWrite<u32, Pp1RendListAddr::Register>,
    /// Renderer State Word Base Address Register
    pub pp1_rend_rsw_base: ReadWrite<u32, Pp1RendRswBase::Register>,
    /// Renderer Vertex Base Register
    pub pp1_rend_vertex_base: ReadWrite<u32, Pp1RendVertexBase::Register>,
    /// Feature Enable Register
    pub pp1_feature_enable: ReadWrite<u32, Pp1FeatureEnable::Register>,
    /// Z Clear Value Register
    pub pp1_z_clear_value: ReadWrite<u32, Pp1ZClearValue::Register>,
    /// Stencil Clear Value Register
    pub pp1_stencil_clear_value: ReadWrite<u32, Pp1StencilClearValue::Register>,
    /// ABGR Clear Value 0 Register
    pub pp1_abgr_clear_value_0: ReadWrite<u32, Pp1AbgrClearValue0::Register>,
    /// ABGR Clear Value 1 Register
    pub pp1_abgr_clear_value_1: ReadWrite<u32, Pp1AbgrClearValue1::Register>,
    /// ABGR Clear Value 2 Register
    pub pp1_abgr_clear_value_2: ReadWrite<u32, Pp1AbgrClearValue2::Register>,
    /// ABGR Clear Value 3 Register
    pub pp1_abgr_clear_value_3: ReadWrite<u32, Pp1AbgrClearValue3::Register>,
    /// Bounding Box Left Right Register
    pub pp1_bounding_box_left_right: ReadWrite<u32, Pp1BoundingBoxLeftRight::Register>,
    /// Bounding Box Bottom Register
    pub pp1_bounding_box_bottom: ReadWrite<u32, Pp1BoundingBoxBottom::Register>,
    /// FS Stack Address Register
    pub pp1_fs_stack_addr: ReadWrite<u32, Pp1FsStackAddr::Register>,
    /// FS Stack Size and Initial Value Register
    pub pp1_fs_stack_size_and_init_val: ReadWrite<u32, Pp1FsStackSizeAndInitVal::Register>,
    _padding41016: [u8; 8],
    /// Origin Offset X Register
    pub pp1_origin_offset_x: ReadWrite<u32, Pp1OriginOffsetX::Register>,
    /// Origin Offset Y Register
    pub pp1_origin_offset_y: ReadWrite<u32, Pp1OriginOffsetY::Register>,
    /// Subpixel Specifier Register
    pub pp1_subpixel_specifier: ReadWrite<u32, Pp1SubpixelSpecifier::Register>,
    /// Tiebreak mode Register
    pub pp1_tiebreak_mode: ReadWrite<u32, Pp1TiebreakMode::Register>,
    /// Polygon List Format Register
    pub pp1_plist_config: ReadWrite<u32, Pp1PlistConfig::Register>,
    /// Scaling Register
    pub pp1_scaling_config: ReadWrite<u32, Pp1ScalingConfig::Register>,
    /// Tilebuffer configuration Register
    pub pp1_tilebuffer_bits: ReadWrite<u32, Pp1TilebufferBits::Register>,
    _padding41052: [u8; 164],
    /// WB0 Source Select Register
    pub pp1_wb0_source_select: ReadWrite<u32, Pp1Wb0SourceSelect::Register>,
    /// WB0 Target Address Register
    pub pp1_wb0_target_addr: ReadWrite<u32, Pp1Wb0TargetAddr::Register>,
    /// WB0 Target Pixel Format Register
    pub pp1_wb0_target_pixel_format: ReadWrite<u32, Pp1Wb0TargetPixelFormat::Register>,
    /// WB0 Target AA Format Register
    pub pp1_wb0_target_aa_format: ReadWrite<u32, Pp1Wb0TargetAaFormat::Register>,
    /// WB0 Target Layout
    pub pp1_wb0_target_layout: ReadWrite<u32, Pp1Wb0TargetLayout::Register>,
    /// WB0 Target Scanline Length
    pub pp1_wb0_target_scanline_length: ReadWrite<u32, Pp1Wb0TargetScanlineLength::Register>,
    /// WB0 Target Flags Register
    pub pp1_wb0_target_flags: ReadWrite<u32, Pp1Wb0TargetFlags::Register>,
    /// WB0 MRT Enable Register
    pub pp1_wb0_mrt_enable: ReadWrite<u32, Pp1Wb0MrtEnable::Register>,
    /// WB0 MRT Offset Register
    pub pp1_wb0_mrt_offset: ReadWrite<u32, Pp1Wb0MrtOffset::Register>,
    /// WB0 Global Test Enable Register
    pub pp1_wb0_global_test_enable: ReadWrite<u32, Pp1Wb0GlobalTestEnable::Register>,
    /// WB0 Global Test Reference Value Register
    pub pp1_wb0_global_test_ref_value: ReadWrite<u32, Pp1Wb0GlobalTestRefValue::Register>,
    /// WB0 Global Test Compare Function Register
    pub pp1_wb0_global_test_cmp_func: ReadWrite<u32, Pp1Wb0GlobalTestCmpFunc::Register>,
    _padding41264: [u8; 208],
    /// WB1 Source Select Register
    pub pp1_wb1_source_select: ReadWrite<u32, Pp1Wb1SourceSelect::Register>,
    /// WB1 Target Address Register
    pub pp1_wb1_target_addr: ReadWrite<u32, Pp1Wb1TargetAddr::Register>,
    /// WB1 Target Pixel Format Register
    pub pp1_wb1_target_pixel_format: ReadWrite<u32, Pp1Wb1TargetPixelFormat::Register>,
    /// WB1 Target AA Format Register
    pub pp1_wb1_target_aa_format: ReadWrite<u32, Pp1Wb1TargetAaFormat::Register>,
    /// WB1 Target Layout
    pub pp1_wb1_target_layout: ReadWrite<u32, Pp1Wb1TargetLayout::Register>,
    /// WB1 Target Scanline Length
    pub pp1_wb1_target_scanline_length: ReadWrite<u32, Pp1Wb1TargetScanlineLength::Register>,
    /// WB1 Target Flags Register
    pub pp1_wb1_target_flags: ReadWrite<u32, Pp1Wb1TargetFlags::Register>,
    /// WB1 MRT Enable Register
    pub pp1_wb1_mrt_enable: ReadWrite<u32, Pp1Wb1MrtEnable::Register>,
    /// WB1 MRT Offset Register
    pub pp1_wb1_mrt_offset: ReadWrite<u32, Pp1Wb1MrtOffset::Register>,
    /// WB1 Global Test Enable Register
    pub pp1_wb1_global_test_enable: ReadWrite<u32, Pp1Wb1GlobalTestEnable::Register>,
    /// WB1 Global Test Reference Value Register
    pub pp1_wb1_global_test_ref_value: ReadWrite<u32, Pp1Wb1GlobalTestRefValue::Register>,
    /// WB1 Global Test Compare Function Register
    pub pp1_wb1_global_test_cmp_func: ReadWrite<u32, Pp1Wb1GlobalTestCmpFunc::Register>,
    _padding41520: [u8; 208],
    /// WB2 Source Select Register
    pub pp1_wb2_source_select: ReadWrite<u32, Pp1Wb2SourceSelect::Register>,
    /// WB2 Target Address Register
    pub pp1_wb2_target_addr: ReadWrite<u32, Pp1Wb2TargetAddr::Register>,
    /// WB2 Target Pixel Format Register
    pub pp1_wb2_target_pixel_format: ReadWrite<u32, Pp1Wb2TargetPixelFormat::Register>,
    /// WB2 Target AA Format Register
    pub pp1_wb2_target_aa_format: ReadWrite<u32, Pp1Wb2TargetAaFormat::Register>,
    /// WB2 Target Layout
    pub pp1_wb2_target_layout: ReadWrite<u32, Pp1Wb2TargetLayout::Register>,
    /// WB2 Target Scanline Length
    pub pp1_wb2_target_scanline_length: ReadWrite<u32, Pp1Wb2TargetScanlineLength::Register>,
    /// WB2 Target Flags Register
    pub pp1_wb2_target_flags: ReadWrite<u32, Pp1Wb2TargetFlags::Register>,
    /// WB2 MRT Enable Register
    pub pp1_wb2_mrt_enable: ReadWrite<u32, Pp1Wb2MrtEnable::Register>,
    /// WB2 MRT Offset Register
    pub pp1_wb2_mrt_offset: ReadWrite<u32, Pp1Wb2MrtOffset::Register>,
    /// WB2 Global Test Enable Register
    pub pp1_wb2_global_test_enable: ReadWrite<u32, Pp1Wb2GlobalTestEnable::Register>,
    /// WB2 Global Test Reference Value Register
    pub pp1_wb2_global_test_ref_value: ReadWrite<u32, Pp1Wb2GlobalTestRefValue::Register>,
    /// WB2 Global Test Compare Function Register
    pub pp1_wb2_global_test_cmp_func: ReadWrite<u32, Pp1Wb2GlobalTestCmpFunc::Register>,
    _padding41776: [u8; 3280],
    /// Version Register
    pub pp1_version: ReadWrite<u32, Pp1Version::Register>,
    /// Current Renderer List Address Register
    pub pp1_current_rend_list_addr: ReadWrite<u32, Pp1CurrentRendListAddr::Register>,
    /// Pixel Processor Status Register
    pub pp1_status: ReadWrite<u32, Pp1Status::Register>,
    /// Control Management Register
    pub pp1_ctrl_mgmt: ReadWrite<u32, Pp1CtrlMgmt::Register>,
    /// Last Tile Where Processing Started Register
    pub pp1_last_tile_pos_start: ReadWrite<u32, Pp1LastTilePosStart::Register>,
    /// Last Tile Where Processing Completed Register
    pub pp1_last_tile_pos_end: ReadWrite<u32, Pp1LastTilePosEnd::Register>,
    _padding45080: [u8; 8],
    /// Interrupt Rawstat Register
    pub pp1_int_rawstat: ReadWrite<u32, Pp1IntRawstat::Register>,
    /// Interrupt Clear Register
    pub pp1_int_clear: ReadWrite<u32, Pp1IntClear::Register>,
    /// Interrupt Mask Register
    pub pp1_int_mask: ReadWrite<u32, Pp1IntMask::Register>,
    /// Interrupt Status Register
    pub pp1_int_status: ReadWrite<u32, Pp1IntStatus::Register>,
    _padding45104: [u8; 16],
    /// Write Boundary Enable Register
    pub pp1_write_boundary_enable: ReadWrite<u32, Pp1WriteBoundaryEnable::Register>,
    /// Write Boundary Low Register
    pub pp1_write_boundary_low: ReadWrite<u32, Pp1WriteBoundaryLow::Register>,
    /// Write Boundary High Register
    pub pp1_write_boundary_high: ReadWrite<u32, Pp1WriteBoundaryHigh::Register>,
    /// Write Boundary Address Register
    pub pp1_write_boundary_address: ReadWrite<u32, Pp1WriteBoundaryAddress::Register>,
    /// Bus Error Status Register
    pub pp1_bus_error_status: ReadWrite<u32, Pp1BusErrorStatus::Register>,
    _padding45140: [u8; 12],
    /// Watchdog Disable Register
    pub pp1_watchdog_disable: ReadWrite<u32, Pp1WatchdogDisable::Register>,
    /// Watchdog Timeout Register
    pub pp1_watchdog_timeout: ReadWrite<u32>,
    _padding45160: [u8; 24],
    /// WB2 Global Test Compare Function Register
    pub pp1_perf_cnt_0_enable: ReadWrite<u32, Pp1PerfCnt0Enable::Register>,
    /// Performance Counter 0 SRC Register
    pub pp1_perf_cnt_0_src: ReadWrite<u32, Pp1PerfCnt0Src::Register>,
    /// Performance Counter 0 Limit Register
    pub pp1_perf_cnt_0_limit: ReadWrite<u32>,
    /// Performance Counter 0 Value Register
    pub pp1_perf_cnt_0_value: ReadWrite<u32>,
    _padding45200: [u8; 16],
    /// Performance Counter 1 Enable Register
    pub pp1_perf_cnt_1_enable: ReadWrite<u32, Pp1PerfCnt1Enable::Register>,
    /// Performance Counter 1 SRC Register
    pub pp1_perf_cnt_1_src: ReadWrite<u32, Pp1PerfCnt1Src::Register>,
    /// Performance Counter 1 Limit Register
    pub pp1_perf_cnt_1_limit: ReadWrite<u32>,
    /// Performance Counter 1 Value Register
    pub pp1_perf_cnt_1_value: ReadWrite<u32, Pp1PerfCnt1Value::Register>,
    /// Performance Monitor Control Register
    pub pp1_perfmon_contr: ReadWrite<u32, Pp1PerfmonContr::Register>,
    /// Performance Monitor Base Address Register
    pub pp1_perfmon_base: ReadWrite<u32, Pp1PerfmonBase::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegVsclStartAddr [
        GP_CONTR_REG_VSCL_START_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegVsclEndAddr [
        GP_CONTR_REG_VSCL_END_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegPlbclStartAddr [
        GP_CONTR_REG_PLBCL_START_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegPlbclEndAddr [
        GP_CONTR_REG_PLBCL_END_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegPlbAllocStartAddr [
        GP_CONTR_REG_PLB_ALLOC_START_ADDR OFFSET(7) NUMBITS(25) [],
        RESERVED0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegPlbAllocEndAddr [
        GP_CONTR_REG_PLB_ALLOC_END_ADDR OFFSET(7) NUMBITS(25) [],
        RESERVED0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegCmd [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        GP_CMD_CLK_OVERRIDE OFFSET(11) NUMBITS(1) [],
        GP_CMD_SOFT_RESET OFFSET(10) NUMBITS(1) [],
        GP_CMD_STOP_BUS OFFSET(9) NUMBITS(1) [],
        GP_CMD_START_BUS OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        GP_CMD_FORCE_HANG OFFSET(6) NUMBITS(1) [],
        GP_CMD_FORCE_RESET OFFSET(5) NUMBITS(1) [],
        GP_CMD_UPDATE_PLB_ALLOC OFFSET(4) NUMBITS(1) [],
        RESERVED2 OFFSET(2) NUMBITS(2) [],
        GP_CMD_START_PLB OFFSET(1) NUMBITS(1) [],
        GP_CMD_START_VS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegIntRawstat [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        GP_IRQ_PTR_OUT_OF_BOUNDS OFFSET(22) NUMBITS(1) [],
        GP_IRQ_SEMAPHORE_OVERFLOW OFFSET(21) NUMBITS(1) [],
        GP_IRQ_SEMAPHORE_UNDERFLOW OFFSET(20) NUMBITS(1) [],
        GP_IRQ_RESET_COMPLETED OFFSET(19) NUMBITS(1) [],
        RESERVED1 OFFSET(15) NUMBITS(4) [],
        GP_IRQ_PLB_INVALID_CMD OFFSET(14) NUMBITS(1) [],
        GP_IRQ_VS_INVALID_CMD OFFSET(13) NUMBITS(1) [],
        GP_IRQ_AXI_BUS_STOPPED OFFSET(12) NUMBITS(1) [],
        GP_IRQ_AXI_BUS_ERROR OFFSET(11) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(1) [],
        GP_IRQ_WRITE_BOUND_ERR OFFSET(9) NUMBITS(1) [],
        GP_IRQ_PERF_CNT_1_LIMIT OFFSET(8) NUMBITS(1) [],
        GP_IRQ_PERF_CNT_0_LIMIT OFFSET(7) NUMBITS(1) [],
        GP_IRQ_FORCED_HANG OFFSET(6) NUMBITS(1) [],
        GP_IRQ_HANG OFFSET(5) NUMBITS(1) [],
        GP_IRQ_PLB_SEM OFFSET(4) NUMBITS(1) [],
        GP_IRQ_VS_SEM OFFSET(3) NUMBITS(1) [],
        GP_IRQ_PLB_OUT_OF_MEM OFFSET(2) NUMBITS(1) [],
        GP_IRQ_PLB_END_CMD_LIST OFFSET(1) NUMBITS(1) [],
        GP_IRQ_VS_END_CMD_LIST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegIntClear [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        GP_IRQ_PTR_OUT_OF_BOUNDS OFFSET(22) NUMBITS(1) [],
        GP_IRQ_SEMAPHORE_OVERFLOW OFFSET(21) NUMBITS(1) [],
        GP_IRQ_SEMAPHORE_UNDERFLOW OFFSET(20) NUMBITS(1) [],
        GP_IRQ_RESET_COMPLETED OFFSET(19) NUMBITS(1) [],
        RESERVED1 OFFSET(15) NUMBITS(4) [],
        GP_IRQ_PLB_INVALID_CMD OFFSET(14) NUMBITS(1) [],
        GP_IRQ_VS_INVALID_CMD OFFSET(13) NUMBITS(1) [],
        GP_IRQ_AXI_BUS_STOPPED OFFSET(12) NUMBITS(1) [],
        GP_IRQ_AXI_BUS_ERROR OFFSET(11) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(1) [],
        GP_IRQ_WRITE_BOUND_ERR OFFSET(9) NUMBITS(1) [],
        GP_IRQ_PERF_CNT_1_LIMIT OFFSET(8) NUMBITS(1) [],
        GP_IRQ_PERF_CNT_0_LIMIT OFFSET(7) NUMBITS(1) [],
        GP_IRQ_FORCED_HANG OFFSET(6) NUMBITS(1) [],
        GP_IRQ_HANG OFFSET(5) NUMBITS(1) [],
        GP_IRQ_PLB_SEM OFFSET(4) NUMBITS(1) [],
        GP_IRQ_VS_SEM OFFSET(3) NUMBITS(1) [],
        GP_IRQ_PLB_OUT_OF_MEM OFFSET(2) NUMBITS(1) [],
        GP_IRQ_PLB_END_CMD_LIST OFFSET(1) NUMBITS(1) [],
        GP_IRQ_VS_END_CMD_LIST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegIntMask [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        GP_IRQ_PTR_OUT_OF_BOUNDS OFFSET(22) NUMBITS(1) [],
        GP_IRQ_SEMAPHORE_OVERFLOW OFFSET(21) NUMBITS(1) [],
        GP_IRQ_SEMAPHORE_UNDERFLOW OFFSET(20) NUMBITS(1) [],
        GP_IRQ_RESET_COMPLETED OFFSET(19) NUMBITS(1) [],
        _0 OFFSET(15) NUMBITS(4) [],
        GP_IRQ_PLB_INVALID_CMD OFFSET(14) NUMBITS(1) [],
        GP_IRQ_VS_INVALID_CMD OFFSET(13) NUMBITS(1) [],
        GP_IRQ_AXI_BUS_STOPPED OFFSET(12) NUMBITS(1) [],
        GP_IRQ_AXI_BUS_ERROR OFFSET(11) NUMBITS(1) [],
        RESERVED1 OFFSET(10) NUMBITS(1) [],
        GP_IRQ_WRITE_BOUND_ERR OFFSET(9) NUMBITS(1) [],
        GP_IRQ_PERF_CNT_1_LIMIT OFFSET(8) NUMBITS(1) [],
        GP_IRQ_PERF_CNT_0_LIMIT OFFSET(7) NUMBITS(1) [],
        GP_IRQ_FORCED_HANG OFFSET(6) NUMBITS(1) [],
        GP_IRQ_HANG OFFSET(5) NUMBITS(1) [],
        GP_IRQ_PLB_SEM OFFSET(4) NUMBITS(1) [],
        GP_IRQ_VS_SEM OFFSET(3) NUMBITS(1) [],
        GP_IRQ_PLB_OUT_OF_MEM OFFSET(2) NUMBITS(1) [],
        GP_IRQ_PLB_END_CMD_LIST OFFSET(1) NUMBITS(1) [],
        GP_IRQ_VS_END_CMD_LIST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegIntStat [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        GP_IRQ_PTR_OUT_OF_BOUNDS OFFSET(22) NUMBITS(1) [],
        GP_IRQ_SEMAPHORE_OVERFLOW OFFSET(21) NUMBITS(1) [],
        GP_IRQ_SEMAPHORE_UNDERFLOW OFFSET(20) NUMBITS(1) [],
        GP_IRQ_RESET_COMPLETED OFFSET(19) NUMBITS(1) [],
        RESERVED1 OFFSET(15) NUMBITS(4) [],
        GP_IRQ_PLB_INVALID_CMD OFFSET(14) NUMBITS(1) [],
        GP_IRQ_VS_INVALID_CMD OFFSET(13) NUMBITS(1) [],
        GP_IRQ_AXI_BUS_STOPPED OFFSET(12) NUMBITS(1) [],
        GP_IRQ_AXI_BUS_ERROR OFFSET(11) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(1) [],
        GP_IRQ_WRITE_BOUND_ERR OFFSET(9) NUMBITS(1) [],
        GP_IRQ_PERF_CNT_1_LIMIT OFFSET(8) NUMBITS(1) [],
        GP_IRQ_PERF_CNT_0_LIMIT OFFSET(7) NUMBITS(1) [],
        GP_IRQ_FORCED_HANG OFFSET(6) NUMBITS(1) [],
        GP_IRQ_HANG OFFSET(5) NUMBITS(1) [],
        GP_IRQ_PLB_SEM OFFSET(4) NUMBITS(1) [],
        GP_IRQ_VS_SEM OFFSET(3) NUMBITS(1) [],
        GP_IRQ_PLB_OUT_OF_MEM OFFSET(2) NUMBITS(1) [],
        GP_IRQ_PLB_END_CMD_LIST OFFSET(1) NUMBITS(1) [],
        GP_IRQ_VS_END_CMD_LIST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegWriteBoundLow [
        GP_CONTR_REG_WRITE_BOUND_LOW OFFSET(8) NUMBITS(24) [],
        RESERVED0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegWriteBoundHigh [
        GP_CONTR_REG_WRITE_BOUND_HIGH OFFSET(8) NUMBITS(24) [],
        RESERVED0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegPerfCnt0Enable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GP_CONTR_REG_PERF_CNT_0_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegPerfCnt1Enable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GP_CONTR_REG_PERF_CNT_1_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegPerfCnt0Src [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GP_CONTR_REG_PERF_CNT_0_SRC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegPerfCnt1Src [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GP_CONTR_REG_PERF_CNT_1_SRC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegStatus [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        CLK_OVERRIDE OFFSET(9) NUMBITS(1) [],
        GP_STATUS_WRITE_BOUND_ERR OFFSET(8) NUMBITS(1) [],
        GP_STATUS_HANG OFFSET(7) NUMBITS(1) [],
        GP_STATUS_BUS_ERROR OFFSET(6) NUMBITS(1) [],
        GP_STATUS_PLB_STALLED OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        GP_STATUS_PLB_ACTIVE OFFSET(3) NUMBITS(1) [],
        GP_STATUS_BUS_STOPPED OFFSET(2) NUMBITS(1) [],
        GP_STATUS_VS_ACTIVE OFFSET(1) NUMBITS(1) [],
        GP_STATUS_IRQ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegVersion [
        PRODUCT_ID OFFSET(16) NUMBITS(16) [],
        VERSION_MAJOR OFFSET(8) NUMBITS(8) [],
        VERSION_MINOR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegVsclInitialAddr [
        GP_CONTR_REG_VSCL_INITIAL_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegPlbclInitialAddr [
        GP_CONTR_REG_PLBCL_INITIAL_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegAxiBusErrorStat [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        GP_READ_ERROR_ID OFFSET(6) NUMBITS(4) [],
        GP_WRITE_ERROR_ID OFFSET(2) NUMBITS(4) [],
        GP_READ_ERROR OFFSET(1) NUMBITS(1) [],
        GP_WRITE_ERROR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpContrRegWatchdogDisable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        GP_CONTR_REG_WATCHDOG_DISABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Version [
        PRODUCT_ID OFFSET(16) NUMBITS(16) [],
        VERSION_MAJOR OFFSET(8) NUMBITS(8) [],
        VERSION_MINOR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Size [
        EXTERNAL_BUS_WIDTH OFFSET(24) NUMBITS(8) [],
        CACHE_SIZE OFFSET(16) NUMBITS(8) [],
        ASSOCIATIVITY OFFSET(8) NUMBITS(8) [],
        LINE_SIZE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Status [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        DATA_BUSY OFFSET(1) NUMBITS(1) [],
        COMMAND_BUSY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Command [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        COMMAND OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MaxReads [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        MAX_READS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Enable [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PERMIT_CACHE_READ_ALLOCATE OFFSET(1) NUMBITS(1) [],
        PERMIT_CACHEABLE_ACCESSES OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PerfcntSrc0 [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        PERFCNT OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PerfcntSrc1 [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        PERFCNT OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpMmuStatus [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        MMU_PAGE_FAULT_BUS_ID OFFSET(6) NUMBITS(5) [],
        MMU_PAGE_FAULT_IS_WRITE OFFSET(5) NUMBITS(1) [],
        MMU_REPLAY_BUFFER_EMPTY OFFSET(4) NUMBITS(1) [],
        MMU_IDLE OFFSET(3) NUMBITS(1) [],
        MMU_STALL_ACTIVE OFFSET(2) NUMBITS(1) [],
        MMU_PAGE_FAULT_ACTIVE OFFSET(1) NUMBITS(1) [],
        MMU_PAGING_ENABLED OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpMmuCommand [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpMmuIntRawstat [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpMmuIntClear [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpMmuIntMask [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpMmuIntStatus [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0MmuStatus [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        MMU_PAGE_FAULT_BUS_ID OFFSET(6) NUMBITS(5) [],
        MMU_PAGE_FAULT_IS_WRITE OFFSET(5) NUMBITS(1) [],
        MMU_REPLAY_BUFFER_EMPTY OFFSET(4) NUMBITS(1) [],
        MMU_IDLE OFFSET(3) NUMBITS(1) [],
        MMU_STALL_ACTIVE OFFSET(2) NUMBITS(1) [],
        MMU_PAGE_FAULT_ACTIVE OFFSET(1) NUMBITS(1) [],
        MMU_PAGING_ENABLED OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0MmuCommand [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0MmuIntRawstat [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0MmuIntClear [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0MmuIntMask [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0MmuIntStatus [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1MmuStatus [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        MMU_PAGE_FAULT_BUS_ID OFFSET(6) NUMBITS(5) [],
        MMU_PAGE_FAULT_IS_WRITE OFFSET(5) NUMBITS(1) [],
        MMU_REPLAY_BUFFER_EMPTY OFFSET(4) NUMBITS(1) [],
        MMU_IDLE OFFSET(3) NUMBITS(1) [],
        MMU_STALL_ACTIVE OFFSET(2) NUMBITS(1) [],
        MMU_PAGE_FAULT_ACTIVE OFFSET(1) NUMBITS(1) [],
        MMU_PAGING_ENABLED OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1MmuCommand [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        MMU_CMD OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1MmuIntRawstat [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1MmuIntClear [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1MmuIntMask [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1MmuIntStatus [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        READ_BUS_ERROR OFFSET(1) NUMBITS(1) [],
        PAGE_FAULT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0RendListAddr [
        REND_LIST_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0RendRswBase [
        REND_RSW_BASE OFFSET(6) NUMBITS(26) [],
        RESERVED0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0RendVertexBase [
        REND_VERTEX_BASE OFFSET(6) NUMBITS(26) [],
        RESERVED0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0FeatureEnable [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        SUMMATE_QUAD_COVER OFFSET(6) NUMBITS(1) [],
        ORIGIN_LOWER_LEFT OFFSET(5) NUMBITS(1) [],
        EARLYZ_DISABLE2 OFFSET(4) NUMBITS(1) [],
        EARLYZ_DISABLE1 OFFSET(3) NUMBITS(1) [],
        _0 OFFSET(2) NUMBITS(1) [],
        EARLYZ_ENABLE OFFSET(1) NUMBITS(1) [],
        FP_TILEBUF_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0ZClearValue [
        _0 OFFSET(24) NUMBITS(8) [],
        Z_CLEAR_VALUE OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0StencilClearValue [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        STENCIL_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0AbgrClearValue0 [
        ALPHA_CLEAR_VALUE OFFSET(24) NUMBITS(8) [],
        BLUE_CLEAR_VALUE OFFSET(16) NUMBITS(8) [],
        GREEN_CLEAR_VALUE OFFSET(8) NUMBITS(8) [],
        RED_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0AbgrClearValue1 [
        ALPHA_CLEAR_VALUE OFFSET(24) NUMBITS(8) [],
        BLUE_CLEAR_VALUE OFFSET(16) NUMBITS(8) [],
        GREEN_CLEAR_VALUE OFFSET(8) NUMBITS(8) [],
        RED_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0AbgrClearValue2 [
        ALPHA_CLEAR_VALUE OFFSET(24) NUMBITS(8) [],
        BLUE_CLEAR_VALUE OFFSET(16) NUMBITS(8) [],
        GREEN_CLEAR_VALUE OFFSET(8) NUMBITS(8) [],
        RED_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0AbgrClearValue3 [
        ALPHA_CLEAR_VALUE OFFSET(24) NUMBITS(8) [],
        BLUE_CLEAR_VALUE OFFSET(16) NUMBITS(8) [],
        GREEN_CLEAR_VALUE OFFSET(8) NUMBITS(8) [],
        RED_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0BoundingBoxLeftRight [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        BOUNDING_BOX_LEFT OFFSET(16) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        BOUNDING_BOX_RIGHT OFFSET(0) NUMBITS(14) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0BoundingBoxBottom [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        BOUNDING_BOX_BOTTOM OFFSET(0) NUMBITS(14) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0FsStackAddr [
        FS_STACK_ADDR OFFSET(6) NUMBITS(26) [],
        RESERVED0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0FsStackSizeAndInitVal [
        FS_STACK_INIT_VAL OFFSET(16) NUMBITS(16) [],
        FS_STACK_SIZE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0OriginOffsetX [
        _0 OFFSET(16) NUMBITS(16) [],
        ORIGIN_OFFSET_X OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0OriginOffsetY [
        _0 OFFSET(16) NUMBITS(16) [],
        ORIGIN_OFFSET_Y OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0SubpixelSpecifier [
        _0 OFFSET(8) NUMBITS(24) [],
        SUBPIXEL_SPECIFIER OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0TiebreakMode [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        TIEBREAK_MODE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0PlistConfig [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        LIST_FORMAT OFFSET(28) NUMBITS(2) [],
        _0 OFFSET(22) NUMBITS(6) [],
        SCALE_Y OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(6) NUMBITS(10) [],
        SCALE_X OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0ScalingConfig [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        SCALE_Y OFFSET(20) NUMBITS(3) [],
        RESERVED1 OFFSET(19) NUMBITS(1) [],
        SCALE_X OFFSET(16) NUMBITS(3) [],
        RESERVED2 OFFSET(12) NUMBITS(4) [],
        FLIP_DERIVATIVE_Y OFFSET(11) NUMBITS(1) [],
        FLIP_FRAGCOORD OFFSET(10) NUMBITS(1) [],
        FLIP_DITHERING_MATRIX OFFSET(9) NUMBITS(1) [],
        FLIP_POINT_SPRITES OFFSET(8) NUMBITS(1) [],
        _0 OFFSET(4) NUMBITS(4) [],
        DERIVATIVE_SCALE_ENABLE OFFSET(3) NUMBITS(1) [],
        FRAGCOORD_SCALE_ENABLE OFFSET(2) NUMBITS(1) [],
        DITHERING_SCALE_ENABLE OFFSET(1) NUMBITS(1) [],
        POINT_AND_LINE_SCALE_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0TilebufferBits [
        _0 OFFSET(16) NUMBITS(16) [],
        TILEBUFFER_A_BITS OFFSET(12) NUMBITS(4) [],
        TILEBUFFER_B_BITS OFFSET(8) NUMBITS(4) [],
        TILEBUFFER_G_BITS OFFSET(4) NUMBITS(4) [],
        TILEBUFFER_R_BITS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0SourceSelect [
        _0 OFFSET(2) NUMBITS(30) [],
        WB0_SOURCE_SELECT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0TargetAddr [
        WB0_TARGET_ADDR OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0TargetPixelFormat [
        _0 OFFSET(4) NUMBITS(28) [],
        WB0_TARGET_PIXEL_FORMAT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0TargetAaFormat [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        WB0_TARGET_AA_Y OFFSET(12) NUMBITS(3) [],
        RESERVED1 OFFSET(10) NUMBITS(2) [],
        WB0_TARGET_AA_X OFFSET(8) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        WB0_TARGET_AA_FORMAT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0TargetLayout [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        WB0_TARGET_LAYOUT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0TargetScanlineLength [
        _0 OFFSET(16) NUMBITS(16) [],
        WB0_TARGET_SCANLINE_LENGTH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0TargetFlags [
        _0 OFFSET(6) NUMBITS(26) [],
        WB0_BIG_ENDIAN OFFSET(5) NUMBITS(1) [],
        WB0_DITHER_ENABLE OFFSET(4) NUMBITS(1) [],
        WB0_INV_COMPONENT_ORDER_ENABLE OFFSET(3) NUMBITS(1) [],
        WB0_SWAP_RED_BLUE_ENABLE OFFSET(2) NUMBITS(1) [],
        WB0_BOUNDING_BOX_ENABLE OFFSET(1) NUMBITS(1) [],
        WB0_DIRTY_BIT_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0MrtEnable [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        WB0_MRT_ENABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0MrtOffset [
        WB0_MRT_OFFSET OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0GlobalTestEnable [
        _0 OFFSET(1) NUMBITS(31) [],
        WB0_GLOBAL_TEST_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0GlobalTestRefValue [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        _0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb0GlobalTestCmpFunc [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1SourceSelect [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        WB1_SOURCE_SELECT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1TargetAddr [
        WB1_TARGET_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1TargetPixelFormat [
        _0 OFFSET(4) NUMBITS(28) [],
        WB1_TARGET_PIXEL_FORMAT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1TargetAaFormat [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        WB1_TARGET_AA_Y OFFSET(12) NUMBITS(3) [],
        RESERVED1 OFFSET(10) NUMBITS(2) [],
        WB1_TARGET_AA_X OFFSET(8) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        WB1_TARGET_AA_FORMAT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1TargetLayout [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        WB1_TARGET_LAYOUT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1TargetScanlineLength [
        _0 OFFSET(16) NUMBITS(16) [],
        WB2_TARGET_SCANLINE_LENGTH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1TargetFlags [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        WB1_BIG_ENDIAN OFFSET(5) NUMBITS(1) [],
        WB1_DITHER_ENABLE OFFSET(4) NUMBITS(1) [],
        WB1_INV_COMPONENT_ORDER_ENABLE OFFSET(3) NUMBITS(1) [],
        WB1_SWAP_RED_BLUE_ENABLE OFFSET(2) NUMBITS(1) [],
        WB1_BOUNDING_BOX_ENABLE OFFSET(1) NUMBITS(1) [],
        WB1_DIRTY_BIT_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1MrtEnable [
        _0 OFFSET(4) NUMBITS(28) [],
        WB1_MRT_ENABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1MrtOffset [
        WB1_MRT_OFFSET OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1GlobalTestEnable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        WB1_GLOBAL_TEST_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1GlobalTestRefValue [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        _0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb1GlobalTestCmpFunc [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2SourceSelect [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        WB2_SOURCE_SELECT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2TargetAddr [
        WB2_TARGET_ADDR OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2TargetPixelFormat [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        WB2_TARGET_PIXEL_FORMAT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2TargetAaFormat [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        WB2_TARGET_AA_Y OFFSET(12) NUMBITS(3) [],
        RESERVED1 OFFSET(10) NUMBITS(2) [],
        WB2_TARGET_AA_X OFFSET(8) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        WB2_TARGET_AA_FORMAT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2TargetLayout [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        WB2_TARGET_LAYOUT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2TargetScanlineLength [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WB2_TARGET_SCANLINE_LENGTH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2TargetFlags [
        _0 OFFSET(6) NUMBITS(26) [],
        WB2_BIG_ENDIAN OFFSET(5) NUMBITS(1) [],
        WB2_DITHER_ENABLE OFFSET(4) NUMBITS(1) [],
        WB2_INV_COMPONENT_ORDER_ENABLE OFFSET(3) NUMBITS(1) [],
        WB2_SWAP_RED_BLUE_ENABLE OFFSET(2) NUMBITS(1) [],
        WB2_BOUNDING_BOX_ENABLE OFFSET(1) NUMBITS(1) [],
        WB2_DIRTY_BIT_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2MrtEnable [
        _0 OFFSET(4) NUMBITS(28) [],
        WB2_MRT_ENABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2MrtOffset [
        WB2_MRT_OFFSET OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2GlobalTestEnable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        WB2_GLOBAL_TEST_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2GlobalTestRefValue [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        _0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Wb2GlobalTestCmpFunc [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Version [
        PRODUCT_ID OFFSET(16) NUMBITS(16) [],
        VERSION_MAJOR OFFSET(8) NUMBITS(8) [],
        VERSION_MINOR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0CurrentRendListAddr [
        CURRENT_REND_LIST_ADDR OFFSET(5) NUMBITS(27) [],
        RESERVED0 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0Status [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        CLK_OVERRIDE OFFSET(7) NUMBITS(1) [],
        INTERRUPT_ASSERTED OFFSET(6) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(5) NUMBITS(1) [],
        BUS_STOPPED OFFSET(4) NUMBITS(1) [],
        BUS_ERROR OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        TILE_STOPPED OFFSET(1) NUMBITS(1) [],
        RENDERING_ACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0CtrlMgmt [
        _0 OFFSET(9) NUMBITS(23) [],
        CLK_OVERRIDE OFFSET(8) NUMBITS(1) [],
        SOFT_RESET OFFSET(7) NUMBITS(1) [],
        START_RENDERING OFFSET(6) NUMBITS(1) [],
        FORCE_RESET OFFSET(5) NUMBITS(1) [],
        FORCE_HANG OFFSET(4) NUMBITS(1) [],
        FLUSH_CACHES OFFSET(3) NUMBITS(1) [],
        END_AFTER_TILE OFFSET(2) NUMBITS(1) [],
        START_BUS OFFSET(1) NUMBITS(1) [],
        STOP_BUS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0LastTilePosStart [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        TILEY_START OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(8) NUMBITS(8) [],
        TILEX_START OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0LastTilePosEnd [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        TILEY_END OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(8) NUMBITS(8) [],
        TILEX_END OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0IntRawstat [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        RESET_COMPLETED OFFSET(12) NUMBITS(1) [],
        CALL_STACK_OVERFLOW OFFSET(11) NUMBITS(1) [],
        CALL_STACK_UNDERFLOW OFFSET(10) NUMBITS(1) [],
        INVALID_PLIST_COMMAND OFFSET(9) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(8) NUMBITS(1) [],
        CNT_1_LIMIT OFFSET(7) NUMBITS(1) [],
        CNT_0_LIMIT OFFSET(6) NUMBITS(1) [],
        BUS_STOP OFFSET(5) NUMBITS(1) [],
        BUS_ERROR OFFSET(4) NUMBITS(1) [],
        FORCE_HANG OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        END_OF_TILE OFFSET(1) NUMBITS(1) [],
        END_OF_FRAME OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0IntClear [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        RESET_COMPLETED OFFSET(12) NUMBITS(1) [],
        CALL_STACK_OVERFLOW OFFSET(11) NUMBITS(1) [],
        CALL_STACK_UNDERFLOW OFFSET(10) NUMBITS(1) [],
        INVALID_PLIST_COMMAND OFFSET(9) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(8) NUMBITS(1) [],
        CNT_1_LIMIT OFFSET(7) NUMBITS(1) [],
        CNT_0_LIMIT OFFSET(6) NUMBITS(1) [],
        BUS_STOP OFFSET(5) NUMBITS(1) [],
        BUS_ERROR OFFSET(4) NUMBITS(1) [],
        FORCE_HANG OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        END_OF_TILE OFFSET(1) NUMBITS(1) [],
        END_OF_FRAME OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0IntMask [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        RESET_COMPLETED OFFSET(12) NUMBITS(1) [],
        CALL_STACK_OVERFLOW OFFSET(11) NUMBITS(1) [],
        CALL_STACK_UNDERFLOW OFFSET(10) NUMBITS(1) [],
        INVALID_PLIST_COMMAND OFFSET(9) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(8) NUMBITS(1) [],
        CNT_1_LIMIT OFFSET(7) NUMBITS(1) [],
        CNT_0_LIMIT OFFSET(6) NUMBITS(1) [],
        BUS_STOP OFFSET(5) NUMBITS(1) [],
        BUS_ERROR OFFSET(4) NUMBITS(1) [],
        FORCE_HANG OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        END_OF_TILE OFFSET(1) NUMBITS(1) [],
        END_OF_FRAME OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0IntStatus [
        _0 OFFSET(13) NUMBITS(19) [],
        RESET_COMPLETED OFFSET(12) NUMBITS(1) [],
        CALL_STACK_OVERFLOW OFFSET(11) NUMBITS(1) [],
        CALL_STACK_UNDERFLOW OFFSET(10) NUMBITS(1) [],
        INVALID_PLIST_COMMAND OFFSET(9) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(8) NUMBITS(1) [],
        CNT_1_LIMIT OFFSET(7) NUMBITS(1) [],
        CNT_0_LIMIT OFFSET(6) NUMBITS(1) [],
        BUS_STOP OFFSET(5) NUMBITS(1) [],
        BUS_ERROR OFFSET(4) NUMBITS(1) [],
        FORCE_HANG OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        END_OF_TILE OFFSET(1) NUMBITS(1) [],
        END_OF_FRAME OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0WriteBoundaryEnable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        _0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0WriteBoundaryLow [
        WRITE_BOUNDARY_LOW OFFSET(8) NUMBITS(24) [],
        RESERVED0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0WriteBoundaryHigh [
        WRITE_BOUNDARY_HIGH OFFSET(8) NUMBITS(24) [],
        RESERVED0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0WriteBoundaryAddress [
        WRITE_BOUNDARY_ADDRESS OFFSET(2) NUMBITS(30) [],
        RESERVED0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0BusErrorStatus [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        READ_ERROR_ID OFFSET(6) NUMBITS(4) [],
        WRITE_ERROR_ID OFFSET(2) NUMBITS(4) [],
        READ_ERROR OFFSET(1) NUMBITS(1) [],
        WRITE_ERROR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0WatchdogDisable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        _0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0PerfCnt0Enable [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PERF_CNT_0_LIM_EN OFFSET(1) NUMBITS(1) [],
        PERF_CNT_0_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0PerfCnt0Src [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        PERF_CNT_0_SRC OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0PerfCnt1Enable [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PERF_CNT_1_LIM_EN OFFSET(1) NUMBITS(1) [],
        PERF_CNT_1_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0PerfCnt1Src [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        PERF_CNT_1_SRC OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0PerfCnt1Value [
        PERF_CNT_1_VALUE OFFSET(26) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0PerfmonContr [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        NUMBER_TILE_X_DIR OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(1) NUMBITS(15) [],
        PERFMON_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp0PerfmonBase [
        _0 OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1RendListAddr [
        REND_LIST_ADDR OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1RendRswBase [
        REND_RSW_BASE OFFSET(6) NUMBITS(26) [],
        _0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1RendVertexBase [
        REND_VERTEX_BASE OFFSET(6) NUMBITS(26) [],
        _0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1FeatureEnable [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        SUMMATE_QUAD_COVER OFFSET(6) NUMBITS(1) [],
        ORIGIN_LOWER_LEFT OFFSET(5) NUMBITS(1) [],
        EARLYZ_DISABLE2 OFFSET(4) NUMBITS(1) [],
        EARLYZ_DISABLE1 OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        EARLYZ_ENABLE OFFSET(1) NUMBITS(1) [],
        FP_TILEBUF_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1ZClearValue [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        Z_CLEAR_VALUE OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1StencilClearValue [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        STENCIL_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1AbgrClearValue0 [
        ALPHA_CLEAR_VALUE OFFSET(24) NUMBITS(8) [],
        BLUE_CLEAR_VALUE OFFSET(16) NUMBITS(8) [],
        GREEN_CLEAR_VALUE OFFSET(8) NUMBITS(8) [],
        RED_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1AbgrClearValue1 [
        ALPHA_CLEAR_VALUE OFFSET(24) NUMBITS(8) [],
        BLUE_CLEAR_VALUE OFFSET(16) NUMBITS(8) [],
        GREEN_CLEAR_VALUE OFFSET(8) NUMBITS(8) [],
        RED_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1AbgrClearValue2 [
        ALPHA_CLEAR_VALUE OFFSET(24) NUMBITS(8) [],
        BLUE_CLEAR_VALUE OFFSET(16) NUMBITS(8) [],
        GREEN_CLEAR_VALUE OFFSET(8) NUMBITS(8) [],
        RED_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1AbgrClearValue3 [
        ALPHA_CLEAR_VALUE OFFSET(24) NUMBITS(8) [],
        BLUE_CLEAR_VALUE OFFSET(16) NUMBITS(8) [],
        GREEN_CLEAR_VALUE OFFSET(8) NUMBITS(8) [],
        RED_CLEAR_VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1BoundingBoxLeftRight [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        BOUNDING_BOX_LEFT OFFSET(16) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        BOUNDING_BOX_RIGHT OFFSET(0) NUMBITS(14) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1BoundingBoxBottom [
        _0 OFFSET(14) NUMBITS(18) [],
        BOUNDING_BOX_BOTTOM OFFSET(0) NUMBITS(14) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1FsStackAddr [
        FS_STACK_ADDR OFFSET(6) NUMBITS(26) [],
        _0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1FsStackSizeAndInitVal [
        FS_STACK_INIT_VAL OFFSET(16) NUMBITS(16) [],
        FS_STACK_SIZE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1OriginOffsetX [
        _0 OFFSET(16) NUMBITS(16) [],
        ORIGIN_OFFSET_X OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1OriginOffsetY [
        _0 OFFSET(16) NUMBITS(16) [],
        ORIGIN_OFFSET_Y OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1SubpixelSpecifier [
        _0 OFFSET(8) NUMBITS(24) [],
        SUBPIXEL_SPECIFIER OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1TiebreakMode [
        _0 OFFSET(3) NUMBITS(29) [],
        TIEBREAK_MODE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1PlistConfig [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        LIST_FORMAT OFFSET(28) NUMBITS(2) [],
        RESERVED1 OFFSET(22) NUMBITS(6) [],
        SCALE_Y OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(6) NUMBITS(10) [],
        SCALE_X OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1ScalingConfig [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        SCALE_Y OFFSET(20) NUMBITS(3) [],
        RESERVED1 OFFSET(19) NUMBITS(1) [],
        SCALE_X OFFSET(16) NUMBITS(3) [],
        RESERVED2 OFFSET(12) NUMBITS(4) [],
        FLIP_DERIVATIVE_Y OFFSET(11) NUMBITS(1) [],
        FLIP_FRAGCOORD OFFSET(10) NUMBITS(1) [],
        FLIP_DITHERING_MATRIX OFFSET(9) NUMBITS(1) [],
        FLIP_POINT_SPRITES OFFSET(8) NUMBITS(1) [],
        _0 OFFSET(4) NUMBITS(4) [],
        DERIVATIVE_SCALE_ENABLE OFFSET(3) NUMBITS(1) [],
        FRAGCOORD_SCALE_ENABLE OFFSET(2) NUMBITS(1) [],
        DITHERING_SCALE_ENABLE OFFSET(1) NUMBITS(1) [],
        POINT_AND_LINE_SCALE_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1TilebufferBits [
        _0 OFFSET(16) NUMBITS(16) [],
        TILEBUFFER_A_BITS OFFSET(12) NUMBITS(4) [],
        TILEBUFFER_B_BITS OFFSET(8) NUMBITS(4) [],
        TILEBUFFER_G_BITS OFFSET(4) NUMBITS(4) [],
        TILEBUFFER_R_BITS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0SourceSelect [
        _0 OFFSET(2) NUMBITS(30) [],
        WB0_SOURCE_SELECT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0TargetAddr [
        WB0_TARGET_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0TargetPixelFormat [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        WB0_TARGET_PIXEL_FORMAT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0TargetAaFormat [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        WB0_TARGET_AA_Y OFFSET(12) NUMBITS(3) [],
        RESERVED1 OFFSET(10) NUMBITS(2) [],
        WB0_TARGET_AA_X OFFSET(8) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        WB0_TARGET_AA_FORMAT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0TargetLayout [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        WB0_TARGET_LAYOUT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0TargetScanlineLength [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WB0_TARGET_SCANLINE_LENGTH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0TargetFlags [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        WB0_BIG_ENDIAN OFFSET(5) NUMBITS(1) [],
        WB0_DITHER_ENABLE OFFSET(4) NUMBITS(1) [],
        WB0_INV_COMPONENT_ORDER_ENABLE OFFSET(3) NUMBITS(1) [],
        WB0_SWAP_RED_BLUE_ENABLE OFFSET(2) NUMBITS(1) [],
        WB0_BOUNDING_BOX_ENABLE OFFSET(1) NUMBITS(1) [],
        WB0_DIRTY_BIT_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0MrtEnable [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        WB0_MRT_ENABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0MrtOffset [
        WB0_MRT_OFFSET OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0GlobalTestEnable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        WB0_GLOBAL_TEST_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0GlobalTestRefValue [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        _0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb0GlobalTestCmpFunc [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1SourceSelect [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        WB1_SOURCE_SELECT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1TargetAddr [
        WB1_TARGET_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1TargetPixelFormat [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        WB1_TARGET_PIXEL_FORMAT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1TargetAaFormat [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        WB1_TARGET_AA_Y OFFSET(12) NUMBITS(3) [],
        RESERVED1 OFFSET(10) NUMBITS(2) [],
        WB1_TARGET_AA_X OFFSET(8) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        WB1_TARGET_AA_FORMAT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1TargetLayout [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        WB1_TARGET_LAYOUT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1TargetScanlineLength [
        _0 OFFSET(16) NUMBITS(16) [],
        WB2_TARGET_SCANLINE_LENGTH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1TargetFlags [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        WB1_BIG_ENDIAN OFFSET(5) NUMBITS(1) [],
        WB1_DITHER_ENABLE OFFSET(4) NUMBITS(1) [],
        WB1_INV_COMPONENT_ORDER_ENABLE OFFSET(3) NUMBITS(1) [],
        WB1_SWAP_RED_BLUE_ENABLE OFFSET(2) NUMBITS(1) [],
        WB1_BOUNDING_BOX_ENABLE OFFSET(1) NUMBITS(1) [],
        WB1_DIRTY_BIT_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1MrtEnable [
        _0 OFFSET(4) NUMBITS(28) [],
        WB1_MRT_ENABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1MrtOffset [
        WB1_MRT_OFFSET OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1GlobalTestEnable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        WB1_GLOBAL_TEST_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1GlobalTestRefValue [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        _0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb1GlobalTestCmpFunc [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2SourceSelect [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        WB2_SOURCE_SELECT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2TargetAddr [
        WB2_TARGET_ADDR OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2TargetPixelFormat [
        _0 OFFSET(4) NUMBITS(28) [],
        WB2_TARGET_PIXEL_FORMAT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2TargetAaFormat [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        WB2_TARGET_AA_Y OFFSET(12) NUMBITS(3) [],
        RESERVED1 OFFSET(10) NUMBITS(2) [],
        WB2_TARGET_AA_X OFFSET(8) NUMBITS(2) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        WB2_TARGET_AA_FORMAT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2TargetLayout [
        _0 OFFSET(2) NUMBITS(30) [],
        WB2_TARGET_LAYOUT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2TargetScanlineLength [
        _0 OFFSET(16) NUMBITS(16) [],
        WB2_TARGET_SCANLINE_LENGTH OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2TargetFlags [
        _0 OFFSET(6) NUMBITS(26) [],
        WB2_BIG_ENDIAN OFFSET(5) NUMBITS(1) [],
        WB2_DITHER_ENABLE OFFSET(4) NUMBITS(1) [],
        WB2_INV_COMPONENT_ORDER_ENABLE OFFSET(3) NUMBITS(1) [],
        WB2_SWAP_RED_BLUE_ENABLE OFFSET(2) NUMBITS(1) [],
        WB2_BOUNDING_BOX_ENABLE OFFSET(1) NUMBITS(1) [],
        WB2_DIRTY_BIT_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2MrtEnable [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        WB2_MRT_ENABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2MrtOffset [
        WB2_MRT_OFFSET OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2GlobalTestEnable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        WB2_GLOBAL_TEST_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2GlobalTestRefValue [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        _0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Wb2GlobalTestCmpFunc [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        _0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Version [
        PRODUCT_ID OFFSET(16) NUMBITS(16) [],
        VERSION_MAJOR OFFSET(8) NUMBITS(8) [],
        VERSION_MINOR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1CurrentRendListAddr [
        CURRENT_REND_LIST_ADDR OFFSET(5) NUMBITS(27) [],
        RESERVED0 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1Status [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        CLK_OVERRIDE OFFSET(7) NUMBITS(1) [],
        INTERRUPT_ASSERTED OFFSET(6) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(5) NUMBITS(1) [],
        BUS_STOPPED OFFSET(4) NUMBITS(1) [],
        BUS_ERROR OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        TILE_STOPPED OFFSET(1) NUMBITS(1) [],
        RENDERING_ACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1CtrlMgmt [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        CLK_OVERRIDE OFFSET(8) NUMBITS(1) [],
        SOFT_RESET OFFSET(7) NUMBITS(1) [],
        START_RENDERING OFFSET(6) NUMBITS(1) [],
        FORCE_RESET OFFSET(5) NUMBITS(1) [],
        FORCE_HANG OFFSET(4) NUMBITS(1) [],
        FLUSH_CACHES OFFSET(3) NUMBITS(1) [],
        END_AFTER_TILE OFFSET(2) NUMBITS(1) [],
        START_BUS OFFSET(1) NUMBITS(1) [],
        STOP_BUS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1LastTilePosStart [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        TILEY_START OFFSET(16) NUMBITS(8) [],
        _0 OFFSET(8) NUMBITS(8) [],
        TILEX_START OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1LastTilePosEnd [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        TILEY_END OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(8) NUMBITS(8) [],
        TILEX_END OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1IntRawstat [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        RESET_COMPLETED OFFSET(12) NUMBITS(1) [],
        CALL_STACK_OVERFLOW OFFSET(11) NUMBITS(1) [],
        CALL_STACK_UNDERFLOW OFFSET(10) NUMBITS(1) [],
        INVALID_PLIST_COMMAND OFFSET(9) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(8) NUMBITS(1) [],
        CNT_1_LIMIT OFFSET(7) NUMBITS(1) [],
        CNT_0_LIMIT OFFSET(6) NUMBITS(1) [],
        BUS_STOP OFFSET(5) NUMBITS(1) [],
        BUS_ERROR OFFSET(4) NUMBITS(1) [],
        FORCE_HANG OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        END_OF_TILE OFFSET(1) NUMBITS(1) [],
        END_OF_FRAME OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1IntClear [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        RESET_COMPLETED OFFSET(12) NUMBITS(1) [],
        CALL_STACK_OVERFLOW OFFSET(11) NUMBITS(1) [],
        CALL_STACK_UNDERFLOW OFFSET(10) NUMBITS(1) [],
        INVALID_PLIST_COMMAND OFFSET(9) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(8) NUMBITS(1) [],
        CNT_1_LIMIT OFFSET(7) NUMBITS(1) [],
        CNT_0_LIMIT OFFSET(6) NUMBITS(1) [],
        BUS_STOP OFFSET(5) NUMBITS(1) [],
        BUS_ERROR OFFSET(4) NUMBITS(1) [],
        FORCE_HANG OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        END_OF_TILE OFFSET(1) NUMBITS(1) [],
        END_OF_FRAME OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1IntMask [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        RESET_COMPLETED OFFSET(12) NUMBITS(1) [],
        CALL_STACK_OVERFLOW OFFSET(11) NUMBITS(1) [],
        CALL_STACK_UNDERFLOW OFFSET(10) NUMBITS(1) [],
        INVALID_PLIST_COMMAND OFFSET(9) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(8) NUMBITS(1) [],
        CNT_1_LIMIT OFFSET(7) NUMBITS(1) [],
        CNT_0_LIMIT OFFSET(6) NUMBITS(1) [],
        BUS_STOP OFFSET(5) NUMBITS(1) [],
        BUS_ERROR OFFSET(4) NUMBITS(1) [],
        FORCE_HANG OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        END_OF_TILE OFFSET(1) NUMBITS(1) [],
        END_OF_FRAME OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1IntStatus [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        RESET_COMPLETED OFFSET(12) NUMBITS(1) [],
        CALL_STACK_OVERFLOW OFFSET(11) NUMBITS(1) [],
        CALL_STACK_UNDERFLOW OFFSET(10) NUMBITS(1) [],
        INVALID_PLIST_COMMAND OFFSET(9) NUMBITS(1) [],
        WRITE_BOUNDARY_ERROR OFFSET(8) NUMBITS(1) [],
        CNT_1_LIMIT OFFSET(7) NUMBITS(1) [],
        CNT_0_LIMIT OFFSET(6) NUMBITS(1) [],
        BUS_STOP OFFSET(5) NUMBITS(1) [],
        BUS_ERROR OFFSET(4) NUMBITS(1) [],
        FORCE_HANG OFFSET(3) NUMBITS(1) [],
        HANG OFFSET(2) NUMBITS(1) [],
        END_OF_TILE OFFSET(1) NUMBITS(1) [],
        END_OF_FRAME OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1WriteBoundaryEnable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        _0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1WriteBoundaryLow [
        WRITE_BOUNDARY_LOW OFFSET(8) NUMBITS(24) [],
        RESERVED0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1WriteBoundaryHigh [
        WRITE_BOUNDARY_HIGH OFFSET(8) NUMBITS(24) [],
        _0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1WriteBoundaryAddress [
        WRITE_BOUNDARY_ADDRESS OFFSET(2) NUMBITS(30) [],
        _0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1BusErrorStatus [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        READ_ERROR_ID OFFSET(6) NUMBITS(4) [],
        WRITE_ERROR_ID OFFSET(2) NUMBITS(4) [],
        READ_ERROR OFFSET(1) NUMBITS(1) [],
        WRITE_ERROR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1WatchdogDisable [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        _0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1PerfCnt0Enable [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PERF_CNT_0_LIM_EN OFFSET(1) NUMBITS(1) [],
        PERF_CNT_0_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1PerfCnt0Src [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        PERF_CNT_0_SRC OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1PerfCnt1Enable [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PERF_CNT_1_LIM_EN OFFSET(1) NUMBITS(1) [],
        PERF_CNT_1_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1PerfCnt1Src [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        PERF_CNT_1_SRC OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1PerfCnt1Value [
        PERF_CNT_1_VALUE OFFSET(26) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1PerfmonContr [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        NUMBER_TILE_X_DIR OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(1) NUMBITS(15) [],
        PERFMON_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp1PerfmonBase [
        _0 OFFSET(3) NUMBITS(29) [],
        RESERVED0 OFFSET(0) NUMBITS(3) [],
    ]
];

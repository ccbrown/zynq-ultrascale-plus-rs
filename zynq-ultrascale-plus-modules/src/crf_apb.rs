// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// FPD Clock and Reset control, Clock and Reset control registers for FPD.
pub static mut CRF_APB: *mut Registers = 0xfd1a0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// SLVERR Error Signal Enable.
        (0x00000000 => pub err_ctrl: ReadWrite<u8, ErrCtrl::Register>),
        (0x00000001 => _padding1),
        /// APB Register Address Decode Error Interrupt Status and Clear.
        (0x00000004 => pub ir_status: ReadWrite<u8, IrStatus::Register>),
        (0x00000005 => _padding5),
        /// Interrupt Mask.
        (0x00000008 => pub ir_mask: ReadOnly<u8, IrMask::Register>),
        (0x00000009 => _padding9),
        /// Interrupt Mask.
        (0x0000000c => pub ir_enable: WriteOnly<u8, IrEnable::Register>),
        (0x0000000d => _padding13),
        /// Interrupt Disable.
        (0x00000010 => pub ir_disable: WriteOnly<u8, IrDisable::Register>),
        (0x00000011 => _padding17),
        /// CRF_APB SLCR Write Protection.
        (0x0000001c => pub crf_wprot: ReadWrite<u8, CrfWprot::Register>),
        (0x0000001d => _padding29),
        /// APLL Clock Unit Control
        (0x00000020 => pub apll_ctrl: ReadWrite<u32, ApllCtrl::Register>),
        /// APLL Integer Helper Data Configuration.
        (0x00000024 => pub apll_cfg: ReadWrite<u32, ApllCfg::Register>),
        /// Fractional control for the PLL
        (0x00000028 => pub apll_frac_cfg: ReadWrite<u32, ApllFracCfg::Register>),
        /// DPLL Clock Unit Control
        (0x0000002c => pub dpll_ctrl: ReadWrite<u32, DpllCtrl::Register>),
        /// DPLL Integer Helper Data Configuration.
        (0x00000030 => pub dpll_cfg: ReadWrite<u32, DpllCfg::Register>),
        /// Fractional control for the PLL
        (0x00000034 => pub dpll_frac_cfg: ReadWrite<u32, DpllFracCfg::Register>),
        /// VPLL Clock Unit Control.
        (0x00000038 => pub vpll_ctrl: ReadWrite<u32, VpllCtrl::Register>),
        /// VPLL Integer Helper Data Configuration.
        (0x0000003c => pub vpll_cfg: ReadWrite<u32, VpllCfg::Register>),
        /// Fractional control for the PLL.
        (0x00000040 => pub vpll_frac_cfg: ReadWrite<u32, VpllFracCfg::Register>),
        /// FPD PLL Clocking Status.
        (0x00000044 => pub pll_status: Aliased<u8, PllStatusR::Register, PllStatusW::Register>),
        (0x00000045 => _padding69),
        /// APLL to LPD Clock Divisor.
        (0x00000048 => pub apll_to_lpd_ctrl: ReadWrite<u16, ApllToLpdCtrl::Register>),
        (0x0000004a => _padding74),
        /// DPLL to LPD Clock Divisor.
        (0x0000004c => pub dpll_to_lpd_ctrl: ReadWrite<u16, DpllToLpdCtrl::Register>),
        (0x0000004e => _padding78),
        /// VPLL to LPD Clock Divisor.
        (0x00000050 => pub vpll_to_lpd_ctrl: ReadWrite<u16, VpllToLpdCtrl::Register>),
        (0x00000052 => _padding82),
        /// APU MPCore Clock Generator Control.
        (0x00000060 => pub acpu_ctrl: ReadWrite<u32, AcpuCtrl::Register>),
        /// Debug Trace Clock Generator Control.
        (0x00000064 => pub dbg_trace_ctrl: ReadWrite<u32, DbgTraceCtrl::Register>),
        /// Debug in FPD Clock Generator Control.
        (0x00000068 => pub dbg_fpd_ctrl: ReadWrite<u32, DbgFpdCtrl::Register>),
        (0x0000006c => _padding108),
        /// DisplayPort Video Clock Generator Control.
        (0x00000070 => pub dp_video_ref_ctrl: ReadWrite<u32, DpVideoRefCtrl::Register>),
        /// DisplayPort Audio Clock Generator Control.
        (0x00000074 => pub dp_audio_ref_ctrl: ReadWrite<u32, DpAudioRefCtrl::Register>),
        (0x00000078 => _padding120),
        /// DisplayPort System Time Clock Generator Control.
        (0x0000007c => pub dp_stc_ref_ctrl: ReadWrite<u32, DpStcRefCtrl::Register>),
        /// DDR Memory Controller Clock Generator Control.
        (0x00000080 => pub ddr_ctrl: ReadWrite<u32, DdrCtrl::Register>),
        /// GPU Clock Generator Control.
        (0x00000084 => pub gpu_ref_ctrl: ReadWrite<u32, GpuRefCtrl::Register>),
        (0x00000088 => _padding136),
        /// SATA Clock Generator Control.
        (0x000000a0 => pub sata_ref_ctrl: ReadWrite<u32, SataRefCtrl::Register>),
        (0x000000a4 => _padding164),
        /// PCIe Clock Generator Control.
        (0x000000b4 => pub pcie_ref_ctrl: ReadWrite<u32, PcieRefCtrl::Register>),
        /// FPD DMA Clock Generator Control.
        (0x000000b8 => pub fpd_dma_ref_ctrl: ReadWrite<u32, FpdDmaRefCtrl::Register>),
        /// DisplayPort DMA Clock Generator Control.
        (0x000000bc => pub dpdma_ref_ctrl: ReadWrite<u32, DpdmaRefCtrl::Register>),
        /// AXI InterconnectClock Generator Config (TOPSW_MAIN_CLK)
        (0x000000c0 => pub topsw_main_ctrl: ReadWrite<u32, TopswMainCtrl::Register>),
        /// APB Clock Generator Config (TOP_LSBUS_CLK)
        (0x000000c4 => pub topsw_lsbus_ctrl: ReadWrite<u32, TopswLsbusCtrl::Register>),
        (0x000000c8 => _padding200),
        /// Debug Time StampClock Generator Control in FPD.
        (0x000000f8 => pub dbg_tstmp_ctrl: ReadWrite<u32, DbgTstmpCtrl::Register>),
        (0x000000fc => _padding252),
        /// Software Controlled FPD Resets.
        (0x00000100 => pub rst_fpd_top: ReadWrite<u32, RstFpdTop::Register>),
        /// Software Controlled APU MPCore Resets.
        (0x00000104 => pub rst_fpd_apu: ReadWrite<u32, RstFpdApu::Register>),
        /// Software Controlled DDR Memory Controller Resets.
        (0x00000108 => pub rst_ddr_ss: ReadWrite<u8, RstDdrSs::Register>),
        (0x00000109 => @END),
    }
}
register_bitfields! [
    u8,
    pub ErrCtrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub IrStatus [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub IrMask [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub IrEnable [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub IrDisable [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub CrfWprot [
        ACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ApllCtrl [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        POST_SRC OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        PRE_SRC OFFSET(20) NUMBITS(3) [],
        RESERVED2 OFFSET(18) NUMBITS(2) [],
        RESERVED3 OFFSET(17) NUMBITS(1) [],
        DIV2 OFFSET(16) NUMBITS(1) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        FBDIV OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(4) NUMBITS(4) [],
        BYPASS OFFSET(3) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(2) [],
        RESET OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ApllCfg [
        LOCK_DLY OFFSET(25) NUMBITS(7) [],
        RESERVED0 OFFSET(23) NUMBITS(2) [],
        LOCK_CNT OFFSET(13) NUMBITS(10) [],
        RESERVED1 OFFSET(12) NUMBITS(1) [],
        LFHF OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        CP OFFSET(5) NUMBITS(4) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub ApllFracCfg [
        ENABLED OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(25) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(20) NUMBITS(2) [],
        RESERVED3 OFFSET(19) NUMBITS(1) [],
        RESERVED4 OFFSET(18) NUMBITS(1) [],
        RESERVED5 OFFSET(16) NUMBITS(2) [],
        DATA OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub DpllCtrl [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        POST_SRC OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        PRE_SRC OFFSET(20) NUMBITS(3) [],
        RESERVED2 OFFSET(18) NUMBITS(2) [],
        RESERVED3 OFFSET(17) NUMBITS(1) [],
        DIV2 OFFSET(16) NUMBITS(1) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        FBDIV OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(4) NUMBITS(4) [],
        BYPASS OFFSET(3) NUMBITS(1) [],
        RESERVED6 OFFSET(1) NUMBITS(2) [],
        RESET OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DpllCfg [
        LOCK_DLY OFFSET(25) NUMBITS(7) [],
        RESERVED0 OFFSET(23) NUMBITS(2) [],
        LOCK_CNT OFFSET(13) NUMBITS(10) [],
        RESERVED1 OFFSET(12) NUMBITS(1) [],
        LFHF OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        CP OFFSET(5) NUMBITS(4) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub DpllFracCfg [
        ENABLED OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(25) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(20) NUMBITS(2) [],
        RESERVED3 OFFSET(19) NUMBITS(1) [],
        RESERVED4 OFFSET(18) NUMBITS(1) [],
        RESERVED5 OFFSET(16) NUMBITS(2) [],
        DATA OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub VpllCtrl [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        POST_SRC OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        PRE_SRC OFFSET(20) NUMBITS(3) [],
        RESERVED2 OFFSET(18) NUMBITS(2) [],
        CLKOUTDIV OFFSET(17) NUMBITS(1) [],
        DIV2 OFFSET(16) NUMBITS(1) [],
        RESERVED3 OFFSET(15) NUMBITS(1) [],
        FBDIV OFFSET(8) NUMBITS(7) [],
        RESERVED4 OFFSET(4) NUMBITS(4) [],
        BYPASS OFFSET(3) NUMBITS(1) [],
        RESERVED5 OFFSET(1) NUMBITS(2) [],
        RESET OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub VpllCfg [
        LOCK_DLY OFFSET(25) NUMBITS(7) [],
        RESERVED0 OFFSET(23) NUMBITS(2) [],
        LOCK_CNT OFFSET(13) NUMBITS(10) [],
        RESERVED1 OFFSET(12) NUMBITS(1) [],
        LFHF OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        CP OFFSET(5) NUMBITS(4) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub VpllFracCfg [
        ENABLED OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(25) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(20) NUMBITS(2) [],
        RESERVED3 OFFSET(19) NUMBITS(1) [],
        RESERVED4 OFFSET(18) NUMBITS(1) [],
        RESERVED5 OFFSET(16) NUMBITS(2) [],
        DATA OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u8,
    pub PllStatusR [
        RESERVED0 OFFSET(6) NUMBITS(2) [],
        VPLL_STABLE OFFSET(5) NUMBITS(1) [],
        DPLL_STABLE OFFSET(4) NUMBITS(1) [],
        APLL_STABLE OFFSET(3) NUMBITS(1) [],
        VPLL_LOCK OFFSET(2) NUMBITS(1) [],
        DPLL_LOCK OFFSET(1) NUMBITS(1) [],
        APLL_LOCK OFFSET(0) NUMBITS(1) [],
    ],
    pub PllStatusW [
        RESERVED0 OFFSET(6) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u16,
    pub ApllToLpdCtrl [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u16,
    pub DpllToLpdCtrl [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u16,
    pub VpllToLpdCtrl [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub AcpuCtrl [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(27) NUMBITS(1) [],
        RESERVED2 OFFSET(26) NUMBITS(1) [],
        CLKACT_HALF OFFSET(25) NUMBITS(1) [],
        CLKACT_FULL OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED4 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub DbgTraceCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub DbgFpdCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub DpVideoRefCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub DpAudioRefCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub DpStcRefCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub DdrCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(24) NUMBITS(1) [],
        RESERVED2 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub GpuRefCtrl [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        PP1_CLKACT OFFSET(26) NUMBITS(1) [],
        PP0_CLKACT OFFSET(25) NUMBITS(1) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub SataRefCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub PcieRefCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub FpdDmaRefCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub DpdmaRefCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub TopswMainCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub TopswLsbusCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub DbgTstmpCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(24) NUMBITS(1) [],
        RESERVED2 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub RstFpdTop [
        RESERVED0 OFFSET(20) NUMBITS(4) [],
        PCIE_CFG_RESET OFFSET(19) NUMBITS(1) [],
        PCIE_BRIDGE_RESET OFFSET(18) NUMBITS(1) [],
        PCIE_CTRL_RESET OFFSET(17) NUMBITS(1) [],
        DP_RESET OFFSET(16) NUMBITS(1) [],
        SWDT_RESET OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(1) [],
        S_AXI_HPC_3_FPD_RESET OFFSET(12) NUMBITS(1) [],
        S_AXI_HPC_2_FPD_RESET OFFSET(11) NUMBITS(1) [],
        S_AXI_HP_1_FPD_RESET OFFSET(10) NUMBITS(1) [],
        S_AXI_HP_0_FPD_RESET OFFSET(9) NUMBITS(1) [],
        S_AXI_HPC_1_FPD_RESET OFFSET(8) NUMBITS(1) [],
        S_AXI_HPC_0_FPD_RESET OFFSET(7) NUMBITS(1) [],
        FPD_DMA_RESET OFFSET(6) NUMBITS(1) [],
        GPU_PP1_RESET OFFSET(5) NUMBITS(1) [],
        GPU_PP0_RESET OFFSET(4) NUMBITS(1) [],
        GPU_RESET OFFSET(3) NUMBITS(1) [],
        GT_RESET OFFSET(2) NUMBITS(1) [],
        SATA_RESET OFFSET(1) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub RstFpdApu [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        ACPU3_PWRON_RESET OFFSET(13) NUMBITS(1) [],
        ACPU2_PWRON_RESET OFFSET(12) NUMBITS(1) [],
        ACPU1_PWRON_RESET OFFSET(11) NUMBITS(1) [],
        ACPU0_PWRON_RESET OFFSET(10) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(1) [],
        APU_L2_RESET OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        ACPU3_RESET OFFSET(3) NUMBITS(1) [],
        ACPU2_RESET OFFSET(2) NUMBITS(1) [],
        ACPU1_RESET OFFSET(1) NUMBITS(1) [],
        ACPU0_RESET OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RstDdrSs [
        RESERVED0 OFFSET(4) NUMBITS(4) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        APM_RESET OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ]
];

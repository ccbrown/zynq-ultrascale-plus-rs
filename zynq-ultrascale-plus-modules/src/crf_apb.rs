// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// FPD Clock and Reset control, Clock and Reset control registers for FPD.
pub static mut CRF_APB: *mut Registers = 0xfd1a0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// SLVERR Error Signal Enable.
    pub err_ctrl: ReadWrite<u8, ErrCtrl::Register>,
    _padding1: [u8; 3],
    /// APB Register Address Decode Error Interrupt Status and Clear.
    pub ir_status: ReadWrite<u8, IrStatus::Register>,
    _padding5: [u8; 3],
    /// Interrupt Mask.
    pub ir_mask: ReadOnly<u8, IrMask::Register>,
    _padding9: [u8; 3],
    /// Interrupt Mask.
    pub ir_enable: WriteOnly<u8, IrEnable::Register>,
    _padding13: [u8; 3],
    /// Interrupt Disable.
    pub ir_disable: WriteOnly<u8, IrDisable::Register>,
    _padding17: [u8; 11],
    /// CRF_APB SLCR Write Protection.
    pub crf_wprot: ReadWrite<u8, CrfWprot::Register>,
    _padding29: [u8; 3],
    /// APLL Clock Unit Control
    pub apll_ctrl: ReadWrite<u32, ApllCtrl::Register>,
    /// APLL Integer Helper Data Configuration.
    pub apll_cfg: ReadWrite<u32, ApllCfg::Register>,
    /// Fractional control for the PLL
    pub apll_frac_cfg: ReadWrite<u32, ApllFracCfg::Register>,
    /// DPLL Clock Unit Control
    pub dpll_ctrl: ReadWrite<u32, DpllCtrl::Register>,
    /// DPLL Integer Helper Data Configuration.
    pub dpll_cfg: ReadWrite<u32, DpllCfg::Register>,
    /// Fractional control for the PLL
    pub dpll_frac_cfg: ReadWrite<u32, DpllFracCfg::Register>,
    /// VPLL Clock Unit Control.
    pub vpll_ctrl: ReadWrite<u32, VpllCtrl::Register>,
    /// VPLL Integer Helper Data Configuration.
    pub vpll_cfg: ReadWrite<u32, VpllCfg::Register>,
    /// Fractional control for the PLL.
    pub vpll_frac_cfg: ReadWrite<u32, VpllFracCfg::Register>,
    /// FPD PLL Clocking Status.
    pub pll_status: Aliased<u8, PllStatusR::Register, PllStatusW::Register>,
    _padding69: [u8; 3],
    /// APLL to LPD Clock Divisor.
    pub apll_to_lpd_ctrl: ReadWrite<u16, ApllToLpdCtrl::Register>,
    _padding74: [u8; 2],
    /// DPLL to LPD Clock Divisor.
    pub dpll_to_lpd_ctrl: ReadWrite<u16, DpllToLpdCtrl::Register>,
    _padding78: [u8; 2],
    /// VPLL to LPD Clock Divisor.
    pub vpll_to_lpd_ctrl: ReadWrite<u16, VpllToLpdCtrl::Register>,
    _padding82: [u8; 14],
    /// APU MPCore Clock Generator Control.
    pub acpu_ctrl: ReadWrite<u32, AcpuCtrl::Register>,
    /// Debug Trace Clock Generator Control.
    pub dbg_trace_ctrl: ReadWrite<u32, DbgTraceCtrl::Register>,
    /// Debug in FPD Clock Generator Control.
    pub dbg_fpd_ctrl: ReadWrite<u32, DbgFpdCtrl::Register>,
    _padding108: [u8; 4],
    /// DisplayPort Video Clock Generator Control.
    pub dp_video_ref_ctrl: ReadWrite<u32, DpVideoRefCtrl::Register>,
    /// DisplayPort Audio Clock Generator Control.
    pub dp_audio_ref_ctrl: ReadWrite<u32, DpAudioRefCtrl::Register>,
    _padding120: [u8; 4],
    /// DisplayPort System Time Clock Generator Control.
    pub dp_stc_ref_ctrl: ReadWrite<u32, DpStcRefCtrl::Register>,
    /// DDR Memory Controller Clock Generator Control.
    pub ddr_ctrl: ReadWrite<u32, DdrCtrl::Register>,
    /// GPU Clock Generator Control.
    pub gpu_ref_ctrl: ReadWrite<u32, GpuRefCtrl::Register>,
    _padding136: [u8; 24],
    /// SATA Clock Generator Control.
    pub sata_ref_ctrl: ReadWrite<u32, SataRefCtrl::Register>,
    _padding164: [u8; 16],
    /// PCIe Clock Generator Control.
    pub pcie_ref_ctrl: ReadWrite<u32, PcieRefCtrl::Register>,
    /// FPD DMA Clock Generator Control.
    pub fpd_dma_ref_ctrl: ReadWrite<u32, FpdDmaRefCtrl::Register>,
    /// DisplayPort DMA Clock Generator Control.
    pub dpdma_ref_ctrl: ReadWrite<u32, DpdmaRefCtrl::Register>,
    /// AXI InterconnectClock Generator Config (TOPSW_MAIN_CLK)
    pub topsw_main_ctrl: ReadWrite<u32, TopswMainCtrl::Register>,
    /// APB Clock Generator Config (TOP_LSBUS_CLK)
    pub topsw_lsbus_ctrl: ReadWrite<u32, TopswLsbusCtrl::Register>,
    _padding200: [u8; 48],
    /// Debug Time StampClock Generator Control in FPD.
    pub dbg_tstmp_ctrl: ReadWrite<u32, DbgTstmpCtrl::Register>,
    _padding252: [u8; 4],
    /// Software Controlled FPD Resets.
    pub rst_fpd_top: ReadWrite<u32, RstFpdTop::Register>,
    /// Software Controlled APU MPCore Resets.
    pub rst_fpd_apu: ReadWrite<u32, RstFpdApu::Register>,
    /// Software Controlled DDR Memory Controller Resets.
    pub rst_ddr_ss: ReadWrite<u8, RstDdrSs::Register>,
}
tock_registers::register_bitfields! [
    u8,
    pub ErrCtrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub IrStatus [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub IrMask [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub IrEnable [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub IrDisable [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub CrfWprot [
        ACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u16,
    pub ApllToLpdCtrl [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub DpllToLpdCtrl [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub VpllToLpdCtrl [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u8,
    pub RstDdrSs [
        RESERVED0 OFFSET(4) NUMBITS(4) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        APM_RESET OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ]
];

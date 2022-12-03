// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// FPD Clock and Reset control, Clock and Reset control registers for LPD.
pub static mut CRL_APB: *mut Registers = 0xff5e0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// SLVERR Error Signal Enable.
    pub err_ctrl: ReadWrite<u8, ErrCtrl::Register>,
    _padding1: [u8; 3],
    /// Interrupt Status and Clear.
    pub ir_status: ReadWrite<u8, IrStatus::Register>,
    _padding5: [u8; 3],
    /// Interrupt Mask.
    pub ir_mask: ReadOnly<u8, IrMask::Register>,
    _padding9: [u8; 3],
    /// Interrupt Enable.
    pub ir_enable: WriteOnly<u8, IrEnable::Register>,
    _padding13: [u8; 3],
    /// Interrupt Disable.
    pub ir_disable: WriteOnly<u8, IrDisable::Register>,
    _padding17: [u8; 11],
    /// CRL SLCR Write Register Protection Control.
    pub crl_wprot: ReadWrite<u8, CrlWprot::Register>,
    _padding29: [u8; 3],
    /// IOPLL Clock Unit Control.
    pub iopll_ctrl: ReadWrite<u32, IopllCtrl::Register>,
    /// IOPLL Integer Helper Data Config.
    pub iopll_cfg: ReadWrite<u32, IopllCfg::Register>,
    /// Fractional control for the PLL
    pub iopll_frac_cfg: ReadWrite<u32, IopllFracCfg::Register>,
    _padding44: [u8; 4],
    /// RPLL Clock Unit Control.
    pub rpll_ctrl: ReadWrite<u32, RpllCtrl::Register>,
    /// RPLL Integer Helper Data Configuration.
    pub rpll_cfg: ReadWrite<u32, RpllCfg::Register>,
    /// Fractional control for the PLL
    pub rpll_frac_cfg: ReadWrite<u32, RpllFracCfg::Register>,
    _padding60: [u8; 4],
    /// LPD PLL Clocking Status.
    pub pll_status: Aliased<u32, PllStatusR::Register, PllStatusW::Register>,
    /// IOPLL clock divider for distribution in FPD.
    pub iopll_to_fpd_ctrl: ReadWrite<u16, IopllToFpdCtrl::Register>,
    _padding70: [u8; 2],
    /// RPLL clock divider for distribution in FPD.
    pub rpll_to_fpd_ctrl: ReadWrite<u16, RpllToFpdCtrl::Register>,
    _padding74: [u8; 2],
    /// USB 3.0 Unit Clock Generator Control.
    pub usb3_dual_ref_ctrl: ReadWrite<u32, Usb3DualRefCtrl::Register>,
    /// GEM 0 Clock Generator Control.
    pub gem0_ref_ctrl: ReadWrite<u32, Gem0RefCtrl::Register>,
    /// GEM 1 Clock Generator Control.
    pub gem1_ref_ctrl: ReadWrite<u32, Gem1RefCtrl::Register>,
    /// GEM 2 Clock Generator Config
    pub gem2_ref_ctrl: ReadWrite<u32, Gem2RefCtrl::Register>,
    /// GEM 3 Clock Generator Config
    pub gem3_ref_ctrl: ReadWrite<u32, Gem3RefCtrl::Register>,
    /// USB 0 Clock Generator Config
    pub usb0_bus_ref_ctrl: ReadWrite<u32, Usb0BusRefCtrl::Register>,
    /// USB 1 Clock Generator Config
    pub usb1_bus_ref_ctrl: ReadWrite<u32, Usb1BusRefCtrl::Register>,
    /// Quad-SPI Clock Generator Config
    pub qspi_ref_ctrl: ReadWrite<u32, QspiRefCtrl::Register>,
    /// SDIO 0 Clock Generator Config
    pub sdio0_ref_ctrl: ReadWrite<u32, Sdio0RefCtrl::Register>,
    /// SDIO 1 Clock Generator Config
    pub sdio1_ref_ctrl: ReadWrite<u32, Sdio1RefCtrl::Register>,
    /// UART 0 Clock Generator Config
    pub uart0_ref_ctrl: ReadWrite<u32, Uart0RefCtrl::Register>,
    /// UART 1 Clock Generator Config
    pub uart1_ref_ctrl: ReadWrite<u32, Uart1RefCtrl::Register>,
    /// SPI 0 Clock Generator Config
    pub spi0_ref_ctrl: ReadWrite<u32, Spi0RefCtrl::Register>,
    /// SPI 1 Clock Generator Config
    pub spi1_ref_ctrl: ReadWrite<u32, Spi1RefCtrl::Register>,
    /// CAN 0 Clock Generator Config
    pub can0_ref_ctrl: ReadWrite<u32, Can0RefCtrl::Register>,
    /// CAN 1 Clock Generator Config
    pub can1_ref_ctrl: ReadWrite<u32, Can1RefCtrl::Register>,
    _padding140: [u8; 4],
    /// RPU MPCore and OCM Clock Generator Config
    pub cpu_r5_ctrl: ReadWrite<u32, CpuR5Ctrl::Register>,
    _padding148: [u8; 8],
    /// AXI Interface Clock Generator Config for LPD In/Outbound Switches
    pub iou_switch_ctrl: ReadWrite<u32, IouSwitchCtrl::Register>,
    /// CSU Clock Generator Config
    pub csu_pll_ctrl: ReadWrite<u32, CsuPllCtrl::Register>,
    /// PCAP Clock Generator Config
    pub pcap_ctrl: ReadWrite<u32, PcapCtrl::Register>,
    /// AXI Interface Clock Generator Config for LPD Main Switch
    pub lpd_switch_ctrl: ReadWrite<u32, LpdSwitchCtrl::Register>,
    /// APB Interface Clock Generator Config for LPD IOP In/Outbound Switches
    pub lpd_lsbus_ctrl: ReadWrite<u32, LpdLsbusCtrl::Register>,
    /// Debug Clock Generator Config in LPD
    pub dbg_lpd_ctrl: ReadWrite<u32, DbgLpdCtrl::Register>,
    /// NAND Clock Generator Config.
    pub nand_ref_ctrl: ReadWrite<u32, NandRefCtrl::Register>,
    /// LPD DMA Clock Generator Config.
    pub lpd_dma_ref_ctrl: ReadWrite<u32, LpdDmaRefCtrl::Register>,
    _padding188: [u8; 4],
    /// PL 0 Clock Generator Config.
    pub pl0_ref_ctrl: ReadWrite<u32, Pl0RefCtrl::Register>,
    /// PL 1 Clock Generator Config.
    pub pl1_ref_ctrl: ReadWrite<u32, Pl1RefCtrl::Register>,
    /// PL 2 Clock Generator Config.
    pub pl2_ref_ctrl: ReadWrite<u32, Pl2RefCtrl::Register>,
    /// PL 3 Clock Generator Config.
    pub pl3_ref_ctrl: ReadWrite<u32, Pl3RefCtrl::Register>,
    /// PL Clock 0 Threshold Control and status
    pub pl0_thr_ctrl: Aliased<u32, Pl0ThrCtrlR::Register, Pl0ThrCtrlW::Register>,
    /// PL Clock 0 Count Value.
    pub pl0_thr_cnt: ReadWrite<u16>,
    _padding214: [u8; 2],
    /// PL Clock 1 Threshold Control and status
    pub pl1_thr_ctrl: Aliased<u32, Pl1ThrCtrlR::Register, Pl1ThrCtrlW::Register>,
    /// PL Clock 1 Threshold Count Value.
    pub pl1_thr_cnt: ReadWrite<u16>,
    _padding222: [u8; 2],
    /// PL Clock 2 Threshold Control and status
    pub pl2_thr_ctrl: Aliased<u32, Pl2ThrCtrlR::Register, Pl2ThrCtrlW::Register>,
    /// PL Clock 2 Threshold Count Value.
    pub pl2_thr_cnt: ReadWrite<u16>,
    _padding230: [u8; 2],
    /// PL Clock 3 Threshold Control and status
    pub pl3_thr_ctrl: Aliased<u32, Pl3ThrCtrlR::Register, Pl3ThrCtrlW::Register>,
    _padding236: [u8; 16],
    /// PL Clock 3 Threshold Count Value.
    pub pl3_thr_cnt: ReadWrite<u16>,
    _padding254: [u8; 2],
    /// GEM TimeStamp Clock Generator Control.
    pub gem_tsu_ref_ctrl: ReadWrite<u32, GemTsuRefCtrl::Register>,
    /// Clock Generator Control.
    pub dll_ref_ctrl: ReadWrite<u8, DllRefCtrl::Register>,
    _padding261: [u8; 3],
    /// PS SYSMON Clock Generator Control.
    pub pssysmon_ref_ctrl: ReadWrite<u32, PssysmonRefCtrl::Register>,
    _padding268: [u8; 20],
    /// I2C 0 Clock Generator Control.
    pub i2c0_ref_ctrl: ReadWrite<u32, I2c0RefCtrl::Register>,
    /// I2C 1 Clock Generator Control.
    pub i2c1_ref_ctrl: ReadWrite<u32, I2c1RefCtrl::Register>,
    /// Timestamp Clock Generator Control.
    pub timestamp_ref_ctrl: ReadWrite<u32, TimestampRefCtrl::Register>,
    _padding300: [u8; 4],
    /// Safety Endpoint Connectivity Check.
    pub safety_chk: ReadWrite<u32>,
    _padding308: [u8; 12],
    /// Clock Monitor Interrupt Status.
    pub clkmon_status: ReadWrite<u16, ClkmonStatus::Register>,
    _padding322: [u8; 2],
    /// Clock Monitor Interrupt Mask.
    pub clkmon_mask: ReadOnly<u16, ClkmonMask::Register>,
    _padding326: [u8; 2],
    /// Clock Monitor Interrupt Enable.
    pub clkmon_enable: WriteOnly<u16, ClkmonEnable::Register>,
    _padding330: [u8; 2],
    /// Clock Monitor Interrupt Disable.
    pub clkmon_disable: WriteOnly<u16, ClkmonDisable::Register>,
    _padding334: [u8; 2],
    /// Clock Monitor Interrupt Trigger.
    pub clkmon_trigger: WriteOnly<u16, ClkmonTrigger::Register>,
    _padding338: [u8; 14],
    /// Upper Clock Comparison Threshold.
    pub chkr0_clka_upper: ReadWrite<u32>,
    /// Lower Clock Comparison Threshold.
    pub chkr0_clka_lower: ReadWrite<u32>,
    /// CLK B Counting Value.
    pub chkr0_clkb_cnt: ReadWrite<u32>,
    /// Clock Checker 0 Control.
    pub chkr0_ctrl: ReadWrite<u16, Chkr0Ctrl::Register>,
    _padding366: [u8; 2],
    /// Upper Clock Comparison Threshold.
    pub chkr1_clka_upper: ReadWrite<u32>,
    /// Lower Clock Comparison Threshold.
    pub chkr1_clka_lower: ReadWrite<u32>,
    /// CLK B Counting Value.
    pub chkr1_clkb_cnt: ReadWrite<u32>,
    /// Clock Checker 1 Control.
    pub chkr1_ctrl: ReadWrite<u16, Chkr1Ctrl::Register>,
    _padding382: [u8; 2],
    /// Upper Clock Comparison Threshold.
    pub chkr2_clka_upper: ReadWrite<u32>,
    /// Lower Clock Comparison Threshold.
    pub chkr2_clka_lower: ReadWrite<u32>,
    /// CLK B Counting Value.
    pub chkr2_clkb_cnt: ReadWrite<u32>,
    /// Clock Checker 2 Control.
    pub chkr2_ctrl: ReadWrite<u16, Chkr2Ctrl::Register>,
    _padding398: [u8; 2],
    /// Upper Clock Comparison Threshold.
    pub chkr3_clka_upper: ReadWrite<u32>,
    /// Lower Clock Comparison Threshold.
    pub chkr3_clka_lower: ReadWrite<u32>,
    /// CLK B Counting Value.
    pub chkr3_clkb_cnt: ReadWrite<u32>,
    /// Clock Checker 3 Control.
    pub chkr3_ctrl: ReadWrite<u16, Chkr3Ctrl::Register>,
    _padding414: [u8; 2],
    /// Upper Clock Comparison Threshold.
    pub chkr4_clka_upper: ReadWrite<u32>,
    /// Lower Clock Comparison Threshold.
    pub chkr4_clka_lower: ReadWrite<u32>,
    /// CLK B Counting Value.
    pub chkr4_clkb_cnt: ReadWrite<u32>,
    /// Clock Checker 4 Control.
    pub chkr4_ctrl: ReadWrite<u16, Chkr4Ctrl::Register>,
    _padding430: [u8; 2],
    /// Upper Clock Comparison Threshold.
    pub chkr5_clka_upper: ReadWrite<u32>,
    /// Lower Clock Comparison Threshold.
    pub chkr5_clka_lower: ReadWrite<u32>,
    /// CLK B Counting Value.
    pub chkr5_clkb_cnt: ReadWrite<u32>,
    /// Clock Checker 5 Control.
    pub chkr5_ctrl: ReadWrite<u16, Chkr5Ctrl::Register>,
    _padding446: [u8; 2],
    /// Upper Clock Comparison Threshold.
    pub chkr6_clka_upper: ReadWrite<u32>,
    /// Lower Clock Comparison Threshold.
    pub chkr6_clka_lower: ReadWrite<u32>,
    /// CLK B Counting Value.
    pub chkr6_clkb_cnt: ReadWrite<u32>,
    /// Clock Checker 6 H723Control.
    pub chkr6_ctrl: ReadWrite<u16, Chkr6Ctrl::Register>,
    _padding462: [u8; 2],
    /// Upper Clock Comparison Threshold.
    pub chkr7_clka_upper: ReadWrite<u32>,
    /// Lower Clock Comparison Threshold.
    pub chkr7_clka_lower: ReadWrite<u32>,
    /// CLK B Counting Value.
    pub chkr7_clkb_cnt: ReadWrite<u32>,
    /// Clock Checker 7 Control.
    pub chkr7_ctrl: ReadWrite<u16, Chkr7Ctrl::Register>,
    _padding478: [u8; 34],
    /// Software controlled BOOT MODE.
    pub boot_mode_user: Aliased<u32, BootModeUserR::Register, BootModeUserW::Register>,
    /// Hardware controlled BOOT MODE register.
    pub boot_mode_por: Aliased<u16, BootModePorR::Register, BootModePorW::Register>,
    _padding518: [u8; 18],
    /// PS_SRST_B Pin Control and Trigger.
    pub reset_ctrl: ReadWrite<u8, ResetCtrl::Register>,
    _padding537: [u8; 3],
    /// Records the Reason for the Block-only Reset.
    pub blockonly_rst: Aliased<u8, BlockonlyRstR::Register, BlockonlyRstW::Register>,
    _padding541: [u8; 3],
    /// Records the Reason for the Reset.
    pub reset_reason: Aliased<u16, ResetReasonR::Register, ResetReasonW::Register>,
    _padding546: [u8; 14],
    /// Software Reset of Ethernet GEM Controllers
    pub rst_lpd_iou0: ReadWrite<u16, RstLpdIou0::Register>,
    _padding562: [u8; 6],
    /// IOP Software Reset Controls
    pub rst_lpd_iou2: ReadWrite<u32, RstLpdIou2::Register>,
    /// Software Reset Control for LPD System Elements.
    pub rst_lpd_top: ReadWrite<u32, RstLpdTop::Register>,
    /// Debug control for both the LPD and FPD.
    pub rst_lpd_dbg: ReadWrite<u16, RstLpdDbg::Register>,
    _padding578: [u8; 14],
    /// Used to control the mode pins after boot.
    pub boot_pin_ctrl: Aliased<u16, BootPinCtrlR::Register, BootPinCtrlW::Register>,
    _padding594: [u8; 30],
    /// Drive strength control 0 for DIO bank 3
    pub bank3_ctrl0: ReadWrite<u16, Bank3Ctrl0::Register>,
    _padding626: [u8; 2],
    /// Drive strength control 1 for DIO bank 3
    pub bank3_ctrl1: ReadWrite<u16, Bank3Ctrl1::Register>,
    _padding630: [u8; 2],
    /// Schmitt/CMOS input select for DIO bank 3
    pub bank3_ctrl2: ReadWrite<u16, Bank3Ctrl2::Register>,
    _padding634: [u8; 2],
    /// Pull-up/down select for DIO bank 3
    pub bank3_ctrl3: ReadWrite<u16, Bank3Ctrl3::Register>,
    _padding638: [u8; 2],
    /// Pull-up/down enable for DIO bank 3
    pub bank3_ctrl4: ReadWrite<u16, Bank3Ctrl4::Register>,
    _padding642: [u8; 2],
    /// Slew rate control for DIO bank 3
    pub bank3_ctrl5: ReadWrite<u16, Bank3Ctrl5::Register>,
    _padding646: [u8; 2],
    /// Voltage mode status for DIO bank 3
    pub bank3_status: ReadOnly<u16, Bank3Status::Register>,
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
    pub CrlWprot [
        ACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IopllCtrl [
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
    pub IopllCfg [
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
    pub IopllFracCfg [
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
    pub RpllCtrl [
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
    pub RpllCfg [
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
    pub RpllFracCfg [
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
    pub PllStatusR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        RPLL_STABLE OFFSET(4) NUMBITS(1) [],
        IOPLL_STABLE OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        RPLL_LOCK OFFSET(1) NUMBITS(1) [],
        IOPLL_LOCK OFFSET(0) NUMBITS(1) [],
    ],
    pub PllStatusW [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub IopllToFpdCtrl [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub RpllToFpdCtrl [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb3DualRefCtrl [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        CLKACT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0RefCtrl [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        RX_CLKACT OFFSET(26) NUMBITS(1) [],
        CLKACT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1RefCtrl [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        RX_CLKACT OFFSET(26) NUMBITS(1) [],
        CLKACT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2RefCtrl [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        RX_CLKACT OFFSET(26) NUMBITS(1) [],
        CLKACT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3RefCtrl [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        RX_CLKACT OFFSET(26) NUMBITS(1) [],
        CLKACT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0BusRefCtrl [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        CLKACT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1BusRefCtrl [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        CLKACT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(3) [],
        DIVISOR1 OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspiRefCtrl [
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
    pub Sdio0RefCtrl [
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
    pub Sdio1RefCtrl [
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
    pub Uart0RefCtrl [
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
    pub Uart1RefCtrl [
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
    pub Spi0RefCtrl [
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
    pub Spi1RefCtrl [
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
    pub Can0RefCtrl [
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
    pub Can1RefCtrl [
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
    pub CpuR5Ctrl [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        CLKACT_CORE OFFSET(25) NUMBITS(1) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IouSwitchCtrl [
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
    pub CsuPllCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        RESERVED3 OFFSET(3) NUMBITS(1) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcapCtrl [
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
    pub LpdSwitchCtrl [
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
    pub LpdLsbusCtrl [
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
    pub DbgLpdCtrl [
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
    pub NandRefCtrl [
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
    pub LpdDmaRefCtrl [
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
    pub Pl0RefCtrl [
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
    pub Pl1RefCtrl [
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
    pub Pl2RefCtrl [
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
    pub Pl3RefCtrl [
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
    pub Pl0ThrCtrlR [
        CURR_VAL OFFSET(16) NUMBITS(16) [],
        RUNNING OFFSET(15) NUMBITS(1) [],
        RESERVED0 OFFSET(2) NUMBITS(13) [],
        CPU_START OFFSET(1) NUMBITS(1) [],
        CNT_RST OFFSET(0) NUMBITS(1) [],
    ],
    pub Pl0ThrCtrlW [
        RESERVED0 OFFSET(2) NUMBITS(13) [],
        CPU_START OFFSET(1) NUMBITS(1) [],
        CNT_RST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pl1ThrCtrlR [
        CURR_VAL OFFSET(16) NUMBITS(16) [],
        RUNNING OFFSET(15) NUMBITS(1) [],
        RESERVED0 OFFSET(2) NUMBITS(13) [],
        CPU_START OFFSET(1) NUMBITS(1) [],
        CNT_RST OFFSET(0) NUMBITS(1) [],
    ],
    pub Pl1ThrCtrlW [
        RESERVED0 OFFSET(2) NUMBITS(13) [],
        CPU_START OFFSET(1) NUMBITS(1) [],
        CNT_RST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pl2ThrCtrlR [
        CURR_VAL OFFSET(16) NUMBITS(16) [],
        RUNNING OFFSET(15) NUMBITS(1) [],
        RESERVED0 OFFSET(2) NUMBITS(13) [],
        CPU_START OFFSET(1) NUMBITS(1) [],
        CNT_RST OFFSET(0) NUMBITS(1) [],
    ],
    pub Pl2ThrCtrlW [
        RESERVED0 OFFSET(2) NUMBITS(13) [],
        CPU_START OFFSET(1) NUMBITS(1) [],
        CNT_RST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pl3ThrCtrlR [
        CURR_VAL OFFSET(16) NUMBITS(16) [],
        RUNNING OFFSET(15) NUMBITS(1) [],
        RESERVED0 OFFSET(2) NUMBITS(13) [],
        CPU_START OFFSET(1) NUMBITS(1) [],
        CNT_RST OFFSET(0) NUMBITS(1) [],
    ],
    pub Pl3ThrCtrlW [
        RESERVED0 OFFSET(2) NUMBITS(13) [],
        CPU_START OFFSET(1) NUMBITS(1) [],
        CNT_RST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GemTsuRefCtrl [
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
    u8,
    pub DllRefCtrl [
        RESERVED0 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PssysmonRefCtrl [
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
    pub I2c0RefCtrl [
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
    pub I2c1RefCtrl [
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
    pub TimestampRefCtrl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        CLKACT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(10) [],
        DIVISOR0 OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(3) NUMBITS(5) [],
        SRCSEL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ClkmonStatus [
        CNTA7_OVER_ERR OFFSET(15) NUMBITS(1) [],
        MON7_ERR OFFSET(14) NUMBITS(1) [],
        CNTA6_OVER_ERR OFFSET(13) NUMBITS(1) [],
        MON6_ERR OFFSET(12) NUMBITS(1) [],
        CNTA5_OVER_ERR OFFSET(11) NUMBITS(1) [],
        MON5_ERR OFFSET(10) NUMBITS(1) [],
        CNTA4_OVER_ERR OFFSET(9) NUMBITS(1) [],
        MON4_ERR OFFSET(8) NUMBITS(1) [],
        CNTA3_OVER_ERR OFFSET(7) NUMBITS(1) [],
        MON3_ERR OFFSET(6) NUMBITS(1) [],
        CNTA2_OVER_ERR OFFSET(5) NUMBITS(1) [],
        MON2_ERR OFFSET(4) NUMBITS(1) [],
        CNTA1_OVER_ERR OFFSET(3) NUMBITS(1) [],
        MON1_ERR OFFSET(2) NUMBITS(1) [],
        CNTA0_OVER_ERR OFFSET(1) NUMBITS(1) [],
        MON0_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ClkmonMask [
        CNTA7_OVER_ERR OFFSET(15) NUMBITS(1) [],
        MON7_ERR OFFSET(14) NUMBITS(1) [],
        CNTA6_OVER_ERR OFFSET(13) NUMBITS(1) [],
        MON6_ERR OFFSET(12) NUMBITS(1) [],
        CNTA5_OVER_ERR OFFSET(11) NUMBITS(1) [],
        MON5_ERR OFFSET(10) NUMBITS(1) [],
        CNTA4_OVER_ERR OFFSET(9) NUMBITS(1) [],
        MON4_ERR OFFSET(8) NUMBITS(1) [],
        CNTA3_OVER_ERR OFFSET(7) NUMBITS(1) [],
        MON3_ERR OFFSET(6) NUMBITS(1) [],
        CNTA2_OVER_ERR OFFSET(5) NUMBITS(1) [],
        MON2_ERR OFFSET(4) NUMBITS(1) [],
        CNTA1_OVER_ERR OFFSET(3) NUMBITS(1) [],
        MON1_ERR OFFSET(2) NUMBITS(1) [],
        CNTA0_OVER_ERR OFFSET(1) NUMBITS(1) [],
        MON0_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ClkmonEnable [
        CNTA7_OVER_ERR OFFSET(15) NUMBITS(1) [],
        MON7_ERR OFFSET(14) NUMBITS(1) [],
        CNTA6_OVER_ERR OFFSET(13) NUMBITS(1) [],
        MON6_ERR OFFSET(12) NUMBITS(1) [],
        CNTA5_OVER_ERR OFFSET(11) NUMBITS(1) [],
        MON5_ERR OFFSET(10) NUMBITS(1) [],
        CNTA4_OVER_ERR OFFSET(9) NUMBITS(1) [],
        MON4_ERR OFFSET(8) NUMBITS(1) [],
        CNTA3_OVER_ERR OFFSET(7) NUMBITS(1) [],
        MON3_ERR OFFSET(6) NUMBITS(1) [],
        CNTA2_OVER_ERR OFFSET(5) NUMBITS(1) [],
        MON2_ERR OFFSET(4) NUMBITS(1) [],
        CNTA1_OVER_ERR OFFSET(3) NUMBITS(1) [],
        MON1_ERR OFFSET(2) NUMBITS(1) [],
        CNTA0_OVER_ERR OFFSET(1) NUMBITS(1) [],
        MON0_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ClkmonDisable [
        CNTA7_OVER_ERR OFFSET(15) NUMBITS(1) [],
        MON7_ERR OFFSET(14) NUMBITS(1) [],
        CNTA6_OVER_ERR OFFSET(13) NUMBITS(1) [],
        MON6_ERR OFFSET(12) NUMBITS(1) [],
        CNTA5_OVER_ERR OFFSET(11) NUMBITS(1) [],
        MON5_ERR OFFSET(10) NUMBITS(1) [],
        CNTA4_OVER_ERR OFFSET(9) NUMBITS(1) [],
        MON4_ERR OFFSET(8) NUMBITS(1) [],
        CNTA3_OVER_ERR OFFSET(7) NUMBITS(1) [],
        MON3_ERR OFFSET(6) NUMBITS(1) [],
        CNTA2_OVER_ERR OFFSET(5) NUMBITS(1) [],
        MON2_ERR OFFSET(4) NUMBITS(1) [],
        CNTA1_OVER_ERR OFFSET(3) NUMBITS(1) [],
        MON1_ERR OFFSET(2) NUMBITS(1) [],
        CNTA0_OVER_ERR OFFSET(1) NUMBITS(1) [],
        MON0_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ClkmonTrigger [
        CNTA7_OVER_ERR OFFSET(15) NUMBITS(1) [],
        MON7_ERR OFFSET(14) NUMBITS(1) [],
        CNTA6_OVER_ERR OFFSET(13) NUMBITS(1) [],
        MON6_ERR OFFSET(12) NUMBITS(1) [],
        CNTA5_OVER_ERR OFFSET(11) NUMBITS(1) [],
        MON5_ERR OFFSET(10) NUMBITS(1) [],
        CNTA4_OVER_ERR OFFSET(9) NUMBITS(1) [],
        MON4_ERR OFFSET(8) NUMBITS(1) [],
        CNTA3_OVER_ERR OFFSET(7) NUMBITS(1) [],
        MON3_ERR OFFSET(6) NUMBITS(1) [],
        CNTA2_OVER_ERR OFFSET(5) NUMBITS(1) [],
        MON2_ERR OFFSET(4) NUMBITS(1) [],
        CNTA1_OVER_ERR OFFSET(3) NUMBITS(1) [],
        MON1_ERR OFFSET(2) NUMBITS(1) [],
        CNTA0_OVER_ERR OFFSET(1) NUMBITS(1) [],
        MON0_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Chkr0Ctrl [
        START_SINGLE OFFSET(8) NUMBITS(1) [],
        START_CONTINUOUS OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        CLKB_MUX_CTRL OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        CLKA_MUX_CTRL OFFSET(1) NUMBITS(3) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Chkr1Ctrl [
        START_SINGLE OFFSET(8) NUMBITS(1) [],
        START_CONTINUOUS OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        CLKB_MUX_CTRL OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        CLKA_MUX_CTRL OFFSET(1) NUMBITS(3) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Chkr2Ctrl [
        START_SINGLE OFFSET(8) NUMBITS(1) [],
        START_CONTINUOUS OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        CLKB_MUX_CTRL OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        CLKA_MUX_CTRL OFFSET(1) NUMBITS(3) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Chkr3Ctrl [
        START_SINGLE OFFSET(8) NUMBITS(1) [],
        START_CONTINUOUS OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        CLKB_MUX_CTRL OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        CLKA_MUX_CTRL OFFSET(1) NUMBITS(3) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Chkr4Ctrl [
        START_SINGLE OFFSET(8) NUMBITS(1) [],
        START_CONTINUOUS OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        CLKB_MUX_CTRL OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        CLKA_MUX_CTRL OFFSET(1) NUMBITS(3) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Chkr5Ctrl [
        START_SINGLE OFFSET(8) NUMBITS(1) [],
        START_CONTINUOUS OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        CLKB_MUX_CTRL OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        CLKA_MUX_CTRL OFFSET(1) NUMBITS(3) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Chkr6Ctrl [
        START_SINGLE OFFSET(8) NUMBITS(1) [],
        START_CONTINUOUS OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        CLKB_MUX_CTRL OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        CLKA_MUX_CTRL OFFSET(1) NUMBITS(3) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Chkr7Ctrl [
        START_SINGLE OFFSET(8) NUMBITS(1) [],
        START_CONTINUOUS OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        CLKB_MUX_CTRL OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        CLKA_MUX_CTRL OFFSET(1) NUMBITS(3) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BootModeUserR [
        RESERVED0 OFFSET(16) NUMBITS(4) [],
        ALT_BOOT_MODE OFFSET(12) NUMBITS(4) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        USE_ALT OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        BOOT_MODE OFFSET(0) NUMBITS(4) [],
    ],
    pub BootModeUserW [
        RESERVED0 OFFSET(16) NUMBITS(4) [],
        ALT_BOOT_MODE OFFSET(12) NUMBITS(4) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        USE_ALT OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub BootModePorR [
        RESERVED0 OFFSET(12) NUMBITS(4) [],
        BOOT_MODE2 OFFSET(8) NUMBITS(4) [],
        BOOT_MODE1 OFFSET(4) NUMBITS(4) [],
        BOOT_MODE0 OFFSET(0) NUMBITS(4) [],
    ],
    pub BootModePorW [
        RESERVED0 OFFSET(12) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ResetCtrl [
        RESERVED0 OFFSET(5) NUMBITS(3) [],
        SOFT_RESET OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(2) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        SRST_DIS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub BlockonlyRstR [
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(2) [],
        DEBUG_ONLY OFFSET(0) NUMBITS(1) [],
    ],
    pub BlockonlyRstW [
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        DEBUG_ONLY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ResetReasonR [
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(8) [],
        DEBUG_SYS OFFSET(6) NUMBITS(1) [],
        SOFT OFFSET(5) NUMBITS(1) [],
        SRST OFFSET(4) NUMBITS(1) [],
        PSONLY_RESET_REQ OFFSET(3) NUMBITS(1) [],
        PMU_SYS_RESET OFFSET(2) NUMBITS(1) [],
        INTERNAL_POR OFFSET(1) NUMBITS(1) [],
        EXTERNAL_POR OFFSET(0) NUMBITS(1) [],
    ],
    pub ResetReasonW [
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        DEBUG_SYS OFFSET(6) NUMBITS(1) [],
        SOFT OFFSET(5) NUMBITS(1) [],
        SRST OFFSET(4) NUMBITS(1) [],
        PSONLY_RESET_REQ OFFSET(3) NUMBITS(1) [],
        PMU_SYS_RESET OFFSET(2) NUMBITS(1) [],
        INTERNAL_POR OFFSET(1) NUMBITS(1) [],
        EXTERNAL_POR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub RstLpdIou0 [
        RESERVED0 OFFSET(4) NUMBITS(12) [],
        GEM3_RESET OFFSET(3) NUMBITS(1) [],
        GEM2_RESET OFFSET(2) NUMBITS(1) [],
        GEM1_RESET OFFSET(1) NUMBITS(1) [],
        GEM0_RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RstLpdIou2 [
        RESERVED0 OFFSET(21) NUMBITS(11) [],
        TIMESTAMP_RESET OFFSET(20) NUMBITS(1) [],
        IOU_CC_RESET OFFSET(19) NUMBITS(1) [],
        GPIO_RESET OFFSET(18) NUMBITS(1) [],
        LPD_DMA_RESET OFFSET(17) NUMBITS(1) [],
        NAND_RESET OFFSET(16) NUMBITS(1) [],
        SWDT_RESET OFFSET(15) NUMBITS(1) [],
        TTC3_RESET OFFSET(14) NUMBITS(1) [],
        TTC2_RESET OFFSET(13) NUMBITS(1) [],
        TTC1_RESET OFFSET(12) NUMBITS(1) [],
        TTC0_RESET OFFSET(11) NUMBITS(1) [],
        I2C1_RESET OFFSET(10) NUMBITS(1) [],
        I2C0_RESET OFFSET(9) NUMBITS(1) [],
        CAN1_RESET OFFSET(8) NUMBITS(1) [],
        CAN0_RESET OFFSET(7) NUMBITS(1) [],
        SDIO1_RESET OFFSET(6) NUMBITS(1) [],
        SDIO0_RESET OFFSET(5) NUMBITS(1) [],
        SPI1_RESET OFFSET(4) NUMBITS(1) [],
        SPI0_RESET OFFSET(3) NUMBITS(1) [],
        UART1_RESET OFFSET(2) NUMBITS(1) [],
        UART0_RESET OFFSET(1) NUMBITS(1) [],
        QSPI_RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RstLpdTop [
        FPD_RESET OFFSET(23) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(2) [],
        LPD_SWDT_RESET OFFSET(20) NUMBITS(1) [],
        S_AXI_LPD_RESET OFFSET(19) NUMBITS(1) [],
        RESERVED1 OFFSET(18) NUMBITS(1) [],
        SYSMON_RESET OFFSET(17) NUMBITS(1) [],
        RTC_RESET OFFSET(16) NUMBITS(1) [],
        APM_RESET OFFSET(15) NUMBITS(1) [],
        IPI_RESET OFFSET(14) NUMBITS(1) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        USB1_APB_RESET OFFSET(11) NUMBITS(1) [],
        USB0_APB_RESET OFFSET(10) NUMBITS(1) [],
        USB1_HIBERRESET OFFSET(9) NUMBITS(1) [],
        USB0_HIBERRESET OFFSET(8) NUMBITS(1) [],
        USB1_CORERESET OFFSET(7) NUMBITS(1) [],
        USB0_CORERESET OFFSET(6) NUMBITS(1) [],
        RESERVED3 OFFSET(5) NUMBITS(1) [],
        RPU_PGE_RESET OFFSET(4) NUMBITS(1) [],
        OCM_RESET OFFSET(3) NUMBITS(1) [],
        RPU_AMBA_RESET OFFSET(2) NUMBITS(1) [],
        RPU_R51_RESET OFFSET(1) NUMBITS(1) [],
        RPU_R50_RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub RstLpdDbg [
        DBG_ACK OFFSET(15) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(9) [],
        RPU_DBG1_RESET OFFSET(5) NUMBITS(1) [],
        RPU_DBG0_RESET OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(2) [],
        DBG_LPD_RESET OFFSET(1) NUMBITS(1) [],
        DBG_FPD_RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub BootPinCtrlR [
        RESERVED0 OFFSET(12) NUMBITS(4) [],
        OUT_VAL OFFSET(8) NUMBITS(4) [],
        IN_VAL OFFSET(4) NUMBITS(4) [],
        OUT_EN OFFSET(0) NUMBITS(4) [],
    ],
    pub BootPinCtrlW [
        RESERVED0 OFFSET(12) NUMBITS(4) [],
        OUT_VAL OFFSET(8) NUMBITS(4) [],
        OUT_EN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Bank3Ctrl0 [
        DRIVE0 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Bank3Ctrl1 [
        DRIVE1 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Bank3Ctrl2 [
        SCHMITT_CMOS_N OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Bank3Ctrl3 [
        PULL_HIGH_LOW_N OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Bank3Ctrl4 [
        PULL_ENABLE OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Bank3Ctrl5 [
        SLOW_FAST_SLEW_N OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub Bank3Status [
        VMODE_1P8_3P3_N OFFSET(0) NUMBITS(10) [],
    ]
];

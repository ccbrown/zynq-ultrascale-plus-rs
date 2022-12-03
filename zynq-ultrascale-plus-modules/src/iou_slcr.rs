// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// IOP System-level Control, IOP System-level Control
pub static mut IOU_SLCR: *mut Registers = 0xff180000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// MIO Device Pin 0 Multiplexer Controls.
    pub mio_pin_0: ReadWrite<u32, MioPin0::Register>,
    /// MIO Device Pin 1 Multiplexer Controls.
    pub mio_pin_1: ReadWrite<u32, MioPin1::Register>,
    /// MIO Device Pin 2 Multiplexer Controls.
    pub mio_pin_2: ReadWrite<u32, MioPin2::Register>,
    /// MIO Device Pin 3 Multiplexer Controls.
    pub mio_pin_3: ReadWrite<u32, MioPin3::Register>,
    /// MIO Device Pin 4 Multiplexer Controls.
    pub mio_pin_4: ReadWrite<u32, MioPin4::Register>,
    /// Configures MIO Pin 5 peripheral interface mapping
    pub mio_pin_5: ReadWrite<u32, MioPin5::Register>,
    /// MIO Device Pin 6 Multiplexer Controls.
    pub mio_pin_6: ReadWrite<u32, MioPin6::Register>,
    /// MIO Device Pin 7 Multiplexer Controls.
    pub mio_pin_7: ReadWrite<u32, MioPin7::Register>,
    /// Configures MIO Pin 8 peripheral interface mapping
    pub mio_pin_8: ReadWrite<u32, MioPin8::Register>,
    /// MIO Device Pin 9 Multiplexer Controls.
    pub mio_pin_9: ReadWrite<u32, MioPin9::Register>,
    /// MIO Device Pin 10 Multiplexer Controls.
    pub mio_pin_10: ReadWrite<u32, MioPin10::Register>,
    /// MIO Device Pin 11 Multiplexer Controls.
    pub mio_pin_11: ReadWrite<u32, MioPin11::Register>,
    /// MIO Device Pin 12 Multiplexer Controls.
    pub mio_pin_12: ReadWrite<u32, MioPin12::Register>,
    /// MIO Device Pin 13 Multiplexer Controls.
    pub mio_pin_13: ReadWrite<u32, MioPin13::Register>,
    /// MIO Device Pin 14 Multiplexer Controls.
    pub mio_pin_14: ReadWrite<u32, MioPin14::Register>,
    /// MIO Device Pin 15 Multiplexer Controls.
    pub mio_pin_15: ReadWrite<u32, MioPin15::Register>,
    /// MIO Device Pin 16 Multiplexer Controls.
    pub mio_pin_16: ReadWrite<u32, MioPin16::Register>,
    /// MIO Device Pin 17 Multiplexer Controls.
    pub mio_pin_17: ReadWrite<u32, MioPin17::Register>,
    /// MIO Device Pin 18 Multiplexer Controls.
    pub mio_pin_18: ReadWrite<u32, MioPin18::Register>,
    /// MIO Device Pin 19 Multiplexer Controls.
    pub mio_pin_19: ReadWrite<u32, MioPin19::Register>,
    /// MIO Device Pin 20 Multiplexer Controls.
    pub mio_pin_20: ReadWrite<u32, MioPin20::Register>,
    /// MIO Device Pin 21 Multiplexer Controls.
    pub mio_pin_21: ReadWrite<u32, MioPin21::Register>,
    /// MIO Device Pin 22 Multiplexer Controls.
    pub mio_pin_22: ReadWrite<u32, MioPin22::Register>,
    /// MIO Device Pin 23 Multiplexer Controls.
    pub mio_pin_23: ReadWrite<u32, MioPin23::Register>,
    /// MIO Device Pin 24 Multiplexer Controls.
    pub mio_pin_24: ReadWrite<u32, MioPin24::Register>,
    /// MIO Device Pin 25 Multiplexer Controls.
    pub mio_pin_25: ReadWrite<u32, MioPin25::Register>,
    /// MIO Device Pin 26 Multiplexer Controls.
    pub mio_pin_26: ReadWrite<u32, MioPin26::Register>,
    /// MIO Device Pin 27 Multiplexer Controls.
    pub mio_pin_27: ReadWrite<u32, MioPin27::Register>,
    /// MIO Device Pin 28 Multiplexer Controls.
    pub mio_pin_28: ReadWrite<u32, MioPin28::Register>,
    /// MIO Device Pin 29 Multiplexer Controls.
    pub mio_pin_29: ReadWrite<u32, MioPin29::Register>,
    /// MIO Device Pin 30 Multiplexer Controls.
    pub mio_pin_30: ReadWrite<u32, MioPin30::Register>,
    /// MIO Device Pin 31 Multiplexer Controls.
    pub mio_pin_31: ReadWrite<u32, MioPin31::Register>,
    /// MIO Device Pin 32 Multiplexer Controls.
    pub mio_pin_32: ReadWrite<u32, MioPin32::Register>,
    /// MIO Device Pin 33 Multiplexer Controls.
    pub mio_pin_33: ReadWrite<u32, MioPin33::Register>,
    /// MIO Device Pin 34 Multiplexer Controls.
    pub mio_pin_34: ReadWrite<u32, MioPin34::Register>,
    /// MIO Device Pin 35 Multiplexer Controls.
    pub mio_pin_35: ReadWrite<u32, MioPin35::Register>,
    /// MIO Device Pin 36 Multiplexer Controls.
    pub mio_pin_36: ReadWrite<u32, MioPin36::Register>,
    /// MIO Device Pin 37 Multiplexer Controls.
    pub mio_pin_37: ReadWrite<u32, MioPin37::Register>,
    /// MIO Device Pin 38 Multiplexer Controls.
    pub mio_pin_38: ReadWrite<u32, MioPin38::Register>,
    /// MIO Device Pin 39 Multiplexer Controls.
    pub mio_pin_39: ReadWrite<u32, MioPin39::Register>,
    /// MIO Device Pin 40 Multiplexer Controls.
    pub mio_pin_40: ReadWrite<u32, MioPin40::Register>,
    /// MIO Device Pin 41 Multiplexer Controls.
    pub mio_pin_41: ReadWrite<u32, MioPin41::Register>,
    /// MIO Device Pin 42 Multiplexer Controls.
    pub mio_pin_42: ReadWrite<u32, MioPin42::Register>,
    /// MIO Device Pin 43 Multiplexer Controls.
    pub mio_pin_43: ReadWrite<u32, MioPin43::Register>,
    /// MIO Device Pin 44 Multiplexer Controls.
    pub mio_pin_44: ReadWrite<u32, MioPin44::Register>,
    /// MIO Device Pin 45 Multiplexer Controls.
    pub mio_pin_45: ReadWrite<u32, MioPin45::Register>,
    /// MIO Device Pin 46 Multiplexer Controls.
    pub mio_pin_46: ReadWrite<u32, MioPin46::Register>,
    /// MIO Device Pin 47 Multiplexer Controls.
    pub mio_pin_47: ReadWrite<u32, MioPin47::Register>,
    /// MIO Device Pin 48 Multiplexer Controls.
    pub mio_pin_48: ReadWrite<u32, MioPin48::Register>,
    /// MIO Device Pin 49 Multiplexer Controls.
    pub mio_pin_49: ReadWrite<u32, MioPin49::Register>,
    /// MIO Device Pin 50 Multiplexer Controls.
    pub mio_pin_50: ReadWrite<u32, MioPin50::Register>,
    /// MIO Device Pin 51 Multiplexer Controls.
    pub mio_pin_51: ReadWrite<u32, MioPin51::Register>,
    /// MIO Device Pin 52 Multiplexer Controls.
    pub mio_pin_52: ReadWrite<u32, MioPin52::Register>,
    /// MIO Device Pin 53 Multiplexer Controls.
    pub mio_pin_53: ReadWrite<u32, MioPin53::Register>,
    /// MIO Device Pin 54 Multiplexer Controls.
    pub mio_pin_54: ReadWrite<u32, MioPin54::Register>,
    /// MIO Device Pin 55 Multiplexer Controls.
    pub mio_pin_55: ReadWrite<u32, MioPin55::Register>,
    /// MIO Device Pin 56 Multiplexer Controls.
    pub mio_pin_56: ReadWrite<u32, MioPin56::Register>,
    /// MIO Device Pin 57 Multiplexer Controls.
    pub mio_pin_57: ReadWrite<u32, MioPin57::Register>,
    /// MIO Device Pin 58 Multiplexer Controls.
    pub mio_pin_58: ReadWrite<u32, MioPin58::Register>,
    /// MIO Device Pin 59 Multiplexer Controls.
    pub mio_pin_59: ReadWrite<u32, MioPin59::Register>,
    /// MIO Device Pin 60 Multiplexer Controls.
    pub mio_pin_60: ReadWrite<u32, MioPin60::Register>,
    /// MIO Device Pin 61 Multiplexer Controls.
    pub mio_pin_61: ReadWrite<u32, MioPin61::Register>,
    /// MIO Device Pin 62 Multiplexer Controls.
    pub mio_pin_62: ReadWrite<u32, MioPin62::Register>,
    /// MIO Device Pin 63 Multiplexer Controls.
    pub mio_pin_63: ReadWrite<u32, MioPin63::Register>,
    /// MIO Device Pin 64 Multiplexer Controls.
    pub mio_pin_64: ReadWrite<u32, MioPin64::Register>,
    /// MIO Device Pin 65 Multiplexer Controls.
    pub mio_pin_65: ReadWrite<u32, MioPin65::Register>,
    /// MIO Device Pin 66 Multiplexer Controls.
    pub mio_pin_66: ReadWrite<u32, MioPin66::Register>,
    /// MIO Device Pin 67 Multiplexer Controls.
    pub mio_pin_67: ReadWrite<u32, MioPin67::Register>,
    /// MIO Device Pin 68 Multiplexer Controls.
    pub mio_pin_68: ReadWrite<u32, MioPin68::Register>,
    /// MIO Device Pin 69 Multiplexer Controls.
    pub mio_pin_69: ReadWrite<u32, MioPin69::Register>,
    /// MIO Device Pin 70 Multiplexer Controls.
    pub mio_pin_70: ReadWrite<u32, MioPin70::Register>,
    /// MIO Device Pin 71 Multiplexer Controls.
    pub mio_pin_71: ReadWrite<u32, MioPin71::Register>,
    /// MIO Device Pin 72 Multiplexer Controls.
    pub mio_pin_72: ReadWrite<u32, MioPin72::Register>,
    /// MIO Device Pin 73 Multiplexer Controls.
    pub mio_pin_73: ReadWrite<u32, MioPin73::Register>,
    /// MIO Device Pin 74 Multiplexer Controls.
    pub mio_pin_74: ReadWrite<u32, MioPin74::Register>,
    /// MIO Device Pin 75 Multiplexer Controls.
    pub mio_pin_75: ReadWrite<u32, MioPin75::Register>,
    /// MIO Device Pin 76 Multiplexer Controls.
    pub mio_pin_76: ReadWrite<u32, MioPin76::Register>,
    /// MIO Device Pin 77 Multiplexer Controls.
    pub mio_pin_77: ReadWrite<u32, MioPin77::Register>,
    /// MIO Bank 0, Drive 0 control.
    pub bank0_ctrl0: ReadWrite<u32>,
    /// MIO Bank 0, Drive 1 control.
    pub bank0_ctrl1: ReadWrite<u32>,
    /// MIO Bank 0, CMOS input type control.
    pub bank0_ctrl3: ReadWrite<u32>,
    /// MIO Bank 0, Pull up/down select.
    pub bank0_ctrl4: ReadWrite<u32>,
    /// MIO Bank 0, Pull up/down enable.
    pub bank0_ctrl5: ReadWrite<u32>,
    /// MIO Bank 0, Output slew rate select.
    pub bank0_ctrl6: ReadWrite<u32>,
    /// MIO Bank 0, Voltage detection.
    pub bank0_status: ReadOnly<u8, Bank0Status::Register>,
    _padding337: [u8; 3],
    /// MIO Bank 1, Drive 0 control.
    pub bank1_ctrl0: ReadWrite<u32>,
    /// MIO Bank 1, Drive 1 control.
    pub bank1_ctrl1: ReadWrite<u32>,
    /// MIO Bank 1, CMOS input type control.
    pub bank1_ctrl3: ReadWrite<u32>,
    /// MIO Bank 1, Pull up/down select.
    pub bank1_ctrl4: ReadWrite<u32>,
    /// MIO Bank 1, Pull up/down enable.
    pub bank1_ctrl5: ReadWrite<u32>,
    /// MIO Bank 1, Output slew rate select.
    pub bank1_ctrl6: ReadWrite<u32>,
    /// MIO Bank 1, Voltage detection.
    pub bank1_status: ReadOnly<u8, Bank1Status::Register>,
    _padding365: [u8; 3],
    /// MIO Bank 2, Drive 0 control.
    pub bank2_ctrl0: ReadWrite<u32>,
    /// MIO Bank 2, Drive 1 control.
    pub bank2_ctrl1: ReadWrite<u32>,
    /// MIO Bank 2, CMOS input type control.
    pub bank2_ctrl3: ReadWrite<u32>,
    /// MIO Bank 2, Pull up/down select.
    pub bank2_ctrl4: ReadWrite<u32>,
    /// MIO Bank 2, Pull up/down enable.
    pub bank2_ctrl5: ReadWrite<u32>,
    /// MIO Bank 2, Output slew rate select.
    pub bank2_ctrl6: ReadWrite<u32>,
    /// MIO Bank 2, Voltage detection.
    pub bank2_status: ReadOnly<u8, Bank2Status::Register>,
    _padding393: [u8; 119],
    /// Loopback function within MIO
    pub mio_loopback: Aliased<u32, MioLoopbackR::Register, MioLoopbackW::Register>,
    /// MIO pin Tri-state Enables, 31:0
    pub mio_mst_tri0: ReadWrite<u32, MioMstTri0::Register>,
    /// MIO pin Tri-state Enables, 63:32
    pub mio_mst_tri1: ReadWrite<u32, MioMstTri1::Register>,
    /// MIO pin Tri-state Enables, 77:64
    pub mio_mst_tri2: Aliased<u32, MioMstTri2R::Register, MioMstTri2W::Register>,
    _padding528: [u8; 240],
    /// SWDT clock source select
    pub wdt_clk_sel: Aliased<u32, WdtClkSelR::Register, WdtClkSelW::Register>,
    /// CAN MIO Control
    pub can_mio_ctrl: Aliased<u32, CanMioCtrlR::Register, CanMioCtrlW::Register>,
    /// GEM I/O Clock Control
    pub gem_clk_ctrl: Aliased<u32, GemClkCtrlR::Register, GemClkCtrlW::Register>,
    /// SD I/O Clock Control
    pub sdio_clk_ctrl: Aliased<u32, SdioClkCtrlR::Register, SdioClkCtrlW::Register>,
    /// SD eMMC selection
    pub ctrl_reg_sd: Aliased<u32, CtrlRegSdR::Register, CtrlRegSdW::Register>,
    /// Input Tap Delay Select
    pub sd_itapdly: Aliased<u32, SdItapdlyR::Register, SdItapdlyW::Register>,
    /// Output Tap Delay Select
    pub sd_otapdlysel: Aliased<u32, SdOtapdlyselR::Register, SdOtapdlyselW::Register>,
    /// SD Configuration, Reg 1.
    pub sd_config_reg1: Aliased<u32, SdConfigReg1R::Register, SdConfigReg1W::Register>,
    /// SD Configuration, Reg 2.
    pub sd_config_reg2: Aliased<u32, SdConfigReg2R::Register, SdConfigReg2W::Register>,
    /// SD Configuration, Reg 3.
    pub sd_config_reg3: Aliased<u32, SdConfigReg3R::Register, SdConfigReg3W::Register>,
    /// Preset Value for Initialization
    pub sd_initpreset: Aliased<u32, SdInitpresetR::Register, SdInitpresetW::Register>,
    /// Preset Value for Default Speed
    pub sd_dsppreset: Aliased<u32, SdDsppresetR::Register, SdDsppresetW::Register>,
    /// Preset Value for High Speed
    pub sd_hspdpreset: Aliased<u32, SdHspdpresetR::Register, SdHspdpresetW::Register>,
    /// Preset Value for SDR12
    pub sd_sdr12preset: Aliased<u32, SdSdr12presetR::Register, SdSdr12presetW::Register>,
    /// Preset Value for SDR25
    pub sd_sdr25preset: Aliased<u32, SdSdr25presetR::Register, SdSdr25presetW::Register>,
    /// Preset Value for SDR50
    pub sd_sdr50prset: Aliased<u32, SdSdr50prsetR::Register, SdSdr50prsetW::Register>,
    /// Preset Value for SDR104
    pub sd_sdr104prst: Aliased<u32, SdSdr104prstR::Register, SdSdr104prstW::Register>,
    /// Preset Value for DDR50
    pub sd_ddr50preset: Aliased<u32, SdDdr50presetR::Register, SdDdr50presetW::Register>,
    _padding840: [u8; 4],
    /// Maximum Current for 1.8V
    pub sd_maxcur1p8: Aliased<u32, SdMaxcur1p8R::Register, SdMaxcur1p8W::Register>,
    /// Maximum Current for 3.0V
    pub sd_maxcur3p0: Aliased<u32, SdMaxcur3p0R::Register, SdMaxcur3p0W::Register>,
    /// Maximum Current for 3.3V
    pub sd_maxcur3p3: Aliased<u32, SdMaxcur3p3R::Register, SdMaxcur3p3W::Register>,
    /// SDIO status register
    pub sd_dll_ctrl: Aliased<u32, SdDllCtrlR::Register, SdDllCtrlW::Register>,
    /// SDIO Card Detect (CDn) connection:
    pub sd_cdn_ctrl: Aliased<u32, SdCdnCtrlR::Register, SdCdnCtrlW::Register>,
    /// GEM SGMII Signal Detect (PCS) connection:
    pub gem_ctrl: Aliased<u32, GemCtrlR::Register, GemCtrlW::Register>,
    _padding868: [u8; 28],
    /// TTC APB Interface Clock Select
    pub iou_ttc_apb_clk: Aliased<u32, IouTtcApbClkR::Register, IouTtcApbClkW::Register>,
    _padding900: [u8; 12],
    /// Tap Delay enables for the LQSPI and NAND controllers.
    pub iou_tapdly_bypass: Aliased<u32, IouTapdlyBypassR::Register, IouTapdlyBypassW::Register>,
    _padding916: [u8; 108],
    /// AXI Coherency selection
    pub iou_coherent_ctrl: ReadWrite<u32, IouCoherentCtrl::Register>,
    /// Select VIDEO_REF_CLK and ALT_REF_CLK from MIO.
    pub video_pss_clk_sel: Aliased<u32, VideoPssClkSelR::Register, VideoPssClkSelW::Register>,
    /// Transaction Routing to Memory for DMA masters in IOP.
    pub iou_interconnect_route:
        Aliased<u32, IouInterconnectRouteR::Register, IouInterconnectRouteW::Register>,
    _padding1036: [u8; 500],
    /// General control register for the IOU SLCR
    pub ctrl: ReadWrite<u8, Ctrl::Register>,
    _padding1537: [u8; 255],
    /// Address Decode Error Interrupt Status
    pub isr: ReadWrite<u8, Isr::Register>,
    _padding1793: [u8; 3],
    /// Address Decode Error Interrupt Mask
    pub imr: ReadOnly<u8, Imr::Register>,
    _padding1797: [u8; 3],
    /// Address Decode Error Interrupt Enable
    pub ier: WriteOnly<u8, Ier::Register>,
    _padding1801: [u8; 3],
    /// Address Decode Error Interrupt Disable
    pub idr: WriteOnly<u8, Idr::Register>,
    _padding1805: [u8; 3],
    /// Address Decode Error Interrupt Trigger
    pub itr: WriteOnly<u8, Itr::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub MioPin0 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin1 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin2 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin3 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin4 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin5 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin6 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin7 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin8 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin9 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin10 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin11 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin12 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin13 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin14 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin15 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin16 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin17 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin18 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin19 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin20 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin21 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin22 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin23 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin24 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin25 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin26 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin27 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin28 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin29 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin30 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin31 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin32 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin33 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin34 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin35 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin36 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin37 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin38 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin39 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin40 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin41 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin42 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin43 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin44 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin45 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin46 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin47 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin48 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin49 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin50 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin51 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin52 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin53 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin54 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin55 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin56 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin57 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin58 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin59 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin60 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin61 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin62 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin63 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin64 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin65 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin66 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin67 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin68 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin69 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin70 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin71 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin72 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin73 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin74 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin75 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin76 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioPin77 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        L3_SEL OFFSET(5) NUMBITS(3) [],
        L2_SEL OFFSET(3) NUMBITS(2) [],
        L1_SEL OFFSET(2) NUMBITS(1) [],
        L0_SEL OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Bank0Status [
        VOLTAGE_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Bank1Status [
        VOLTAGE_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Bank2Status [
        VOLTAGE_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioLoopbackR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        I2C0_LOOP_I2C1 OFFSET(3) NUMBITS(1) [],
        CAN0_LOOP_CAN1 OFFSET(2) NUMBITS(1) [],
        UA0_LOOP_UA1 OFFSET(1) NUMBITS(1) [],
        SPI0_LOOP_SPI1 OFFSET(0) NUMBITS(1) [],
    ],
    pub MioLoopbackW [
        I2C0_LOOP_I2C1 OFFSET(3) NUMBITS(1) [],
        CAN0_LOOP_CAN1 OFFSET(2) NUMBITS(1) [],
        UA0_LOOP_UA1 OFFSET(1) NUMBITS(1) [],
        SPI0_LOOP_SPI1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioMstTri0 [
        PIN_31_TRI OFFSET(31) NUMBITS(1) [],
        PIN_30_TRI OFFSET(30) NUMBITS(1) [],
        PIN_29_TRI OFFSET(29) NUMBITS(1) [],
        PIN_28_TRI OFFSET(28) NUMBITS(1) [],
        PIN_27_TRI OFFSET(27) NUMBITS(1) [],
        PIN_26_TRI OFFSET(26) NUMBITS(1) [],
        PIN_25_TRI OFFSET(25) NUMBITS(1) [],
        PIN_24_TRI OFFSET(24) NUMBITS(1) [],
        PIN_23_TRI OFFSET(23) NUMBITS(1) [],
        PIN_22_TRI OFFSET(22) NUMBITS(1) [],
        PIN_21_TRI OFFSET(21) NUMBITS(1) [],
        PIN_20_TRI OFFSET(20) NUMBITS(1) [],
        PIN_19_TRI OFFSET(19) NUMBITS(1) [],
        PIN_18_TRI OFFSET(18) NUMBITS(1) [],
        PIN_17_TRI OFFSET(17) NUMBITS(1) [],
        PIN_16_TRI OFFSET(16) NUMBITS(1) [],
        PIN_15_TRI OFFSET(15) NUMBITS(1) [],
        PIN_14_TRI OFFSET(14) NUMBITS(1) [],
        PIN_13_TRI OFFSET(13) NUMBITS(1) [],
        PIN_12_TRI OFFSET(12) NUMBITS(1) [],
        PIN_11_TRI OFFSET(11) NUMBITS(1) [],
        PIN_10_TRI OFFSET(10) NUMBITS(1) [],
        PIN_09_TRI OFFSET(9) NUMBITS(1) [],
        PIN_08_TRI OFFSET(8) NUMBITS(1) [],
        PIN_07_TRI OFFSET(7) NUMBITS(1) [],
        PIN_06_TRI OFFSET(6) NUMBITS(1) [],
        PIN_05_TRI OFFSET(5) NUMBITS(1) [],
        PIN_04_TRI OFFSET(4) NUMBITS(1) [],
        PIN_03_TRI OFFSET(3) NUMBITS(1) [],
        PIN_02_TRI OFFSET(2) NUMBITS(1) [],
        PIN_01_TRI OFFSET(1) NUMBITS(1) [],
        PIN_00_TRI OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioMstTri1 [
        PIN_63_TRI OFFSET(31) NUMBITS(1) [],
        PIN_62_TRI OFFSET(30) NUMBITS(1) [],
        PIN_61_TRI OFFSET(29) NUMBITS(1) [],
        PIN_60_TRI OFFSET(28) NUMBITS(1) [],
        PIN_59_TRI OFFSET(27) NUMBITS(1) [],
        PIN_58_TRI OFFSET(26) NUMBITS(1) [],
        PIN_57_TRI OFFSET(25) NUMBITS(1) [],
        PIN_56_TRI OFFSET(24) NUMBITS(1) [],
        PIN_55_TRI OFFSET(23) NUMBITS(1) [],
        PIN_54_TRI OFFSET(22) NUMBITS(1) [],
        PIN_53_TRI OFFSET(21) NUMBITS(1) [],
        PIN_52_TRI OFFSET(20) NUMBITS(1) [],
        PIN_51_TRI OFFSET(19) NUMBITS(1) [],
        PIN_50_TRI OFFSET(18) NUMBITS(1) [],
        PIN_49_TRI OFFSET(17) NUMBITS(1) [],
        PIN_48_TRI OFFSET(16) NUMBITS(1) [],
        PIN_47_TRI OFFSET(15) NUMBITS(1) [],
        PIN_46_TRI OFFSET(14) NUMBITS(1) [],
        PIN_45_TRI OFFSET(13) NUMBITS(1) [],
        PIN_44_TRI OFFSET(12) NUMBITS(1) [],
        PIN_43_TRI OFFSET(11) NUMBITS(1) [],
        PIN_42_TRI OFFSET(10) NUMBITS(1) [],
        PIN_41_TRI OFFSET(9) NUMBITS(1) [],
        PIN_40_TRI OFFSET(8) NUMBITS(1) [],
        PIN_39_TRI OFFSET(7) NUMBITS(1) [],
        PIN_38_TRI OFFSET(6) NUMBITS(1) [],
        PIN_37_TRI OFFSET(5) NUMBITS(1) [],
        PIN_36_TRI OFFSET(4) NUMBITS(1) [],
        PIN_35_TRI OFFSET(3) NUMBITS(1) [],
        PIN_34_TRI OFFSET(2) NUMBITS(1) [],
        PIN_33_TRI OFFSET(1) NUMBITS(1) [],
        PIN_32_TRI OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MioMstTri2R [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        PIN_77_TRI OFFSET(13) NUMBITS(1) [],
        PIN_76_TRI OFFSET(12) NUMBITS(1) [],
        PIN_75_TRI OFFSET(11) NUMBITS(1) [],
        PIN_74_TRI OFFSET(10) NUMBITS(1) [],
        PIN_73_TRI OFFSET(9) NUMBITS(1) [],
        PIN_72_TRI OFFSET(8) NUMBITS(1) [],
        PIN_71_TRI OFFSET(7) NUMBITS(1) [],
        PIN_70_TRI OFFSET(6) NUMBITS(1) [],
        PIN_69_TRI OFFSET(5) NUMBITS(1) [],
        PIN_68_TRI OFFSET(4) NUMBITS(1) [],
        PIN_67_TRI OFFSET(3) NUMBITS(1) [],
        PIN_66_TRI OFFSET(2) NUMBITS(1) [],
        PIN_65_TRI OFFSET(1) NUMBITS(1) [],
        PIN_64_TRI OFFSET(0) NUMBITS(1) [],
    ],
    pub MioMstTri2W [
        PIN_77_TRI OFFSET(13) NUMBITS(1) [],
        PIN_76_TRI OFFSET(12) NUMBITS(1) [],
        PIN_75_TRI OFFSET(11) NUMBITS(1) [],
        PIN_74_TRI OFFSET(10) NUMBITS(1) [],
        PIN_73_TRI OFFSET(9) NUMBITS(1) [],
        PIN_72_TRI OFFSET(8) NUMBITS(1) [],
        PIN_71_TRI OFFSET(7) NUMBITS(1) [],
        PIN_70_TRI OFFSET(6) NUMBITS(1) [],
        PIN_69_TRI OFFSET(5) NUMBITS(1) [],
        PIN_68_TRI OFFSET(4) NUMBITS(1) [],
        PIN_67_TRI OFFSET(3) NUMBITS(1) [],
        PIN_66_TRI OFFSET(2) NUMBITS(1) [],
        PIN_65_TRI OFFSET(1) NUMBITS(1) [],
        PIN_64_TRI OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub WdtClkSelR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SELECT OFFSET(0) NUMBITS(1) [],
    ],
    pub WdtClkSelW [
        SELECT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CanMioCtrlR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        CAN1_RXIN_REG OFFSET(23) NUMBITS(1) [],
        CAN1_REF_SEL OFFSET(22) NUMBITS(1) [],
        CAN1_MUX OFFSET(15) NUMBITS(7) [],
        RESERVED1 OFFSET(9) NUMBITS(6) [],
        CAN0_RXIN_REG OFFSET(8) NUMBITS(1) [],
        CAN0_REF_SEL OFFSET(7) NUMBITS(1) [],
        CAN0_MUX OFFSET(0) NUMBITS(7) [],
    ],
    pub CanMioCtrlW [
        CAN1_RXIN_REG OFFSET(23) NUMBITS(1) [],
        CAN1_REF_SEL OFFSET(22) NUMBITS(1) [],
        CAN1_MUX OFFSET(15) NUMBITS(7) [],
        RESERVED0 OFFSET(9) NUMBITS(6) [],
        CAN0_RXIN_REG OFFSET(8) NUMBITS(1) [],
        CAN0_REF_SEL OFFSET(7) NUMBITS(1) [],
        CAN0_MUX OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GemClkCtrlR [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        TSU_CLK_LB_SEL OFFSET(22) NUMBITS(1) [],
        TSU_CLK_SEL OFFSET(20) NUMBITS(2) [],
        RESERVED1 OFFSET(19) NUMBITS(1) [],
        GEM3_FIFO_CLK_SEL OFFSET(18) NUMBITS(1) [],
        GEM3_SGMII_MODE OFFSET(17) NUMBITS(1) [],
        GEM3_REF_SRC_SEL OFFSET(16) NUMBITS(1) [],
        GEM3_RX_SRC_SEL OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(14) NUMBITS(1) [],
        GEM2_FIFO_CLK_SEL OFFSET(13) NUMBITS(1) [],
        GEM2_SGMII_MODE OFFSET(12) NUMBITS(1) [],
        GEM2_REF_SRC_SEL OFFSET(11) NUMBITS(1) [],
        GEM2_RX_SRC_SEL OFFSET(10) NUMBITS(1) [],
        RESERVED3 OFFSET(9) NUMBITS(1) [],
        GEM1_FIFO_CLK_SEL OFFSET(8) NUMBITS(1) [],
        GEM1_SGMII_MODE OFFSET(7) NUMBITS(1) [],
        GEM1_REF_SRC_SEL OFFSET(6) NUMBITS(1) [],
        GEM1_RX_SRC_SEL OFFSET(5) NUMBITS(1) [],
        RESERVED4 OFFSET(4) NUMBITS(1) [],
        GEM0_FIFO_CLK_SEL OFFSET(3) NUMBITS(1) [],
        GEM0_SGMII_MODE OFFSET(2) NUMBITS(1) [],
        GEM0_REF_SRC_SEL OFFSET(1) NUMBITS(1) [],
        GEM0_RX_SRC_SEL OFFSET(0) NUMBITS(1) [],
    ],
    pub GemClkCtrlW [
        TSU_CLK_LB_SEL OFFSET(22) NUMBITS(1) [],
        TSU_CLK_SEL OFFSET(20) NUMBITS(2) [],
        GEM3_FIFO_CLK_SEL OFFSET(18) NUMBITS(1) [],
        GEM3_SGMII_MODE OFFSET(17) NUMBITS(1) [],
        GEM3_REF_SRC_SEL OFFSET(16) NUMBITS(1) [],
        GEM3_RX_SRC_SEL OFFSET(15) NUMBITS(1) [],
        GEM2_FIFO_CLK_SEL OFFSET(13) NUMBITS(1) [],
        GEM2_SGMII_MODE OFFSET(12) NUMBITS(1) [],
        GEM2_REF_SRC_SEL OFFSET(11) NUMBITS(1) [],
        GEM2_RX_SRC_SEL OFFSET(10) NUMBITS(1) [],
        GEM1_FIFO_CLK_SEL OFFSET(8) NUMBITS(1) [],
        GEM1_SGMII_MODE OFFSET(7) NUMBITS(1) [],
        GEM1_REF_SRC_SEL OFFSET(6) NUMBITS(1) [],
        GEM1_RX_SRC_SEL OFFSET(5) NUMBITS(1) [],
        GEM0_FIFO_CLK_SEL OFFSET(3) NUMBITS(1) [],
        GEM0_SGMII_MODE OFFSET(2) NUMBITS(1) [],
        GEM0_REF_SRC_SEL OFFSET(1) NUMBITS(1) [],
        GEM0_RX_SRC_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdioClkCtrlR [
        RESERVED0 OFFSET(19) NUMBITS(13) [],
        SDIO1_FBCLK_SEL OFFSET(18) NUMBITS(1) [],
        SDIO1_RX_SRC_SEL OFFSET(17) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(14) [],
        SDIO0_FBCLK_SEL OFFSET(2) NUMBITS(1) [],
        SDIO0_RX_SRC_SEL OFFSET(0) NUMBITS(2) [],
    ],
    pub SdioClkCtrlW [
        SDIO1_FBCLK_SEL OFFSET(18) NUMBITS(1) [],
        SDIO1_RX_SRC_SEL OFFSET(17) NUMBITS(1) [],
        SDIO0_FBCLK_SEL OFFSET(2) NUMBITS(1) [],
        SDIO0_RX_SRC_SEL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CtrlRegSdR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        SD1_EMMC_SEL OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(14) [],
        SD0_EMMC_SEL OFFSET(0) NUMBITS(1) [],
    ],
    pub CtrlRegSdW [
        SD1_EMMC_SEL OFFSET(15) NUMBITS(1) [],
        RESERVED0 OFFSET(1) NUMBITS(14) [],
        SD0_EMMC_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdItapdlyR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        SD1_ITAPCHGWIN OFFSET(25) NUMBITS(1) [],
        SD1_ITAPDLYENA OFFSET(24) NUMBITS(1) [],
        SD1_ITAPDLYSEL OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        SD0_ITAPCHGWIN OFFSET(9) NUMBITS(1) [],
        SD0_ITAPDLYENA OFFSET(8) NUMBITS(1) [],
        SD0_ITAPDLYSEL OFFSET(0) NUMBITS(8) [],
    ],
    pub SdItapdlyW [
        SD1_ITAPCHGWIN OFFSET(25) NUMBITS(1) [],
        SD1_ITAPDLYENA OFFSET(24) NUMBITS(1) [],
        SD1_ITAPDLYSEL OFFSET(16) NUMBITS(8) [],
        SD0_ITAPCHGWIN OFFSET(9) NUMBITS(1) [],
        SD0_ITAPDLYENA OFFSET(8) NUMBITS(1) [],
        SD0_ITAPDLYSEL OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdOtapdlyselR [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        SD1_OTAPDLYENA OFFSET(22) NUMBITS(1) [],
        SD1_OTAPDLYSEL OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(7) NUMBITS(9) [],
        SD0_OTAPDLYENA OFFSET(6) NUMBITS(1) [],
        SD0_OTAPDLYSEL OFFSET(0) NUMBITS(6) [],
    ],
    pub SdOtapdlyselW [
        SD1_OTAPDLYENA OFFSET(22) NUMBITS(1) [],
        SD1_OTAPDLYSEL OFFSET(16) NUMBITS(6) [],
        SD0_OTAPDLYENA OFFSET(6) NUMBITS(1) [],
        SD0_OTAPDLYSEL OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdConfigReg1R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        SD1_BASECLK OFFSET(23) NUMBITS(8) [],
        SD1_TUNIGCOUNT OFFSET(17) NUMBITS(6) [],
        SD1_ASYNCWKPENA OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(15) NUMBITS(1) [],
        SD0_BASECLK OFFSET(7) NUMBITS(8) [],
        SD0_TUNIGCOUNT OFFSET(1) NUMBITS(6) [],
        SD0_ASYNCWKPENA OFFSET(0) NUMBITS(1) [],
    ],
    pub SdConfigReg1W [
        SD1_BASECLK OFFSET(23) NUMBITS(8) [],
        SD1_TUNIGCOUNT OFFSET(17) NUMBITS(6) [],
        SD1_ASYNCWKPENA OFFSET(16) NUMBITS(1) [],
        SD0_BASECLK OFFSET(7) NUMBITS(8) [],
        SD0_TUNIGCOUNT OFFSET(1) NUMBITS(6) [],
        SD0_ASYNCWKPENA OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdConfigReg2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        SD1_SLOTTYPE OFFSET(28) NUMBITS(2) [],
        SD1_ASYCINTR OFFSET(27) NUMBITS(1) [],
        SD1_64BIT OFFSET(26) NUMBITS(1) [],
        SD1_1P8V OFFSET(25) NUMBITS(1) [],
        SD1_3P0V OFFSET(24) NUMBITS(1) [],
        SD1_3P3V OFFSET(23) NUMBITS(1) [],
        SD1_SUSPRES OFFSET(22) NUMBITS(1) [],
        SD1_SDMA OFFSET(21) NUMBITS(1) [],
        SD1_HIGHSPEED OFFSET(20) NUMBITS(1) [],
        SD1_LPD_DMA2 OFFSET(19) NUMBITS(1) [],
        SD1_8BIT OFFSET(18) NUMBITS(1) [],
        SD1_MAXBLK OFFSET(16) NUMBITS(2) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        SD0_SLOTTYPE OFFSET(12) NUMBITS(2) [],
        SD0_ASYCINTR OFFSET(11) NUMBITS(1) [],
        SD0_64BIT OFFSET(10) NUMBITS(1) [],
        SD0_1P8V OFFSET(9) NUMBITS(1) [],
        SD0_3P0V OFFSET(8) NUMBITS(1) [],
        SD0_3P3V OFFSET(7) NUMBITS(1) [],
        SD0_SUSPRES OFFSET(6) NUMBITS(1) [],
        SD0_SDMA OFFSET(5) NUMBITS(1) [],
        SD0_HIGHSPEED OFFSET(4) NUMBITS(1) [],
        SD0_ADMA2 OFFSET(3) NUMBITS(1) [],
        SD0_8BIT OFFSET(2) NUMBITS(1) [],
        SD0_MAXBLK OFFSET(0) NUMBITS(2) [],
    ],
    pub SdConfigReg2W [
        SD1_SLOTTYPE OFFSET(28) NUMBITS(2) [],
        SD1_ASYCINTR OFFSET(27) NUMBITS(1) [],
        SD1_64BIT OFFSET(26) NUMBITS(1) [],
        SD1_1P8V OFFSET(25) NUMBITS(1) [],
        SD1_3P0V OFFSET(24) NUMBITS(1) [],
        SD1_3P3V OFFSET(23) NUMBITS(1) [],
        SD1_SUSPRES OFFSET(22) NUMBITS(1) [],
        SD1_SDMA OFFSET(21) NUMBITS(1) [],
        SD1_HIGHSPEED OFFSET(20) NUMBITS(1) [],
        SD1_LPD_DMA2 OFFSET(19) NUMBITS(1) [],
        SD1_8BIT OFFSET(18) NUMBITS(1) [],
        SD1_MAXBLK OFFSET(16) NUMBITS(2) [],
        SD0_SLOTTYPE OFFSET(12) NUMBITS(2) [],
        SD0_ASYCINTR OFFSET(11) NUMBITS(1) [],
        SD0_64BIT OFFSET(10) NUMBITS(1) [],
        SD0_1P8V OFFSET(9) NUMBITS(1) [],
        SD0_3P0V OFFSET(8) NUMBITS(1) [],
        SD0_3P3V OFFSET(7) NUMBITS(1) [],
        SD0_SUSPRES OFFSET(6) NUMBITS(1) [],
        SD0_SDMA OFFSET(5) NUMBITS(1) [],
        SD0_HIGHSPEED OFFSET(4) NUMBITS(1) [],
        SD0_ADMA2 OFFSET(3) NUMBITS(1) [],
        SD0_8BIT OFFSET(2) NUMBITS(1) [],
        SD0_MAXBLK OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdConfigReg3R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        SD1_TUNINGSDR50 OFFSET(26) NUMBITS(1) [],
        SD1_RETUNETMR OFFSET(22) NUMBITS(4) [],
        SD1_DDRIVER OFFSET(21) NUMBITS(1) [],
        SD1_CDRIVER OFFSET(20) NUMBITS(1) [],
        SD1_ADRIVER OFFSET(19) NUMBITS(1) [],
        SD1_DDR50 OFFSET(18) NUMBITS(1) [],
        SD1_SDR104 OFFSET(17) NUMBITS(1) [],
        SD1_SDR50 OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(5) [],
        SD0_TUNINGSDR50 OFFSET(10) NUMBITS(1) [],
        SD0_RETUNETMR OFFSET(6) NUMBITS(4) [],
        SD0_DDRIVER OFFSET(5) NUMBITS(1) [],
        SD0_CDRIVER OFFSET(4) NUMBITS(1) [],
        SD0_ADRIVER OFFSET(3) NUMBITS(1) [],
        SD0_DDR50 OFFSET(2) NUMBITS(1) [],
        SD0_SDR104 OFFSET(1) NUMBITS(1) [],
        SD0_SDR50 OFFSET(0) NUMBITS(1) [],
    ],
    pub SdConfigReg3W [
        SD1_TUNINGSDR50 OFFSET(26) NUMBITS(1) [],
        SD1_RETUNETMR OFFSET(22) NUMBITS(4) [],
        SD1_DDRIVER OFFSET(21) NUMBITS(1) [],
        SD1_CDRIVER OFFSET(20) NUMBITS(1) [],
        SD1_ADRIVER OFFSET(19) NUMBITS(1) [],
        SD1_DDR50 OFFSET(18) NUMBITS(1) [],
        SD1_SDR104 OFFSET(17) NUMBITS(1) [],
        SD1_SDR50 OFFSET(16) NUMBITS(1) [],
        SD0_TUNINGSDR50 OFFSET(10) NUMBITS(1) [],
        SD0_RETUNETMR OFFSET(6) NUMBITS(4) [],
        SD0_DDRIVER OFFSET(5) NUMBITS(1) [],
        SD0_CDRIVER OFFSET(4) NUMBITS(1) [],
        SD0_ADRIVER OFFSET(3) NUMBITS(1) [],
        SD0_DDR50 OFFSET(2) NUMBITS(1) [],
        SD0_SDR104 OFFSET(1) NUMBITS(1) [],
        SD0_SDR50 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdInitpresetR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SD1_INITPRESET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        SD0_INITPRESET OFFSET(0) NUMBITS(13) [],
    ],
    pub SdInitpresetW [
        SD1_INITPRESET OFFSET(16) NUMBITS(13) [],
        SD0_INITPRESET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdDsppresetR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SD1_DSPPRESET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        SD0_DSPPRESET OFFSET(0) NUMBITS(13) [],
    ],
    pub SdDsppresetW [
        SD1_DSPPRESET OFFSET(16) NUMBITS(13) [],
        SD0_DSPPRESET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdHspdpresetR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SD1_HSPDPRESET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        SD0_HSPDPRESET OFFSET(0) NUMBITS(13) [],
    ],
    pub SdHspdpresetW [
        SD1_HSPDPRESET OFFSET(16) NUMBITS(13) [],
        SD0_HSPDPRESET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdSdr12presetR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SD1_SDR12PRESET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        SD0_SDR12PRESET OFFSET(0) NUMBITS(13) [],
    ],
    pub SdSdr12presetW [
        SD1_SDR12PRESET OFFSET(16) NUMBITS(13) [],
        SD0_SDR12PRESET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdSdr25presetR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SD1_SDR25PRESET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        SD0_SDR25PRESET OFFSET(0) NUMBITS(13) [],
    ],
    pub SdSdr25presetW [
        SD1_SDR25PRESET OFFSET(16) NUMBITS(13) [],
        SD0_SDR25PRESET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdSdr50prsetR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SD1_SDR50PRESET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        SD0_SDR50PRESET OFFSET(0) NUMBITS(13) [],
    ],
    pub SdSdr50prsetW [
        SD1_SDR50PRESET OFFSET(16) NUMBITS(13) [],
        SD0_SDR50PRESET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdSdr104prstR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SD1_SDR104PRESET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        SD0_SDR104PRESET OFFSET(0) NUMBITS(13) [],
    ],
    pub SdSdr104prstW [
        SD1_SDR104PRESET OFFSET(16) NUMBITS(13) [],
        SD0_SDR104PRESET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdDdr50presetR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        SD1_DDR50PRESET OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        SD0_DDR50PRESET OFFSET(0) NUMBITS(13) [],
    ],
    pub SdDdr50presetW [
        SD1_DDR50PRESET OFFSET(16) NUMBITS(13) [],
        SD0_DDR50PRESET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdMaxcur1p8R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        SD1_MAXCUR1P8 OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(8) NUMBITS(8) [],
        SD0_MAXCUR1P8 OFFSET(0) NUMBITS(8) [],
    ],
    pub SdMaxcur1p8W [
        SD1_MAXCUR1P8 OFFSET(16) NUMBITS(8) [],
        SD0_MAXCUR1P8 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdMaxcur3p0R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        SD1_MAXCUR3P0 OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(8) NUMBITS(8) [],
        SD0_MAXCUR3P0 OFFSET(0) NUMBITS(8) [],
    ],
    pub SdMaxcur3p0W [
        SD1_MAXCUR3P0 OFFSET(16) NUMBITS(8) [],
        SD0_MAXCUR3P0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdMaxcur3p3R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        SD1_MAXCUR3P3 OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(8) NUMBITS(8) [],
        SD0_MAXCUR3P3 OFFSET(0) NUMBITS(8) [],
    ],
    pub SdMaxcur3p3W [
        SD1_MAXCUR3P3 OFFSET(16) NUMBITS(8) [],
        SD0_MAXCUR3P3 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdDllCtrlR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        SD1_DLL_RST_DIS OFFSET(19) NUMBITS(1) [],
        SD1_DLL_RST OFFSET(18) NUMBITS(1) [],
        SD1_DLL_TESTMODE OFFSET(17) NUMBITS(1) [],
        SD1_DLL_LOCK OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(12) [],
        SD0_DLL_RST_DIS OFFSET(3) NUMBITS(1) [],
        SD0_DLL_RST OFFSET(2) NUMBITS(1) [],
        SD0_DLL_TESTMODE OFFSET(1) NUMBITS(1) [],
        SD0_DLL_LOCK OFFSET(0) NUMBITS(1) [],
    ],
    pub SdDllCtrlW [
        SD1_DLL_RST_DIS OFFSET(19) NUMBITS(1) [],
        SD1_DLL_RST OFFSET(18) NUMBITS(1) [],
        SD1_DLL_TESTMODE OFFSET(17) NUMBITS(1) [],
        SD0_DLL_RST_DIS OFFSET(3) NUMBITS(1) [],
        SD0_DLL_RST OFFSET(2) NUMBITS(1) [],
        SD0_DLL_TESTMODE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SdCdnCtrlR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        SD1_CDN_CTRL OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(15) [],
        SD0_CDN_CTRL OFFSET(0) NUMBITS(1) [],
    ],
    pub SdCdnCtrlW [
        SD1_CDN_CTRL OFFSET(16) NUMBITS(1) [],
        SD0_CDN_CTRL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GemCtrlR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        GEM3_SGMII_SD OFFSET(6) NUMBITS(2) [],
        GEM2_SGMII_SD OFFSET(4) NUMBITS(2) [],
        GEM1_SGMII_SD OFFSET(2) NUMBITS(2) [],
        GEM0_SGMII_SD OFFSET(0) NUMBITS(2) [],
    ],
    pub GemCtrlW [
        GEM3_SGMII_SD OFFSET(6) NUMBITS(2) [],
        GEM2_SGMII_SD OFFSET(4) NUMBITS(2) [],
        GEM1_SGMII_SD OFFSET(2) NUMBITS(2) [],
        GEM0_SGMII_SD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IouTtcApbClkR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        TTC3_SEL OFFSET(6) NUMBITS(2) [],
        TTC2_SEL OFFSET(4) NUMBITS(2) [],
        TTC1_SEL OFFSET(2) NUMBITS(2) [],
        TTC0_SEL OFFSET(0) NUMBITS(2) [],
    ],
    pub IouTtcApbClkW [
        TTC3_SEL OFFSET(6) NUMBITS(2) [],
        TTC2_SEL OFFSET(4) NUMBITS(2) [],
        TTC1_SEL OFFSET(2) NUMBITS(2) [],
        TTC0_SEL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IouTapdlyBypassR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        LQSPI_RX OFFSET(2) NUMBITS(1) [],
        NAND_DQS_OUT OFFSET(1) NUMBITS(1) [],
        NAND_DQS_IN OFFSET(0) NUMBITS(1) [],
    ],
    pub IouTapdlyBypassW [
        LQSPI_RX OFFSET(2) NUMBITS(1) [],
        NAND_DQS_OUT OFFSET(1) NUMBITS(1) [],
        NAND_DQS_IN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IouCoherentCtrl [
        QSPI_AXI_COH OFFSET(28) NUMBITS(4) [],
        NAND_AXI_COH OFFSET(24) NUMBITS(4) [],
        SD1_AXI_COH OFFSET(20) NUMBITS(4) [],
        SD0_AXI_COH OFFSET(16) NUMBITS(4) [],
        GEM3_AXI_COH OFFSET(12) NUMBITS(4) [],
        GEM2_AXI_COH OFFSET(8) NUMBITS(4) [],
        GEM1_AXI_COH OFFSET(4) NUMBITS(4) [],
        GEM0_AXI_COH OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VideoPssClkSelR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PSS_ALT_CLK OFFSET(1) NUMBITS(1) [],
        VIDEO_CLK OFFSET(0) NUMBITS(1) [],
    ],
    pub VideoPssClkSelW [
        PSS_ALT_CLK OFFSET(1) NUMBITS(1) [],
        VIDEO_CLK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IouInterconnectRouteR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        NAND OFFSET(7) NUMBITS(1) [],
        QSPI OFFSET(6) NUMBITS(1) [],
        SD1 OFFSET(5) NUMBITS(1) [],
        SD0 OFFSET(4) NUMBITS(1) [],
        GEM3 OFFSET(3) NUMBITS(1) [],
        GEM2 OFFSET(2) NUMBITS(1) [],
        GEM1 OFFSET(1) NUMBITS(1) [],
        GEM0 OFFSET(0) NUMBITS(1) [],
    ],
    pub IouInterconnectRouteW [
        NAND OFFSET(7) NUMBITS(1) [],
        QSPI OFFSET(6) NUMBITS(1) [],
        SD1 OFFSET(5) NUMBITS(1) [],
        SD0 OFFSET(4) NUMBITS(1) [],
        GEM3 OFFSET(3) NUMBITS(1) [],
        GEM2 OFFSET(2) NUMBITS(1) [],
        GEM1 OFFSET(1) NUMBITS(1) [],
        GEM0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Ctrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Isr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Imr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Ier [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Idr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Itr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];

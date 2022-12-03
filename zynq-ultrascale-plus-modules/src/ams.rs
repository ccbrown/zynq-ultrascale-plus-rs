// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// PS and PL SysMon Units Control and Status, PS and PL SysMon Units Control and Status
pub static mut AMS_CTRL: *mut Registers = 0xffa50000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Register Access Error Signal Enables.
    pub misc_ctrl: Aliased<u32, MiscCtrlR::Register, MiscCtrlW::Register>,
    _padding4: [u8; 12],
    /// Alarm Interrupt Status and Clear, Reg 0. PS and PL.
    pub isr_0: ReadWrite<u32, Isr0::Register>,
    /// Alarm and Access Error Interrupt Status and Clear, Reg 1.
    pub isr_1: Aliased<u32, Isr1R::Register, Isr1W::Register>,
    /// Interrupt Mask, Reg 0.
    pub imr_0: ReadOnly<u32, Imr0::Register>,
    /// Interrupt Mask, Reg 1.
    pub imr_1: ReadOnly<u32, Imr1::Register>,
    /// Interrupt Enable, Reg 0.
    pub ier_0: WriteOnly<u32, Ier0::Register>,
    /// Interrupt Enable, Reg 1.
    pub ier_1: Aliased<u32, Ier1R::Register, Ier1W::Register>,
    /// Interrupt Disable, Reg 0.
    pub idr_0: WriteOnly<u32, Idr0::Register>,
    /// Interrupt Disable, Reg 1.
    pub idr_1: Aliased<u32, Idr1R::Register, Idr1W::Register>,
    _padding48: [u8; 16],
    /// PS SysMon Unit Control and Status
    pub ps_ctrl_status: Aliased<u32, PsCtrlStatusR::Register, PsCtrlStatusW::Register>,
    /// PL SysMon register access control status.
    pub pl_ctrl_status: ReadOnly<u32, PlCtrlStatus::Register>,
    _padding72: [u8; 8],
    /// ADC SysMon status.
    pub mon_status: ReadOnly<u32, MonStatus::Register>,
    _padding84: [u8; 12],
    /// System PLLs voltage measurement, VCC_PSPLL.
    pub vcc_pspll: ReadOnly<u32, VccPspll::Register>,
    _padding100: [u8; 8],
    /// Battery voltage measurement, VCC_PSBATT.
    pub vcc_psbatt: ReadOnly<u32, VccPsbatt::Register>,
    _padding112: [u8; 8],
    /// PL Internal voltage measurement, VCCINT.
    pub vccint: ReadOnly<u32, Vccint::Register>,
    /// Block RAM voltage measurement, VCCBRAM.
    pub vccbram: ReadOnly<u32, Vccbram::Register>,
    /// PL Aux voltage measurement, VCCAUX.
    pub vccaux: ReadOnly<u32, Vccaux::Register>,
    /// Voltage measurement for six DDR I/O PLLs, VCC_PSDDR_PLL.
    pub vcc_psddr_pll: ReadOnly<u32, VccPsddrPll::Register>,
    _padding136: [u8; 20],
    /// VCC_PSINTFP_DDR voltage measurement.
    pub vcc_psintfp_ddr: ReadOnly<u32, VccPsintfpDdr::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub MiscCtrlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        SLVERR_ENABLE_DRP OFFSET(1) NUMBITS(1) [],
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub MiscCtrlW [
        SLVERR_ENABLE_DRP OFFSET(1) NUMBITS(1) [],
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Isr0 [
        PL_ALM_15 OFFSET(31) NUMBITS(1) [],
        PL_ALM_14 OFFSET(30) NUMBITS(1) [],
        PL_ALM_13 OFFSET(29) NUMBITS(1) [],
        PL_ALM_12 OFFSET(28) NUMBITS(1) [],
        PL_ALM_11 OFFSET(27) NUMBITS(1) [],
        PL_ALM_10 OFFSET(26) NUMBITS(1) [],
        PL_ALM_9 OFFSET(25) NUMBITS(1) [],
        PL_ALM_8 OFFSET(24) NUMBITS(1) [],
        PL_ALM_7 OFFSET(23) NUMBITS(1) [],
        PL_ALM_6 OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        PL_ALM_3 OFFSET(19) NUMBITS(1) [],
        PL_ALM_2 OFFSET(18) NUMBITS(1) [],
        PL_ALM_1 OFFSET(17) NUMBITS(1) [],
        PL_ALM_0 OFFSET(16) NUMBITS(1) [],
        PS_ALM_15 OFFSET(15) NUMBITS(1) [],
        PS_ALM_14 OFFSET(14) NUMBITS(1) [],
        PS_ALM_13 OFFSET(13) NUMBITS(1) [],
        PS_ALM_12 OFFSET(12) NUMBITS(1) [],
        PS_ALM_11 OFFSET(11) NUMBITS(1) [],
        PS_ALM_10 OFFSET(10) NUMBITS(1) [],
        PS_ALM_9 OFFSET(9) NUMBITS(1) [],
        PS_ALM_8 OFFSET(8) NUMBITS(1) [],
        PS_ALM_7 OFFSET(7) NUMBITS(1) [],
        PS_ALM_6 OFFSET(6) NUMBITS(1) [],
        PS_ALM_5 OFFSET(5) NUMBITS(1) [],
        PS_ALM_4 OFFSET(4) NUMBITS(1) [],
        PS_ALM_3 OFFSET(3) NUMBITS(1) [],
        PS_ALM_2 OFFSET(2) NUMBITS(1) [],
        PS_ALM_1 OFFSET(1) NUMBITS(1) [],
        PS_ALM_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Isr1R [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        ADDR_DECODE_ERR_PL_SYSMON OFFSET(30) NUMBITS(1) [],
        ADDR_DECODE_ERR_PS_SYSMON OFFSET(29) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(24) [],
        EOS OFFSET(4) NUMBITS(1) [],
        EOC OFFSET(3) NUMBITS(1) [],
        PL_OT OFFSET(2) NUMBITS(1) [],
        PS_LPD_OT OFFSET(1) NUMBITS(1) [],
        PS_FPD_OT OFFSET(0) NUMBITS(1) [],
    ],
    pub Isr1W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        ADDR_DECODE_ERR_PL_SYSMON OFFSET(30) NUMBITS(1) [],
        ADDR_DECODE_ERR_PS_SYSMON OFFSET(29) NUMBITS(1) [],
        EOS OFFSET(4) NUMBITS(1) [],
        EOC OFFSET(3) NUMBITS(1) [],
        PL_OT OFFSET(2) NUMBITS(1) [],
        PS_LPD_OT OFFSET(1) NUMBITS(1) [],
        PS_FPD_OT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imr0 [
        PL_ALM_15 OFFSET(31) NUMBITS(1) [],
        PL_ALM_14 OFFSET(30) NUMBITS(1) [],
        PL_ALM_13 OFFSET(29) NUMBITS(1) [],
        PL_ALM_12 OFFSET(28) NUMBITS(1) [],
        PL_ALM_11 OFFSET(27) NUMBITS(1) [],
        PL_ALM_10 OFFSET(26) NUMBITS(1) [],
        PL_ALM_9 OFFSET(25) NUMBITS(1) [],
        PL_ALM_8 OFFSET(24) NUMBITS(1) [],
        PL_ALM_7 OFFSET(23) NUMBITS(1) [],
        PL_ALM_6 OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        PL_ALM_3 OFFSET(19) NUMBITS(1) [],
        PL_ALM_2 OFFSET(18) NUMBITS(1) [],
        PL_ALM_1 OFFSET(17) NUMBITS(1) [],
        PL_ALM_0 OFFSET(16) NUMBITS(1) [],
        PS_ALM_15 OFFSET(15) NUMBITS(1) [],
        PS_ALM_14 OFFSET(14) NUMBITS(1) [],
        PS_ALM_13 OFFSET(13) NUMBITS(1) [],
        PS_ALM_12 OFFSET(12) NUMBITS(1) [],
        PS_ALM_11 OFFSET(11) NUMBITS(1) [],
        PS_ALM_10 OFFSET(10) NUMBITS(1) [],
        PS_ALM_9 OFFSET(9) NUMBITS(1) [],
        PS_ALM_8 OFFSET(8) NUMBITS(1) [],
        PS_ALM_7 OFFSET(7) NUMBITS(1) [],
        PS_ALM_6 OFFSET(6) NUMBITS(1) [],
        PS_ALM_5 OFFSET(5) NUMBITS(1) [],
        PS_ALM_4 OFFSET(4) NUMBITS(1) [],
        PS_ALM_3 OFFSET(3) NUMBITS(1) [],
        PS_ALM_2 OFFSET(2) NUMBITS(1) [],
        PS_ALM_1 OFFSET(1) NUMBITS(1) [],
        PS_ALM_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imr1 [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        ADDR_DECODE_ERR_PL_SYSMON OFFSET(30) NUMBITS(1) [],
        ADDR_DECODE_ERR_PS_SYSMON OFFSET(29) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(24) [],
        EOS OFFSET(4) NUMBITS(1) [],
        EOC OFFSET(3) NUMBITS(1) [],
        PL_OT OFFSET(2) NUMBITS(1) [],
        PS_LPD_OT OFFSET(1) NUMBITS(1) [],
        PS_FPD_OT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ier0 [
        PL_ALM_15 OFFSET(31) NUMBITS(1) [],
        PL_ALM_14 OFFSET(30) NUMBITS(1) [],
        PL_ALM_13 OFFSET(29) NUMBITS(1) [],
        PL_ALM_12 OFFSET(28) NUMBITS(1) [],
        PL_ALM_11 OFFSET(27) NUMBITS(1) [],
        PL_ALM_10 OFFSET(26) NUMBITS(1) [],
        PL_ALM_9 OFFSET(25) NUMBITS(1) [],
        PL_ALM_8 OFFSET(24) NUMBITS(1) [],
        PL_ALM_7 OFFSET(23) NUMBITS(1) [],
        PL_ALM_6 OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        PL_ALM_3 OFFSET(19) NUMBITS(1) [],
        PL_ALM_2 OFFSET(18) NUMBITS(1) [],
        PL_ALM_1 OFFSET(17) NUMBITS(1) [],
        PL_ALM_0 OFFSET(16) NUMBITS(1) [],
        PS_ALM_15 OFFSET(15) NUMBITS(1) [],
        PS_ALM_14 OFFSET(14) NUMBITS(1) [],
        PS_ALM_13 OFFSET(13) NUMBITS(1) [],
        PS_ALM_12 OFFSET(12) NUMBITS(1) [],
        PS_ALM_11 OFFSET(11) NUMBITS(1) [],
        PS_ALM_10 OFFSET(10) NUMBITS(1) [],
        PS_ALM_9 OFFSET(9) NUMBITS(1) [],
        PS_ALM_8 OFFSET(8) NUMBITS(1) [],
        PS_ALM_7 OFFSET(7) NUMBITS(1) [],
        PS_ALM_6 OFFSET(6) NUMBITS(1) [],
        PS_ALM_5 OFFSET(5) NUMBITS(1) [],
        PS_ALM_4 OFFSET(4) NUMBITS(1) [],
        PS_ALM_3 OFFSET(3) NUMBITS(1) [],
        PS_ALM_2 OFFSET(2) NUMBITS(1) [],
        PS_ALM_1 OFFSET(1) NUMBITS(1) [],
        PS_ALM_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ier1R [
        RESERVED0 OFFSET(5) NUMBITS(24) [],
    ],
    pub Ier1W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        ADDR_DECODE_ERR_PL_SYSMON OFFSET(30) NUMBITS(1) [],
        ADDR_DECODE_ERR_PS_SYSMON OFFSET(29) NUMBITS(1) [],
        EOS OFFSET(4) NUMBITS(1) [],
        EOC OFFSET(3) NUMBITS(1) [],
        PL_OT OFFSET(2) NUMBITS(1) [],
        PS_LPD_OT OFFSET(1) NUMBITS(1) [],
        PS_FPD_OT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr0 [
        PL_ALM_15 OFFSET(31) NUMBITS(1) [],
        PL_ALM_14 OFFSET(30) NUMBITS(1) [],
        PL_ALM_13 OFFSET(29) NUMBITS(1) [],
        PL_ALM_12 OFFSET(28) NUMBITS(1) [],
        PL_ALM_11 OFFSET(27) NUMBITS(1) [],
        PL_ALM_10 OFFSET(26) NUMBITS(1) [],
        PL_ALM_9 OFFSET(25) NUMBITS(1) [],
        PL_ALM_8 OFFSET(24) NUMBITS(1) [],
        PL_ALM_7 OFFSET(23) NUMBITS(1) [],
        PL_ALM_6 OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        PL_ALM_3 OFFSET(19) NUMBITS(1) [],
        PL_ALM_2 OFFSET(18) NUMBITS(1) [],
        PL_ALM_1 OFFSET(17) NUMBITS(1) [],
        PL_ALM_0 OFFSET(16) NUMBITS(1) [],
        PS_ALM_15 OFFSET(15) NUMBITS(1) [],
        PS_ALM_14 OFFSET(14) NUMBITS(1) [],
        PS_ALM_13 OFFSET(13) NUMBITS(1) [],
        PS_ALM_12 OFFSET(12) NUMBITS(1) [],
        PS_ALM_11 OFFSET(11) NUMBITS(1) [],
        PS_ALM_10 OFFSET(10) NUMBITS(1) [],
        PS_ALM_9 OFFSET(9) NUMBITS(1) [],
        PS_ALM_8 OFFSET(8) NUMBITS(1) [],
        PS_ALM_7 OFFSET(7) NUMBITS(1) [],
        PS_ALM_6 OFFSET(6) NUMBITS(1) [],
        PS_ALM_5 OFFSET(5) NUMBITS(1) [],
        PS_ALM_4 OFFSET(4) NUMBITS(1) [],
        PS_ALM_3 OFFSET(3) NUMBITS(1) [],
        PS_ALM_2 OFFSET(2) NUMBITS(1) [],
        PS_ALM_1 OFFSET(1) NUMBITS(1) [],
        PS_ALM_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr1R [
        RESERVED0 OFFSET(5) NUMBITS(24) [],
    ],
    pub Idr1W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        ADDR_DECODE_ERR_PL_SYSMON OFFSET(30) NUMBITS(1) [],
        ADDR_DECODE_ERR_PS_SYSMON OFFSET(29) NUMBITS(1) [],
        EOS OFFSET(4) NUMBITS(1) [],
        EOC OFFSET(3) NUMBITS(1) [],
        PL_OT OFFSET(2) NUMBITS(1) [],
        PS_LPD_OT OFFSET(1) NUMBITS(1) [],
        PS_FPD_OT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PsCtrlStatusR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        STARTUP_STATE OFFSET(24) NUMBITS(4) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        STARTUP_DONE OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(12) [],
        AUTO_CONVST OFFSET(3) NUMBITS(1) [],
        RESET_USER OFFSET(1) NUMBITS(1) [],
        STARTUP_TRIGGER OFFSET(0) NUMBITS(1) [],
    ],
    pub PsCtrlStatusW [
        AUTO_CONVST OFFSET(3) NUMBITS(1) [],
        CONVST OFFSET(2) NUMBITS(1) [],
        RESET_USER OFFSET(1) NUMBITS(1) [],
        STARTUP_TRIGGER OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PlCtrlStatus [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ACCESSIBLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MonStatus [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        JTAG_LOCKED OFFSET(23) NUMBITS(1) [],
        BUSY OFFSET(22) NUMBITS(1) [],
        CHANNEL OFFSET(16) NUMBITS(6) [],
        MON_DATA OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VccPspll [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VccPsbatt [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vccint [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vccbram [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vccaux [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VccPsddrPll [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VccPsintfpDdr [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];

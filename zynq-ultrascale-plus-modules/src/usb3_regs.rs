// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// Universal Serial Bus 3.0, USB Core 0 Controller
pub static mut USB3_0: *mut Registers = 0xff9d0000 as *mut Registers;
/// Universal Serial Bus 3.0, USB Core 1 Controller
pub static mut USB3_1: *mut Registers = 0xff9e0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Indicates current power state of the core
        (0x00000000 => pub cur_pwr_st: ReadOnly<u32, CurPwrSt::Register>),
        /// Indicates if USB3 Always ON block has connection to at least 1 host or device during Hibernation state
        (0x00000004 => pub connect_st: ReadOnly<u32, ConnectSt::Register>),
        (0x00000008 => _padding8),
        /// Disables internal bus filters that are enabled by DWC_USB3_EN_BUS_FILTERS
        (0x00000030 => pub bus_filter: Aliased<u32, BusFilterR::Register, BusFilterW::Register>),
        /// Enable signal for pme_generation
        (0x00000034 => pub pme_en: Aliased<u32, PmeEnR::Register, PmeEnW::Register>),
        /// Device characteristics
        (0x00000038 => pub port: Aliased<u32, PortR::Register, PortW::Register>),
        /// Power state transition request byPMU to USB
        (0x0000003c => pub pmu_usb: Aliased<u32, PmuUsbR::Register, PmuUsbW::Register>),
        /// High Speed Jitter Adjustment
        (0x00000040 => pub jitter_adjust: Aliased<u32, JitterAdjustR::Register, JitterAdjustW::Register>),
        (0x00000044 => _padding68),
        /// USB3 PHY power config
        (0x00000048 => pub pwr_config_usb3: Aliased<u32, PwrConfigUsb3R::Register, PwrConfigUsb3W::Register>),
        /// USB2 PHY power config
        (0x0000004c => pub pwr_config_usb2: Aliased<u32, PwrConfigUsb2R::Register, PwrConfigUsb2W::Register>),
        /// Current BELT value
        (0x00000050 => pub host: ReadOnly<u32, Host::Register>),
        (0x00000054 => _padding84),
        /// Coherency Mode
        (0x0000005c => pub coherency: Aliased<u32, CoherencyR::Register, CoherencyW::Register>),
        /// reg_ctrl
        (0x00000060 => pub reg_ctrl: Aliased<u32, RegCtrlR::Register, RegCtrlW::Register>),
        /// ir_status
        (0x00000064 => pub ir_status: Aliased<u32, IrStatusR::Register, IrStatusW::Register>),
        /// ir_mask
        (0x00000068 => pub ir_mask: ReadOnly<u32, IrMask::Register>),
        /// ir_enable
        (0x0000006c => pub ir_enable: Aliased<u32, IrEnableR::Register, IrEnableW::Register>),
        /// ir_disable
        (0x00000070 => pub ir_disable: Aliased<u32, IrDisableR::Register, IrDisableW::Register>),
        (0x00000074 => _padding116),
        /// usb3
        (0x00000078 => pub usb3: ReadWrite<u32>),
        /// fpd_pipe_clk
        (0x0000007c => pub fpd_pipe_clk: Aliased<u32, FpdPipeClkR::Register, FpdPipeClkW::Register>),
        /// fpd_power_prsnt
        (0x00000080 => pub fpd_power_prsnt: Aliased<u32, FpdPowerPrsntR::Register, FpdPowerPrsntW::Register>),
        (0x00000084 => @END),
    }
}
register_bitfields! [
    u32,
    pub CurPwrSt [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        U2PMU OFFSET(2) NUMBITS(2) [],
        U3PMU OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub ConnectSt [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        U2PMU OFFSET(1) NUMBITS(1) [],
        U3PMU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub BusFilterR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        BYPASS OFFSET(0) NUMBITS(4) [],
    ],
    pub BusFilterW [
        BYPASS OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub PmeEnR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        BIT OFFSET(0) NUMBITS(1) [],
    ],
    pub PmeEnW [
        BIT OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub PortR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        PWR_CTRL_PRSNT OFFSET(2) NUMBITS(1) [],
        PERM_ATTACH OFFSET(0) NUMBITS(2) [],
    ],
    pub PortW [
        PWR_CTRL_PRSNT OFFSET(2) NUMBITS(1) [],
        PERM_ATTACH OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub PmuUsbR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PWR_ST_REQ OFFSET(0) NUMBITS(2) [],
    ],
    pub PmuUsbW [
        PWR_ST_REQ OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub JitterAdjustR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        FLADJ OFFSET(0) NUMBITS(6) [],
    ],
    pub JitterAdjustW [
        FLADJ OFFSET(0) NUMBITS(6) [],
    ]
];
register_bitfields! [
    u32,
    pub PwrConfigUsb3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        STRAP OFFSET(0) NUMBITS(30) [],
    ],
    pub PwrConfigUsb3W [
        STRAP OFFSET(0) NUMBITS(30) [],
    ]
];
register_bitfields! [
    u32,
    pub PwrConfigUsb2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        STRAP OFFSET(0) NUMBITS(30) [],
    ],
    pub PwrConfigUsb2W [
        STRAP OFFSET(0) NUMBITS(30) [],
    ]
];
register_bitfields! [
    u32,
    pub Host [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        CUR_BELT OFFSET(0) NUMBITS(12) [],
    ]
];
register_bitfields! [
    u32,
    pub CoherencyR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        USB OFFSET(0) NUMBITS(1) [],
    ],
    pub CoherencyW [
        USB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub RegCtrlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub RegCtrlW [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrStatusR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        HOST_SYS_ERR OFFSET(1) NUMBITS(1) [],
        ADDR_DEC_ERR OFFSET(0) NUMBITS(1) [],
    ],
    pub IrStatusW [
        HOST_SYS_ERR OFFSET(1) NUMBITS(1) [],
        ADDR_DEC_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrMask [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        HOST_SYS_ERR OFFSET(1) NUMBITS(1) [],
        ADDR_DEC_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrEnableR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub IrEnableW [
        HOST_SYS_ERR OFFSET(1) NUMBITS(1) [],
        ADDR_DEC_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrDisableR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub IrDisableW [
        HOST_SYS_ERR OFFSET(1) NUMBITS(1) [],
        ADDR_DEC_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub FpdPipeClkR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        OPTION OFFSET(0) NUMBITS(1) [],
    ],
    pub FpdPipeClkW [
        OPTION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub FpdPowerPrsntR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        OPTION OFFSET(0) NUMBITS(1) [],
    ],
    pub FpdPowerPrsntW [
        OPTION OFFSET(0) NUMBITS(1) [],
    ]
];

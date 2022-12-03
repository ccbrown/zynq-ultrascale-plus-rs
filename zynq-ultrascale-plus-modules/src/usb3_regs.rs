// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// Universal Serial Bus 3.0, USB Core 0 Controller
pub static mut USB3_0: *mut Registers = 0xff9d0000 as *mut Registers;
/// Universal Serial Bus 3.0, USB Core 1 Controller
pub static mut USB3_1: *mut Registers = 0xff9e0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Indicates current power state of the core
    pub cur_pwr_st: ReadOnly<u32, CurPwrSt::Register>,
    /// Indicates if USB3 Always ON block has connection to at least 1 host or device during Hibernation state
    pub connect_st: ReadOnly<u32, ConnectSt::Register>,
    _padding8: [u8; 40],
    /// Disables internal bus filters that are enabled by DWC_USB3_EN_BUS_FILTERS
    pub bus_filter: Aliased<u32, BusFilterR::Register, BusFilterW::Register>,
    /// Enable signal for pme_generation
    pub pme_en: Aliased<u32, PmeEnR::Register, PmeEnW::Register>,
    /// Device characteristics
    pub port: Aliased<u32, PortR::Register, PortW::Register>,
    /// Power state transition request byPMU to USB
    pub pmu_usb: Aliased<u32, PmuUsbR::Register, PmuUsbW::Register>,
    /// High Speed Jitter Adjustment
    pub jitter_adjust: Aliased<u32, JitterAdjustR::Register, JitterAdjustW::Register>,
    _padding68: [u8; 4],
    /// USB3 PHY power config
    pub pwr_config_usb3: Aliased<u32, PwrConfigUsb3R::Register, PwrConfigUsb3W::Register>,
    /// USB2 PHY power config
    pub pwr_config_usb2: Aliased<u32, PwrConfigUsb2R::Register, PwrConfigUsb2W::Register>,
    /// Current BELT value
    pub host: ReadOnly<u32, Host::Register>,
    _padding84: [u8; 8],
    /// Coherency Mode
    pub coherency: Aliased<u32, CoherencyR::Register, CoherencyW::Register>,
    /// reg_ctrl
    pub reg_ctrl: Aliased<u32, RegCtrlR::Register, RegCtrlW::Register>,
    /// ir_status
    pub ir_status: Aliased<u32, IrStatusR::Register, IrStatusW::Register>,
    /// ir_mask
    pub ir_mask: ReadOnly<u32, IrMask::Register>,
    /// ir_enable
    pub ir_enable: Aliased<u32, IrEnableR::Register, IrEnableW::Register>,
    /// ir_disable
    pub ir_disable: Aliased<u32, IrDisableR::Register, IrDisableW::Register>,
    _padding116: [u8; 4],
    /// usb3
    pub usb3: ReadWrite<u32>,
    /// fpd_pipe_clk
    pub fpd_pipe_clk: Aliased<u32, FpdPipeClkR::Register, FpdPipeClkW::Register>,
    /// fpd_power_prsnt
    pub fpd_power_prsnt: Aliased<u32, FpdPowerPrsntR::Register, FpdPowerPrsntW::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub CurPwrSt [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        U2PMU OFFSET(2) NUMBITS(2) [],
        U3PMU OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ConnectSt [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        U2PMU OFFSET(1) NUMBITS(1) [],
        U3PMU OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BusFilterR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        BYPASS OFFSET(0) NUMBITS(4) [],
    ],
    pub BusFilterW [
        BYPASS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PmeEnR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        BIT OFFSET(0) NUMBITS(1) [],
    ],
    pub PmeEnW [
        BIT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub PmuUsbR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PWR_ST_REQ OFFSET(0) NUMBITS(2) [],
    ],
    pub PmuUsbW [
        PWR_ST_REQ OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub JitterAdjustR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        FLADJ OFFSET(0) NUMBITS(6) [],
    ],
    pub JitterAdjustW [
        FLADJ OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PwrConfigUsb3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        STRAP OFFSET(0) NUMBITS(30) [],
    ],
    pub PwrConfigUsb3W [
        STRAP OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PwrConfigUsb2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        STRAP OFFSET(0) NUMBITS(30) [],
    ],
    pub PwrConfigUsb2W [
        STRAP OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Host [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        CUR_BELT OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoherencyR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        USB OFFSET(0) NUMBITS(1) [],
    ],
    pub CoherencyW [
        USB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RegCtrlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub RegCtrlW [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub IrMask [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        HOST_SYS_ERR OFFSET(1) NUMBITS(1) [],
        ADDR_DEC_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IrEnableR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub IrEnableW [
        HOST_SYS_ERR OFFSET(1) NUMBITS(1) [],
        ADDR_DEC_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IrDisableR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub IrDisableW [
        HOST_SYS_ERR OFFSET(1) NUMBITS(1) [],
        ADDR_DEC_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub FpdPipeClkR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        OPTION OFFSET(0) NUMBITS(1) [],
    ],
    pub FpdPipeClkW [
        OPTION OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub FpdPowerPrsntR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        OPTION OFFSET(0) NUMBITS(1) [],
    ],
    pub FpdPowerPrsntW [
        OPTION OFFSET(0) NUMBITS(1) [],
    ]
];

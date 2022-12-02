// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly};
/// Serial Input Output Unit, SerDes Control and Debug
pub static mut SIOU: *mut Registers = 0xfd3d0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Miscellaneous control functions for SIOU
        (0x00000000 => pub reg_ctrl: Aliased<u32, RegCtrlR::Register, RegCtrlW::Register>),
        /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
        (0x00000004 => pub ir_status: Aliased<u32, IrStatusR::Register, IrStatusW::Register>),
        /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
        (0x00000008 => pub ir_mask: ReadOnly<u32, IrMask::Register>),
        /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
        (0x0000000c => pub ir_enable: Aliased<u32, IrEnableR::Register, IrEnableW::Register>),
        /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
        (0x00000010 => pub ir_disable: Aliased<u32, IrDisableR::Register, IrDisableW::Register>),
        (0x00000014 => _padding20),
        /// Misc Contorls for SATA.This register may only be modified during bootup (while SATA block is disabled)
        (0x00000100 => pub sata_misc_ctrl: Aliased<u32, SataMiscCtrlR::Register, SataMiscCtrlW::Register>),
        (0x00000104 => _padding260),
        /// crx_ctrl
        (0x00000410 => pub crx_ctrl: Aliased<u32, CrxCtrlR::Register, CrxCtrlW::Register>),
        (0x00000414 => _padding1044),
        /// dp_stc_ref_clk control register
        (0x00000430 => pub dp_stc_clkctrl: Aliased<u32, DpStcClkctrlR::Register, DpStcClkctrlW::Register>),
        (0x00000434 => @END),
    }
}
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
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ],
    pub IrStatusW [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrMask [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrEnableR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub IrEnableW [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IrDisableR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub IrDisableW [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub SataMiscCtrlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        SATA_PM_CLK_SEL OFFSET(0) NUMBITS(2) [],
    ],
    pub SataMiscCtrlW [
        SATA_PM_CLK_SEL OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub CrxCtrlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        REFCLK_SEL OFFSET(0) NUMBITS(2) [],
    ],
    pub CrxCtrlW [
        REFCLK_SEL OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub DpStcClkctrlR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        REFSEL OFFSET(10) NUMBITS(1) [],
        LANESEL OFFSET(8) NUMBITS(2) [],
        UPTOG OFFSET(7) NUMBITS(1) [],
        DIVISOR OFFSET(1) NUMBITS(6) [],
        SOFT_RST OFFSET(0) NUMBITS(1) [],
    ],
    pub DpStcClkctrlW [
        REFSEL OFFSET(10) NUMBITS(1) [],
        LANESEL OFFSET(8) NUMBITS(2) [],
        UPTOG OFFSET(7) NUMBITS(1) [],
        DIVISOR OFFSET(1) NUMBITS(6) [],
        SOFT_RST OFFSET(0) NUMBITS(1) [],
    ]
];

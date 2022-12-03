// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Application Processing Unit, APU Configuration
pub static mut APU: *mut Registers = 0xfd5c0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Control register
    pub err_ctrl: ReadWrite<u32, ErrCtrl::Register>,
    _padding4: [u8; 12],
    /// Interrupt Status Register
    pub isr: ReadWrite<u32, Isr::Register>,
    /// Interrupt Mask Register
    pub imr: ReadOnly<u32, Imr::Register>,
    /// Interrupt Enable Register
    pub ien: WriteOnly<u32, Ien::Register>,
    /// Interrupt Disable Register
    pub ids: WriteOnly<u32, Ids::Register>,
    /// CPU Core Configuration
    pub config_0: ReadWrite<u32, Config0::Register>,
    /// L2 Configuration
    pub config_1: ReadWrite<u32, Config1::Register>,
    _padding40: [u8; 24],
    /// Reset Vector Base Address
    pub rvbaraddr0l: ReadWrite<u32, Rvbaraddr0l::Register>,
    /// Reset Vector Base Address
    pub rvbaraddr0h: ReadWrite<u32, Rvbaraddr0h::Register>,
    /// Reset Vector Base Address
    pub rvbaraddr1l: ReadWrite<u32, Rvbaraddr1l::Register>,
    /// Reset Vector Base Address
    pub rvbaraddr1h: ReadWrite<u32, Rvbaraddr1h::Register>,
    /// Reset Vector Base Address
    pub rvbaraddr2l: ReadWrite<u32, Rvbaraddr2l::Register>,
    /// Reset Vector Base Address
    pub rvbaraddr2h: ReadWrite<u32, Rvbaraddr2h::Register>,
    /// Reset Vector Base Address
    pub rvbaraddr3l: ReadWrite<u32, Rvbaraddr3l::Register>,
    /// Reset Vector Base Address
    pub rvbaraddr3h: ReadWrite<u32, Rvbaraddr3h::Register>,
    /// ACE Control Register
    pub ace_ctrl: ReadWrite<u32, AceCtrl::Register>,
    _padding100: [u8; 28],
    /// Snoop Control Register
    pub snoop_ctrl: ReadWrite<u32, SnoopCtrl::Register>,
    _padding132: [u8; 12],
    /// Power Control Register
    pub pwrctl: ReadWrite<u32, Pwrctl::Register>,
    /// Power Status Register
    pub pwrstat: ReadOnly<u32, Pwrstat::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub ErrCtrl [
        PSLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Isr [
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imr [
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ien [
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ids [
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Config0 [
        CFGTE OFFSET(24) NUMBITS(4) [],
        CFGEND OFFSET(16) NUMBITS(4) [],
        VINITHI OFFSET(8) NUMBITS(4) [],
        AA64NAA32 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Config1 [
        L2RSTDISABLE OFFSET(29) NUMBITS(1) [],
        L1RSTDISABLE OFFSET(28) NUMBITS(1) [],
        CP15DISABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rvbaraddr0l [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rvbaraddr0h [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rvbaraddr1l [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rvbaraddr1h [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rvbaraddr2l [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rvbaraddr2h [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rvbaraddr3l [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rvbaraddr3h [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AceCtrl [
        AWQOS OFFSET(16) NUMBITS(4) [],
        ARQOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SnoopCtrl [
        ACE_INACT OFFSET(4) NUMBITS(1) [],
        ACP_INACT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pwrctl [
        CLREXMONREQ OFFSET(17) NUMBITS(1) [],
        L2FLUSHREQ OFFSET(16) NUMBITS(1) [],
        CPUPWRDWNREQ OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pwrstat [
        CLREXMONACK OFFSET(17) NUMBITS(1) [],
        L2FLUSHDONE OFFSET(16) NUMBITS(1) [],
        DBGNOPWRDWN OFFSET(0) NUMBITS(4) [],
    ]
];

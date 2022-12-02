// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Application Processing Unit, APU Configuration
pub static mut APU: *mut Registers = 0xfd5c0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Control register
        (0x00000000 => pub err_ctrl: ReadWrite<u32, ErrCtrl::Register>),
        (0x00000004 => _padding4),
        /// Interrupt Status Register
        (0x00000010 => pub isr: ReadWrite<u32, Isr::Register>),
        /// Interrupt Mask Register
        (0x00000014 => pub imr: ReadOnly<u32, Imr::Register>),
        /// Interrupt Enable Register
        (0x00000018 => pub ien: WriteOnly<u32, Ien::Register>),
        /// Interrupt Disable Register
        (0x0000001c => pub ids: WriteOnly<u32, Ids::Register>),
        /// CPU Core Configuration
        (0x00000020 => pub config_0: ReadWrite<u32, Config0::Register>),
        /// L2 Configuration
        (0x00000024 => pub config_1: ReadWrite<u32, Config1::Register>),
        (0x00000028 => _padding40),
        /// Reset Vector Base Address
        (0x00000040 => pub rvbaraddr0l: ReadWrite<u32, Rvbaraddr0l::Register>),
        /// Reset Vector Base Address
        (0x00000044 => pub rvbaraddr0h: ReadWrite<u32, Rvbaraddr0h::Register>),
        /// Reset Vector Base Address
        (0x00000048 => pub rvbaraddr1l: ReadWrite<u32, Rvbaraddr1l::Register>),
        /// Reset Vector Base Address
        (0x0000004c => pub rvbaraddr1h: ReadWrite<u32, Rvbaraddr1h::Register>),
        /// Reset Vector Base Address
        (0x00000050 => pub rvbaraddr2l: ReadWrite<u32, Rvbaraddr2l::Register>),
        /// Reset Vector Base Address
        (0x00000054 => pub rvbaraddr2h: ReadWrite<u32, Rvbaraddr2h::Register>),
        /// Reset Vector Base Address
        (0x00000058 => pub rvbaraddr3l: ReadWrite<u32, Rvbaraddr3l::Register>),
        /// Reset Vector Base Address
        (0x0000005c => pub rvbaraddr3h: ReadWrite<u32, Rvbaraddr3h::Register>),
        /// ACE Control Register
        (0x00000060 => pub ace_ctrl: ReadWrite<u32, AceCtrl::Register>),
        (0x00000064 => _padding100),
        /// Snoop Control Register
        (0x00000080 => pub snoop_ctrl: ReadWrite<u32, SnoopCtrl::Register>),
        (0x00000084 => _padding132),
        /// Power Control Register
        (0x00000090 => pub pwrctl: ReadWrite<u32, Pwrctl::Register>),
        /// Power Status Register
        (0x00000094 => pub pwrstat: ReadOnly<u32, Pwrstat::Register>),
        (0x00000098 => @END),
    }
}
register_bitfields! [
    u32,
    pub ErrCtrl [
        PSLVERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Isr [
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Imr [
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Ien [
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Ids [
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Config0 [
        CFGTE OFFSET(24) NUMBITS(4) [],
        CFGEND OFFSET(16) NUMBITS(4) [],
        VINITHI OFFSET(8) NUMBITS(4) [],
        AA64NAA32 OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Config1 [
        L2RSTDISABLE OFFSET(29) NUMBITS(1) [],
        L1RSTDISABLE OFFSET(28) NUMBITS(1) [],
        CP15DISABLE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Rvbaraddr0l [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
register_bitfields! [
    u32,
    pub Rvbaraddr0h [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Rvbaraddr1l [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
register_bitfields! [
    u32,
    pub Rvbaraddr1h [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Rvbaraddr2l [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
register_bitfields! [
    u32,
    pub Rvbaraddr2h [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Rvbaraddr3l [
        ADDR OFFSET(2) NUMBITS(30) [],
    ]
];
register_bitfields! [
    u32,
    pub Rvbaraddr3h [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub AceCtrl [
        AWQOS OFFSET(16) NUMBITS(4) [],
        ARQOS OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub SnoopCtrl [
        ACE_INACT OFFSET(4) NUMBITS(1) [],
        ACP_INACT OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pwrctl [
        CLREXMONREQ OFFSET(17) NUMBITS(1) [],
        L2FLUSHREQ OFFSET(16) NUMBITS(1) [],
        CPUPWRDWNREQ OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pwrstat [
        CLREXMONACK OFFSET(17) NUMBITS(1) [],
        L2FLUSHDONE OFFSET(16) NUMBITS(1) [],
        DBGNOPWRDWN OFFSET(0) NUMBITS(4) [],
    ]
];

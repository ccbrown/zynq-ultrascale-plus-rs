// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, WriteOnly};
/// System Watchdog Timer, CSU System Watchdog Timer
pub static mut CSU_WDT: *mut Registers = 0xffcb0000 as *mut Registers;
/// System Watchdog Timer, LPD System Watchdog Timer
pub static mut SWDT: *mut Registers = 0xff150000 as *mut Registers;
/// System Watchdog Timer, FPD System Watchdog Timer
pub static mut WDT: *mut Registers = 0xfd4d0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// WD zero mode register
        (0x00000000 => pub mode: Aliased<u32, ModeR::Register, ModeW::Register>),
        /// Counter Control Register
        (0x00000004 => pub control: Aliased<u32, ControlR::Register, ControlW::Register>),
        /// Restart key register - this not a real register as no data is stored
        (0x00000008 => pub restart: WriteOnly<u16>),
        (0x0000000a => _padding10),
        /// Status Register
        (0x0000000c => pub status: ReadOnly<u8, Status::Register>),
        (0x0000000d => @END),
    }
}
register_bitfields! [
    u32,
    pub ModeR [
        IRQLN OFFSET(7) NUMBITS(2) [],
        RSTLN OFFSET(4) NUMBITS(3) [],
        IRQEN OFFSET(2) NUMBITS(1) [],
        RSTEN OFFSET(1) NUMBITS(1) [],
        WDEN OFFSET(0) NUMBITS(1) [],
    ],
    pub ModeW [
        ZKEY OFFSET(12) NUMBITS(12) [],
        IRQLN OFFSET(7) NUMBITS(2) [],
        RSTLN OFFSET(4) NUMBITS(3) [],
        IRQEN OFFSET(2) NUMBITS(1) [],
        RSTEN OFFSET(1) NUMBITS(1) [],
        WDEN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ControlR [
        CRV OFFSET(2) NUMBITS(12) [],
        CLKSEL OFFSET(0) NUMBITS(2) [],
    ],
    pub ControlW [
        CKEY OFFSET(14) NUMBITS(12) [],
        CRV OFFSET(2) NUMBITS(12) [],
        CLKSEL OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u8,
    pub Status [
        WDZ OFFSET(0) NUMBITS(1) [],
    ]
];

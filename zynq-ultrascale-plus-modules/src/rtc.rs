// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// Real-time Clock Control and Configuration, Real Time Clock Control and Configuration
pub static mut RTC: *mut Registers = 0xffa60000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Set the Current Time.
    pub set_time_write: WriteOnly<u32>,
    /// Last Current-time Value Programmed.
    pub set_time_read: ReadOnly<u32>,
    /// One Second Time Based.
    pub calib_write: WriteOnly<u32, CalibWrite::Register>,
    /// Read-back Calibration Value.
    pub calib_read: ReadOnly<u32, CalibRead::Register>,
    /// Current time in seconds.
    pub current_time: ReadOnly<u32>,
    _padding20: [u8; 4],
    /// Alarm Value.
    pub alarm: ReadWrite<u32>,
    _padding28: [u8; 4],
    /// Interrupt Status.
    pub rtc_int_status: ReadWrite<u8, RtcIntStatus::Register>,
    _padding33: [u8; 3],
    /// Interrupt Mask.
    pub rtc_int_mask: ReadOnly<u8, RtcIntMask::Register>,
    _padding37: [u8; 3],
    /// Interrupt Enable.
    pub rtc_int_en: WriteOnly<u8, RtcIntEn::Register>,
    _padding41: [u8; 3],
    /// Interrupt Disable.
    pub rtc_int_dis: WriteOnly<u8, RtcIntDis::Register>,
    _padding45: [u8; 3],
    /// Address Decode Error Status.
    pub addr_error: ReadWrite<u8, AddrError::Register>,
    _padding49: [u8; 3],
    /// Address Decode Error Interrupt Mask.
    pub addr_error_int_mask: ReadOnly<u8, AddrErrorIntMask::Register>,
    _padding53: [u8; 3],
    /// Address Decode Error Interrupt Enable.
    pub addr_error_int_en: WriteOnly<u8, AddrErrorIntEn::Register>,
    _padding57: [u8; 3],
    /// Address Decode Error Interrupt Disable.
    pub addr_error_int_dis: WriteOnly<u8, AddrErrorIntDis::Register>,
    _padding61: [u8; 3],
    /// Control.
    pub control: Aliased<u32, ControlR::Register, ControlW::Register>,
    _padding68: [u8; 12],
    /// Safety Check.
    pub safety_chk: ReadWrite<u32>,
}
tock_registers::register_bitfields! [
    u32,
    pub CalibWrite [
        FRACTION_EN OFFSET(20) NUMBITS(1) [],
        FRACTION_DATA OFFSET(16) NUMBITS(4) [],
        MAX_TICK OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CalibRead [
        FRACTION_EN OFFSET(20) NUMBITS(1) [],
        FRACTION_DATA OFFSET(16) NUMBITS(4) [],
        MAX_TICK OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub RtcIntStatus [
        ALARM OFFSET(1) NUMBITS(1) [],
        SECONDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub RtcIntMask [
        ALARM OFFSET(1) NUMBITS(1) [],
        SECONDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub RtcIntEn [
        ALARM OFFSET(1) NUMBITS(1) [],
        SECONDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub RtcIntDis [
        ALARM OFFSET(1) NUMBITS(1) [],
        SECONDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub AddrError [
        STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub AddrErrorIntMask [
        MASK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub AddrErrorIntEn [
        MASK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub AddrErrorIntDis [
        MASK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ControlR [
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        OSC_CNTRL OFFSET(24) NUMBITS(4) [],
        RESERVED1 OFFSET(1) NUMBITS(23) [],
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub ControlW [
        BATTERY_ENABLE OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        OSC_CNTRL OFFSET(24) NUMBITS(4) [],
        RESERVED1 OFFSET(1) NUMBITS(23) [],
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];

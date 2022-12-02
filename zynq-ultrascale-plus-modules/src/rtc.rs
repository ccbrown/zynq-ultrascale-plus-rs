// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// Real-time Clock Control and Configuration, Real Time Clock Control and Configuration
pub static mut RTC: *mut Registers = 0xffa60000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Set the Current Time.
        (0x00000000 => pub set_time_write: WriteOnly<u32>),
        /// Last Current-time Value Programmed.
        (0x00000004 => pub set_time_read: ReadOnly<u32>),
        /// One Second Time Based.
        (0x00000008 => pub calib_write: WriteOnly<u32, CalibWrite::Register>),
        /// Read-back Calibration Value.
        (0x0000000c => pub calib_read: ReadOnly<u32, CalibRead::Register>),
        /// Current time in seconds.
        (0x00000010 => pub current_time: ReadOnly<u32>),
        (0x00000014 => _padding20),
        /// Alarm Value.
        (0x00000018 => pub alarm: ReadWrite<u32>),
        (0x0000001c => _padding28),
        /// Interrupt Status.
        (0x00000020 => pub rtc_int_status: ReadWrite<u8, RtcIntStatus::Register>),
        (0x00000021 => _padding33),
        /// Interrupt Mask.
        (0x00000024 => pub rtc_int_mask: ReadOnly<u8, RtcIntMask::Register>),
        (0x00000025 => _padding37),
        /// Interrupt Enable.
        (0x00000028 => pub rtc_int_en: WriteOnly<u8, RtcIntEn::Register>),
        (0x00000029 => _padding41),
        /// Interrupt Disable.
        (0x0000002c => pub rtc_int_dis: WriteOnly<u8, RtcIntDis::Register>),
        (0x0000002d => _padding45),
        /// Address Decode Error Status.
        (0x00000030 => pub addr_error: ReadWrite<u8, AddrError::Register>),
        (0x00000031 => _padding49),
        /// Address Decode Error Interrupt Mask.
        (0x00000034 => pub addr_error_int_mask: ReadOnly<u8, AddrErrorIntMask::Register>),
        (0x00000035 => _padding53),
        /// Address Decode Error Interrupt Enable.
        (0x00000038 => pub addr_error_int_en: WriteOnly<u8, AddrErrorIntEn::Register>),
        (0x00000039 => _padding57),
        /// Address Decode Error Interrupt Disable.
        (0x0000003c => pub addr_error_int_dis: WriteOnly<u8, AddrErrorIntDis::Register>),
        (0x0000003d => _padding61),
        /// Control.
        (0x00000040 => pub control: Aliased<u32, ControlR::Register, ControlW::Register>),
        (0x00000044 => _padding68),
        /// Safety Check.
        (0x00000050 => pub safety_chk: ReadWrite<u32>),
        (0x00000054 => @END),
    }
}
register_bitfields! [
    u32,
    pub CalibWrite [
        FRACTION_EN OFFSET(20) NUMBITS(1) [],
        FRACTION_DATA OFFSET(16) NUMBITS(4) [],
        MAX_TICK OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub CalibRead [
        FRACTION_EN OFFSET(20) NUMBITS(1) [],
        FRACTION_DATA OFFSET(16) NUMBITS(4) [],
        MAX_TICK OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u8,
    pub RtcIntStatus [
        ALARM OFFSET(1) NUMBITS(1) [],
        SECONDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RtcIntMask [
        ALARM OFFSET(1) NUMBITS(1) [],
        SECONDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RtcIntEn [
        ALARM OFFSET(1) NUMBITS(1) [],
        SECONDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RtcIntDis [
        ALARM OFFSET(1) NUMBITS(1) [],
        SECONDS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub AddrError [
        STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub AddrErrorIntMask [
        MASK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub AddrErrorIntEn [
        MASK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub AddrErrorIntDis [
        MASK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
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

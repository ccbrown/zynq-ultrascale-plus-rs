// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// Triple Timer Counter, Triple Timer Counter
pub static mut TTC0: *mut Registers = 0xff110000 as *mut Registers;
/// Triple Timer Counter, Triple Timer Counter
pub static mut TTC1: *mut Registers = 0xff120000 as *mut Registers;
/// Triple Timer Counter, Triple Timer Counter
pub static mut TTC2: *mut Registers = 0xff130000 as *mut Registers;
/// Triple Timer Counter, Triple Timer Counter
pub static mut TTC3: *mut Registers = 0xff140000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Clock Control register
    pub clock_control_1: ReadWrite<u8, ClockControl1::Register>,
    _padding1: [u8; 3],
    /// Clock Control register
    pub clock_control_2: ReadWrite<u8, ClockControl2::Register>,
    _padding5: [u8; 3],
    /// Clock Control register
    pub clock_control_3: ReadWrite<u8, ClockControl3::Register>,
    _padding9: [u8; 3],
    /// Operational mode and reset
    pub counter_control_1: ReadWrite<u8, CounterControl1::Register>,
    _padding13: [u8; 3],
    /// Operational mode and reset
    pub counter_control_2: ReadWrite<u8, CounterControl2::Register>,
    _padding17: [u8; 3],
    /// Operational mode and reset
    pub counter_control_3: ReadWrite<u8, CounterControl3::Register>,
    _padding21: [u8; 3],
    /// Current counter value
    pub counter_value_1: ReadOnly<u32>,
    /// Current counter value
    pub counter_value_2: ReadOnly<u32>,
    /// Current counter value
    pub counter_value_3: ReadOnly<u32>,
    /// Interval value
    pub interval_counter_1: ReadWrite<u32>,
    /// Interval value
    pub interval_counter_2: ReadWrite<u32>,
    /// Interval value
    pub interval_counter_3: ReadWrite<u32>,
    /// Match value
    pub match_1_counter_1: ReadWrite<u32>,
    /// Match value
    pub match_1_counter_2: ReadWrite<u32>,
    /// Match value
    pub match_1_counter_3: ReadWrite<u32>,
    /// Match value
    pub match_2_counter_1: ReadWrite<u32>,
    /// Match value
    pub match_2_counter_2: ReadWrite<u32>,
    /// Match value
    pub match_2_counter_3: ReadWrite<u32>,
    /// Match value
    pub match_3_counter_1: ReadWrite<u32>,
    /// Match value
    pub match_3_counter_2: ReadWrite<u32>,
    /// Match value
    pub match_3_counter_3: ReadWrite<u32>,
    /// Counter 1 Interval, Match, Overflowand Event interrupts
    pub interrupt_register_1: ReadOnly<u8, InterruptRegister1::Register>,
    _padding85: [u8; 3],
    /// Counter 2 Interval, Match, Overflow and Event interrupts
    pub interrupt_register_2: ReadOnly<u8, InterruptRegister2::Register>,
    _padding89: [u8; 3],
    /// Counter 3 Interval, Match, Overflow and Event interrupts
    pub interrupt_register_3: ReadOnly<u8, InterruptRegister3::Register>,
    _padding93: [u8; 3],
    /// ANDed with corresponding InterruptRegister
    pub interrupt_enable_1: ReadWrite<u8, InterruptEnable1::Register>,
    _padding97: [u8; 3],
    /// ANDed with corresponding InterruptRegister
    pub interrupt_enable_2: ReadWrite<u8, InterruptEnable2::Register>,
    _padding101: [u8; 3],
    /// ANDed with corresponding InterruptRegister
    pub interrupt_enable_3: ReadWrite<u8, InterruptEnable3::Register>,
    _padding105: [u8; 3],
    /// Enable, pulse and overflow
    pub event_control_timer_1: ReadWrite<u8, EventControlTimer1::Register>,
    _padding109: [u8; 3],
    /// Enable, pulse and overflow
    pub event_control_timer_2: ReadWrite<u8, EventControlTimer2::Register>,
    _padding113: [u8; 3],
    /// Enable, pulse and overflow
    pub event_control_timer_3: ReadWrite<u8, EventControlTimer3::Register>,
    _padding117: [u8; 3],
    /// LPD_LSBUS_CLK clock count for event
    pub event_register_1: ReadOnly<u32>,
    /// LPD_LSBUS_CLK clock count for event
    pub event_register_2: ReadOnly<u32>,
    /// LPD_LSBUS_CLK clock count for event
    pub event_register_3: ReadOnly<u32>,
}
tock_registers::register_bitfields! [
    u8,
    pub ClockControl1 [
        EX_E OFFSET(6) NUMBITS(1) [],
        C_SRC OFFSET(5) NUMBITS(1) [],
        PS_V OFFSET(1) NUMBITS(4) [],
        PS_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ClockControl2 [
        EX_E OFFSET(6) NUMBITS(1) [],
        C_SRC OFFSET(5) NUMBITS(1) [],
        PS_V OFFSET(1) NUMBITS(4) [],
        PS_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ClockControl3 [
        EX_E OFFSET(6) NUMBITS(1) [],
        C_SRC OFFSET(5) NUMBITS(1) [],
        PS_V OFFSET(1) NUMBITS(4) [],
        PS_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub CounterControl1 [
        WAVE_POL OFFSET(6) NUMBITS(1) [],
        WAVE_EN OFFSET(5) NUMBITS(1) [],
        RST OFFSET(4) NUMBITS(1) [],
        MATCH OFFSET(3) NUMBITS(1) [],
        DEC OFFSET(2) NUMBITS(1) [],
        INT OFFSET(1) NUMBITS(1) [],
        DIS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub CounterControl2 [
        WAVE_POL OFFSET(6) NUMBITS(1) [],
        WAVE_EN OFFSET(5) NUMBITS(1) [],
        RST OFFSET(4) NUMBITS(1) [],
        MATCH OFFSET(3) NUMBITS(1) [],
        DEC OFFSET(2) NUMBITS(1) [],
        INT OFFSET(1) NUMBITS(1) [],
        DIS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub CounterControl3 [
        WAVE_POL OFFSET(6) NUMBITS(1) [],
        WAVE_EN OFFSET(5) NUMBITS(1) [],
        RST OFFSET(4) NUMBITS(1) [],
        MATCH OFFSET(3) NUMBITS(1) [],
        DEC OFFSET(2) NUMBITS(1) [],
        INT OFFSET(1) NUMBITS(1) [],
        DIS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub InterruptRegister1 [
        EV OFFSET(5) NUMBITS(1) [],
        OV OFFSET(4) NUMBITS(1) [],
        M3 OFFSET(3) NUMBITS(1) [],
        M2 OFFSET(2) NUMBITS(1) [],
        M1 OFFSET(1) NUMBITS(1) [],
        IV OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub InterruptRegister2 [
        EV OFFSET(5) NUMBITS(1) [],
        OV OFFSET(4) NUMBITS(1) [],
        M3 OFFSET(3) NUMBITS(1) [],
        M2 OFFSET(2) NUMBITS(1) [],
        M1 OFFSET(1) NUMBITS(1) [],
        IV OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub InterruptRegister3 [
        EV OFFSET(5) NUMBITS(1) [],
        OV OFFSET(4) NUMBITS(1) [],
        M3 OFFSET(3) NUMBITS(1) [],
        M2 OFFSET(2) NUMBITS(1) [],
        M1 OFFSET(1) NUMBITS(1) [],
        IV OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub InterruptEnable1 [
        IEN OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub InterruptEnable2 [
        IEN OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub InterruptEnable3 [
        IEN OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EventControlTimer1 [
        E_TM OFFSET(3) NUMBITS(1) [],
        E_OV OFFSET(2) NUMBITS(1) [],
        E_LO OFFSET(1) NUMBITS(1) [],
        E_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EventControlTimer2 [
        E_TM OFFSET(3) NUMBITS(1) [],
        E_OV OFFSET(2) NUMBITS(1) [],
        E_LO OFFSET(1) NUMBITS(1) [],
        E_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EventControlTimer3 [
        E_TM OFFSET(3) NUMBITS(1) [],
        E_OV OFFSET(2) NUMBITS(1) [],
        E_LO OFFSET(1) NUMBITS(1) [],
        E_EN OFFSET(0) NUMBITS(1) [],
    ]
];

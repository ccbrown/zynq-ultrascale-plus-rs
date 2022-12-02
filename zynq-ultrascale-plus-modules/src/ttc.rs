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
register_structs! {
    pub Registers {
        /// Clock Control register
        (0x00000000 => pub clock_control_1: ReadWrite<u8, ClockControl1::Register>),
        (0x00000001 => _padding1),
        /// Clock Control register
        (0x00000004 => pub clock_control_2: ReadWrite<u8, ClockControl2::Register>),
        (0x00000005 => _padding5),
        /// Clock Control register
        (0x00000008 => pub clock_control_3: ReadWrite<u8, ClockControl3::Register>),
        (0x00000009 => _padding9),
        /// Operational mode and reset
        (0x0000000c => pub counter_control_1: ReadWrite<u8, CounterControl1::Register>),
        (0x0000000d => _padding13),
        /// Operational mode and reset
        (0x00000010 => pub counter_control_2: ReadWrite<u8, CounterControl2::Register>),
        (0x00000011 => _padding17),
        /// Operational mode and reset
        (0x00000014 => pub counter_control_3: ReadWrite<u8, CounterControl3::Register>),
        (0x00000015 => _padding21),
        /// Current counter value
        (0x00000018 => pub counter_value_1: ReadOnly<u32>),
        /// Current counter value
        (0x0000001c => pub counter_value_2: ReadOnly<u32>),
        /// Current counter value
        (0x00000020 => pub counter_value_3: ReadOnly<u32>),
        /// Interval value
        (0x00000024 => pub interval_counter_1: ReadWrite<u32>),
        /// Interval value
        (0x00000028 => pub interval_counter_2: ReadWrite<u32>),
        /// Interval value
        (0x0000002c => pub interval_counter_3: ReadWrite<u32>),
        /// Match value
        (0x00000030 => pub match_1_counter_1: ReadWrite<u32>),
        /// Match value
        (0x00000034 => pub match_1_counter_2: ReadWrite<u32>),
        /// Match value
        (0x00000038 => pub match_1_counter_3: ReadWrite<u32>),
        /// Match value
        (0x0000003c => pub match_2_counter_1: ReadWrite<u32>),
        /// Match value
        (0x00000040 => pub match_2_counter_2: ReadWrite<u32>),
        /// Match value
        (0x00000044 => pub match_2_counter_3: ReadWrite<u32>),
        /// Match value
        (0x00000048 => pub match_3_counter_1: ReadWrite<u32>),
        /// Match value
        (0x0000004c => pub match_3_counter_2: ReadWrite<u32>),
        /// Match value
        (0x00000050 => pub match_3_counter_3: ReadWrite<u32>),
        /// Counter 1 Interval, Match, Overflowand Event interrupts
        (0x00000054 => pub interrupt_register_1: ReadOnly<u8, InterruptRegister1::Register>),
        (0x00000055 => _padding85),
        /// Counter 2 Interval, Match, Overflow and Event interrupts
        (0x00000058 => pub interrupt_register_2: ReadOnly<u8, InterruptRegister2::Register>),
        (0x00000059 => _padding89),
        /// Counter 3 Interval, Match, Overflow and Event interrupts
        (0x0000005c => pub interrupt_register_3: ReadOnly<u8, InterruptRegister3::Register>),
        (0x0000005d => _padding93),
        /// ANDed with corresponding InterruptRegister
        (0x00000060 => pub interrupt_enable_1: ReadWrite<u8, InterruptEnable1::Register>),
        (0x00000061 => _padding97),
        /// ANDed with corresponding InterruptRegister
        (0x00000064 => pub interrupt_enable_2: ReadWrite<u8, InterruptEnable2::Register>),
        (0x00000065 => _padding101),
        /// ANDed with corresponding InterruptRegister
        (0x00000068 => pub interrupt_enable_3: ReadWrite<u8, InterruptEnable3::Register>),
        (0x00000069 => _padding105),
        /// Enable, pulse and overflow
        (0x0000006c => pub event_control_timer_1: ReadWrite<u8, EventControlTimer1::Register>),
        (0x0000006d => _padding109),
        /// Enable, pulse and overflow
        (0x00000070 => pub event_control_timer_2: ReadWrite<u8, EventControlTimer2::Register>),
        (0x00000071 => _padding113),
        /// Enable, pulse and overflow
        (0x00000074 => pub event_control_timer_3: ReadWrite<u8, EventControlTimer3::Register>),
        (0x00000075 => _padding117),
        /// LPD_LSBUS_CLK clock count for event
        (0x00000078 => pub event_register_1: ReadOnly<u32>),
        /// LPD_LSBUS_CLK clock count for event
        (0x0000007c => pub event_register_2: ReadOnly<u32>),
        /// LPD_LSBUS_CLK clock count for event
        (0x00000080 => pub event_register_3: ReadOnly<u32>),
        (0x00000084 => @END),
    }
}
register_bitfields! [
    u8,
    pub ClockControl1 [
        EX_E OFFSET(6) NUMBITS(1) [],
        C_SRC OFFSET(5) NUMBITS(1) [],
        PS_V OFFSET(1) NUMBITS(4) [],
        PS_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub ClockControl2 [
        EX_E OFFSET(6) NUMBITS(1) [],
        C_SRC OFFSET(5) NUMBITS(1) [],
        PS_V OFFSET(1) NUMBITS(4) [],
        PS_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub ClockControl3 [
        EX_E OFFSET(6) NUMBITS(1) [],
        C_SRC OFFSET(5) NUMBITS(1) [],
        PS_V OFFSET(1) NUMBITS(4) [],
        PS_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
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
register_bitfields! [
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
register_bitfields! [
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
register_bitfields! [
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
register_bitfields! [
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
register_bitfields! [
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
register_bitfields! [
    u8,
    pub InterruptEnable1 [
        IEN OFFSET(0) NUMBITS(6) [],
    ]
];
register_bitfields! [
    u8,
    pub InterruptEnable2 [
        IEN OFFSET(0) NUMBITS(6) [],
    ]
];
register_bitfields! [
    u8,
    pub InterruptEnable3 [
        IEN OFFSET(0) NUMBITS(6) [],
    ]
];
register_bitfields! [
    u8,
    pub EventControlTimer1 [
        E_TM OFFSET(3) NUMBITS(1) [],
        E_OV OFFSET(2) NUMBITS(1) [],
        E_LO OFFSET(1) NUMBITS(1) [],
        E_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub EventControlTimer2 [
        E_TM OFFSET(3) NUMBITS(1) [],
        E_OV OFFSET(2) NUMBITS(1) [],
        E_LO OFFSET(1) NUMBITS(1) [],
        E_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub EventControlTimer3 [
        E_TM OFFSET(3) NUMBITS(1) [],
        E_OV OFFSET(2) NUMBITS(1) [],
        E_LO OFFSET(1) NUMBITS(1) [],
        E_EN OFFSET(0) NUMBITS(1) [],
    ]
];

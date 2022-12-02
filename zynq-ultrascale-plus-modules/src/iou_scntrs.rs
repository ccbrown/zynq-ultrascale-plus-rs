// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// System Timestamp Generator - Secure, System Timestamp Generator - Secure
pub static mut IOU_SCNTRS: *mut Registers = 0xff260000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Controls the counter increments. This register is not accessible to the read-only programming interface.
        (0x00000000 => pub counter_control: Aliased<u32, CounterControlR::Register, CounterControlW::Register>),
        /// Identifies the status of the counter. This register is not accessible to the read-only programming interface.
        (0x00000004 => pub counter_status: ReadOnly<u32, CounterStatus::Register>),
        /// Reads or writes the lower 32 bits of the current counter value. The read-only programming interface can read but not write to this register. The control interface must clear the CNTCR.EN bit before writing to this register.
        (0x00000008 => pub current_counter_value_lower: ReadWrite<u32>),
        /// Reads or writes the upper 32 bits of the current counter value. The read-only programming interface can read but not write this register. The control interface must clear the CNTCR.EN bit before writing to this register.
        (0x0000000c => pub current_counter_value_upper: ReadWrite<u32>),
        (0x00000010 => _padding16),
        /// Program this register to match the clock frequency of the timestamp generator, in ticks per second. For example, for a 50 MHz clock, program 0x02FAF080. This register is not accessible to the read-only programming interface.
        (0x00000020 => pub base_frequency_id: ReadWrite<u32>),
        (0x00000024 => @END),
    }
}
register_bitfields! [
    u32,
    pub CounterControlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        HDBG OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub CounterControlW [
        HDBG OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub CounterStatus [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        DBGH OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];

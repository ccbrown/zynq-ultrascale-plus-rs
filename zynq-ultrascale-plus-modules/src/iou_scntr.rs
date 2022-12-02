// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::ReadWrite;
/// System Timestamp Generator, System Timestamp Generator
pub static mut IOU_SCNTR: *mut Registers = 0xff250000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Reads or writes the lower 32 bits of the current counter value. The read-only programming interface can read but not write to this register. The control interface must clear the CNTCR.EN bit before writing to this register.
        (0x00000000 => pub current_counter_value_lower: ReadWrite<u32>),
        /// Reads or writes the upper 32 bits of the current counter value. The read-only programming interface can read but not write this register. The control interface must clear the CNTCR.EN bit before writing to this register.
        (0x00000004 => pub current_counter_value_upper: ReadWrite<u32>),
        (0x00000008 => @END),
    }
}

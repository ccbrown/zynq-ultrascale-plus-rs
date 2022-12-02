// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::ReadOnly;
/// R5 Integration ROM
pub static mut CORESIGHT_R5_ROM: *mut Registers = 0xfebe0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// CPU 0 Debug Component
        (0x00000000 => pub entry00: ReadOnly<u32>),
        /// CPU 0 CTI Component
        (0x00000004 => pub entry01: ReadOnly<u32>),
        /// CPU 0 ETM Component
        (0x00000008 => pub entry02: ReadOnly<u32>),
        /// CPU 1 Debug Component
        (0x0000000c => pub entry03: ReadOnly<u32>),
        /// CPU 1 CTI Component
        (0x00000010 => pub entry04: ReadOnly<u32>),
        /// CPU 1 ETM Component
        (0x00000014 => pub entry05: ReadOnly<u32>),
        (0x00000018 => _padding24),
        /// This register indicates the capabilities.
        (0x00000fc8 => pub devid: ReadOnly<u32>),
        /// It provides a debugger with information about the component.
        (0x00000fcc => pub devtype: ReadOnly<u32, Devtype::Register>),
        /// ROM Peripheral ID 4
        (0x00000fd0 => pub pidr4: ReadOnly<u32>),
        /// ROM Peripheral ID 5
        (0x00000fd4 => pub pidr5: ReadOnly<u32>),
        /// ROM Peripheral ID 6
        (0x00000fd8 => pub pidr6: ReadOnly<u32>),
        /// ROM Peripheral ID 7
        (0x00000fdc => pub pidr7: ReadOnly<u32>),
        /// ROM Peripheral ID 0
        (0x00000fe0 => pub pidr0: ReadOnly<u32>),
        /// ROM Peripheral ID 1
        (0x00000fe4 => pub pidr1: ReadOnly<u32>),
        /// ROM Peripheral ID 2
        (0x00000fe8 => pub pidr2: ReadOnly<u32>),
        /// ROM Peripheral ID 3
        (0x00000fec => pub pidr3: ReadOnly<u32>),
        /// ROM Component ID 0
        (0x00000ff0 => pub cidr0: ReadOnly<u32>),
        /// ROM Component ID 1
        (0x00000ff4 => pub cidr1: ReadOnly<u32>),
        /// ROM Component ID 2
        (0x00000ff8 => pub cidr2: ReadOnly<u32>),
        /// ROM Component ID 3
        (0x00000ffc => pub cidr3: ReadOnly<u32>),
        (0x00001000 => @END),
    }
}
register_bitfields! [
    u32,
    pub Devtype [
        SUB_TYPE OFFSET(4) NUMBITS(4) [],
        MAJOR_TYPE OFFSET(0) NUMBITS(4) [],
    ]
];

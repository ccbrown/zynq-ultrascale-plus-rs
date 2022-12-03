// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::ReadOnly;
/// R5 Integration ROM
pub static mut CORESIGHT_R5_ROM: *mut Registers = 0xfebe0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// CPU 0 Debug Component
    pub entry00: ReadOnly<u32>,
    /// CPU 0 CTI Component
    pub entry01: ReadOnly<u32>,
    /// CPU 0 ETM Component
    pub entry02: ReadOnly<u32>,
    /// CPU 1 Debug Component
    pub entry03: ReadOnly<u32>,
    /// CPU 1 CTI Component
    pub entry04: ReadOnly<u32>,
    /// CPU 1 ETM Component
    pub entry05: ReadOnly<u32>,
    _padding24: [u8; 4016],
    /// This register indicates the capabilities.
    pub devid: ReadOnly<u32>,
    /// It provides a debugger with information about the component.
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// ROM Peripheral ID 4
    pub pidr4: ReadOnly<u32>,
    /// ROM Peripheral ID 5
    pub pidr5: ReadOnly<u32>,
    /// ROM Peripheral ID 6
    pub pidr6: ReadOnly<u32>,
    /// ROM Peripheral ID 7
    pub pidr7: ReadOnly<u32>,
    /// ROM Peripheral ID 0
    pub pidr0: ReadOnly<u32>,
    /// ROM Peripheral ID 1
    pub pidr1: ReadOnly<u32>,
    /// ROM Peripheral ID 2
    pub pidr2: ReadOnly<u32>,
    /// ROM Peripheral ID 3
    pub pidr3: ReadOnly<u32>,
    /// ROM Component ID 0
    pub cidr0: ReadOnly<u32>,
    /// ROM Component ID 1
    pub cidr1: ReadOnly<u32>,
    /// ROM Component ID 2
    pub cidr2: ReadOnly<u32>,
    /// ROM Component ID 3
    pub cidr3: ReadOnly<u32>,
}
tock_registers::register_bitfields! [
    u32,
    pub Devtype [
        SUB_TYPE OFFSET(4) NUMBITS(4) [],
        MAJOR_TYPE OFFSET(0) NUMBITS(4) [],
    ]
];

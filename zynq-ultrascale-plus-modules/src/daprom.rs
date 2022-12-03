// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// CoreSight Debug Access Port ROM, DAP ROM
pub static mut CORESIGHT_SOC_ROM: *mut Registers = 0xfe800000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// ROM entry 0
    pub entry00: ReadOnly<u32, Entry00::Register>,
    /// ROM entry 1
    pub entry01: ReadOnly<u32, Entry01::Register>,
    /// ROM entry 2
    pub entry02: ReadOnly<u32, Entry02::Register>,
    /// ROM entry 3
    pub entry03: ReadOnly<u32, Entry03::Register>,
    /// ROM entry 4
    pub entry04: ReadOnly<u32, Entry04::Register>,
    /// ROM entry 5
    pub entry05: ReadOnly<u32, Entry05::Register>,
    /// ROM entry 6
    pub entry06: ReadOnly<u32, Entry06::Register>,
    /// ROM entry 7
    pub entry07: ReadOnly<u32, Entry07::Register>,
    /// ROM entry 8
    pub entry08: ReadOnly<u32, Entry08::Register>,
    /// ROM entry 9
    pub entry09: ReadOnly<u32, Entry09::Register>,
    /// ROM entry 10
    pub entry10: ReadOnly<u32, Entry10::Register>,
    /// ROM entry 11
    pub entry11: ReadOnly<u32, Entry11::Register>,
    /// ROM entry 12
    pub entry12: ReadOnly<u32, Entry12::Register>,
    /// ROM entry 13
    pub entry13: ReadOnly<u32, Entry13::Register>,
    /// ROM entry 14
    pub entry14: ReadOnly<u32, Entry14::Register>,
    /// ROM entry 15
    pub entry15: ReadOnly<u32, Entry15::Register>,
    /// ROM entry 16
    pub entry16: ReadOnly<u32, Entry16::Register>,
    /// ROM entry 17
    pub entry17: ReadOnly<u32, Entry17::Register>,
    /// Non-zero filler
    pub entry18: ReadOnly<u32, Entry18::Register>,
    /// Non-zero filler
    pub entry19: ReadOnly<u32, Entry19::Register>,
    /// Non-zero filler
    pub entry20: ReadOnly<u32, Entry20::Register>,
    /// Non-zero filler
    pub entry21: ReadOnly<u32, Entry21::Register>,
    /// Non-zero filler
    pub entry22: ReadOnly<u32, Entry22::Register>,
    /// Non-zero filler
    pub entry23: ReadOnly<u32, Entry23::Register>,
    /// Non-zero filler
    pub entry24: ReadOnly<u32, Entry24::Register>,
    /// Non-zero filler
    pub entry25: ReadOnly<u32, Entry25::Register>,
    /// Non-zero filler
    pub entry26: ReadOnly<u32, Entry26::Register>,
    /// Non-zero filler
    pub entry27: ReadOnly<u32, Entry27::Register>,
    /// Non-zero filler
    pub entry28: ReadOnly<u32, Entry28::Register>,
    /// Non-zero filler
    pub entry29: ReadOnly<u32, Entry29::Register>,
    /// Non-zero filler
    pub entry30: ReadOnly<u32, Entry30::Register>,
    /// Non-zero filler
    pub entry31: ReadOnly<u32, Entry31::Register>,
    _padding128: [u8; 3912],
    /// This register indicates the capabilities.
    pub devid: ReadOnly<u32>,
    /// It provides a debugger with information about the component.
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// Peripheral Identification register 4
    pub pidr4: ReadOnly<u32, Pidr4::Register>,
    /// Peripheral Identification register 5
    pub pidr5: ReadWrite<u32>,
    /// Peripheral Identification register 6
    pub pidr6: ReadWrite<u32>,
    /// Peripheral Identification register 7
    pub pidr7: ReadWrite<u32>,
    /// Peripheral Identification register 0
    pub pidr0: ReadOnly<u32, Pidr0::Register>,
    /// Peripheral Identification register 1
    pub pidr1: ReadOnly<u32, Pidr1::Register>,
    /// Peripheral Identification register 2
    pub pidr2: ReadOnly<u32, Pidr2::Register>,
    /// Peripheral Identification register 3
    pub pidr3: ReadOnly<u32, Pidr3::Register>,
    /// Component Identification Register 0
    pub cidr0: ReadOnly<u32, Cidr0::Register>,
    /// Component Identification Register 1
    pub cidr1: ReadOnly<u32, Cidr1::Register>,
    /// Component Identification Register 2
    pub cidr2: ReadOnly<u32, Cidr2::Register>,
    /// Component Identification Register 3
    pub cidr3: ReadOnly<u32, Cidr3::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Entry00 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry01 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry02 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry03 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry04 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry05 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry06 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry07 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry08 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry09 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry10 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry11 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry12 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry13 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry14 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry15 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry16 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry17 [
        BASE_ADDR OFFSET(12) NUMBITS(19) [],
        FORMAT OFFSET(1) NUMBITS(1) [],
        ENTRY_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry18 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry19 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry20 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry21 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry22 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry23 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry24 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry25 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry26 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry27 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry28 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry29 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry30 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Entry31 [
        RESERVED0 OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devtype [
        SUB_TYPE OFFSET(4) NUMBITS(4) [],
        MAJOR_TYPE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr4 [
        SIZE OFFSET(4) NUMBITS(4) [],
        DES_2 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr0 [
        PART_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr1 [
        DES_0 OFFSET(4) NUMBITS(4) [],
        PART_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC OFFSET(3) NUMBITS(1) [],
        DES_1 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        CMOD OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr0 [
        PRMBL_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr1 [
        CLASS OFFSET(4) NUMBITS(4) [],
        PRMBL_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr2 [
        PRMBL_2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr3 [
        PRMBL_3 OFFSET(0) NUMBITS(8) [],
    ]
];

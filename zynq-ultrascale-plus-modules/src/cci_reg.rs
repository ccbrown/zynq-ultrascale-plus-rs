// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// AXI Cache Coherent Interconnect Configuration, AXI Cache Coherent Interconnect Configuration
pub static mut CCI_REG: *mut Registers = 0xfd5e0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Controls for the register block.
        (0x00000000 => pub misc_ctrl: ReadWrite<u8, MiscCtrl::Register>),
        (0x00000001 => _padding1),
        /// Interrupt Status Register. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
        (0x00000010 => pub isr_0: Aliased<u32, Isr0R::Register, Isr0W::Register>),
        /// Interrupt Mask Register. This is a read-only location and can be atomically altered by either the IDR or the IER.
        (0x00000014 => pub imr_0: ReadOnly<u32, Imr0::Register>),
        /// Interrupt Enable Register. A write of 1 to this location will unmask the interrupt
        (0x00000018 => pub ier_0: Aliased<u32, Ier0R::Register, Ier0W::Register>),
        /// Interrupt Disable Register. A write of 1 to this location will mask the interrupt
        (0x0000001c => pub idr_0: Aliased<u32, Idr0R::Register, Idr0W::Register>),
        (0x00000020 => _padding32),
        /// Misc control registesr
        (0x00000040 => pub cci_misc_ctrl: Aliased<u32, CciMiscCtrlR::Register, CciMiscCtrlW::Register>),
        (0x00000044 => @END),
    }
}
register_bitfields! [
    u8,
    pub MiscCtrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Isr0R [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(25) [],
        CCNT_OFLW OFFSET(5) NUMBITS(1) [],
        EC3_OFLW OFFSET(4) NUMBITS(1) [],
        EC2_OFLW OFFSET(3) NUMBITS(1) [],
        EC1_OFLW OFFSET(2) NUMBITS(1) [],
        EC0_OFLW OFFSET(1) NUMBITS(1) [],
        ERRORIRQ OFFSET(0) NUMBITS(1) [],
    ],
    pub Isr0W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        CCNT_OFLW OFFSET(5) NUMBITS(1) [],
        EC3_OFLW OFFSET(4) NUMBITS(1) [],
        EC2_OFLW OFFSET(3) NUMBITS(1) [],
        EC1_OFLW OFFSET(2) NUMBITS(1) [],
        EC0_OFLW OFFSET(1) NUMBITS(1) [],
        ERRORIRQ OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Imr0 [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(25) [],
        CCNT_OFLW OFFSET(5) NUMBITS(1) [],
        EC3_OFLW OFFSET(4) NUMBITS(1) [],
        EC2_OFLW OFFSET(3) NUMBITS(1) [],
        EC1_OFLW OFFSET(2) NUMBITS(1) [],
        EC0_OFLW OFFSET(1) NUMBITS(1) [],
        ERRORIRQ OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Ier0R [
        RESERVED0 OFFSET(6) NUMBITS(25) [],
    ],
    pub Ier0W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        CCNT_OFLW OFFSET(5) NUMBITS(1) [],
        EC3_OFLW OFFSET(4) NUMBITS(1) [],
        EC2_OFLW OFFSET(3) NUMBITS(1) [],
        EC1_OFLW OFFSET(2) NUMBITS(1) [],
        EC0_OFLW OFFSET(1) NUMBITS(1) [],
        ERRORIRQ OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Idr0R [
        RESERVED0 OFFSET(6) NUMBITS(25) [],
    ],
    pub Idr0W [
        ADDR_DECODE_ERR OFFSET(31) NUMBITS(1) [],
        CCNT_OFLW OFFSET(5) NUMBITS(1) [],
        EC3_OFLW OFFSET(4) NUMBITS(1) [],
        EC2_OFLW OFFSET(3) NUMBITS(1) [],
        EC1_OFLW OFFSET(2) NUMBITS(1) [],
        EC0_OFLW OFFSET(1) NUMBITS(1) [],
        ERRORIRQ OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub CciMiscCtrlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        NIDEN OFFSET(1) NUMBITS(1) [],
        SPINDEN OFFSET(0) NUMBITS(1) [],
    ],
    pub CciMiscCtrlW [
        NIDEN OFFSET(1) NUMBITS(1) [],
        SPINDEN OFFSET(0) NUMBITS(1) [],
    ]
];

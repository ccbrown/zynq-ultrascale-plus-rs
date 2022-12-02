// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// Inter Integrated Circuit (I2C) Controller, I2C 0 Controller
pub static mut I2C0: *mut Registers = 0xff020000 as *mut Registers;
/// Inter Integrated Circuit (I2C) Controller, I2C 1 Controller
pub static mut I2C1: *mut Registers = 0xff030000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Control Register
        (0x00000000 => pub control_reg: Aliased<u16, ControlRegR::Register, ControlRegW::Register>),
        (0x00000002 => _padding2),
        /// Status register
        (0x00000004 => pub status_reg: ReadOnly<u16, StatusReg::Register>),
        (0x00000006 => _padding6),
        /// IIC Address register
        (0x00000008 => pub i2c_address: Aliased<u16, I2cAddressR::Register, I2cAddressW::Register>),
        (0x0000000a => _padding10),
        /// IIC data register
        (0x0000000c => pub i2c_data: Aliased<u16, I2cDataR::Register, I2cDataW::Register>),
        (0x0000000e => _padding14),
        /// IIC interrupt status register
        (0x00000010 => pub interrupt_status: ReadWrite<u16, InterruptStatus::Register>),
        (0x00000012 => _padding18),
        /// Transfer Size Register
        (0x00000014 => pub transfer_size: ReadWrite<u8, TransferSize::Register>),
        (0x00000015 => _padding21),
        /// Slave Monitor Pause
        (0x00000018 => pub slave_mon_pause: Aliased<u8, SlaveMonPauseR::Register, SlaveMonPauseW::Register>),
        (0x00000019 => _padding25),
        /// I/O Clock Signal (SCL) Timeout Period
        (0x0000001c => pub time_out: ReadWrite<u8, TimeOut::Register>),
        (0x0000001d => _padding29),
        /// Interrupt Mask
        (0x00000020 => pub intrpt_mask: ReadOnly<u16, IntrptMask::Register>),
        (0x00000022 => _padding34),
        /// Interrupt Enable Register
        (0x00000024 => pub intrpt_enable: Aliased<u16, IntrptEnableR::Register, IntrptEnableW::Register>),
        (0x00000026 => _padding38),
        /// Interrupt Disable Register
        (0x00000028 => pub intrpt_disable: Aliased<u16, IntrptDisableR::Register, IntrptDisableW::Register>),
        (0x0000002a => _padding42),
        /// Glitch Filter Control RegisterIt is used for setting the length of the glitch filter shift register.If the length of glitch filter shift register is set to zero (0x0) then the glitch filter is bypassed.
        (0x0000002c => pub glitch_filter: Aliased<u16, GlitchFilterR::Register, GlitchFilterW::Register>),
        (0x0000002e => @END),
    }
}
register_bitfields! [
    u16,
    pub ControlRegR [
        DIVISOR_A OFFSET(14) NUMBITS(2) [],
        DIVISOR_B OFFSET(8) NUMBITS(6) [],
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        CLR_FIFO OFFSET(6) NUMBITS(1) [],
        SLVMON OFFSET(5) NUMBITS(1) [],
        HOLD OFFSET(4) NUMBITS(1) [],
        ACK_EN OFFSET(3) NUMBITS(1) [],
        NEA OFFSET(2) NUMBITS(1) [],
        MS OFFSET(1) NUMBITS(1) [],
        RW OFFSET(0) NUMBITS(1) [],
    ],
    pub ControlRegW [
        DIVISOR_A OFFSET(14) NUMBITS(2) [],
        DIVISOR_B OFFSET(8) NUMBITS(6) [],
        CLR_FIFO OFFSET(6) NUMBITS(1) [],
        SLVMON OFFSET(5) NUMBITS(1) [],
        HOLD OFFSET(4) NUMBITS(1) [],
        ACK_EN OFFSET(3) NUMBITS(1) [],
        NEA OFFSET(2) NUMBITS(1) [],
        MS OFFSET(1) NUMBITS(1) [],
        RW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub StatusReg [
        RESERVED0 OFFSET(9) NUMBITS(7) [],
        BA OFFSET(8) NUMBITS(1) [],
        RXOVF OFFSET(7) NUMBITS(1) [],
        TXDV OFFSET(6) NUMBITS(1) [],
        RXDV OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        RXRW OFFSET(3) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u16,
    pub I2cAddressR [
        RESERVED0 OFFSET(10) NUMBITS(6) [],
        ADD OFFSET(0) NUMBITS(10) [],
    ],
    pub I2cAddressW [
        ADD OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u16,
    pub I2cDataR [
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DATA OFFSET(0) NUMBITS(8) [],
    ],
    pub I2cDataW [
        DATA OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u16,
    pub InterruptStatus [
        RESERVED0 OFFSET(10) NUMBITS(6) [],
        ARB_LOST OFFSET(9) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        RX_UNF OFFSET(7) NUMBITS(1) [],
        TX_OVF OFFSET(6) NUMBITS(1) [],
        RX_OVF OFFSET(5) NUMBITS(1) [],
        SLV_RDY OFFSET(4) NUMBITS(1) [],
        TO OFFSET(3) NUMBITS(1) [],
        NACK OFFSET(2) NUMBITS(1) [],
        DATA OFFSET(1) NUMBITS(1) [],
        COMP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub TransferSize [
        TRANSFER_SIZE OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u8,
    pub SlaveMonPauseR [
        RESERVED0 OFFSET(4) NUMBITS(4) [],
        PAUSE OFFSET(0) NUMBITS(4) [],
    ],
    pub SlaveMonPauseW [
        PAUSE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u8,
    pub TimeOut [
        TO OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u16,
    pub IntrptMask [
        RESERVED0 OFFSET(10) NUMBITS(6) [],
        ARB_LOST OFFSET(9) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        RX_UNF OFFSET(7) NUMBITS(1) [],
        TX_OVF OFFSET(6) NUMBITS(1) [],
        RX_OVF OFFSET(5) NUMBITS(1) [],
        SLV_RDY OFFSET(4) NUMBITS(1) [],
        TO OFFSET(3) NUMBITS(1) [],
        NACK OFFSET(2) NUMBITS(1) [],
        DATA OFFSET(1) NUMBITS(1) [],
        COMP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub IntrptEnableR [
        RESERVED0 OFFSET(10) NUMBITS(6) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
    ],
    pub IntrptEnableW [
        ARB_LOST OFFSET(9) NUMBITS(1) [],
        RX_UNF OFFSET(7) NUMBITS(1) [],
        TX_OVF OFFSET(6) NUMBITS(1) [],
        RX_OVF OFFSET(5) NUMBITS(1) [],
        SLV_RDY OFFSET(4) NUMBITS(1) [],
        TO OFFSET(3) NUMBITS(1) [],
        NACK OFFSET(2) NUMBITS(1) [],
        DATA OFFSET(1) NUMBITS(1) [],
        COMP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub IntrptDisableR [
        RESERVED0 OFFSET(10) NUMBITS(6) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
    ],
    pub IntrptDisableW [
        ARB_LOST OFFSET(9) NUMBITS(1) [],
        RX_UNF OFFSET(7) NUMBITS(1) [],
        TX_OVF OFFSET(6) NUMBITS(1) [],
        RX_OVF OFFSET(5) NUMBITS(1) [],
        SLV_RDY OFFSET(4) NUMBITS(1) [],
        TO OFFSET(3) NUMBITS(1) [],
        NACK OFFSET(2) NUMBITS(1) [],
        DATA OFFSET(1) NUMBITS(1) [],
        COMP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub GlitchFilterR [
        RESERVED0 OFFSET(4) NUMBITS(12) [],
        GF OFFSET(0) NUMBITS(4) [],
    ],
    pub GlitchFilterW [
        GF OFFSET(0) NUMBITS(4) [],
    ]
];

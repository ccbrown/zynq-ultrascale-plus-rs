// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// Serial Peripheral Interface Controller, SPI 0 Controller
pub static mut SPI0: *mut Registers = 0xff040000 as *mut Registers;
/// Serial Peripheral Interface Controller, SPI 1 Controller
pub static mut SPI1: *mut Registers = 0xff050000 as *mut Registers;
register_structs! {
    pub Registers {
        /// SPI configuration
        (0x00000000 => pub config: Aliased<u32, ConfigR::Register, ConfigW::Register>),
        /// SPI interrupt status
        (0x00000004 => pub isr: Aliased<u32, IsrR::Register, IsrW::Register>),
        /// Interrupt Enable
        (0x00000008 => pub ier: Aliased<u32, IerR::Register, IerW::Register>),
        /// Interrupt disable
        (0x0000000c => pub idr: Aliased<u32, IdrR::Register, IdrW::Register>),
        /// Interrupt mask
        (0x00000010 => pub imr: ReadOnly<u32, Imr::Register>),
        /// SPI_Enable
        (0x00000014 => pub enable: Aliased<u32, EnableR::Register, EnableW::Register>),
        /// Clock Delay
        (0x00000018 => pub delay: ReadWrite<u32, Delay::Register>),
        /// Transmit Data.
        (0x0000001c => pub tx_data: WriteOnly<u32>),
        /// Receive Data
        (0x00000020 => pub rx_data: ReadOnly<u32>),
        /// Slave Idle Count
        (0x00000024 => pub slave_idle_count: Aliased<u32, SlaveIdleCountR::Register, SlaveIdleCountW::Register>),
        /// TX FIFO Threshold
        (0x00000028 => pub tx_thres: ReadWrite<u32>),
        /// RX FIFO Threshold
        (0x0000002c => pub rx_thres: ReadWrite<u32>),
        (0x00000030 => _padding48),
        /// Module ID
        (0x000000fc => pub mod_id: ReadOnly<u32, ModId::Register>),
        (0x00000100 => @END),
    }
}
register_bitfields! [
    u32,
    pub ConfigR [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        MODEFAIL_GEN_EN OFFSET(17) NUMBITS(1) [],
        MAN_START_EN OFFSET(15) NUMBITS(1) [],
        MANUAL_CS OFFSET(14) NUMBITS(1) [],
        CS OFFSET(10) NUMBITS(4) [],
        PERI_SEL OFFSET(9) NUMBITS(1) [],
        REF_CLK OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(2) [],
        BAUD_RATE_DIV OFFSET(3) NUMBITS(3) [],
        CLK_PH OFFSET(2) NUMBITS(1) [],
        CLK_POL OFFSET(1) NUMBITS(1) [],
        MODE_SEL OFFSET(0) NUMBITS(1) [],
    ],
    pub ConfigW [
        MODEFAIL_GEN_EN OFFSET(17) NUMBITS(1) [],
        MAN_START_COM OFFSET(16) NUMBITS(1) [],
        MAN_START_EN OFFSET(15) NUMBITS(1) [],
        MANUAL_CS OFFSET(14) NUMBITS(1) [],
        CS OFFSET(10) NUMBITS(4) [],
        PERI_SEL OFFSET(9) NUMBITS(1) [],
        REF_CLK OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(2) [],
        BAUD_RATE_DIV OFFSET(3) NUMBITS(3) [],
        CLK_PH OFFSET(2) NUMBITS(1) [],
        CLK_POL OFFSET(1) NUMBITS(1) [],
        MODE_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IsrR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        MODE_FAIL OFFSET(1) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ],
    pub IsrW [
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        MODE_FAIL OFFSET(1) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IerR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
    ],
    pub IerW [
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        MODE_FAIL OFFSET(1) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IdrR [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
    ],
    pub IdrW [
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        MODE_FAIL OFFSET(1) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Imr [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        TX_FIFO_UNDERFLOW OFFSET(6) NUMBITS(1) [],
        RX_FIFO_FULL OFFSET(5) NUMBITS(1) [],
        RX_FIFO_NOT_EMPTY OFFSET(4) NUMBITS(1) [],
        TX_FIFO_FULL OFFSET(3) NUMBITS(1) [],
        TX_FIFO_NOT_FULL OFFSET(2) NUMBITS(1) [],
        MODE_FAIL OFFSET(1) NUMBITS(1) [],
        RX_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EnableR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        SPI_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub EnableW [
        SPI_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Delay [
        D_NSS OFFSET(24) NUMBITS(8) [],
        D_BTWN OFFSET(16) NUMBITS(8) [],
        D_AFTER OFFSET(8) NUMBITS(8) [],
        D_INT OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub SlaveIdleCountR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        SLAVE_IDLE_COUN OFFSET(0) NUMBITS(8) [],
    ],
    pub SlaveIdleCountW [
        SLAVE_IDLE_COUN OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub ModId [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        MODULE_ID OFFSET(0) NUMBITS(25) [],
    ]
];

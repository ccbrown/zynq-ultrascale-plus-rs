// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// Controller Area Network Controller, CAN 0 Controller
pub static mut CAN0: *mut Registers = 0xff060000 as *mut Registers;
/// Controller Area Network Controller, CAN 1 Controller
pub static mut CAN1: *mut Registers = 0xff070000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Software Reset and Enable
        (0x00000000 => pub srr: ReadWrite<u32, Srr::Register>),
        /// Mode Select
        (0x00000004 => pub msr: ReadWrite<u32, Msr::Register>),
        /// Baud Rate Prescaler
        (0x00000008 => pub brpr: ReadWrite<u32, Brpr::Register>),
        /// Bit Timing and Synchronization
        (0x0000000c => pub btr: ReadWrite<u32, Btr::Register>),
        /// Rx and Tx Error Counters
        (0x00000010 => pub ecr: ReadOnly<u32, Ecr::Register>),
        /// Error Status
        (0x00000014 => pub esr: ReadWrite<u32, Esr::Register>),
        /// Controller Status
        (0x00000018 => pub sr: Aliased<u32, SrR::Register, SrW::Register>),
        /// Interrupt Status
        (0x0000001c => pub isr: Aliased<u32, IsrR::Register, IsrW::Register>),
        /// Interrupt Enable
        (0x00000020 => pub ier: ReadWrite<u32, Ier::Register>),
        /// Interrupt Clear
        (0x00000024 => pub icr: Aliased<u32, IcrR::Register, IcrW::Register>),
        /// Timestamp Clear.
        (0x00000028 => pub tcr: Aliased<u32, TcrR::Register, TcrW::Register>),
        /// Rx and Tx Watermark Settings.
        (0x0000002c => pub wir: ReadWrite<u32, Wir::Register>),
        /// Tx Message FIFO, Identifier, Request.
        (0x00000030 => pub txfifo_id: WriteOnly<u32, TxfifoId::Register>),
        /// Tx Message FIFO Data Length Code.
        (0x00000034 => pub txfifo_dlc: WriteOnly<u32, TxfifoDlc::Register>),
        /// Tx Message FIFO, data word 1.
        (0x00000038 => pub txfifo_data1: WriteOnly<u32, TxfifoData1::Register>),
        /// Tx Message FIFO, data word 2.
        (0x0000003c => pub txfifo_data2: WriteOnly<u32, TxfifoData2::Register>),
        /// High Priority Tx Message FIFO, Identifier, Request.
        (0x00000040 => pub txhpb_id: WriteOnly<u32, TxhpbId::Register>),
        /// High Priority Tx Message FIFO Data Length Code.
        (0x00000044 => pub txhpb_dlc: WriteOnly<u32, TxhpbDlc::Register>),
        /// High Priority Tx Message FIFO, data word 1.
        (0x00000048 => pub txhpb_data1: WriteOnly<u32, TxhpbData1::Register>),
        /// High Priority Tx Message FIFO, data word 0.
        (0x0000004c => pub txhpb_data2: WriteOnly<u32, TxhpbData2::Register>),
        /// Rx Message FIFO, Identifier, Request.
        (0x00000050 => pub rxfifo_id: ReadOnly<u32, RxfifoId::Register>),
        /// Rx Message FIFO Data Length Code.
        (0x00000054 => pub rxfifo_dlc: ReadWrite<u32, RxfifoDlc::Register>),
        /// Rx Message FIFO, data word 1.
        (0x00000058 => pub rxfifo_data1: ReadWrite<u32, RxfifoData1::Register>),
        /// Rx Message FIFO, data word 2.
        (0x0000005c => pub rxfifo_data2: ReadWrite<u32, RxfifoData2::Register>),
        /// Acceptance Filter Enables.
        (0x00000060 => pub afr: ReadWrite<u32, Afr::Register>),
        /// Acceptance Filter 1 Mask.
        (0x00000064 => pub afmr1: ReadWrite<u32, Afmr1::Register>),
        /// Acceptance Filter 1 ID.
        (0x00000068 => pub afir1: ReadWrite<u32, Afir1::Register>),
        /// Acceptance Filter 2 Mask.
        (0x0000006c => pub afmr2: ReadWrite<u32, Afmr2::Register>),
        /// Acceptance Filter 2 ID.
        (0x00000070 => pub afir2: ReadWrite<u32, Afir2::Register>),
        /// Acceptance Filter 3 Mask.
        (0x00000074 => pub afmr3: ReadWrite<u32, Afmr3::Register>),
        /// Acceptance Filter 3 ID.
        (0x00000078 => pub afir3: ReadWrite<u32, Afir3::Register>),
        /// Acceptance Filter 4 Mask.
        (0x0000007c => pub afmr4: ReadWrite<u32, Afmr4::Register>),
        /// Acceptance Filter 4 ID.
        (0x00000080 => pub afir4: ReadWrite<u32, Afir4::Register>),
        (0x00000084 => @END),
    }
}
register_bitfields! [
    u32,
    pub Srr [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        CEN OFFSET(1) NUMBITS(1) [],
        SRST OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Msr [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        SNOOP OFFSET(2) NUMBITS(1) [],
        LBACK OFFSET(1) NUMBITS(1) [],
        SLEEP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Brpr [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        BRP OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Btr [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        SJW OFFSET(7) NUMBITS(2) [],
        TS2 OFFSET(4) NUMBITS(3) [],
        TS1 OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ecr [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        REC OFFSET(8) NUMBITS(8) [],
        TEC OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Esr [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ACKER OFFSET(4) NUMBITS(1) [],
        BERR OFFSET(3) NUMBITS(1) [],
        STER OFFSET(2) NUMBITS(1) [],
        FMER OFFSET(1) NUMBITS(1) [],
        CRCER OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub SrR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        SNOOP OFFSET(12) NUMBITS(1) [],
        ACFBSY OFFSET(11) NUMBITS(1) [],
        TXFLL OFFSET(10) NUMBITS(1) [],
        TXBFLL OFFSET(9) NUMBITS(1) [],
        ESTAT OFFSET(7) NUMBITS(2) [],
        ERRWRN OFFSET(6) NUMBITS(1) [],
        BBSY OFFSET(5) NUMBITS(1) [],
        BIDLE OFFSET(4) NUMBITS(1) [],
        NORMAL OFFSET(3) NUMBITS(1) [],
        SLEEP OFFSET(2) NUMBITS(1) [],
        LBACK OFFSET(1) NUMBITS(1) [],
        CONFIG OFFSET(0) NUMBITS(1) [],
    ],
    pub SrW [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
    ]
];
register_bitfields! [
    u32,
    pub IsrR [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        TXFEMP OFFSET(14) NUMBITS(1) [],
        TXFWMEMP OFFSET(13) NUMBITS(1) [],
        RXFWMFLL OFFSET(12) NUMBITS(1) [],
        WKUP OFFSET(11) NUMBITS(1) [],
        SLP OFFSET(10) NUMBITS(1) [],
        BSOFF OFFSET(9) NUMBITS(1) [],
        ERROR OFFSET(8) NUMBITS(1) [],
        RXNEMP OFFSET(7) NUMBITS(1) [],
        RXOFLW OFFSET(6) NUMBITS(1) [],
        RXUFLW OFFSET(5) NUMBITS(1) [],
        RXOK OFFSET(4) NUMBITS(1) [],
        TXBFLL OFFSET(3) NUMBITS(1) [],
        TXFLL OFFSET(2) NUMBITS(1) [],
        TXOK OFFSET(1) NUMBITS(1) [],
        ARBLST OFFSET(0) NUMBITS(1) [],
    ],
    pub IsrW [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
    ]
];
register_bitfields! [
    u32,
    pub Ier [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        ETXFEMP OFFSET(14) NUMBITS(1) [],
        ETXFWMEMP OFFSET(13) NUMBITS(1) [],
        ERXFWMFLL OFFSET(12) NUMBITS(1) [],
        EWKUP OFFSET(11) NUMBITS(1) [],
        ESLP OFFSET(10) NUMBITS(1) [],
        EBSOFF OFFSET(9) NUMBITS(1) [],
        EERROR OFFSET(8) NUMBITS(1) [],
        ERXNEMP OFFSET(7) NUMBITS(1) [],
        ERXOFLW OFFSET(6) NUMBITS(1) [],
        ERXUFLW OFFSET(5) NUMBITS(1) [],
        ERXOK OFFSET(4) NUMBITS(1) [],
        ETXBFLL OFFSET(3) NUMBITS(1) [],
        ETXFLL OFFSET(2) NUMBITS(1) [],
        ETXOK OFFSET(1) NUMBITS(1) [],
        EARBLST OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IcrR [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
    ],
    pub IcrW [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        CTXFEMP OFFSET(14) NUMBITS(1) [],
        CTXFWMEMP OFFSET(13) NUMBITS(1) [],
        CRXFWMFLL OFFSET(12) NUMBITS(1) [],
        CWKUP OFFSET(11) NUMBITS(1) [],
        CSLP OFFSET(10) NUMBITS(1) [],
        CBSOFF OFFSET(9) NUMBITS(1) [],
        CERROR OFFSET(8) NUMBITS(1) [],
        CRXNEMP OFFSET(7) NUMBITS(1) [],
        CRXOFLW OFFSET(6) NUMBITS(1) [],
        CRXUFLW OFFSET(5) NUMBITS(1) [],
        CRXOK OFFSET(4) NUMBITS(1) [],
        CTXBFLL OFFSET(3) NUMBITS(1) [],
        CTXFLL OFFSET(2) NUMBITS(1) [],
        CTXOK OFFSET(1) NUMBITS(1) [],
        CARBLST OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TcrR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub TcrW [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        CTS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Wir [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        EW OFFSET(8) NUMBITS(8) [],
        FW OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TxfifoId [
        IDH OFFSET(21) NUMBITS(11) [],
        SRRRTR OFFSET(20) NUMBITS(1) [],
        IDE OFFSET(19) NUMBITS(1) [],
        IDL OFFSET(1) NUMBITS(18) [],
        RTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TxfifoDlc [
        DLC OFFSET(28) NUMBITS(4) [],
        RESERVED0 OFFSET(0) NUMBITS(28) [],
    ]
];
register_bitfields! [
    u32,
    pub TxfifoData1 [
        DB0 OFFSET(24) NUMBITS(8) [],
        DB1 OFFSET(16) NUMBITS(8) [],
        DB2 OFFSET(8) NUMBITS(8) [],
        DB3 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TxfifoData2 [
        DB4 OFFSET(24) NUMBITS(8) [],
        DB5 OFFSET(16) NUMBITS(8) [],
        DB6 OFFSET(8) NUMBITS(8) [],
        DB7 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TxhpbId [
        IDH OFFSET(21) NUMBITS(11) [],
        SRRRTR OFFSET(20) NUMBITS(1) [],
        IDE OFFSET(19) NUMBITS(1) [],
        IDL OFFSET(1) NUMBITS(18) [],
        RTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TxhpbDlc [
        DLC OFFSET(28) NUMBITS(4) [],
        RESERVED0 OFFSET(0) NUMBITS(28) [],
    ]
];
register_bitfields! [
    u32,
    pub TxhpbData1 [
        DB0 OFFSET(24) NUMBITS(8) [],
        DB1 OFFSET(16) NUMBITS(8) [],
        DB2 OFFSET(8) NUMBITS(8) [],
        DB3 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TxhpbData2 [
        DB4 OFFSET(24) NUMBITS(8) [],
        DB5 OFFSET(16) NUMBITS(8) [],
        DB6 OFFSET(8) NUMBITS(8) [],
        DB7 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub RxfifoId [
        IDH OFFSET(21) NUMBITS(11) [],
        SRRRTR OFFSET(20) NUMBITS(1) [],
        IDE OFFSET(19) NUMBITS(1) [],
        IDL OFFSET(1) NUMBITS(18) [],
        RTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub RxfifoDlc [
        DLC OFFSET(28) NUMBITS(4) [],
        RESERVED0 OFFSET(16) NUMBITS(12) [],
        RXT OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub RxfifoData1 [
        DB0 OFFSET(24) NUMBITS(8) [],
        DB1 OFFSET(16) NUMBITS(8) [],
        DB2 OFFSET(8) NUMBITS(8) [],
        DB3 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub RxfifoData2 [
        DB4 OFFSET(24) NUMBITS(8) [],
        DB5 OFFSET(16) NUMBITS(8) [],
        DB6 OFFSET(8) NUMBITS(8) [],
        DB7 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Afr [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        UAF4 OFFSET(3) NUMBITS(1) [],
        UAF3 OFFSET(2) NUMBITS(1) [],
        UAF2 OFFSET(1) NUMBITS(1) [],
        UAF1 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Afmr1 [
        AMIDH OFFSET(21) NUMBITS(11) [],
        AMSRR OFFSET(20) NUMBITS(1) [],
        AMIDE OFFSET(19) NUMBITS(1) [],
        AMIDL OFFSET(1) NUMBITS(18) [],
        AMRTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Afir1 [
        AIIDH OFFSET(21) NUMBITS(11) [],
        AISRR OFFSET(20) NUMBITS(1) [],
        AIIDE OFFSET(19) NUMBITS(1) [],
        AIIDL OFFSET(1) NUMBITS(18) [],
        AIRTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Afmr2 [
        AMIDH OFFSET(21) NUMBITS(11) [],
        AMSRR OFFSET(20) NUMBITS(1) [],
        AMIDE OFFSET(19) NUMBITS(1) [],
        AMIDL OFFSET(1) NUMBITS(18) [],
        AMRTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Afir2 [
        AIIDH OFFSET(21) NUMBITS(11) [],
        AISRR OFFSET(20) NUMBITS(1) [],
        AIIDE OFFSET(19) NUMBITS(1) [],
        AIIDL OFFSET(1) NUMBITS(18) [],
        AIRTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Afmr3 [
        AMIDH OFFSET(21) NUMBITS(11) [],
        AMSRR OFFSET(20) NUMBITS(1) [],
        AMIDE OFFSET(19) NUMBITS(1) [],
        AMIDL OFFSET(1) NUMBITS(18) [],
        AMRTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Afir3 [
        AIIDH OFFSET(21) NUMBITS(11) [],
        AISRR OFFSET(20) NUMBITS(1) [],
        AIIDE OFFSET(19) NUMBITS(1) [],
        AIIDL OFFSET(1) NUMBITS(18) [],
        AIRTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Afmr4 [
        AMIDH OFFSET(21) NUMBITS(11) [],
        AMSRR OFFSET(20) NUMBITS(1) [],
        AMIDE OFFSET(19) NUMBITS(1) [],
        AMIDL OFFSET(1) NUMBITS(18) [],
        AMRTR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Afir4 [
        AIIDH OFFSET(21) NUMBITS(11) [],
        AISRR OFFSET(20) NUMBITS(1) [],
        AIIDE OFFSET(19) NUMBITS(1) [],
        AIIDL OFFSET(1) NUMBITS(18) [],
        AIRTR OFFSET(0) NUMBITS(1) [],
    ]
];

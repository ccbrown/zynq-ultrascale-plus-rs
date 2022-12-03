// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// UART Controller, UART 0 Controller
pub static mut UART0: *mut Registers = 0xff000000 as *mut Registers;
/// UART Controller, UART 1 Controller
pub static mut UART1: *mut Registers = 0xff010000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// UART Control Register
    pub control: Aliased<u32, ControlR::Register, ControlW::Register>,
    /// UART Mode Register
    pub mode: Aliased<u32, ModeR::Register, ModeW::Register>,
    /// Interrupt Enable Register
    pub intrpt_en: Aliased<u32, IntrptEnR::Register, IntrptEnW::Register>,
    /// Interrupt Disable Register
    pub intrpt_dis: Aliased<u32, IntrptDisR::Register, IntrptDisW::Register>,
    /// Interrupt Mask Register
    pub intrpt_mask: Aliased<u32, IntrptMaskR::Register, IntrptMaskW::Register>,
    /// Channel Interrupt Status Register
    pub chnl_int_sts: ReadWrite<u32, ChnlIntSts::Register>,
    /// Baud Rate Generator Register.
    pub baud_rate_gen: Aliased<u32, BaudRateGenR::Register, BaudRateGenW::Register>,
    /// Receiver Timeout Register
    pub rcvr_timeout: Aliased<u32, RcvrTimeoutR::Register, RcvrTimeoutW::Register>,
    /// Receiver FIFO Trigger Level Register
    pub rcvr_fifo_trigger_level:
        Aliased<u32, RcvrFifoTriggerLevelR::Register, RcvrFifoTriggerLevelW::Register>,
    /// Modem Control Register
    pub modem_ctrl: Aliased<u32, ModemCtrlR::Register, ModemCtrlW::Register>,
    /// Modem Status Register
    pub modem_sts: Aliased<u32, ModemStsR::Register, ModemStsW::Register>,
    /// Channel Status Register
    pub channel_sts: ReadOnly<u32, ChannelSts::Register>,
    /// Transmit and Receive FIFO
    pub tx_rx_fifo: Aliased<u32, TxRxFifoR::Register, TxRxFifoW::Register>,
    /// Baud Rate Divider Register
    pub baud_rate_divider: Aliased<u32, BaudRateDividerR::Register, BaudRateDividerW::Register>,
    /// Flow Control Delay Register
    pub flow_delay: Aliased<u32, FlowDelayR::Register, FlowDelayW::Register>,
    _padding60: [u8; 8],
    /// Transmitter FIFO Trigger Level Register
    pub tx_fifo_trigger_level:
        Aliased<u32, TxFifoTriggerLevelR::Register, TxFifoTriggerLevelW::Register>,
    /// RX FIFO Byte Status Register
    pub rx_fifo_byte_status: Aliased<u32, RxFifoByteStatusR::Register, RxFifoByteStatusW::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub ControlR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        STPBRK OFFSET(8) NUMBITS(1) [],
        STTBRK OFFSET(7) NUMBITS(1) [],
        RSTTO OFFSET(6) NUMBITS(1) [],
        TXDIS OFFSET(5) NUMBITS(1) [],
        TXEN OFFSET(4) NUMBITS(1) [],
        RXDIS OFFSET(3) NUMBITS(1) [],
        RXEN OFFSET(2) NUMBITS(1) [],
        TXRES OFFSET(1) NUMBITS(1) [],
        RXRES OFFSET(0) NUMBITS(1) [],
    ],
    pub ControlW [
        STPBRK OFFSET(8) NUMBITS(1) [],
        STTBRK OFFSET(7) NUMBITS(1) [],
        RSTTO OFFSET(6) NUMBITS(1) [],
        TXDIS OFFSET(5) NUMBITS(1) [],
        TXEN OFFSET(4) NUMBITS(1) [],
        RXDIS OFFSET(3) NUMBITS(1) [],
        RXEN OFFSET(2) NUMBITS(1) [],
        TXRES OFFSET(1) NUMBITS(1) [],
        RXRES OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ModeR [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        WSIZE OFFSET(12) NUMBITS(2) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(1) [],
        CHMODE OFFSET(8) NUMBITS(2) [],
        NBSTOP OFFSET(6) NUMBITS(2) [],
        PAR OFFSET(3) NUMBITS(3) [],
        CHRL OFFSET(1) NUMBITS(2) [],
        CLKS OFFSET(0) NUMBITS(1) [],
    ],
    pub ModeW [
        WSIZE OFFSET(12) NUMBITS(2) [],
        RESERVED0 OFFSET(11) NUMBITS(1) [],
        RESERVED1 OFFSET(10) NUMBITS(1) [],
        CHMODE OFFSET(8) NUMBITS(2) [],
        NBSTOP OFFSET(6) NUMBITS(2) [],
        PAR OFFSET(3) NUMBITS(3) [],
        CHRL OFFSET(1) NUMBITS(2) [],
        CLKS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntrptEnR [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
    ],
    pub IntrptEnW [
        RBRK OFFSET(13) NUMBITS(1) [],
        TOVR OFFSET(12) NUMBITS(1) [],
        TNFUL OFFSET(11) NUMBITS(1) [],
        TTRIG OFFSET(10) NUMBITS(1) [],
        DMSI OFFSET(9) NUMBITS(1) [],
        TIMEOUT OFFSET(8) NUMBITS(1) [],
        PARE OFFSET(7) NUMBITS(1) [],
        FRAME OFFSET(6) NUMBITS(1) [],
        ROVR OFFSET(5) NUMBITS(1) [],
        TFUL OFFSET(4) NUMBITS(1) [],
        TEMPTY OFFSET(3) NUMBITS(1) [],
        RFUL OFFSET(2) NUMBITS(1) [],
        REMPTY OFFSET(1) NUMBITS(1) [],
        RTRIG OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntrptDisR [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
    ],
    pub IntrptDisW [
        RBRK OFFSET(13) NUMBITS(1) [],
        TOVR OFFSET(12) NUMBITS(1) [],
        TNFUL OFFSET(11) NUMBITS(1) [],
        TTRIG OFFSET(10) NUMBITS(1) [],
        DMSI OFFSET(9) NUMBITS(1) [],
        TIMEOUT OFFSET(8) NUMBITS(1) [],
        PARE OFFSET(7) NUMBITS(1) [],
        FRAME OFFSET(6) NUMBITS(1) [],
        ROVR OFFSET(5) NUMBITS(1) [],
        TFUL OFFSET(4) NUMBITS(1) [],
        TEMPTY OFFSET(3) NUMBITS(1) [],
        RFUL OFFSET(2) NUMBITS(1) [],
        REMPTY OFFSET(1) NUMBITS(1) [],
        RTRIG OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntrptMaskR [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        TOVR OFFSET(12) NUMBITS(1) [],
        TNFUL OFFSET(11) NUMBITS(1) [],
        TTRIG OFFSET(10) NUMBITS(1) [],
        DMSI OFFSET(9) NUMBITS(1) [],
        TIMEOUT OFFSET(8) NUMBITS(1) [],
        PARE OFFSET(7) NUMBITS(1) [],
        FRAME OFFSET(6) NUMBITS(1) [],
        ROVR OFFSET(5) NUMBITS(1) [],
        TFUL OFFSET(4) NUMBITS(1) [],
        TEMPTY OFFSET(3) NUMBITS(1) [],
        RFUL OFFSET(2) NUMBITS(1) [],
        REMPTY OFFSET(1) NUMBITS(1) [],
        RTRIG OFFSET(0) NUMBITS(1) [],
    ],
    pub IntrptMaskW [
        RBRK OFFSET(13) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ChnlIntSts [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        RBRK OFFSET(13) NUMBITS(1) [],
        TOVR OFFSET(12) NUMBITS(1) [],
        TNFUL OFFSET(11) NUMBITS(1) [],
        TTRIG OFFSET(10) NUMBITS(1) [],
        DMSI OFFSET(9) NUMBITS(1) [],
        TIMEOUT OFFSET(8) NUMBITS(1) [],
        PARE OFFSET(7) NUMBITS(1) [],
        FRAME OFFSET(6) NUMBITS(1) [],
        ROVR OFFSET(5) NUMBITS(1) [],
        TFUL OFFSET(4) NUMBITS(1) [],
        TEMPTY OFFSET(3) NUMBITS(1) [],
        RFUL OFFSET(2) NUMBITS(1) [],
        REMPTY OFFSET(1) NUMBITS(1) [],
        RTRIG OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BaudRateGenR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CD OFFSET(0) NUMBITS(16) [],
    ],
    pub BaudRateGenW [
        CD OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RcvrTimeoutR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RTO OFFSET(0) NUMBITS(8) [],
    ],
    pub RcvrTimeoutW [
        RTO OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RcvrFifoTriggerLevelR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        RTRIG OFFSET(0) NUMBITS(6) [],
    ],
    pub RcvrFifoTriggerLevelW [
        RTRIG OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ModemCtrlR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        FCM OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(3) [],
        RTS OFFSET(1) NUMBITS(1) [],
        DTR OFFSET(0) NUMBITS(1) [],
    ],
    pub ModemCtrlW [
        FCM OFFSET(5) NUMBITS(1) [],
        RTS OFFSET(1) NUMBITS(1) [],
        DTR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ModemStsR [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        FCMS OFFSET(8) NUMBITS(1) [],
        DCD OFFSET(7) NUMBITS(1) [],
        RI OFFSET(6) NUMBITS(1) [],
        DSR OFFSET(5) NUMBITS(1) [],
        CTS OFFSET(4) NUMBITS(1) [],
        DDCD OFFSET(3) NUMBITS(1) [],
        TERI OFFSET(2) NUMBITS(1) [],
        DDSR OFFSET(1) NUMBITS(1) [],
        DCTS OFFSET(0) NUMBITS(1) [],
    ],
    pub ModemStsW [
        FCMS OFFSET(8) NUMBITS(1) [],
        DDCD OFFSET(3) NUMBITS(1) [],
        TERI OFFSET(2) NUMBITS(1) [],
        DDSR OFFSET(1) NUMBITS(1) [],
        DCTS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ChannelSts [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        TNFUL OFFSET(14) NUMBITS(1) [],
        TTRIG OFFSET(13) NUMBITS(1) [],
        FDELT OFFSET(12) NUMBITS(1) [],
        TACTIVE OFFSET(11) NUMBITS(1) [],
        RACTIVE OFFSET(10) NUMBITS(1) [],
        RESERVED1 OFFSET(5) NUMBITS(5) [],
        TFUL OFFSET(4) NUMBITS(1) [],
        TEMPTY OFFSET(3) NUMBITS(1) [],
        RFUL OFFSET(2) NUMBITS(1) [],
        REMPTY OFFSET(1) NUMBITS(1) [],
        RTRIG OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxRxFifoR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        FIFO OFFSET(0) NUMBITS(8) [],
    ],
    pub TxRxFifoW [
        FIFO OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub BaudRateDividerR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        BDIV OFFSET(0) NUMBITS(8) [],
    ],
    pub BaudRateDividerW [
        BDIV OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub FlowDelayR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        FDEL OFFSET(0) NUMBITS(6) [],
    ],
    pub FlowDelayW [
        FDEL OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TxFifoTriggerLevelR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        TTRIG OFFSET(0) NUMBITS(6) [],
    ],
    pub TxFifoTriggerLevelW [
        TTRIG OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RxFifoByteStatusR [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        BYTE3_BREAK OFFSET(11) NUMBITS(1) [],
        BYTE3_FRM_ERR OFFSET(10) NUMBITS(1) [],
        BYTE3_PAR_ERR OFFSET(9) NUMBITS(1) [],
        BYTE2_BREAK OFFSET(8) NUMBITS(1) [],
        BYTE2_FRM_ERR OFFSET(7) NUMBITS(1) [],
        BYTE2_PAR_ERR OFFSET(6) NUMBITS(1) [],
        BYTE1_BREAK OFFSET(5) NUMBITS(1) [],
        BYTE1_FRM_ERR OFFSET(4) NUMBITS(1) [],
        BYTE1_PAR_ERR OFFSET(3) NUMBITS(1) [],
        BYTE0_BREAK OFFSET(2) NUMBITS(1) [],
        BYTE0_FRM_ERR OFFSET(1) NUMBITS(1) [],
        BYTE0_PAR_ERR OFFSET(0) NUMBITS(1) [],
    ],
    pub RxFifoByteStatusW [
        BYTE3_BREAK OFFSET(11) NUMBITS(1) [],
        BYTE3_FRM_ERR OFFSET(10) NUMBITS(1) [],
        BYTE3_PAR_ERR OFFSET(9) NUMBITS(1) [],
        BYTE2_BREAK OFFSET(8) NUMBITS(1) [],
        BYTE2_FRM_ERR OFFSET(7) NUMBITS(1) [],
        BYTE2_PAR_ERR OFFSET(6) NUMBITS(1) [],
        BYTE1_BREAK OFFSET(5) NUMBITS(1) [],
        BYTE1_FRM_ERR OFFSET(4) NUMBITS(1) [],
        BYTE1_PAR_ERR OFFSET(3) NUMBITS(1) [],
        BYTE0_BREAK OFFSET(2) NUMBITS(1) [],
        BYTE0_FRM_ERR OFFSET(1) NUMBITS(1) [],
        BYTE0_PAR_ERR OFFSET(0) NUMBITS(1) [],
    ]
];

// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// AXI PCIe Bridge - DMA Channel, PCIe Bridge - DMA Channel 0
pub static mut AXIPCIE_DMA0: *mut Registers = 0xfd0f0000 as *mut Registers;
/// AXI PCIe Bridge - DMA Channel, PCIe Bridge - DMA Channel 1
pub static mut AXIPCIE_DMA1: *mut Registers = 0xfd0f0080 as *mut Registers;
/// AXI PCIe Bridge - DMA Channel, PCIe Bridge - DMA Channel 2
pub static mut AXIPCIE_DMA2: *mut Registers = 0xfd0f0100 as *mut Registers;
/// AXI PCIe Bridge - DMA Channel, PCIe Bridge - DMA Channel 3
pub static mut AXIPCIE_DMA3: *mut Registers = 0xfd0f0180 as *mut Registers;
register_structs! {
    pub Registers {
        /// Queue Base Address Low
        (0x00000000 => pub dma_channel_src_q_ptr_lo: ReadWrite<u32, DmaChannelSrcQPtrLo::Register>),
        /// Queue Base Address High
        (0x00000004 => pub dma_channel_src_q_ptr_hi: ReadWrite<u32>),
        /// Queue Size
        (0x00000008 => pub dma_channel_src_q_size: ReadWrite<u32>),
        /// Queue Limit Pointer
        (0x0000000c => pub dma_channel_src_q_limit: ReadWrite<u32>),
        /// Queue Base Address Low
        (0x00000010 => pub dma_channel_dst_q_ptr_lo: ReadWrite<u32, DmaChannelDstQPtrLo::Register>),
        /// Queue Base Address High
        (0x00000014 => pub dma_channel_dst_q_ptr_hi: ReadWrite<u32>),
        /// Queue Size
        (0x00000018 => pub dma_channel_dst_q_size: ReadWrite<u32>),
        /// Queue Limit Pointer
        (0x0000001c => pub dma_channel_dst_q_limit: ReadWrite<u32>),
        /// Queue Base Address Low
        (0x00000020 => pub dma_channel_stas_q_ptr_lo: ReadWrite<u32, DmaChannelStasQPtrLo::Register>),
        /// Queue Base Address High
        (0x00000024 => pub dma_channel_stas_q_ptr_hi: ReadWrite<u32>),
        /// Queue Size
        (0x00000028 => pub dma_channel_stas_q_size: ReadWrite<u32>),
        /// Queue Limit Pointer
        (0x0000002c => pub dma_channel_stas_q_limit: ReadWrite<u32>),
        /// Queue Base Address Low
        (0x00000030 => pub dma_channel_stad_q_ptr_lo: ReadWrite<u32, DmaChannelStadQPtrLo::Register>),
        /// Queue Base Address High
        (0x00000034 => pub dma_channel_stad_q_ptr_hi: ReadWrite<u32>),
        /// Queue Size
        (0x00000038 => pub dma_channel_stad_q_size: ReadWrite<u32>),
        /// Queue Limit Pointer
        (0x0000003c => pub dma_channel_stad_q_limit: ReadWrite<u32>),
        /// Queue Next Pointer
        (0x00000040 => pub dma_channel_src_q_next: ReadWrite<u32>),
        /// Queue Next Pointer
        (0x00000044 => pub dma_channel_dst_q_next: ReadWrite<u32>),
        /// Queue Next Pointer
        (0x00000048 => pub dma_channel_stas_q_next: ReadWrite<u32>),
        /// Write only to initialize DMA Channel
        (0x0000004c => pub dma_channel_stad_q_next: ReadWrite<u32>),
        /// Scratchpad Register
        (0x00000050 => pub dma_channel_scratch0: ReadWrite<u32>),
        /// Scratchpad Register
        (0x00000054 => pub dma_channel_scratch1: ReadWrite<u32>),
        /// Scratchpad Register
        (0x00000058 => pub dma_channel_scratch2: ReadWrite<u32>),
        /// Scratchpad Register
        (0x0000005c => pub dma_channel_scratch3: ReadWrite<u32>),
        /// PCI Express Interrupt Control
        (0x00000060 => pub dma_channel_pcie_interrupt_control: Aliased<u32, DmaChannelPcieInterruptControlR::Register, DmaChannelPcieInterruptControlW::Register>),
        /// PCIe Interrupt Status
        (0x00000064 => pub dma_channel_pcie_interrupt_status: Aliased<u32, DmaChannelPcieInterruptStatusR::Register, DmaChannelPcieInterruptStatusW::Register>),
        /// PCI Express Interrupt Control
        (0x00000068 => pub dma_channel_axi_interrupt_control: Aliased<u32, DmaChannelAxiInterruptControlR::Register, DmaChannelAxiInterruptControlW::Register>),
        /// AXI Interrupt Status
        (0x0000006c => pub dma_channel_axi_interrupt_status: Aliased<u32, DmaChannelAxiInterruptStatusR::Register, DmaChannelAxiInterruptStatusW::Register>),
        /// PCIe Interrupt Assertion.
        (0x00000070 => pub dma_channel_pcie_interrupt_assert: Aliased<u32, DmaChannelPcieInterruptAssertR::Register, DmaChannelPcieInterruptAssertW::Register>),
        /// AXI Interrupt Assertion.
        (0x00000074 => pub dma_channel_axi_interrupt_assert: Aliased<u32, DmaChannelAxiInterruptAssertR::Register, DmaChannelAxiInterruptAssertW::Register>),
        /// DMA Channel Control
        (0x00000078 => pub dma_channel_dma_control: Aliased<u32, DmaChannelDmaControlR::Register, DmaChannelDmaControlW::Register>),
        /// DMA Channel Status
        (0x0000007c => pub dma_channel_dma_status: ReadOnly<u32, DmaChannelDmaStatus::Register>),
        (0x00000080 => @END),
    }
}
register_bitfields! [
    u32,
    pub DmaChannelSrcQPtrLo [
        START_ADDR_LO OFFSET(6) NUMBITS(26) [],
        READ_ATTR OFFSET(2) NUMBITS(4) [],
        QUEUE_ENABLE OFFSET(1) NUMBITS(1) [],
        QUEUE_LOCATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelDstQPtrLo [
        START_ADDR_LO OFFSET(6) NUMBITS(26) [],
        READ_ATTR OFFSET(2) NUMBITS(4) [],
        QUEUE_ENABLE OFFSET(1) NUMBITS(1) [],
        QUEUE_LOCATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelStasQPtrLo [
        START_ADDR_LO OFFSET(6) NUMBITS(26) [],
        READ_ATTR OFFSET(2) NUMBITS(4) [],
        QUEUE_ENABLE OFFSET(1) NUMBITS(1) [],
        QUEUE_LOCATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelStadQPtrLo [
        START_ADDR_LO OFFSET(6) NUMBITS(26) [],
        READ_ATTR OFFSET(2) NUMBITS(4) [],
        QUEUE_ENABLE OFFSET(1) NUMBITS(1) [],
        QUEUE_LOCATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelPcieInterruptControlR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        COALESCE_COUNT OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(3) NUMBITS(13) [],
        SGL_INT_ENABLE OFFSET(2) NUMBITS(1) [],
        DMA_ERR_INT_ENABLE OFFSET(1) NUMBITS(1) [],
        INTERRUPT_MASK OFFSET(0) NUMBITS(1) [],
    ],
    pub DmaChannelPcieInterruptControlW [
        COALESCE_COUNT OFFSET(16) NUMBITS(8) [],
        SGL_INT_ENABLE OFFSET(2) NUMBITS(1) [],
        DMA_ERR_INT_ENABLE OFFSET(1) NUMBITS(1) [],
        INTERRUPT_MASK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelPcieInterruptStatusR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        SOFTWARE_INT OFFSET(3) NUMBITS(1) [],
        DMA_SGL_INT OFFSET(2) NUMBITS(1) [],
        DMA_ERROR_INT OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ],
    pub DmaChannelPcieInterruptStatusW [
        SOFTWARE_INT OFFSET(3) NUMBITS(1) [],
        DMA_SGL_INT OFFSET(2) NUMBITS(1) [],
        DMA_ERROR_INT OFFSET(1) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelAxiInterruptControlR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        COALESCE_COUNT OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(3) NUMBITS(13) [],
        SGL_INT_ENABLE OFFSET(2) NUMBITS(1) [],
        DMA_ERR_INT_ENABLE OFFSET(1) NUMBITS(1) [],
        INTERRUPT_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub DmaChannelAxiInterruptControlW [
        COALESCE_COUNT OFFSET(16) NUMBITS(8) [],
        SGL_INT_ENABLE OFFSET(2) NUMBITS(1) [],
        DMA_ERR_INT_ENABLE OFFSET(1) NUMBITS(1) [],
        INTERRUPT_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelAxiInterruptStatusR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        SOFTWARE_INT OFFSET(3) NUMBITS(1) [],
        DMA_SGL_INT OFFSET(2) NUMBITS(1) [],
        DMA_ERROR_INT OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ],
    pub DmaChannelAxiInterruptStatusW [
        SOFTWARE_INT OFFSET(3) NUMBITS(1) [],
        DMA_SGL_INT OFFSET(2) NUMBITS(1) [],
        DMA_ERROR_INT OFFSET(1) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelPcieInterruptAssertR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PCIE_SOFTWARE_INTERRUPT OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(3) [],
    ],
    pub DmaChannelPcieInterruptAssertW [
        PCIE_SOFTWARE_INTERRUPT OFFSET(3) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelAxiInterruptAssertR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        AXI_SOFTWARE_INTERRUPT OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(3) [],
    ],
    pub DmaChannelAxiInterruptAssertW [
        AXI_SOFTWARE_INTERRUPT OFFSET(3) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelDmaControlR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        CMPL_STAT_Q_ELEM_SIZE OFFSET(2) NUMBITS(1) [],
        DMA_RESET OFFSET(1) NUMBITS(1) [],
        DMA_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub DmaChannelDmaControlW [
        CMPL_STAT_Q_ELEM_SIZE OFFSET(2) NUMBITS(1) [],
        DMA_RESET OFFSET(1) NUMBITS(1) [],
        DMA_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DmaChannelDmaStatus [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CHANNEL_PRESENT OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(1) [],
        CHANNEL_NUMBER OFFSET(4) NUMBITS(10) [],
        RESERVED2 OFFSET(1) NUMBITS(3) [],
        DMA_RUNNING OFFSET(0) NUMBITS(1) [],
    ]
];

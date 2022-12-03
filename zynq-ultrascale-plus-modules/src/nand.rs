// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// NAND ONFI Control, NAND ONFI Control
pub static mut NAND: *mut Registers = 0xff100000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Packet Configuration.
    pub packet: Aliased<u32, PacketR::Register, PacketW::Register>,
    /// Memory Address, reg 1.
    pub memory_address_register1: ReadWrite<u32>,
    /// Memory Address, reg 2.
    pub memory_address_register2:
        Aliased<u32, MemoryAddressRegister2R::Register, MemoryAddressRegister2W::Register>,
    /// Command and Configuration.
    pub command: Aliased<u32, CommandR::Register, CommandW::Register>,
    /// Initiate Controller Operations.
    pub program: Aliased<u32, ProgramR::Register, ProgramW::Register>,
    /// Interrupt Status Enable.
    pub interrupt_status_enable:
        Aliased<u32, InterruptStatusEnableR::Register, InterruptStatusEnableW::Register>,
    /// Interrupt Signal Enable.
    pub interrupt_signal_enable:
        Aliased<u32, InterruptSignalEnableR::Register, InterruptSignalEnableW::Register>,
    /// Interrupt Status.
    pub interrupt_status: Aliased<u32, InterruptStatusR::Register, InterruptStatusW::Register>,
    /// Ready Busy Status.
    pub ready_busy: ReadOnly<u32, ReadyBusy::Register>,
    /// DMA System Address, reg 1.
    pub dma_system_address1: ReadWrite<u32>,
    /// Flash Memory Status.
    pub flash_status: ReadOnly<u32, FlashStatus::Register>,
    /// Interface Timing Control.
    pub timing: Aliased<u32, TimingR::Register, TimingW::Register>,
    /// Buffer Data Port.
    pub buffer_data_port: ReadWrite<u32>,
    /// ECC Configuration.
    pub ecc: Aliased<u32, EccR::Register, EccW::Register>,
    /// ECC Error Count
    pub ecc_error_count: ReadOnly<u32, EccErrorCount::Register>,
    /// ECC Spare Command
    pub ecc_spare_command: Aliased<u32, EccSpareCommandR::Register, EccSpareCommandW::Register>,
    /// Count of 1-bit Errors
    pub error_count_1bit: ReadWrite<u32>,
    /// Count of 2-bit Errors
    pub error_count_2bit: ReadWrite<u32>,
    /// Count of 3-bit Errors
    pub error_count_3bit: ReadWrite<u32>,
    /// Count of 4-bit Errors
    pub error_count_4bit: ReadWrite<u32>,
    /// DMA System Address, reg2.
    pub dma_system_address0: ReadWrite<u32>,
    /// DMA Buffer Boundary.
    pub dma_buffer_boundary:
        Aliased<u32, DmaBufferBoundaryR::Register, DmaBufferBoundaryW::Register>,
    /// CPU Release after Transferring Primary Boot Code.
    pub cpu_release: Aliased<u32, CpuReleaseR::Register, CpuReleaseW::Register>,
    /// Count of 5-bit Errors
    pub error_count_5bit: ReadWrite<u32>,
    /// Count of 6-bit Errors
    pub error_count_6bit: ReadWrite<u32>,
    /// Count of 7-bit Errors
    pub error_count_7bit: ReadWrite<u32>,
    /// Count of 8-bit Errors
    pub error_count_8bit: ReadWrite<u32>,
    /// Data Interface Configuration
    pub data_interface: Aliased<u32, DataInterfaceR::Register, DataInterfaceW::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub PacketR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        PACKET_COUNT OFFSET(12) NUMBITS(12) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        PACKET_SIZE OFFSET(0) NUMBITS(11) [],
    ],
    pub PacketW [
        PACKET_COUNT OFFSET(12) NUMBITS(12) [],
        PACKET_SIZE OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MemoryAddressRegister2R [
        CHIP_SELECT OFFSET(30) NUMBITS(2) [],
        RESERVED0 OFFSET(28) NUMBITS(2) [],
        NFC_BCH_MODE OFFSET(25) NUMBITS(3) [],
        RESERVED1 OFFSET(24) NUMBITS(1) [],
        RESERVED2 OFFSET(8) NUMBITS(16) [],
        MEMORY_ADDRESS OFFSET(0) NUMBITS(8) [],
    ],
    pub MemoryAddressRegister2W [
        CHIP_SELECT OFFSET(30) NUMBITS(2) [],
        RESERVED0 OFFSET(28) NUMBITS(2) [],
        NFC_BCH_MODE OFFSET(25) NUMBITS(3) [],
        RESERVED1 OFFSET(24) NUMBITS(1) [],
        MEMORY_ADDRESS OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CommandR [
        ECC_ON_OFF OFFSET(31) NUMBITS(1) [],
        NUMBER_OF_ADDRESS_CYCELS OFFSET(28) NUMBITS(3) [],
        DMA_ENABLE OFFSET(26) NUMBITS(2) [],
        PAGE_SIZE OFFSET(23) NUMBITS(3) [],
        RESERVED0 OFFSET(16) NUMBITS(7) [],
        COMMAND2 OFFSET(8) NUMBITS(8) [],
        COMMAND1 OFFSET(0) NUMBITS(8) [],
    ],
    pub CommandW [
        ECC_ON_OFF OFFSET(31) NUMBITS(1) [],
        NUMBER_OF_ADDRESS_CYCELS OFFSET(28) NUMBITS(3) [],
        DMA_ENABLE OFFSET(26) NUMBITS(2) [],
        PAGE_SIZE OFFSET(23) NUMBITS(3) [],
        COMMAND2 OFFSET(8) NUMBITS(8) [],
        COMMAND1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ProgramR [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ODT_CONFIGURE OFFSET(26) NUMBITS(1) [],
        VOLUME_SELECT OFFSET(25) NUMBITS(1) [],
        PGM_PG_REG_CLR OFFSET(24) NUMBITS(1) [],
        RESET_LUN OFFSET(23) NUMBITS(1) [],
        CHANGE_ROW_ADDR_END OFFSET(22) NUMBITS(1) [],
        CHANGE_ROW_ADDR OFFSET(21) NUMBITS(1) [],
        SMALL_DATA_MOVE OFFSET(20) NUMBITS(1) [],
        READ_CACHE_END OFFSET(19) NUMBITS(1) [],
        READ_CACHE_RANDOM OFFSET(18) NUMBITS(1) [],
        READ_CACHE_SEQUENTIAL OFFSET(17) NUMBITS(1) [],
        READ_CACHE_START OFFSET(16) NUMBITS(1) [],
        COPY_BACK_INTERLEAVED OFFSET(15) NUMBITS(1) [],
        CHANGE_READ_COLUMN_ENHANCED OFFSET(14) NUMBITS(1) [],
        READ_INTERLEAVED OFFSET(13) NUMBITS(1) [],
        READ_STATUS_ENHANCED OFFSET(12) NUMBITS(1) [],
        READ_UNIQUE_ID OFFSET(11) NUMBITS(1) [],
        SET_FEATURES OFFSET(10) NUMBITS(1) [],
        GET_FEATURES OFFSET(9) NUMBITS(1) [],
        RESET OFFSET(8) NUMBITS(1) [],
        READ_PARAMETER_PAGE OFFSET(7) NUMBITS(1) [],
        READ_ID OFFSET(6) NUMBITS(1) [],
        MULTI_DIE_RD OFFSET(5) NUMBITS(1) [],
        PAGE_PROGRAM OFFSET(4) NUMBITS(1) [],
        READ_STATUS OFFSET(3) NUMBITS(1) [],
        BLOCK_ERASE OFFSET(2) NUMBITS(1) [],
        MULTI_DIE OFFSET(1) NUMBITS(1) [],
        READ OFFSET(0) NUMBITS(1) [],
    ],
    pub ProgramW [
        ODT_CONFIGURE OFFSET(26) NUMBITS(1) [],
        VOLUME_SELECT OFFSET(25) NUMBITS(1) [],
        PGM_PG_REG_CLR OFFSET(24) NUMBITS(1) [],
        RESET_LUN OFFSET(23) NUMBITS(1) [],
        CHANGE_ROW_ADDR_END OFFSET(22) NUMBITS(1) [],
        CHANGE_ROW_ADDR OFFSET(21) NUMBITS(1) [],
        SMALL_DATA_MOVE OFFSET(20) NUMBITS(1) [],
        READ_CACHE_END OFFSET(19) NUMBITS(1) [],
        READ_CACHE_RANDOM OFFSET(18) NUMBITS(1) [],
        READ_CACHE_SEQUENTIAL OFFSET(17) NUMBITS(1) [],
        READ_CACHE_START OFFSET(16) NUMBITS(1) [],
        COPY_BACK_INTERLEAVED OFFSET(15) NUMBITS(1) [],
        CHANGE_READ_COLUMN_ENHANCED OFFSET(14) NUMBITS(1) [],
        READ_INTERLEAVED OFFSET(13) NUMBITS(1) [],
        READ_STATUS_ENHANCED OFFSET(12) NUMBITS(1) [],
        READ_UNIQUE_ID OFFSET(11) NUMBITS(1) [],
        SET_FEATURES OFFSET(10) NUMBITS(1) [],
        GET_FEATURES OFFSET(9) NUMBITS(1) [],
        RESET OFFSET(8) NUMBITS(1) [],
        READ_PARAMETER_PAGE OFFSET(7) NUMBITS(1) [],
        READ_ID OFFSET(6) NUMBITS(1) [],
        MULTI_DIE_RD OFFSET(5) NUMBITS(1) [],
        PAGE_PROGRAM OFFSET(4) NUMBITS(1) [],
        READ_STATUS OFFSET(3) NUMBITS(1) [],
        BLOCK_ERASE OFFSET(2) NUMBITS(1) [],
        MULTI_DIE OFFSET(1) NUMBITS(1) [],
        READ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub InterruptStatusEnableR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ERROR_AHB_STS_EN OFFSET(7) NUMBITS(1) [],
        DMA_INT_STS_EN OFFSET(6) NUMBITS(1) [],
        ECC_ERR_INTRPT_STS_EN OFFSET(5) NUMBITS(1) [],
        ERR_INTRPT_STS_EN OFFSET(4) NUMBITS(1) [],
        MUL_BIT_ERR_STS_EN OFFSET(3) NUMBITS(1) [],
        TRANS_COMP_STS_EN OFFSET(2) NUMBITS(1) [],
        BUFF_RD_RDY_STS_EN OFFSET(1) NUMBITS(1) [],
        BUFF_WR_RDY_STS_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub InterruptStatusEnableW [
        ERROR_AHB_STS_EN OFFSET(7) NUMBITS(1) [],
        DMA_INT_STS_EN OFFSET(6) NUMBITS(1) [],
        ECC_ERR_INTRPT_STS_EN OFFSET(5) NUMBITS(1) [],
        ERR_INTRPT_STS_EN OFFSET(4) NUMBITS(1) [],
        MUL_BIT_ERR_STS_EN OFFSET(3) NUMBITS(1) [],
        TRANS_COMP_STS_EN OFFSET(2) NUMBITS(1) [],
        BUFF_RD_RDY_STS_EN OFFSET(1) NUMBITS(1) [],
        BUFF_WR_RDY_STS_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub InterruptSignalEnableR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ERROR_AHB_SIG_EN OFFSET(7) NUMBITS(1) [],
        DMA_INT_SIG_EN OFFSET(6) NUMBITS(1) [],
        ECC_ERR_INTRPT_SIG_EN OFFSET(5) NUMBITS(1) [],
        ERR_INTRPT_SIG_EN OFFSET(4) NUMBITS(1) [],
        MUL_BIT_ERR_SIG_EN OFFSET(3) NUMBITS(1) [],
        TRANS_COMP_SIG_EN OFFSET(2) NUMBITS(1) [],
        BUFF_RD_RDY_SIG_EN OFFSET(1) NUMBITS(1) [],
        BUFF_WR_RDY_SIG_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub InterruptSignalEnableW [
        ERROR_AHB_SIG_EN OFFSET(7) NUMBITS(1) [],
        DMA_INT_SIG_EN OFFSET(6) NUMBITS(1) [],
        ECC_ERR_INTRPT_SIG_EN OFFSET(5) NUMBITS(1) [],
        ERR_INTRPT_SIG_EN OFFSET(4) NUMBITS(1) [],
        MUL_BIT_ERR_SIG_EN OFFSET(3) NUMBITS(1) [],
        TRANS_COMP_SIG_EN OFFSET(2) NUMBITS(1) [],
        BUFF_RD_RDY_SIG_EN OFFSET(1) NUMBITS(1) [],
        BUFF_WR_RDY_SIG_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub InterruptStatusR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ERROR_AHB_REG OFFSET(7) NUMBITS(1) [],
        DMA_INT_REG OFFSET(6) NUMBITS(1) [],
        ECC_ERR_INTRPT_REG OFFSET(5) NUMBITS(1) [],
        ERR_INTRPT_REG OFFSET(4) NUMBITS(1) [],
        MUL_BIT_ERR_REG OFFSET(3) NUMBITS(1) [],
        TRANS_COMP_REG OFFSET(2) NUMBITS(1) [],
        BUFF_RD_RDY_REG OFFSET(1) NUMBITS(1) [],
        BUFF_WR_RDY_REG OFFSET(0) NUMBITS(1) [],
    ],
    pub InterruptStatusW [
        ERROR_AHB_REG OFFSET(7) NUMBITS(1) [],
        DMA_INT_REG OFFSET(6) NUMBITS(1) [],
        ECC_ERR_INTRPT_REG OFFSET(5) NUMBITS(1) [],
        ERR_INTRPT_REG OFFSET(4) NUMBITS(1) [],
        MUL_BIT_ERR_REG OFFSET(3) NUMBITS(1) [],
        TRANS_COMP_REG OFFSET(2) NUMBITS(1) [],
        BUFF_RD_RDY_REG OFFSET(1) NUMBITS(1) [],
        BUFF_WR_RDY_REG OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReadyBusy [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        RB_N1_STATUS OFFSET(1) NUMBITS(1) [],
        RB_N0_STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub FlashStatus [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        FLASH_STATUS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TimingR [
        RESERVED0 OFFSET(19) NUMBITS(13) [],
        DQS_BUFF_SEL_OUT OFFSET(15) NUMBITS(4) [],
        TADL_TIME OFFSET(7) NUMBITS(8) [],
        DQS_BUFF_SEL_IN OFFSET(3) NUMBITS(4) [],
        SLOW_FAST_TCAD OFFSET(2) NUMBITS(1) [],
        TCCS_TIME OFFSET(0) NUMBITS(2) [],
    ],
    pub TimingW [
        DQS_BUFF_SEL_OUT OFFSET(15) NUMBITS(4) [],
        TADL_TIME OFFSET(7) NUMBITS(8) [],
        DQS_BUFF_SEL_IN OFFSET(3) NUMBITS(4) [],
        SLOW_FAST_TCAD OFFSET(2) NUMBITS(1) [],
        TCCS_TIME OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EccR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        SLC_MLC OFFSET(27) NUMBITS(1) [],
        ECC_SIZE OFFSET(16) NUMBITS(11) [],
        ECC_ADDR OFFSET(0) NUMBITS(16) [],
    ],
    pub EccW [
        SLC_MLC OFFSET(27) NUMBITS(1) [],
        ECC_SIZE OFFSET(16) NUMBITS(11) [],
        ECC_ADDR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EccErrorCount [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        PAGE_BOUND_ERR_COUNT OFFSET(8) NUMBITS(9) [],
        PACKET_BOUND_ERR_COUNT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EccSpareCommandR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        NUMBER_OF_ECC_AND_SPARE_ADDRESS_CYCLES OFFSET(28) NUMBITS(3) [],
        RESERVED1 OFFSET(16) NUMBITS(12) [],
        ECC_SPARE_CMD OFFSET(0) NUMBITS(16) [],
    ],
    pub EccSpareCommandW [
        NUMBER_OF_ECC_AND_SPARE_ADDRESS_CYCLES OFFSET(28) NUMBITS(3) [],
        ECC_SPARE_CMD OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DmaBufferBoundaryR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        DMA_BOUND_INT_EN OFFSET(3) NUMBITS(1) [],
        DMA_BUFFER_BOUNDARY_REGISTER OFFSET(0) NUMBITS(3) [],
    ],
    pub DmaBufferBoundaryW [
        DMA_BOUND_INT_EN OFFSET(3) NUMBITS(1) [],
        DMA_BUFFER_BOUNDARY_REGISTER OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CpuReleaseR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        RELEASE_RESET_TO_CPU OFFSET(0) NUMBITS(1) [],
    ],
    pub CpuReleaseW [
        RELEASE_RESET_TO_CPU OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DataInterfaceR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        DATA_INTF OFFSET(9) NUMBITS(2) [],
        RESERVED1 OFFSET(6) NUMBITS(3) [],
        NVDDR OFFSET(3) NUMBITS(3) [],
        SDR OFFSET(0) NUMBITS(3) [],
    ],
    pub DataInterfaceW [
        DATA_INTF OFFSET(9) NUMBITS(2) [],
        RESERVED0 OFFSET(6) NUMBITS(3) [],
        NVDDR OFFSET(3) NUMBITS(3) [],
        SDR OFFSET(0) NUMBITS(3) [],
    ]
];

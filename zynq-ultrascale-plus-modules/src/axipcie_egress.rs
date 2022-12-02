// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// PCIe Bridge - Egress Addr Translation, PCIe Bridge - Egress Addr Translation 0
pub static mut AXIPCIE_EGRESS0: *mut Registers = 0xfd0e0c00 as *mut Registers;
/// PCIe Bridge - Egress Addr Translation, PCIe Bridge - Egress Addr Translation 1
pub static mut AXIPCIE_EGRESS1: *mut Registers = 0xfd0e0c20 as *mut Registers;
/// PCIe Bridge - Egress Addr Translation, PCIe Bridge - Egress Addr Translation 2
pub static mut AXIPCIE_EGRESS2: *mut Registers = 0xfd0e0c40 as *mut Registers;
/// PCIe Bridge - Egress Addr Translation, PCIe Bridge - Egress Addr Translation 3
pub static mut AXIPCIE_EGRESS3: *mut Registers = 0xfd0e0c60 as *mut Registers;
/// PCIe Bridge - Egress Addr Translation, PCIe Bridge - Egress Addr Translation 4
pub static mut AXIPCIE_EGRESS4: *mut Registers = 0xfd0e0c80 as *mut Registers;
/// PCIe Bridge - Egress Addr Translation, PCIe Bridge - Egress Addr Translation 5
pub static mut AXIPCIE_EGRESS5: *mut Registers = 0xfd0e0ca0 as *mut Registers;
/// PCIe Bridge - Egress Addr Translation, PCIe Bridge - Egress Addr Translation 6
pub static mut AXIPCIE_EGRESS6: *mut Registers = 0xfd0e0cc0 as *mut Registers;
/// PCIe Bridge - Egress Addr Translation, PCIe Bridge - Egress Addr Translation 7
pub static mut AXIPCIE_EGRESS7: *mut Registers = 0xfd0e0ce0 as *mut Registers;
register_structs! {
    pub Registers {
        /// Egress AXI Translation - Capabilities
        (0x00000000 => pub tran_egress_capabilities: ReadOnly<u32, TranEgressCapabilities::Register>),
        /// Egress AXI Translation - Status
        (0x00000004 => pub tran_egress_status: ReadOnly<u32, TranEgressStatus::Register>),
        /// Egress AXI Translation - Control
        (0x00000008 => pub tran_egress_control: Aliased<u32, TranEgressControlR::Register, TranEgressControlW::Register>),
        (0x0000000c => _padding12),
        /// Egress AXI Translation - Source Address Low
        (0x00000010 => pub tran_egress_src_base_lo: Aliased<u32, TranEgressSrcBaseLoR::Register, TranEgressSrcBaseLoW::Register>),
        /// Egress AXI Translation - Source Address High
        (0x00000014 => pub tran_egress_src_base_hi: ReadWrite<u32>),
        /// Egress AXI Translation - Destination Address Low
        (0x00000018 => pub tran_egress_dst_base_lo: Aliased<u32, TranEgressDstBaseLoR::Register, TranEgressDstBaseLoW::Register>),
        /// Egress AXI Translation - Destination Address High
        (0x0000001c => pub tran_egress_dst_base_hi: ReadWrite<u32>),
        (0x00000020 => @END),
    }
}
register_bitfields! [
    u32,
    pub TranEgressCapabilities [
        EGRESS_SIZE_MAX OFFSET(24) NUMBITS(8) [],
        EGRESS_SIZE_OFFSET OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        EGRESS_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TranEgressStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        WR_PENDING_CTR OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        RD_PENDING_CTR OFFSET(0) NUMBITS(9) [],
    ]
];
register_bitfields! [
    u32,
    pub TranEgressControlR [
        EGRESS_ATTR_W OFFSET(28) NUMBITS(4) [],
        EGRESS_ATTR_R OFFSET(24) NUMBITS(4) [],
        EGRESS_ATTR_ENABLE OFFSET(23) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(2) [],
        EGRESS_SIZE OFFSET(16) NUMBITS(5) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(5) NUMBITS(3) [],
        RESERVED4 OFFSET(4) NUMBITS(1) [],
        EGRESS_INVALID OFFSET(3) NUMBITS(1) [],
        EGRESS_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        RESERVED5 OFFSET(1) NUMBITS(1) [],
        EGRESS_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub TranEgressControlW [
        EGRESS_ATTR_W OFFSET(28) NUMBITS(4) [],
        EGRESS_ATTR_R OFFSET(24) NUMBITS(4) [],
        EGRESS_ATTR_ENABLE OFFSET(23) NUMBITS(1) [],
        EGRESS_SIZE OFFSET(16) NUMBITS(5) [],
        EGRESS_INVALID OFFSET(3) NUMBITS(1) [],
        EGRESS_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        EGRESS_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TranEgressSrcBaseLoR [
        EGRESS_SRC_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub TranEgressSrcBaseLoW [
        EGRESS_SRC_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];
register_bitfields! [
    u32,
    pub TranEgressDstBaseLoR [
        EGRESS_DST_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub TranEgressDstBaseLoW [
        EGRESS_DST_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];

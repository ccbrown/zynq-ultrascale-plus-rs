// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// PCIe Bridge - Ingress Addr Translation, PCIe Bridge - Ingress Addr Translation 0
pub static mut AXIPCIE_INGRESS0: *mut Registers = 0xfd0e0800 as *mut Registers;
/// PCIe Bridge - Ingress Addr Translation, PCIe Bridge - Ingress Addr Translation 1
pub static mut AXIPCIE_INGRESS1: *mut Registers = 0xfd0e0820 as *mut Registers;
/// PCIe Bridge - Ingress Addr Translation, PCIe Bridge - Ingress Addr Translation 2
pub static mut AXIPCIE_INGRESS2: *mut Registers = 0xfd0e0840 as *mut Registers;
/// PCIe Bridge - Ingress Addr Translation, PCIe Bridge - Ingress Addr Translation 3
pub static mut AXIPCIE_INGRESS3: *mut Registers = 0xfd0e0860 as *mut Registers;
/// PCIe Bridge - Ingress Addr Translation, PCIe Bridge - Ingress Addr Translation 4
pub static mut AXIPCIE_INGRESS4: *mut Registers = 0xfd0e0880 as *mut Registers;
/// PCIe Bridge - Ingress Addr Translation, PCIe Bridge - Ingress Addr Translation 5
pub static mut AXIPCIE_INGRESS5: *mut Registers = 0xfd0e08a0 as *mut Registers;
/// PCIe Bridge - Ingress Addr Translation, PCIe Bridge - Ingress Addr Translation 6
pub static mut AXIPCIE_INGRESS6: *mut Registers = 0xfd0e08c0 as *mut Registers;
/// PCIe Bridge - Ingress Addr Translation, PCIe Bridge - Ingress Addr Translation 7
pub static mut AXIPCIE_INGRESS7: *mut Registers = 0xfd0e08e0 as *mut Registers;
register_structs! {
    pub Registers {
        /// Ingress AXI Translation - Capabilities
        (0x00000000 => pub tran_ingress_capabilities: ReadOnly<u32, TranIngressCapabilities::Register>),
        /// Ingress AXI Translation - Status
        (0x00000004 => pub tran_ingress_status: ReadOnly<u32, TranIngressStatus::Register>),
        /// Ingress AXI Translation - Control
        (0x00000008 => pub tran_ingress_control: Aliased<u32, TranIngressControlR::Register, TranIngressControlW::Register>),
        (0x0000000c => _padding12),
        /// Ingress AXI Translation - Source Address Low
        (0x00000010 => pub tran_ingress_src_base_lo: Aliased<u32, TranIngressSrcBaseLoR::Register, TranIngressSrcBaseLoW::Register>),
        /// Ingress AXI Translation - Source Address High
        (0x00000014 => pub tran_ingress_src_base_hi: ReadWrite<u32>),
        /// Ingress AXI Translation - Destination Address Low
        (0x00000018 => pub tran_ingress_dst_base_lo: Aliased<u32, TranIngressDstBaseLoR::Register, TranIngressDstBaseLoW::Register>),
        /// Ingress AXI Translation - Destination Address High
        (0x0000001c => pub tran_ingress_dst_base_hi: ReadWrite<u32>),
        (0x00000020 => @END),
    }
}
register_bitfields! [
    u32,
    pub TranIngressCapabilities [
        INGRESS_SIZE_MAX OFFSET(24) NUMBITS(8) [],
        INGRESS_SIZE_OFFSET OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        INGRESS_PRESENT OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TranIngressStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        WR_PENDING_CTR OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        RD_PENDING_CTR OFFSET(0) NUMBITS(9) [],
    ]
];
register_bitfields! [
    u32,
    pub TranIngressControlR [
        INGRESS_ATTR_W OFFSET(28) NUMBITS(4) [],
        INGRESS_ATTR_R OFFSET(24) NUMBITS(4) [],
        INGRESS_ATTR_ENABLE OFFSET(23) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(2) [],
        INGRESS_SIZE OFFSET(16) NUMBITS(5) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(5) NUMBITS(3) [],
        RESERVED4 OFFSET(4) NUMBITS(1) [],
        INGRESS_INVALID OFFSET(3) NUMBITS(1) [],
        INGRESS_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        RESERVED5 OFFSET(1) NUMBITS(1) [],
        INGRESS_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub TranIngressControlW [
        INGRESS_ATTR_W OFFSET(28) NUMBITS(4) [],
        INGRESS_ATTR_R OFFSET(24) NUMBITS(4) [],
        INGRESS_ATTR_ENABLE OFFSET(23) NUMBITS(1) [],
        INGRESS_SIZE OFFSET(16) NUMBITS(5) [],
        INGRESS_INVALID OFFSET(3) NUMBITS(1) [],
        INGRESS_SECURITY_ENABLE OFFSET(2) NUMBITS(1) [],
        INGRESS_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TranIngressSrcBaseLoR [
        INGRESS_SRC_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub TranIngressSrcBaseLoW [
        INGRESS_SRC_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];
register_bitfields! [
    u32,
    pub TranIngressDstBaseLoR [
        INGRESS_DST_BASE_LO OFFSET(12) NUMBITS(20) [],
        RESERVED0 OFFSET(0) NUMBITS(12) [],
    ],
    pub TranIngressDstBaseLoW [
        INGRESS_DST_BASE_LO OFFSET(12) NUMBITS(20) [],
    ]
];

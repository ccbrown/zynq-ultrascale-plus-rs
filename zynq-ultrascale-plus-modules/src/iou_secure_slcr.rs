// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// IOP System-level Control - Secure, IOP System-level Control - Secure
pub static mut IOU_SECURE_SLCR: *mut Registers = 0xff240000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// AXI write protection type selection
    pub iou_axi_wprtcn: Aliased<u32, IouAxiWprtcnR::Register, IouAxiWprtcnW::Register>,
    /// AXI read protection type selection
    pub iou_axi_rprtcn: Aliased<u32, IouAxiRprtcnR::Register, IouAxiRprtcnW::Register>,
    _padding8: [u8; 56],
    /// General control register for the IOU SLCR
    pub ctrl: ReadWrite<u8, Ctrl::Register>,
    _padding65: [u8; 3],
    /// Interrupt Status Register
    pub isr: ReadWrite<u8, Isr::Register>,
    _padding69: [u8; 3],
    /// Interrupt Mask Register
    pub imr: ReadOnly<u8, Imr::Register>,
    _padding73: [u8; 3],
    /// Interrupt Enable Register
    pub ier: WriteOnly<u8, Ier::Register>,
    _padding77: [u8; 3],
    /// Interrupt Disable Register
    pub idr: WriteOnly<u8, Idr::Register>,
    _padding81: [u8; 3],
    /// Interrupt Trigger Register
    pub itr: WriteOnly<u8, Itr::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub IouAxiWprtcnR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        QSPI_AXI_AWPROT OFFSET(25) NUMBITS(3) [],
        NAND_AXI_AWPROT OFFSET(22) NUMBITS(3) [],
        SD1_AXI_AWPROT OFFSET(19) NUMBITS(3) [],
        SD0_AXI_AWPROT OFFSET(16) NUMBITS(3) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        GEM3_AXI_AWPROT OFFSET(9) NUMBITS(3) [],
        GEM2_AXI_AWPROT OFFSET(6) NUMBITS(3) [],
        GEM1_AXI_AWPROT OFFSET(3) NUMBITS(3) [],
        GEM0_AXI_AWPROT OFFSET(0) NUMBITS(3) [],
    ],
    pub IouAxiWprtcnW [
        QSPI_AXI_AWPROT OFFSET(25) NUMBITS(3) [],
        NAND_AXI_AWPROT OFFSET(22) NUMBITS(3) [],
        SD1_AXI_AWPROT OFFSET(19) NUMBITS(3) [],
        SD0_AXI_AWPROT OFFSET(16) NUMBITS(3) [],
        GEM3_AXI_AWPROT OFFSET(9) NUMBITS(3) [],
        GEM2_AXI_AWPROT OFFSET(6) NUMBITS(3) [],
        GEM1_AXI_AWPROT OFFSET(3) NUMBITS(3) [],
        GEM0_AXI_AWPROT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IouAxiRprtcnR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        NAND_AXI_ARPROT OFFSET(22) NUMBITS(3) [],
        SD1_AXI_ARPROT OFFSET(19) NUMBITS(3) [],
        SD0_AXI_ARPROT OFFSET(16) NUMBITS(3) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        GEM3_AXI_ARPROT OFFSET(9) NUMBITS(3) [],
        GEM2_AXI_ARPROT OFFSET(6) NUMBITS(3) [],
        GEM1_AXI_ARPROT OFFSET(3) NUMBITS(3) [],
        GEM0_AXI_ARPROT OFFSET(0) NUMBITS(3) [],
    ],
    pub IouAxiRprtcnW [
        NAND_AXI_ARPROT OFFSET(22) NUMBITS(3) [],
        SD1_AXI_ARPROT OFFSET(19) NUMBITS(3) [],
        SD0_AXI_ARPROT OFFSET(16) NUMBITS(3) [],
        GEM3_AXI_ARPROT OFFSET(9) NUMBITS(3) [],
        GEM2_AXI_ARPROT OFFSET(6) NUMBITS(3) [],
        GEM1_AXI_ARPROT OFFSET(3) NUMBITS(3) [],
        GEM0_AXI_ARPROT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Ctrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Isr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Imr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Ier [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Idr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Itr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];

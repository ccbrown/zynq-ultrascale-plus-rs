// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// IOP System-level Control - Secure, IOP System-level Control - Secure
pub static mut IOU_SECURE_SLCR: *mut Registers = 0xff240000 as *mut Registers;
register_structs! {
    pub Registers {
        /// AXI write protection type selection
        (0x00000000 => pub iou_axi_wprtcn: Aliased<u32, IouAxiWprtcnR::Register, IouAxiWprtcnW::Register>),
        /// AXI read protection type selection
        (0x00000004 => pub iou_axi_rprtcn: Aliased<u32, IouAxiRprtcnR::Register, IouAxiRprtcnW::Register>),
        (0x00000008 => _padding8),
        /// General control register for the IOU SLCR
        (0x00000040 => pub ctrl: ReadWrite<u8, Ctrl::Register>),
        (0x00000041 => _padding65),
        /// Interrupt Status Register
        (0x00000044 => pub isr: ReadWrite<u8, Isr::Register>),
        (0x00000045 => _padding69),
        /// Interrupt Mask Register
        (0x00000048 => pub imr: ReadOnly<u8, Imr::Register>),
        (0x00000049 => _padding73),
        /// Interrupt Enable Register
        (0x0000004c => pub ier: WriteOnly<u8, Ier::Register>),
        (0x0000004d => _padding77),
        /// Interrupt Disable Register
        (0x00000050 => pub idr: WriteOnly<u8, Idr::Register>),
        (0x00000051 => _padding81),
        /// Interrupt Trigger Register
        (0x00000054 => pub itr: WriteOnly<u8, Itr::Register>),
        (0x00000055 => @END),
    }
}
register_bitfields! [
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
register_bitfields! [
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
register_bitfields! [
    u8,
    pub Ctrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Isr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Imr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Ier [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Idr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub Itr [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];

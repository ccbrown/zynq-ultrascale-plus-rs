// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// Real-time Processing Unit (RPU), Real time Processing Unit
pub static mut RPU: *mut Registers = 0xff9a0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Global Control Regiter for RPU
    pub rpu_glbl_cntl: Aliased<u32, RpuGlblCntlR::Register, RpuGlblCntlW::Register>,
    /// Miscellaneous status information for RPU
    pub rpu_glbl_status: ReadOnly<u32, RpuGlblStatus::Register>,
    /// Error Response Enable/Disable Register
    pub rpu_err_cntl: Aliased<u32, RpuErrCntlR::Register, RpuErrCntlW::Register>,
    /// Control for extra features of RAMs
    pub rpu_ram: Aliased<u32, RpuRamR::Register, RpuRamW::Register>,
    _padding16: [u8; 16],
    /// Reserved for future use
    pub rpu_err_inj: Aliased<u32, RpuErrInjR::Register, RpuErrInjW::Register>,
    /// Common Cause Signal Mask Register
    pub rpu_ccf_mask: Aliased<u32, RpuCcfMaskR::Register, RpuCcfMaskW::Register>,
    /// RPU Interrupt Injection register
    pub rpu_intr_0: ReadWrite<u32>,
    /// RPU Interrupt Injection register
    pub rpu_intr_1: ReadWrite<u32>,
    /// RPU Interrupt Injection register
    pub rpu_intr_2: ReadWrite<u32>,
    /// RPU Interrupt Injection register
    pub rpu_intr_3: ReadWrite<u32>,
    /// RPU Interrupt Injection register
    pub rpu_intr_4: ReadWrite<u32>,
    _padding60: [u8; 4],
    /// RPU Interrupt Injection Mask register
    pub rpu_intr_mask_0: ReadWrite<u32>,
    /// RPU Interrupt Injection Mask register
    pub rpu_intr_mask_1: ReadWrite<u32>,
    /// RPU Interrupt Injection Mask register
    pub rpu_intr_mask_2: ReadWrite<u32>,
    /// RPU Interrupt Injection Mask register
    pub rpu_intr_mask_3: ReadWrite<u32>,
    /// RPU Interrupt Injection Mask register
    pub rpu_intr_mask_4: ReadWrite<u32>,
    /// Common Cause Signal Value Register
    pub rpu_ccf_val: Aliased<u32, RpuCcfValR::Register, RpuCcfValW::Register>,
    _padding88: [u8; 152],
    /// RPU Safety Check Register
    pub rpu_safety_chk: ReadWrite<u32>,
    /// Reserved for future use
    pub rpu: ReadWrite<u32>,
    _padding248: [u8; 8],
    /// Configuration Parameters specific to RPU0
    pub rpu0_cfg: Aliased<u32, Rpu0CfgR::Register, Rpu0CfgW::Register>,
    /// R5_0 Status Register
    pub rpu0_status: ReadOnly<u32, Rpu0Status::Register>,
    /// Power down request from R5s
    pub rpu0_pwrdwn: Aliased<u32, Rpu0PwrdwnR::Register, Rpu0PwrdwnW::Register>,
    _padding268: [u8; 8],
    /// Interrupt Status Register
    pub rpu0_isr: Aliased<u32, Rpu0IsrR::Register, Rpu0IsrW::Register>,
    /// Interrupt Mask Register
    pub rpu0_imr: ReadOnly<u32, Rpu0Imr::Register>,
    /// Interrupt Enable Register
    pub rpu0_ien: Aliased<u32, Rpu0IenR::Register, Rpu0IenW::Register>,
    /// Interrupt Disable Register
    pub rpu0_ids: Aliased<u32, Rpu0IdsR::Register, Rpu0IdsW::Register>,
    /// Slave Base Address Register
    pub rpu0_slv_base: Aliased<u32, Rpu0SlvBaseR::Register, Rpu0SlvBaseW::Register>,
    /// RPU 0 AXI Override Register
    pub rpu0_axi_over: Aliased<u32, Rpu0AxiOverR::Register, Rpu0AxiOverW::Register>,
    _padding300: [u8; 212],
    /// Configuration Parameters specific to RPU1
    pub rpu1_cfg: Aliased<u32, Rpu1CfgR::Register, Rpu1CfgW::Register>,
    /// R5_1 Status Register
    pub rpu1_status: ReadOnly<u32, Rpu1Status::Register>,
    /// Power down request from R5s
    pub rpu1_pwrdwn: Aliased<u32, Rpu1PwrdwnR::Register, Rpu1PwrdwnW::Register>,
    _padding524: [u8; 8],
    /// Interrupt Status Register
    pub rpu1_isr: Aliased<u32, Rpu1IsrR::Register, Rpu1IsrW::Register>,
    /// Interrupt Mask Register
    pub rpu1_imr: ReadOnly<u32, Rpu1Imr::Register>,
    /// Interrupt Enable Register
    pub rpu1_ien: Aliased<u32, Rpu1IenR::Register, Rpu1IenW::Register>,
    /// Interrupt Disable Register
    pub rpu1_ids: Aliased<u32, Rpu1IdsR::Register, Rpu1IdsW::Register>,
    /// Slave Base Address Register
    pub rpu1_slv_base: Aliased<u32, Rpu1SlvBaseR::Register, Rpu1SlvBaseW::Register>,
    /// RPU 1 AXI Override Register
    pub rpu1_axi_over: Aliased<u32, Rpu1AxiOverR::Register, Rpu1AxiOverW::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub RpuGlblCntlR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        GIC_AXPROT OFFSET(10) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(1) [],
        TCM_CLK_CNTL OFFSET(8) NUMBITS(1) [],
        TCM_WAIT OFFSET(7) NUMBITS(1) [],
        TCM_COMB OFFSET(6) NUMBITS(1) [],
        TEINIT OFFSET(5) NUMBITS(1) [],
        SLCLAMP OFFSET(4) NUMBITS(1) [],
        SLSPLIT OFFSET(3) NUMBITS(1) [],
        DBGNOCLKSTOP OFFSET(2) NUMBITS(1) [],
        CFGIE OFFSET(1) NUMBITS(1) [],
        CFGEE OFFSET(0) NUMBITS(1) [],
    ],
    pub RpuGlblCntlW [
        GIC_AXPROT OFFSET(10) NUMBITS(1) [],
        TCM_CLK_CNTL OFFSET(8) NUMBITS(1) [],
        TCM_WAIT OFFSET(7) NUMBITS(1) [],
        TCM_COMB OFFSET(6) NUMBITS(1) [],
        TEINIT OFFSET(5) NUMBITS(1) [],
        SLCLAMP OFFSET(4) NUMBITS(1) [],
        SLSPLIT OFFSET(3) NUMBITS(1) [],
        DBGNOCLKSTOP OFFSET(2) NUMBITS(1) [],
        CFGIE OFFSET(1) NUMBITS(1) [],
        CFGEE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RpuGlblStatus [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        DBGNOPWRDWN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RpuErrCntlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ],
    pub RpuErrCntlW [
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RpuRamR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RAMCONTROL1 OFFSET(8) NUMBITS(8) [],
        RAMCONTROL0 OFFSET(0) NUMBITS(8) [],
    ],
    pub RpuRamW [
        RAMCONTROL1 OFFSET(8) NUMBITS(8) [],
        RAMCONTROL0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RpuErrInjR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        DCCMINP2 OFFSET(8) NUMBITS(8) [],
        DCCMINP OFFSET(0) NUMBITS(8) [],
    ],
    pub RpuErrInjW [
        DCCMINP2 OFFSET(8) NUMBITS(8) [],
        DCCMINP OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RpuCcfMaskR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        TEST_MBIST_MODE OFFSET(7) NUMBITS(1) [],
        TEST_SCAN_MODE_LP OFFSET(6) NUMBITS(1) [],
        TEST_SCAN_MODE OFFSET(5) NUMBITS(1) [],
        ISO OFFSET(4) NUMBITS(1) [],
        PGE OFFSET(3) NUMBITS(1) [],
        R50_DBG_RST OFFSET(2) NUMBITS(1) [],
        R50_RST OFFSET(1) NUMBITS(1) [],
        PGE_RST OFFSET(0) NUMBITS(1) [],
    ],
    pub RpuCcfMaskW [
        TEST_MBIST_MODE OFFSET(7) NUMBITS(1) [],
        TEST_SCAN_MODE_LP OFFSET(6) NUMBITS(1) [],
        TEST_SCAN_MODE OFFSET(5) NUMBITS(1) [],
        ISO OFFSET(4) NUMBITS(1) [],
        PGE OFFSET(3) NUMBITS(1) [],
        R50_DBG_RST OFFSET(2) NUMBITS(1) [],
        R50_RST OFFSET(1) NUMBITS(1) [],
        PGE_RST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RpuCcfValR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        TEST_MBIST_MODE OFFSET(7) NUMBITS(1) [],
        TEST_SCAN_MODE_LP OFFSET(6) NUMBITS(1) [],
        TEST_SCAN_MODE OFFSET(5) NUMBITS(1) [],
        ISO OFFSET(4) NUMBITS(1) [],
        PGE OFFSET(3) NUMBITS(1) [],
        R50_DBG_RST OFFSET(2) NUMBITS(1) [],
        R50_RST OFFSET(1) NUMBITS(1) [],
        PGE_RST OFFSET(0) NUMBITS(1) [],
    ],
    pub RpuCcfValW [
        TEST_MBIST_MODE OFFSET(7) NUMBITS(1) [],
        TEST_SCAN_MODE_LP OFFSET(6) NUMBITS(1) [],
        TEST_SCAN_MODE OFFSET(5) NUMBITS(1) [],
        ISO OFFSET(4) NUMBITS(1) [],
        PGE OFFSET(3) NUMBITS(1) [],
        R50_DBG_RST OFFSET(2) NUMBITS(1) [],
        R50_RST OFFSET(1) NUMBITS(1) [],
        PGE_RST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu0CfgR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        CFGNMFI0 OFFSET(3) NUMBITS(1) [],
        VINITHI OFFSET(2) NUMBITS(1) [],
        COHERENT OFFSET(1) NUMBITS(1) [],
        NCPUHALT OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu0CfgW [
        CFGNMFI0 OFFSET(3) NUMBITS(1) [],
        VINITHI OFFSET(2) NUMBITS(1) [],
        COHERENT OFFSET(1) NUMBITS(1) [],
        NCPUHALT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu0Status [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        NVALRESET OFFSET(5) NUMBITS(1) [],
        NVALIRQ OFFSET(4) NUMBITS(1) [],
        NVALFIQ OFFSET(3) NUMBITS(1) [],
        NWFIPIPESTOPPED OFFSET(2) NUMBITS(1) [],
        NWFEPIPESTOPPED OFFSET(1) NUMBITS(1) [],
        NCLKSTOPPED OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu0PwrdwnR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu0PwrdwnW [
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu0IsrR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu0IsrW [
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu0Imr [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu0IenR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
    ],
    pub Rpu0IenW [
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu0IdsR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
    ],
    pub Rpu0IdsW [
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu0SlvBaseR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ADDR OFFSET(0) NUMBITS(8) [],
    ],
    pub Rpu0SlvBaseW [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu0AxiOverR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        AWCACHE OFFSET(6) NUMBITS(4) [],
        ARCACHE OFFSET(2) NUMBITS(4) [],
        AWCACHE_EN OFFSET(1) NUMBITS(1) [],
        ARCACHE_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu0AxiOverW [
        AWCACHE OFFSET(6) NUMBITS(4) [],
        ARCACHE OFFSET(2) NUMBITS(4) [],
        AWCACHE_EN OFFSET(1) NUMBITS(1) [],
        ARCACHE_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu1CfgR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        CFGNMFI1 OFFSET(3) NUMBITS(1) [],
        VINITHI OFFSET(2) NUMBITS(1) [],
        COHERENT OFFSET(1) NUMBITS(1) [],
        NCPUHALT OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu1CfgW [
        CFGNMFI1 OFFSET(3) NUMBITS(1) [],
        VINITHI OFFSET(2) NUMBITS(1) [],
        COHERENT OFFSET(1) NUMBITS(1) [],
        NCPUHALT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu1Status [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        NVALRESET OFFSET(5) NUMBITS(1) [],
        NVALIRQ OFFSET(4) NUMBITS(1) [],
        NVALFIQ OFFSET(3) NUMBITS(1) [],
        NWFIPIPESTOPPED OFFSET(2) NUMBITS(1) [],
        NWFEPIPESTOPPED OFFSET(1) NUMBITS(1) [],
        NCLKSTOPPED OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu1PwrdwnR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu1PwrdwnW [
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu1IsrR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu1IsrW [
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu1Imr [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu1IenR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
    ],
    pub Rpu1IenW [
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu1IdsR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
    ],
    pub Rpu1IdsW [
        FPUFC OFFSET(24) NUMBITS(1) [],
        FPOFC OFFSET(23) NUMBITS(1) [],
        FPIXC OFFSET(22) NUMBITS(1) [],
        FPIOC OFFSET(21) NUMBITS(1) [],
        FPIDC OFFSET(20) NUMBITS(1) [],
        FPDZC OFFSET(19) NUMBITS(1) [],
        TCM_ASLV_CE OFFSET(18) NUMBITS(1) [],
        TCM_ASLV_FAT OFFSET(17) NUMBITS(1) [],
        TCM_LST_CE OFFSET(16) NUMBITS(1) [],
        TCM_PREFETCH_CE OFFSET(15) NUMBITS(1) [],
        B1TCM_CE OFFSET(14) NUMBITS(1) [],
        B0TCM_CE OFFSET(13) NUMBITS(1) [],
        ATCM_CE OFFSET(12) NUMBITS(1) [],
        B1TCM_UE OFFSET(11) NUMBITS(1) [],
        B0TCM_UE OFFSET(10) NUMBITS(1) [],
        ATCM_UE OFFSET(9) NUMBITS(1) [],
        DTAG_DIRTY_FAT OFFSET(8) NUMBITS(1) [],
        DDATA_FAT OFFSET(7) NUMBITS(1) [],
        TCM_LST_FAT OFFSET(6) NUMBITS(1) [],
        TCM_PREFETCH_FAT OFFSET(5) NUMBITS(1) [],
        DDATA_CE OFFSET(4) NUMBITS(1) [],
        DTAG_DIRTY_CE OFFSET(3) NUMBITS(1) [],
        IDATA_CE OFFSET(2) NUMBITS(1) [],
        ITAG_CE OFFSET(1) NUMBITS(1) [],
        APB_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu1SlvBaseR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ADDR OFFSET(0) NUMBITS(8) [],
    ],
    pub Rpu1SlvBaseW [
        ADDR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpu1AxiOverR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        AWCACHE OFFSET(6) NUMBITS(4) [],
        ARCACHE OFFSET(2) NUMBITS(4) [],
        AWCACHE_EN OFFSET(1) NUMBITS(1) [],
        ARCACHE_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu1AxiOverW [
        AWCACHE OFFSET(6) NUMBITS(4) [],
        ARCACHE OFFSET(2) NUMBITS(4) [],
        AWCACHE_EN OFFSET(1) NUMBITS(1) [],
        ARCACHE_EN OFFSET(0) NUMBITS(1) [],
    ]
];

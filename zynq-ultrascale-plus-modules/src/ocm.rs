// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// On-Chip Memory, OCM Memory Controller Configuration
pub static mut OCM: *mut Registers = 0xff960000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Enable/Disable a error response
    pub ocm_err_ctrl: Aliased<u32, OcmErrCtrlR::Register, OcmErrCtrlW::Register>,
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub ocm_isr: Aliased<u32, OcmIsrR::Register, OcmIsrW::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub ocm_imr: ReadOnly<u32, OcmImr::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub ocm_ien: Aliased<u32, OcmIenR::Register, OcmIenW::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub ocm_ids: Aliased<u32, OcmIdsR::Register, OcmIdsW::Register>,
    /// control register for OCM
    pub ocm_ecc_ctrl: Aliased<u32, OcmEccCtrlR::Register, OcmEccCtrlW::Register>,
    /// CLEAR REGITER FOR EXLCUSIVE ACCESS MONITORS
    pub ocm_clr_exe: Aliased<u32, OcmClrExeR::Register, OcmClrExeW::Register>,
    /// Correctable Error First Failing Address Register
    pub ocm_ce_ffa: ReadOnly<u32, OcmCeFfa::Register>,
    /// Correctable Error First Failing Data Register (CE_FFD [31:0])
    pub ocm_ce_ffd0: ReadOnly<u32>,
    /// Correctable Error First Failing Data Register (CE_FFD [63:32])
    pub ocm_ce_ffd1: ReadOnly<u32>,
    /// Correctable Error First Failing Data Register (CE_FFD [95:64])
    pub ocm_ce_ffd2: ReadOnly<u32>,
    /// Correctable Error First Failing Data Register (CE_FFD [127:96])
    pub ocm_ce_ffd3: ReadOnly<u32>,
    /// Correctable Error First Failing ECC Register
    pub ocm_ce_ffe: ReadOnly<u32, OcmCeFfe::Register>,
    /// Uncorrectable Error First Failing Address Register
    pub ocm_ue_ffa: ReadOnly<u32, OcmUeFfa::Register>,
    /// Uncorrectable Error First Failing Data Register (UE_FFD [31:0])
    pub ocm_ue_ffd0: ReadOnly<u32>,
    /// Uncorrectable Error First Failing Data Register (UE_FFD [63:32])
    pub ocm_ue_ffd1: ReadOnly<u32>,
    /// Uncorrectable Error First Failing Data Register (UE_FFD [95:64])
    pub ocm_ue_ffd2: ReadOnly<u32>,
    /// Uncorrectable Error First Failing Data Register (UE_FFD [127:96])
    pub ocm_ue_ffd3: ReadOnly<u32>,
    /// Uncorrectable Error First Failing ECC Register
    pub ocm_ue_ffe: ReadOnly<u32, OcmUeFfe::Register>,
    /// Fault Injection Data Register (FI_D0)
    pub ocm_fi_d0: ReadWrite<u32>,
    /// Fault Injection Data Register (FI_D1)
    pub ocm_fi_d1: ReadWrite<u32>,
    /// Fault Injection Data Register (FI_D2)
    pub ocm_fi_d2: ReadWrite<u32>,
    /// Fault Injection Data Register (FI_D3)
    pub ocm_fi_d3: ReadWrite<u32>,
    /// Fault Injection Syndrome Register (FI_SY)
    pub ocm_fi_sy: Aliased<u32, OcmFiSyR::Register, OcmFiSyW::Register>,
    _padding96: [u8; 16],
    /// RMW uncorrectable error log register
    pub ocm_rmw_ue_ffa: ReadOnly<u32, OcmRmwUeFfa::Register>,
    /// Fault Injection Count Register
    pub ocm_fi_cntr: Aliased<u32, OcmFiCntrR::Register, OcmFiCntrW::Register>,
    _padding120: [u8; 8],
    /// OCM Implementation Register
    pub ocm_imp: ReadOnly<u32, OcmImp::Register>,
    /// OCM PRDY Debug register
    pub ocm_prdy_dbg: ReadOnly<u32, OcmPrdyDbg::Register>,
    _padding136: [u8; 3952],
    /// OCM Safety Check Register
    pub ocm_safety_chk: ReadWrite<u32>,
}
tock_registers::register_bitfields! [
    u32,
    pub OcmErrCtrlR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        UE_RES OFFSET(3) NUMBITS(1) [],
        PWR_ERR_RES OFFSET(2) NUMBITS(1) [],
        PZ_ERR_RES OFFSET(1) NUMBITS(1) [],
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ],
    pub OcmErrCtrlW [
        UE_RES OFFSET(3) NUMBITS(1) [],
        PWR_ERR_RES OFFSET(2) NUMBITS(1) [],
        PZ_ERR_RES OFFSET(1) NUMBITS(1) [],
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmIsrR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        UE_RMW OFFSET(10) NUMBITS(1) [],
        FIX_BURST_WR OFFSET(9) NUMBITS(1) [],
        FIX_BURST_RD OFFSET(8) NUMBITS(1) [],
        ECC_UE OFFSET(7) NUMBITS(1) [],
        ECC_CE OFFSET(6) NUMBITS(1) [],
        LOCK_ERR_WR OFFSET(5) NUMBITS(1) [],
        LOCK_ERR_RD OFFSET(4) NUMBITS(1) [],
        INV_OCM_WR OFFSET(3) NUMBITS(1) [],
        INV_OCM_RD OFFSET(2) NUMBITS(1) [],
        PWR_DWN OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ],
    pub OcmIsrW [
        UE_RMW OFFSET(10) NUMBITS(1) [],
        FIX_BURST_WR OFFSET(9) NUMBITS(1) [],
        FIX_BURST_RD OFFSET(8) NUMBITS(1) [],
        ECC_UE OFFSET(7) NUMBITS(1) [],
        ECC_CE OFFSET(6) NUMBITS(1) [],
        LOCK_ERR_WR OFFSET(5) NUMBITS(1) [],
        LOCK_ERR_RD OFFSET(4) NUMBITS(1) [],
        INV_OCM_WR OFFSET(3) NUMBITS(1) [],
        INV_OCM_RD OFFSET(2) NUMBITS(1) [],
        PWR_DWN OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmImr [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        UE_RMW OFFSET(10) NUMBITS(1) [],
        FIX_BURST_WR OFFSET(9) NUMBITS(1) [],
        FIX_BURST_RD OFFSET(8) NUMBITS(1) [],
        ECC_UE OFFSET(7) NUMBITS(1) [],
        ECC_CE OFFSET(6) NUMBITS(1) [],
        LOCK_ERR_WR OFFSET(5) NUMBITS(1) [],
        LOCK_ERR_RD OFFSET(4) NUMBITS(1) [],
        INV_OCM_WR OFFSET(3) NUMBITS(1) [],
        INV_OCM_RD OFFSET(2) NUMBITS(1) [],
        PWR_DWN OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmIenR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
    ],
    pub OcmIenW [
        UE_RMW OFFSET(10) NUMBITS(1) [],
        FIX_BURST_WR OFFSET(9) NUMBITS(1) [],
        FIX_BURST_RD OFFSET(8) NUMBITS(1) [],
        ECC_UE OFFSET(7) NUMBITS(1) [],
        ECC_CE OFFSET(6) NUMBITS(1) [],
        LOCK_ERR_WR OFFSET(5) NUMBITS(1) [],
        LOCK_ERR_RD OFFSET(4) NUMBITS(1) [],
        INV_OCM_WR OFFSET(3) NUMBITS(1) [],
        INV_OCM_RD OFFSET(2) NUMBITS(1) [],
        PWR_DWN OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmIdsR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
    ],
    pub OcmIdsW [
        UE_RMW OFFSET(10) NUMBITS(1) [],
        FIX_BURST_WR OFFSET(9) NUMBITS(1) [],
        FIX_BURST_RD OFFSET(8) NUMBITS(1) [],
        ECC_UE OFFSET(7) NUMBITS(1) [],
        ECC_CE OFFSET(6) NUMBITS(1) [],
        LOCK_ERR_WR OFFSET(5) NUMBITS(1) [],
        LOCK_ERR_RD OFFSET(4) NUMBITS(1) [],
        INV_OCM_WR OFFSET(3) NUMBITS(1) [],
        INV_OCM_RD OFFSET(2) NUMBITS(1) [],
        PWR_DWN OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmEccCtrlR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        FI_MODE OFFSET(2) NUMBITS(1) [],
        DET_ONLY OFFSET(1) NUMBITS(1) [],
        ECC_ON_OFF OFFSET(0) NUMBITS(1) [],
    ],
    pub OcmEccCtrlW [
        FI_MODE OFFSET(2) NUMBITS(1) [],
        DET_ONLY OFFSET(1) NUMBITS(1) [],
        ECC_ON_OFF OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmClrExeR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
    ],
    pub OcmClrExeW [
        MON_7 OFFSET(7) NUMBITS(1) [],
        MON_6 OFFSET(6) NUMBITS(1) [],
        MON_5 OFFSET(5) NUMBITS(1) [],
        MON_4 OFFSET(4) NUMBITS(1) [],
        MON_3 OFFSET(3) NUMBITS(1) [],
        MON_2 OFFSET(2) NUMBITS(1) [],
        MON_1 OFFSET(1) NUMBITS(1) [],
        MON_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmCeFfa [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        ADDR OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmCeFfe [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        SYNDROME OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmUeFfa [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        ADDR OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmUeFfe [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        SYNDROME OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmFiSyR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        DATA OFFSET(0) NUMBITS(16) [],
    ],
    pub OcmFiSyW [
        DATA OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmRmwUeFfa [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        ADDR OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmFiCntrR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        COUNT OFFSET(0) NUMBITS(24) [],
    ],
    pub OcmFiCntrW [
        COUNT OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmImp [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        SIZE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcmPrdyDbg [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        BANK3 OFFSET(12) NUMBITS(4) [],
        BANK2 OFFSET(8) NUMBITS(4) [],
        BANK1 OFFSET(4) NUMBITS(4) [],
        BANK0 OFFSET(0) NUMBITS(4) [],
    ]
];

// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// PMU Global Control, PMU Global Control
pub static mut PMU_GLOBAL: *mut Registers = 0xffd80000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// PMU control and status register.
    pub global_cntrl: Aliased<u32, GlobalCntrlR::Register, GlobalCntrlW::Register>,
    /// PL Program Initiation Control.
    pub ps_cntrl: Aliased<u32, PsCntrlR::Register, PsCntrlW::Register>,
    /// APU Power Initialization Status.
    pub apu_pwr_status_init: Aliased<u32, ApuPwrStatusInitR::Register, ApuPwrStatusInitW::Register>,
    _padding12: [u8; 4],
    /// Register Address Error; Interrupt Status and Clear.
    pub addr_error_status: Aliased<u32, AddrErrorStatusR::Register, AddrErrorStatusW::Register>,
    /// Register Address Error; Interrupt Mask.
    pub addr_error_int_mask: ReadOnly<u32, AddrErrorIntMask::Register>,
    /// Register Address Error; Interrupt Enable.
    pub addr_error_int_en: WriteOnly<u32, AddrErrorIntEn::Register>,
    /// Register Address Error; Interrupt Disable.
    pub addr_error_int_dis: WriteOnly<u32, AddrErrorIntDis::Register>,
    _padding32: [u8; 16],
    /// Global Storage, Reg 0.
    pub global_gen_storage0: ReadWrite<u32>,
    /// Global Storage, Reg 1.
    pub global_gen_storage1: ReadWrite<u32>,
    /// Global Storage, Reg 2.
    pub global_gen_storage2: ReadWrite<u32>,
    /// Global Storage, Reg 3.
    pub global_gen_storage3: ReadWrite<u32>,
    /// Global Storage, Reg 4.
    pub global_gen_storage4: ReadWrite<u32>,
    /// Global Storage, Reg 5.
    pub global_gen_storage5: ReadWrite<u32>,
    /// Global Storage, Reg 6.
    pub global_gen_storage6: ReadWrite<u32>,
    _padding76: [u8; 4],
    /// Persistent Global Storage, Reg 0.
    pub pers_glob_gen_storage0: ReadWrite<u32>,
    /// Persistent Global Storage, Reg 1.
    pub pers_glob_gen_storage1: ReadWrite<u32>,
    /// Persistent Global Storage, Reg 2.
    pub pers_glob_gen_storage2: ReadWrite<u32>,
    /// Persistent Global Storage, Reg 3.
    pub pers_glob_gen_storage3: ReadWrite<u32>,
    /// Persistent Global Storage, Reg 4.
    pub pers_glob_gen_storage4: ReadWrite<u32>,
    /// Persistent Global Storage, Reg 5.
    pub pers_glob_gen_storage5: ReadWrite<u32>,
    /// Persistent Global Storage, Reg 6.
    pub pers_glob_gen_storage6: ReadWrite<u32>,
    /// Persistent Global Storage, Reg 7.
    pub pers_glob_gen_storage7: ReadWrite<u32>,
    /// DDR Output Signal Latch Control.
    pub ddr_cntrl: ReadWrite<u8, DdrCntrl::Register>,
    _padding113: [u8; 143],
    /// Power State Status; PS Islands, PL Internal and FPD.
    pub pwr_state: ReadOnly<u32, PwrState::Register>,
    /// Memory Retention and RPU Emulation State.
    pub aux_pwr_state: ReadOnly<u32, AuxPwrState::Register>,
    /// Memory Retention Requests.
    pub ram_ret_cntrl: Aliased<u32, RamRetCntrlR::Register, RamRetCntrlW::Register>,
    /// PS Power Supply Status.
    pub pwr_supply_status: ReadOnly<u32, PwrSupplyStatus::Register>,
    /// Power-up Request; Interrupt Status and Clear.
    pub req_pwrup_status: Aliased<u32, ReqPwrupStatusR::Register, ReqPwrupStatusW::Register>,
    /// Power-up Request; Interrupt Mask.
    pub req_pwrup_int_mask: ReadOnly<u32, ReqPwrupIntMask::Register>,
    /// Power-up Request; Interrupt Enable.
    pub req_pwrup_int_en: WriteOnly<u32, ReqPwrupIntEn::Register>,
    /// Power-up Request; Interrupt Disable.
    pub req_pwrup_int_dis: WriteOnly<u32, ReqPwrupIntDis::Register>,
    /// Power-up Request; Interrupt Trigger.
    pub req_pwrup_trig: WriteOnly<u32, ReqPwrupTrig::Register>,
    _padding292: [u8; 236],
    /// Power-down or RAM Retention Request; Interrupt Status and Clear.
    pub req_pwrdwn_status: Aliased<u32, ReqPwrdwnStatusR::Register, ReqPwrdwnStatusW::Register>,
    /// Power-down or RAM Retention Request; Interrupt Mask.
    pub req_pwrdwn_int_mask: ReadOnly<u32, ReqPwrdwnIntMask::Register>,
    /// Power-down or RAM Retention Request; Interrupt Enable.
    pub req_pwrdwn_int_en: WriteOnly<u32, ReqPwrdwnIntEn::Register>,
    /// Power-down or RAM Retention Request; Interrupt Disable.
    pub req_pwrdwn_int_dis: WriteOnly<u32, ReqPwrdwnIntDis::Register>,
    /// Power-down or RAM Retention Request; Interrupt Trigger.
    pub req_pwrdwn_trig: WriteOnly<u32, ReqPwrdwnTrig::Register>,
    _padding548: [u8; 236],
    /// Isolation Request; Interrupt Status and Clear.
    pub req_iso_status: Aliased<u32, ReqIsoStatusR::Register, ReqIsoStatusW::Register>,
    /// Isolation Request; Interrupt Mask.
    pub req_iso_int_mask: ReadOnly<u32, ReqIsoIntMask::Register>,
    /// Isolation Request; Interrupt Enable.
    pub req_iso_int_en: WriteOnly<u32, ReqIsoIntEn::Register>,
    /// Isolation Request; Interrupt Disable.
    pub req_iso_int_dis: WriteOnly<u32, ReqIsoIntDis::Register>,
    /// Isolation Request; Interrupt Trigger.
    pub req_iso_trig: WriteOnly<u32, ReqIsoTrig::Register>,
    _padding804: [u8; 236],
    /// Reset Request; Interrupt Status and Clear.
    pub req_swrst_status: Aliased<u32, ReqSwrstStatusR::Register, ReqSwrstStatusW::Register>,
    /// Reset Request; Interrupt Mask.Check the REQ_SWRST_STATUS register bits for more information.
    pub req_swrst_int_mask: ReadOnly<u32, ReqSwrstIntMask::Register>,
    /// Reset Request; Interrupt Enable.Check the REQ_SWRST_STATUS register bits for more information.
    pub req_swrst_int_en: WriteOnly<u32, ReqSwrstIntEn::Register>,
    /// Reset Request; Interrupt Disable.Check the REQ_SWRST_STATUS register bits for more information.
    pub req_swrst_int_dis: WriteOnly<u32, ReqSwrstIntDis::Register>,
    /// Reset Request; Interrupt Trigger.Check the REQ_SWRST_STATUS register bits for more information.
    pub req_swrst_trig: WriteOnly<u32, ReqSwrstTrig::Register>,
    _padding1060: [u8; 260],
    /// BootROM Error detection and code.
    pub csu_br_error: Aliased<u32, CsuBrErrorR::Register, CsuBrErrorW::Register>,
    /// PMU Fault Status; Lockstep, Fatal, Selfcheck, Sleep Instruction.
    pub mb_fault_status: ReadOnly<u32, MbFaultStatus::Register>,
    /// System Errors; Interrupt Clear and Status, Reg 1.
    pub error_status_1: Aliased<u32, ErrorStatus1R::Register, ErrorStatus1W::Register>,
    /// System Errors to PMU; Interrupt Mask, Reg 1.
    pub error_int_mask_1: ReadOnly<u32, ErrorIntMask1::Register>,
    /// System Errors to PMU; Interrupt Enable, Reg 1.
    pub error_int_en_1: WriteOnly<u32, ErrorIntEn1::Register>,
    /// System Errors to PMU; Interrupt Disable, Reg 1.
    pub error_int_dis_1: WriteOnly<u32, ErrorIntDis1::Register>,
    /// System Errors; Interrupt Clear and Status, Reg 2.
    pub error_status_2: Aliased<u32, ErrorStatus2R::Register, ErrorStatus2W::Register>,
    /// System Errors to PMU; Interrupt Mask, Reg 2.
    pub error_int_mask_2: ReadOnly<u32, ErrorIntMask2::Register>,
    /// System Errors to PMU; Interrupt Enable, Reg 2.
    pub error_int_en_2: WriteOnly<u32, ErrorIntEn2::Register>,
    /// System Errors to PMU; Interrupt Disable, Reg 2.
    pub error_int_dis_2: WriteOnly<u32, ErrorIntDis2::Register>,
    /// System Errors to POR; Interrupt Mask, Reg 1.
    pub error_por_mask_1: ReadOnly<u32, ErrorPorMask1::Register>,
    /// System Errors to POR; Interrupt Enable, Reg 1.
    pub error_por_en_1: WriteOnly<u32, ErrorPorEn1::Register>,
    /// System Errors to POR; Interrupt Disable, Reg 1.
    pub error_por_dis_1: WriteOnly<u32, ErrorPorDis1::Register>,
    /// System Error to POR; Interrupt Mask, Reg 2.
    pub error_por_mask_2: ReadOnly<u32, ErrorPorMask2::Register>,
    /// System Errors to POR; Interrupt Enable, Reg 2.
    pub error_por_en_2: WriteOnly<u32, ErrorPorEn2::Register>,
    /// System Errors to POR; Interrupt Disable, Reg 2.
    pub error_por_dis_2: WriteOnly<u32, ErrorPorDis2::Register>,
    /// System Errors to Reset; Interrupt Mask, Reg 1.
    pub error_srst_mask_1: ReadOnly<u32, ErrorSrstMask1::Register>,
    /// System Errors to Reset; Interrupt Enable, Reg 1.
    pub error_srst_en_1: WriteOnly<u32, ErrorSrstEn1::Register>,
    /// System Errors to Reset; Interrupt Disable, Reg 1.
    pub error_srst_dis_1: WriteOnly<u32, ErrorSrstDis1::Register>,
    /// System Errors to Reset; Interrupt Mask, Reg 2.
    pub error_srst_mask_2: ReadOnly<u32, ErrorSrstMask2::Register>,
    /// System Errors to Reset; Interrupt Enable, Reg 2.
    pub error_srst_en_2: WriteOnly<u32, ErrorSrstEn2::Register>,
    /// System Errors to Reset; Interrupt Disable, Reg 2.
    pub error_srst_dis_2: WriteOnly<u32, ErrorSrstDis2::Register>,
    /// System Errors to PL; Interrupt Mask, Reg 1.
    pub error_sig_mask_1: ReadOnly<u32, ErrorSigMask1::Register>,
    /// System Errors to PL; Interrupt Enable, Reg 1.
    pub error_sig_en_1: WriteOnly<u32, ErrorSigEn1::Register>,
    /// System Errors to PL; Interrupt Disable, Reg 1.
    pub error_sig_dis_1: WriteOnly<u32, ErrorSigDis1::Register>,
    /// System Errors to PL; Interrupt Mask, Reg 2.
    pub error_sig_mask_2: ReadOnly<u32, ErrorSigMask2::Register>,
    /// System Errors to PL; Interrupt Enable, Reg 2.
    pub error_sig_en_2: WriteOnly<u32, ErrorSigEn2::Register>,
    /// System Errors to PL; Interrupt Disable, Reg 2.
    pub error_sig_dis_2: WriteOnly<u32, ErrorSigDis2::Register>,
    _padding1432: [u8; 8],
    /// System Error Enables, Reg 1.
    pub error_en_1: ReadWrite<u32, ErrorEn1::Register>,
    /// System Error Enables, Reg 2.
    pub error_en_2: ReadWrite<u32, ErrorEn2::Register>,
    _padding1448: [u8; 88],
    /// PS-PL AXI Bus Logic Isolation Requests.
    pub aib_cntrl: WriteOnly<u32, AibCntrl::Register>,
    /// PS-PL AXI Bus Logic Isolation Status.
    pub aib_status: ReadOnly<u32, AibStatus::Register>,
    /// GLOBAL_RESET
    pub global_reset: Aliased<u32, GlobalResetR::Register, GlobalResetW::Register>,
    _padding1548: [u8; 4],
    /// PMU ROM validation engine status.
    pub rom_validation_status: ReadOnly<u32, RomValidationStatus::Register>,
    /// PMU ROM Validation SHA value, Word 0.
    pub rom_validation_digest_0: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 1.
    pub rom_validation_digest_1: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 2.
    pub rom_validation_digest_2: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 3.
    pub rom_validation_digest_3: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 4.
    pub rom_validation_digest_4: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 5.
    pub rom_validation_digest_5: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 6.
    pub rom_validation_digest_6: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 7.
    pub rom_validation_digest_7: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 8.
    pub rom_validation_digest_8: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 9.
    pub rom_validation_digest_9: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 10.
    pub rom_validation_digest_10: ReadOnly<u32>,
    /// PMU ROM Validation SHA value, Word 11.
    pub rom_validation_digest_11: ReadOnly<u32>,
    _padding1604: [u8; 12],
    /// Safety gates disable hardware functions.
    pub safety_gate: Aliased<u32, SafetyGateR::Register, SafetyGateW::Register>,
    _padding1620: [u8; 172],
    /// On-demand MBIST Controller Reset, Trigger 0.
    pub mbist_rst: ReadWrite<u32, MbistRst::Register>,
    /// On-demand MBIST, Trigger 1.
    pub mbist_pg_en: ReadWrite<u32, MbistPgEn::Register>,
    /// On-demand MBIST, Trigger 2.
    pub mbist_setup: ReadWrite<u32, MbistSetup::Register>,
    _padding1804: [u8; 4],
    /// MBIST Done Indicator.
    pub mbist_done: ReadOnly<u32, MbistDone::Register>,
    /// MBIST Result Status.
    pub mbist_good: ReadOnly<u32, MbistGood::Register>,
    _padding1816: [u8; 232],
    /// Test ability to access this register set. Can be used at any time by any master with accessibility.
    pub safety_chk: ReadWrite<u32>,
}
tock_registers::register_bitfields! [
    u32,
    pub GlobalCntrlR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        MB_SLEEP OFFSET(16) NUMBITS(1) [],
        WRITE_QOS OFFSET(12) NUMBITS(4) [],
        READ_QOS OFFSET(8) NUMBITS(4) [],
        RESERVED1 OFFSET(5) NUMBITS(3) [],
        FW_IS_PRESENT OFFSET(4) NUMBITS(1) [],
        RESERVED2 OFFSET(3) NUMBITS(1) [],
        COHERENT OFFSET(2) NUMBITS(1) [],
        SLVERR_ENABLE OFFSET(1) NUMBITS(1) [],
        DONT_SLEEP OFFSET(0) NUMBITS(1) [],
    ],
    pub GlobalCntrlW [
        WRITE_QOS OFFSET(12) NUMBITS(4) [],
        READ_QOS OFFSET(8) NUMBITS(4) [],
        FW_IS_PRESENT OFFSET(4) NUMBITS(1) [],
        COHERENT OFFSET(2) NUMBITS(1) [],
        SLVERR_ENABLE OFFSET(1) NUMBITS(1) [],
        DONT_SLEEP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PsCntrlR [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        PROG_GATE_STATUS OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(14) [],
        PROG_ENABLE OFFSET(1) NUMBITS(1) [],
        PROG_GATE OFFSET(0) NUMBITS(1) [],
    ],
    pub PsCntrlW [
        PROG_ENABLE OFFSET(1) NUMBITS(1) [],
        PROG_GATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ApuPwrStatusInitR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ],
    pub ApuPwrStatusInitW [
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AddrErrorStatusR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        STATUS OFFSET(0) NUMBITS(1) [],
    ],
    pub AddrErrorStatusW [
        STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AddrErrorIntMask [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        MASK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AddrErrorIntEn [
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AddrErrorIntDis [
        DISABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub DdrCntrl [
        RET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PwrState [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        R5_1 OFFSET(11) NUMBITS(1) [],
        R5_0 OFFSET(10) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(1) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        RESERVED3 OFFSET(6) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AuxPwrState [
        ACPU3_EMULATION OFFSET(31) NUMBITS(1) [],
        ACPU2_EMULATION OFFSET(30) NUMBITS(1) [],
        ACPU1_EMULATION OFFSET(29) NUMBITS(1) [],
        ACPU0_EMULATION OFFSET(28) NUMBITS(1) [],
        RPU_EMULATION OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RamRetCntrlR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(7) [],
    ],
    pub RamRetCntrlW [
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PwrSupplyStatus [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        VCC_INT OFFSET(1) NUMBITS(1) [],
        VCC_PSINTFP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrupStatusR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        RESERVED3 OFFSET(8) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        RESERVED4 OFFSET(6) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ],
    pub ReqPwrupStatusW [
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrupIntMask [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        RESERVED3 OFFSET(8) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        RESERVED4 OFFSET(6) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrupIntEn [
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrupIntDis [
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrupTrig [
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrdwnStatusR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        RESERVED3 OFFSET(8) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        RESERVED4 OFFSET(6) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ],
    pub ReqPwrdwnStatusW [
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrdwnIntMask [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        RESERVED3 OFFSET(8) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        RESERVED4 OFFSET(6) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrdwnIntEn [
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrdwnIntDis [
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqPwrdwnTrig [
        PL OFFSET(23) NUMBITS(1) [],
        FP OFFSET(22) NUMBITS(1) [],
        USB1 OFFSET(21) NUMBITS(1) [],
        USB0 OFFSET(20) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        L2_BANK0 OFFSET(7) NUMBITS(1) [],
        PP1 OFFSET(5) NUMBITS(1) [],
        PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqIsoStatusR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        FP_LOCKED OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        PL_NONPCAP OFFSET(2) NUMBITS(1) [],
        PL OFFSET(1) NUMBITS(1) [],
        FP OFFSET(0) NUMBITS(1) [],
    ],
    pub ReqIsoStatusW [
        FP_LOCKED OFFSET(4) NUMBITS(1) [],
        PL_NONPCAP OFFSET(2) NUMBITS(1) [],
        PL OFFSET(1) NUMBITS(1) [],
        FP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqIsoIntMask [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        FP_LOCKED OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        PL_NONPCAP OFFSET(2) NUMBITS(1) [],
        PL OFFSET(1) NUMBITS(1) [],
        FP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqIsoIntEn [
        FP_LOCKED OFFSET(4) NUMBITS(1) [],
        PL_NONPCAP OFFSET(2) NUMBITS(1) [],
        PL OFFSET(1) NUMBITS(1) [],
        FP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqIsoIntDis [
        FP_LOCKED OFFSET(4) NUMBITS(1) [],
        PL_NONPCAP OFFSET(2) NUMBITS(1) [],
        PL OFFSET(1) NUMBITS(1) [],
        FP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqIsoTrig [
        FP_LOCKED OFFSET(4) NUMBITS(1) [],
        PL_NONPCAP OFFSET(2) NUMBITS(1) [],
        PL OFFSET(1) NUMBITS(1) [],
        FP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqSwrstStatusR [
        PL OFFSET(31) NUMBITS(1) [],
        FP OFFSET(30) NUMBITS(1) [],
        LP OFFSET(29) NUMBITS(1) [],
        PS_ONLY OFFSET(28) NUMBITS(1) [],
        IOU OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(1) [],
        USB1 OFFSET(25) NUMBITS(1) [],
        USB0 OFFSET(24) NUMBITS(1) [],
        GEM3 OFFSET(23) NUMBITS(1) [],
        GEM2 OFFSET(22) NUMBITS(1) [],
        GEM1 OFFSET(21) NUMBITS(1) [],
        GEM0 OFFSET(20) NUMBITS(1) [],
        RESERVED1 OFFSET(19) NUMBITS(1) [],
        RPU OFFSET(18) NUMBITS(1) [],
        R5_1 OFFSET(17) NUMBITS(1) [],
        R5_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        DISPLAY_PORT OFFSET(12) NUMBITS(1) [],
        RESERVED3 OFFSET(11) NUMBITS(1) [],
        SATA OFFSET(10) NUMBITS(1) [],
        PCIE OFFSET(9) NUMBITS(1) [],
        GPU OFFSET(8) NUMBITS(1) [],
        PP1 OFFSET(7) NUMBITS(1) [],
        PP0 OFFSET(6) NUMBITS(1) [],
        RESERVED4 OFFSET(5) NUMBITS(1) [],
        APU OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ],
    pub ReqSwrstStatusW [
        PL OFFSET(31) NUMBITS(1) [],
        FP OFFSET(30) NUMBITS(1) [],
        LP OFFSET(29) NUMBITS(1) [],
        PS_ONLY OFFSET(28) NUMBITS(1) [],
        IOU OFFSET(27) NUMBITS(1) [],
        USB1 OFFSET(25) NUMBITS(1) [],
        USB0 OFFSET(24) NUMBITS(1) [],
        GEM3 OFFSET(23) NUMBITS(1) [],
        GEM2 OFFSET(22) NUMBITS(1) [],
        GEM1 OFFSET(21) NUMBITS(1) [],
        GEM0 OFFSET(20) NUMBITS(1) [],
        RPU OFFSET(18) NUMBITS(1) [],
        R5_1 OFFSET(17) NUMBITS(1) [],
        R5_0 OFFSET(16) NUMBITS(1) [],
        DISPLAY_PORT OFFSET(12) NUMBITS(1) [],
        SATA OFFSET(10) NUMBITS(1) [],
        PCIE OFFSET(9) NUMBITS(1) [],
        GPU OFFSET(8) NUMBITS(1) [],
        PP1 OFFSET(7) NUMBITS(1) [],
        PP0 OFFSET(6) NUMBITS(1) [],
        APU OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqSwrstIntMask [
        PL OFFSET(31) NUMBITS(1) [],
        FP OFFSET(30) NUMBITS(1) [],
        LP OFFSET(29) NUMBITS(1) [],
        PS_ONLY OFFSET(28) NUMBITS(1) [],
        IOU OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(1) [],
        USB1 OFFSET(25) NUMBITS(1) [],
        USB0 OFFSET(24) NUMBITS(1) [],
        GEM3 OFFSET(23) NUMBITS(1) [],
        GEM2 OFFSET(22) NUMBITS(1) [],
        GEM1 OFFSET(21) NUMBITS(1) [],
        GEM0 OFFSET(20) NUMBITS(1) [],
        RESERVED1 OFFSET(19) NUMBITS(1) [],
        LS_R5 OFFSET(18) NUMBITS(1) [],
        R5_1 OFFSET(17) NUMBITS(1) [],
        R5_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        DISPLAY_PORT OFFSET(12) NUMBITS(1) [],
        RESERVED3 OFFSET(11) NUMBITS(1) [],
        SATA OFFSET(10) NUMBITS(1) [],
        PCIE OFFSET(9) NUMBITS(1) [],
        GPU OFFSET(8) NUMBITS(1) [],
        PP1 OFFSET(7) NUMBITS(1) [],
        PP0 OFFSET(6) NUMBITS(1) [],
        RESERVED4 OFFSET(5) NUMBITS(1) [],
        APU OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqSwrstIntEn [
        PL OFFSET(31) NUMBITS(1) [],
        FP OFFSET(30) NUMBITS(1) [],
        LP OFFSET(29) NUMBITS(1) [],
        PS_ONLY OFFSET(28) NUMBITS(1) [],
        IOU OFFSET(27) NUMBITS(1) [],
        USB1 OFFSET(25) NUMBITS(1) [],
        USB0 OFFSET(24) NUMBITS(1) [],
        GEM3 OFFSET(23) NUMBITS(1) [],
        GEM2 OFFSET(22) NUMBITS(1) [],
        GEM1 OFFSET(21) NUMBITS(1) [],
        GEM0 OFFSET(20) NUMBITS(1) [],
        LS_R5 OFFSET(18) NUMBITS(1) [],
        R5_1 OFFSET(17) NUMBITS(1) [],
        R5_0 OFFSET(16) NUMBITS(1) [],
        DISPLAY_PORT OFFSET(12) NUMBITS(1) [],
        SATA OFFSET(10) NUMBITS(1) [],
        PCIE OFFSET(9) NUMBITS(1) [],
        GPU OFFSET(8) NUMBITS(1) [],
        PP1 OFFSET(7) NUMBITS(1) [],
        PP0 OFFSET(6) NUMBITS(1) [],
        APU OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqSwrstIntDis [
        PL OFFSET(31) NUMBITS(1) [],
        FP OFFSET(30) NUMBITS(1) [],
        LP OFFSET(29) NUMBITS(1) [],
        PS_ONLY OFFSET(28) NUMBITS(1) [],
        IOU OFFSET(27) NUMBITS(1) [],
        USB1 OFFSET(25) NUMBITS(1) [],
        USB0 OFFSET(24) NUMBITS(1) [],
        GEM3 OFFSET(23) NUMBITS(1) [],
        GEM2 OFFSET(22) NUMBITS(1) [],
        GEM1 OFFSET(21) NUMBITS(1) [],
        GEM0 OFFSET(20) NUMBITS(1) [],
        LS_R5 OFFSET(18) NUMBITS(1) [],
        R5_1 OFFSET(17) NUMBITS(1) [],
        R5_0 OFFSET(16) NUMBITS(1) [],
        DISPLAY_PORT OFFSET(12) NUMBITS(1) [],
        SATA OFFSET(10) NUMBITS(1) [],
        PCIE OFFSET(9) NUMBITS(1) [],
        GPU OFFSET(8) NUMBITS(1) [],
        PP1 OFFSET(7) NUMBITS(1) [],
        PP0 OFFSET(6) NUMBITS(1) [],
        APU OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ReqSwrstTrig [
        PL OFFSET(31) NUMBITS(1) [],
        FP OFFSET(30) NUMBITS(1) [],
        LP OFFSET(29) NUMBITS(1) [],
        PS_ONLY OFFSET(28) NUMBITS(1) [],
        IOU OFFSET(27) NUMBITS(1) [],
        USB1 OFFSET(25) NUMBITS(1) [],
        USB0 OFFSET(24) NUMBITS(1) [],
        GEM3 OFFSET(23) NUMBITS(1) [],
        GEM2 OFFSET(22) NUMBITS(1) [],
        GEM1 OFFSET(21) NUMBITS(1) [],
        GEM0 OFFSET(20) NUMBITS(1) [],
        LS_R5 OFFSET(18) NUMBITS(1) [],
        R5_1 OFFSET(17) NUMBITS(1) [],
        R5_0 OFFSET(16) NUMBITS(1) [],
        DISPLAY_PORT OFFSET(12) NUMBITS(1) [],
        SATA OFFSET(10) NUMBITS(1) [],
        PCIE OFFSET(9) NUMBITS(1) [],
        GPU OFFSET(8) NUMBITS(1) [],
        PP1 OFFSET(7) NUMBITS(1) [],
        PP0 OFFSET(6) NUMBITS(1) [],
        APU OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuBrErrorR [
        BR_ERROR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        ERR_TYPE OFFSET(0) NUMBITS(16) [],
    ],
    pub CsuBrErrorW [
        BR_ERROR OFFSET(31) NUMBITS(1) [],
        ERR_TYPE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MbFaultStatus [
        R_FFAIL OFFSET(24) NUMBITS(8) [],
        RESERVED0 OFFSET(20) NUMBITS(4) [],
        R_SLEEP_RST OFFSET(19) NUMBITS(1) [],
        R_LSFAIL OFFSET(16) NUMBITS(3) [],
        N_FFAIL OFFSET(8) NUMBITS(8) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        N_SLEEP_RST OFFSET(3) NUMBITS(1) [],
        N_LSFAIL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorStatus1R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ],
    pub ErrorStatus1W [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorIntMask1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorIntEn1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorIntDis1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorStatus2R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ],
    pub ErrorStatus2W [
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorIntMask2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorIntEn2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorIntDis2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorPorMask1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorPorEn1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorPorDis1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorPorMask2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorPorEn2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorPorDis2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSrstMask1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSrstEn1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSrstDis1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSrstMask2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSrstEn2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSrstDis2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSigMask1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSigEn1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSigDis1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSigMask2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSigEn2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorSigDis2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorEn1 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        CSU_SWDT OFFSET(27) NUMBITS(1) [],
        CLK_MON OFFSET(26) NUMBITS(1) [],
        XMPU OFFSET(24) NUMBITS(2) [],
        PWR_SUPPLY OFFSET(16) NUMBITS(8) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        FPD_SWDT OFFSET(13) NUMBITS(1) [],
        LPD_SWDT OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(10) NUMBITS(2) [],
        RPU_CCF OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RPU_LS OFFSET(6) NUMBITS(2) [],
        FPD_TEMP OFFSET(5) NUMBITS(1) [],
        LPD_TEMP OFFSET(4) NUMBITS(1) [],
        RPU1 OFFSET(3) NUMBITS(1) [],
        RPU0 OFFSET(2) NUMBITS(1) [],
        OCM_ECC OFFSET(1) NUMBITS(1) [],
        DDR_ECC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrorEn2 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        CSU_ROM OFFSET(26) NUMBITS(1) [],
        PMU_PB OFFSET(25) NUMBITS(1) [],
        PMU_SERVICE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        PMU_FW OFFSET(18) NUMBITS(4) [],
        PMU_UC OFFSET(17) NUMBITS(1) [],
        CSU OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        PLL_LOCK OFFSET(8) NUMBITS(5) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PL OFFSET(2) NUMBITS(4) [],
        TO OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AibCntrl [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        FPD_AFI_FS OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        LPD_AFI_FS OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AibStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        FPD_AFI_FS OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        LPD_AFI_FS OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GlobalResetR [
        RESERVED0 OFFSET(11) NUMBITS(21) [],
        PS_ONLY_RST OFFSET(10) NUMBITS(1) [],
        FPD_RST OFFSET(9) NUMBITS(1) [],
        RPU_LS_RST OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ],
    pub GlobalResetW [
        PS_ONLY_RST OFFSET(10) NUMBITS(1) [],
        FPD_RST OFFSET(9) NUMBITS(1) [],
        RPU_LS_RST OFFSET(8) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RomValidationStatus [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        PASS OFFSET(1) NUMBITS(1) [],
        DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SafetyGateR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        PMU_LOGCLR_ENABLE OFFSET(2) NUMBITS(1) [],
        LBIST_ENABLE OFFSET(1) NUMBITS(1) [],
        SCAN_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub SafetyGateW [
        PMU_LOGCLR_ENABLE OFFSET(2) NUMBITS(1) [],
        LBIST_ENABLE OFFSET(1) NUMBITS(1) [],
        SCAN_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MbistRst [
        PCIE OFFSET(31) NUMBITS(1) [],
        SIOU OFFSET(30) NUMBITS(1) [],
        M400_1 OFFSET(29) NUMBITS(1) [],
        M400_0 OFFSET(28) NUMBITS(1) [],
        GPU OFFSET(27) NUMBITS(1) [],
        DDR OFFSET(26) NUMBITS(1) [],
        ACPU_3 OFFSET(25) NUMBITS(1) [],
        ACPU_2 OFFSET(24) NUMBITS(1) [],
        ACPU_1 OFFSET(23) NUMBITS(1) [],
        ACPU_0 OFFSET(22) NUMBITS(1) [],
        APU OFFSET(21) NUMBITS(1) [],
        AFI_5 OFFSET(20) NUMBITS(1) [],
        AFI_4 OFFSET(19) NUMBITS(1) [],
        AFI_3 OFFSET(18) NUMBITS(1) [],
        AFI_2 OFFSET(17) NUMBITS(1) [],
        AFI_1 OFFSET(16) NUMBITS(1) [],
        AFI_0 OFFSET(15) NUMBITS(1) [],
        FPD OFFSET(14) NUMBITS(1) [],
        PSS_CORE_TOP OFFSET(13) NUMBITS(1) [],
        OCM OFFSET(12) NUMBITS(1) [],
        AFI_LPD OFFSET(11) NUMBITS(1) [],
        USB1 OFFSET(10) NUMBITS(1) [],
        USB0 OFFSET(9) NUMBITS(1) [],
        RPU_TIEOFF_WRAPPER OFFSET(8) NUMBITS(1) [],
        RPU OFFSET(7) NUMBITS(1) [],
        IOU OFFSET(6) NUMBITS(1) [],
        GEGM3 OFFSET(5) NUMBITS(1) [],
        GEM2 OFFSET(4) NUMBITS(1) [],
        GEM1 OFFSET(3) NUMBITS(1) [],
        GEM0 OFFSET(2) NUMBITS(1) [],
        CAN1 OFFSET(1) NUMBITS(1) [],
        CAN0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MbistPgEn [
        PCIE OFFSET(31) NUMBITS(1) [],
        SIOU OFFSET(30) NUMBITS(1) [],
        M400_1 OFFSET(29) NUMBITS(1) [],
        M400_0 OFFSET(28) NUMBITS(1) [],
        GPU OFFSET(27) NUMBITS(1) [],
        DDR OFFSET(26) NUMBITS(1) [],
        ACPU_3 OFFSET(25) NUMBITS(1) [],
        ACPU_2 OFFSET(24) NUMBITS(1) [],
        ACPU_1 OFFSET(23) NUMBITS(1) [],
        ACPU_0 OFFSET(22) NUMBITS(1) [],
        APU OFFSET(21) NUMBITS(1) [],
        AFI_5 OFFSET(20) NUMBITS(1) [],
        AFI_4 OFFSET(19) NUMBITS(1) [],
        AFI_3 OFFSET(18) NUMBITS(1) [],
        AFI_2 OFFSET(17) NUMBITS(1) [],
        AFI_1 OFFSET(16) NUMBITS(1) [],
        AFI_0 OFFSET(15) NUMBITS(1) [],
        FPD OFFSET(14) NUMBITS(1) [],
        PSS_CORE_TOP OFFSET(13) NUMBITS(1) [],
        OCM OFFSET(12) NUMBITS(1) [],
        AFI_LPD OFFSET(11) NUMBITS(1) [],
        USB1 OFFSET(10) NUMBITS(1) [],
        USB0 OFFSET(9) NUMBITS(1) [],
        RPU_TIEOFF_WRAPPER OFFSET(8) NUMBITS(1) [],
        RPU OFFSET(7) NUMBITS(1) [],
        IOU OFFSET(6) NUMBITS(1) [],
        GEGM3 OFFSET(5) NUMBITS(1) [],
        GEM2 OFFSET(4) NUMBITS(1) [],
        GEM1 OFFSET(3) NUMBITS(1) [],
        GEM0 OFFSET(2) NUMBITS(1) [],
        CAN1 OFFSET(1) NUMBITS(1) [],
        CAN0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MbistSetup [
        PCIE OFFSET(31) NUMBITS(1) [],
        SIOU OFFSET(30) NUMBITS(1) [],
        M400_1 OFFSET(29) NUMBITS(1) [],
        M400_0 OFFSET(28) NUMBITS(1) [],
        GPU OFFSET(27) NUMBITS(1) [],
        DDR OFFSET(26) NUMBITS(1) [],
        ACPU_3 OFFSET(25) NUMBITS(1) [],
        ACPU_2 OFFSET(24) NUMBITS(1) [],
        ACPU_1 OFFSET(23) NUMBITS(1) [],
        ACPU_0 OFFSET(22) NUMBITS(1) [],
        APU OFFSET(21) NUMBITS(1) [],
        AFI_5 OFFSET(20) NUMBITS(1) [],
        AFI_4 OFFSET(19) NUMBITS(1) [],
        AFI_3 OFFSET(18) NUMBITS(1) [],
        AFI_2 OFFSET(17) NUMBITS(1) [],
        AFI_1 OFFSET(16) NUMBITS(1) [],
        AFI_0 OFFSET(15) NUMBITS(1) [],
        FPD OFFSET(14) NUMBITS(1) [],
        PSS_CORE_TOP OFFSET(13) NUMBITS(1) [],
        OCM OFFSET(12) NUMBITS(1) [],
        AFI_LPD OFFSET(11) NUMBITS(1) [],
        USB1 OFFSET(10) NUMBITS(1) [],
        USB0 OFFSET(9) NUMBITS(1) [],
        RPU_TIEOFF_WRAPPER OFFSET(8) NUMBITS(1) [],
        RPU OFFSET(7) NUMBITS(1) [],
        IOU OFFSET(6) NUMBITS(1) [],
        GEGM3 OFFSET(5) NUMBITS(1) [],
        GEM2 OFFSET(4) NUMBITS(1) [],
        GEM1 OFFSET(3) NUMBITS(1) [],
        GEM0 OFFSET(2) NUMBITS(1) [],
        CAN1 OFFSET(1) NUMBITS(1) [],
        CAN0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MbistDone [
        PCIE OFFSET(31) NUMBITS(1) [],
        SIOU OFFSET(30) NUMBITS(1) [],
        M400_1 OFFSET(29) NUMBITS(1) [],
        M400_0 OFFSET(28) NUMBITS(1) [],
        GPU OFFSET(27) NUMBITS(1) [],
        DDR OFFSET(26) NUMBITS(1) [],
        ACPU_3 OFFSET(25) NUMBITS(1) [],
        ACPU_2 OFFSET(24) NUMBITS(1) [],
        ACPU_1 OFFSET(23) NUMBITS(1) [],
        ACPU_0 OFFSET(22) NUMBITS(1) [],
        APU OFFSET(21) NUMBITS(1) [],
        AFI_5 OFFSET(20) NUMBITS(1) [],
        AFI_4 OFFSET(19) NUMBITS(1) [],
        AFI_3 OFFSET(18) NUMBITS(1) [],
        AFI_2 OFFSET(17) NUMBITS(1) [],
        AFI_1 OFFSET(16) NUMBITS(1) [],
        AFI_0 OFFSET(15) NUMBITS(1) [],
        FPD OFFSET(14) NUMBITS(1) [],
        PSS_CORE_TOP OFFSET(13) NUMBITS(1) [],
        OCM OFFSET(12) NUMBITS(1) [],
        AFI_LPD OFFSET(11) NUMBITS(1) [],
        USB1 OFFSET(10) NUMBITS(1) [],
        USB0 OFFSET(9) NUMBITS(1) [],
        RPU_TIEOFF_WRAPPER OFFSET(8) NUMBITS(1) [],
        RPU OFFSET(7) NUMBITS(1) [],
        IOU OFFSET(6) NUMBITS(1) [],
        GEGM3 OFFSET(5) NUMBITS(1) [],
        GEM2 OFFSET(4) NUMBITS(1) [],
        GEM1 OFFSET(3) NUMBITS(1) [],
        GEM0 OFFSET(2) NUMBITS(1) [],
        CAN1 OFFSET(1) NUMBITS(1) [],
        CAN0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MbistGood [
        PCIE OFFSET(31) NUMBITS(1) [],
        SIOU OFFSET(30) NUMBITS(1) [],
        M400_1 OFFSET(29) NUMBITS(1) [],
        M400_0 OFFSET(28) NUMBITS(1) [],
        GPU OFFSET(27) NUMBITS(1) [],
        DDR OFFSET(26) NUMBITS(1) [],
        ACPU_3 OFFSET(25) NUMBITS(1) [],
        ACPU_2 OFFSET(24) NUMBITS(1) [],
        ACPU_1 OFFSET(23) NUMBITS(1) [],
        ACPU_0 OFFSET(22) NUMBITS(1) [],
        APU OFFSET(21) NUMBITS(1) [],
        AFI_5 OFFSET(20) NUMBITS(1) [],
        AFI_4 OFFSET(19) NUMBITS(1) [],
        AFI_3 OFFSET(18) NUMBITS(1) [],
        AFI_2 OFFSET(17) NUMBITS(1) [],
        AFI_1 OFFSET(16) NUMBITS(1) [],
        AFI_0 OFFSET(15) NUMBITS(1) [],
        FPD OFFSET(14) NUMBITS(1) [],
        PSS_CORE_TOP OFFSET(13) NUMBITS(1) [],
        OCM OFFSET(12) NUMBITS(1) [],
        AFI_LPD OFFSET(11) NUMBITS(1) [],
        USB1 OFFSET(10) NUMBITS(1) [],
        USB0 OFFSET(9) NUMBITS(1) [],
        RPU_TIEOFF_WRAPPER OFFSET(8) NUMBITS(1) [],
        RPU OFFSET(7) NUMBITS(1) [],
        IOU OFFSET(6) NUMBITS(1) [],
        GEGM3 OFFSET(5) NUMBITS(1) [],
        GEM2 OFFSET(4) NUMBITS(1) [],
        GEM1 OFFSET(3) NUMBITS(1) [],
        GEM0 OFFSET(2) NUMBITS(1) [],
        CAN1 OFFSET(1) NUMBITS(1) [],
        CAN0 OFFSET(0) NUMBITS(1) [],
    ]
];

// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// PMU MicroBlaze Processor Local Control, PMU Local Control
pub static mut PMU_LOCAL: *mut Registers = 0xffd60000 as *mut Registers;
register_structs! {
    pub Registers {
        /// APU Core 0 Power and Isolation Control. Reset by POR only.
        (0x00000000 => pub acpu0_pwr_cntrl: Aliased<u32, Acpu0PwrCntrlR::Register, Acpu0PwrCntrlW::Register>),
        /// APU Core 0 Power Status. Reset by POR only.
        (0x00000004 => pub acpu0_pwr_status: ReadOnly<u32, Acpu0PwrStatus::Register>),
        (0x00000008 => _padding8),
        /// APU Core 1 Power and Isolation Control.Reset by POR only.
        (0x00000010 => pub acpu1_pwr_cntrl: Aliased<u32, Acpu1PwrCntrlR::Register, Acpu1PwrCntrlW::Register>),
        /// APU Core 1 Power Status. Reset by POR only.
        (0x00000014 => pub acpu1_pwr_status: ReadOnly<u32, Acpu1PwrStatus::Register>),
        (0x00000018 => _padding24),
        /// APU Core 2 Power and Isolation Control. Reset by POR only.
        (0x00000020 => pub acpu2_pwr_cntrl: Aliased<u32, Acpu2PwrCntrlR::Register, Acpu2PwrCntrlW::Register>),
        /// APU Core 2 Power Status. Reset by POR only.
        (0x00000024 => pub acpu2_pwr_status: ReadOnly<u32, Acpu2PwrStatus::Register>),
        (0x00000028 => _padding40),
        /// APU Core 3 Power and Isolation Control. Reset by POR only.
        (0x00000030 => pub acpu3_pwr_cntrl: Aliased<u32, Acpu3PwrCntrlR::Register, Acpu3PwrCntrlW::Register>),
        /// APU Core 3 Power Status. Reset by POR only.
        (0x00000034 => pub acpu3_pwr_status: ReadOnly<u32, Acpu3PwrStatus::Register>),
        (0x00000038 => _padding56),
        /// GPU PP0 Power and Isolation Control. Reset by POR only.
        (0x00000040 => pub pp0_pwr_cntrl: Aliased<u32, Pp0PwrCntrlR::Register, Pp0PwrCntrlW::Register>),
        /// GPU PP0 Power Status. Reset by POR only.
        (0x00000044 => pub pp0_pwr_status: ReadOnly<u32, Pp0PwrStatus::Register>),
        /// GPU PP1 Power and Isolation Control. Reset by POR only.
        (0x00000048 => pub pp1_pwr_cntrl: Aliased<u32, Pp1PwrCntrlR::Register, Pp1PwrCntrlW::Register>),
        /// GPU PP1 Power Status. Reset by POR only.
        (0x0000004c => pub pp1_pwr_status: ReadOnly<u32, Pp1PwrStatus::Register>),
        (0x00000050 => _padding80),
        /// USB 0 Power and Isolation Control. Reset by POR only.
        (0x00000060 => pub usb0_pwr_cntrl: Aliased<u32, Usb0PwrCntrlR::Register, Usb0PwrCntrlW::Register>),
        /// USB0 Power Status. Reset by POR only.
        (0x00000064 => pub usb0_pwr_status: ReadOnly<u32, Usb0PwrStatus::Register>),
        (0x00000068 => _padding104),
        /// USB 1 Power and Isolation Control. Reset by POR only.
        (0x00000070 => pub usb1_pwr_cntrl: Aliased<u32, Usb1PwrCntrlR::Register, Usb1PwrCntrlW::Register>),
        /// USB1 Power Status. Reset by POR only.
        (0x00000074 => pub usb1_pwr_status: ReadOnly<u32, Usb1PwrStatus::Register>),
        (0x00000078 => _padding120),
        /// RPU MPCore Power and Isolation Control. Reset by POR only.
        (0x00000080 => pub rpu_pwr_cntrl: Aliased<u32, RpuPwrCntrlR::Register, RpuPwrCntrlW::Register>),
        /// RPU MPCore Power Status. Reset by POR only.
        (0x00000084 => pub rpu_pwr_status: ReadOnly<u32, RpuPwrStatus::Register>),
        (0x00000088 => _padding136),
        /// L2 Cache Power Control. Reset by POR only.
        (0x000000b0 => pub l2_pwr_cntrl: Aliased<u32, L2PwrCntrlR::Register, L2PwrCntrlW::Register>),
        /// L2 Cache Memory Retention Controls. Reset only by POR.
        (0x000000b4 => pub l2_ret_cntrl: Aliased<u32, L2RetCntrlR::Register, L2RetCntrlW::Register>),
        /// L2 Cache Memory Chip Enables. Reset only by POR.
        (0x000000b8 => pub l2_ce_cntrl: Aliased<u32, L2CeCntrlR::Register, L2CeCntrlW::Register>),
        /// L2 Cache Memory Power Status.
        (0x000000bc => pub l2_pwr_status: ReadOnly<u32, L2PwrStatus::Register>),
        /// OCM Memory Power Control. Reset only by POR.
        (0x000000c0 => pub ocm_pwr_cntrl: Aliased<u32, OcmPwrCntrlR::Register, OcmPwrCntrlW::Register>),
        /// OCM Memory Retention Controls. Reset only by POR.
        (0x000000c4 => pub ocm_ret_cntrl: Aliased<u32, OcmRetCntrlR::Register, OcmRetCntrlW::Register>),
        /// OCM Memory Chip Enables. Reset only by POR.
        (0x000000c8 => pub ocm_ce_cntrl: Aliased<u32, OcmCeCntrlR::Register, OcmCeCntrlW::Register>),
        /// OCM Memory Power Status.
        (0x000000cc => pub ocm_pwr_status: ReadOnly<u32, OcmPwrStatus::Register>),
        /// RPU TCM Memory Power Control. Reset by POR only.
        (0x000000d0 => pub tcm_pwr_cntrl: Aliased<u32, TcmPwrCntrlR::Register, TcmPwrCntrlW::Register>),
        /// RPU TCM Retention Controls.
        (0x000000d4 => pub tcm_ret_cntrl: Aliased<u32, TcmRetCntrlR::Register, TcmRetCntrlW::Register>),
        /// RPU TCM Chip Enables.
        (0x000000d8 => pub tcm_ce_cntrl: Aliased<u32, TcmCeCntrlR::Register, TcmCeCntrlW::Register>),
        /// RPUTCM Power Switch Status.
        (0x000000dc => pub tcm_pwr_status: ReadOnly<u32, TcmPwrStatus::Register>),
        (0x000000e0 => _padding224),
        /// Isolation Wall Enable Control. Reset only by POR.
        (0x000000f0 => pub domain_iso_cntrl: Aliased<u32, DomainIsoCntrlR::Register, DomainIsoCntrlW::Register>),
        (0x000000f4 => _padding244),
        /// Power Island Status.
        (0x00000100 => pub loc_pwr_state: Aliased<u32, LocPwrStateR::Register, LocPwrStateW::Register>),
        /// RAM Retention and Processor Emulation States.
        (0x00000104 => pub loc_aux_pwr_state: Aliased<u32, LocAuxPwrStateR::Register, LocAuxPwrStateW::Register>),
        (0x00000108 => _padding264),
        /// CSU Reset Control.
        (0x00000200 => pub local_reset: Aliased<u32, LocalResetR::Register, LocalResetW::Register>),
        /// PMU Controls.
        (0x00000204 => pub local_cntrl: Aliased<u32, LocalCntrlR::Register, LocalCntrlW::Register>),
        (0x00000208 => _padding520),
        /// PMU GPO1 Output Register State.
        (0x00000214 => pub gpo1_read: ReadOnly<u32, Gpo1Read::Register>),
        /// PMU GPO2 Output Register State.
        (0x00000218 => pub gpo2_read: ReadOnly<u32, Gpo2Read::Register>),
        /// PMU GPO3 Output Register State.
        (0x0000021c => pub gpo3_read: ReadOnly<u32>),
        (0x00000220 => _padding544),
        /// Enable Events on PMU GPI1 Input Register.
        (0x00000224 => pub gpi1_enable: Aliased<u32, Gpi1EnableR::Register, Gpi1EnableW::Register>),
        /// Enable Events on PMU GPI2 Input Register.
        (0x00000228 => pub gpi2_enable: Aliased<u32, Gpi2EnableR::Register, Gpi2EnableW::Register>),
        /// Enable Events on PMU GPI3 Input Register.
        (0x0000022c => pub gpi3_enable: ReadWrite<u32>),
        (0x00000230 => _padding560),
        /// Power Supply Status and General Purpose Read-Write.
        (0x00000300 => pub boot_pwr_supply_cache: ReadWrite<u32>),
        /// PMU Boot Stage and General Purpose Read-Write.
        (0x00000304 => pub boot_stage: ReadWrite<u32>),
        /// PMU ROM code: ID Value of ROM Undefiend Service Request.
        (0x00000308 => pub last_undefined_serv: ReadWrite<u32>),
        /// PMU ROM code: ID Value of ROM Service Request.
        (0x0000030c => pub last_serv: ReadWrite<u32>),
        /// Persistent Local General Storage. Reset by POR only.
        (0x00000310 => pub pers_loc_gen_storage0: ReadWrite<u32>),
        /// Persistent Local General Storage. Reset by POR only.
        (0x00000314 => pub pers_loc_gen_storage1: ReadWrite<u32>),
        /// Persistent Local General Storage. Reset by POR only.
        (0x00000318 => pub pers_loc_gen_storage2: ReadWrite<u32>),
        /// Persistent Local General Storage. Reset by POR only.
        (0x0000031c => pub pers_loc_gen_storage3: ReadWrite<u32>),
        /// Address Error Decode Interrupt Status.
        (0x00000320 => pub addr_error_status: ReadWrite<u8, AddrErrorStatus::Register>),
        (0x00000321 => _padding801),
        /// Address Error Decode Interrupt Mask.
        (0x00000324 => pub addr_error_int_mask: ReadOnly<u8, AddrErrorIntMask::Register>),
        (0x00000325 => _padding805),
        /// Address Error Decode Interrupt Enable.
        (0x00000328 => pub addr_error_int_en: WriteOnly<u8, AddrErrorIntEn::Register>),
        (0x00000329 => _padding809),
        /// Address Error Decode Interrupt Disable.
        (0x0000032c => pub addr_error_int_dis: WriteOnly<u8>),
        (0x0000032d => _padding813),
        /// Controls the MBISR engines in the FPD.
        (0x00000330 => pub mbisr_cntrl: Aliased<u32, MbisrCntrlR::Register, MbisrCntrlW::Register>),
        /// Completion Status of MBISR engines.
        (0x00000334 => pub mbisr_status: ReadOnly<u32, MbisrStatus::Register>),
        /// Errors Detected During PMU Pre-Boot. Reset by POR only.
        (0x00000338 => pub pmu_pb_err: ReadWrite<u32, PmuPbErr::Register>),
        /// Errors Detected During PMU ROM Pre-Boot. Reset by POR only.
        (0x0000033c => pub pmu_serv_err: Aliased<u32, PmuServErrR::Register, PmuServErrW::Register>),
        /// PRDY Status Error for Power Islands in LPD. Reset only by POR.
        (0x00000340 => pub pwr_ack_err_lpd: ReadWrite<u32>),
        /// PRDY Status Error for Power Islands in FPD. Reset only by POR.
        (0x00000344 => pub pwr_ack_err_fpd: ReadWrite<u32>),
        /// Logic Clear Services Log Error Status. Reset only by POR.
        (0x00000348 => pub serv_logclr_err: ReadWrite<u32>),
        (0x0000034c => _padding844),
        /// Request to start the Logic Clear Engines.
        (0x00000350 => pub logclr_trig: WriteOnly<u32, LogclrTrig::Register>),
        /// This register provides the Acknowledge from the Logic Clear engines after they are run. (1 = Zeroization is Completed)
        (0x00000354 => pub logclr_ack: ReadOnly<u32, LogclrAck::Register>),
        (0x00000358 => _padding856),
        /// This register provides the status of the WFI state for the ACPU3-ACPU0 and the L2 Cache.
        (0x00000360 => pub apu_wfi_status: ReadOnly<u32, ApuWfiStatus::Register>),
        (0x00000364 => _padding868),
        /// This register is used to control the Reset to the MBIST Memory Controllers for PMU and CSU
        (0x0000036c => pub mbist_rst: ReadWrite<u8, MbistRst::Register>),
        (0x0000036d => _padding877),
        /// This register is used to control the PG_EN signal to the MBIST Memory Controllers for PMU and CSU
        (0x00000370 => pub mbist_pg_en: ReadWrite<u8, MbistPgEn::Register>),
        (0x00000371 => _padding881),
        /// This register is used to control the SETUP_1 signal to the MBIST Memory Controllers for PMU and CSU
        (0x00000374 => pub mbist_setup: ReadWrite<u8, MbistSetup::Register>),
        (0x00000375 => _padding885),
        /// This register is used to read the DONE status of the MBIST Memory Controllers for PMU and CSU
        (0x00000378 => pub mbist_done: ReadOnly<u8, MbistDone::Register>),
        (0x00000379 => _padding889),
        /// This register is used to read the GO status of the MBIST Memory Controllers for PMU and CSU
        (0x0000037c => pub mbist_good: ReadOnly<u8, MbistGood::Register>),
        (0x0000037d => @END),
    }
}
register_bitfields! [
    u32,
    pub Acpu0PwrCntrlR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ],
    pub Acpu0PwrCntrlW [
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Acpu0PwrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Acpu1PwrCntrlR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ],
    pub Acpu1PwrCntrlW [
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Acpu1PwrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Acpu2PwrCntrlR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ],
    pub Acpu2PwrCntrlW [
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Acpu2PwrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Acpu3PwrCntrlR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ],
    pub Acpu3PwrCntrlW [
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Acpu3PwrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pp0PwrCntrlR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ],
    pub Pp0PwrCntrlW [
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pp0PwrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pp1PwrCntrlR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ],
    pub Pp1PwrCntrlW [
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pp1PwrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Usb0PwrCntrlR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ],
    pub Usb0PwrCntrlW [
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Usb0PwrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Usb1PwrCntrlR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ],
    pub Usb1PwrCntrlW [
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Usb1PwrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub RpuPwrCntrlR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ],
    pub RpuPwrCntrlW [
        ISOLATION OFFSET(4) NUMBITS(1) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub RpuPwrStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PWR_GATES OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub L2PwrCntrlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ],
    pub L2PwrCntrlW [
        BANK0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub L2RetCntrlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ],
    pub L2RetCntrlW [
        BANK0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub L2CeCntrlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ],
    pub L2CeCntrlW [
        BANK0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub L2PwrStatus [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub OcmPwrCntrlR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        BANK3 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        BANK2 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        BANK1 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ],
    pub OcmPwrCntrlW [
        BANK3 OFFSET(24) NUMBITS(1) [],
        BANK2 OFFSET(16) NUMBITS(1) [],
        BANK1 OFFSET(8) NUMBITS(1) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub OcmRetCntrlR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        BANK3 OFFSET(3) NUMBITS(1) [],
        BANK2 OFFSET(2) NUMBITS(1) [],
        BANK1 OFFSET(1) NUMBITS(1) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ],
    pub OcmRetCntrlW [
        BANK3 OFFSET(3) NUMBITS(1) [],
        BANK2 OFFSET(2) NUMBITS(1) [],
        BANK1 OFFSET(1) NUMBITS(1) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub OcmCeCntrlR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        BANK3 OFFSET(3) NUMBITS(1) [],
        BANK2 OFFSET(2) NUMBITS(1) [],
        BANK1 OFFSET(1) NUMBITS(1) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ],
    pub OcmCeCntrlW [
        BANK3 OFFSET(3) NUMBITS(1) [],
        BANK2 OFFSET(2) NUMBITS(1) [],
        BANK1 OFFSET(1) NUMBITS(1) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub OcmPwrStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        BANK3 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        BANK2 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        BANK1 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        BANK0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TcmPwrCntrlR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TCMB1 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        TCMA1 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        TCMB0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        TCMA0 OFFSET(0) NUMBITS(1) [],
    ],
    pub TcmPwrCntrlW [
        TCMB1 OFFSET(24) NUMBITS(1) [],
        TCMA1 OFFSET(16) NUMBITS(1) [],
        TCMB0 OFFSET(8) NUMBITS(1) [],
        TCMA0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TcmRetCntrlR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        TCMB1 OFFSET(3) NUMBITS(1) [],
        TCMA1 OFFSET(2) NUMBITS(1) [],
        TCMB0 OFFSET(1) NUMBITS(1) [],
        TCMA0 OFFSET(0) NUMBITS(1) [],
    ],
    pub TcmRetCntrlW [
        TCMB1 OFFSET(3) NUMBITS(1) [],
        TCMA1 OFFSET(2) NUMBITS(1) [],
        TCMB0 OFFSET(1) NUMBITS(1) [],
        TCMA0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TcmCeCntrlR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        TCMB1 OFFSET(3) NUMBITS(1) [],
        TCMA1 OFFSET(2) NUMBITS(1) [],
        TCMB0 OFFSET(1) NUMBITS(1) [],
        TCMA0 OFFSET(0) NUMBITS(1) [],
    ],
    pub TcmCeCntrlW [
        TCMB1 OFFSET(3) NUMBITS(1) [],
        TCMA1 OFFSET(2) NUMBITS(1) [],
        TCMB0 OFFSET(1) NUMBITS(1) [],
        TCMA0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TcmPwrStatus [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TCMB1 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        TCMA1 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        TCMB0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        TCMA0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub DomainIsoCntrlR [
        LP_FP_LOCKED OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(25) [],
        FP_PL OFFSET(5) NUMBITS(1) [],
        LP_PL_PCAP OFFSET(4) NUMBITS(1) [],
        LP_PL_NON_PCAP OFFSET(3) NUMBITS(1) [],
        LP_FP_2 OFFSET(2) NUMBITS(1) [],
        LP_FP_1 OFFSET(1) NUMBITS(1) [],
        PMU OFFSET(0) NUMBITS(1) [],
    ],
    pub DomainIsoCntrlW [
        LP_FP_LOCKED OFFSET(31) NUMBITS(1) [],
        FP_PL OFFSET(5) NUMBITS(1) [],
        LP_PL_PCAP OFFSET(4) NUMBITS(1) [],
        LP_PL_NON_PCAP OFFSET(3) NUMBITS(1) [],
        LP_FP_2 OFFSET(2) NUMBITS(1) [],
        LP_FP_1 OFFSET(1) NUMBITS(1) [],
        PMU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub LocPwrStateR [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
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
        L2 OFFSET(7) NUMBITS(1) [],
        RESERVED3 OFFSET(6) NUMBITS(1) [],
        GPU_PP1 OFFSET(5) NUMBITS(1) [],
        GPU_PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ],
    pub LocPwrStateW [
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
        L2 OFFSET(7) NUMBITS(1) [],
        GPU_PP1 OFFSET(5) NUMBITS(1) [],
        GPU_PP0 OFFSET(4) NUMBITS(1) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub LocAuxPwrStateR [
        ACPU3_EMUL OFFSET(31) NUMBITS(1) [],
        ACPU2_EMUL OFFSET(30) NUMBITS(1) [],
        ACPU1_EMUL OFFSET(29) NUMBITS(1) [],
        ACPU0_EMUL OFFSET(28) NUMBITS(1) [],
        RPU_EMUL OFFSET(27) NUMBITS(1) [],
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
        L2 OFFSET(7) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(7) [],
    ],
    pub LocAuxPwrStateW [
        ACPU3_EMUL OFFSET(31) NUMBITS(1) [],
        ACPU2_EMUL OFFSET(30) NUMBITS(1) [],
        ACPU1_EMUL OFFSET(29) NUMBITS(1) [],
        ACPU0_EMUL OFFSET(28) NUMBITS(1) [],
        RPU_EMUL OFFSET(27) NUMBITS(1) [],
        OCM_BANK3 OFFSET(19) NUMBITS(1) [],
        OCM_BANK2 OFFSET(18) NUMBITS(1) [],
        OCM_BANK1 OFFSET(17) NUMBITS(1) [],
        OCM_BANK0 OFFSET(16) NUMBITS(1) [],
        TCM1B OFFSET(15) NUMBITS(1) [],
        TCM1A OFFSET(14) NUMBITS(1) [],
        TCM0B OFFSET(13) NUMBITS(1) [],
        TCM0A OFFSET(12) NUMBITS(1) [],
        L2 OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(0) NUMBITS(7) [],
    ]
];
register_bitfields! [
    u32,
    pub LocalResetR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        CSU_RST OFFSET(0) NUMBITS(1) [],
    ],
    pub LocalResetW [
        CSU_RST OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub LocalCntrlR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        ROM_VALID_OVRD OFFSET(1) NUMBITS(1) [],
        BUS_CLK_DIS OFFSET(0) NUMBITS(1) [],
    ],
    pub LocalCntrlW [
        ROM_VALID_OVRD OFFSET(1) NUMBITS(1) [],
        BUS_CLK_DIS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpo1Read [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        MIO_GPO OFFSET(0) NUMBITS(6) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpo2Read [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        DAP_RPU_WAKE_ACK OFFSET(9) NUMBITS(1) [],
        DAP_FP_WAKE_ACK OFFSET(8) NUMBITS(1) [],
        PS_STATUS OFFSET(7) NUMBITS(1) [],
        PCAP_ENABLE OFFSET(6) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(6) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpi1EnableR [
        APB_AIB_ERROR OFFSET(31) NUMBITS(1) [],
        AXI_AIB_ERROR OFFSET(30) NUMBITS(1) [],
        ERROR_REG2_INT OFFSET(29) NUMBITS(1) [],
        ERROR_REG1_INT OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        DBG_ACPU3_PWRUP_REQ OFFSET(23) NUMBITS(1) [],
        DBG_ACPU2_PWRUP_REQ OFFSET(22) NUMBITS(1) [],
        DBG_ACPU1_PWRUP_REQ OFFSET(21) NUMBITS(1) [],
        DBG_ACPU0_PWRUP_REQ OFFSET(20) NUMBITS(1) [],
        RESERVED1 OFFSET(17) NUMBITS(3) [],
        FPD_WAKE_GIC_PROX OFFSET(16) NUMBITS(1) [],
        MIO_WAKE OFFSET(10) NUMBITS(6) [],
        DAP_RPU_WAKE OFFSET(9) NUMBITS(1) [],
        DAP_FP_WAKE OFFSET(8) NUMBITS(1) [],
        USB1_WAKE OFFSET(7) NUMBITS(1) [],
        USB0_WAKE OFFSET(6) NUMBITS(1) [],
        R5_1_WAKE OFFSET(5) NUMBITS(1) [],
        R5_0_WAKE OFFSET(4) NUMBITS(1) [],
        ACPU3_WAKE OFFSET(3) NUMBITS(1) [],
        ACPU2_WAKE OFFSET(2) NUMBITS(1) [],
        ACPU1_WAKE OFFSET(1) NUMBITS(1) [],
        ACPU0_WAKE OFFSET(0) NUMBITS(1) [],
    ],
    pub Gpi1EnableW [
        APB_AIB_ERROR OFFSET(31) NUMBITS(1) [],
        AXI_AIB_ERROR OFFSET(30) NUMBITS(1) [],
        ERROR_REG2_INT OFFSET(29) NUMBITS(1) [],
        ERROR_REG1_INT OFFSET(28) NUMBITS(1) [],
        DBG_ACPU3_PWRUP_REQ OFFSET(23) NUMBITS(1) [],
        DBG_ACPU2_PWRUP_REQ OFFSET(22) NUMBITS(1) [],
        DBG_ACPU1_PWRUP_REQ OFFSET(21) NUMBITS(1) [],
        DBG_ACPU0_PWRUP_REQ OFFSET(20) NUMBITS(1) [],
        FPD_WAKE_GIC_PROX OFFSET(16) NUMBITS(1) [],
        MIO_WAKE OFFSET(10) NUMBITS(6) [],
        DAP_RPU_WAKE OFFSET(9) NUMBITS(1) [],
        DAP_FP_WAKE OFFSET(8) NUMBITS(1) [],
        USB1_WAKE OFFSET(7) NUMBITS(1) [],
        USB0_WAKE OFFSET(6) NUMBITS(1) [],
        R5_1_WAKE OFFSET(5) NUMBITS(1) [],
        R5_0_WAKE OFFSET(4) NUMBITS(1) [],
        ACPU3_WAKE OFFSET(3) NUMBITS(1) [],
        ACPU2_WAKE OFFSET(2) NUMBITS(1) [],
        ACPU1_WAKE OFFSET(1) NUMBITS(1) [],
        ACPU0_WAKE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Gpi2EnableR [
        VCC_PSINTFP_ALARM OFFSET(31) NUMBITS(1) [],
        VCC_PSINT_ALARM OFFSET(30) NUMBITS(1) [],
        VCC_PSAUX_ALARM OFFSET(29) NUMBITS(1) [],
        RESERVED0 OFFSET(24) NUMBITS(5) [],
        DBG_ACPU3_RST_REQ OFFSET(23) NUMBITS(1) [],
        DBG_ACPU2_RST_REQ OFFSET(22) NUMBITS(1) [],
        DBG_ACPU1_RST_REQ OFFSET(21) NUMBITS(1) [],
        DBG_ACPU0_RST_REQ OFFSET(20) NUMBITS(1) [],
        CP_ACPU3_RST_REQ OFFSET(19) NUMBITS(1) [],
        CP_ACPU2_RST_REQ OFFSET(18) NUMBITS(1) [],
        CP_ACPU1_RST_REQ OFFSET(17) NUMBITS(1) [],
        CP_ACPU0_RST_REQ OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        DBG_RPU1_RST_REQ OFFSET(9) NUMBITS(1) [],
        DBG_RPU0_RST_REQ OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(7) NUMBITS(1) [],
        FP_LP_PWRDWN_ACK OFFSET(6) NUMBITS(1) [],
        R5_1_PWRDWN_REQ OFFSET(5) NUMBITS(1) [],
        R5_0_PWRDWN_REQ OFFSET(4) NUMBITS(1) [],
        ACPU3_PWRDWN_REQ OFFSET(3) NUMBITS(1) [],
        ACPU2_PWRDWN_REQ OFFSET(2) NUMBITS(1) [],
        ACPU1_PWRDWN_REQ OFFSET(1) NUMBITS(1) [],
        ACPU0_PWRDWN_REQ OFFSET(0) NUMBITS(1) [],
    ],
    pub Gpi2EnableW [
        VCC_PSINTFP_ALARM OFFSET(31) NUMBITS(1) [],
        VCC_PSINT_ALARM OFFSET(30) NUMBITS(1) [],
        VCC_PSAUX_ALARM OFFSET(29) NUMBITS(1) [],
        DBG_ACPU3_RST_REQ OFFSET(23) NUMBITS(1) [],
        DBG_ACPU2_RST_REQ OFFSET(22) NUMBITS(1) [],
        DBG_ACPU1_RST_REQ OFFSET(21) NUMBITS(1) [],
        DBG_ACPU0_RST_REQ OFFSET(20) NUMBITS(1) [],
        CP_ACPU3_RST_REQ OFFSET(19) NUMBITS(1) [],
        CP_ACPU2_RST_REQ OFFSET(18) NUMBITS(1) [],
        CP_ACPU1_RST_REQ OFFSET(17) NUMBITS(1) [],
        CP_ACPU0_RST_REQ OFFSET(16) NUMBITS(1) [],
        DBG_RPU1_RST_REQ OFFSET(9) NUMBITS(1) [],
        DBG_RPU0_RST_REQ OFFSET(8) NUMBITS(1) [],
        FP_LP_PWRDWN_ACK OFFSET(6) NUMBITS(1) [],
        R5_1_PWRDWN_REQ OFFSET(5) NUMBITS(1) [],
        R5_0_PWRDWN_REQ OFFSET(4) NUMBITS(1) [],
        ACPU3_PWRDWN_REQ OFFSET(3) NUMBITS(1) [],
        ACPU2_PWRDWN_REQ OFFSET(2) NUMBITS(1) [],
        ACPU1_PWRDWN_REQ OFFSET(1) NUMBITS(1) [],
        ACPU0_PWRDWN_REQ OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub AddrErrorStatus [
        STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub AddrErrorIntMask [
        MASK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub AddrErrorIntEn [
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub MbisrCntrlR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        FPD_GROUP OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(4) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub MbisrCntrlW [
        FPD_GROUP OFFSET(5) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub MbisrStatus [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        PASS OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(3) [],
        DONE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub PmuPbErr [
        PBERR_FLAG OFFSET(31) NUMBITS(1) [],
        PBERR_DATA OFFSET(0) NUMBITS(31) [],
    ]
];
register_bitfields! [
    u32,
    pub PmuServErrR [
        FWERR OFFSET(28) NUMBITS(4) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        SERVERR_FLAG OFFSET(23) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(3) [],
        SERVERR_DATA OFFSET(0) NUMBITS(20) [],
    ],
    pub PmuServErrW [
        FWERR OFFSET(28) NUMBITS(4) [],
        SERVERR_FLAG OFFSET(23) NUMBITS(1) [],
        SERVERR_DATA OFFSET(0) NUMBITS(20) [],
    ]
];
register_bitfields! [
    u32,
    pub LogclrTrig [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        FP OFFSET(17) NUMBITS(1) [],
        LP OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        USB1 OFFSET(13) NUMBITS(1) [],
        USB0 OFFSET(12) NUMBITS(1) [],
        RESERVED2 OFFSET(11) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        RESERVED3 OFFSET(8) NUMBITS(2) [],
        PP1 OFFSET(7) NUMBITS(1) [],
        PP0 OFFSET(6) NUMBITS(1) [],
        RESERVED4 OFFSET(4) NUMBITS(2) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub LogclrAck [
        RESERVED0 OFFSET(18) NUMBITS(14) [],
        FP OFFSET(17) NUMBITS(1) [],
        LP OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        USB1 OFFSET(13) NUMBITS(1) [],
        USB0 OFFSET(12) NUMBITS(1) [],
        RESERVED2 OFFSET(11) NUMBITS(1) [],
        RPU OFFSET(10) NUMBITS(1) [],
        RESERVED3 OFFSET(8) NUMBITS(2) [],
        PP1 OFFSET(7) NUMBITS(1) [],
        PP0 OFFSET(6) NUMBITS(1) [],
        RESERVED4 OFFSET(4) NUMBITS(2) [],
        ACPU3 OFFSET(3) NUMBITS(1) [],
        ACPU2 OFFSET(2) NUMBITS(1) [],
        ACPU1 OFFSET(1) NUMBITS(1) [],
        ACPU0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ApuWfiStatus [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        L2_WFI OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(12) [],
        ACPU3_WFI OFFSET(3) NUMBITS(1) [],
        ACPU2_WFI OFFSET(2) NUMBITS(1) [],
        ACPU1_WFI OFFSET(1) NUMBITS(1) [],
        ACPU0_WFI OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub MbistRst [
        CSU OFFSET(1) NUMBITS(1) [],
        PMU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub MbistPgEn [
        CSU OFFSET(1) NUMBITS(1) [],
        PMU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub MbistSetup [
        CSU OFFSET(1) NUMBITS(1) [],
        PMU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub MbistDone [
        CSU OFFSET(1) NUMBITS(1) [],
        PMU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub MbistGood [
        CSU OFFSET(1) NUMBITS(1) [],
        PMU OFFSET(0) NUMBITS(1) [],
    ]
];

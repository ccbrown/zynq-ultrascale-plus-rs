// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// eFuse Controller, eFUSE Control
pub static mut EFUSE: *mut Registers = 0xffcc0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Write Lock
        (0x00000000 => pub wr_lock: ReadWrite<u16>),
        (0x00000002 => _padding2),
        /// Configuration
        (0x00000004 => pub cfg: ReadWrite<u32, Cfg::Register>),
        /// Status
        (0x00000008 => pub status: ReadOnly<u32, Status::Register>),
        /// eFuse Program Bit Address
        (0x0000000c => pub efuse_pgm_addr: WriteOnly<u32, EfusePgmAddr::Register>),
        /// eFuse Read Address
        (0x00000010 => pub efuse_rd_addr: WriteOnly<u32, EfuseRdAddr::Register>),
        /// eFuse Read Data
        (0x00000014 => pub efuse_rd_data: ReadOnly<u32>),
        /// Program Strobe Width
        (0x00000018 => pub tpgm: ReadWrite<u32, Tpgm::Register>),
        /// Read Strobe Width
        (0x0000001c => pub trd: ReadWrite<u32, Trd::Register>),
        /// PS to STROBE timing
        (0x00000020 => pub tsu_h_ps: ReadWrite<u32, TsuHPs::Register>),
        /// PS to CS timing
        (0x00000024 => pub tsu_h_ps_cs: ReadWrite<u32, TsuHPsCs::Register>),
        (0x00000028 => _padding40),
        /// CS to STROBE timing
        (0x0000002c => pub tsu_h_cs: ReadWrite<u32, TsuHCs::Register>),
        /// eFuse Interrupt Status
        (0x00000030 => pub efuse_isr: Aliased<u32, EfuseIsrR::Register, EfuseIsrW::Register>),
        /// eFuse Interrupt Mask Status
        (0x00000034 => pub efuse_imr: ReadOnly<u32, EfuseImr::Register>),
        /// eFuse Interrupt Enable
        (0x00000038 => pub efuse_ier: WriteOnly<u32, EfuseIer::Register>),
        /// eFuse Interrupt Disable
        (0x0000003c => pub efuse_idr: WriteOnly<u32, EfuseIdr::Register>),
        /// eFuse Cache Load
        (0x00000040 => pub efuse_cache_load: WriteOnly<u32, EfuseCacheLoad::Register>),
        /// eFuse Program Lock
        (0x00000044 => pub efuse_pgm_lock: ReadWrite<u32, EfusePgmLock::Register>),
        /// EFUSE AES Key Integrity Check
        (0x00000048 => pub efuse_aes_crc: WriteOnly<u32>),
        (0x0000004c => _padding76),
        /// Device DNA 0
        (0x0000100c => pub dna_0: ReadOnly<u32>),
        /// Device DNA 1
        (0x00001010 => pub dna_1: ReadOnly<u32>),
        /// Device DNA 2
        (0x00001014 => pub dna_2: ReadOnly<u32>),
        /// Available Functionality
        (0x00001018 => pub extended_idcode: ReadOnly<u32, ExtendedIdcode::Register>),
        /// SysOsc Clock Control
        (0x0000101c => pub sysosc_ctrl: ReadOnly<u32, SysoscCtrl::Register>),
        /// User Fuses 0
        (0x00001020 => pub user_0: ReadOnly<u32>),
        /// User Fuses 1
        (0x00001024 => pub user_1: ReadOnly<u32>),
        /// User Fuses 2
        (0x00001028 => pub user_2: ReadOnly<u32>),
        /// User Fuses 3
        (0x0000102c => pub user_3: ReadOnly<u32>),
        /// User Fuses 4
        (0x00001030 => pub user_4: ReadOnly<u32>),
        /// User Fuses 5
        (0x00001034 => pub user_5: ReadOnly<u32>),
        /// User Fuses 6
        (0x00001038 => pub user_6: ReadOnly<u32>),
        /// User Fuses 7
        (0x0000103c => pub user_7: ReadOnly<u32>),
        /// Miscellaneous User Control
        (0x00001040 => pub misc_user_ctrl: ReadOnly<u32, MiscUserCtrl::Register>),
        /// ROM Control
        (0x00001044 => pub rom_ctrl: ReadOnly<u32, RomCtrl::Register>),
        (0x00001048 => _padding4168),
        /// PUF Miscellaneous Control
        (0x00001054 => pub pmu_bootrom_ctrl: ReadOnly<u32, PmuBootromCtrl::Register>),
        /// Security Control
        (0x00001058 => pub sec_ctrl: ReadOnly<u32, SecCtrl::Register>),
        /// SPK Identification code
        (0x0000105c => pub spk_id: ReadOnly<u32>),
        (0x00001060 => _padding4192),
        /// PPK0 0
        (0x000010a0 => pub ppk0_0: ReadOnly<u32>),
        /// PPK0 1
        (0x000010a4 => pub ppk0_1: ReadOnly<u32>),
        /// PPK0 2
        (0x000010a8 => pub ppk0_2: ReadOnly<u32>),
        /// PPK0 3
        (0x000010ac => pub ppk0_3: ReadOnly<u32>),
        /// PPK0 4
        (0x000010b0 => pub ppk0_4: ReadOnly<u32>),
        /// PPK0 5
        (0x000010b4 => pub ppk0_5: ReadOnly<u32>),
        /// PPK0 6
        (0x000010b8 => pub ppk0_6: ReadOnly<u32>),
        /// PPK0 7
        (0x000010bc => pub ppk0_7: ReadOnly<u32>),
        /// PPK0 8
        (0x000010c0 => pub ppk0_8: ReadOnly<u32>),
        /// PPK0 9
        (0x000010c4 => pub ppk0_9: ReadOnly<u32>),
        /// PPK0 10
        (0x000010c8 => pub ppk0_10: ReadOnly<u32>),
        /// PPK0 11
        (0x000010cc => pub ppk0_11: ReadOnly<u32>),
        /// PPK1 0
        (0x000010d0 => pub ppk1_0: ReadOnly<u32>),
        /// PPK1 1
        (0x000010d4 => pub ppk1_1: ReadOnly<u32>),
        /// PPK1 2
        (0x000010d8 => pub ppk1_2: ReadOnly<u32>),
        /// PPK1 3
        (0x000010dc => pub ppk1_3: ReadOnly<u32>),
        /// PPK1 4
        (0x000010e0 => pub ppk1_4: ReadOnly<u32>),
        /// PPK1 5
        (0x000010e4 => pub ppk1_5: ReadOnly<u32>),
        /// PPK1 6
        (0x000010e8 => pub ppk1_6: ReadOnly<u32>),
        /// PPK1 7
        (0x000010ec => pub ppk1_7: ReadOnly<u32>),
        /// PPK1 8
        (0x000010f0 => pub ppk1_8: ReadOnly<u32>),
        /// PPK1 9
        (0x000010f4 => pub ppk1_9: ReadOnly<u32>),
        /// PPK1 10
        (0x000010f8 => pub ppk1_10: ReadOnly<u32>),
        /// PPK1 11
        (0x000010fc => pub ppk1_11: ReadOnly<u32>),
        (0x00001100 => @END),
    }
}
register_bitfields! [
    u32,
    pub Cfg [
        SLVERR_ENABLE OFFSET(5) NUMBITS(1) [],
        MARGIN_RD OFFSET(2) NUMBITS(2) [],
        PGM_EN OFFSET(1) NUMBITS(1) [],
        EFUSE_CLK_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Status [
        AES_CRC_PASS OFFSET(7) NUMBITS(1) [],
        AES_CRC_DONE OFFSET(6) NUMBITS(1) [],
        CACHE_DONE OFFSET(5) NUMBITS(1) [],
        CACHE_LOAD OFFSET(4) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        EFUSE_0_TBIT OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EfusePgmAddr [
        EFUSE OFFSET(11) NUMBITS(2) [],
        ROW OFFSET(5) NUMBITS(6) [],
        COLUMN OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u32,
    pub EfuseRdAddr [
        EFUSE OFFSET(11) NUMBITS(2) [],
        ROW OFFSET(5) NUMBITS(6) [],
    ]
];
register_bitfields! [
    u32,
    pub Tpgm [
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Trd [
        VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TsuHPs [
        VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TsuHPsCs [
        VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TsuHCs [
        VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub EfuseIsrR [
        APB_SLVERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(26) [],
        CACHE_ERROR OFFSET(4) NUMBITS(1) [],
        RD_ERROR OFFSET(3) NUMBITS(1) [],
        RD_DONE OFFSET(2) NUMBITS(1) [],
        PGM_ERROR OFFSET(1) NUMBITS(1) [],
        PGM_DONE OFFSET(0) NUMBITS(1) [],
    ],
    pub EfuseIsrW [
        APB_SLVERR OFFSET(31) NUMBITS(1) [],
        CACHE_ERROR OFFSET(4) NUMBITS(1) [],
        RD_ERROR OFFSET(3) NUMBITS(1) [],
        RD_DONE OFFSET(2) NUMBITS(1) [],
        PGM_ERROR OFFSET(1) NUMBITS(1) [],
        PGM_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EfuseImr [
        APB_SLVERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(26) [],
        CACHE_ERROR OFFSET(4) NUMBITS(1) [],
        RD_ERROR OFFSET(3) NUMBITS(1) [],
        RD_DONE OFFSET(2) NUMBITS(1) [],
        PGM_ERROR OFFSET(1) NUMBITS(1) [],
        PGM_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EfuseIer [
        APB_SLVERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(26) [],
        CACHE_ERROR OFFSET(4) NUMBITS(1) [],
        RD_ERROR OFFSET(3) NUMBITS(1) [],
        RD_DONE OFFSET(2) NUMBITS(1) [],
        PGM_ERROR OFFSET(1) NUMBITS(1) [],
        PGM_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EfuseIdr [
        APB_SLVERR OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(5) NUMBITS(26) [],
        CACHE_ERROR OFFSET(4) NUMBITS(1) [],
        RD_ERROR OFFSET(3) NUMBITS(1) [],
        RD_DONE OFFSET(2) NUMBITS(1) [],
        PGM_ERROR OFFSET(1) NUMBITS(1) [],
        PGM_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EfuseCacheLoad [
        LOAD OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EfusePgmLock [
        SPK_ID_LOCK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ExtendedIdcode [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RESERVED1 OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(14) NUMBITS(1) [],
        RESERVED3 OFFSET(13) NUMBITS(1) [],
        RESERVED4 OFFSET(12) NUMBITS(1) [],
        RESERVED5 OFFSET(11) NUMBITS(1) [],
        RESERVED6 OFFSET(10) NUMBITS(1) [],
        RESERVED7 OFFSET(9) NUMBITS(1) [],
        VCU_DIS OFFSET(8) NUMBITS(1) [],
        RESERVED8 OFFSET(7) NUMBITS(1) [],
        RESERVED9 OFFSET(6) NUMBITS(1) [],
        GPU_DIS OFFSET(5) NUMBITS(1) [],
        RESERVED10 OFFSET(4) NUMBITS(1) [],
        APU3_DIS OFFSET(3) NUMBITS(1) [],
        APU2_DIS OFFSET(2) NUMBITS(1) [],
        APU1_DIS OFFSET(1) NUMBITS(1) [],
        APU0_DIS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub SysoscCtrl [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        RESERVED2 OFFSET(3) NUMBITS(1) [],
        RESERVED3 OFFSET(2) NUMBITS(1) [],
        RESERVED4 OFFSET(1) NUMBITS(1) [],
        SYSOSC_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub MiscUserCtrl [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(28) NUMBITS(1) [],
        RESERVED4 OFFSET(27) NUMBITS(1) [],
        RESERVED5 OFFSET(26) NUMBITS(1) [],
        RESERVED6 OFFSET(25) NUMBITS(1) [],
        RESERVED7 OFFSET(24) NUMBITS(1) [],
        RESERVED8 OFFSET(23) NUMBITS(1) [],
        RESERVED9 OFFSET(22) NUMBITS(1) [],
        RESERVED10 OFFSET(21) NUMBITS(1) [],
        RESERVED11 OFFSET(20) NUMBITS(1) [],
        RESERVED12 OFFSET(19) NUMBITS(1) [],
        RESERVED13 OFFSET(18) NUMBITS(1) [],
        RESERVED14 OFFSET(17) NUMBITS(1) [],
        FPD_SC_EN_2 OFFSET(16) NUMBITS(1) [],
        FPD_SC_EN_1 OFFSET(15) NUMBITS(1) [],
        FPD_SC_EN_0 OFFSET(14) NUMBITS(1) [],
        LPD_SC_EN_2 OFFSET(13) NUMBITS(1) [],
        LPD_SC_EN_1 OFFSET(12) NUMBITS(1) [],
        LPD_SC_EN_0 OFFSET(11) NUMBITS(1) [],
        LBIST_EN OFFSET(10) NUMBITS(1) [],
        RESERVED15 OFFSET(9) NUMBITS(1) [],
        RESERVED16 OFFSET(8) NUMBITS(1) [],
        USR_WRLK_7 OFFSET(7) NUMBITS(1) [],
        USR_WRLK_6 OFFSET(6) NUMBITS(1) [],
        USR_WRLK_5 OFFSET(5) NUMBITS(1) [],
        USR_WRLK_4 OFFSET(4) NUMBITS(1) [],
        USR_WRLK_3 OFFSET(3) NUMBITS(1) [],
        USR_WRLK_2 OFFSET(2) NUMBITS(1) [],
        USR_WRLK_1 OFFSET(1) NUMBITS(1) [],
        USR_WRLK_0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub RomCtrl [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        RESERVED1 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(11) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        PBR_BOOT_ERROR OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub PmuBootromCtrl [
        REGISTER_DIS OFFSET(31) NUMBITS(1) [],
        SYN_WRLK OFFSET(30) NUMBITS(1) [],
        SYN_INVLD OFFSET(29) NUMBITS(1) [],
        TEST_DIS OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(27) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(1) [],
        RESERVED2 OFFSET(25) NUMBITS(1) [],
        RESERVED3 OFFSET(24) NUMBITS(1) [],
        AUX OFFSET(0) NUMBITS(24) [],
    ]
];
register_bitfields! [
    u32,
    pub SecCtrl [
        PPK1_INVLD OFFSET(30) NUMBITS(2) [],
        PPK1_WRLK OFFSET(29) NUMBITS(1) [],
        PPK0_INVLD OFFSET(27) NUMBITS(2) [],
        PPK0_WRLK OFFSET(26) NUMBITS(1) [],
        RSA_EN OFFSET(11) NUMBITS(15) [],
        SEC_LOCK OFFSET(10) NUMBITS(1) [],
        PROG_GATE_2 OFFSET(9) NUMBITS(1) [],
        PROG_GATE_1 OFFSET(8) NUMBITS(1) [],
        PROG_GATE_0 OFFSET(7) NUMBITS(1) [],
        DFT_DIS OFFSET(6) NUMBITS(1) [],
        JTAG_DIS OFFSET(5) NUMBITS(1) [],
        ERROR_DIS OFFSET(4) NUMBITS(1) [],
        BBRAM_DIS OFFSET(3) NUMBITS(1) [],
        ENC_ONLY OFFSET(2) NUMBITS(1) [],
        AES_WRLK OFFSET(1) NUMBITS(1) [],
        AES_RDLK OFFSET(0) NUMBITS(1) [],
    ]
];

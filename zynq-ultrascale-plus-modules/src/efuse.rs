// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// eFuse Controller, eFUSE Control
pub static mut EFUSE: *mut Registers = 0xffcc0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Write Lock
    pub wr_lock: ReadWrite<u16>,
    _padding2: [u8; 2],
    /// Configuration
    pub cfg: ReadWrite<u32, Cfg::Register>,
    /// Status
    pub status: ReadOnly<u32, Status::Register>,
    /// eFuse Program Bit Address
    pub efuse_pgm_addr: WriteOnly<u32, EfusePgmAddr::Register>,
    /// eFuse Read Address
    pub efuse_rd_addr: WriteOnly<u32, EfuseRdAddr::Register>,
    /// eFuse Read Data
    pub efuse_rd_data: ReadOnly<u32>,
    /// Program Strobe Width
    pub tpgm: ReadWrite<u32, Tpgm::Register>,
    /// Read Strobe Width
    pub trd: ReadWrite<u32, Trd::Register>,
    /// PS to STROBE timing
    pub tsu_h_ps: ReadWrite<u32, TsuHPs::Register>,
    /// PS to CS timing
    pub tsu_h_ps_cs: ReadWrite<u32, TsuHPsCs::Register>,
    _padding40: [u8; 4],
    /// CS to STROBE timing
    pub tsu_h_cs: ReadWrite<u32, TsuHCs::Register>,
    /// eFuse Interrupt Status
    pub efuse_isr: Aliased<u32, EfuseIsrR::Register, EfuseIsrW::Register>,
    /// eFuse Interrupt Mask Status
    pub efuse_imr: ReadOnly<u32, EfuseImr::Register>,
    /// eFuse Interrupt Enable
    pub efuse_ier: WriteOnly<u32, EfuseIer::Register>,
    /// eFuse Interrupt Disable
    pub efuse_idr: WriteOnly<u32, EfuseIdr::Register>,
    /// eFuse Cache Load
    pub efuse_cache_load: WriteOnly<u32, EfuseCacheLoad::Register>,
    /// eFuse Program Lock
    pub efuse_pgm_lock: ReadWrite<u32, EfusePgmLock::Register>,
    /// EFUSE AES Key Integrity Check
    pub efuse_aes_crc: WriteOnly<u32>,
    _padding76: [u8; 4032],
    /// Device DNA 0
    pub dna_0: ReadOnly<u32>,
    /// Device DNA 1
    pub dna_1: ReadOnly<u32>,
    /// Device DNA 2
    pub dna_2: ReadOnly<u32>,
    /// Available Functionality
    pub extended_idcode: ReadOnly<u32, ExtendedIdcode::Register>,
    /// SysOsc Clock Control
    pub sysosc_ctrl: ReadOnly<u32, SysoscCtrl::Register>,
    /// User Fuses 0
    pub user_0: ReadOnly<u32>,
    /// User Fuses 1
    pub user_1: ReadOnly<u32>,
    /// User Fuses 2
    pub user_2: ReadOnly<u32>,
    /// User Fuses 3
    pub user_3: ReadOnly<u32>,
    /// User Fuses 4
    pub user_4: ReadOnly<u32>,
    /// User Fuses 5
    pub user_5: ReadOnly<u32>,
    /// User Fuses 6
    pub user_6: ReadOnly<u32>,
    /// User Fuses 7
    pub user_7: ReadOnly<u32>,
    /// Miscellaneous User Control
    pub misc_user_ctrl: ReadOnly<u32, MiscUserCtrl::Register>,
    /// ROM Control
    pub rom_ctrl: ReadOnly<u32, RomCtrl::Register>,
    _padding4168: [u8; 12],
    /// PUF Miscellaneous Control
    pub pmu_bootrom_ctrl: ReadOnly<u32, PmuBootromCtrl::Register>,
    /// Security Control
    pub sec_ctrl: ReadOnly<u32, SecCtrl::Register>,
    /// SPK Identification code
    pub spk_id: ReadOnly<u32>,
    _padding4192: [u8; 64],
    /// PPK0 0
    pub ppk0_0: ReadOnly<u32>,
    /// PPK0 1
    pub ppk0_1: ReadOnly<u32>,
    /// PPK0 2
    pub ppk0_2: ReadOnly<u32>,
    /// PPK0 3
    pub ppk0_3: ReadOnly<u32>,
    /// PPK0 4
    pub ppk0_4: ReadOnly<u32>,
    /// PPK0 5
    pub ppk0_5: ReadOnly<u32>,
    /// PPK0 6
    pub ppk0_6: ReadOnly<u32>,
    /// PPK0 7
    pub ppk0_7: ReadOnly<u32>,
    /// PPK0 8
    pub ppk0_8: ReadOnly<u32>,
    /// PPK0 9
    pub ppk0_9: ReadOnly<u32>,
    /// PPK0 10
    pub ppk0_10: ReadOnly<u32>,
    /// PPK0 11
    pub ppk0_11: ReadOnly<u32>,
    /// PPK1 0
    pub ppk1_0: ReadOnly<u32>,
    /// PPK1 1
    pub ppk1_1: ReadOnly<u32>,
    /// PPK1 2
    pub ppk1_2: ReadOnly<u32>,
    /// PPK1 3
    pub ppk1_3: ReadOnly<u32>,
    /// PPK1 4
    pub ppk1_4: ReadOnly<u32>,
    /// PPK1 5
    pub ppk1_5: ReadOnly<u32>,
    /// PPK1 6
    pub ppk1_6: ReadOnly<u32>,
    /// PPK1 7
    pub ppk1_7: ReadOnly<u32>,
    /// PPK1 8
    pub ppk1_8: ReadOnly<u32>,
    /// PPK1 9
    pub ppk1_9: ReadOnly<u32>,
    /// PPK1 10
    pub ppk1_10: ReadOnly<u32>,
    /// PPK1 11
    pub ppk1_11: ReadOnly<u32>,
}
tock_registers::register_bitfields! [
    u32,
    pub Cfg [
        SLVERR_ENABLE OFFSET(5) NUMBITS(1) [],
        MARGIN_RD OFFSET(2) NUMBITS(2) [],
        PGM_EN OFFSET(1) NUMBITS(1) [],
        EFUSE_CLK_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub EfusePgmAddr [
        EFUSE OFFSET(11) NUMBITS(2) [],
        ROW OFFSET(5) NUMBITS(6) [],
        COLUMN OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EfuseRdAddr [
        EFUSE OFFSET(11) NUMBITS(2) [],
        ROW OFFSET(5) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Tpgm [
        VALUE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Trd [
        VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuHPs [
        VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuHPsCs [
        VALUE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TsuHCs [
        VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub EfuseCacheLoad [
        LOAD OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EfusePgmLock [
        SPK_ID_LOCK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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

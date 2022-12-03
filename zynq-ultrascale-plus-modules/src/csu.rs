// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// PMU Configuration Security Unit, CSU Control
pub static mut CSU: *mut Registers = 0xffca0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// CSU Status
    pub csu_status: ReadOnly<u32, CsuStatus::Register>,
    /// CSU Control
    pub csu_ctrl: ReadWrite<u32, CsuCtrl::Register>,
    /// CSU Secure Stream Switch Configuration
    pub csu_sss_cfg: ReadWrite<u32, CsuSssCfg::Register>,
    /// CSU DMA Reset
    pub csu_dma_reset: ReadWrite<u32, CsuDmaReset::Register>,
    /// Multiboot Address
    pub csu_multi_boot: ReadWrite<u32>,
    /// CSU Secure Lockdown
    pub csu_tamper_trig: WriteOnly<u32, CsuTamperTrig::Register>,
    /// CSU Secure Processor Fault Tolerant Status
    pub csu_ft_status: ReadOnly<u32, CsuFtStatus::Register>,
    _padding28: [u8; 4],
    /// CSU Interrupt Status
    pub csu_isr: ReadWrite<u32, CsuIsr::Register>,
    /// CSU Interrupt Mask
    pub csu_imr: ReadOnly<u32, CsuImr::Register>,
    /// CSU Interrupt Enable
    pub csu_ier: WriteOnly<u32, CsuIer::Register>,
    /// CSU Interrupt Disable
    pub csu_idr: WriteOnly<u32, CsuIdr::Register>,
    /// JTAG Chain Configuration
    pub jtag_chain_cfg: WriteOnly<u32, JtagChainCfg::Register>,
    /// JTAG Chain Configuration Status
    pub jtag_chain_status: ReadOnly<u32, JtagChainStatus::Register>,
    /// JTAG Security Gates
    pub jtag_sec: ReadWrite<u32, JtagSec::Register>,
    /// DAP Configuration
    pub jtag_dap_cfg: ReadWrite<u32, JtagDapCfg::Register>,
    /// Device IDCODE
    pub idcode: ReadOnly<u32>,
    /// PS Version
    pub version: ReadOnly<u32, Version::Register>,
    _padding72: [u8; 8],
    /// CSU ROM SHA-3 Digest 0
    pub csu_rom_digest_0: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 1
    pub csu_rom_digest_1: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 2
    pub csu_rom_digest_2: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 3
    pub csu_rom_digest_3: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 4
    pub csu_rom_digest_4: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 5
    pub csu_rom_digest_5: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 6
    pub csu_rom_digest_6: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 7
    pub csu_rom_digest_7: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 8
    pub csu_rom_digest_8: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 9
    pub csu_rom_digest_9: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 10
    pub csu_rom_digest_10: ReadOnly<u32>,
    /// CSU ROM SHA-3 Digest 11
    pub csu_rom_digest_11: ReadOnly<u32>,
    _padding128: [u8; 3968],
    /// AES Status
    pub aes_status: ReadOnly<u32, AesStatus::Register>,
    /// AES Key Source
    pub aes_key_src: ReadWrite<u32, AesKeySrc::Register>,
    /// AES Key Load
    pub aes_key_load: ReadWrite<u32, AesKeyLoad::Register>,
    /// AES Start Message
    pub aes_start_msg: ReadWrite<u32, AesStartMsg::Register>,
    /// AES Reset
    pub aes_reset: ReadWrite<u32, AesReset::Register>,
    /// AES Key Clear
    pub aes_key_clear: ReadWrite<u32, AesKeyClear::Register>,
    /// AES Operational Mode
    pub aes_cfg: ReadWrite<u32, AesCfg::Register>,
    /// AES KUP Write Control
    pub aes_kup_wr: ReadWrite<u32, AesKupWr::Register>,
    /// AES Key Update 0
    pub aes_kup_0: WriteOnly<u32>,
    /// AES Key Update 1
    pub aes_kup_1: WriteOnly<u32>,
    /// AES Key Update 2
    pub aes_kup_2: WriteOnly<u32>,
    /// AES Key Update 3
    pub aes_kup_3: WriteOnly<u32>,
    /// AES Key Update 4
    pub aes_kup_4: WriteOnly<u32>,
    /// AES Key Update 5
    pub aes_kup_5: WriteOnly<u32>,
    /// AES Key Update 6
    pub aes_kup_6: WriteOnly<u32>,
    /// AES Key Update 7
    pub aes_kup_7: WriteOnly<u32>,
    /// AES IV 0
    pub aes_iv_0: ReadOnly<u32>,
    /// AES IV 1
    pub aes_iv_1: ReadOnly<u32>,
    /// AES IV 2
    pub aes_iv_2: ReadOnly<u32>,
    /// AES IV 3
    pub aes_iv_3: ReadOnly<u32>,
    _padding4176: [u8; 4016],
    /// SHA Start Message
    pub sha_start: WriteOnly<u32, ShaStart::Register>,
    /// SHA Reset
    pub sha_reset: ReadWrite<u32, ShaReset::Register>,
    /// SHA Done
    pub sha_done: ReadOnly<u32, ShaDone::Register>,
    _padding8204: [u8; 4],
    /// SHA Digest 0
    pub sha_digest_0: ReadOnly<u32>,
    /// SHA Digest 1
    pub sha_digest_1: ReadOnly<u32>,
    /// SHA Digest 2
    pub sha_digest_2: ReadOnly<u32>,
    /// SHA Digest 3
    pub sha_digest_3: ReadOnly<u32>,
    /// SHA Digest 4
    pub sha_digest_4: ReadOnly<u32>,
    /// SHA Digest 5
    pub sha_digest_5: ReadOnly<u32>,
    /// SHA Digest 6
    pub sha_digest_6: ReadOnly<u32>,
    /// SHA Digest 7
    pub sha_digest_7: ReadOnly<u32>,
    /// SHA Digest 8
    pub sha_digest_8: ReadOnly<u32>,
    /// SHA Digest 9
    pub sha_digest_9: ReadOnly<u32>,
    /// SHA Digest 10
    pub sha_digest_10: ReadOnly<u32>,
    /// SHA Digest 11
    pub sha_digest_11: ReadOnly<u32>,
    _padding8256: [u8; 4032],
    /// PCAP PROG
    pub pcap_prog: ReadWrite<u32, PcapProg::Register>,
    /// PCAP Read Write Control
    pub pcap_rdwr: ReadWrite<u32, PcapRdwr::Register>,
    /// PCAP Control
    pub pcap_ctrl: ReadWrite<u32, PcapCtrl::Register>,
    /// PCAP Reset
    pub pcap_reset: ReadWrite<u32, PcapReset::Register>,
    /// PCAP Status
    pub pcap_status: ReadOnly<u32, PcapStatus::Register>,
    _padding12308: [u8; 4076],
    /// PUF Command
    pub puf_cmd: ReadWrite<u32, PufCmd::Register>,
    /// PUF Configuration 0
    pub puf_cfg0: ReadWrite<u32>,
    /// PUF Configuration 1
    pub puf_cfg1: ReadWrite<u32>,
    /// PUF Configuration 2
    pub puf_shut: ReadWrite<u32, PufShut::Register>,
    /// PUF Status
    pub puf_status: ReadOnly<u32, PufStatus::Register>,
    /// Debug
    pub puf_dbg: ReadWrite<u32>,
    /// PUF Word
    pub puf_word: ReadOnly<u32>,
    _padding16412: [u8; 2024],
    /// PUF Testmode Status
    pub puf_tm_status: ReadOnly<u32, PufTmStatus::Register>,
    /// PUF Testmode 1 Upper Limit
    pub puf_tm_ul: ReadWrite<u32, PufTmUl::Register>,
    /// PUF Testmode 1 Lower Limit
    pub puf_tm_ll: ReadWrite<u32, PufTmLl::Register>,
    /// PUF Testmode 1 Sample Window
    pub puf_tm_sw: ReadWrite<u32, PufTmSw::Register>,
    /// PUF Testmode Test Results
    pub puf_tm_tr: ReadWrite<u32, PufTmTr::Register>,
    _padding18456: [u8; 2024],
    /// Tamper Response Status
    pub tamper_status: ReadWrite<u32, TamperStatus::Register>,
    /// CSU Tamper Response 0, CSU Register.
    pub csu_tamper_0: ReadWrite<u32, CsuTamper0::Register>,
    /// CSU Tamper Response 1, MIO Signal Pin.
    pub csu_tamper_1: ReadWrite<u32, CsuTamper1::Register>,
    /// CSU Tamper Response 2, PS JTAG.
    pub csu_tamper_2: ReadWrite<u32, CsuTamper2::Register>,
    /// CSU Tamper Response 3, PL SEU or Error.
    pub csu_tamper_3: ReadWrite<u32, CsuTamper3::Register>,
    /// CSU Tamper Response 4, LPD Over Temp.
    pub csu_tamper_4: ReadWrite<u32, CsuTamper4::Register>,
    /// CSU Tamper Response 5, FPD Over Temp.
    pub csu_tamper_5: ReadWrite<u32, CsuTamper5::Register>,
    /// CSU Tamper Response 6, LPD Voltage.
    pub csu_tamper_6: ReadWrite<u32, CsuTamper6::Register>,
    /// CSU Tamper Response 7, FPD Voltage.
    pub csu_tamper_7: ReadWrite<u32, CsuTamper7::Register>,
    /// CSU Tamper Response 8, PS Auxiliary Voltage.
    pub csu_tamper_8: ReadWrite<u32, CsuTamper8::Register>,
    /// CSU Tamper Response 9, DDR Controller Voltage.
    pub csu_tamper_9: ReadWrite<u32, CsuTamper9::Register>,
    /// CSU Tamper Response 10, MIO Voltage Banks.
    pub csu_tamper_10: ReadWrite<u32, CsuTamper10::Register>,
    /// CSU Tamper Response 11, PS Dedicated Pins Voltage.
    pub csu_tamper_11: ReadWrite<u32, CsuTamper11::Register>,
    /// CSU Tamper Response 12, PS GTR VCC or VTT Voltage.
    pub csu_tamper_12: ReadWrite<u32, CsuTamper12::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub CsuStatus [
        UNUSED OFFSET(2) NUMBITS(30) [],
        BOOT_ENC OFFSET(1) NUMBITS(1) [],
        BOOT_AUTH OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuCtrl [
        SLVERR_ENABLE OFFSET(4) NUMBITS(1) [],
        RESERVED0 OFFSET(1) NUMBITS(3) [],
        CSU_CLK_SEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuSssCfg [
        RESERVED0 OFFSET(16) NUMBITS(4) [],
        SHA_SSS OFFSET(12) NUMBITS(4) [],
        AES_SSS OFFSET(8) NUMBITS(4) [],
        DMA_SSS OFFSET(4) NUMBITS(4) [],
        PCAP_SSS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuDmaReset [
        RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamperTrig [
        TAMPER OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuFtStatus [
        R_UE OFFSET(31) NUMBITS(1) [],
        R_VOTER_ERROR OFFSET(30) NUMBITS(1) [],
        R_COMP_ERR_23 OFFSET(29) NUMBITS(1) [],
        R_COMP_ERR_13 OFFSET(28) NUMBITS(1) [],
        R_COMP_ERR_12 OFFSET(27) NUMBITS(1) [],
        R_MISMATCH_23_A OFFSET(26) NUMBITS(1) [],
        R_MISMATCH_13_A OFFSET(25) NUMBITS(1) [],
        R_MISMATCH_12_A OFFSET(24) NUMBITS(1) [],
        R_FT_ST_MISMATCH OFFSET(23) NUMBITS(1) [],
        R_CPU_ID_MISMATCH OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(2) [],
        R_SLEEP_RESET OFFSET(19) NUMBITS(1) [],
        R_MISMATCH_23_B OFFSET(18) NUMBITS(1) [],
        R_MISMATCH_13_B OFFSET(17) NUMBITS(1) [],
        R_MISMATCH_12_B OFFSET(16) NUMBITS(1) [],
        N_UE OFFSET(15) NUMBITS(1) [],
        N_VOTER_ERROR OFFSET(14) NUMBITS(1) [],
        N_COMP_ERR_23 OFFSET(13) NUMBITS(1) [],
        N_COMP_ERR_13 OFFSET(12) NUMBITS(1) [],
        N_COMP_ERR_12 OFFSET(11) NUMBITS(1) [],
        N_MISMATCH_23_A OFFSET(10) NUMBITS(1) [],
        N_MISMATCH_13_A OFFSET(9) NUMBITS(1) [],
        N_MISMATCH_12_A OFFSET(8) NUMBITS(1) [],
        N_FT_ST_MISMATCH OFFSET(7) NUMBITS(1) [],
        N_CPU_ID_MISMATCH OFFSET(6) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(2) [],
        N_SLEEP_RESET OFFSET(3) NUMBITS(1) [],
        N_MISMATCH_23_B OFFSET(2) NUMBITS(1) [],
        N_MISMATCH_13_B OFFSET(1) NUMBITS(1) [],
        N_MISMATCH_12_B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuIsr [
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
        RESERVED15 OFFSET(16) NUMBITS(1) [],
        CSU_PL_ISO OFFSET(15) NUMBITS(1) [],
        CSU_RAM_ECC_ERROR OFFSET(14) NUMBITS(1) [],
        TAMPER OFFSET(13) NUMBITS(1) [],
        RESERVED16 OFFSET(12) NUMBITS(1) [],
        APB_SLVERR OFFSET(11) NUMBITS(1) [],
        TMR_FATAL OFFSET(10) NUMBITS(1) [],
        PL_SEU_ERROR OFFSET(9) NUMBITS(1) [],
        AES_ERROR OFFSET(8) NUMBITS(1) [],
        PCAP_WR_OVERFLOW OFFSET(7) NUMBITS(1) [],
        PCAP_RD_OVERFLOW OFFSET(6) NUMBITS(1) [],
        PL_POR_B OFFSET(5) NUMBITS(1) [],
        PL_INIT OFFSET(4) NUMBITS(1) [],
        PL_DONE OFFSET(3) NUMBITS(1) [],
        SHA_DONE OFFSET(2) NUMBITS(1) [],
        RSA_DONE OFFSET(1) NUMBITS(1) [],
        AES_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuImr [
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
        RESERVED15 OFFSET(16) NUMBITS(1) [],
        CSU_PL_ISO OFFSET(15) NUMBITS(1) [],
        CSU_RAM_ECC_ERROR OFFSET(14) NUMBITS(1) [],
        TAMPER OFFSET(13) NUMBITS(1) [],
        RESERVED16 OFFSET(12) NUMBITS(1) [],
        APB_SLVERR OFFSET(11) NUMBITS(1) [],
        TMR_FATAL OFFSET(10) NUMBITS(1) [],
        PL_SEU_ERROR OFFSET(9) NUMBITS(1) [],
        AES_ERROR OFFSET(8) NUMBITS(1) [],
        PCAP_WR_OVERFLOW OFFSET(7) NUMBITS(1) [],
        PCAP_RD_OVERFLOW OFFSET(6) NUMBITS(1) [],
        PL_POR_B OFFSET(5) NUMBITS(1) [],
        PL_INIT OFFSET(4) NUMBITS(1) [],
        PL_DONE OFFSET(3) NUMBITS(1) [],
        SHA_DONE OFFSET(2) NUMBITS(1) [],
        RSA_DONE OFFSET(1) NUMBITS(1) [],
        AES_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuIer [
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
        RESERVED15 OFFSET(16) NUMBITS(1) [],
        CSU_PL_ISO OFFSET(15) NUMBITS(1) [],
        CSU_RAM_ECC_ERROR OFFSET(14) NUMBITS(1) [],
        TAMPER OFFSET(13) NUMBITS(1) [],
        RESERVED16 OFFSET(12) NUMBITS(1) [],
        APB_SLVERR OFFSET(11) NUMBITS(1) [],
        TMR_FATAL OFFSET(10) NUMBITS(1) [],
        PL_SEU_ERROR OFFSET(9) NUMBITS(1) [],
        AES_ERROR OFFSET(8) NUMBITS(1) [],
        PCAP_WR_OVERFLOW OFFSET(7) NUMBITS(1) [],
        PCAP_RD_OVERFLOW OFFSET(6) NUMBITS(1) [],
        PL_POR_B OFFSET(5) NUMBITS(1) [],
        PL_INIT OFFSET(4) NUMBITS(1) [],
        PL_DONE OFFSET(3) NUMBITS(1) [],
        SHA_DONE OFFSET(2) NUMBITS(1) [],
        RSA_DONE OFFSET(1) NUMBITS(1) [],
        AES_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuIdr [
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
        RESERVED15 OFFSET(16) NUMBITS(1) [],
        CSU_PL_ISO OFFSET(15) NUMBITS(1) [],
        CSU_RAM_ECC_ERROR OFFSET(14) NUMBITS(1) [],
        TAMPER OFFSET(13) NUMBITS(1) [],
        RESERVED16 OFFSET(12) NUMBITS(1) [],
        APB_SLVERR OFFSET(11) NUMBITS(1) [],
        TMR_FATAL OFFSET(10) NUMBITS(1) [],
        PL_SEU_ERROR OFFSET(9) NUMBITS(1) [],
        AES_ERROR OFFSET(8) NUMBITS(1) [],
        PCAP_WR_OVERFLOW OFFSET(7) NUMBITS(1) [],
        PCAP_RD_OVERFLOW OFFSET(6) NUMBITS(1) [],
        PL_POR_B OFFSET(5) NUMBITS(1) [],
        PL_INIT OFFSET(4) NUMBITS(1) [],
        PL_DONE OFFSET(3) NUMBITS(1) [],
        SHA_DONE OFFSET(2) NUMBITS(1) [],
        RSA_DONE OFFSET(1) NUMBITS(1) [],
        AES_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub JtagChainCfg [
        SSSS_LINK_ARM_DAP OFFSET(1) NUMBITS(1) [],
        SSSS_LINK_PL_TAP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub JtagChainStatus [
        ARM_DAP OFFSET(1) NUMBITS(1) [],
        PL_TAP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub JtagSec [
        RESERVED0 OFFSET(12) NUMBITS(3) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        SSSS_PMU_SEC OFFSET(6) NUMBITS(3) [],
        SSSS_PLTAP_SEC OFFSET(3) NUMBITS(3) [],
        SSSS_DAP_SEC OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub JtagDapCfg [
        RESERVED0 OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        SSSS_RPU_NIDEN OFFSET(5) NUMBITS(1) [],
        SSSS_RPU_DBGEN OFFSET(4) NUMBITS(1) [],
        SSSS_APU_SPNIDEN OFFSET(3) NUMBITS(1) [],
        SSSS_APU_SPIDEN OFFSET(2) NUMBITS(1) [],
        SSSS_APU_NIDEN OFFSET(1) NUMBITS(1) [],
        SSSS_APU_DBGEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Version [
        RESERVED0 OFFSET(16) NUMBITS(4) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        RESERVED2 OFFSET(4) NUMBITS(8) [],
        PS_VERSION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AesStatus [
        OKR_ZEROED OFFSET(11) NUMBITS(1) [],
        BOOT_ZEROED OFFSET(10) NUMBITS(1) [],
        KUP_ZEROED OFFSET(9) NUMBITS(1) [],
        AES_KEY_ZEROED OFFSET(8) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(2) [],
        RESERVED1 OFFSET(5) NUMBITS(1) [],
        KEY_INIT_DONE OFFSET(4) NUMBITS(1) [],
        GCM_TAG_PASS OFFSET(3) NUMBITS(1) [],
        DONE OFFSET(2) NUMBITS(1) [],
        READY OFFSET(1) NUMBITS(1) [],
        BUSY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AesKeySrc [
        KEY_SRC OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AesKeyLoad [
        KEY_LOAD OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AesStartMsg [
        START_MSG OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AesReset [
        RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AesKeyClear [
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        AES_KUP_ZERO OFFSET(1) NUMBITS(1) [],
        AES_KEY_ZERO OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AesCfg [
        ENCRYPT_DECRYPT_N OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AesKupWr [
        IV_WRITE OFFSET(1) NUMBITS(1) [],
        KUP_WRITE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ShaStart [
        START_MSG OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ShaReset [
        RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ShaDone [
        SHA_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcapProg [
        PCFG_PROG_B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcapRdwr [
        PCAP_RDWR_B OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcapCtrl [
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        PCFG_POR_CNT_4K OFFSET(1) NUMBITS(1) [],
        PCAP_PR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcapReset [
        RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcapStatus [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        RESERVED2 OFFSET(29) NUMBITS(1) [],
        RESERVED3 OFFSET(14) NUMBITS(15) [],
        PCFG_GWE OFFSET(13) NUMBITS(1) [],
        PCFG_MCAP_MODE OFFSET(12) NUMBITS(1) [],
        PL_GTS_USR_B OFFSET(11) NUMBITS(1) [],
        PL_GTS_CFG_B OFFSET(10) NUMBITS(1) [],
        PL_GPWRDWN_B OFFSET(9) NUMBITS(1) [],
        PL_GHIGH_B OFFSET(8) NUMBITS(1) [],
        PL_FST_CFG OFFSET(7) NUMBITS(1) [],
        PL_CFG_RESET_B OFFSET(6) NUMBITS(1) [],
        PL_SEU_ERROR OFFSET(5) NUMBITS(1) [],
        PL_EOS OFFSET(4) NUMBITS(1) [],
        PL_DONE OFFSET(3) NUMBITS(1) [],
        PL_INIT OFFSET(2) NUMBITS(1) [],
        PCAP_RD_IDLE OFFSET(1) NUMBITS(1) [],
        PCAP_WR_IDLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PufCmd [
        CMD OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PufShut [
        SOSET OFFSET(24) NUMBITS(8) [],
        SOPEN OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PufStatus [
        OVERFLOW OFFSET(28) NUMBITS(2) [],
        AUX OFFSET(4) NUMBITS(24) [],
        KEY_RDY OFFSET(3) NUMBITS(1) [],
        RESERVED0 OFFSET(2) NUMBITS(1) [],
        KEY_ZERO OFFSET(1) NUMBITS(1) [],
        SYN_WRD_RDY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PufTmStatus [
        DN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PufTmUl [
        UL OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PufTmLl [
        LL OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PufTmSw [
        OFF OFFSET(24) NUMBITS(8) [],
        SW OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PufTmTr [
        US OFFSET(24) NUMBITS(2) [],
        ER OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(1) NUMBITS(15) [],
        FRR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TamperStatus [
        RESERVED0 OFFSET(13) NUMBITS(1) [],
        TAMPER_12 OFFSET(12) NUMBITS(1) [],
        TAMPER_11 OFFSET(11) NUMBITS(1) [],
        TAMPER_10 OFFSET(10) NUMBITS(1) [],
        TAMPER_9 OFFSET(9) NUMBITS(1) [],
        TAMPER_8 OFFSET(8) NUMBITS(1) [],
        TAMPER_7 OFFSET(7) NUMBITS(1) [],
        TAMPER_6 OFFSET(6) NUMBITS(1) [],
        TAMPER_5 OFFSET(5) NUMBITS(1) [],
        TAMPER_4 OFFSET(4) NUMBITS(1) [],
        TAMPER_3 OFFSET(3) NUMBITS(1) [],
        TAMPER_2 OFFSET(2) NUMBITS(1) [],
        TAMPER_1 OFFSET(1) NUMBITS(1) [],
        TAMPER_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper0 [
        BBRAM_ERASE OFFSET(5) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper1 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper2 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper3 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper4 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper5 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper6 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper7 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper8 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper9 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper10 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper11 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CsuTamper12 [
        BBRAM_ERASE OFFSET(4) NUMBITS(1) [],
        SEC_LOCKDOWN_1 OFFSET(3) NUMBITS(1) [],
        SEC_LOCKDOWN_0 OFFSET(2) NUMBITS(1) [],
        SYS_RESET OFFSET(1) NUMBITS(1) [],
        SYS_INTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];

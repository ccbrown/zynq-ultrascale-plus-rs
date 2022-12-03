// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly};
/// XPPU Protection Unit, XPPU Configuration
pub static mut LPD_XPPU: *mut Registers = 0xff980000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Control.
    pub ctrl: Aliased<u32, CtrlR::Register, CtrlW::Register>,
    /// Error Status, Reg 1. Poisoned address.
    pub err_status1: ReadOnly<u32, ErrStatus1::Register>,
    /// Error Status, Reg 2. Poisoned Master ID.
    pub err_status2: ReadOnly<u32, ErrStatus2::Register>,
    /// Poison Address for Sink.
    pub poison: ReadOnly<u32, Poison::Register>,
    /// Interrupt Status and Clear.
    pub isr: Aliased<u32, IsrR::Register, IsrW::Register>,
    /// Interrupt Mask.
    pub imr: ReadOnly<u32, Imr::Register>,
    /// Interrupt Enable.
    pub ien: Aliased<u32, IenR::Register, IenW::Register>,
    /// Interrupt Disable.
    pub ids: Aliased<u32, IdsR::Register, IdsW::Register>,
    _padding32: [u8; 28],
    /// Number of Master IDs Implemented.
    pub m_master_ids: ReadOnly<u32>,
    /// Number of 32-Byte Apertures Implemented for IPI.
    pub m_aperture_32b: ReadOnly<u32>,
    /// Number of 64-kB Apertures Implemented for IOP CSRs.
    pub m_aperture_64kb: ReadOnly<u32>,
    /// Number of 1-MB Apertures Implemented for Memory.
    pub m_aperture_1mb: ReadOnly<u32>,
    /// Number of 512-MB Apertures Implemented for QSPI.
    pub m_aperture_512mb: ReadOnly<u32>,
    /// Base Address of 32-byte Apertures; fixed value.
    pub base_32b: ReadOnly<u32>,
    /// Base Address of 64-kB Apertures; fixed value.
    pub base_64kb: ReadOnly<u32>,
    /// Base Address of 1-MB Apertures; fixed value.
    pub base_1mb: ReadOnly<u32>,
    /// Base Address of 512-MB Apertures; fixed value.
    pub base_512mb: ReadOnly<u32>,
    _padding96: [u8; 160],
    /// Master Profile 0. Predefined for PMU.
    pub master_id00: Aliased<u32, MasterId00R::Register, MasterId00W::Register>,
    /// Master Profile 1. Predefined for RPU0.
    pub master_id01: Aliased<u32, MasterId01R::Register, MasterId01W::Register>,
    /// Master Profile 2. Predefined for RPU1.
    pub master_id02: Aliased<u32, MasterId02R::Register, MasterId02W::Register>,
    /// Master Profile 3. Predefined for any APU.
    pub master_id03: Aliased<u32, MasterId03R::Register, MasterId03W::Register>,
    /// Master Profile 4. Predefined for APU0.
    pub master_id04: Aliased<u32, MasterId04R::Register, MasterId04W::Register>,
    /// Master Profile 5. Predefined for APU1.
    pub master_id05: Aliased<u32, MasterId05R::Register, MasterId05W::Register>,
    /// Master Profile 6. Predefined for APU2.
    pub master_id06: Aliased<u32, MasterId06R::Register, MasterId06W::Register>,
    /// Master Profile 7. Predefined for APU3
    pub master_id07: Aliased<u32, MasterId07R::Register, MasterId07W::Register>,
    /// Master Profile 8. Any master; programmable.
    pub master_id08: Aliased<u32, MasterId08R::Register, MasterId08W::Register>,
    /// Master Profile 9. Any master; programmable.
    pub master_id09: Aliased<u32, MasterId09R::Register, MasterId09W::Register>,
    /// Master Profile 10. Any master; programmable.
    pub master_id10: Aliased<u32, MasterId10R::Register, MasterId10W::Register>,
    /// Master Profile 11. Any master; programmable.
    pub master_id11: Aliased<u32, MasterId11R::Register, MasterId11W::Register>,
    /// Master Profile 12. Any master; programmable.
    pub master_id12: Aliased<u32, MasterId12R::Register, MasterId12W::Register>,
    /// Master Profile 13. Any master; programmable.
    pub master_id13: Aliased<u32, MasterId13R::Register, MasterId13W::Register>,
    /// Master Profile 14. Any master; programmable.
    pub master_id14: Aliased<u32, MasterId14R::Register, MasterId14W::Register>,
    /// Master Profile 15. Any master; programmable.
    pub master_id15: Aliased<u32, MasterId15R::Register, MasterId15W::Register>,
    /// Master Profile 16. Any master; programmable.
    pub master_id16: Aliased<u32, MasterId16R::Register, MasterId16W::Register>,
    /// Master Profile 17. Any master; programmable.
    pub master_id17: Aliased<u32, MasterId17R::Register, MasterId17W::Register>,
    /// Master Profile 18. Any master; programmable.
    pub master_id18: Aliased<u32, MasterId18R::Register, MasterId18W::Register>,
    /// Master Profile 19. Any master; programmable.
    pub master_id19: Aliased<u32, MasterId19R::Register, MasterId19W::Register>,
    _padding336: [u8; 3760],
    /// Aperture 000. IOP Reg 64KB page at 0xFF00_0000.
    pub aperperm_000: Aliased<u32, Aperperm000R::Register, Aperperm000W::Register>,
    /// Aperture 001. IOP Reg 64KB page at 0xFF01_0000.
    pub aperperm_001: Aliased<u32, Aperperm001R::Register, Aperperm001W::Register>,
    /// Aperture 002. IOP Reg 64KB page at 0xFF02_0000.
    pub aperperm_002: Aliased<u32, Aperperm002R::Register, Aperperm002W::Register>,
    /// Aperture 003. IOP Reg 64KB page at 0xFF03_0000.
    pub aperperm_003: Aliased<u32, Aperperm003R::Register, Aperperm003W::Register>,
    /// Aperture 004. IOP Reg 64KB page at 0xFF04_0000.
    pub aperperm_004: Aliased<u32, Aperperm004R::Register, Aperperm004W::Register>,
    /// Aperture 005. IOP Reg 64KB page at 0xFF05_0000.
    pub aperperm_005: Aliased<u32, Aperperm005R::Register, Aperperm005W::Register>,
    /// Aperture 006. IOP Reg 64KB page at 0xFF06_0000.
    pub aperperm_006: Aliased<u32, Aperperm006R::Register, Aperperm006W::Register>,
    /// Aperture 007. IOP Reg 64KB page at 0xFF07_0000.
    pub aperperm_007: Aliased<u32, Aperperm007R::Register, Aperperm007W::Register>,
    /// Aperture 008. IOP Reg 64KB page at 0xFF08_0000.
    pub aperperm_008: Aliased<u32, Aperperm008R::Register, Aperperm008W::Register>,
    /// Aperture 009. IOP Reg 64KB page at 0xFF09_0000.
    pub aperperm_009: Aliased<u32, Aperperm009R::Register, Aperperm009W::Register>,
    /// Aperture 010. IOP Reg 64KB page at 0xFF0A_0000.
    pub aperperm_010: Aliased<u32, Aperperm010R::Register, Aperperm010W::Register>,
    /// Aperture 011. IOP Reg 64KB page at 0xFF0B_0000.
    pub aperperm_011: Aliased<u32, Aperperm011R::Register, Aperperm011W::Register>,
    /// Aperture 012. IOP Reg 64KB page at 0xFF0C_0000.
    pub aperperm_012: Aliased<u32, Aperperm012R::Register, Aperperm012W::Register>,
    /// Aperture 013. IOP Reg 64KB page at 0xFF0D_0000.
    pub aperperm_013: Aliased<u32, Aperperm013R::Register, Aperperm013W::Register>,
    /// Aperture 014. IOP Reg 64KB page at 0xFF0E_0000.
    pub aperperm_014: Aliased<u32, Aperperm014R::Register, Aperperm014W::Register>,
    /// Aperture 015. IOP Reg 64KB page at 0xFF0F_0000.
    pub aperperm_015: Aliased<u32, Aperperm015R::Register, Aperperm015W::Register>,
    /// Aperture 016. IOP Reg 64KB page at 0xFF10_0000.
    pub aperperm_016: Aliased<u32, Aperperm016R::Register, Aperperm016W::Register>,
    /// Aperture 017. IOP Reg 64KB page at 0xFF11_0000.
    pub aperperm_017: Aliased<u32, Aperperm017R::Register, Aperperm017W::Register>,
    /// Aperture 018. IOP Reg 64KB page at 0xFF12_0000.
    pub aperperm_018: Aliased<u32, Aperperm018R::Register, Aperperm018W::Register>,
    /// Aperture 019. IOP Reg 64KB page at 0xFF13_0000.
    pub aperperm_019: Aliased<u32, Aperperm019R::Register, Aperperm019W::Register>,
    /// Aperture 020. IOP Reg 64KB page at 0xFF14_0000.
    pub aperperm_020: Aliased<u32, Aperperm020R::Register, Aperperm020W::Register>,
    /// Aperture 021. IOP Reg 64KB page at 0xFF15_0000.
    pub aperperm_021: Aliased<u32, Aperperm021R::Register, Aperperm021W::Register>,
    /// Aperture 022. IOP Reg 64KB page at 0xFF16_0000.
    pub aperperm_022: Aliased<u32, Aperperm022R::Register, Aperperm022W::Register>,
    /// Aperture 023. IOP Reg 64KB page at 0xFF17_0000.
    pub aperperm_023: Aliased<u32, Aperperm023R::Register, Aperperm023W::Register>,
    /// Aperture 024. IOP Reg 64KB page at 0xFF18_0000.
    pub aperperm_024: Aliased<u32, Aperperm024R::Register, Aperperm024W::Register>,
    /// Aperture 025. IOP Reg 64KB page at 0xFF19_0000.
    pub aperperm_025: Aliased<u32, Aperperm025R::Register, Aperperm025W::Register>,
    /// Aperture 026. IOP Reg 64KB page at 0xFF1A_0000.
    pub aperperm_026: Aliased<u32, Aperperm026R::Register, Aperperm026W::Register>,
    /// Aperture 027. IOP Reg 64KB page at 0xFF1B_0000.
    pub aperperm_027: Aliased<u32, Aperperm027R::Register, Aperperm027W::Register>,
    /// Aperture 028. IOP Reg 64KB page at 0xFF1C_0000.
    pub aperperm_028: Aliased<u32, Aperperm028R::Register, Aperperm028W::Register>,
    /// Aperture 029. IOP Reg 64KB page at 0xFF1D_0000.
    pub aperperm_029: Aliased<u32, Aperperm029R::Register, Aperperm029W::Register>,
    /// Aperture 030. IOP Reg 64KB page at 0xFF1E_0000.
    pub aperperm_030: Aliased<u32, Aperperm030R::Register, Aperperm030W::Register>,
    /// Aperture 031. IOP Reg 64KB page at 0xFF1F_0000.
    pub aperperm_031: Aliased<u32, Aperperm031R::Register, Aperperm031W::Register>,
    /// Aperture 032. IOP Reg 64KB page at 0xFF20_0000.
    pub aperperm_032: Aliased<u32, Aperperm032R::Register, Aperperm032W::Register>,
    /// Aperture 033. IOP Reg 64KB page at 0xFF21_0000.
    pub aperperm_033: Aliased<u32, Aperperm033R::Register, Aperperm033W::Register>,
    /// Aperture 034. IOP Reg 64KB page at 0xFF22_0000.
    pub aperperm_034: Aliased<u32, Aperperm034R::Register, Aperperm034W::Register>,
    /// Aperture 035. IOP Reg 64KB page at 0xFF23_0000.
    pub aperperm_035: Aliased<u32, Aperperm035R::Register, Aperperm035W::Register>,
    /// Aperture 036. IOP Reg 64KB page at 0xFF24_0000.
    pub aperperm_036: Aliased<u32, Aperperm036R::Register, Aperperm036W::Register>,
    /// Aperture 037. IOP Reg 64KB page at 0xFF25_0000.
    pub aperperm_037: Aliased<u32, Aperperm037R::Register, Aperperm037W::Register>,
    /// Aperture 038. IOP Reg 64KB page at 0xFF26_0000.
    pub aperperm_038: Aliased<u32, Aperperm038R::Register, Aperperm038W::Register>,
    /// Aperture 039. IOP Reg 64KB page at 0xFF27_0000.
    pub aperperm_039: Aliased<u32, Aperperm039R::Register, Aperperm039W::Register>,
    /// Aperture 040. IOP Reg 64KB page at 0xFF28_0000.
    pub aperperm_040: Aliased<u32, Aperperm040R::Register, Aperperm040W::Register>,
    /// Aperture 041. IOP Reg 64KB page at 0xFF29_0000.
    pub aperperm_041: Aliased<u32, Aperperm041R::Register, Aperperm041W::Register>,
    /// Aperture 042. IOP Reg 64KB page at 0xFF2A_0000.
    pub aperperm_042: Aliased<u32, Aperperm042R::Register, Aperperm042W::Register>,
    /// Aperture 043. IOP Reg 64KB page at 0xFF2B_0000.
    pub aperperm_043: Aliased<u32, Aperperm043R::Register, Aperperm043W::Register>,
    /// Aperture 044. IOP Reg 64KB page at 0xFF2C_0000.
    pub aperperm_044: Aliased<u32, Aperperm044R::Register, Aperperm044W::Register>,
    /// Aperture 045. IOP Reg 64KB page at 0xFF2D_0000.
    pub aperperm_045: Aliased<u32, Aperperm045R::Register, Aperperm045W::Register>,
    /// Aperture 046. IOP Reg 64KB page at 0xFF2E_0000.
    pub aperperm_046: Aliased<u32, Aperperm046R::Register, Aperperm046W::Register>,
    /// Aperture 047. IOP Reg 64KB page at 0xFF2F_0000.
    pub aperperm_047: Aliased<u32, Aperperm047R::Register, Aperperm047W::Register>,
    /// Aperture 048. IOP Reg 64KB page at 0xFF30_0000.
    pub aperperm_048: Aliased<u32, Aperperm048R::Register, Aperperm048W::Register>,
    /// Aperture 049. IOP Reg 64KB page at 0xFF31_0000.
    pub aperperm_049: Aliased<u32, Aperperm049R::Register, Aperperm049W::Register>,
    /// Aperture 050. IOP Reg 64KB page at 0xFF32_0000.
    pub aperperm_050: Aliased<u32, Aperperm050R::Register, Aperperm050W::Register>,
    /// Aperture 051. IOP Reg 64KB page at 0xFF33_0000.
    pub aperperm_051: Aliased<u32, Aperperm051R::Register, Aperperm051W::Register>,
    /// Aperture 052. IOP Reg 64KB page at 0xFF34_0000.
    pub aperperm_052: Aliased<u32, Aperperm052R::Register, Aperperm052W::Register>,
    /// Aperture 053. IOP Reg 64KB page at 0xFF35_0000.
    pub aperperm_053: Aliased<u32, Aperperm053R::Register, Aperperm053W::Register>,
    /// Aperture 054. IOP Reg 64KB page at 0xFF36_0000.
    pub aperperm_054: Aliased<u32, Aperperm054R::Register, Aperperm054W::Register>,
    /// Aperture 055. IOP Reg 64KB page at 0xFF37_0000.
    pub aperperm_055: Aliased<u32, Aperperm055R::Register, Aperperm055W::Register>,
    /// Aperture 056. IOP Reg 64KB page at 0xFF38_0000.
    pub aperperm_056: Aliased<u32, Aperperm056R::Register, Aperperm056W::Register>,
    /// Aperture 057. IOP Reg 64KB page at 0xFF39_0000.
    pub aperperm_057: Aliased<u32, Aperperm057R::Register, Aperperm057W::Register>,
    /// Aperture 058. IOP Reg 64KB page at 0xFF3A_0000.
    pub aperperm_058: Aliased<u32, Aperperm058R::Register, Aperperm058W::Register>,
    /// Aperture 059. IOP Reg 64KB page at 0xFF3B_0000.
    pub aperperm_059: Aliased<u32, Aperperm059R::Register, Aperperm059W::Register>,
    /// Aperture 060. IOP Reg 64KB page at 0xFF3C_0000.
    pub aperperm_060: Aliased<u32, Aperperm060R::Register, Aperperm060W::Register>,
    /// Aperture 061. IOP Reg 64KB page at 0xFF3D_0000.
    pub aperperm_061: Aliased<u32, Aperperm061R::Register, Aperperm061W::Register>,
    /// Aperture 062. IOP Reg 64KB page at 0xFF3E_0000.
    pub aperperm_062: Aliased<u32, Aperperm062R::Register, Aperperm062W::Register>,
    /// Aperture 063. IOP Reg 64KB page at 0xFF3F_0000.
    pub aperperm_063: Aliased<u32, Aperperm063R::Register, Aperperm063W::Register>,
    /// Aperture 064. IOP Reg 64KB page at 0xFF40_0000.
    pub aperperm_064: Aliased<u32, Aperperm064R::Register, Aperperm064W::Register>,
    /// Aperture 065. IOP Reg 64KB page at 0xFF41_0000.
    pub aperperm_065: Aliased<u32, Aperperm065R::Register, Aperperm065W::Register>,
    /// Aperture 066. IOP Reg 64KB page at 0xFF42_0000.
    pub aperperm_066: Aliased<u32, Aperperm066R::Register, Aperperm066W::Register>,
    /// Aperture 067. IOP Reg 64KB page at 0xFF43_0000.
    pub aperperm_067: Aliased<u32, Aperperm067R::Register, Aperperm067W::Register>,
    /// Aperture 068. IOP Reg 64KB page at 0xFF44_0000.
    pub aperperm_068: Aliased<u32, Aperperm068R::Register, Aperperm068W::Register>,
    /// Aperture 069. IOP Reg 64KB page at 0xFF45_0000.
    pub aperperm_069: Aliased<u32, Aperperm069R::Register, Aperperm069W::Register>,
    /// Aperture 070. IOP Reg 64KB page at 0xFF46_0000.
    pub aperperm_070: Aliased<u32, Aperperm070R::Register, Aperperm070W::Register>,
    /// Aperture 071. IOP Reg 64KB page at 0xFF47_0000.
    pub aperperm_071: Aliased<u32, Aperperm071R::Register, Aperperm071W::Register>,
    /// Aperture 072. IOP Reg 64KB page at 0xFF48_0000.
    pub aperperm_072: Aliased<u32, Aperperm072R::Register, Aperperm072W::Register>,
    /// Aperture 073. IOP Reg 64KB page at 0xFF49_0000.
    pub aperperm_073: Aliased<u32, Aperperm073R::Register, Aperperm073W::Register>,
    /// Aperture 074. IOP Reg 64KB page at 0xFF4A_0000.
    pub aperperm_074: Aliased<u32, Aperperm074R::Register, Aperperm074W::Register>,
    /// Aperture 075. IOP Reg 64KB page at 0xFF4B_0000.
    pub aperperm_075: Aliased<u32, Aperperm075R::Register, Aperperm075W::Register>,
    /// Aperture 076. IOP Reg 64KB page at 0xFF4C_0000.
    pub aperperm_076: Aliased<u32, Aperperm076R::Register, Aperperm076W::Register>,
    /// Aperture 077. IOP Reg 64KB page at 0xFF4D_0000.
    pub aperperm_077: Aliased<u32, Aperperm077R::Register, Aperperm077W::Register>,
    /// Aperture 078. IOP Reg 64KB page at 0xFF4E_0000.
    pub aperperm_078: Aliased<u32, Aperperm078R::Register, Aperperm078W::Register>,
    /// Aperture 079. IOP Reg 64KB page at 0xFF4F_0000.
    pub aperperm_079: Aliased<u32, Aperperm079R::Register, Aperperm079W::Register>,
    /// Aperture 080. IOP Reg 64KB page at 0xFF50_0000.
    pub aperperm_080: Aliased<u32, Aperperm080R::Register, Aperperm080W::Register>,
    /// Aperture 081. IOP Reg 64KB page at 0xFF51_0000.
    pub aperperm_081: Aliased<u32, Aperperm081R::Register, Aperperm081W::Register>,
    /// Aperture 082. IOP Reg 64KB page at 0xFF52_0000.
    pub aperperm_082: Aliased<u32, Aperperm082R::Register, Aperperm082W::Register>,
    /// Aperture 083. IOP Reg 64KB page at 0xFF53_0000.
    pub aperperm_083: Aliased<u32, Aperperm083R::Register, Aperperm083W::Register>,
    /// Aperture 084. IOP Reg 64KB page at 0xFF54_0000.
    pub aperperm_084: Aliased<u32, Aperperm084R::Register, Aperperm084W::Register>,
    /// Aperture 085. IOP Reg 64KB page at 0xFF55_0000.
    pub aperperm_085: Aliased<u32, Aperperm085R::Register, Aperperm085W::Register>,
    /// Aperture 086. IOP Reg 64KB page at 0xFF56_0000.
    pub aperperm_086: Aliased<u32, Aperperm086R::Register, Aperperm086W::Register>,
    /// Aperture 087. IOP Reg 64KB page at 0xFF57_0000.
    pub aperperm_087: Aliased<u32, Aperperm087R::Register, Aperperm087W::Register>,
    /// Aperture 088. IOP Reg 64KB page at 0xFF58_0000.
    pub aperperm_088: Aliased<u32, Aperperm088R::Register, Aperperm088W::Register>,
    /// Aperture 089. IOP Reg 64KB page at 0xFF59_0000.
    pub aperperm_089: Aliased<u32, Aperperm089R::Register, Aperperm089W::Register>,
    /// Aperture 090. IOP Reg 64KB page at 0xFF5A_0000.
    pub aperperm_090: Aliased<u32, Aperperm090R::Register, Aperperm090W::Register>,
    /// Aperture 091. IOP Reg 64KB page at 0xFF5B_0000.
    pub aperperm_091: Aliased<u32, Aperperm091R::Register, Aperperm091W::Register>,
    /// Aperture 092. IOP Reg 64KB page at 0xFF5C_0000.
    pub aperperm_092: Aliased<u32, Aperperm092R::Register, Aperperm092W::Register>,
    /// Aperture 093. IOP Reg 64KB page at 0xFF5D_0000.
    pub aperperm_093: Aliased<u32, Aperperm093R::Register, Aperperm093W::Register>,
    /// Aperture 094. IOP Reg 64KB page at 0xFF5E_0000.
    pub aperperm_094: Aliased<u32, Aperperm094R::Register, Aperperm094W::Register>,
    /// Aperture 095. IOP Reg 64KB page at 0xFF5F_0000.
    pub aperperm_095: Aliased<u32, Aperperm095R::Register, Aperperm095W::Register>,
    /// Aperture 096. IOP Reg 64KB page at 0xFF60_0000.
    pub aperperm_096: Aliased<u32, Aperperm096R::Register, Aperperm096W::Register>,
    /// Aperture 097. IOP Reg 64KB page at 0xFF61_0000.
    pub aperperm_097: Aliased<u32, Aperperm097R::Register, Aperperm097W::Register>,
    /// Aperture 098. IOP Reg 64KB page at 0xFF62_0000.
    pub aperperm_098: Aliased<u32, Aperperm098R::Register, Aperperm098W::Register>,
    /// Aperture 099. IOP Reg 64KB page at 0xFF63_0000.
    pub aperperm_099: Aliased<u32, Aperperm099R::Register, Aperperm099W::Register>,
    /// Aperture 100. IOP Reg 64KB page at 0xFF64_0000.
    pub aperperm_100: Aliased<u32, Aperperm100R::Register, Aperperm100W::Register>,
    /// Aperture 101. IOP Reg 64KB page at 0xFF65_0000.
    pub aperperm_101: Aliased<u32, Aperperm101R::Register, Aperperm101W::Register>,
    /// Aperture 102. IOP Reg 64KB page at 0xFF66_0000.
    pub aperperm_102: Aliased<u32, Aperperm102R::Register, Aperperm102W::Register>,
    /// Aperture 103. IOP Reg 64KB page at 0xFF67_0000.
    pub aperperm_103: Aliased<u32, Aperperm103R::Register, Aperperm103W::Register>,
    /// Aperture 104. IOP Reg 64KB page at 0xFF68_0000.
    pub aperperm_104: Aliased<u32, Aperperm104R::Register, Aperperm104W::Register>,
    /// Aperture 105. IOP Reg 64KB page at 0xFF69_0000.
    pub aperperm_105: Aliased<u32, Aperperm105R::Register, Aperperm105W::Register>,
    /// Aperture 106. IOP Reg 64KB page at 0xFF6A_0000.
    pub aperperm_106: Aliased<u32, Aperperm106R::Register, Aperperm106W::Register>,
    /// Aperture 107. IOP Reg 64KB page at 0xFF6B_0000.
    pub aperperm_107: Aliased<u32, Aperperm107R::Register, Aperperm107W::Register>,
    /// Aperture 108. IOP Reg 64KB page at 0xFF6C_0000.
    pub aperperm_108: Aliased<u32, Aperperm108R::Register, Aperperm108W::Register>,
    /// Aperture 109. IOP Reg 64KB page at 0xFF6D_0000.
    pub aperperm_109: Aliased<u32, Aperperm109R::Register, Aperperm109W::Register>,
    /// Aperture 110. IOP Reg 64KB page at 0xFF6E_0000.
    pub aperperm_110: Aliased<u32, Aperperm110R::Register, Aperperm110W::Register>,
    /// Aperture 111. IOP Reg 64KB page at 0xFF6F_0000.
    pub aperperm_111: Aliased<u32, Aperperm111R::Register, Aperperm111W::Register>,
    /// Aperture 112. IOP Reg 64KB page at 0xFF70_0000.
    pub aperperm_112: Aliased<u32, Aperperm112R::Register, Aperperm112W::Register>,
    /// Aperture 113. IOP Reg 64KB page at 0xFF71_0000.
    pub aperperm_113: Aliased<u32, Aperperm113R::Register, Aperperm113W::Register>,
    /// Aperture 114. IOP Reg 64KB page at 0xFF72_0000.
    pub aperperm_114: Aliased<u32, Aperperm114R::Register, Aperperm114W::Register>,
    /// Aperture 115. IOP Reg 64KB page at 0xFF73_0000.
    pub aperperm_115: Aliased<u32, Aperperm115R::Register, Aperperm115W::Register>,
    /// Aperture 116. IOP Reg 64KB page at 0xFF74_0000.
    pub aperperm_116: Aliased<u32, Aperperm116R::Register, Aperperm116W::Register>,
    /// Aperture 117. IOP Reg 64KB page at 0xFF75_0000.
    pub aperperm_117: Aliased<u32, Aperperm117R::Register, Aperperm117W::Register>,
    /// Aperture 118. IOP Reg 64KB page at 0xFF76_0000.
    pub aperperm_118: Aliased<u32, Aperperm118R::Register, Aperperm118W::Register>,
    /// Aperture 119. IOP Reg 64KB page at 0xFF77_0000.
    pub aperperm_119: Aliased<u32, Aperperm119R::Register, Aperperm119W::Register>,
    /// Aperture 120. IOP Reg 64KB page at 0xFF78_0000.
    pub aperperm_120: Aliased<u32, Aperperm120R::Register, Aperperm120W::Register>,
    /// Aperture 121. IOP Reg 64KB page at 0xFF79_0000.
    pub aperperm_121: Aliased<u32, Aperperm121R::Register, Aperperm121W::Register>,
    /// Aperture 122. IOP Reg 64KB page at 0xFF7A_0000.
    pub aperperm_122: Aliased<u32, Aperperm122R::Register, Aperperm122W::Register>,
    /// Aperture 123. IOP Reg 64KB page at 0xFF7B_0000.
    pub aperperm_123: Aliased<u32, Aperperm123R::Register, Aperperm123W::Register>,
    /// Aperture 124. IOP Reg 64KB page at 0xFF7C_0000.
    pub aperperm_124: Aliased<u32, Aperperm124R::Register, Aperperm124W::Register>,
    /// Aperture 125. IOP Reg 64KB page at 0xFF7D_0000.
    pub aperperm_125: Aliased<u32, Aperperm125R::Register, Aperperm125W::Register>,
    /// Aperture 126. IOP Reg 64KB page at 0xFF7E_0000.
    pub aperperm_126: Aliased<u32, Aperperm126R::Register, Aperperm126W::Register>,
    /// Aperture 127. IOP Reg 64KB page at 0xFF7F_0000.
    pub aperperm_127: Aliased<u32, Aperperm127R::Register, Aperperm127W::Register>,
    /// Aperture 128. IOP Reg 64KB page at 0xFF80_0000.
    pub aperperm_128: Aliased<u32, Aperperm128R::Register, Aperperm128W::Register>,
    /// Aperture 129. IOP Reg 64KB page at 0xFF81_0000.
    pub aperperm_129: Aliased<u32, Aperperm129R::Register, Aperperm129W::Register>,
    /// Aperture 130. IOP Reg 64KB page at 0xFF82_0000.
    pub aperperm_130: Aliased<u32, Aperperm130R::Register, Aperperm130W::Register>,
    /// Aperture 131. IOP Reg 64KB page at 0xFF83_0000.
    pub aperperm_131: Aliased<u32, Aperperm131R::Register, Aperperm131W::Register>,
    /// Aperture 132. IOP Reg 64KB page at 0xFF84_0000.
    pub aperperm_132: Aliased<u32, Aperperm132R::Register, Aperperm132W::Register>,
    /// Aperture 133. IOP Reg 64KB page at 0xFF85_0000.
    pub aperperm_133: Aliased<u32, Aperperm133R::Register, Aperperm133W::Register>,
    /// Aperture 134. IOP Reg 64KB page at 0xFF86_0000.
    pub aperperm_134: Aliased<u32, Aperperm134R::Register, Aperperm134W::Register>,
    /// Aperture 135. IOP Reg 64KB page at 0xFF87_0000.
    pub aperperm_135: Aliased<u32, Aperperm135R::Register, Aperperm135W::Register>,
    /// Aperture 136. IOP Reg 64KB page at 0xFF88_0000.
    pub aperperm_136: Aliased<u32, Aperperm136R::Register, Aperperm136W::Register>,
    /// Aperture 137. IOP Reg 64KB page at 0xFF89_0000.
    pub aperperm_137: Aliased<u32, Aperperm137R::Register, Aperperm137W::Register>,
    /// Aperture 138. IOP Reg 64KB page at 0xFF8A_0000.
    pub aperperm_138: Aliased<u32, Aperperm138R::Register, Aperperm138W::Register>,
    /// Aperture 139. IOP Reg 64KB page at 0xFF8B_0000.
    pub aperperm_139: Aliased<u32, Aperperm139R::Register, Aperperm139W::Register>,
    /// Aperture 140. IOP Reg 64KB page at 0xFF8C_0000.
    pub aperperm_140: Aliased<u32, Aperperm140R::Register, Aperperm140W::Register>,
    /// Aperture 141. IOP Reg 64KB page at 0xFF8D_0000.
    pub aperperm_141: Aliased<u32, Aperperm141R::Register, Aperperm141W::Register>,
    /// Aperture 142. IOP Reg 64KB page at 0xFF8E_0000.
    pub aperperm_142: Aliased<u32, Aperperm142R::Register, Aperperm142W::Register>,
    /// Aperture 143. IOP Reg 64KB page at 0xFF8F_0000.
    pub aperperm_143: Aliased<u32, Aperperm143R::Register, Aperperm143W::Register>,
    /// Aperture 144. IOP Reg 64KB page at 0xFF90_0000.
    pub aperperm_144: Aliased<u32, Aperperm144R::Register, Aperperm144W::Register>,
    /// Aperture 145. IOP Reg 64KB page at 0xFF91_0000.
    pub aperperm_145: Aliased<u32, Aperperm145R::Register, Aperperm145W::Register>,
    /// Aperture 146. IOP Reg 64KB page at 0xFF92_0000.
    pub aperperm_146: Aliased<u32, Aperperm146R::Register, Aperperm146W::Register>,
    /// Aperture 147. IOP Reg 64KB page at 0xFF93_0000.
    pub aperperm_147: Aliased<u32, Aperperm147R::Register, Aperperm147W::Register>,
    /// Aperture 148. IOP Reg 64KB page at 0xFF94_0000.
    pub aperperm_148: Aliased<u32, Aperperm148R::Register, Aperperm148W::Register>,
    /// Aperture 149. IOP Reg 64KB page at 0xFF95_0000.
    pub aperperm_149: Aliased<u32, Aperperm149R::Register, Aperperm149W::Register>,
    /// Aperture 150. IOP Reg 64KB page at 0xFF96_0000.
    pub aperperm_150: Aliased<u32, Aperperm150R::Register, Aperperm150W::Register>,
    /// Aperture 151. IOP Reg 64KB page at 0xFF97_0000.
    pub aperperm_151: Aliased<u32, Aperperm151R::Register, Aperperm151W::Register>,
    /// Aperture 152. IOP Reg 64KB page at 0xFF98_0000.
    pub aperperm_152: Aliased<u32, Aperperm152R::Register, Aperperm152W::Register>,
    /// Aperture 153. IOP Reg 64KB page at 0xFF99_0000.Note: For 4KB, address range is 0xFF99_0000 - 0xFF99_0FFF & for 60KB address range is 0xFF99_1000 - 0xFF99_FFFF.
    pub aperperm_153: Aliased<u32, Aperperm153R::Register, Aperperm153W::Register>,
    /// Aperture 154. IOP Reg 64KB page at 0xFF9A_0000.
    pub aperperm_154: Aliased<u32, Aperperm154R::Register, Aperperm154W::Register>,
    /// Aperture 155. IOP Reg 64KB page at 0xFF9B_0000.
    pub aperperm_155: Aliased<u32, Aperperm155R::Register, Aperperm155W::Register>,
    /// Aperture 156. IOP Reg 64KB page at 0xFF9C_0000.
    pub aperperm_156: Aliased<u32, Aperperm156R::Register, Aperperm156W::Register>,
    /// Aperture 157. IOP Reg 64KB page at 0xFF9D_0000.
    pub aperperm_157: Aliased<u32, Aperperm157R::Register, Aperperm157W::Register>,
    /// Aperture 158. IOP Reg 64KB page at 0xFF9E_0000.
    pub aperperm_158: Aliased<u32, Aperperm158R::Register, Aperperm158W::Register>,
    /// Aperture 159. IOP Reg 64KB page at 0xFF9F_0000.
    pub aperperm_159: Aliased<u32, Aperperm159R::Register, Aperperm159W::Register>,
    /// Aperture 160. IOP Reg 64KB page at 0xFFA0_0000.
    pub aperperm_160: Aliased<u32, Aperperm160R::Register, Aperperm160W::Register>,
    /// Aperture 161. IOP Reg 64KB page at 0xFFA1_0000.
    pub aperperm_161: Aliased<u32, Aperperm161R::Register, Aperperm161W::Register>,
    /// Aperture 162. IOP Reg 64KB page at 0xFFA2_0000.
    pub aperperm_162: Aliased<u32, Aperperm162R::Register, Aperperm162W::Register>,
    /// Aperture 163. IOP Reg 64KB page at 0xFFA3_0000.
    pub aperperm_163: Aliased<u32, Aperperm163R::Register, Aperperm163W::Register>,
    /// Aperture 164. IOP Reg 64KB page at 0xFFA4_0000.
    pub aperperm_164: Aliased<u32, Aperperm164R::Register, Aperperm164W::Register>,
    /// Aperture 165. IOP Reg 64KB page at 0xFFA5_0000.
    pub aperperm_165: Aliased<u32, Aperperm165R::Register, Aperperm165W::Register>,
    /// Aperture 166. IOP Reg 64KB page at 0xFFA6_0000.
    pub aperperm_166: Aliased<u32, Aperperm166R::Register, Aperperm166W::Register>,
    /// Aperture 167. IOP Reg 64KB page at 0xFFA7_0000.
    pub aperperm_167: Aliased<u32, Aperperm167R::Register, Aperperm167W::Register>,
    /// Aperture 168. IOP Reg 64KB page at 0xFFA8_0000.
    pub aperperm_168: Aliased<u32, Aperperm168R::Register, Aperperm168W::Register>,
    /// Aperture 169. IOP Reg 64KB page at 0xFFA9_0000.
    pub aperperm_169: Aliased<u32, Aperperm169R::Register, Aperperm169W::Register>,
    /// Aperture 170. IOP Reg 64KB page at 0xFFAA_0000.
    pub aperperm_170: Aliased<u32, Aperperm170R::Register, Aperperm170W::Register>,
    /// Aperture 171. IOP Reg 64KB page at 0xFFAB_0000.
    pub aperperm_171: Aliased<u32, Aperperm171R::Register, Aperperm171W::Register>,
    /// Aperture 172. IOP Reg 64KB page at 0xFFAC_0000.
    pub aperperm_172: Aliased<u32, Aperperm172R::Register, Aperperm172W::Register>,
    /// Aperture 173. IOP Reg 64KB page at 0xFFAD_0000.
    pub aperperm_173: Aliased<u32, Aperperm173R::Register, Aperperm173W::Register>,
    /// Aperture 174. IOP Reg 64KB page at 0xFFAE_0000.
    pub aperperm_174: Aliased<u32, Aperperm174R::Register, Aperperm174W::Register>,
    /// Aperture 175. IOP Reg 64KB page at 0xFFAF_0000.
    pub aperperm_175: Aliased<u32, Aperperm175R::Register, Aperperm175W::Register>,
    /// Aperture 176. IOP Reg 64KB page at 0xFFB0_0000.
    pub aperperm_176: Aliased<u32, Aperperm176R::Register, Aperperm176W::Register>,
    /// Aperture 177. IOP Reg 64KB page at 0xFFB1_0000.
    pub aperperm_177: Aliased<u32, Aperperm177R::Register, Aperperm177W::Register>,
    /// Aperture 178. IOP Reg 64KB page at 0xFFB2_0000.
    pub aperperm_178: Aliased<u32, Aperperm178R::Register, Aperperm178W::Register>,
    /// Aperture 179. IOP Reg 64KB page at 0xFFB3_0000.
    pub aperperm_179: Aliased<u32, Aperperm179R::Register, Aperperm179W::Register>,
    /// Aperture 180. IOP Reg 64KB page at 0xFFB4_0000.
    pub aperperm_180: Aliased<u32, Aperperm180R::Register, Aperperm180W::Register>,
    /// Aperture 181. IOP Reg 64KB page at 0xFFB5_0000.
    pub aperperm_181: Aliased<u32, Aperperm181R::Register, Aperperm181W::Register>,
    /// Aperture 182. IOP Reg 64KB page at 0xFFB6_0000.
    pub aperperm_182: Aliased<u32, Aperperm182R::Register, Aperperm182W::Register>,
    /// Aperture 183. IOP Reg 64KB page at 0xFFB7_0000.
    pub aperperm_183: Aliased<u32, Aperperm183R::Register, Aperperm183W::Register>,
    /// Aperture 184. IOP Reg 64KB page at 0xFFB8_0000.
    pub aperperm_184: Aliased<u32, Aperperm184R::Register, Aperperm184W::Register>,
    /// Aperture 185. IOP Reg 64KB page at 0xFFB9_0000.
    pub aperperm_185: Aliased<u32, Aperperm185R::Register, Aperperm185W::Register>,
    /// Aperture 186. IOP Reg 64KB page at 0xFFBA_0000.
    pub aperperm_186: Aliased<u32, Aperperm186R::Register, Aperperm186W::Register>,
    /// Aperture 187. IOP Reg 64KB page at 0xFFBB_0000.
    pub aperperm_187: Aliased<u32, Aperperm187R::Register, Aperperm187W::Register>,
    /// Aperture 188. IOP Reg 64KB page at 0xFFBC_0000.
    pub aperperm_188: Aliased<u32, Aperperm188R::Register, Aperperm188W::Register>,
    /// Aperture 189. IOP Reg 64KB page at 0xFFBD_0000.
    pub aperperm_189: Aliased<u32, Aperperm189R::Register, Aperperm189W::Register>,
    /// Aperture 190. IOP Reg 64KB page at 0xFFBE_0000.
    pub aperperm_190: Aliased<u32, Aperperm190R::Register, Aperperm190W::Register>,
    /// Aperture 191. IOP Reg 64KB page at 0xFFBF_0000.
    pub aperperm_191: Aliased<u32, Aperperm191R::Register, Aperperm191W::Register>,
    /// Aperture 192. IOP Reg 64KB page at 0xFFC0_0000.
    pub aperperm_192: Aliased<u32, Aperperm192R::Register, Aperperm192W::Register>,
    /// Aperture 193. IOP Reg 64KB page at 0xFFC1_0000.
    pub aperperm_193: Aliased<u32, Aperperm193R::Register, Aperperm193W::Register>,
    /// Aperture 194. IOP Reg 64KB page at 0xFFC2_0000.
    pub aperperm_194: Aliased<u32, Aperperm194R::Register, Aperperm194W::Register>,
    /// Aperture 195. IOP Reg 64KB page at 0xFFC3_0000.
    pub aperperm_195: Aliased<u32, Aperperm195R::Register, Aperperm195W::Register>,
    /// Aperture 196. IOP Reg 64KB page at 0xFFC4_0000.
    pub aperperm_196: Aliased<u32, Aperperm196R::Register, Aperperm196W::Register>,
    /// Aperture 197. IOP Reg 64KB page at 0xFFC5_0000.
    pub aperperm_197: Aliased<u32, Aperperm197R::Register, Aperperm197W::Register>,
    /// Aperture 198. IOP Reg 64KB page at 0xFFC6_0000.
    pub aperperm_198: Aliased<u32, Aperperm198R::Register, Aperperm198W::Register>,
    /// Aperture 199. IOP Reg 64KB page at 0xFFC7_0000.
    pub aperperm_199: Aliased<u32, Aperperm199R::Register, Aperperm199W::Register>,
    /// Aperture 200. IOP Reg 64KB page at 0xFFC8_0000.
    pub aperperm_200: Aliased<u32, Aperperm200R::Register, Aperperm200W::Register>,
    /// Aperture 201. IOP Reg 64KB page at 0xFFC9_0000.
    pub aperperm_201: Aliased<u32, Aperperm201R::Register, Aperperm201W::Register>,
    /// Aperture 202. IOP Reg 64KB page at 0xFFCA_0000.
    pub aperperm_202: Aliased<u32, Aperperm202R::Register, Aperperm202W::Register>,
    /// Aperture 203. IOP Reg 64KB page at 0xFFCB_0000.
    pub aperperm_203: Aliased<u32, Aperperm203R::Register, Aperperm203W::Register>,
    /// Aperture 204. IOP Reg 64KB page at 0xFFCC_0000.
    pub aperperm_204: Aliased<u32, Aperperm204R::Register, Aperperm204W::Register>,
    /// Aperture 205. IOP Reg 64KB page at 0xFFCD_0000.
    pub aperperm_205: Aliased<u32, Aperperm205R::Register, Aperperm205W::Register>,
    /// Aperture 206. IOP Reg 64KB page at 0xFFCE_0000.
    pub aperperm_206: Aliased<u32, Aperperm206R::Register, Aperperm206W::Register>,
    /// Aperture 207. IOP Reg 64KB page at 0xFFCF_0000.
    pub aperperm_207: Aliased<u32, Aperperm207R::Register, Aperperm207W::Register>,
    /// Aperture 208. IOP Reg 64KB page at 0xFFD0_0000.
    pub aperperm_208: Aliased<u32, Aperperm208R::Register, Aperperm208W::Register>,
    /// Aperture 209. IOP Reg 64KB page at 0xFFD1_0000.
    pub aperperm_209: Aliased<u32, Aperperm209R::Register, Aperperm209W::Register>,
    /// Aperture 210. IOP Reg 64KB page at 0xFFD2_0000.
    pub aperperm_210: Aliased<u32, Aperperm210R::Register, Aperperm210W::Register>,
    /// Aperture 211. IOP Reg 64KB page at 0xFFD3_0000.
    pub aperperm_211: Aliased<u32, Aperperm211R::Register, Aperperm211W::Register>,
    /// Aperture 212. IOP Reg 64KB page at 0xFFD4_0000.
    pub aperperm_212: Aliased<u32, Aperperm212R::Register, Aperperm212W::Register>,
    /// Aperture 213. IOP Reg 64KB page at 0xFFD5_0000.
    pub aperperm_213: Aliased<u32, Aperperm213R::Register, Aperperm213W::Register>,
    /// Aperture 214. IOP Reg 64KB page at 0xFFD6_0000.
    pub aperperm_214: Aliased<u32, Aperperm214R::Register, Aperperm214W::Register>,
    /// Aperture 215. IOP Reg 64KB page at 0xFFD7_0000.
    pub aperperm_215: Aliased<u32, Aperperm215R::Register, Aperperm215W::Register>,
    /// Aperture 216. IOP Reg 64KB page at 0xFFD8_0000.
    pub aperperm_216: Aliased<u32, Aperperm216R::Register, Aperperm216W::Register>,
    /// Aperture 217. IOP Reg 64KB page at 0xFFD9_0000.
    pub aperperm_217: Aliased<u32, Aperperm217R::Register, Aperperm217W::Register>,
    /// Aperture 218. IOP Reg 64KB page at 0xFFDA_0000.
    pub aperperm_218: Aliased<u32, Aperperm218R::Register, Aperperm218W::Register>,
    /// Aperture 219. IOP Reg 64KB page at 0xFFDB_0000.
    pub aperperm_219: Aliased<u32, Aperperm219R::Register, Aperperm219W::Register>,
    /// Aperture 220. IOP Reg 64KB page at 0xFFDC_0000.
    pub aperperm_220: Aliased<u32, Aperperm220R::Register, Aperperm220W::Register>,
    /// Aperture 221. IOP Reg 64KB page at 0xFFDD_0000.
    pub aperperm_221: Aliased<u32, Aperperm221R::Register, Aperperm221W::Register>,
    /// Aperture 222. IOP Reg 64KB page at 0xFFDE_0000.
    pub aperperm_222: Aliased<u32, Aperperm222R::Register, Aperperm222W::Register>,
    /// Aperture 223. IOP Reg 64KB page at 0xFFDF_0000.
    pub aperperm_223: Aliased<u32, Aperperm223R::Register, Aperperm223W::Register>,
    /// Aperture 224. IOP Reg 64KB page at 0xFFE0_0000.
    pub aperperm_224: Aliased<u32, Aperperm224R::Register, Aperperm224W::Register>,
    /// Aperture 225. IOP Reg 64KB page at 0xFFE1_0000.
    pub aperperm_225: Aliased<u32, Aperperm225R::Register, Aperperm225W::Register>,
    /// Aperture 226. IOP Reg 64KB page at 0xFFE2_0000.
    pub aperperm_226: Aliased<u32, Aperperm226R::Register, Aperperm226W::Register>,
    /// Aperture 227. IOP Reg 64KB page at 0xFFE3_0000.
    pub aperperm_227: Aliased<u32, Aperperm227R::Register, Aperperm227W::Register>,
    /// Aperture 228. IOP Reg 64KB page at 0xFFE4_0000.
    pub aperperm_228: Aliased<u32, Aperperm228R::Register, Aperperm228W::Register>,
    /// Aperture 229. IOP Reg 64KB page at 0xFFE5_0000.
    pub aperperm_229: Aliased<u32, Aperperm229R::Register, Aperperm229W::Register>,
    /// Aperture 230. IOP Reg 64KB page at 0xFFE6_0000.
    pub aperperm_230: Aliased<u32, Aperperm230R::Register, Aperperm230W::Register>,
    /// Aperture 231. IOP Reg 64KB page at 0xFFE7_0000.
    pub aperperm_231: Aliased<u32, Aperperm231R::Register, Aperperm231W::Register>,
    /// Aperture 232. IOP Reg 64KB page at 0xFFE8_0000.
    pub aperperm_232: Aliased<u32, Aperperm232R::Register, Aperperm232W::Register>,
    /// Aperture 233. IOP Reg 64KB page at 0xFFE9_0000.
    pub aperperm_233: Aliased<u32, Aperperm233R::Register, Aperperm233W::Register>,
    /// Aperture 234. IOP Reg 64KB page at 0xFFEA_0000.
    pub aperperm_234: Aliased<u32, Aperperm234R::Register, Aperperm234W::Register>,
    /// Aperture 235. IOP Reg 64KB page at 0xFFEB_0000.
    pub aperperm_235: Aliased<u32, Aperperm235R::Register, Aperperm235W::Register>,
    /// Aperture 236. IOP Reg 64KB page at 0xFFEC_0000.
    pub aperperm_236: Aliased<u32, Aperperm236R::Register, Aperperm236W::Register>,
    /// Aperture 237. IOP Reg 64KB page at 0xFFED_0000.
    pub aperperm_237: Aliased<u32, Aperperm237R::Register, Aperperm237W::Register>,
    /// Aperture 238. IOP Reg 64KB page at 0xFFEE_0000.
    pub aperperm_238: Aliased<u32, Aperperm238R::Register, Aperperm238W::Register>,
    /// Aperture 239. IOP Reg 64KB page at 0xFFEF_0000.
    pub aperperm_239: Aliased<u32, Aperperm239R::Register, Aperperm239W::Register>,
    /// Aperture 240. IOP Reg 64KB page at 0xFFF0_0000.
    pub aperperm_240: Aliased<u32, Aperperm240R::Register, Aperperm240W::Register>,
    /// Aperture 241. IOP Reg 64KB page at 0xFFF1_0000.
    pub aperperm_241: Aliased<u32, Aperperm241R::Register, Aperperm241W::Register>,
    /// Aperture 242. IOP Reg 64KB page at 0xFFF2_0000.
    pub aperperm_242: Aliased<u32, Aperperm242R::Register, Aperperm242W::Register>,
    /// Aperture 243. IOP Reg 64KB page at 0xFFF3_0000.
    pub aperperm_243: Aliased<u32, Aperperm243R::Register, Aperperm243W::Register>,
    /// Aperture 244. IOP Reg 64KB page at 0xFFF4_0000.
    pub aperperm_244: Aliased<u32, Aperperm244R::Register, Aperperm244W::Register>,
    /// Aperture 245. IOP Reg 64KB page at 0xFFF5_0000.
    pub aperperm_245: Aliased<u32, Aperperm245R::Register, Aperperm245W::Register>,
    /// Aperture 246. IOP Reg 64KB page at 0xFFF6_0000.
    pub aperperm_246: Aliased<u32, Aperperm246R::Register, Aperperm246W::Register>,
    /// Aperture 247. IOP Reg 64KB page at 0xFFF7_0000.
    pub aperperm_247: Aliased<u32, Aperperm247R::Register, Aperperm247W::Register>,
    /// Aperture 248. IOP Reg 64KB page at 0xFFF8_0000.
    pub aperperm_248: Aliased<u32, Aperperm248R::Register, Aperperm248W::Register>,
    /// Aperture 249. IOP Reg 64KB page at 0xFFF9_0000.
    pub aperperm_249: Aliased<u32, Aperperm249R::Register, Aperperm249W::Register>,
    /// Aperture 250. IOP Reg 64KB page at 0xFFFA_0000.
    pub aperperm_250: Aliased<u32, Aperperm250R::Register, Aperperm250W::Register>,
    /// Aperture 251. IOP Reg 64KB page at 0xFFFB_0000.
    pub aperperm_251: Aliased<u32, Aperperm251R::Register, Aperperm251W::Register>,
    /// Aperture 252. IOP Reg 64KB page at 0xFFFC_0000.
    pub aperperm_252: Aliased<u32, Aperperm252R::Register, Aperperm252W::Register>,
    /// Aperture 253. IOP Reg 64KB page at 0xFFFD_0000.
    pub aperperm_253: Aliased<u32, Aperperm253R::Register, Aperperm253W::Register>,
    /// Aperture 254. IOP Reg 64KB page at 0xFFFE_0000.
    pub aperperm_254: Aliased<u32, Aperperm254R::Register, Aperperm254W::Register>,
    /// Aperture 255. IOP Reg 64KB page at 0xFFFF_0000.
    pub aperperm_255: Aliased<u32, Aperperm255R::Register, Aperperm255W::Register>,
    /// Aperture 256. IPI 32B buffer at 0xFF99_0000
    pub aperperm_256: Aliased<u32, Aperperm256R::Register, Aperperm256W::Register>,
    /// Aperture 257. IPI 32B buffer at 0xFF99_0020
    pub aperperm_257: Aliased<u32, Aperperm257R::Register, Aperperm257W::Register>,
    /// Aperture 258. IPI 32B buffer at 0xFF99_0040
    pub aperperm_258: Aliased<u32, Aperperm258R::Register, Aperperm258W::Register>,
    /// Aperture 259. IPI 32B buffer at 0xFF99_0060
    pub aperperm_259: Aliased<u32, Aperperm259R::Register, Aperperm259W::Register>,
    /// Aperture 260. IPI 32B buffer at 0xFF99_0080
    pub aperperm_260: Aliased<u32, Aperperm260R::Register, Aperperm260W::Register>,
    /// Aperture 261. IPI 32B buffer at 0xFF99_00A0
    pub aperperm_261: Aliased<u32, Aperperm261R::Register, Aperperm261W::Register>,
    /// Aperture 262. IPI 32B buffer at 0xFF99_00C0
    pub aperperm_262: Aliased<u32, Aperperm262R::Register, Aperperm262W::Register>,
    /// Aperture 263. IPI 32B buffer at 0xFF99_00E0
    pub aperperm_263: Aliased<u32, Aperperm263R::Register, Aperperm263W::Register>,
    /// Aperture 264. IPI 32B buffer at 0xFF99_0100
    pub aperperm_264: Aliased<u32, Aperperm264R::Register, Aperperm264W::Register>,
    /// Aperture 265. IPI 32B buffer at 0xFF99_0120
    pub aperperm_265: Aliased<u32, Aperperm265R::Register, Aperperm265W::Register>,
    /// Aperture 266. IPI 32B buffer at 0xFF99_0140
    pub aperperm_266: Aliased<u32, Aperperm266R::Register, Aperperm266W::Register>,
    /// Aperture 267. IPI 32B buffer at 0xFF99_0160
    pub aperperm_267: Aliased<u32, Aperperm267R::Register, Aperperm267W::Register>,
    /// Aperture 268. IPI 32B buffer at 0xFF99_0180
    pub aperperm_268: Aliased<u32, Aperperm268R::Register, Aperperm268W::Register>,
    /// Aperture 269. IPI 32B buffer at 0xFF99_01A0
    pub aperperm_269: Aliased<u32, Aperperm269R::Register, Aperperm269W::Register>,
    /// Aperture 270. IPI 32B buffer at 0xFF99_01C0
    pub aperperm_270: Aliased<u32, Aperperm270R::Register, Aperperm270W::Register>,
    /// Aperture 271. IPI 32B buffer at 0xFF99_01E0
    pub aperperm_271: Aliased<u32, Aperperm271R::Register, Aperperm271W::Register>,
    /// Aperture 272. IPI 32B buffer at 0xFF99_0200
    pub aperperm_272: Aliased<u32, Aperperm272R::Register, Aperperm272W::Register>,
    /// Aperture 273. IPI 32B buffer at 0xFF99_0220
    pub aperperm_273: Aliased<u32, Aperperm273R::Register, Aperperm273W::Register>,
    /// Aperture 274. IPI 32B buffer at 0xFF99_0240
    pub aperperm_274: Aliased<u32, Aperperm274R::Register, Aperperm274W::Register>,
    /// Aperture 275. IPI 32B buffer at 0xFF99_0260
    pub aperperm_275: Aliased<u32, Aperperm275R::Register, Aperperm275W::Register>,
    /// Aperture 276. IPI 32B buffer at 0xFF99_0280
    pub aperperm_276: Aliased<u32, Aperperm276R::Register, Aperperm276W::Register>,
    /// Aperture 277. IPI 32B buffer at 0xFF99_02A0
    pub aperperm_277: Aliased<u32, Aperperm277R::Register, Aperperm277W::Register>,
    /// Aperture 278. IPI 32B buffer at 0xFF99_02C0
    pub aperperm_278: Aliased<u32, Aperperm278R::Register, Aperperm278W::Register>,
    /// Aperture 279. IPI 32B buffer at 0xFF99_02E0
    pub aperperm_279: Aliased<u32, Aperperm279R::Register, Aperperm279W::Register>,
    /// Aperture 280. IPI 32B buffer at 0xFF99_0300
    pub aperperm_280: Aliased<u32, Aperperm280R::Register, Aperperm280W::Register>,
    /// Aperture 281. IPI 32B buffer at 0xFF99_0320
    pub aperperm_281: Aliased<u32, Aperperm281R::Register, Aperperm281W::Register>,
    /// Aperture 282. IPI 32B buffer at 0xFF99_0340
    pub aperperm_282: Aliased<u32, Aperperm282R::Register, Aperperm282W::Register>,
    /// Aperture 283. IPI 32B buffer at 0xFF99_0360
    pub aperperm_283: Aliased<u32, Aperperm283R::Register, Aperperm283W::Register>,
    /// Aperture 284. IPI 32B buffer at 0xFF99_0380
    pub aperperm_284: Aliased<u32, Aperperm284R::Register, Aperperm284W::Register>,
    /// Aperture 285. IPI 32B buffer at 0xFF99_03A0
    pub aperperm_285: Aliased<u32, Aperperm285R::Register, Aperperm285W::Register>,
    /// Aperture 286. IPI 32B buffer at 0xFF99_03C0
    pub aperperm_286: Aliased<u32, Aperperm286R::Register, Aperperm286W::Register>,
    /// Aperture 287. IPI 32B buffer at 0xFF99_03E0
    pub aperperm_287: Aliased<u32, Aperperm287R::Register, Aperperm287W::Register>,
    /// Aperture 288. IPI 32B buffer at 0xFF99_0400
    pub aperperm_288: Aliased<u32, Aperperm288R::Register, Aperperm288W::Register>,
    /// Aperture 289. IPI 32B buffer at 0xFF99_0420
    pub aperperm_289: Aliased<u32, Aperperm289R::Register, Aperperm289W::Register>,
    /// Aperture 290. IPI 32B buffer at 0xFF99_0440
    pub aperperm_290: Aliased<u32, Aperperm290R::Register, Aperperm290W::Register>,
    /// Aperture 291. IPI 32B buffer at 0xFF99_0460
    pub aperperm_291: Aliased<u32, Aperperm291R::Register, Aperperm291W::Register>,
    /// Aperture 292. IPI 32B buffer at 0xFF99_0480
    pub aperperm_292: Aliased<u32, Aperperm292R::Register, Aperperm292W::Register>,
    /// Aperture 293. IPI 32B buffer at 0xFF99_04A0
    pub aperperm_293: Aliased<u32, Aperperm293R::Register, Aperperm293W::Register>,
    /// Aperture 294. IPI 32B buffer at 0xFF99_04C0
    pub aperperm_294: Aliased<u32, Aperperm294R::Register, Aperperm294W::Register>,
    /// Aperture 295. IPI 32B buffer at 0xFF99_04E0
    pub aperperm_295: Aliased<u32, Aperperm295R::Register, Aperperm295W::Register>,
    /// Aperture 296. IPI 32B buffer at 0xFF99_0500
    pub aperperm_296: Aliased<u32, Aperperm296R::Register, Aperperm296W::Register>,
    /// Aperture 297. IPI 32B buffer at 0xFF99_0520
    pub aperperm_297: Aliased<u32, Aperperm297R::Register, Aperperm297W::Register>,
    /// Aperture 298. IPI 32B buffer at 0xFF99_0540
    pub aperperm_298: Aliased<u32, Aperperm298R::Register, Aperperm298W::Register>,
    /// Aperture 299. IPI 32B buffer at 0xFF99_0560
    pub aperperm_299: Aliased<u32, Aperperm299R::Register, Aperperm299W::Register>,
    /// Aperture 300. IPI 32B buffer at 0xFF99_0580
    pub aperperm_300: Aliased<u32, Aperperm300R::Register, Aperperm300W::Register>,
    /// Aperture 301. IPI 32B buffer at 0xFF99_05A0
    pub aperperm_301: Aliased<u32, Aperperm301R::Register, Aperperm301W::Register>,
    /// Aperture 302. IPI 32B buffer at 0xFF99_05C0
    pub aperperm_302: Aliased<u32, Aperperm302R::Register, Aperperm302W::Register>,
    /// Aperture 303. IPI 32B buffer at 0xFF99_05E0
    pub aperperm_303: Aliased<u32, Aperperm303R::Register, Aperperm303W::Register>,
    /// Aperture 304. IPI 32B buffer at 0xFF99_0600
    pub aperperm_304: Aliased<u32, Aperperm304R::Register, Aperperm304W::Register>,
    /// Aperture 305. IPI 32B buffer at 0xFF99_0620
    pub aperperm_305: Aliased<u32, Aperperm305R::Register, Aperperm305W::Register>,
    /// Aperture 306. IPI 32B buffer at 0xFF99_0640
    pub aperperm_306: Aliased<u32, Aperperm306R::Register, Aperperm306W::Register>,
    /// Aperture 307. IPI 32B buffer at 0xFF99_0660
    pub aperperm_307: Aliased<u32, Aperperm307R::Register, Aperperm307W::Register>,
    /// Aperture 308. IPI 32B buffer at 0xFF99_0680
    pub aperperm_308: Aliased<u32, Aperperm308R::Register, Aperperm308W::Register>,
    /// Aperture 309. IPI 32B buffer at 0xFF99_06A0
    pub aperperm_309: Aliased<u32, Aperperm309R::Register, Aperperm309W::Register>,
    /// Aperture 310. IPI 32B buffer at 0xFF99_06C0
    pub aperperm_310: Aliased<u32, Aperperm310R::Register, Aperperm310W::Register>,
    /// Aperture 311. IPI 32B buffer at 0xFF99_06E0
    pub aperperm_311: Aliased<u32, Aperperm311R::Register, Aperperm311W::Register>,
    /// Aperture 312. IPI 32B buffer at 0xFF99_0700
    pub aperperm_312: Aliased<u32, Aperperm312R::Register, Aperperm312W::Register>,
    /// Aperture 313. IPI 32B buffer at 0xFF99_0720
    pub aperperm_313: Aliased<u32, Aperperm313R::Register, Aperperm313W::Register>,
    /// Aperture 314. IPI 32B buffer at 0xFF99_0740
    pub aperperm_314: Aliased<u32, Aperperm314R::Register, Aperperm314W::Register>,
    /// Aperture 315. IPI 32B buffer at 0xFF99_0760
    pub aperperm_315: Aliased<u32, Aperperm315R::Register, Aperperm315W::Register>,
    /// Aperture 316. IPI 32B buffer at 0xFF99_0780
    pub aperperm_316: Aliased<u32, Aperperm316R::Register, Aperperm316W::Register>,
    /// Aperture 317. IPI 32B buffer at 0xFF99_07A0
    pub aperperm_317: Aliased<u32, Aperperm317R::Register, Aperperm317W::Register>,
    /// Aperture 318. IPI 32B buffer at 0xFF99_07C0
    pub aperperm_318: Aliased<u32, Aperperm318R::Register, Aperperm318W::Register>,
    /// Aperture 319. IPI 32B buffer at 0xFF99_07E0
    pub aperperm_319: Aliased<u32, Aperperm319R::Register, Aperperm319W::Register>,
    /// Aperture 320. IPI 32B buffer at 0xFF99_0800
    pub aperperm_320: Aliased<u32, Aperperm320R::Register, Aperperm320W::Register>,
    /// Aperture 321. IPI 32B buffer at 0xFF99_0820
    pub aperperm_321: Aliased<u32, Aperperm321R::Register, Aperperm321W::Register>,
    /// Aperture 322. IPI 32B buffer at 0xFF99_0840
    pub aperperm_322: Aliased<u32, Aperperm322R::Register, Aperperm322W::Register>,
    /// Aperture 323. IPI 32B buffer at 0xFF99_0860
    pub aperperm_323: Aliased<u32, Aperperm323R::Register, Aperperm323W::Register>,
    /// Aperture 324. IPI 32B buffer at 0xFF99_0880
    pub aperperm_324: Aliased<u32, Aperperm324R::Register, Aperperm324W::Register>,
    /// Aperture 325. IPI 32B buffer at 0xFF99_08A0
    pub aperperm_325: Aliased<u32, Aperperm325R::Register, Aperperm325W::Register>,
    /// Aperture 326. IPI 32B buffer at 0xFF99_08C0
    pub aperperm_326: Aliased<u32, Aperperm326R::Register, Aperperm326W::Register>,
    /// Aperture 327. IPI 32B buffer at 0xFF99_08E0
    pub aperperm_327: Aliased<u32, Aperperm327R::Register, Aperperm327W::Register>,
    /// Aperture 328. IPI 32B buffer at 0xFF99_0900
    pub aperperm_328: Aliased<u32, Aperperm328R::Register, Aperperm328W::Register>,
    /// Aperture 329. IPI 32B buffer at 0xFF99_0920
    pub aperperm_329: Aliased<u32, Aperperm329R::Register, Aperperm329W::Register>,
    /// Aperture 330. IPI 32B buffer at 0xFF99_0940
    pub aperperm_330: Aliased<u32, Aperperm330R::Register, Aperperm330W::Register>,
    /// Aperture 331. IPI 32B buffer at 0xFF99_0960
    pub aperperm_331: Aliased<u32, Aperperm331R::Register, Aperperm331W::Register>,
    /// Aperture 332. IPI 32B buffer at 0xFF99_0980
    pub aperperm_332: Aliased<u32, Aperperm332R::Register, Aperperm332W::Register>,
    /// Aperture 333. IPI 32B buffer at 0xFF99_09A0
    pub aperperm_333: Aliased<u32, Aperperm333R::Register, Aperperm333W::Register>,
    /// Aperture 334. IPI 32B buffer at 0xFF99_09C0
    pub aperperm_334: Aliased<u32, Aperperm334R::Register, Aperperm334W::Register>,
    /// Aperture 335. IPI 32B buffer at 0xFF99_09E0
    pub aperperm_335: Aliased<u32, Aperperm335R::Register, Aperperm335W::Register>,
    /// Aperture 336. IPI 32B buffer at 0xFF99_0A00
    pub aperperm_336: Aliased<u32, Aperperm336R::Register, Aperperm336W::Register>,
    /// Aperture 337. IPI 32B buffer at 0xFF99_0A20
    pub aperperm_337: Aliased<u32, Aperperm337R::Register, Aperperm337W::Register>,
    /// Aperture 338. IPI 32B buffer at 0xFF99_0A40
    pub aperperm_338: Aliased<u32, Aperperm338R::Register, Aperperm338W::Register>,
    /// Aperture 339. IPI 32B buffer at 0xFF99_0A60
    pub aperperm_339: Aliased<u32, Aperperm339R::Register, Aperperm339W::Register>,
    /// Aperture 340. IPI 32B buffer at 0xFF99_0A80
    pub aperperm_340: Aliased<u32, Aperperm340R::Register, Aperperm340W::Register>,
    /// Aperture 341. IPI 32B buffer at 0xFF99_0AA0
    pub aperperm_341: Aliased<u32, Aperperm341R::Register, Aperperm341W::Register>,
    /// Aperture 342. IPI 32B buffer at 0xFF99_0AC0
    pub aperperm_342: Aliased<u32, Aperperm342R::Register, Aperperm342W::Register>,
    /// Aperture 343. IPI 32B buffer at 0xFF99_0AE0
    pub aperperm_343: Aliased<u32, Aperperm343R::Register, Aperperm343W::Register>,
    /// Aperture 344. IPI 32B buffer at 0xFF99_0B00
    pub aperperm_344: Aliased<u32, Aperperm344R::Register, Aperperm344W::Register>,
    /// Aperture 345. IPI 32B buffer at 0xFF99_0B20
    pub aperperm_345: Aliased<u32, Aperperm345R::Register, Aperperm345W::Register>,
    /// Aperture 346. IPI 32B buffer at 0xFF99_0B40
    pub aperperm_346: Aliased<u32, Aperperm346R::Register, Aperperm346W::Register>,
    /// Aperture 347. IPI 32B buffer at 0xFF99_0B60
    pub aperperm_347: Aliased<u32, Aperperm347R::Register, Aperperm347W::Register>,
    /// Aperture 348. IPI 32B buffer at 0xFF99_0B80
    pub aperperm_348: Aliased<u32, Aperperm348R::Register, Aperperm348W::Register>,
    /// Aperture 349. IPI 32B buffer at 0xFF99_0BA0
    pub aperperm_349: Aliased<u32, Aperperm349R::Register, Aperperm349W::Register>,
    /// Aperture 350. IPI 32B buffer at 0xFF99_0BC0
    pub aperperm_350: Aliased<u32, Aperperm350R::Register, Aperperm350W::Register>,
    /// Aperture 351. IPI 32B buffer at 0xFF99_0BE0
    pub aperperm_351: Aliased<u32, Aperperm351R::Register, Aperperm351W::Register>,
    /// Aperture 352. IPI 32B buffer at 0xFF99_0C00
    pub aperperm_352: Aliased<u32, Aperperm352R::Register, Aperperm352W::Register>,
    /// Aperture 353. IPI 32B buffer at 0xFF99_0C20
    pub aperperm_353: Aliased<u32, Aperperm353R::Register, Aperperm353W::Register>,
    /// Aperture 354. IPI 32B buffer at 0xFF99_0C40
    pub aperperm_354: Aliased<u32, Aperperm354R::Register, Aperperm354W::Register>,
    /// Aperture 355. IPI 32B buffer at 0xFF99_0C60
    pub aperperm_355: Aliased<u32, Aperperm355R::Register, Aperperm355W::Register>,
    /// Aperture 356. IPI 32B buffer at 0xFF99_0C80
    pub aperperm_356: Aliased<u32, Aperperm356R::Register, Aperperm356W::Register>,
    /// Aperture 357. IPI 32B buffer at 0xFF99_0CA0
    pub aperperm_357: Aliased<u32, Aperperm357R::Register, Aperperm357W::Register>,
    /// Aperture 358. IPI 32B buffer at 0xFF99_0CC0
    pub aperperm_358: Aliased<u32, Aperperm358R::Register, Aperperm358W::Register>,
    /// Aperture 359. IPI 32B buffer at 0xFF99_0CE0
    pub aperperm_359: Aliased<u32, Aperperm359R::Register, Aperperm359W::Register>,
    /// Aperture 360. IPI 32B buffer at 0xFF99_0D00
    pub aperperm_360: Aliased<u32, Aperperm360R::Register, Aperperm360W::Register>,
    /// Aperture 361. IPI 32B buffer at 0xFF99_0D20
    pub aperperm_361: Aliased<u32, Aperperm361R::Register, Aperperm361W::Register>,
    /// Aperture 362. IPI 32B buffer at 0xFF99_0D40
    pub aperperm_362: Aliased<u32, Aperperm362R::Register, Aperperm362W::Register>,
    /// Aperture 363. IPI 32B buffer at 0xFF99_0D60
    pub aperperm_363: Aliased<u32, Aperperm363R::Register, Aperperm363W::Register>,
    /// Aperture 364. IPI 32B buffer at 0xFF99_0D80
    pub aperperm_364: Aliased<u32, Aperperm364R::Register, Aperperm364W::Register>,
    /// Aperture 365. IPI 32B buffer at 0xFF99_0DA0
    pub aperperm_365: Aliased<u32, Aperperm365R::Register, Aperperm365W::Register>,
    /// Aperture 366. IPI 32B buffer at 0xFF99_0DC0
    pub aperperm_366: Aliased<u32, Aperperm366R::Register, Aperperm366W::Register>,
    /// Aperture 367. IPI 32B buffer at 0xFF99_0DE0
    pub aperperm_367: Aliased<u32, Aperperm367R::Register, Aperperm367W::Register>,
    /// Aperture 368. IPI 32B buffer at 0xFF99_0E00
    pub aperperm_368: Aliased<u32, Aperperm368R::Register, Aperperm368W::Register>,
    /// Aperture 369. IPI 32B buffer at 0xFF99_0E20
    pub aperperm_369: Aliased<u32, Aperperm369R::Register, Aperperm369W::Register>,
    /// Aperture 370. IPI 32B buffer at 0xFF99_0E40
    pub aperperm_370: Aliased<u32, Aperperm370R::Register, Aperperm370W::Register>,
    /// Aperture 371. IPI 32B buffer at 0xFF99_0E60
    pub aperperm_371: Aliased<u32, Aperperm371R::Register, Aperperm371W::Register>,
    /// Aperture 372. IPI 32B buffer at 0xFF99_0E80
    pub aperperm_372: Aliased<u32, Aperperm372R::Register, Aperperm372W::Register>,
    /// Aperture 373. IPI 32B buffer at 0xFF99_0EA0
    pub aperperm_373: Aliased<u32, Aperperm373R::Register, Aperperm373W::Register>,
    /// Aperture 374. IPI 32B buffer at 0xFF99_0EC0
    pub aperperm_374: Aliased<u32, Aperperm374R::Register, Aperperm374W::Register>,
    /// Aperture 375. IPI 32B buffer at 0xFF99_0EE0
    pub aperperm_375: Aliased<u32, Aperperm375R::Register, Aperperm375W::Register>,
    /// Aperture 376. IPI 32B buffer at 0xFF99_0F00
    pub aperperm_376: Aliased<u32, Aperperm376R::Register, Aperperm376W::Register>,
    /// Aperture 377. IPI 32B buffer at 0xFF99_0F20
    pub aperperm_377: Aliased<u32, Aperperm377R::Register, Aperperm377W::Register>,
    /// Aperture 378. IPI 32B buffer at 0xFF99_0F40
    pub aperperm_378: Aliased<u32, Aperperm378R::Register, Aperperm378W::Register>,
    /// Aperture 379. IPI 32B buffer at 0xFF99_0F60
    pub aperperm_379: Aliased<u32, Aperperm379R::Register, Aperperm379W::Register>,
    /// Aperture 380. IPI 32B buffer at 0xFF99_0F80
    pub aperperm_380: Aliased<u32, Aperperm380R::Register, Aperperm380W::Register>,
    /// Aperture 381. IPI 32B buffer at 0xFF99_0FA0
    pub aperperm_381: Aliased<u32, Aperperm381R::Register, Aperperm381W::Register>,
    /// Aperture 382. IPI 32B buffer at 0xFF99_0FC0
    pub aperperm_382: Aliased<u32, Aperperm382R::Register, Aperperm382W::Register>,
    /// Aperture 383. IPI 32B buffer at 0xFF99_0FE0
    pub aperperm_383: Aliased<u32, Aperperm383R::Register, Aperperm383W::Register>,
    /// Aperture 384. IOP Mem 1MBat 0xFE00_0000.
    pub aperperm_384: Aliased<u32, Aperperm384R::Register, Aperperm384W::Register>,
    /// Aperture 385. IOP Mem 1MBat 0xFE10_0000.
    pub aperperm_385: Aliased<u32, Aperperm385R::Register, Aperperm385W::Register>,
    /// Aperture 386. IOP Mem 1MBat 0xFE20_0000.
    pub aperperm_386: Aliased<u32, Aperperm386R::Register, Aperperm386W::Register>,
    /// Aperture 387. IOP Mem 1MBat 0xFE30_0000.
    pub aperperm_387: Aliased<u32, Aperperm387R::Register, Aperperm387W::Register>,
    /// Aperture 388. IOP Mem 1MBat 0xFE40_0000.
    pub aperperm_388: Aliased<u32, Aperperm388R::Register, Aperperm388W::Register>,
    /// Aperture 389. IOP Mem 1MBat 0xFE50_0000.
    pub aperperm_389: Aliased<u32, Aperperm389R::Register, Aperperm389W::Register>,
    /// Aperture 390. IOP Mem 1MBat 0xFE60_0000.
    pub aperperm_390: Aliased<u32, Aperperm390R::Register, Aperperm390W::Register>,
    /// Aperture 391. IOP Mem 1MBat 0xFE70_0000.
    pub aperperm_391: Aliased<u32, Aperperm391R::Register, Aperperm391W::Register>,
    /// Aperture 392. IOP Mem 1MBat 0xFE80_0000.
    pub aperperm_392: Aliased<u32, Aperperm392R::Register, Aperperm392W::Register>,
    /// Aperture 393. IOP Mem 1MBat 0xFE90_0000.
    pub aperperm_393: Aliased<u32, Aperperm393R::Register, Aperperm393W::Register>,
    /// Aperture 394. IOP Mem 1MBat 0xFEA0_0000.
    pub aperperm_394: Aliased<u32, Aperperm394R::Register, Aperperm394W::Register>,
    /// Aperture 395. IOP Mem 1MBat 0xFEB0_0000.
    pub aperperm_395: Aliased<u32, Aperperm395R::Register, Aperperm395W::Register>,
    /// Aperture 396. IOP Mem 1MBat 0xFEC0_0000.
    pub aperperm_396: Aliased<u32, Aperperm396R::Register, Aperperm396W::Register>,
    /// Aperture 397. IOP Mem 1MBat 0xFED0_0000.
    pub aperperm_397: Aliased<u32, Aperperm397R::Register, Aperperm397W::Register>,
    /// Aperture 398. IOP Mem 1MBat 0xFEE0_0000.
    pub aperperm_398: Aliased<u32, Aperperm398R::Register, Aperperm398W::Register>,
    /// Aperture 399. IOP Mem 1MBat 0xFEF0_0000.
    pub aperperm_399: Aliased<u32, Aperperm399R::Register, Aperperm399W::Register>,
    /// Aperture 400. QSPI 512-MB page at 0xC000_0000.
    pub aperperm_400: Aliased<u32, Aperperm400R::Register, Aperperm400W::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub CtrlR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        APER_PARITY_EN OFFSET(2) NUMBITS(1) [],
        MID_PARITY_EN OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub CtrlW [
        APER_PARITY_EN OFFSET(2) NUMBITS(1) [],
        MID_PARITY_EN OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrStatus1 [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        AXI_ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErrStatus2 [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        AXI_ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Poison [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        BASE OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IsrR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        APER_PARITY OFFSET(7) NUMBITS(1) [],
        APER_TZ OFFSET(6) NUMBITS(1) [],
        APER_PERM OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        MID_PARITY OFFSET(3) NUMBITS(1) [],
        MID_RO OFFSET(2) NUMBITS(1) [],
        MID_MISS OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ],
    pub IsrW [
        APER_PARITY OFFSET(7) NUMBITS(1) [],
        APER_TZ OFFSET(6) NUMBITS(1) [],
        APER_PERM OFFSET(5) NUMBITS(1) [],
        MID_PARITY OFFSET(3) NUMBITS(1) [],
        MID_RO OFFSET(2) NUMBITS(1) [],
        MID_MISS OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imr [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        APER_PARITY OFFSET(7) NUMBITS(1) [],
        APER_TZ OFFSET(6) NUMBITS(1) [],
        APER_PERM OFFSET(5) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
        MID_PARITY OFFSET(3) NUMBITS(1) [],
        MID_RO OFFSET(2) NUMBITS(1) [],
        MID_MISS OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IenR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
    ],
    pub IenW [
        APER_PARITY OFFSET(7) NUMBITS(1) [],
        APER_TZ OFFSET(6) NUMBITS(1) [],
        APER_PERM OFFSET(5) NUMBITS(1) [],
        MID_PARITY OFFSET(3) NUMBITS(1) [],
        MID_RO OFFSET(2) NUMBITS(1) [],
        MID_MISS OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdsR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RESERVED1 OFFSET(4) NUMBITS(1) [],
    ],
    pub IdsW [
        APER_PARITY OFFSET(7) NUMBITS(1) [],
        APER_TZ OFFSET(6) NUMBITS(1) [],
        APER_PERM OFFSET(5) NUMBITS(1) [],
        MID_PARITY OFFSET(3) NUMBITS(1) [],
        MID_RO OFFSET(2) NUMBITS(1) [],
        MID_MISS OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId00R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId00W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId01R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId01W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId02R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId02W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId03R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId03W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId04R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId04W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId05R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId05W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId06R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId06W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId07R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId07W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId08R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId08W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId09R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId09W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId10R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId10W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId11R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId11W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId12R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId12W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId13R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId13W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId14R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId14W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId15R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId15W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId16R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId16W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId17R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId17W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId18R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId18W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub MasterId19R [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(4) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        MID OFFSET(0) NUMBITS(10) [],
    ],
    pub MasterId19W [
        MIDP OFFSET(31) NUMBITS(1) [],
        MIDR OFFSET(30) NUMBITS(1) [],
        MIDM OFFSET(16) NUMBITS(10) [],
        MID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm000R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm000W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm001R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm001W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm002R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm002W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm003R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm003W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm004R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm004W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm005R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm005W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm006R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm006W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm007R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm007W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm008R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm008W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm009R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm009W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm010R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm010W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm011R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm011W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm012R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm012W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm013R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm013W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm014R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm014W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm015R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm015W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm016R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm016W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm017R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm017W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm018R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm018W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm019R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm019W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm020R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm020W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm021R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm021W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm022R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm022W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm023R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm023W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm024R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm024W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm025R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm025W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm026R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm026W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm027R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm027W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm028R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm028W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm029R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm029W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm030R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm030W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm031R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm031W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm032R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm032W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm033R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm033W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm034R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm034W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm035R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm035W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm036R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm036W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm037R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm037W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm038R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm038W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm039R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm039W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm040R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm040W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm041R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm041W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm042R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm042W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm043R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm043W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm044R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm044W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm045R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm045W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm046R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm046W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm047R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm047W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm048R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm048W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm049R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm049W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm050R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm050W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm051R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm051W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm052R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm052W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm053R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm053W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm054R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm054W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm055R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm055W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm056R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm056W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm057R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm057W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm058R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm058W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm059R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm059W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm060R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm060W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm061R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm061W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm062R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm062W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm063R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm063W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm064R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm064W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm065R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm065W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm066R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm066W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm067R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm067W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm068R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm068W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm069R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm069W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm070R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm070W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm071R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm071W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm072R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm072W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm073R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm073W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm074R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm074W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm075R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm075W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm076R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm076W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm077R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm077W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm078R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm078W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm079R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm079W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm080R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm080W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm081R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm081W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm082R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm082W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm083R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm083W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm084R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm084W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm085R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm085W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm086R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm086W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm087R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm087W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm088R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm088W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm089R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm089W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm090R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm090W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm091R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm091W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm092R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm092W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm093R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm093W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm094R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm094W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm095R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm095W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm096R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm096W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm097R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm097W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm098R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm098W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm099R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm099W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm100R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm100W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm101R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm101W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm102R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm102W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm103R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm103W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm104R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm104W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm105R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm105W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm106R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm106W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm107R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm107W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm108R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm108W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm109R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm109W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm110R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm110W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm111R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm111W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm112R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm112W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm113R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm113W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm114R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm114W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm115R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm115W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm116R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm116W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm117R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm117W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm118R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm118W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm119R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm119W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm120R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm120W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm121R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm121W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm122R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm122W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm123R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm123W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm124R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm124W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm125R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm125W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm126R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm126W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm127R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm127W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm128R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm128W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm129R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm129W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm130R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm130W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm131R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm131W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm132R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm132W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm133R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm133W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm134R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm134W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm135R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm135W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm136R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm136W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm137R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm137W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm138R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm138W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm139R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm139W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm140R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm140W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm141R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm141W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm142R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm142W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm143R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm143W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm144R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm144W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm145R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm145W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm146R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm146W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm147R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm147W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm148R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm148W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm149R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm149W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm150R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm150W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm151R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm151W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm152R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm152W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm153R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm153W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm154R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm154W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm155R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm155W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm156R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm156W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm157R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm157W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm158R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm158W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm159R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm159W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm160R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm160W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm161R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm161W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm162R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm162W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm163R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm163W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm164R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm164W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm165R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm165W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm166R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm166W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm167R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm167W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm168R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm168W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm169R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm169W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm170R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm170W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm171R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm171W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm172R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm172W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm173R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm173W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm174R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm174W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm175R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm175W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm176R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm176W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm177R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm177W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm178R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm178W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm179R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm179W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm180R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm180W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm181R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm181W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm182R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm182W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm183R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm183W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm184R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm184W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm185R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm185W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm186R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm186W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm187R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm187W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm188R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm188W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm189R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm189W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm190R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm190W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm191R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm191W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm192R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm192W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm193R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm193W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm194R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm194W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm195R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm195W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm196R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm196W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm197R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm197W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm198R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm198W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm199R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm199W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm200R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm200W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm201R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm201W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm202R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm202W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm203R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm203W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm204R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm204W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm205R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm205W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm206R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm206W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm207R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm207W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm208R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm208W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm209R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm209W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm210R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm210W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm211R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm211W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm212R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm212W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm213R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm213W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm214R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm214W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm215R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm215W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm216R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm216W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm217R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm217W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm218R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm218W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm219R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm219W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm220R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm220W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm221R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm221W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm222R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm222W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm223R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm223W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm224R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm224W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm225R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm225W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm226R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm226W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm227R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm227W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm228R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm228W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm229R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm229W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm230R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm230W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm231R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm231W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm232R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm232W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm233R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm233W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm234R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm234W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm235R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm235W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm236R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm236W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm237R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm237W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm238R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm238W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm239R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm239W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm240R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm240W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm241R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm241W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm242R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm242W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm243R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm243W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm244R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm244W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm245R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm245W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm246R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm246W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm247R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm247W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm248R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm248W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm249R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm249W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm250R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm250W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm251R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm251W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm252R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm252W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm253R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm253W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm254R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm254W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm255R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm255W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm256R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm256W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm257R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm257W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm258R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm258W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm259R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm259W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm260R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm260W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm261R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm261W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm262R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm262W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm263R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm263W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm264R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm264W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm265R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm265W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm266R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm266W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm267R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm267W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm268R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm268W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm269R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm269W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm270R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm270W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm271R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm271W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm272R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm272W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm273R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm273W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm274R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm274W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm275R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm275W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm276R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm276W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm277R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm277W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm278R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm278W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm279R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm279W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm280R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm280W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm281R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm281W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm282R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm282W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm283R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm283W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm284R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm284W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm285R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm285W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm286R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm286W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm287R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm287W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm288R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm288W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm289R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm289W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm290R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm290W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm291R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm291W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm292R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm292W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm293R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm293W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm294R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm294W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm295R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm295W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm296R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm296W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm297R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm297W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm298R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm298W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm299R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm299W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm300R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm300W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm301R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm301W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm302R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm302W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm303R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm303W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm304R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm304W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm305R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm305W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm306R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm306W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm307R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm307W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm308R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm308W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm309R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm309W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm310R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm310W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm311R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm311W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm312R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm312W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm313R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm313W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm314R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm314W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm315R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm315W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm316R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm316W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm317R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm317W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm318R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm318W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm319R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm319W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm320R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm320W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm321R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm321W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm322R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm322W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm323R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm323W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm324R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm324W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm325R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm325W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm326R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm326W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm327R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm327W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm328R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm328W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm329R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm329W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm330R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm330W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm331R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm331W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm332R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm332W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm333R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm333W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm334R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm334W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm335R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm335W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm336R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm336W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm337R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm337W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm338R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm338W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm339R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm339W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm340R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm340W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm341R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm341W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm342R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm342W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm343R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm343W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm344R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm344W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm345R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm345W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm346R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm346W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm347R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm347W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm348R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm348W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm349R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm349W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm350R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm350W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm351R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm351W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm352R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm352W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm353R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm353W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm354R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm354W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm355R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm355W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm356R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm356W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm357R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm357W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm358R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm358W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm359R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm359W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm360R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm360W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm361R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm361W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm362R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm362W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm363R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm363W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm364R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm364W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm365R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm365W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm366R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm366W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm367R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm367W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm368R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm368W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm369R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm369W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm370R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm370W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm371R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm371W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm372R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm372W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm373R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm373W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm374R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm374W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm375R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm375W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm376R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm376W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm377R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm377W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm378R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm378W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm379R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm379W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm380R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm380W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm381R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm381W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm382R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm382W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm383R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm383W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm384R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm384W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm385R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm385W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm386R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm386W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm387R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm387W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm388R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm388W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm389R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm389W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm390R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm390W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm391R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm391W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm392R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm392W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm393R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm393W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm394R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm394W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm395R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm395W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm396R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm396W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm397R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm397W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm398R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm398W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm399R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm399W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aperperm400R [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(20) NUMBITS(7) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ],
    pub Aperperm400W [
        PARITY OFFSET(28) NUMBITS(4) [],
        TRUSTZONE OFFSET(27) NUMBITS(1) [],
        PERMISSION OFFSET(0) NUMBITS(20) [],
    ]
];

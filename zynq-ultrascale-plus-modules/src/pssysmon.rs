// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// PS System Monitor, PS System Monitor
pub static mut AMS_PS_SYSMON: *mut Registers = 0xffa50800 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// LPD temperature measurement.
    pub temp_lpd: ReadOnly<u16>,
    _padding2: [u8; 2],
    /// VCC PS LPD voltage measurement (supply1).
    pub vcc_psintlp: ReadOnly<u16>,
    _padding6: [u8; 2],
    /// VCC PS FPD voltage measurement (supply2).
    pub vcc_psintfp: ReadOnly<u16>,
    _padding10: [u8; 2],
    /// VP_VN voltage measurement.
    pub vp_vn: ReadOnly<u16>,
    _padding14: [u8; 10],
    /// PS Aux voltage reference (supply3).
    pub vcc_psaux: ReadOnly<u16>,
    _padding26: [u8; 26],
    /// DDR I/O VCC voltage measurement.
    pub vcco_psddr: ReadOnly<u16>,
    _padding54: [u8; 2],
    /// PS IO Bank 503 voltage measurement (supply5).
    pub vcco_psio3: ReadOnly<u16>,
    _padding58: [u8; 2],
    /// PS IO Bank 500 voltage measurement (supply6).
    pub vcco_psio0: ReadOnly<u16>,
    _padding62: [u8; 66],
    /// Max Temperature measured for LPD.
    pub max_temp_lpd: ReadOnly<u16>,
    _padding130: [u8; 2],
    /// Max voltage measured for VCC PS LPD (supply1).
    pub max_vcc_psintlp: ReadOnly<u16>,
    _padding134: [u8; 2],
    /// Max voltage measured for VCC PS FPD (supply2).
    pub max_vcc_psintfp: ReadOnly<u16>,
    _padding138: [u8; 2],
    /// Max voltage measured for VCC PS Aux (supply3).
    pub max_vcc_psaux: ReadOnly<u16>,
    _padding142: [u8; 2],
    /// Min temperature measured for LPD.
    pub min_temp_lpd: ReadOnly<u16>,
    _padding146: [u8; 2],
    /// Min voltage measured for VCC PS LPD (supply1).
    pub min_vcc_psintlp: ReadOnly<u16>,
    _padding150: [u8; 2],
    /// Min voltage measured for VCC PS FPD (supply2).
    pub min_vcc_psintfp: ReadOnly<u16>,
    _padding154: [u8; 2],
    /// Min voltage measured for VCC PS Aux (supply3).
    pub min_vcc_psaux: ReadOnly<u16>,
    _padding158: [u8; 2],
    /// Max voltage measured for DDR I/O VCC (supply4).
    pub max_vcco_psddr: ReadOnly<u16>,
    _padding162: [u8; 2],
    /// Max voltage measured for VCCO PSIO 3 (supply5).
    pub max_vcco_psio3: ReadOnly<u16>,
    _padding166: [u8; 2],
    /// Max voltage measured for VCCO PSIO 0 (supply6).
    pub max_vcco_psio0: ReadOnly<u16>,
    _padding170: [u8; 6],
    /// Min voltage measured for DDR I/O VCC (supply4).
    pub min_vcco_psddr: ReadOnly<u16>,
    _padding178: [u8; 2],
    /// Min voltage measured for VCCO PSIO 3 (supply5).
    pub min_vcco_psio3: ReadOnly<u16>,
    _padding182: [u8; 2],
    /// Min voltage measured for VCCO PSIO 0 (supply6).
    pub min_vcco_psio0: ReadOnly<u16>,
    _padding186: [u8; 66],
    /// Alarm Status, Flag 0.
    pub status_flag: ReadOnly<u16, StatusFlag::Register>,
    _padding254: [u8; 2],
    /// Configuration, Reg 0.
    pub config_reg0: ReadWrite<u16, ConfigReg0::Register>,
    _padding258: [u8; 2],
    /// Configuration, Reg 1.
    pub config_reg1: ReadWrite<u16, ConfigReg1::Register>,
    _padding262: [u8; 2],
    /// Configuration, Reg 2.
    pub config_reg2: ReadWrite<u16, ConfigReg2::Register>,
    _padding266: [u8; 2],
    /// Config Reg 3: Alarm disables.
    pub config_reg3: ReadWrite<u16, ConfigReg3::Register>,
    _padding270: [u8; 2],
    /// Configuration, Reg 4.
    pub config_reg4: ReadWrite<u16, ConfigReg4::Register>,
    _padding274: [u8; 6],
    /// Sequencer Channel Inclusion, Group 2.
    pub seq_channel2: ReadWrite<u16, SeqChannel2::Register>,
    _padding282: [u8; 2],
    /// Sequencer Average Enable, Group 2.
    pub seq_average2: ReadWrite<u16>,
    _padding286: [u8; 2],
    /// Sequencer Channel Inclusion, Group 0.
    pub seq_channel0: ReadWrite<u16, SeqChannel0::Register>,
    _padding290: [u8; 6],
    /// Sequencer Average Enable, Group 0.
    pub seq_average0: ReadWrite<u16>,
    _padding298: [u8; 18],
    /// Sequencer Acquisition Time Extension, Group 0.
    pub seq_acq1: ReadWrite<u16>,
    _padding318: [u8; 2],
    /// Upper Alarm Temperature Threshold for LPD.
    pub alarm_temp_lpd_upper: ReadWrite<u16>,
    _padding322: [u8; 2],
    /// Upper Alarm Voltage Threshold for LPD (supply1).
    pub alarm_vcc_psintlp_upper: ReadWrite<u16>,
    _padding326: [u8; 2],
    /// Upper Alarm Voltage Threshold for FPD (supply2).
    pub alarm_vcc_psintfp_upper: ReadWrite<u16>,
    _padding330: [u8; 2],
    /// Upper Alarm Over-Temperature (OT) Threshold for LPD and FPD.
    pub alarm_ot_upper: ReadWrite<u16>,
    _padding334: [u8; 2],
    /// Lower Alarm Temperature Threshold and Config for LPD.
    pub alarm_temp_lpd_lower: ReadWrite<u16, AlarmTempLpdLower::Register>,
    _padding338: [u8; 2],
    /// Lower Alarm Voltage Thresholdfor LPD (supply1).
    pub alarm_lower_vcc_psintlp: ReadWrite<u16>,
    _padding342: [u8; 2],
    /// Lower Alarm Voltage Threshold for FPD (supply2).
    pub alarm_vcc_psintfp_lower: ReadWrite<u16>,
    _padding346: [u8; 2],
    /// Lower Alarm Over-Temperature (OT) Threshold and Control for LPD and FPD.
    pub alarm_ot_lower: ReadWrite<u16, AlarmOtLower::Register>,
    _padding350: [u8; 2],
    /// Upper Alarm Threshold Setting for VCC_PSAUX (supply3).
    pub alarm_vcc_psaux_upper: ReadWrite<u16>,
    _padding354: [u8; 2],
    /// Upper Alarm Threshold for DDR I/O VCC (supply4).
    pub alarm_vcco_psddr_upper: ReadWrite<u16>,
    _padding358: [u8; 2],
    /// Upper Alarm Threshold for VCCO_PSIO3 (supply5).
    pub alarm_vcco_psio3_upper: ReadWrite<u16>,
    _padding362: [u8; 2],
    /// Upper Alarm Threshold for VCCO_PSIO0 (supply6).
    pub alarm_vcco_psio0_upper: ReadWrite<u16>,
    _padding366: [u8; 2],
    /// Lower Alarm Threshold for VCC_PSAUX (supply3).
    pub alarm_vcc_psaux_lower: ReadWrite<u16>,
    _padding370: [u8; 2],
    /// Lower Alarm Threshold for DDR I/O VCC (supply4).
    pub alarm_vcco_psddr_lower: ReadWrite<u16>,
    _padding374: [u8; 2],
    /// Lower Alarm Threshold for VCCO_PSIO3 (supply5).
    pub alarm_vcco_psio3_lower: ReadWrite<u16>,
    _padding378: [u8; 2],
    /// Lower Alarm Threshold for VCCO_PSIO0 (supply6).
    pub alarm_vcco_psio0_lower: ReadWrite<u16>,
    _padding382: [u8; 2],
    /// AlarmUpper Threshold for VCCO_PSIO1 (supply7).
    pub alarm_vcco_psio1_upper: ReadWrite<u16>,
    _padding386: [u8; 2],
    /// Upper Alarm Threshold for VCCO_PSIO2 (supply8).
    pub alarm_vcco_psio2_upper: ReadWrite<u16>,
    _padding390: [u8; 2],
    /// Upper Alarm Threshold for PS_MGTRAVCC (supply9).
    pub alarm_mgtravcc_upper: ReadWrite<u16>,
    _padding394: [u8; 2],
    /// Upper Alarm Threshold for PS_MGTRAVTT (supply10).
    pub alarm_mgtravtt_upper: ReadWrite<u16>,
    _padding398: [u8; 2],
    /// AlarmUpper Threshold for VCC_PSADC (vccams).
    pub alarm_vcc_psadc_upper: ReadWrite<u16>,
    _padding402: [u8; 2],
    /// AlarmUpper Temperature Threshold for FPD (t-remote).
    pub alarm_temp_fpd_upper: ReadWrite<u16>,
    _padding406: [u8; 10],
    /// Lower Alarm Threshold for VCCO PSIO1 (supply7).
    pub alarm_vcco_psio1_lower: ReadWrite<u16>,
    _padding418: [u8; 2],
    /// Lower Alarm Threshold for VCCO_PSIO2 (supply8).
    pub alarm_vcco_psio2_lower: ReadWrite<u16>,
    _padding422: [u8; 2],
    /// Lower Alarm Threshold for PS_MGTRAVCC (supply9).
    pub alarm_mgtravcc_lower: ReadWrite<u16>,
    _padding426: [u8; 2],
    /// Lower Alarm Threshold for PS_MGTRAVTT (supply10).
    pub alarm_mgtravtt_lower: ReadWrite<u16>,
    _padding430: [u8; 2],
    /// Lower Alarm Threshold for VCC_PSADC (vccams).
    pub alarm_vcc_psadc_lower: ReadWrite<u16>,
    _padding434: [u8; 2],
    /// Lower Alarm Temperature Threshold and Control for FPD (t-remote).
    pub alarm_temp_fpd_lower: ReadWrite<u16, AlarmTempFpdLower::Register>,
    _padding438: [u8; 50],
    /// Low-Rate Sequence Channel, Group 0.
    pub seq_low_rate_channel0: ReadWrite<u16>,
    _padding490: [u8; 6],
    /// Low-Rate Sequence Channel, Group 2.
    pub seq_low_rate_channel2: ReadWrite<u16>,
    _padding498: [u8; 14],
    /// VCCO_PSIO1 voltage measurement.
    pub vcco_psio1: ReadOnly<u16>,
    _padding514: [u8; 2],
    /// VCCO_PSIO2 voltage measurement.
    pub vcco_psio2: ReadOnly<u16>,
    _padding518: [u8; 2],
    /// VCC_PS_GTR voltage measurement (VPS_MGTRAVCC).
    pub vcc_psgtr: ReadOnly<u16>,
    _padding522: [u8; 2],
    /// VTT_PS_GTR voltage measurement (VPS_MGTRAVTT).
    pub vtt_psgtr: ReadOnly<u16>,
    _padding526: [u8; 2],
    /// VCC_PSADC voltage measurement.
    pub vcc_psadc: ReadOnly<u16>,
    _padding530: [u8; 2],
    /// FPD Temperature Measurment (REMOTE).
    pub temp_fpd: ReadOnly<u16>,
    _padding534: [u8; 106],
    /// Max voltage measured for VCCO_PSIO1.
    pub max_vcco_psio1: ReadOnly<u16>,
    _padding642: [u8; 2],
    /// Max voltage measured for VCCO_PSIO2.
    pub max_vcco_psio2: ReadOnly<u16>,
    _padding646: [u8; 2],
    /// Max voltage measured for VCC_PS_GTR (VPS_MGTRAVCC).
    pub max_vcc_psgtr: ReadOnly<u16>,
    _padding650: [u8; 2],
    /// Max voltage measured for VTT_PS_GTR (VPS_MGTRAVTT).
    pub max_vtt_psgtr: ReadOnly<u16>,
    _padding654: [u8; 2],
    /// Max voltage measured for VCC_PSADC.
    pub max_psadc: ReadOnly<u16>,
    _padding658: [u8; 2],
    /// Max temperature measured for FPD (remote).
    pub max_temp_fpd: ReadOnly<u16>,
    _padding662: [u8; 10],
    /// Min voltage measured for VCCO_PSIO1.
    pub min_vcco_psio1: ReadOnly<u16>,
    _padding674: [u8; 2],
    /// Min voltage measured for VCCO_PSIO2.
    pub min_vcco_psio2: ReadOnly<u16>,
    _padding678: [u8; 2],
    /// Min voltage measured for VCC_PS_GTR (VPS_MGTRAVCC).
    pub min_vcc_psgtr: ReadOnly<u16>,
    _padding682: [u8; 2],
    /// Min voltage measured for VTT_PS_GTR (VPS_MGTRAVTT).
    pub min_vtt_psgtr: ReadOnly<u16>,
    _padding686: [u8; 2],
    /// Min voltage measured for VCC_PSADC.
    pub min_vcc_psadc: ReadOnly<u16>,
    _padding690: [u8; 2],
    /// Min temperature measured for FPD (remote).
    pub min_temp_fpd: ReadOnly<u16>,
}
tock_registers::register_bitfields! [
    u16,
    pub StatusFlag [
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(1) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        RESERVED3 OFFSET(11) NUMBITS(1) [],
        RESERVED4 OFFSET(10) NUMBITS(1) [],
        INTERNAL_REF OFFSET(9) NUMBITS(1) [],
        DISABLED OFFSET(8) NUMBITS(1) [],
        ALM_6_3 OFFSET(4) NUMBITS(4) [],
        OT OFFSET(3) NUMBITS(1) [],
        ALM_2_0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ConfigReg0 [
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(14) NUMBITS(1) [],
        AVERAGING OFFSET(12) NUMBITS(2) [],
        RESERVED2 OFFSET(11) NUMBITS(1) [],
        RESERVED3 OFFSET(10) NUMBITS(1) [],
        EC OFFSET(9) NUMBITS(1) [],
        ACQ OFFSET(8) NUMBITS(1) [],
        RESERVED4 OFFSET(6) NUMBITS(2) [],
        MUX_CHANNEL OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ConfigReg1 [
        SEQUENCE_MODE OFFSET(12) NUMBITS(4) [],
        ALARM_DISABLE6TO3 OFFSET(8) NUMBITS(4) [],
        RESERVED0 OFFSET(4) NUMBITS(4) [],
        ALARM_DISABLE2TO0 OFFSET(1) NUMBITS(3) [],
        OVER_TEMP_DISABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ConfigReg2 [
        CLOCK_DIVIDER OFFSET(8) NUMBITS(8) [],
        POWER_DOWN OFFSET(4) NUMBITS(4) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ConfigReg3 [
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(7) [],
        RESERVED2 OFFSET(7) NUMBITS(1) [],
        RESERVED3 OFFSET(6) NUMBITS(1) [],
        ALARM_DISABLE13TO8 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ConfigReg4 [
        RESERVED0 OFFSET(12) NUMBITS(4) [],
        LOW_RATE_EOS OFFSET(10) NUMBITS(2) [],
        SEQUENCE_RATE OFFSET(8) NUMBITS(2) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        RESERVED2 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub SeqChannel2 [
        RESERVED0 OFFSET(6) NUMBITS(10) [],
        TEMPERATURE_REMOTE OFFSET(5) NUMBITS(1) [],
        VCCAMS OFFSET(4) NUMBITS(1) [],
        SUPPLY10 OFFSET(3) NUMBITS(1) [],
        SUPPLY9 OFFSET(2) NUMBITS(1) [],
        SUPPLY8 OFFSET(1) NUMBITS(1) [],
        SUPPLY7 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub SeqChannel0 [
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        SUPPLY3 OFFSET(14) NUMBITS(1) [],
        RESERVED1 OFFSET(13) NUMBITS(1) [],
        RESERVED2 OFFSET(12) NUMBITS(1) [],
        RESERVED3 OFFSET(11) NUMBITS(1) [],
        SUPPLY2 OFFSET(10) NUMBITS(1) [],
        SUPPLY1 OFFSET(9) NUMBITS(1) [],
        TEMPERATURE OFFSET(8) NUMBITS(1) [],
        SUPPLY6 OFFSET(7) NUMBITS(1) [],
        SUPPLY5 OFFSET(6) NUMBITS(1) [],
        SUPPLY4 OFFSET(5) NUMBITS(1) [],
        RESERVED4 OFFSET(4) NUMBITS(1) [],
        RESERVED5 OFFSET(3) NUMBITS(1) [],
        RESERVED6 OFFSET(2) NUMBITS(1) [],
        RESERVED7 OFFSET(1) NUMBITS(1) [],
        CALIBRATION OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub AlarmTempLpdLower [
        TEMPERATURE_ALARM OFFSET(1) NUMBITS(15) [],
        THRESHOLD_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub AlarmOtLower [
        TEMPERATURE_ALARM OFFSET(1) NUMBITS(15) [],
        THRESHOLD_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub AlarmTempFpdLower [
        TEMPERATURE_ALARM OFFSET(1) NUMBITS(15) [],
        THRESHOLD_MODE OFFSET(0) NUMBITS(1) [],
    ]
];

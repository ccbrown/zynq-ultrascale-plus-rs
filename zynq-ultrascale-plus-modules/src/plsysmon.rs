// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// PL System Monitor, PL System Monitor
pub static mut AMS_PL_SYSMON: *mut Registers = 0xffa50c00 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// PL Temperature measurement.Pin: 16
    pub temp_pl: ReadOnly<u16>,
    _padding2: [u8; 2],
    /// PL Internal Voltage Voltage measurement, VCCINT.Pin: 17
    pub vccint: ReadOnly<u16>,
    _padding6: [u8; 2],
    /// PL AuxiliaryVoltage measurement, VCCAUX.Pin: 18
    pub vccaux: ReadOnly<u16>,
    _padding10: [u8; 2],
    /// Differencial analog input signal Voltage measurment.
    pub vp_vn: ReadOnly<u16>,
    _padding14: [u8; 2],
    /// ADC Reference P+ Voltage measurement.
    pub vrefp: ReadOnly<u16>,
    _padding18: [u8; 2],
    /// ADC Reference N- Voltage measurement.
    pub vrefn: ReadOnly<u16>,
    _padding22: [u8; 2],
    /// PL Block RAM Voltage measurement, VCCBRAM.Pin:19
    pub vccbram: ReadOnly<u16>,
    _padding26: [u8; 26],
    /// LPD Internal Voltage measurement, VCC_PSINTLP (supply4).Pin:20
    pub vcc_psintlp: ReadOnly<u16>,
    _padding54: [u8; 2],
    /// FPD Internal Voltage measurement, VCC_PSINTFP (supply5).Pin:21
    pub vcc_psintfp: ReadOnly<u16>,
    _padding58: [u8; 2],
    /// PS Auxiliary Voltage measurement (supply6). Pin:22
    pub vcc_psaux: ReadOnly<u16>,
    _padding62: [u8; 2],
    /// Auxiliary ch 0 Voltage measurement (VAux0).
    pub vaux00: ReadOnly<u16>,
    _padding66: [u8; 2],
    /// Auxiliary ch 1 Voltage measurement (VAux1).
    pub vaux01: ReadOnly<u16>,
    _padding70: [u8; 2],
    /// Auxiliary ch2 Voltage measurement (VAux2).
    pub vaux02: ReadOnly<u16>,
    _padding74: [u8; 2],
    /// Auxiliary ch 3 Voltage measurement (VAux3).
    pub vaux03: ReadOnly<u16>,
    _padding78: [u8; 2],
    /// Auxiliary ch 4 Voltage measurement (VAux4).
    pub vaux04: ReadOnly<u16>,
    _padding82: [u8; 2],
    /// Auxiliary ch 5 Voltage measurement (VAux5).
    pub vaux05: ReadOnly<u16>,
    _padding86: [u8; 2],
    /// Auxiliary ch 6 Voltage measurement (VAux6).
    pub vaux06: ReadOnly<u16>,
    _padding90: [u8; 2],
    /// Auxiliary ch 7 Voltage measurement (VAux7).
    pub vaux07: ReadOnly<u16>,
    _padding94: [u8; 2],
    /// Auxiliary ch 8 Voltage measurement (VAux8).
    pub vaux08: ReadOnly<u16>,
    _padding98: [u8; 2],
    /// Auxiliary ch 9 Voltage measurement (VAux9).
    pub vaux09: ReadOnly<u16>,
    _padding102: [u8; 2],
    /// Auxiliary ch 10 Voltage measurement (VAux10).
    pub vaux0a: ReadOnly<u16>,
    _padding106: [u8; 2],
    /// Auxiliary ch 11 Voltage measurement (VAux11).
    pub vaux0b: ReadOnly<u16>,
    _padding110: [u8; 2],
    /// Auxiliary ch 12 Voltage measurement (VAux12).
    pub vaux0c: ReadOnly<u16>,
    _padding114: [u8; 2],
    /// Auxiliary ch 13 Voltage measurement (VAux13).
    pub vaux0d: ReadOnly<u16>,
    _padding118: [u8; 2],
    /// Auxiliary ch 14 Voltage measurement (VAux14).
    pub vaux0e: ReadOnly<u16>,
    _padding122: [u8; 2],
    /// Auxiliary ch 15 Voltage measurement (VAux15).
    pub vaux0f: ReadOnly<u16>,
    _padding126: [u8; 2],
    /// Max Temperature reached in the PL.
    pub max_temp_pl: ReadOnly<u16>,
    _padding130: [u8; 2],
    /// Max PL Internal Voltage, VCCINT (supply1).
    pub max_vccint: ReadOnly<u16>,
    _padding134: [u8; 2],
    /// Max PL Auxiliary Voltage, VCCAUX (supply2).
    pub max_vccaux: ReadOnly<u16>,
    _padding138: [u8; 2],
    /// Max Block RAM Voltage, VCCBRAM (supply3).
    pub max_vccbram: ReadOnly<u16>,
    _padding142: [u8; 2],
    /// Min Temperature reached in the PL.
    pub min_temp_pl: ReadOnly<u16>,
    _padding146: [u8; 2],
    /// Min PL Internal Voltage, VCCINT (supply1).
    pub min_vccint: ReadOnly<u16>,
    _padding150: [u8; 2],
    /// Min PL Auxiliary Voltage, VCCAUX (supply2).
    pub min_vccaux: ReadOnly<u16>,
    _padding154: [u8; 2],
    /// Min Block RAM Voltage, VCCBRAM (supply3).
    pub min_vccbram: ReadOnly<u16>,
    _padding158: [u8; 2],
    /// Max PS LPD Voltage measurement, VCC_PSINTLP (supply4).
    pub max_vcc_psintlp: ReadOnly<u16>,
    _padding162: [u8; 2],
    /// Max PS FPD Voltage measurement, VCC_PSINTFP (supply5).
    pub max_vcc_psintfp: ReadOnly<u16>,
    _padding166: [u8; 2],
    /// Max PS Auxiliary Voltage measurement, VCC_PSAUX (supply6).
    pub max_vcc_psaux: ReadOnly<u16>,
    _padding170: [u8; 6],
    /// Min PS LPD Voltage measurement, VCC_PSINTLP (supply4).
    pub min_vcc_psintlp: ReadOnly<u16>,
    _padding178: [u8; 2],
    /// Min PS FPD Voltage measurement, VCC_PSINTFP (supply5).
    pub min_vcc_psintfp: ReadOnly<u16>,
    _padding182: [u8; 2],
    /// Min PS Auxiliary Voltage measurement, VCC_PSAUX (supply6).
    pub min_vcc_psaux: ReadOnly<u16>,
    _padding186: [u8; 62],
    /// Status Flags, Reg 1.
    pub status_flag_1: ReadOnly<u16, StatusFlag1::Register>,
    _padding250: [u8; 2],
    /// Status Flags, Reg 0.
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
    _padding266: [u8; 6],
    /// Configuration, Reg 4.
    pub config_reg4: ReadWrite<u16, ConfigReg4::Register>,
    _padding274: [u8; 2],
    /// Analog Bus Control
    pub analog_bus: ReadWrite<u16, AnalogBus::Register>,
    _padding278: [u8; 2],
    /// Sequencer Channel Inclusion, Group 2.
    pub seq_channel2: ReadWrite<u16, SeqChannel2::Register>,
    _padding282: [u8; 2],
    /// Sequencer Average Config, Group 2.
    pub seq_average2: ReadWrite<u16>,
    _padding286: [u8; 2],
    /// Sequencer Channel Inclusion, Group 0.
    pub seq_channel0: ReadWrite<u16, SeqChannel0::Register>,
    _padding290: [u8; 2],
    /// Sequencer Channel Inclusion, Group 1.
    pub seq_channel1: ReadWrite<u16, SeqChannel1::Register>,
    _padding294: [u8; 2],
    /// Sequencer Channel Averaging, Group 0.
    pub seq_average0: ReadWrite<u16>,
    _padding298: [u8; 2],
    /// Sequencer Channel Averaging, Group 1.
    pub seq_average1: ReadWrite<u16>,
    _padding302: [u8; 2],
    /// Sequencer Bipolar Input Mode, Group 0.
    pub seq_input_mode0: ReadWrite<u16>,
    _padding306: [u8; 2],
    /// Sequencer Bipolar Input Mode, Group 1.
    pub seq_input_mode1: ReadWrite<u16>,
    _padding310: [u8; 2],
    /// Sequence Acquisition Time Extension, Group 0.
    pub seq_acq0: ReadWrite<u16>,
    _padding314: [u8; 2],
    /// Sequence Acquisition Time Extension, Group 1.
    pub seq_acq1: ReadWrite<u16>,
    _padding318: [u8; 2],
    /// PL Upper temperature Alarm Threshold.
    pub alarm_temp_pl_upper: ReadWrite<u16>,
    _padding322: [u8; 2],
    /// PL Internal Voltage Alarm Threshold. VCCINT (supply1).
    pub alarm_vccint_upper: ReadWrite<u16>,
    _padding326: [u8; 2],
    /// PL Auxiliary Voltage Alarm Threshold, VCCAUX (supply2).
    pub alarm_vccaux_lower: ReadWrite<u16>,
    _padding330: [u8; 2],
    /// PL Upper over-temperature Threshold.
    pub alarm_ot_upper: ReadWrite<u16>,
    _padding334: [u8; 2],
    /// PL Lower temperature Lower Alarm Threshold and config.
    pub alarm_temp_pl_lower: ReadWrite<u16, AlarmTempPlLower::Register>,
    _padding338: [u8; 2],
    /// VCC Internal Voltage Lower Alarm Threshold, VCCINT (supply1).
    pub alarm_lower_vccint: ReadWrite<u16>,
    _padding342: [u8; 2],
    /// VCC Auxiliary Voltage Lower Alarm Threshold, VCCAUX (supply2).
    pub alarm_lower_vccaux: ReadWrite<u16>,
    _padding346: [u8; 2],
    /// Lower Alarm Threshold Setting for Temp_PL Over-Temperature (OT).
    pub alarm_ot_lower: ReadWrite<u16, AlarmOtLower::Register>,
    _padding350: [u8; 2],
    /// Upper Alarm Threshold Setting for VCCBRAM (supply3).
    pub alarm_vccbram_upper: ReadWrite<u16>,
    _padding354: [u8; 2],
    /// Upper Alarm Threshold Setting for VCC_PSINTLP (supply4).
    pub alarm_vcc_psintlp_upper: ReadWrite<u16>,
    _padding358: [u8; 2],
    /// Upper Alarm Threshold Setting for VCC_PSINTFP (supply5).
    pub alarm_vcc_psintfp_upper: ReadWrite<u16>,
    _padding362: [u8; 2],
    /// Upper Alarm Threshold Setting for VCC_PSAUX (supply6).
    pub alarm_vcc_psaux_upper: ReadWrite<u16>,
    _padding366: [u8; 2],
    /// Lower Alarm Threshold Setting for VCCBRAM (supply3).
    pub alarm_vccbram_lower: ReadWrite<u16>,
    _padding370: [u8; 2],
    /// Lower Alarm Threshold Setting for VCC_PSINTLP (supply4).
    pub alarm_vcc_psintlp_lower: ReadWrite<u16>,
    _padding374: [u8; 2],
    /// Lower Alarm Threshold Setting for VCC_PSINTFP (supply5).
    pub alarm_vcc_plintfp_lower: ReadWrite<u16>,
    _padding378: [u8; 2],
    /// Lower Alarm Threshold Setting for VCC_PSAUX (supply6).
    pub alarm_vcc_psaux_lower: ReadWrite<u16>,
    _padding382: [u8; 2],
    /// Upper Alarm Threshold Setting for VUser0 (supply7).
    pub alarm_vuser0_upper: ReadWrite<u16>,
    _padding386: [u8; 2],
    /// Upper Alarm Threshold Setting for VUser1 (supply8).
    pub alarm_vuser1_upper: ReadWrite<u16>,
    _padding390: [u8; 2],
    /// Upper Alarm Threshold Setting for VUser2 (supply9).
    pub alarm_vuser2_upper: ReadWrite<u16>,
    _padding394: [u8; 2],
    /// Upper Alarm Threshold Setting for VUser3 (supply10).
    pub alarm_vuser3_upper: ReadWrite<u16>,
    _padding398: [u8; 2],
    /// Upper Alarm Threshold Setting for PL VCCADC (vccams).
    pub alarm_vccadc_upper: ReadWrite<u16>,
    _padding402: [u8; 14],
    /// Lower Alarm Threshold Setting for VUser0 (supply7).
    pub alarm_vuser0_lower: ReadWrite<u16>,
    _padding418: [u8; 2],
    /// Lower Alarm Threshold Setting for VUser1 (supply8).
    pub alarm_vuser1_lower: ReadWrite<u16>,
    _padding422: [u8; 2],
    /// Lower Alarm Threshold Setting for VUser2 (supply9).
    pub alarm_vuser2_lower: ReadWrite<u16>,
    _padding426: [u8; 2],
    /// Lower Alarm Threshold Setting for VUser3 (supply10).
    pub alarm_vuser3_lower: ReadWrite<u16>,
    _padding430: [u8; 2],
    /// Lower Alarm Threshold Setting for PL VCCADC (vccams).
    pub alarm_vccadc_lower: ReadWrite<u16>,
    _padding434: [u8; 46],
    /// Sequencer Bipolar Input Mode, Group 2.
    pub seq_input_mode2: ReadWrite<u16>,
    _padding482: [u8; 2],
    /// Sequence Acquisition Time Extension, Group 2.
    pub seq_acq2: ReadWrite<u16>,
    _padding486: [u8; 2],
    /// Low-Rate Sequence Channel, Group 0.
    pub seq_low_rate_channel0: ReadWrite<u16>,
    _padding490: [u8; 2],
    /// Low-Rate Sequence Channel, Group 1.
    pub seq_low_rate_channel1: ReadWrite<u16>,
    _padding494: [u8; 2],
    /// Low-Rate Sequence Channel, Group 2.
    pub seq_low_rate_channel2: ReadWrite<u16>,
    _padding498: [u8; 14],
    /// VUser0 Voltage measurement (supply7).
    pub vuser0: ReadOnly<u16>,
    _padding514: [u8; 2],
    /// VUser1 Voltage measurement (supply8).Pin: 24
    pub vuser1: ReadOnly<u16>,
    _padding518: [u8; 2],
    /// VUser2 Voltage measurement (supply9).Pin: 25
    pub vuser2: ReadOnly<u16>,
    _padding522: [u8; 2],
    /// VUser3 Voltage measurement (supply10). Pin: 26
    pub vuser3: ReadOnly<u16>,
    _padding526: [u8; 2],
    /// PL VCCADC Voltage measurement (vccams).
    pub vccadc: ReadOnly<u16>,
    _padding530: [u8; 110],
    /// Max voltage measured for VUser0 (supply7).
    pub max_vuser0: ReadOnly<u16>,
    _padding642: [u8; 2],
    /// Max voltage measured for VUser1 (supply8).
    pub max_vuser1: ReadOnly<u16>,
    _padding646: [u8; 2],
    /// Max voltage measured for VUser2 (supply9).
    pub max_vuser2: ReadOnly<u16>,
    _padding650: [u8; 2],
    /// Max voltage measured for VUser3 (supply10).
    pub max_vuser3: ReadOnly<u16>,
    _padding654: [u8; 2],
    /// Max voltage measured for PL VCCAD (vccams).
    pub max_vccadc: ReadOnly<u16>,
    _padding658: [u8; 14],
    /// Min voltage measured for VUser0 (supply7).
    pub min_vuser0: ReadOnly<u16>,
    _padding674: [u8; 2],
    /// Min voltage measured for VUser1 (supply8).
    pub min_vuser1: ReadOnly<u16>,
    _padding678: [u8; 2],
    /// Min voltage measured for VUser2 (supply9).
    pub min_vuser2: ReadOnly<u16>,
    _padding682: [u8; 2],
    /// Min voltage measured for VUser3 (supply10).
    pub min_vuser3: ReadOnly<u16>,
    _padding686: [u8; 2],
    /// Min voltage measured for PL VCCADC (vccams).
    pub min_vccadc: ReadOnly<u16>,
}
tock_registers::register_bitfields! [
    u16,
    pub StatusFlag1 [
        RESERVED0 OFFSET(5) NUMBITS(1) [],
        ALM_VCCAMS OFFSET(4) NUMBITS(1) [],
        ALM_10_7 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub StatusFlag [
        CLK_OSC_USED OFFSET(15) NUMBITS(1) [],
        BLOCK_IN_RESET OFFSET(14) NUMBITS(1) [],
        RESERVED0 OFFSET(12) NUMBITS(2) [],
        JTAG_DISABLED OFFSET(11) NUMBITS(1) [],
        JTAG_READ_ONLY OFFSET(10) NUMBITS(1) [],
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
        EXTERNAL_MUX OFFSET(11) NUMBITS(1) [],
        BU OFFSET(10) NUMBITS(1) [],
        EC OFFSET(9) NUMBITS(1) [],
        ACQ OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(6) NUMBITS(2) [],
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
        RESERVED0 OFFSET(4) NUMBITS(4) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        RESERVED2 OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub ConfigReg4 [
        RESERVED0 OFFSET(12) NUMBITS(4) [],
        LOW_RATE_EOS OFFSET(10) NUMBITS(2) [],
        SEQUENCE_RATE OFFSET(8) NUMBITS(2) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        VUSER_ENABLE_HRANGE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub AnalogBus [
        VUSER3 OFFSET(12) NUMBITS(4) [],
        VUSER2 OFFSET(8) NUMBITS(4) [],
        VUSER1 OFFSET(4) NUMBITS(4) [],
        VUSER0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub SeqChannel2 [
        RESERVED0 OFFSET(6) NUMBITS(10) [],
        RESERVED1 OFFSET(5) NUMBITS(1) [],
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
        CURRENT_MON OFFSET(15) NUMBITS(1) [],
        VCCBRAM OFFSET(14) NUMBITS(1) [],
        VREFN OFFSET(13) NUMBITS(1) [],
        VREFP OFFSET(12) NUMBITS(1) [],
        VP_VN OFFSET(11) NUMBITS(1) [],
        VCCAUX OFFSET(10) NUMBITS(1) [],
        VCCINT OFFSET(9) NUMBITS(1) [],
        TEMPERATURE OFFSET(8) NUMBITS(1) [],
        VCC_PSAUX OFFSET(7) NUMBITS(1) [],
        VCC_PSINTFP OFFSET(6) NUMBITS(1) [],
        VCC_PSINTLP OFFSET(5) NUMBITS(1) [],
        RESERVED0 OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        RESERVED2 OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(1) [],
        CALIBRATION OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub SeqChannel1 [
        VAUX0F OFFSET(15) NUMBITS(1) [],
        VAUX0E OFFSET(14) NUMBITS(1) [],
        VAUX0D OFFSET(13) NUMBITS(1) [],
        VAUX0C OFFSET(12) NUMBITS(1) [],
        VAUX0B OFFSET(11) NUMBITS(1) [],
        VAUX0A OFFSET(10) NUMBITS(1) [],
        VAUX09 OFFSET(9) NUMBITS(1) [],
        VAUX08 OFFSET(8) NUMBITS(1) [],
        VAUX07 OFFSET(7) NUMBITS(1) [],
        VAUX06 OFFSET(6) NUMBITS(1) [],
        VAUX05 OFFSET(5) NUMBITS(1) [],
        VAUX04 OFFSET(4) NUMBITS(1) [],
        VAUX03 OFFSET(3) NUMBITS(1) [],
        VAUX02 OFFSET(2) NUMBITS(1) [],
        VAUX01 OFFSET(1) NUMBITS(1) [],
        VAUX00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u16,
    pub AlarmTempPlLower [
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

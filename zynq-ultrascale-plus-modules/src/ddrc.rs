// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// DDR Memory Controller, DDR Memory Controller
pub static mut DDRC: *mut Registers = 0xfd070000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Master Register
    pub mstr: ReadWrite<u32, Mstr::Register>,
    /// Operating Mode Status Register
    pub stat: ReadOnly<u32, Stat::Register>,
    _padding8: [u8; 8],
    /// Mode Register Read/Write Control Register 0.Note: Do not enable more than one of the following fields simultaneously:- sw_init_int- pda_en- mpr_en
    pub mrctrl0: ReadWrite<u32, Mrctrl0::Register>,
    /// Mode Register Read/Write Control Register 1
    pub mrctrl1: ReadWrite<u32, Mrctrl1::Register>,
    /// Mode Register Read/Write Status Register
    pub mrstat: ReadOnly<u32, Mrstat::Register>,
    /// Mode Register Read/Write Control Register 2
    pub mrctrl2: ReadWrite<u32>,
    /// Temperature Derate Enable Register
    pub derateen: ReadWrite<u32, Derateen::Register>,
    /// Temperature Derate Interval Register
    pub derateint: ReadWrite<u32>,
    _padding40: [u8; 8],
    /// Low Power Control Register
    pub pwrctl: ReadWrite<u32, Pwrctl::Register>,
    /// Low Power Timing Register
    pub pwrtmg: ReadWrite<u32, Pwrtmg::Register>,
    /// Hardware Low Power Control Register
    pub hwlpctl: ReadWrite<u32, Hwlpctl::Register>,
    _padding60: [u8; 20],
    /// Refresh Control Register 0
    pub rfshctl0: ReadWrite<u32, Rfshctl0::Register>,
    /// Refresh Control Register 1
    pub rfshctl1: ReadWrite<u32, Rfshctl1::Register>,
    _padding88: [u8; 8],
    /// Refresh Control Register 3
    pub rfshctl3: ReadWrite<u32, Rfshctl3::Register>,
    /// Refresh Timing Register
    pub rfshtmg: ReadWrite<u32, Rfshtmg::Register>,
    _padding104: [u8; 8],
    /// ECC Configuration Register 0
    pub ecccfg0: ReadWrite<u32, Ecccfg0::Register>,
    /// ECC Configuration Register 1
    pub ecccfg1: ReadWrite<u32, Ecccfg1::Register>,
    /// ECC Status Register
    pub eccstat: ReadOnly<u32, Eccstat::Register>,
    /// ECC Clear Register
    pub eccclr: ReadWrite<u32, Eccclr::Register>,
    /// ECC Error Counter Register
    pub eccerrcnt: ReadOnly<u32, Eccerrcnt::Register>,
    /// ECC Corrected Error Address Register 0
    pub ecccaddr0: ReadOnly<u32, Ecccaddr0::Register>,
    /// ECC Corrected Error Address Register 1
    pub ecccaddr1: ReadOnly<u32, Ecccaddr1::Register>,
    /// ECC Corrected Syndrome Register 0
    pub ecccsyn0: ReadOnly<u32>,
    /// ECC Corrected Syndrome Register 1
    pub ecccsyn1: ReadOnly<u32>,
    /// ECC Corrected Syndrome Register 2
    pub ecccsyn2: ReadOnly<u32, Ecccsyn2::Register>,
    /// ECC Corrected Data Bit Mask Register 0
    pub eccbitmask0: ReadOnly<u32>,
    /// ECC Corrected Data Bit Mask Register 1
    pub eccbitmask1: ReadOnly<u32>,
    /// ECC Corrected Data Bit Mask Register 2
    pub eccbitmask2: ReadOnly<u32, Eccbitmask2::Register>,
    /// ECC Uncorrected Error Address Register 0
    pub eccuaddr0: ReadOnly<u32, Eccuaddr0::Register>,
    /// ECC Uncorrected Error Address Register 1
    pub eccuaddr1: ReadOnly<u32, Eccuaddr1::Register>,
    /// ECC Uncorrected Syndrome Register 0
    pub eccusyn0: ReadOnly<u32>,
    /// ECC Uncorrected Syndrome Register 1
    pub eccusyn1: ReadOnly<u32>,
    /// ECC Uncorrected Syndrome Register 2
    pub eccusyn2: ReadOnly<u32, Eccusyn2::Register>,
    /// ECC Data Poisoning Address Register 0.If a write transaction matches the address specified in this register, an ECC error will be introduced on that transaction, if ECCCFG1.data_poison_en=1
    pub eccpoisonaddr0: ReadWrite<u32, Eccpoisonaddr0::Register>,
    /// ECC Data Poisoning Address Register 1.If a write transaction matches the address specified in this register, an ECC error will be introduced on that transaction, if ECCCFG1.data_poison_en=1
    pub eccpoisonaddr1: ReadWrite<u32, Eccpoisonaddr1::Register>,
    /// CRC Parity Control Register0
    pub crcparctl0: ReadWrite<u32, Crcparctl0::Register>,
    /// CRC Parity Control Register1
    pub crcparctl1: ReadWrite<u32, Crcparctl1::Register>,
    /// CRC Parity Control Register2
    pub crcparctl2: ReadWrite<u32, Crcparctl2::Register>,
    /// CRC Parity Status Register
    pub crcparstat: ReadOnly<u32, Crcparstat::Register>,
    /// SDRAM Initialization Register 0
    pub init0: ReadWrite<u32, Init0::Register>,
    /// SDRAM Initialization Register 1
    pub init1: ReadWrite<u32, Init1::Register>,
    /// SDRAM Initialization Register 2
    pub init2: ReadWrite<u32, Init2::Register>,
    /// SDRAM Initialization Register 3
    pub init3: ReadWrite<u32, Init3::Register>,
    /// SDRAM Initialization Register 4
    pub init4: ReadWrite<u32, Init4::Register>,
    /// SDRAM Initialization Register 5
    pub init5: ReadWrite<u32, Init5::Register>,
    /// SDRAM Initialization Register 6
    pub init6: ReadWrite<u32, Init6::Register>,
    /// SDRAM Initialization Register 7
    pub init7: ReadWrite<u32, Init7::Register>,
    /// DIMM Control Register
    pub dimmctl: ReadWrite<u32, Dimmctl::Register>,
    /// Rank Control Register
    pub rankctl: ReadWrite<u32, Rankctl::Register>,
    _padding248: [u8; 8],
    /// SDRAM Timing Register 0
    pub dramtmg0: ReadWrite<u32, Dramtmg0::Register>,
    /// SDRAM Timing Register 1
    pub dramtmg1: ReadWrite<u32, Dramtmg1::Register>,
    /// SDRAM Timing Register 2
    pub dramtmg2: ReadWrite<u32, Dramtmg2::Register>,
    /// SDRAM Timing Register 3
    pub dramtmg3: ReadWrite<u32, Dramtmg3::Register>,
    /// SDRAM Timing Register 4
    pub dramtmg4: ReadWrite<u32, Dramtmg4::Register>,
    /// SDRAM Timing Register 5
    pub dramtmg5: ReadWrite<u32, Dramtmg5::Register>,
    /// SDRAM Timing Register 6
    pub dramtmg6: ReadWrite<u32, Dramtmg6::Register>,
    /// SDRAM Timing Register 7
    pub dramtmg7: ReadWrite<u32, Dramtmg7::Register>,
    /// SDRAM Timing Register 8
    pub dramtmg8: ReadWrite<u32, Dramtmg8::Register>,
    /// SDRAM Timing Register 9
    pub dramtmg9: ReadWrite<u32, Dramtmg9::Register>,
    /// SDRAM Timing Register 10
    pub dramtmg10: ReadWrite<u32, Dramtmg10::Register>,
    /// SDRAM Timing Register 11
    pub dramtmg11: ReadWrite<u32, Dramtmg11::Register>,
    /// SDRAM Timing Register 12
    pub dramtmg12: ReadWrite<u32, Dramtmg12::Register>,
    /// SDRAM Timing Register 13
    pub dramtmg13: ReadWrite<u32, Dramtmg13::Register>,
    /// SDRAM Timing Register 14
    pub dramtmg14: ReadWrite<u32, Dramtmg14::Register>,
    _padding316: [u8; 68],
    /// ZQ Control Register 0
    pub zqctl0: ReadWrite<u32, Zqctl0::Register>,
    /// ZQ Control Register 1
    pub zqctl1: ReadWrite<u32, Zqctl1::Register>,
    /// ZQ Control Register 2
    pub zqctl2: ReadWrite<u32, Zqctl2::Register>,
    /// ZQ Status Register
    pub zqstat: ReadOnly<u32, Zqstat::Register>,
    /// DFI Timing Register 0
    pub dfitmg0: ReadWrite<u32, Dfitmg0::Register>,
    /// DFI Timing Register 1
    pub dfitmg1: ReadWrite<u32, Dfitmg1::Register>,
    /// DFI Low Power Configuration Register 0
    pub dfilpcfg0: ReadWrite<u32, Dfilpcfg0::Register>,
    /// DFI Low Power Configuration Register 1
    pub dfilpcfg1: ReadWrite<u32, Dfilpcfg1::Register>,
    /// DFI Update Register 0
    pub dfiupd0: ReadWrite<u32, Dfiupd0::Register>,
    /// DFI Update Register 1
    pub dfiupd1: ReadWrite<u32, Dfiupd1::Register>,
    /// DFI Update Register 2
    pub dfiupd2: ReadWrite<u32, Dfiupd2::Register>,
    _padding428: [u8; 4],
    /// DFI Miscellaneous Control Register
    pub dfimisc: ReadWrite<u32, Dfimisc::Register>,
    /// DFI Timing Register 2
    pub dfitmg2: ReadWrite<u32, Dfitmg2::Register>,
    _padding440: [u8; 8],
    /// DM/DBI Control Register
    pub dbictl: ReadWrite<u32, Dbictl::Register>,
    _padding452: [u8; 60],
    /// Address Map Register 0
    pub addrmap0: ReadWrite<u32, Addrmap0::Register>,
    /// Address Map Register 1
    pub addrmap1: ReadWrite<u32, Addrmap1::Register>,
    /// Address Map Register 2
    pub addrmap2: ReadWrite<u32, Addrmap2::Register>,
    /// Address Map Register 3
    pub addrmap3: ReadWrite<u32, Addrmap3::Register>,
    /// Address Map Register 4
    pub addrmap4: ReadWrite<u32, Addrmap4::Register>,
    /// Address Map Register 5
    pub addrmap5: ReadWrite<u32, Addrmap5::Register>,
    /// Address Map Register 6
    pub addrmap6: ReadWrite<u32, Addrmap6::Register>,
    /// Address Map Register 7
    pub addrmap7: ReadWrite<u32, Addrmap7::Register>,
    /// Address Map Register 8
    pub addrmap8: ReadWrite<u32, Addrmap8::Register>,
    /// Address Map Register 9
    pub addrmap9: ReadWrite<u32, Addrmap9::Register>,
    /// Address Map Register 10
    pub addrmap10: ReadWrite<u32, Addrmap10::Register>,
    /// Address Map Register 11
    pub addrmap11: ReadWrite<u32, Addrmap11::Register>,
    _padding560: [u8; 16],
    /// ODT Configuration Register
    pub odtcfg: ReadWrite<u32, Odtcfg::Register>,
    /// ODT/Rank Map Register
    pub odtmap: ReadWrite<u32, Odtmap::Register>,
    _padding584: [u8; 8],
    /// Scheduler Control Register
    pub sched: ReadWrite<u32, Sched::Register>,
    /// Scheduler Control Register 1
    pub sched1: ReadWrite<u32, Sched1::Register>,
    _padding600: [u8; 4],
    /// High Priority Read CAM Register 1
    pub perfhpr1: ReadWrite<u32, Perfhpr1::Register>,
    _padding608: [u8; 4],
    /// Low Priority Read CAM Register 1
    pub perflpr1: ReadWrite<u32, Perflpr1::Register>,
    _padding616: [u8; 4],
    /// Write CAM Register 1
    pub perfwr1: ReadWrite<u32, Perfwr1::Register>,
    _padding624: [u8; 4],
    /// Variable Priority Read CAM Register 1
    pub perfvpr1: ReadWrite<u32, Perfvpr1::Register>,
    /// Variable Priority Write CAM Register 1
    pub perfvpw1: ReadWrite<u32, Perfvpw1::Register>,
    _padding636: [u8; 4],
    /// DQ Map Register 0
    pub dqmap0: ReadWrite<u32, Dqmap0::Register>,
    /// DQ Map Register 1
    pub dqmap1: ReadWrite<u32, Dqmap1::Register>,
    /// DQ Map Register 2
    pub dqmap2: ReadWrite<u32, Dqmap2::Register>,
    /// DQ Map Register 3
    pub dqmap3: ReadWrite<u32, Dqmap3::Register>,
    /// DQ Map Register 4
    pub dqmap4: ReadWrite<u32, Dqmap4::Register>,
    /// DQ Map Register 5
    pub dqmap5: ReadWrite<u32, Dqmap5::Register>,
    _padding664: [u8; 108],
    /// Debug Register 1
    pub dbg1: ReadWrite<u32, Dbg1::Register>,
    /// CAM Debug Register
    pub dbgcam: ReadOnly<u32, Dbgcam::Register>,
    /// Command Debug Register
    pub dbgcmd: ReadWrite<u32, Dbgcmd::Register>,
    /// Status Debug Register
    pub dbgstat: ReadOnly<u32, Dbgstat::Register>,
    _padding788: [u8; 12],
    /// Software register programming control enable
    pub swctl: ReadWrite<u32, Swctl::Register>,
    /// Software register programming control status
    pub swstat: ReadOnly<u32, Swstat::Register>,
    _padding808: [u8; 68],
    /// AXI Poison Configuration Register
    pub poisoncfg: ReadWrite<u32, Poisoncfg::Register>,
    /// AXI Poison status register
    pub poisonstat: ReadOnly<u32, Poisonstat::Register>,
    _padding884: [u8; 136],
    /// Port Status Register
    pub pstat: ReadOnly<u32, Pstat::Register>,
    /// Port Common Configuration Register
    pub pccfg: ReadWrite<u32, Pccfg::Register>,
    /// Port 0 Configuration Read Register
    pub pcfgr_0: ReadWrite<u32, Pcfgr0::Register>,
    /// Port 0 Configuration Write Register
    pub pcfgw_0: ReadWrite<u32, Pcfgw0::Register>,
    _padding1036: [u8; 132],
    /// Port 0 Control Register
    pub pctrl_0: ReadWrite<u32, Pctrl0::Register>,
    /// Port 0 Read QoS Configuration Register 0
    pub pcfgqos0_0: ReadWrite<u32, Pcfgqos00::Register>,
    /// Port 0 Read QoS Configuration Register 1
    pub pcfgqos1_0: ReadWrite<u32, Pcfgqos10::Register>,
    /// Port 0 Write QoS Configuration Register 0
    pub pcfgwqos0_0: ReadWrite<u32, Pcfgwqos00::Register>,
    /// Port 0 Write QoS Configuration Register 1
    pub pcfgwqos1_0: ReadWrite<u32, Pcfgwqos10::Register>,
    _padding1188: [u8; 16],
    /// Port 1 Configuration Read Register
    pub pcfgr_1: ReadWrite<u32, Pcfgr1::Register>,
    /// Port 1 Configuration Write Register
    pub pcfgw_1: ReadWrite<u32, Pcfgw1::Register>,
    _padding1212: [u8; 132],
    /// Port 1 Control Register
    pub pctrl_1: ReadWrite<u32, Pctrl1::Register>,
    /// Port 1 Read QoS Configuration Register 0
    pub pcfgqos0_1: ReadWrite<u32, Pcfgqos01::Register>,
    /// Port 1 Read QoS Configuration Register 1
    pub pcfgqos1_1: ReadWrite<u32, Pcfgqos11::Register>,
    /// Port 1 Write QoS Configuration Register 0
    pub pcfgwqos0_1: ReadWrite<u32, Pcfgwqos01::Register>,
    /// Port 1 Write QoS Configuration Register 1
    pub pcfgwqos1_1: ReadWrite<u32, Pcfgwqos11::Register>,
    _padding1364: [u8; 16],
    /// Port 2 Configuration Read Register
    pub pcfgr_2: ReadWrite<u32, Pcfgr2::Register>,
    /// Port 2 Configuration Write Register
    pub pcfgw_2: ReadWrite<u32, Pcfgw2::Register>,
    _padding1388: [u8; 132],
    /// Port 2 Control Register
    pub pctrl_2: ReadWrite<u32, Pctrl2::Register>,
    /// Port 2 Read QoS Configuration Register 0
    pub pcfgqos0_2: ReadWrite<u32, Pcfgqos02::Register>,
    /// Port 2 Read QoS Configuration Register 1
    pub pcfgqos1_2: ReadWrite<u32, Pcfgqos12::Register>,
    /// Port 2 Write QoS Configuration Register 0
    pub pcfgwqos0_2: ReadWrite<u32, Pcfgwqos02::Register>,
    /// Port 2 Write QoS Configuration Register 1
    pub pcfgwqos1_2: ReadWrite<u32, Pcfgwqos12::Register>,
    _padding1540: [u8; 16],
    /// Port 3 Configuration Read Register
    pub pcfgr_3: ReadWrite<u32, Pcfgr3::Register>,
    /// Port 3 Configuration Write Register
    pub pcfgw_3: ReadWrite<u32, Pcfgw3::Register>,
    _padding1564: [u8; 132],
    /// Port 3 Control Register
    pub pctrl_3: ReadWrite<u32, Pctrl3::Register>,
    /// Port 3 Read QoS Configuration Register 0
    pub pcfgqos0_3: ReadWrite<u32, Pcfgqos03::Register>,
    /// Port 3 Read QoS Configuration Register 1
    pub pcfgqos1_3: ReadWrite<u32, Pcfgqos13::Register>,
    /// Port 3 Write QoS Configuration Register 0
    pub pcfgwqos0_3: ReadWrite<u32, Pcfgwqos03::Register>,
    /// Port 3 Write QoS Configuration Register 1
    pub pcfgwqos1_3: ReadWrite<u32, Pcfgwqos13::Register>,
    _padding1716: [u8; 16],
    /// Port 4 Configuration Read Register
    pub pcfgr_4: ReadWrite<u32, Pcfgr4::Register>,
    /// Port 4 Configuration Write Register
    pub pcfgw_4: ReadWrite<u32, Pcfgw4::Register>,
    _padding1740: [u8; 132],
    /// Port 4 Control Register
    pub pctrl_4: ReadWrite<u32, Pctrl4::Register>,
    /// Port 4 Read QoS Configuration Register 0
    pub pcfgqos0_4: ReadWrite<u32, Pcfgqos04::Register>,
    /// Port 4 Read QoS Configuration Register 1
    pub pcfgqos1_4: ReadWrite<u32, Pcfgqos14::Register>,
    /// Port 4 Write QoS Configuration Register 0
    pub pcfgwqos0_4: ReadWrite<u32, Pcfgwqos04::Register>,
    /// Port 4 Write QoS Configuration Register 1
    pub pcfgwqos1_4: ReadWrite<u32, Pcfgwqos14::Register>,
    _padding1892: [u8; 16],
    /// Port 5 Configuration Read Register
    pub pcfgr_5: ReadWrite<u32, Pcfgr5::Register>,
    /// Port 5 Configuration Write Register
    pub pcfgw_5: ReadWrite<u32, Pcfgw5::Register>,
    _padding1916: [u8; 132],
    /// Port 5 Control Register
    pub pctrl_5: ReadWrite<u32, Pctrl5::Register>,
    /// Port 5 Read QoS Configuration Register 0
    pub pcfgqos0_5: ReadWrite<u32, Pcfgqos05::Register>,
    /// Port 5 Read QoS Configuration Register 1
    pub pcfgqos1_5: ReadWrite<u32, Pcfgqos15::Register>,
    /// Port 5 Write QoS Configuration Register 0
    pub pcfgwqos0_5: ReadWrite<u32, Pcfgwqos05::Register>,
    /// Port 5 Write QoS Configuration Register 1
    pub pcfgwqos1_5: ReadWrite<u32, Pcfgwqos15::Register>,
    _padding2068: [u8; 1776],
    /// SAR Base Address Register 0
    pub sarbase0: ReadWrite<u32, Sarbase0::Register>,
    /// SAR Size Register 0
    pub sarsize0: ReadWrite<u32, Sarsize0::Register>,
    /// SAR Base Address Register 1
    pub sarbase1: ReadWrite<u32, Sarbase1::Register>,
    /// SAR Size Register 1
    pub sarsize1: ReadWrite<u32, Sarsize1::Register>,
    _padding3860: [u8; 4368],
    /// Temperature Derate Interval Shadow Register
    pub derateint_shadow: ReadWrite<u32>,
    _padding8232: [u8; 40],
    /// Refresh Control Shadow Register 0
    pub rfshctl0_shadow: ReadWrite<u32, Rfshctl0Shadow::Register>,
    _padding8276: [u8; 16],
    /// Refresh Timing Shadow Register
    pub rfshtmg_shadow: ReadWrite<u32, RfshtmgShadow::Register>,
    _padding8296: [u8; 116],
    /// SDRAM Initialization Shadow Register 3
    pub init3_shadow: ReadWrite<u32, Init3Shadow::Register>,
    /// SDRAM Initialization Shadow Register 4
    pub init4_shadow: ReadWrite<u32, Init4Shadow::Register>,
    _padding8420: [u8; 4],
    /// SDRAM Initialization Shadow Register 6
    pub init6_shadow: ReadWrite<u32, Init6Shadow::Register>,
    /// SDRAM Initialization Shadow Register 7
    pub init7_shadow: ReadWrite<u32, Init7Shadow::Register>,
    _padding8432: [u8; 16],
    /// SDRAM Timing Shadow Register 0
    pub dramtmg0_shadow: ReadWrite<u32, Dramtmg0Shadow::Register>,
    /// SDRAM Timing Shadow Register 1
    pub dramtmg1_shadow: ReadWrite<u32, Dramtmg1Shadow::Register>,
    /// SDRAM Timing Shadow Register 2
    pub dramtmg2_shadow: ReadWrite<u32, Dramtmg2Shadow::Register>,
    /// SDRAM Timing Shadow Register 3
    pub dramtmg3_shadow: ReadWrite<u32, Dramtmg3Shadow::Register>,
    /// SDRAM Timing Shadow Register 4
    pub dramtmg4_shadow: ReadWrite<u32, Dramtmg4Shadow::Register>,
    /// SDRAM Timing Shadow Register 5
    pub dramtmg5_shadow: ReadWrite<u32, Dramtmg5Shadow::Register>,
    /// SDRAM Timing Shadow Register 6
    pub dramtmg6_shadow: ReadWrite<u32, Dramtmg6Shadow::Register>,
    /// SDRAM Timing Shadow Register 7
    pub dramtmg7_shadow: ReadWrite<u32, Dramtmg7Shadow::Register>,
    /// SDRAM Timing Shadow Register 8
    pub dramtmg8_shadow: ReadWrite<u32, Dramtmg8Shadow::Register>,
    /// SDRAM Timing Shadow Register 9
    pub dramtmg9_shadow: ReadWrite<u32, Dramtmg9Shadow::Register>,
    /// SDRAM Timing Shadow Register 10
    pub dramtmg10_shadow: ReadWrite<u32, Dramtmg10Shadow::Register>,
    /// SDRAM Timing Shadow Register 11
    pub dramtmg11_shadow: ReadWrite<u32, Dramtmg11Shadow::Register>,
    /// SDRAM Timing Shadow Register 12
    pub dramtmg12_shadow: ReadWrite<u32, Dramtmg12Shadow::Register>,
    /// SDRAM Timing Shadow Register 13
    pub dramtmg13_shadow: ReadWrite<u32, Dramtmg13Shadow::Register>,
    /// SDRAM Timing Shadow Register 14
    pub dramtmg14_shadow: ReadWrite<u32, Dramtmg14Shadow::Register>,
    _padding8508: [u8; 68],
    /// ZQ Control Shadow Register 0
    pub zqctl0_shadow: ReadWrite<u32, Zqctl0Shadow::Register>,
    _padding8580: [u8; 12],
    /// DFI Timing Shadow Register 0
    pub dfitmg0_shadow: ReadWrite<u32, Dfitmg0Shadow::Register>,
    /// DFI Timing Shadow Register 1
    pub dfitmg1_shadow: ReadWrite<u32, Dfitmg1Shadow::Register>,
    _padding8600: [u8; 28],
    /// DFI Timing Shadow Register 2
    pub dfitmg2_shadow: ReadWrite<u32, Dfitmg2Shadow::Register>,
    _padding8632: [u8; 136],
    /// ODT Configuration Shadow Register
    pub odtcfg_shadow: ReadWrite<u32, OdtcfgShadow::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Mstr [
        DEVICE_CONFIG OFFSET(30) NUMBITS(2) [],
        FREQUENCY_MODE OFFSET(29) NUMBITS(1) [],
        ACTIVE_RANKS OFFSET(24) NUMBITS(2) [],
        BURST_RDWR OFFSET(16) NUMBITS(4) [],
        DLL_OFF_MODE OFFSET(15) NUMBITS(1) [],
        DATA_BUS_WIDTH OFFSET(12) NUMBITS(2) [],
        RESERVED0 OFFSET(11) NUMBITS(1) [],
        EN_2T_TIMING_MODE OFFSET(10) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(1) [],
        LPDDR4 OFFSET(5) NUMBITS(1) [],
        DDR4 OFFSET(4) NUMBITS(1) [],
        LPDDR3 OFFSET(3) NUMBITS(1) [],
        RESERVED2 OFFSET(2) NUMBITS(1) [],
        DDR3 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Stat [
        SELFREF_STATE OFFSET(8) NUMBITS(2) [],
        SELFREF_TYPE OFFSET(4) NUMBITS(2) [],
        OPERATING_MODE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mrctrl0 [
        MR_WR OFFSET(31) NUMBITS(1) [],
        MR_ADDR OFFSET(12) NUMBITS(4) [],
        MR_RANK OFFSET(4) NUMBITS(2) [],
        SW_INIT_INT OFFSET(3) NUMBITS(1) [],
        PDA_EN OFFSET(2) NUMBITS(1) [],
        MPR_EN OFFSET(1) NUMBITS(1) [],
        MR_TYPE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mrctrl1 [
        MR_DATA OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mrstat [
        PDA_DONE OFFSET(8) NUMBITS(1) [],
        MR_WR_BUSY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Derateen [
        RC_DERATE_VALUE OFFSET(8) NUMBITS(2) [],
        DERATE_BYTE OFFSET(4) NUMBITS(4) [],
        DERATE_VALUE OFFSET(1) NUMBITS(1) [],
        DERATE_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pwrctl [
        STAY_IN_SELFREF OFFSET(6) NUMBITS(1) [],
        SELFREF_SW OFFSET(5) NUMBITS(1) [],
        MPSM_EN OFFSET(4) NUMBITS(1) [],
        EN_DFI_DRAM_CLK_DISABLE OFFSET(3) NUMBITS(1) [],
        DEEPPOWERDOWN_EN OFFSET(2) NUMBITS(1) [],
        POWERDOWN_EN OFFSET(1) NUMBITS(1) [],
        SELFREF_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pwrtmg [
        SELFREF_TO_X32 OFFSET(16) NUMBITS(8) [],
        T_DPD_X4096 OFFSET(8) NUMBITS(8) [],
        POWERDOWN_TO_X32 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Hwlpctl [
        HW_LP_IDLE_X32 OFFSET(16) NUMBITS(12) [],
        HW_LP_EXIT_IDLE_EN OFFSET(1) NUMBITS(1) [],
        HW_LP_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rfshctl0 [
        REFRESH_MARGIN OFFSET(20) NUMBITS(4) [],
        REFRESH_TO_X32 OFFSET(12) NUMBITS(5) [],
        REFRESH_BURST OFFSET(4) NUMBITS(5) [],
        PER_BANK_REFRESH OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rfshctl1 [
        REFRESH_TIMER1_START_VALUE_X32 OFFSET(16) NUMBITS(12) [],
        REFRESH_TIMER0_START_VALUE_X32 OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rfshctl3 [
        REFRESH_MODE OFFSET(4) NUMBITS(3) [],
        REFRESH_UPDATE_LEVEL OFFSET(1) NUMBITS(1) [],
        DIS_AUTO_REFRESH OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rfshtmg [
        T_RFC_NOM_X32 OFFSET(16) NUMBITS(12) [],
        LPDDR3_TREFBW_EN OFFSET(15) NUMBITS(1) [],
        T_RFC_MIN OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ecccfg0 [
        DIS_SCRUB OFFSET(4) NUMBITS(1) [],
        ECC_MODE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ecccfg1 [
        DATA_POISON_BIT OFFSET(1) NUMBITS(1) [],
        DATA_POISON_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eccstat [
        ECC_UNCORRECTED_ERR OFFSET(16) NUMBITS(4) [],
        ECC_CORRECTED_ERR OFFSET(8) NUMBITS(4) [],
        ECC_CORRECTED_BIT_NUM OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eccclr [
        ECC_CLR_UNCORR_ERR_CNT OFFSET(3) NUMBITS(1) [],
        ECC_CLR_CORR_ERR_CNT OFFSET(2) NUMBITS(1) [],
        RESERVED0 OFFSET(1) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eccerrcnt [
        ECC_UNCORR_ERR_CNT OFFSET(16) NUMBITS(16) [],
        ECC_CORR_ERR_CNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ecccaddr0 [
        ECC_CORR_RANK OFFSET(24) NUMBITS(1) [],
        ECC_CORR_ROW OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ecccaddr1 [
        ECC_CORR_BG OFFSET(24) NUMBITS(2) [],
        ECC_CORR_BANK OFFSET(16) NUMBITS(3) [],
        ECC_CORR_COL OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ecccsyn2 [
        ECC_CORR_SYNDROMES_71_64 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eccbitmask2 [
        ECC_CORR_BIT_MASK_71_64 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eccuaddr0 [
        ECC_UNCORR_RANK OFFSET(24) NUMBITS(1) [],
        ECC_UNCORR_ROW OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eccuaddr1 [
        ECC_UNCORR_BG OFFSET(24) NUMBITS(2) [],
        ECC_UNCORR_BANK OFFSET(16) NUMBITS(3) [],
        ECC_UNCORR_COL OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eccusyn2 [
        ECC_UNCORR_SYNDROMES_71_64 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eccpoisonaddr0 [
        ECC_POISON_RANK OFFSET(24) NUMBITS(1) [],
        ECC_POISON_COL OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Eccpoisonaddr1 [
        ECC_POISON_BG OFFSET(28) NUMBITS(2) [],
        ECC_POISON_BANK OFFSET(24) NUMBITS(3) [],
        ECC_POISON_ROW OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Crcparctl0 [
        RETRY_CTRLUPD_ENABLE OFFSET(15) NUMBITS(1) [],
        DFI_ALERT_ERR_MAX_REACHED_INT_CLR OFFSET(8) NUMBITS(1) [],
        DFI_ALERT_ERR_FATL_INT_CLR OFFSET(4) NUMBITS(1) [],
        DFI_ALERT_ERR_CNT_CLR OFFSET(2) NUMBITS(1) [],
        DFI_ALERT_ERR_INT_CLR OFFSET(1) NUMBITS(1) [],
        DFI_ALERT_ERR_INT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Crcparctl1 [
        DFI_T_PHY_RDLAT OFFSET(24) NUMBITS(6) [],
        ALERT_WAIT_FOR_SW OFFSET(9) NUMBITS(1) [],
        CRC_PARITY_RETRY_ENABLE OFFSET(8) NUMBITS(1) [],
        CRC_INC_DM OFFSET(7) NUMBITS(1) [],
        CRC_ENABLE OFFSET(4) NUMBITS(1) [],
        PARITY_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Crcparctl2 [
        T_PAR_ALERT_PW_MAX OFFSET(16) NUMBITS(9) [],
        T_CRC_ALERT_PW_MAX OFFSET(8) NUMBITS(5) [],
        RETRY_FIFO_MAX_HOLD_TIMER_X4 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Crcparstat [
        CMD_IN_ERR_WINDOW OFFSET(29) NUMBITS(1) [],
        RETRY_OPERATING_MODE OFFSET(28) NUMBITS(1) [],
        RETRY_CURRENT_STATE OFFSET(24) NUMBITS(4) [],
        DFI_ALERT_ERR_FATL_CODE OFFSET(20) NUMBITS(3) [],
        DFI_ALERT_ERR_NO_SW OFFSET(19) NUMBITS(1) [],
        DFI_ALERT_ERR_MAX_REACHED_INT OFFSET(18) NUMBITS(1) [],
        DFI_ALERT_ERR_FATL_INT OFFSET(17) NUMBITS(1) [],
        DFI_ALERT_ERR_INT OFFSET(16) NUMBITS(1) [],
        DFI_ALERT_ERR_CNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init0 [
        SKIP_DRAM_INIT OFFSET(30) NUMBITS(2) [],
        POST_CKE_X1024 OFFSET(16) NUMBITS(10) [],
        PRE_CKE_X1024 OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init1 [
        DRAM_RSTN_X1024 OFFSET(16) NUMBITS(9) [],
        FINAL_WAIT_X32 OFFSET(8) NUMBITS(7) [],
        PRE_OCD_X32 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init2 [
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        MIN_STABLE_CLOCK_X1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init3 [
        MR OFFSET(16) NUMBITS(16) [],
        EMR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init4 [
        EMR2 OFFSET(16) NUMBITS(16) [],
        EMR3 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init5 [
        DEV_ZQINIT_X32 OFFSET(16) NUMBITS(8) [],
        MAX_AUTO_INIT_X1024 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init6 [
        MR4 OFFSET(16) NUMBITS(16) [],
        MR5 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init7 [
        MR6 OFFSET(16) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dimmctl [
        DIMM_DIS_BG_MIRRORING OFFSET(5) NUMBITS(1) [],
        MRS_BG1_EN OFFSET(4) NUMBITS(1) [],
        MRS_A17_EN OFFSET(3) NUMBITS(1) [],
        DIMM_OUTPUT_INV_EN OFFSET(2) NUMBITS(1) [],
        DIMM_ADDR_MIRR_EN OFFSET(1) NUMBITS(1) [],
        DIMM_STAGGER_CS_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rankctl [
        DIFF_RANK_WR_GAP OFFSET(8) NUMBITS(4) [],
        DIFF_RANK_RD_GAP OFFSET(4) NUMBITS(4) [],
        MAX_RANK_RD OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg0 [
        WR2PRE OFFSET(24) NUMBITS(7) [],
        T_FAW OFFSET(16) NUMBITS(6) [],
        T_RAS_MAX OFFSET(8) NUMBITS(7) [],
        T_RAS_MIN OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg1 [
        T_XP OFFSET(16) NUMBITS(5) [],
        RD2PRE OFFSET(8) NUMBITS(5) [],
        T_RC OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg2 [
        WRITE_LATENCY OFFSET(24) NUMBITS(6) [],
        READ_LATENCY OFFSET(16) NUMBITS(6) [],
        RD2WR OFFSET(8) NUMBITS(6) [],
        WR2RD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg3 [
        T_MRW OFFSET(20) NUMBITS(10) [],
        T_MRD OFFSET(12) NUMBITS(6) [],
        T_MOD OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg4 [
        T_RCD OFFSET(24) NUMBITS(5) [],
        T_CCD OFFSET(16) NUMBITS(4) [],
        T_RRD OFFSET(8) NUMBITS(4) [],
        T_RP OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg5 [
        T_CKSRX OFFSET(24) NUMBITS(4) [],
        T_CKSRE OFFSET(16) NUMBITS(4) [],
        T_CKESR OFFSET(8) NUMBITS(6) [],
        T_CKE OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg6 [
        T_CKDPDE OFFSET(24) NUMBITS(4) [],
        T_CKDPDX OFFSET(16) NUMBITS(4) [],
        T_CKCSX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg7 [
        T_CKPDE OFFSET(8) NUMBITS(4) [],
        T_CKPDX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg8 [
        T_XS_FAST_X32 OFFSET(24) NUMBITS(7) [],
        T_XS_ABORT_X32 OFFSET(16) NUMBITS(7) [],
        T_XS_DLL_X32 OFFSET(8) NUMBITS(7) [],
        T_XS_X32 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg9 [
        DDR4_WR_PREAMBLE OFFSET(30) NUMBITS(1) [],
        T_CCD_S OFFSET(16) NUMBITS(3) [],
        T_RRD_S OFFSET(8) NUMBITS(4) [],
        WR2RD_S OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg10 [
        RESERVED0 OFFSET(16) NUMBITS(5) [],
        RESERVED1 OFFSET(8) NUMBITS(5) [],
        RESERVED2 OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg11 [
        POST_MPSM_GAP_X32 OFFSET(24) NUMBITS(7) [],
        T_MPX_LH OFFSET(16) NUMBITS(5) [],
        T_MPX_S OFFSET(8) NUMBITS(2) [],
        T_CKMPE OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg12 [
        T_CMDCKE OFFSET(16) NUMBITS(2) [],
        T_CKEHCMD OFFSET(8) NUMBITS(4) [],
        T_MRD_PDA OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg13 [
        ODTLOFF OFFSET(24) NUMBITS(7) [],
        T_CCD_MW OFFSET(16) NUMBITS(6) [],
        T_PPD OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg14 [
        T_XSR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zqctl0 [
        DIS_AUTO_ZQ OFFSET(31) NUMBITS(1) [],
        DIS_SRX_ZQCL OFFSET(30) NUMBITS(1) [],
        ZQ_RESISTOR_SHARED OFFSET(29) NUMBITS(1) [],
        DIS_MPSMX_ZQCL OFFSET(28) NUMBITS(1) [],
        T_ZQ_LONG_NOP OFFSET(16) NUMBITS(11) [],
        T_ZQ_SHORT_NOP OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zqctl1 [
        T_ZQ_RESET_NOP OFFSET(20) NUMBITS(10) [],
        T_ZQ_SHORT_INTERVAL_X1024 OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zqctl2 [
        ZQ_RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zqstat [
        ZQ_RESET_BUSY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfitmg0 [
        DFI_T_CTRL_DELAY OFFSET(24) NUMBITS(5) [],
        DFI_RDDATA_USE_SDR OFFSET(23) NUMBITS(1) [],
        DFI_T_RDDATA_EN OFFSET(16) NUMBITS(6) [],
        DFI_WRDATA_USE_SDR OFFSET(15) NUMBITS(1) [],
        DFI_TPHY_WRDATA OFFSET(8) NUMBITS(6) [],
        DFI_TPHY_WRLAT OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfitmg1 [
        DFI_T_CMD_LAT OFFSET(28) NUMBITS(4) [],
        DFI_T_PARIN_LAT OFFSET(24) NUMBITS(2) [],
        DFI_T_WRDATA_DELAY OFFSET(16) NUMBITS(5) [],
        DFI_T_DRAM_CLK_DISABLE OFFSET(8) NUMBITS(4) [],
        DFI_T_DRAM_CLK_ENABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfilpcfg0 [
        DFI_TLP_RESP OFFSET(24) NUMBITS(4) [],
        DFI_LP_WAKEUP_DPD OFFSET(20) NUMBITS(4) [],
        DFI_LP_EN_DPD OFFSET(16) NUMBITS(1) [],
        DFI_LP_WAKEUP_SR OFFSET(12) NUMBITS(4) [],
        DFI_LP_EN_SR OFFSET(8) NUMBITS(1) [],
        DFI_LP_WAKEUP_PD OFFSET(4) NUMBITS(4) [],
        DFI_LP_EN_PD OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfilpcfg1 [
        DFI_LP_WAKEUP_MPSM OFFSET(4) NUMBITS(4) [],
        DFI_LP_EN_MPSM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfiupd0 [
        DIS_AUTO_CTRLUPD OFFSET(31) NUMBITS(1) [],
        DIS_AUTO_CTRLUPD_SRX OFFSET(30) NUMBITS(1) [],
        DFI_T_CTRLUP_MAX OFFSET(16) NUMBITS(10) [],
        DFI_T_CTRLUP_MIN OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfiupd1 [
        DFI_T_CTRLUPD_INTERVAL_MIN_X1024 OFFSET(16) NUMBITS(8) [],
        DFI_T_CTRLUPD_INTERVAL_MAX_X1024 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfiupd2 [
        DFI_PHYUPD_EN OFFSET(31) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfimisc [
        DFI_DATA_CS_POLARITY OFFSET(2) NUMBITS(1) [],
        PHY_DBI_MODE OFFSET(1) NUMBITS(1) [],
        DFI_INIT_COMPLETE_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfitmg2 [
        DFI_TPHY_RDCSLAT OFFSET(8) NUMBITS(6) [],
        DFI_TPHY_WRCSLAT OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbictl [
        RD_DBI_EN OFFSET(2) NUMBITS(1) [],
        WR_DBI_EN OFFSET(1) NUMBITS(1) [],
        DM_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap0 [
        ADDRMAP_CS_BIT0 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap1 [
        ADDRMAP_BANK_B2 OFFSET(16) NUMBITS(5) [],
        ADDRMAP_BANK_B1 OFFSET(8) NUMBITS(5) [],
        ADDRMAP_BANK_B0 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap2 [
        ADDRMAP_COL_B5 OFFSET(24) NUMBITS(4) [],
        ADDRMAP_COL_B4 OFFSET(16) NUMBITS(4) [],
        ADDRMAP_COL_B3 OFFSET(8) NUMBITS(4) [],
        ADDRMAP_COL_B2 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap3 [
        ADDRMAP_COL_B9 OFFSET(24) NUMBITS(4) [],
        ADDRMAP_COL_B8 OFFSET(16) NUMBITS(4) [],
        ADDRMAP_COL_B7 OFFSET(8) NUMBITS(4) [],
        ADDRMAP_COL_B6 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap4 [
        ADDRMAP_COL_B11 OFFSET(8) NUMBITS(4) [],
        ADDRMAP_COL_B10 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap5 [
        ADDRMAP_ROW_B11 OFFSET(24) NUMBITS(4) [],
        ADDRMAP_ROW_B2_10 OFFSET(16) NUMBITS(4) [],
        ADDRMAP_ROW_B1 OFFSET(8) NUMBITS(4) [],
        ADDRMAP_ROW_B0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap6 [
        LPDDR3_6GB_12GB OFFSET(31) NUMBITS(1) [],
        ADDRMAP_ROW_B15 OFFSET(24) NUMBITS(4) [],
        ADDRMAP_ROW_B14 OFFSET(16) NUMBITS(4) [],
        ADDRMAP_ROW_B13 OFFSET(8) NUMBITS(4) [],
        ADDRMAP_ROW_B12 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap7 [
        ADDRMAP_ROW_B17 OFFSET(8) NUMBITS(4) [],
        ADDRMAP_ROW_B16 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap8 [
        ADDRMAP_BG_B1 OFFSET(8) NUMBITS(5) [],
        ADDRMAP_BG_B0 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap9 [
        ADDRMAP_ROW_B5 OFFSET(24) NUMBITS(4) [],
        ADDRMAP_ROW_B4 OFFSET(16) NUMBITS(4) [],
        ADDRMAP_ROW_B3 OFFSET(8) NUMBITS(4) [],
        ADDRMAP_ROW_B2 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap10 [
        ADDRMAP_ROW_B9 OFFSET(24) NUMBITS(4) [],
        ADDRMAP_ROW_B8 OFFSET(16) NUMBITS(4) [],
        ADDRMAP_ROW_B7 OFFSET(8) NUMBITS(4) [],
        ADDRMAP_ROW_B6 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Addrmap11 [
        ADDRMAP_ROW_B10 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Odtcfg [
        WR_ODT_HOLD OFFSET(24) NUMBITS(4) [],
        WR_ODT_DELAY OFFSET(16) NUMBITS(5) [],
        RD_ODT_HOLD OFFSET(8) NUMBITS(4) [],
        RD_ODT_DELAY OFFSET(2) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Odtmap [
        RANK1_RD_ODT OFFSET(12) NUMBITS(2) [],
        RANK1_WR_ODT OFFSET(8) NUMBITS(2) [],
        RANK0_RD_ODT OFFSET(4) NUMBITS(2) [],
        RANK0_WR_ODT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sched [
        RDWR_IDLE_GAP OFFSET(24) NUMBITS(7) [],
        GO2CRITICAL_HYSTERESIS OFFSET(16) NUMBITS(8) [],
        LPR_NUM_ENTRIES OFFSET(8) NUMBITS(6) [],
        PAGECLOSE OFFSET(2) NUMBITS(1) [],
        PREFER_WRITE OFFSET(1) NUMBITS(1) [],
        FORCE_LOW_PRI_N OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sched1 [
        PAGECLOSE_TIMER OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Perfhpr1 [
        HPR_XACT_RUN_LENGTH OFFSET(24) NUMBITS(8) [],
        HPR_MAX_STARVE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Perflpr1 [
        LPR_XACT_RUN_LENGTH OFFSET(24) NUMBITS(8) [],
        LPR_MAX_STARVE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Perfwr1 [
        W_XACT_RUN_LENGTH OFFSET(24) NUMBITS(8) [],
        W_MAX_STARVE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Perfvpr1 [
        VPR_TIMEOUT_RANGE OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Perfvpw1 [
        VPW_TIMEOUT_RANGE OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dqmap0 [
        DQ_NIBBLE_MAP_12_15 OFFSET(24) NUMBITS(8) [],
        DQ_NIBBLE_MAP_8_11 OFFSET(16) NUMBITS(8) [],
        DQ_NIBBLE_MAP_4_7 OFFSET(8) NUMBITS(8) [],
        DQ_NIBBLE_MAP_0_3 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dqmap1 [
        DQ_NIBBLE_MAP_28_31 OFFSET(24) NUMBITS(8) [],
        DQ_NIBBLE_MAP_24_27 OFFSET(16) NUMBITS(8) [],
        DQ_NIBBLE_MAP_20_23 OFFSET(8) NUMBITS(8) [],
        DQ_NIBBLE_MAP_16_19 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dqmap2 [
        DQ_NIBBLE_MAP_44_47 OFFSET(24) NUMBITS(8) [],
        DQ_NIBBLE_MAP_40_43 OFFSET(16) NUMBITS(8) [],
        DQ_NIBBLE_MAP_36_39 OFFSET(8) NUMBITS(8) [],
        DQ_NIBBLE_MAP_32_35 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dqmap3 [
        DQ_NIBBLE_MAP_60_63 OFFSET(24) NUMBITS(8) [],
        DQ_NIBBLE_MAP_56_59 OFFSET(16) NUMBITS(8) [],
        DQ_NIBBLE_MAP_52_55 OFFSET(8) NUMBITS(8) [],
        DQ_NIBBLE_MAP_48_51 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dqmap4 [
        DQ_NIBBLE_MAP_CB_4_7 OFFSET(8) NUMBITS(8) [],
        DQ_NIBBLE_MAP_CB_0_3 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dqmap5 [
        DIS_DQ_RANK_SWAP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbg1 [
        DIS_HIF OFFSET(1) NUMBITS(1) [],
        DIS_DQ OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgcam [
        DBG_STALL_RD OFFSET(31) NUMBITS(1) [],
        DBG_STALL_WR OFFSET(30) NUMBITS(1) [],
        WR_DATA_PIPELINE_EMPTY OFFSET(29) NUMBITS(1) [],
        RD_DATA_PIPELINE_EMPTY OFFSET(28) NUMBITS(1) [],
        DBG_WR_Q_EMPTY OFFSET(26) NUMBITS(1) [],
        DBG_RD_Q_EMPTY OFFSET(25) NUMBITS(1) [],
        DBG_STALL OFFSET(24) NUMBITS(1) [],
        DBG_W_Q_DEPTH OFFSET(16) NUMBITS(7) [],
        DBG_LPR_Q_DEPTH OFFSET(8) NUMBITS(7) [],
        DBG_HPR_Q_DEPTH OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgcmd [
        HW_REF_ZQ_EN OFFSET(31) NUMBITS(1) [],
        CTRLUPD OFFSET(5) NUMBITS(1) [],
        ZQ_CALIB_SHORT OFFSET(4) NUMBITS(1) [],
        RANK1_REFRESH OFFSET(1) NUMBITS(1) [],
        RANK0_REFRESH OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dbgstat [
        CTRLUPD_BUSY OFFSET(5) NUMBITS(1) [],
        ZQ_CALIB_SHORT_BUSY OFFSET(4) NUMBITS(1) [],
        RANK1_REFRESH_BUSY OFFSET(1) NUMBITS(1) [],
        RANK0_REFRESH_BUSY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Swctl [
        SW_DONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Swstat [
        SW_DONE_ACK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Poisoncfg [
        RD_POISON_INTR_CLR OFFSET(24) NUMBITS(1) [],
        RD_POISON_INTR_EN OFFSET(20) NUMBITS(1) [],
        RD_POISON_SLVERR_EN OFFSET(16) NUMBITS(1) [],
        WR_POISON_INTR_CLR OFFSET(8) NUMBITS(1) [],
        WR_POISON_INTR_EN OFFSET(4) NUMBITS(1) [],
        WR_POISON_SLVERR_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Poisonstat [
        RD_POISON_INTR_5 OFFSET(21) NUMBITS(1) [],
        RD_POISON_INTR_4 OFFSET(20) NUMBITS(1) [],
        RD_POISON_INTR_3 OFFSET(19) NUMBITS(1) [],
        RD_POISON_INTR_2 OFFSET(18) NUMBITS(1) [],
        RD_POISON_INTR_1 OFFSET(17) NUMBITS(1) [],
        RD_POISON_INTR_0 OFFSET(16) NUMBITS(1) [],
        WR_POISON_INTR_5 OFFSET(5) NUMBITS(1) [],
        WR_POISON_INTR_4 OFFSET(4) NUMBITS(1) [],
        WR_POISON_INTR_3 OFFSET(3) NUMBITS(1) [],
        WR_POISON_INTR_2 OFFSET(2) NUMBITS(1) [],
        WR_POISON_INTR_1 OFFSET(1) NUMBITS(1) [],
        WR_POISON_INTR_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pstat [
        WR_PORT_BUSY_5 OFFSET(21) NUMBITS(1) [],
        WR_PORT_BUSY_4 OFFSET(20) NUMBITS(1) [],
        WR_PORT_BUSY_3 OFFSET(19) NUMBITS(1) [],
        WR_PORT_BUSY_2 OFFSET(18) NUMBITS(1) [],
        WR_PORT_BUSY_1 OFFSET(17) NUMBITS(1) [],
        WR_PORT_BUSY_0 OFFSET(16) NUMBITS(1) [],
        RD_PORT_BUSY_5 OFFSET(5) NUMBITS(1) [],
        RD_PORT_BUSY_4 OFFSET(4) NUMBITS(1) [],
        RD_PORT_BUSY_3 OFFSET(3) NUMBITS(1) [],
        RD_PORT_BUSY_2 OFFSET(2) NUMBITS(1) [],
        RD_PORT_BUSY_1 OFFSET(1) NUMBITS(1) [],
        RD_PORT_BUSY_0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pccfg [
        BL_EXP_MODE OFFSET(8) NUMBITS(1) [],
        PAGEMATCH_LIMIT OFFSET(4) NUMBITS(1) [],
        GO2CRITICAL_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgr0 [
        RD_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        RD_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        RD_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        RD_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgw0 [
        WR_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        WR_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        WR_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        WR_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pctrl0 [
        PORT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos00 [
        RQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        RQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        RQOS_MAP_LEVEL1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos10 [
        RQOS_MAP_TIMEOUTR OFFSET(16) NUMBITS(11) [],
        RQOS_MAP_TIMEOUTB OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos00 [
        WQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        WQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        WQOS_MAP_LEVEL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos10 [
        WQOS_MAP_TIMEOUT OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgr1 [
        RD_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        RD_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        RD_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        RD_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgw1 [
        WR_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        WR_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        WR_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        WR_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pctrl1 [
        PORT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos01 [
        RQOS_MAP_REGION2 OFFSET(24) NUMBITS(2) [],
        RQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        RQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        RQOS_MAP_LEVEL2 OFFSET(8) NUMBITS(4) [],
        RQOS_MAP_LEVEL1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos11 [
        RQOS_MAP_TIMEOUTR OFFSET(16) NUMBITS(11) [],
        RQOS_MAP_TIMEOUTB OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos01 [
        WQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        WQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        WQOS_MAP_LEVEL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos11 [
        WQOS_MAP_TIMEOUT OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgr2 [
        RD_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        RD_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        RD_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        RD_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgw2 [
        WR_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        WR_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        WR_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        WR_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pctrl2 [
        PORT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos02 [
        RQOS_MAP_REGION2 OFFSET(24) NUMBITS(2) [],
        RQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        RQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        RQOS_MAP_LEVEL2 OFFSET(8) NUMBITS(4) [],
        RQOS_MAP_LEVEL1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos12 [
        RQOS_MAP_TIMEOUTR OFFSET(16) NUMBITS(11) [],
        RQOS_MAP_TIMEOUTB OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos02 [
        WQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        WQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        WQOS_MAP_LEVEL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos12 [
        WQOS_MAP_TIMEOUT OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgr3 [
        RD_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        RD_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        RD_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        RD_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgw3 [
        WR_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        WR_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        WR_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        WR_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pctrl3 [
        PORT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos03 [
        RQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        RQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        RQOS_MAP_LEVEL1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos13 [
        RQOS_MAP_TIMEOUTR OFFSET(16) NUMBITS(11) [],
        RQOS_MAP_TIMEOUTB OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos03 [
        WQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        WQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        WQOS_MAP_LEVEL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos13 [
        WQOS_MAP_TIMEOUT OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgr4 [
        RD_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        RD_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        RD_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        RD_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgw4 [
        WR_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        WR_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        WR_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        WR_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pctrl4 [
        PORT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos04 [
        RQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        RQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        RQOS_MAP_LEVEL1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos14 [
        RQOS_MAP_TIMEOUTR OFFSET(16) NUMBITS(11) [],
        RQOS_MAP_TIMEOUTB OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos04 [
        WQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        WQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        WQOS_MAP_LEVEL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos14 [
        WQOS_MAP_TIMEOUT OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgr5 [
        RD_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        RD_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        RD_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        RD_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgw5 [
        WR_PORT_PAGEMATCH_EN OFFSET(14) NUMBITS(1) [],
        WR_PORT_URGENT_EN OFFSET(13) NUMBITS(1) [],
        WR_PORT_AGING_EN OFFSET(12) NUMBITS(1) [],
        WR_PORT_PRIORITY OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pctrl5 [
        PORT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos05 [
        RQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        RQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        RQOS_MAP_LEVEL1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgqos15 [
        RQOS_MAP_TIMEOUTR OFFSET(16) NUMBITS(11) [],
        RQOS_MAP_TIMEOUTB OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos05 [
        WQOS_MAP_REGION1 OFFSET(20) NUMBITS(2) [],
        WQOS_MAP_REGION0 OFFSET(16) NUMBITS(2) [],
        WQOS_MAP_LEVEL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pcfgwqos15 [
        WQOS_MAP_TIMEOUT OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sarbase0 [
        BASE_ADDR OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sarsize0 [
        NBLOCKS OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sarbase1 [
        BASE_ADDR OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sarsize1 [
        NBLOCKS OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rfshctl0Shadow [
        REFRESH_MARGIN OFFSET(20) NUMBITS(4) [],
        REFRESH_TO_X32 OFFSET(12) NUMBITS(5) [],
        REFRESH_BURST OFFSET(4) NUMBITS(5) [],
        PER_BANK_REFRESH OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RfshtmgShadow [
        T_RFC_NOM_X32 OFFSET(16) NUMBITS(12) [],
        LPDDR3_TREFBW_EN OFFSET(15) NUMBITS(1) [],
        T_RFC_MIN OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init3Shadow [
        MR OFFSET(16) NUMBITS(16) [],
        EMR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init4Shadow [
        EMR2 OFFSET(16) NUMBITS(16) [],
        EMR3 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init6Shadow [
        MR4 OFFSET(16) NUMBITS(16) [],
        MR5 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Init7Shadow [
        MR6 OFFSET(16) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg0Shadow [
        WR2PRE OFFSET(24) NUMBITS(7) [],
        T_FAW OFFSET(16) NUMBITS(6) [],
        T_RAS_MAX OFFSET(8) NUMBITS(7) [],
        T_RAS_MIN OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg1Shadow [
        T_XP OFFSET(16) NUMBITS(5) [],
        RD2PRE OFFSET(8) NUMBITS(5) [],
        T_RC OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg2Shadow [
        WRITE_LATENCY OFFSET(24) NUMBITS(6) [],
        READ_LATENCY OFFSET(16) NUMBITS(6) [],
        RD2WR OFFSET(8) NUMBITS(6) [],
        WR2RD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg3Shadow [
        T_MRW OFFSET(20) NUMBITS(10) [],
        T_MRD OFFSET(12) NUMBITS(6) [],
        T_MOD OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg4Shadow [
        T_RCD OFFSET(24) NUMBITS(5) [],
        T_CCD OFFSET(16) NUMBITS(4) [],
        T_RRD OFFSET(8) NUMBITS(4) [],
        T_RP OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg5Shadow [
        T_CKSRX OFFSET(24) NUMBITS(4) [],
        T_CKSRE OFFSET(16) NUMBITS(4) [],
        T_CKESR OFFSET(8) NUMBITS(6) [],
        T_CKE OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg6Shadow [
        T_CKDPDE OFFSET(24) NUMBITS(4) [],
        T_CKDPDX OFFSET(16) NUMBITS(4) [],
        T_CKCSX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg7Shadow [
        T_CKPDE OFFSET(8) NUMBITS(4) [],
        T_CKPDX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg8Shadow [
        T_XS_FAST_X32 OFFSET(24) NUMBITS(7) [],
        T_XS_ABORT_X32 OFFSET(16) NUMBITS(7) [],
        T_XS_DLL_X32 OFFSET(8) NUMBITS(7) [],
        T_XS_X32 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg9Shadow [
        DDR4_WR_PREAMBLE OFFSET(30) NUMBITS(1) [],
        T_CCD_S OFFSET(16) NUMBITS(3) [],
        T_RRD_S OFFSET(8) NUMBITS(4) [],
        WR2RD_S OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg10Shadow [
        RESERVED0 OFFSET(16) NUMBITS(5) [],
        RESERVED1 OFFSET(8) NUMBITS(5) [],
        RESERVED2 OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg11Shadow [
        POST_MPSM_GAP_X32 OFFSET(24) NUMBITS(7) [],
        T_MPX_LH OFFSET(16) NUMBITS(5) [],
        T_MPX_S OFFSET(8) NUMBITS(2) [],
        T_CKMPE OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg12Shadow [
        T_CMDCKE OFFSET(16) NUMBITS(2) [],
        T_CKEHCMD OFFSET(8) NUMBITS(4) [],
        T_MRD_PDA OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg13Shadow [
        ODTLOFF OFFSET(24) NUMBITS(7) [],
        T_CCD_MW OFFSET(16) NUMBITS(6) [],
        T_PPD OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dramtmg14Shadow [
        T_XSR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zqctl0Shadow [
        DIS_AUTO_ZQ OFFSET(31) NUMBITS(1) [],
        DIS_SRX_ZQCL OFFSET(30) NUMBITS(1) [],
        ZQ_RESISTOR_SHARED OFFSET(29) NUMBITS(1) [],
        DIS_MPSMX_ZQCL OFFSET(28) NUMBITS(1) [],
        T_ZQ_LONG_NOP OFFSET(16) NUMBITS(11) [],
        T_ZQ_SHORT_NOP OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfitmg0Shadow [
        DFI_T_CTRL_DELAY OFFSET(24) NUMBITS(5) [],
        DFI_RDDATA_USE_SDR OFFSET(23) NUMBITS(1) [],
        DFI_T_RDDATA_EN OFFSET(16) NUMBITS(6) [],
        DFI_WRDATA_USE_SDR OFFSET(15) NUMBITS(1) [],
        DFI_TPHY_WRDATA OFFSET(8) NUMBITS(6) [],
        DFI_TPHY_WRLAT OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfitmg1Shadow [
        DFI_T_CMD_LAT OFFSET(28) NUMBITS(4) [],
        DFI_T_PARIN_LAT OFFSET(24) NUMBITS(2) [],
        DFI_T_WRDATA_DELAY OFFSET(16) NUMBITS(5) [],
        DFI_T_DRAM_CLK_DISABLE OFFSET(8) NUMBITS(4) [],
        DFI_T_DRAM_CLK_ENABLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dfitmg2Shadow [
        DFI_TPHY_RDCSLAT OFFSET(8) NUMBITS(6) [],
        DFI_TPHY_WRCSLAT OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OdtcfgShadow [
        WR_ODT_HOLD OFFSET(24) NUMBITS(4) [],
        WR_ODT_DELAY OFFSET(16) NUMBITS(5) [],
        RD_ODT_HOLD OFFSET(8) NUMBITS(4) [],
        RD_ODT_DELAY OFFSET(2) NUMBITS(5) [],
    ]
];

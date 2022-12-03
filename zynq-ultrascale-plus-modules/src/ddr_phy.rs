// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// DDR PHY Control, DDR PHY Control
pub static mut DDR_PHY: *mut Registers = 0xfd080000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 4],
    /// PHY Initialization Register
    pub pir: Aliased<u32, PirR::Register, PirW::Register>,
    _padding8: [u8; 8],
    /// PHY General Configuration Register 0
    pub pgcr0: Aliased<u32, Pgcr0R::Register, Pgcr0W::Register>,
    _padding20: [u8; 4],
    /// PHY General Configuration Register 2
    pub pgcr2: ReadWrite<u32, Pgcr2::Register>,
    /// PHY General Configuration Register 3
    pub pgcr3: Aliased<u32, Pgcr3R::Register, Pgcr3W::Register>,
    /// PHY General Configuration Register 4
    pub pgcr4: Aliased<u32, Pgcr4R::Register, Pgcr4W::Register>,
    /// PHY General Configuration Register 5
    pub pgcr5: Aliased<u32, Pgcr5R::Register, Pgcr5W::Register>,
    /// PHY General Configuration Register 6
    pub pgcr6: Aliased<u32, Pgcr6R::Register, Pgcr6W::Register>,
    /// PHY General Configuration Register 7
    pub pgcr7: Aliased<u32, Pgcr7R::Register, Pgcr7W::Register>,
    /// PHY General Status Register 0
    pub pgsr0: ReadOnly<u32, Pgsr0::Register>,
    /// PHY General Status Register 1
    pub pgsr1: ReadOnly<u32, Pgsr1::Register>,
    /// PHY General Status Register 2
    pub pgsr2: ReadOnly<u32, Pgsr2::Register>,
    _padding60: [u8; 4],
    /// PHY Timing Register 0
    pub ptr0: ReadWrite<u32, Ptr0::Register>,
    /// PHY Timing Register 1
    pub ptr1: Aliased<u32, Ptr1R::Register, Ptr1W::Register>,
    /// PHY Timing Register 2
    pub ptr2: Aliased<u32, Ptr2R::Register, Ptr2W::Register>,
    /// PHY Timing Register 3
    pub ptr3: Aliased<u32, Ptr3R::Register, Ptr3W::Register>,
    /// PHY Timing Register 4
    pub ptr4: Aliased<u32, Ptr4R::Register, Ptr4W::Register>,
    /// PHY Timing Register 5
    pub ptr5: Aliased<u32, Ptr5R::Register, Ptr5W::Register>,
    /// PHY Timing Register 6
    pub ptr6: Aliased<u32, Ptr6R::Register, Ptr6W::Register>,
    _padding92: [u8; 12],
    /// PLL Control Register 0 (Type B PLL Only)
    pub pllcr0: Aliased<u32, Pllcr0R::Register, Pllcr0W::Register>,
    /// PLL Control Register 1 (Type B PLL Only)
    pub pllcr1: Aliased<u32, Pllcr1R::Register, Pllcr1W::Register>,
    /// PLL Control Register 2 (Type B PLL Only)
    pub pllcr2: ReadWrite<u32>,
    /// PLL Control Register 3 (Type B PLL Only)
    pub pllcr3: ReadWrite<u32>,
    /// PLL Control Register 4 (Type B PLL Only)
    pub pllcr4: ReadWrite<u32>,
    /// PLL Control Register 5 (Type B PLL Only)
    pub pllcr5: Aliased<u32, Pllcr5R::Register, Pllcr5W::Register>,
    _padding128: [u8; 8],
    /// DATX8 Common Configuration Register
    pub dxccr: Aliased<u32, DxccrR::Register, DxccrW::Register>,
    _padding140: [u8; 4],
    /// DDR System General Configuration Register
    pub dsgcr: Aliased<u32, DsgcrR::Register, DsgcrW::Register>,
    _padding148: [u8; 4],
    /// ODT Configuration Register
    pub odtcr: Aliased<u32, OdtcrR::Register, OdtcrW::Register>,
    _padding156: [u8; 4],
    /// Anti-Aging Control Register
    pub aacr: ReadWrite<u32, Aacr::Register>,
    _padding164: [u8; 28],
    /// General Purpose Register 0
    pub gpr0: ReadWrite<u32, Gpr0::Register>,
    /// General Purpose Register 1
    pub gpr1: ReadWrite<u32, Gpr1::Register>,
    _padding200: [u8; 56],
    /// DRAM Configuration Register
    pub dcr: Aliased<u32, DcrR::Register, DcrW::Register>,
    _padding260: [u8; 12],
    /// DRAM Timing Parameters Register 0
    pub dtpr0: Aliased<u32, Dtpr0R::Register, Dtpr0W::Register>,
    /// DRAM Timing Parameters Register 1
    pub dtpr1: Aliased<u32, Dtpr1R::Register, Dtpr1W::Register>,
    /// DRAM Timing Parameters Register 2
    pub dtpr2: Aliased<u32, Dtpr2R::Register, Dtpr2W::Register>,
    /// DRAM Timing Parameters Register 3
    pub dtpr3: Aliased<u32, Dtpr3R::Register, Dtpr3W::Register>,
    /// DRAM Timing Parameters Register 4
    pub dtpr4: Aliased<u32, Dtpr4R::Register, Dtpr4W::Register>,
    /// DRAM Timing Parameters Register 5
    pub dtpr5: Aliased<u32, Dtpr5R::Register, Dtpr5W::Register>,
    /// DRAM Timing Parameters Register 6
    pub dtpr6: Aliased<u32, Dtpr6R::Register, Dtpr6W::Register>,
    _padding300: [u8; 20],
    /// RDIMM General Configuration Register 0
    pub rdimmgcr0: Aliased<u32, Rdimmgcr0R::Register, Rdimmgcr0W::Register>,
    /// RDIMM General Configuration Register 1
    pub rdimmgcr1: Aliased<u32, Rdimmgcr1R::Register, Rdimmgcr1W::Register>,
    /// RDIMM General Configuration Register 2
    pub rdimmgcr2: ReadWrite<u32>,
    _padding332: [u8; 4],
    /// RDIMM Control Register 0
    pub rdimmcr0: ReadWrite<u32, Rdimmcr0::Register>,
    /// RDIMM Control Register 1
    pub rdimmcr1: ReadWrite<u32, Rdimmcr1::Register>,
    /// RDIMM Control Register 2
    pub rdimmcr2: ReadWrite<u32, Rdimmcr2::Register>,
    /// RDIMM Control Register 3
    pub rdimmcr3: ReadWrite<u32, Rdimmcr3::Register>,
    /// RDIMM Control Register 4
    pub rdimmcr4: ReadWrite<u32, Rdimmcr4::Register>,
    _padding356: [u8; 4],
    /// Scheduler Command Register 0
    pub schcr0: Aliased<u32, Schcr0R::Register, Schcr0W::Register>,
    /// Scheduler Command Register 1
    pub schcr1: Aliased<u32, Schcr1R::Register, Schcr1W::Register>,
    _padding368: [u8; 16],
    /// LPDDR4 Mode Register 0
    pub mr0: Aliased<u32, Mr0R::Register, Mr0W::Register>,
    /// LPDDR4 Mode Register 1
    pub mr1: Aliased<u32, Mr1R::Register, Mr1W::Register>,
    /// LPDDR4 Mode Register 2
    pub mr2: Aliased<u32, Mr2R::Register, Mr2W::Register>,
    /// LPDDR4 Mode Register 3
    pub mr3: Aliased<u32, Mr3R::Register, Mr3W::Register>,
    /// LPDDR4 Mode Register 4
    pub mr4: Aliased<u32, Mr4R::Register, Mr4W::Register>,
    /// LPDDR4 Mode Register 5
    pub mr5: Aliased<u32, Mr5R::Register, Mr5W::Register>,
    /// LPDDR4 Mode Register 6
    pub mr6: Aliased<u32, Mr6R::Register, Mr6W::Register>,
    /// LPDDR4 Mode Register 7
    pub mr7: Aliased<u32, Mr7R::Register, Mr7W::Register>,
    _padding416: [u8; 12],
    /// LPDDR4 Mode Register 11
    pub mr11: Aliased<u32, Mr11R::Register, Mr11W::Register>,
    /// LPDDR4 Mode Register 12
    pub mr12: Aliased<u32, Mr12R::Register, Mr12W::Register>,
    /// LPDDR4 Mode Register 13
    pub mr13: Aliased<u32, Mr13R::Register, Mr13W::Register>,
    /// LPDDR4 Mode Register 14
    pub mr14: Aliased<u32, Mr14R::Register, Mr14W::Register>,
    _padding444: [u8; 28],
    /// LPDDR4 Mode Register 22
    pub mr22: Aliased<u32, Mr22R::Register, Mr22W::Register>,
    _padding476: [u8; 36],
    /// Data Training Configuration Register 0
    pub dtcr0: Aliased<u32, Dtcr0R::Register, Dtcr0W::Register>,
    /// Data Training Configuration Register 1
    pub dtcr1: Aliased<u32, Dtcr1R::Register, Dtcr1W::Register>,
    /// Data Training Address Register 0
    pub dtar0: Aliased<u32, Dtar0R::Register, Dtar0W::Register>,
    /// Data Training Address Register 1
    pub dtar1: Aliased<u32, Dtar1R::Register, Dtar1W::Register>,
    /// Data Training Address Register 2
    pub dtar2: Aliased<u32, Dtar2R::Register, Dtar2W::Register>,
    _padding532: [u8; 4],
    /// Data Training Data Register 0
    pub dtdr0: ReadWrite<u32, Dtdr0::Register>,
    /// Data Training Data Register 1
    pub dtdr1: ReadWrite<u32, Dtdr1::Register>,
    _padding544: [u8; 16],
    /// Data Training Eye Data Register 0
    pub dtedr0: ReadOnly<u32, Dtedr0::Register>,
    /// Data Training Eye Data Register 1
    pub dtedr1: ReadOnly<u32, Dtedr1::Register>,
    /// Data Training Eye Data Register 2
    pub dtedr2: ReadOnly<u32, Dtedr2::Register>,
    /// VREF Training Data Register
    pub vtdr: ReadOnly<u32, Vtdr::Register>,
    /// CA Training Register 0
    pub catr0: Aliased<u32, Catr0R::Register, Catr0W::Register>,
    /// CA Training Register 1
    pub catr1: Aliased<u32, Catr1R::Register, Catr1W::Register>,
    _padding584: [u8; 8],
    /// DQS Drift Register 0
    pub dqsdr0: Aliased<u32, Dqsdr0R::Register, Dqsdr0W::Register>,
    /// DQS Drift Register 1
    pub dqsdr1: ReadWrite<u32, Dqsdr1::Register>,
    /// DQS Drift Register 2
    pub dqsdr2: Aliased<u32, Dqsdr2R::Register, Dqsdr2W::Register>,
    /// Data Training Eye Data Register 3
    pub dtedr3: ReadOnly<u32, Dtedr3::Register>,
    _padding608: [u8; 160],
    /// DCU Address Register
    pub dcuar: Aliased<u32, DcuarR::Register, DcuarW::Register>,
    /// DCU Data Register
    pub dcudr: ReadWrite<u32>,
    /// DCU Run Register
    pub dcurr: Aliased<u32, DcurrR::Register, DcurrW::Register>,
    /// DCU Loop Register
    pub dculr: Aliased<u32, DculrR::Register, DculrW::Register>,
    /// DCU General Configuration Register
    pub dcugcr: Aliased<u32, DcugcrR::Register, DcugcrW::Register>,
    /// DCU Timing Parameters Register
    pub dcutpr: ReadWrite<u32, Dcutpr::Register>,
    /// DCU Status Register 0
    pub dcusr0: ReadOnly<u32, Dcusr0::Register>,
    /// DCU Status Register 1
    pub dcusr1: ReadOnly<u32, Dcusr1::Register>,
    _padding800: [u8; 444],
    /// Rank ID Register
    pub rankidr: Aliased<u32, RankidrR::Register, RankidrW::Register>,
    /// Rank I/O Configuration Register 0
    pub riocr0: ReadOnly<u32>,
    /// Rank I/O Configuration Register 1
    pub riocr1: ReadOnly<u32>,
    /// Rank I/O Configuration Register 2
    pub riocr2: Aliased<u32, Riocr2R::Register, Riocr2W::Register>,
    /// Rank I/O Configuration Register 3
    pub riocr3: ReadOnly<u32>,
    /// Rank I/O Configuration Register 4
    pub riocr4: Aliased<u32, Riocr4R::Register, Riocr4W::Register>,
    /// Rank I/O Configuration Register 5
    pub riocr5: Aliased<u32, Riocr5R::Register, Riocr5W::Register>,
    _padding1272: [u8; 8],
    /// AC I/O Configuration Register 0
    pub aciocr0: Aliased<u32, Aciocr0R::Register, Aciocr0W::Register>,
    /// AC I/O Configuration Register 1
    pub aciocr1: ReadWrite<u32>,
    /// AC I/O Configuration Register 2
    pub aciocr2: ReadWrite<u32, Aciocr2::Register>,
    /// AC I/O Configuration Register 3
    pub aciocr3: Aliased<u32, Aciocr3R::Register, Aciocr3W::Register>,
    /// AC I/O Configuration Register 4
    pub aciocr4: ReadWrite<u32, Aciocr4::Register>,
    /// AC I/O Configuration Register 5
    pub aciocr5: Aliased<u32, Aciocr5R::Register, Aciocr5W::Register>,
    _padding1304: [u8; 8],
    /// IO VREF Control Register 0
    pub iovcr0: Aliased<u32, Iovcr0R::Register, Iovcr0W::Register>,
    /// IO VREF Control Register 1
    pub iovcr1: ReadOnly<u32>,
    /// VREF Training Control Register 0
    pub vtcr0: Aliased<u32, Vtcr0R::Register, Vtcr0W::Register>,
    /// VREF Training Control Register 1
    pub vtcr1: Aliased<u32, Vtcr1R::Register, Vtcr1W::Register>,
    _padding1328: [u8; 16],
    /// AC Bit Delay Line Register 0
    pub acbdlr0: Aliased<u32, Acbdlr0R::Register, Acbdlr0W::Register>,
    /// AC Bit Delay Line Register 1
    pub acbdlr1: Aliased<u32, Acbdlr1R::Register, Acbdlr1W::Register>,
    /// AC Bit Delay Line Register 2
    pub acbdlr2: Aliased<u32, Acbdlr2R::Register, Acbdlr2W::Register>,
    /// AC Bit Delay Line Register 3
    pub acbdlr3: Aliased<u32, Acbdlr3R::Register, Acbdlr3W::Register>,
    /// AC Bit Delay Line Register 4
    pub acbdlr4: Aliased<u32, Acbdlr4R::Register, Acbdlr4W::Register>,
    /// AC Bit Delay Line Register 5
    pub acbdlr5: Aliased<u32, Acbdlr5R::Register, Acbdlr5W::Register>,
    /// AC Bit Delay Line Register 6
    pub acbdlr6: Aliased<u32, Acbdlr6R::Register, Acbdlr6W::Register>,
    /// AC Bit Delay Line Register 7
    pub acbdlr7: Aliased<u32, Acbdlr7R::Register, Acbdlr7W::Register>,
    /// AC Bit Delay Line Register 8
    pub acbdlr8: Aliased<u32, Acbdlr8R::Register, Acbdlr8W::Register>,
    /// AC Bit Delay Line Register 9
    pub acbdlr9: Aliased<u32, Acbdlr9R::Register, Acbdlr9W::Register>,
    _padding1384: [u8; 20],
    /// AC Bit Delay Line Register 15
    pub acbdlr15: Aliased<u32, Acbdlr15R::Register, Acbdlr15W::Register>,
    /// AC Bit Delay Line Register 16
    pub acbdlr16: Aliased<u32, Acbdlr16R::Register, Acbdlr16W::Register>,
    /// AC Local Calibrated Delay Line Register
    pub aclcdlr: Aliased<u32, AclcdlrR::Register, AclcdlrW::Register>,
    _padding1416: [u8; 24],
    /// AC Master Delay Line Register 0
    pub acmdlr0: Aliased<u32, Acmdlr0R::Register, Acmdlr0W::Register>,
    /// AC Master Delay Line Register 1
    pub acmdlr1: Aliased<u32, Acmdlr1R::Register, Acmdlr1W::Register>,
    _padding1448: [u8; 216],
    /// ZQ Impedance Control Register
    pub zqcr: Aliased<u32, ZqcrR::Register, ZqcrW::Register>,
    /// ZQ 0 Impedance Control Program Register 0
    pub zq0pr0: ReadWrite<u32, Zq0pr0::Register>,
    /// ZQ 0 Impedance Control Program Register 1
    pub zq0pr1: Aliased<u32, Zq0pr1R::Register, Zq0pr1W::Register>,
    /// ZQ n Impedance Control Data Register 0
    pub zq0dr0: ReadOnly<u32, Zq0dr0::Register>,
    /// ZQ n Impedance Control Data Register 1
    pub zq0dr1: ReadOnly<u32, Zq0dr1::Register>,
    /// ZQ 0 Impedance Control Override Data Register 0
    pub zq0or0: Aliased<u32, Zq0or0R::Register, Zq0or0W::Register>,
    /// ZQ 0 Impedance Control Override Data Register 1
    pub zq0or1: Aliased<u32, Zq0or1R::Register, Zq0or1W::Register>,
    /// ZQ 0 Impedance Control Status Register
    pub zq0sr: ReadOnly<u32, Zq0sr::Register>,
    _padding1696: [u8; 4],
    /// ZQ 1 Impedance Control Program Register 0
    pub zq1pr0: ReadWrite<u32, Zq1pr0::Register>,
    /// ZQ 1 Impedance Control Program Register 1
    pub zq1pr1: Aliased<u32, Zq1pr1R::Register, Zq1pr1W::Register>,
    /// ZQ 1 Impedance Control Data Register 0
    pub zq1dr0: ReadOnly<u32, Zq1dr0::Register>,
    /// ZQ 1 Impedance Control Data Register 1
    pub zq1dr1: ReadOnly<u32, Zq1dr1::Register>,
    /// ZQ 1 Impedance Control Override Data Register 0
    pub zq1or0: Aliased<u32, Zq1or0R::Register, Zq1or0W::Register>,
    /// ZQ 1 Impedance Control Override Data Register 1
    pub zq1or1: Aliased<u32, Zq1or1R::Register, Zq1or1W::Register>,
    /// ZQ 1 Impedance Control Status Register
    pub zq1sr: ReadOnly<u32, Zq1sr::Register>,
    _padding1728: [u8; 64],
    /// DATX8 n General Configuration Register 0
    pub dx0gcr0: Aliased<u32, Dx0gcr0R::Register, Dx0gcr0W::Register>,
    /// DATX8 n General Configuration Register 1
    pub dx0gcr1: Aliased<u32, Dx0gcr1R::Register, Dx0gcr1W::Register>,
    /// DATX8 n General Configuration Register 2
    pub dx0gcr2: ReadWrite<u32, Dx0gcr2::Register>,
    /// DATX8 n General Configuration Register 3
    pub dx0gcr3: Aliased<u32, Dx0gcr3R::Register, Dx0gcr3W::Register>,
    /// DATX8 n General Configuration Register 4
    pub dx0gcr4: Aliased<u32, Dx0gcr4R::Register, Dx0gcr4W::Register>,
    /// DATX8 n General Configuration Register 5
    pub dx0gcr5: Aliased<u32, Dx0gcr5R::Register, Dx0gcr5W::Register>,
    /// DATX8 n General Configuration Register 6
    pub dx0gcr6: Aliased<u32, Dx0gcr6R::Register, Dx0gcr6W::Register>,
    _padding1820: [u8; 36],
    /// DATX8 n Bit Delay Line Register 0
    pub dx0bdlr0: Aliased<u32, Dx0bdlr0R::Register, Dx0bdlr0W::Register>,
    /// DATX8 n Bit Delay Line Register 1
    pub dx0bdlr1: Aliased<u32, Dx0bdlr1R::Register, Dx0bdlr1W::Register>,
    /// DATX8 n Bit Delay Line Register 2
    pub dx0bdlr2: Aliased<u32, Dx0bdlr2R::Register, Dx0bdlr2W::Register>,
    _padding1868: [u8; 4],
    /// DATX8 n Bit Delay Line Register 3
    pub dx0bdlr3: Aliased<u32, Dx0bdlr3R::Register, Dx0bdlr3W::Register>,
    /// DATX8 n Bit Delay Line Register 4
    pub dx0bdlr4: Aliased<u32, Dx0bdlr4R::Register, Dx0bdlr4W::Register>,
    /// DATX8 n Bit Delay Line Register 5
    pub dx0bdlr5: Aliased<u32, Dx0bdlr5R::Register, Dx0bdlr5W::Register>,
    _padding1884: [u8; 4],
    /// DATX8 n Bit Delay Line Register 6
    pub dx0bdlr6: Aliased<u32, Dx0bdlr6R::Register, Dx0bdlr6W::Register>,
    _padding1892: [u8; 28],
    /// DATX8 n Local Calibrated Delay Line Register 0
    pub dx0lcdlr0: Aliased<u32, Dx0lcdlr0R::Register, Dx0lcdlr0W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 1
    pub dx0lcdlr1: Aliased<u32, Dx0lcdlr1R::Register, Dx0lcdlr1W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 2
    pub dx0lcdlr2: Aliased<u32, Dx0lcdlr2R::Register, Dx0lcdlr2W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 3
    pub dx0lcdlr3: Aliased<u32, Dx0lcdlr3R::Register, Dx0lcdlr3W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 4
    pub dx0lcdlr4: Aliased<u32, Dx0lcdlr4R::Register, Dx0lcdlr4W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 5
    pub dx0lcdlr5: Aliased<u32, Dx0lcdlr5R::Register, Dx0lcdlr5W::Register>,
    _padding1944: [u8; 8],
    /// DATX8 n Master Delay Line Register 0
    pub dx0mdlr0: Aliased<u32, Dx0mdlr0R::Register, Dx0mdlr0W::Register>,
    /// DATX8 n Master Delay Line Register 1
    pub dx0mdlr1: Aliased<u32, Dx0mdlr1R::Register, Dx0mdlr1W::Register>,
    _padding1960: [u8; 24],
    /// DATX8 n General Timing Register 0
    pub dx0gtr0: Aliased<u32, Dx0gtr0R::Register, Dx0gtr0W::Register>,
    _padding1988: [u8; 16],
    /// DATX8 n Rank Status Register 1
    pub dx0rsr1: ReadOnly<u32, Dx0rsr1::Register>,
    /// DATX8 n Rank Status Register 2
    pub dx0rsr2: ReadOnly<u32, Dx0rsr2::Register>,
    /// DATX8 n Rank Status Register 3
    pub dx0rsr3: ReadOnly<u32, Dx0rsr3::Register>,
    /// DATX8 n General Status Register 0
    pub dx0gsr0: ReadOnly<u32, Dx0gsr0::Register>,
    /// DATX8 n General Status Register 1
    pub dx0gsr1: ReadOnly<u32, Dx0gsr1::Register>,
    /// DATX8 n General Status Register 2
    pub dx0gsr2: ReadOnly<u32, Dx0gsr2::Register>,
    /// DATX8 n General Status Register 3
    pub dx0gsr3: ReadOnly<u32, Dx0gsr3::Register>,
    _padding2032: [u8; 16],
    /// DATX8 n General Configuration Register 0
    pub dx1gcr0: Aliased<u32, Dx1gcr0R::Register, Dx1gcr0W::Register>,
    /// DATX8 n General Configuration Register 1
    pub dx1gcr1: Aliased<u32, Dx1gcr1R::Register, Dx1gcr1W::Register>,
    /// DATX8 n General Configuration Register 2
    pub dx1gcr2: ReadWrite<u32, Dx1gcr2::Register>,
    /// DATX8 n General Configuration Register 3
    pub dx1gcr3: Aliased<u32, Dx1gcr3R::Register, Dx1gcr3W::Register>,
    /// DATX8 n General Configuration Register 4
    pub dx1gcr4: Aliased<u32, Dx1gcr4R::Register, Dx1gcr4W::Register>,
    /// DATX8 n General Configuration Register 5
    pub dx1gcr5: Aliased<u32, Dx1gcr5R::Register, Dx1gcr5W::Register>,
    /// DATX8 n General Configuration Register 6
    pub dx1gcr6: Aliased<u32, Dx1gcr6R::Register, Dx1gcr6W::Register>,
    _padding2076: [u8; 36],
    /// DATX8 n Bit Delay Line Register 0
    pub dx1bdlr0: Aliased<u32, Dx1bdlr0R::Register, Dx1bdlr0W::Register>,
    /// DATX8 n Bit Delay Line Register 1
    pub dx1bdlr1: Aliased<u32, Dx1bdlr1R::Register, Dx1bdlr1W::Register>,
    /// DATX8 n Bit Delay Line Register 2
    pub dx1bdlr2: Aliased<u32, Dx1bdlr2R::Register, Dx1bdlr2W::Register>,
    _padding2124: [u8; 4],
    /// DATX8 n Bit Delay Line Register 3
    pub dx1bdlr3: Aliased<u32, Dx1bdlr3R::Register, Dx1bdlr3W::Register>,
    /// DATX8 n Bit Delay Line Register 4
    pub dx1bdlr4: Aliased<u32, Dx1bdlr4R::Register, Dx1bdlr4W::Register>,
    /// DATX8 n Bit Delay Line Register 5
    pub dx1bdlr5: Aliased<u32, Dx1bdlr5R::Register, Dx1bdlr5W::Register>,
    _padding2140: [u8; 4],
    /// DATX8 n Bit Delay Line Register 6
    pub dx1bdlr6: Aliased<u32, Dx1bdlr6R::Register, Dx1bdlr6W::Register>,
    _padding2148: [u8; 28],
    /// DATX8 n Local Calibrated Delay Line Register 0
    pub dx1lcdlr0: Aliased<u32, Dx1lcdlr0R::Register, Dx1lcdlr0W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 1
    pub dx1lcdlr1: Aliased<u32, Dx1lcdlr1R::Register, Dx1lcdlr1W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 2
    pub dx1lcdlr2: Aliased<u32, Dx1lcdlr2R::Register, Dx1lcdlr2W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 3
    pub dx1lcdlr3: Aliased<u32, Dx1lcdlr3R::Register, Dx1lcdlr3W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 4
    pub dx1lcdlr4: Aliased<u32, Dx1lcdlr4R::Register, Dx1lcdlr4W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 5
    pub dx1lcdlr5: Aliased<u32, Dx1lcdlr5R::Register, Dx1lcdlr5W::Register>,
    _padding2200: [u8; 8],
    /// DATX8 n Master Delay Line Register 0
    pub dx1mdlr0: Aliased<u32, Dx1mdlr0R::Register, Dx1mdlr0W::Register>,
    /// DATX8 n Master Delay Line Register 1
    pub dx1mdlr1: Aliased<u32, Dx1mdlr1R::Register, Dx1mdlr1W::Register>,
    _padding2216: [u8; 24],
    /// DATX8 n General Timing Register 0
    pub dx1gtr0: Aliased<u32, Dx1gtr0R::Register, Dx1gtr0W::Register>,
    _padding2244: [u8; 16],
    /// DATX8 n Rank Status Register 1
    pub dx1rsr1: ReadOnly<u32, Dx1rsr1::Register>,
    /// DATX8 n Rank Status Register 2
    pub dx1rsr2: ReadOnly<u32, Dx1rsr2::Register>,
    /// DATX8 n Rank Status Register 3
    pub dx1rsr3: ReadOnly<u32, Dx1rsr3::Register>,
    /// DATX8 n General Status Register 0
    pub dx1gsr0: ReadOnly<u32, Dx1gsr0::Register>,
    /// DATX8 n General Status Register 1
    pub dx1gsr1: ReadOnly<u32, Dx1gsr1::Register>,
    /// DATX8 n General Status Register 2
    pub dx1gsr2: ReadOnly<u32, Dx1gsr2::Register>,
    /// DATX8 n General Status Register 3
    pub dx1gsr3: ReadOnly<u32, Dx1gsr3::Register>,
    _padding2288: [u8; 16],
    /// DATX8 n General Configuration Register 0
    pub dx2gcr0: Aliased<u32, Dx2gcr0R::Register, Dx2gcr0W::Register>,
    /// DATX8 n General Configuration Register 1
    pub dx2gcr1: Aliased<u32, Dx2gcr1R::Register, Dx2gcr1W::Register>,
    /// DATX8 n General Configuration Register 2
    pub dx2gcr2: ReadWrite<u32, Dx2gcr2::Register>,
    /// DATX8 n General Configuration Register 3
    pub dx2gcr3: Aliased<u32, Dx2gcr3R::Register, Dx2gcr3W::Register>,
    /// DATX8 n General Configuration Register 4
    pub dx2gcr4: Aliased<u32, Dx2gcr4R::Register, Dx2gcr4W::Register>,
    /// DATX8 n General Configuration Register 5
    pub dx2gcr5: Aliased<u32, Dx2gcr5R::Register, Dx2gcr5W::Register>,
    /// DATX8 n General Configuration Register 6
    pub dx2gcr6: Aliased<u32, Dx2gcr6R::Register, Dx2gcr6W::Register>,
    _padding2332: [u8; 36],
    /// DATX8 n Bit Delay Line Register 0
    pub dx2bdlr0: Aliased<u32, Dx2bdlr0R::Register, Dx2bdlr0W::Register>,
    /// DATX8 n Bit Delay Line Register 1
    pub dx2bdlr1: Aliased<u32, Dx2bdlr1R::Register, Dx2bdlr1W::Register>,
    /// DATX8 n Bit Delay Line Register 2
    pub dx2bdlr2: Aliased<u32, Dx2bdlr2R::Register, Dx2bdlr2W::Register>,
    _padding2380: [u8; 4],
    /// DATX8 n Bit Delay Line Register 3
    pub dx2bdlr3: Aliased<u32, Dx2bdlr3R::Register, Dx2bdlr3W::Register>,
    /// DATX8 n Bit Delay Line Register 4
    pub dx2bdlr4: Aliased<u32, Dx2bdlr4R::Register, Dx2bdlr4W::Register>,
    /// DATX8 n Bit Delay Line Register 5
    pub dx2bdlr5: Aliased<u32, Dx2bdlr5R::Register, Dx2bdlr5W::Register>,
    _padding2396: [u8; 4],
    /// DATX8 n Bit Delay Line Register 6
    pub dx2bdlr6: Aliased<u32, Dx2bdlr6R::Register, Dx2bdlr6W::Register>,
    _padding2404: [u8; 28],
    /// DATX8 n Local Calibrated Delay Line Register 0
    pub dx2lcdlr0: Aliased<u32, Dx2lcdlr0R::Register, Dx2lcdlr0W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 1
    pub dx2lcdlr1: Aliased<u32, Dx2lcdlr1R::Register, Dx2lcdlr1W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 2
    pub dx2lcdlr2: Aliased<u32, Dx2lcdlr2R::Register, Dx2lcdlr2W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 3
    pub dx2lcdlr3: Aliased<u32, Dx2lcdlr3R::Register, Dx2lcdlr3W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 4
    pub dx2lcdlr4: Aliased<u32, Dx2lcdlr4R::Register, Dx2lcdlr4W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 5
    pub dx2lcdlr5: Aliased<u32, Dx2lcdlr5R::Register, Dx2lcdlr5W::Register>,
    _padding2456: [u8; 8],
    /// DATX8 n Master Delay Line Register 0
    pub dx2mdlr0: Aliased<u32, Dx2mdlr0R::Register, Dx2mdlr0W::Register>,
    /// DATX8 n Master Delay Line Register 1
    pub dx2mdlr1: Aliased<u32, Dx2mdlr1R::Register, Dx2mdlr1W::Register>,
    _padding2472: [u8; 24],
    /// DATX8 n General Timing Register 0
    pub dx2gtr0: Aliased<u32, Dx2gtr0R::Register, Dx2gtr0W::Register>,
    _padding2500: [u8; 16],
    /// DATX8 n Rank Status Register 1
    pub dx2rsr1: ReadOnly<u32, Dx2rsr1::Register>,
    /// DATX8 n Rank Status Register 2
    pub dx2rsr2: ReadOnly<u32, Dx2rsr2::Register>,
    /// DATX8 n Rank Status Register 3
    pub dx2rsr3: ReadOnly<u32, Dx2rsr3::Register>,
    /// DATX8 n General Status Register 0
    pub dx2gsr0: ReadOnly<u32, Dx2gsr0::Register>,
    /// DATX8 n General Status Register 1
    pub dx2gsr1: ReadOnly<u32, Dx2gsr1::Register>,
    /// DATX8 n General Status Register 2
    pub dx2gsr2: ReadOnly<u32, Dx2gsr2::Register>,
    /// DATX8 n General Status Register 3
    pub dx2gsr3: ReadOnly<u32, Dx2gsr3::Register>,
    _padding2544: [u8; 16],
    /// DATX8 n General Configuration Register 0
    pub dx3gcr0: Aliased<u32, Dx3gcr0R::Register, Dx3gcr0W::Register>,
    /// DATX8 n General Configuration Register 1
    pub dx3gcr1: Aliased<u32, Dx3gcr1R::Register, Dx3gcr1W::Register>,
    /// DATX8 n General Configuration Register 2
    pub dx3gcr2: ReadWrite<u32, Dx3gcr2::Register>,
    /// DATX8 n General Configuration Register 3
    pub dx3gcr3: Aliased<u32, Dx3gcr3R::Register, Dx3gcr3W::Register>,
    /// DATX8 n General Configuration Register 4
    pub dx3gcr4: Aliased<u32, Dx3gcr4R::Register, Dx3gcr4W::Register>,
    /// DATX8 n General Configuration Register 5
    pub dx3gcr5: Aliased<u32, Dx3gcr5R::Register, Dx3gcr5W::Register>,
    /// DATX8 n General Configuration Register 6
    pub dx3gcr6: Aliased<u32, Dx3gcr6R::Register, Dx3gcr6W::Register>,
    _padding2588: [u8; 36],
    /// DATX8 n Bit Delay Line Register 0
    pub dx3bdlr0: Aliased<u32, Dx3bdlr0R::Register, Dx3bdlr0W::Register>,
    /// DATX8 n Bit Delay Line Register 1
    pub dx3bdlr1: Aliased<u32, Dx3bdlr1R::Register, Dx3bdlr1W::Register>,
    /// DATX8 n Bit Delay Line Register 2
    pub dx3bdlr2: Aliased<u32, Dx3bdlr2R::Register, Dx3bdlr2W::Register>,
    _padding2636: [u8; 4],
    /// DATX8 n Bit Delay Line Register 3
    pub dx3bdlr3: Aliased<u32, Dx3bdlr3R::Register, Dx3bdlr3W::Register>,
    /// DATX8 n Bit Delay Line Register 4
    pub dx3bdlr4: Aliased<u32, Dx3bdlr4R::Register, Dx3bdlr4W::Register>,
    /// DATX8 n Bit Delay Line Register 5
    pub dx3bdlr5: Aliased<u32, Dx3bdlr5R::Register, Dx3bdlr5W::Register>,
    _padding2652: [u8; 4],
    /// DATX8 n Bit Delay Line Register 6
    pub dx3bdlr6: Aliased<u32, Dx3bdlr6R::Register, Dx3bdlr6W::Register>,
    _padding2660: [u8; 28],
    /// DATX8 n Local Calibrated Delay Line Register 0
    pub dx3lcdlr0: Aliased<u32, Dx3lcdlr0R::Register, Dx3lcdlr0W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 1
    pub dx3lcdlr1: Aliased<u32, Dx3lcdlr1R::Register, Dx3lcdlr1W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 2
    pub dx3lcdlr2: Aliased<u32, Dx3lcdlr2R::Register, Dx3lcdlr2W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 3
    pub dx3lcdlr3: Aliased<u32, Dx3lcdlr3R::Register, Dx3lcdlr3W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 4
    pub dx3lcdlr4: Aliased<u32, Dx3lcdlr4R::Register, Dx3lcdlr4W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 5
    pub dx3lcdlr5: Aliased<u32, Dx3lcdlr5R::Register, Dx3lcdlr5W::Register>,
    _padding2712: [u8; 8],
    /// DATX8 n Master Delay Line Register 0
    pub dx3mdlr0: Aliased<u32, Dx3mdlr0R::Register, Dx3mdlr0W::Register>,
    /// DATX8 n Master Delay Line Register 1
    pub dx3mdlr1: Aliased<u32, Dx3mdlr1R::Register, Dx3mdlr1W::Register>,
    _padding2728: [u8; 24],
    /// DATX8 n General Timing Register 0
    pub dx3gtr0: Aliased<u32, Dx3gtr0R::Register, Dx3gtr0W::Register>,
    _padding2756: [u8; 16],
    /// DATX8 n Rank Status Register 1
    pub dx3rsr1: ReadOnly<u32, Dx3rsr1::Register>,
    /// DATX8 n Rank Status Register 2
    pub dx3rsr2: ReadOnly<u32, Dx3rsr2::Register>,
    /// DATX8 n Rank Status Register 3
    pub dx3rsr3: ReadOnly<u32, Dx3rsr3::Register>,
    /// DATX8 n General Status Register 0
    pub dx3gsr0: ReadOnly<u32, Dx3gsr0::Register>,
    /// DATX8 n General Status Register 1
    pub dx3gsr1: ReadOnly<u32, Dx3gsr1::Register>,
    /// DATX8 n General Status Register 2
    pub dx3gsr2: ReadOnly<u32, Dx3gsr2::Register>,
    /// DATX8 n General Status Register 3
    pub dx3gsr3: ReadOnly<u32, Dx3gsr3::Register>,
    _padding2800: [u8; 16],
    /// DATX8 n General Configuration Register 0
    pub dx4gcr0: Aliased<u32, Dx4gcr0R::Register, Dx4gcr0W::Register>,
    /// DATX8 n General Configuration Register 1
    pub dx4gcr1: Aliased<u32, Dx4gcr1R::Register, Dx4gcr1W::Register>,
    /// DATX8 n General Configuration Register 2
    pub dx4gcr2: ReadWrite<u32, Dx4gcr2::Register>,
    /// DATX8 n General Configuration Register 3
    pub dx4gcr3: Aliased<u32, Dx4gcr3R::Register, Dx4gcr3W::Register>,
    /// DATX8 n General Configuration Register 4
    pub dx4gcr4: Aliased<u32, Dx4gcr4R::Register, Dx4gcr4W::Register>,
    /// DATX8 n General Configuration Register 5
    pub dx4gcr5: Aliased<u32, Dx4gcr5R::Register, Dx4gcr5W::Register>,
    /// DATX8 n General Configuration Register 6
    pub dx4gcr6: Aliased<u32, Dx4gcr6R::Register, Dx4gcr6W::Register>,
    _padding2844: [u8; 36],
    /// DATX8 n Bit Delay Line Register 0
    pub dx4bdlr0: Aliased<u32, Dx4bdlr0R::Register, Dx4bdlr0W::Register>,
    /// DATX8 n Bit Delay Line Register 1
    pub dx4bdlr1: Aliased<u32, Dx4bdlr1R::Register, Dx4bdlr1W::Register>,
    /// DATX8 n Bit Delay Line Register 2
    pub dx4bdlr2: Aliased<u32, Dx4bdlr2R::Register, Dx4bdlr2W::Register>,
    _padding2892: [u8; 4],
    /// DATX8 n Bit Delay Line Register 3
    pub dx4bdlr3: Aliased<u32, Dx4bdlr3R::Register, Dx4bdlr3W::Register>,
    /// DATX8 n Bit Delay Line Register 4
    pub dx4bdlr4: Aliased<u32, Dx4bdlr4R::Register, Dx4bdlr4W::Register>,
    /// DATX8 n Bit Delay Line Register 5
    pub dx4bdlr5: Aliased<u32, Dx4bdlr5R::Register, Dx4bdlr5W::Register>,
    _padding2908: [u8; 4],
    /// DATX8 n Bit Delay Line Register 6
    pub dx4bdlr6: Aliased<u32, Dx4bdlr6R::Register, Dx4bdlr6W::Register>,
    _padding2916: [u8; 28],
    /// DATX8 n Local Calibrated Delay Line Register 0
    pub dx4lcdlr0: Aliased<u32, Dx4lcdlr0R::Register, Dx4lcdlr0W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 1
    pub dx4lcdlr1: Aliased<u32, Dx4lcdlr1R::Register, Dx4lcdlr1W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 2
    pub dx4lcdlr2: Aliased<u32, Dx4lcdlr2R::Register, Dx4lcdlr2W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 3
    pub dx4lcdlr3: Aliased<u32, Dx4lcdlr3R::Register, Dx4lcdlr3W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 4
    pub dx4lcdlr4: Aliased<u32, Dx4lcdlr4R::Register, Dx4lcdlr4W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 5
    pub dx4lcdlr5: Aliased<u32, Dx4lcdlr5R::Register, Dx4lcdlr5W::Register>,
    _padding2968: [u8; 8],
    /// DATX8 n Master Delay Line Register 0
    pub dx4mdlr0: Aliased<u32, Dx4mdlr0R::Register, Dx4mdlr0W::Register>,
    /// DATX8 n Master Delay Line Register 1
    pub dx4mdlr1: Aliased<u32, Dx4mdlr1R::Register, Dx4mdlr1W::Register>,
    _padding2984: [u8; 24],
    /// DATX8 n General Timing Register 0
    pub dx4gtr0: Aliased<u32, Dx4gtr0R::Register, Dx4gtr0W::Register>,
    _padding3012: [u8; 16],
    /// DATX8 n Rank Status Register 1
    pub dx4rsr1: ReadOnly<u32, Dx4rsr1::Register>,
    /// DATX8 n Rank Status Register 2
    pub dx4rsr2: ReadOnly<u32, Dx4rsr2::Register>,
    /// DATX8 n Rank Status Register 3
    pub dx4rsr3: ReadOnly<u32, Dx4rsr3::Register>,
    /// DATX8 n General Status Register 0
    pub dx4gsr0: ReadOnly<u32, Dx4gsr0::Register>,
    /// DATX8 n General Status Register 1
    pub dx4gsr1: ReadOnly<u32, Dx4gsr1::Register>,
    /// DATX8 n General Status Register 2
    pub dx4gsr2: ReadOnly<u32, Dx4gsr2::Register>,
    /// DATX8 n General Status Register 3
    pub dx4gsr3: ReadOnly<u32, Dx4gsr3::Register>,
    _padding3056: [u8; 16],
    /// DATX8 n General Configuration Register 0
    pub dx5gcr0: Aliased<u32, Dx5gcr0R::Register, Dx5gcr0W::Register>,
    /// DATX8 n General Configuration Register 1
    pub dx5gcr1: Aliased<u32, Dx5gcr1R::Register, Dx5gcr1W::Register>,
    /// DATX8 n General Configuration Register 2
    pub dx5gcr2: ReadWrite<u32, Dx5gcr2::Register>,
    /// DATX8 n General Configuration Register 3
    pub dx5gcr3: Aliased<u32, Dx5gcr3R::Register, Dx5gcr3W::Register>,
    /// DATX8 n General Configuration Register 4
    pub dx5gcr4: Aliased<u32, Dx5gcr4R::Register, Dx5gcr4W::Register>,
    /// DATX8 n General Configuration Register 5
    pub dx5gcr5: Aliased<u32, Dx5gcr5R::Register, Dx5gcr5W::Register>,
    /// DATX8 n General Configuration Register 6
    pub dx5gcr6: Aliased<u32, Dx5gcr6R::Register, Dx5gcr6W::Register>,
    _padding3100: [u8; 36],
    /// DATX8 n Bit Delay Line Register 0
    pub dx5bdlr0: Aliased<u32, Dx5bdlr0R::Register, Dx5bdlr0W::Register>,
    /// DATX8 n Bit Delay Line Register 1
    pub dx5bdlr1: Aliased<u32, Dx5bdlr1R::Register, Dx5bdlr1W::Register>,
    /// DATX8 n Bit Delay Line Register 2
    pub dx5bdlr2: Aliased<u32, Dx5bdlr2R::Register, Dx5bdlr2W::Register>,
    _padding3148: [u8; 4],
    /// DATX8 n Bit Delay Line Register 3
    pub dx5bdlr3: Aliased<u32, Dx5bdlr3R::Register, Dx5bdlr3W::Register>,
    /// DATX8 n Bit Delay Line Register 4
    pub dx5bdlr4: Aliased<u32, Dx5bdlr4R::Register, Dx5bdlr4W::Register>,
    /// DATX8 n Bit Delay Line Register 5
    pub dx5bdlr5: Aliased<u32, Dx5bdlr5R::Register, Dx5bdlr5W::Register>,
    _padding3164: [u8; 4],
    /// DATX8 n Bit Delay Line Register 6
    pub dx5bdlr6: Aliased<u32, Dx5bdlr6R::Register, Dx5bdlr6W::Register>,
    _padding3172: [u8; 28],
    /// DATX8 n Local Calibrated Delay Line Register 0
    pub dx5lcdlr0: Aliased<u32, Dx5lcdlr0R::Register, Dx5lcdlr0W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 1
    pub dx5lcdlr1: Aliased<u32, Dx5lcdlr1R::Register, Dx5lcdlr1W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 2
    pub dx5lcdlr2: Aliased<u32, Dx5lcdlr2R::Register, Dx5lcdlr2W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 3
    pub dx5lcdlr3: Aliased<u32, Dx5lcdlr3R::Register, Dx5lcdlr3W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 4
    pub dx5lcdlr4: Aliased<u32, Dx5lcdlr4R::Register, Dx5lcdlr4W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 5
    pub dx5lcdlr5: Aliased<u32, Dx5lcdlr5R::Register, Dx5lcdlr5W::Register>,
    _padding3224: [u8; 8],
    /// DATX8 n Master Delay Line Register 0
    pub dx5mdlr0: Aliased<u32, Dx5mdlr0R::Register, Dx5mdlr0W::Register>,
    /// DATX8 n Master Delay Line Register 1
    pub dx5mdlr1: Aliased<u32, Dx5mdlr1R::Register, Dx5mdlr1W::Register>,
    _padding3240: [u8; 24],
    /// DATX8 n General Timing Register 0
    pub dx5gtr0: Aliased<u32, Dx5gtr0R::Register, Dx5gtr0W::Register>,
    _padding3268: [u8; 16],
    /// DATX8 n Rank Status Register 1
    pub dx5rsr1: ReadOnly<u32, Dx5rsr1::Register>,
    /// DATX8 n Rank Status Register 2
    pub dx5rsr2: ReadOnly<u32, Dx5rsr2::Register>,
    /// DATX8 n Rank Status Register 3
    pub dx5rsr3: ReadOnly<u32, Dx5rsr3::Register>,
    /// DATX8 n General Status Register 0
    pub dx5gsr0: ReadOnly<u32, Dx5gsr0::Register>,
    /// DATX8 n General Status Register 1
    pub dx5gsr1: ReadOnly<u32, Dx5gsr1::Register>,
    /// DATX8 n General Status Register 2
    pub dx5gsr2: ReadOnly<u32, Dx5gsr2::Register>,
    /// DATX8 n General Status Register 3
    pub dx5gsr3: ReadOnly<u32, Dx5gsr3::Register>,
    _padding3312: [u8; 16],
    /// DATX8 n General Configuration Register 0
    pub dx6gcr0: Aliased<u32, Dx6gcr0R::Register, Dx6gcr0W::Register>,
    /// DATX8 n General Configuration Register 1
    pub dx6gcr1: Aliased<u32, Dx6gcr1R::Register, Dx6gcr1W::Register>,
    /// DATX8 n General Configuration Register 2
    pub dx6gcr2: ReadWrite<u32, Dx6gcr2::Register>,
    /// DATX8 n General Configuration Register 3
    pub dx6gcr3: Aliased<u32, Dx6gcr3R::Register, Dx6gcr3W::Register>,
    /// DATX8 n General Configuration Register 4
    pub dx6gcr4: Aliased<u32, Dx6gcr4R::Register, Dx6gcr4W::Register>,
    /// DATX8 n General Configuration Register 5
    pub dx6gcr5: Aliased<u32, Dx6gcr5R::Register, Dx6gcr5W::Register>,
    /// DATX8 n General Configuration Register 6
    pub dx6gcr6: Aliased<u32, Dx6gcr6R::Register, Dx6gcr6W::Register>,
    _padding3356: [u8; 36],
    /// DATX8 n Bit Delay Line Register 0
    pub dx6bdlr0: Aliased<u32, Dx6bdlr0R::Register, Dx6bdlr0W::Register>,
    /// DATX8 n Bit Delay Line Register 1
    pub dx6bdlr1: Aliased<u32, Dx6bdlr1R::Register, Dx6bdlr1W::Register>,
    /// DATX8 n Bit Delay Line Register 2
    pub dx6bdlr2: Aliased<u32, Dx6bdlr2R::Register, Dx6bdlr2W::Register>,
    _padding3404: [u8; 4],
    /// DATX8 n Bit Delay Line Register 3
    pub dx6bdlr3: Aliased<u32, Dx6bdlr3R::Register, Dx6bdlr3W::Register>,
    /// DATX8 n Bit Delay Line Register 4
    pub dx6bdlr4: Aliased<u32, Dx6bdlr4R::Register, Dx6bdlr4W::Register>,
    /// DATX8 n Bit Delay Line Register 5
    pub dx6bdlr5: Aliased<u32, Dx6bdlr5R::Register, Dx6bdlr5W::Register>,
    _padding3420: [u8; 4],
    /// DATX8 n Bit Delay Line Register 6
    pub dx6bdlr6: Aliased<u32, Dx6bdlr6R::Register, Dx6bdlr6W::Register>,
    _padding3428: [u8; 28],
    /// DATX8 n Local Calibrated Delay Line Register 0
    pub dx6lcdlr0: Aliased<u32, Dx6lcdlr0R::Register, Dx6lcdlr0W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 1
    pub dx6lcdlr1: Aliased<u32, Dx6lcdlr1R::Register, Dx6lcdlr1W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 2
    pub dx6lcdlr2: Aliased<u32, Dx6lcdlr2R::Register, Dx6lcdlr2W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 3
    pub dx6lcdlr3: Aliased<u32, Dx6lcdlr3R::Register, Dx6lcdlr3W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 4
    pub dx6lcdlr4: Aliased<u32, Dx6lcdlr4R::Register, Dx6lcdlr4W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 5
    pub dx6lcdlr5: Aliased<u32, Dx6lcdlr5R::Register, Dx6lcdlr5W::Register>,
    _padding3480: [u8; 8],
    /// DATX8 n Master Delay Line Register 0
    pub dx6mdlr0: Aliased<u32, Dx6mdlr0R::Register, Dx6mdlr0W::Register>,
    /// DATX8 n Master Delay Line Register 1
    pub dx6mdlr1: Aliased<u32, Dx6mdlr1R::Register, Dx6mdlr1W::Register>,
    _padding3496: [u8; 24],
    /// DATX8 n General Timing Register 0
    pub dx6gtr0: Aliased<u32, Dx6gtr0R::Register, Dx6gtr0W::Register>,
    _padding3524: [u8; 16],
    /// DATX8 n Rank Status Register 1
    pub dx6rsr1: ReadOnly<u32, Dx6rsr1::Register>,
    /// DATX8 n Rank Status Register 2
    pub dx6rsr2: ReadOnly<u32, Dx6rsr2::Register>,
    /// DATX8 n Rank Status Register 3
    pub dx6rsr3: ReadOnly<u32, Dx6rsr3::Register>,
    /// DATX8 n General Status Register 0
    pub dx6gsr0: ReadOnly<u32, Dx6gsr0::Register>,
    /// DATX8 n General Status Register 1
    pub dx6gsr1: ReadOnly<u32, Dx6gsr1::Register>,
    /// DATX8 n General Status Register 2
    pub dx6gsr2: ReadOnly<u32, Dx6gsr2::Register>,
    /// DATX8 n General Status Register 3
    pub dx6gsr3: ReadOnly<u32, Dx6gsr3::Register>,
    _padding3568: [u8; 16],
    /// DATX8 n General Configuration Register 0
    pub dx7gcr0: Aliased<u32, Dx7gcr0R::Register, Dx7gcr0W::Register>,
    /// DATX8 n General Configuration Register 1
    pub dx7gcr1: Aliased<u32, Dx7gcr1R::Register, Dx7gcr1W::Register>,
    /// DATX8 n General Configuration Register 2
    pub dx7gcr2: ReadWrite<u32, Dx7gcr2::Register>,
    /// DATX8 n General Configuration Register 3
    pub dx7gcr3: Aliased<u32, Dx7gcr3R::Register, Dx7gcr3W::Register>,
    /// DATX8 n General Configuration Register 4
    pub dx7gcr4: Aliased<u32, Dx7gcr4R::Register, Dx7gcr4W::Register>,
    /// DATX8 n General Configuration Register 5
    pub dx7gcr5: Aliased<u32, Dx7gcr5R::Register, Dx7gcr5W::Register>,
    /// DATX8 n General Configuration Register 6
    pub dx7gcr6: Aliased<u32, Dx7gcr6R::Register, Dx7gcr6W::Register>,
    _padding3612: [u8; 36],
    /// DATX8 n Bit Delay Line Register 0
    pub dx7bdlr0: Aliased<u32, Dx7bdlr0R::Register, Dx7bdlr0W::Register>,
    /// DATX8 n Bit Delay Line Register 1
    pub dx7bdlr1: Aliased<u32, Dx7bdlr1R::Register, Dx7bdlr1W::Register>,
    /// DATX8 n Bit Delay Line Register 2
    pub dx7bdlr2: Aliased<u32, Dx7bdlr2R::Register, Dx7bdlr2W::Register>,
    _padding3660: [u8; 4],
    /// DATX8 n Bit Delay Line Register 3
    pub dx7bdlr3: Aliased<u32, Dx7bdlr3R::Register, Dx7bdlr3W::Register>,
    /// DATX8 n Bit Delay Line Register 4
    pub dx7bdlr4: Aliased<u32, Dx7bdlr4R::Register, Dx7bdlr4W::Register>,
    /// DATX8 n Bit Delay Line Register 5
    pub dx7bdlr5: Aliased<u32, Dx7bdlr5R::Register, Dx7bdlr5W::Register>,
    _padding3676: [u8; 4],
    /// DATX8 n Bit Delay Line Register 6
    pub dx7bdlr6: Aliased<u32, Dx7bdlr6R::Register, Dx7bdlr6W::Register>,
    _padding3684: [u8; 28],
    /// DATX8 n Local Calibrated Delay Line Register 0
    pub dx7lcdlr0: Aliased<u32, Dx7lcdlr0R::Register, Dx7lcdlr0W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 1
    pub dx7lcdlr1: Aliased<u32, Dx7lcdlr1R::Register, Dx7lcdlr1W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 2
    pub dx7lcdlr2: Aliased<u32, Dx7lcdlr2R::Register, Dx7lcdlr2W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 3
    pub dx7lcdlr3: Aliased<u32, Dx7lcdlr3R::Register, Dx7lcdlr3W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 4
    pub dx7lcdlr4: Aliased<u32, Dx7lcdlr4R::Register, Dx7lcdlr4W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 5
    pub dx7lcdlr5: Aliased<u32, Dx7lcdlr5R::Register, Dx7lcdlr5W::Register>,
    _padding3736: [u8; 8],
    /// DATX8 n Master Delay Line Register 0
    pub dx7mdlr0: Aliased<u32, Dx7mdlr0R::Register, Dx7mdlr0W::Register>,
    /// DATX8 n Master Delay Line Register 1
    pub dx7mdlr1: Aliased<u32, Dx7mdlr1R::Register, Dx7mdlr1W::Register>,
    _padding3752: [u8; 24],
    /// DATX8 n General Timing Register 0
    pub dx7gtr0: Aliased<u32, Dx7gtr0R::Register, Dx7gtr0W::Register>,
    _padding3780: [u8; 16],
    /// DATX8 n Rank Status Register 1
    pub dx7rsr1: ReadOnly<u32, Dx7rsr1::Register>,
    /// DATX8 n Rank Status Register 2
    pub dx7rsr2: ReadOnly<u32, Dx7rsr2::Register>,
    /// DATX8 n Rank Status Register 3
    pub dx7rsr3: ReadOnly<u32, Dx7rsr3::Register>,
    /// DATX8 n General Status Register 0
    pub dx7gsr0: ReadOnly<u32, Dx7gsr0::Register>,
    /// DATX8 n General Status Register 1
    pub dx7gsr1: ReadOnly<u32, Dx7gsr1::Register>,
    /// DATX8 n General Status Register 2
    pub dx7gsr2: ReadOnly<u32, Dx7gsr2::Register>,
    /// DATX8 n General Status Register 3
    pub dx7gsr3: ReadOnly<u32, Dx7gsr3::Register>,
    _padding3824: [u8; 16],
    /// DATX8 n General Configuration Register 0
    pub dx8gcr0: Aliased<u32, Dx8gcr0R::Register, Dx8gcr0W::Register>,
    /// DATX8 n General Configuration Register 1
    pub dx8gcr1: Aliased<u32, Dx8gcr1R::Register, Dx8gcr1W::Register>,
    /// DATX8 n General Configuration Register 2
    pub dx8gcr2: ReadWrite<u32, Dx8gcr2::Register>,
    /// DATX8 n General Configuration Register 3
    pub dx8gcr3: Aliased<u32, Dx8gcr3R::Register, Dx8gcr3W::Register>,
    /// DATX8 n General Configuration Register 4
    pub dx8gcr4: Aliased<u32, Dx8gcr4R::Register, Dx8gcr4W::Register>,
    /// DATX8 n General Configuration Register 5
    pub dx8gcr5: Aliased<u32, Dx8gcr5R::Register, Dx8gcr5W::Register>,
    /// DATX8 n General Configuration Register 6
    pub dx8gcr6: Aliased<u32, Dx8gcr6R::Register, Dx8gcr6W::Register>,
    _padding3868: [u8; 36],
    /// DATX8 n Bit Delay Line Register 0
    pub dx8bdlr0: Aliased<u32, Dx8bdlr0R::Register, Dx8bdlr0W::Register>,
    /// DATX8 n Bit Delay Line Register 1
    pub dx8bdlr1: Aliased<u32, Dx8bdlr1R::Register, Dx8bdlr1W::Register>,
    /// DATX8 n Bit Delay Line Register 2
    pub dx8bdlr2: Aliased<u32, Dx8bdlr2R::Register, Dx8bdlr2W::Register>,
    _padding3916: [u8; 4],
    /// DATX8 n Bit Delay Line Register 3
    pub dx8bdlr3: Aliased<u32, Dx8bdlr3R::Register, Dx8bdlr3W::Register>,
    /// DATX8 n Bit Delay Line Register 4
    pub dx8bdlr4: Aliased<u32, Dx8bdlr4R::Register, Dx8bdlr4W::Register>,
    /// DATX8 n Bit Delay Line Register 5
    pub dx8bdlr5: Aliased<u32, Dx8bdlr5R::Register, Dx8bdlr5W::Register>,
    _padding3932: [u8; 4],
    /// DATX8 n Bit Delay Line Register 6
    pub dx8bdlr6: Aliased<u32, Dx8bdlr6R::Register, Dx8bdlr6W::Register>,
    _padding3940: [u8; 28],
    /// DATX8 n Local Calibrated Delay Line Register 0
    pub dx8lcdlr0: Aliased<u32, Dx8lcdlr0R::Register, Dx8lcdlr0W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 1
    pub dx8lcdlr1: Aliased<u32, Dx8lcdlr1R::Register, Dx8lcdlr1W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 2
    pub dx8lcdlr2: Aliased<u32, Dx8lcdlr2R::Register, Dx8lcdlr2W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 3
    pub dx8lcdlr3: Aliased<u32, Dx8lcdlr3R::Register, Dx8lcdlr3W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 4
    pub dx8lcdlr4: Aliased<u32, Dx8lcdlr4R::Register, Dx8lcdlr4W::Register>,
    /// DATX8 n Local Calibrated Delay Line Register 5
    pub dx8lcdlr5: Aliased<u32, Dx8lcdlr5R::Register, Dx8lcdlr5W::Register>,
    _padding3992: [u8; 8],
    /// DATX8 n Master Delay Line Register 0
    pub dx8mdlr0: Aliased<u32, Dx8mdlr0R::Register, Dx8mdlr0W::Register>,
    /// DATX8 n Master Delay Line Register 1
    pub dx8mdlr1: Aliased<u32, Dx8mdlr1R::Register, Dx8mdlr1W::Register>,
    _padding4008: [u8; 24],
    /// DATX8 n General Timing Register 0
    pub dx8gtr0: Aliased<u32, Dx8gtr0R::Register, Dx8gtr0W::Register>,
    _padding4036: [u8; 16],
    /// DATX8 n Rank Status Register 1
    pub dx8rsr1: ReadOnly<u32, Dx8rsr1::Register>,
    /// DATX8 n Rank Status Register 2
    pub dx8rsr2: ReadOnly<u32, Dx8rsr2::Register>,
    /// DATX8 n Rank Status Register 3
    pub dx8rsr3: ReadOnly<u32, Dx8rsr3::Register>,
    /// DATX8 n General Status Register 0
    pub dx8gsr0: ReadOnly<u32, Dx8gsr0::Register>,
    /// DATX8 n General Status Register 1
    pub dx8gsr1: ReadOnly<u32, Dx8gsr1::Register>,
    /// DATX8 n General Status Register 2
    pub dx8gsr2: ReadOnly<u32, Dx8gsr2::Register>,
    /// DATX8 n General Status Register 3
    pub dx8gsr3: ReadOnly<u32, Dx8gsr3::Register>,
    _padding4080: [u8; 1040],
    /// DATX8 0-1 Oscillator, Delay Line Test, PHY FIFO and High Speed Reset, Loopback, and Gated Clock Control Register
    pub dx8sl0osc: Aliased<u32, Dx8sl0oscR::Register, Dx8sl0oscW::Register>,
    /// DAXT8 0-1 PLL Control Register 0
    pub dx8sl0pllcr0: Aliased<u32, Dx8sl0pllcr0R::Register, Dx8sl0pllcr0W::Register>,
    /// DAXT8 0-1 PLL Control Register 1 (Type B PLL Only)
    pub dx8sl0pllcr1: Aliased<u32, Dx8sl0pllcr1R::Register, Dx8sl0pllcr1W::Register>,
    /// DAXT8 0-1 PLL Control Register 2 (Type B PLL Only)
    pub dx8sl0pllcr2: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 3 (Type B PLL Only)
    pub dx8sl0pllcr3: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 4 (Type B PLL Only)
    pub dx8sl0pllcr4: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 5 (Type B PLL Only)
    pub dx8sl0pllcr5: Aliased<u32, Dx8sl0pllcr5R::Register, Dx8sl0pllcr5W::Register>,
    /// DATX8 0-1 DQS Control Register
    pub dx8sl0dqsctl: Aliased<u32, Dx8sl0dqsctlR::Register, Dx8sl0dqsctlW::Register>,
    /// DATX8 0-1 Training Control Register
    pub dx8sl0trnctl: ReadOnly<u32>,
    /// DATX8 0-1 DDL Control Register
    pub dx8sl0ddlctl: Aliased<u32, Dx8sl0ddlctlR::Register, Dx8sl0ddlctlW::Register>,
    /// DATX8 0-1 DX Control Register 1
    pub dx8sl0dxctl1: Aliased<u32, Dx8sl0dxctl1R::Register, Dx8sl0dxctl1W::Register>,
    /// DATX8 0-1 DX Control Register 2
    pub dx8sl0dxctl2: Aliased<u32, Dx8sl0dxctl2R::Register, Dx8sl0dxctl2W::Register>,
    /// DATX8 0-1 I/O Configuration Register
    pub dx8sl0iocr: Aliased<u32, Dx8sl0iocrR::Register, Dx8sl0iocrW::Register>,
    _padding5172: [u8; 12],
    /// DATX8 0-1 Oscillator, Delay Line Test, PHY FIFO and High Speed Reset, Loopback, and Gated Clock Control Register
    pub dx8sl1osc: Aliased<u32, Dx8sl1oscR::Register, Dx8sl1oscW::Register>,
    /// DAXT8 0-1 PLL Control Register 0
    pub dx8sl1pllcr0: Aliased<u32, Dx8sl1pllcr0R::Register, Dx8sl1pllcr0W::Register>,
    /// DAXT8 0-1 PLL Control Register 1 (Type B PLL Only)
    pub dx8sl1pllcr1: Aliased<u32, Dx8sl1pllcr1R::Register, Dx8sl1pllcr1W::Register>,
    /// DAXT8 0-1 PLL Control Register 2 (Type B PLL Only)
    pub dx8sl1pllcr2: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 3 (Type B PLL Only)
    pub dx8sl1pllcr3: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 4 (Type B PLL Only)
    pub dx8sl1pllcr4: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 5 (Type B PLL Only)
    pub dx8sl1pllcr5: Aliased<u32, Dx8sl1pllcr5R::Register, Dx8sl1pllcr5W::Register>,
    /// DATX8 0-1 DQS Control Register
    pub dx8sl1dqsctl: Aliased<u32, Dx8sl1dqsctlR::Register, Dx8sl1dqsctlW::Register>,
    /// DATX8 0-1 Training Control Register
    pub dx8sl1trnctl: ReadOnly<u32>,
    /// DATX8 0-1 DDL Control Register
    pub dx8sl1ddlctl: Aliased<u32, Dx8sl1ddlctlR::Register, Dx8sl1ddlctlW::Register>,
    /// DATX8 0-1 DX Control Register 1
    pub dx8sl1dxctl1: Aliased<u32, Dx8sl1dxctl1R::Register, Dx8sl1dxctl1W::Register>,
    /// DATX8 0-1 DX Control Register 2
    pub dx8sl1dxctl2: Aliased<u32, Dx8sl1dxctl2R::Register, Dx8sl1dxctl2W::Register>,
    /// DATX8 0-1 I/O Configuration Register
    pub dx8sl1iocr: Aliased<u32, Dx8sl1iocrR::Register, Dx8sl1iocrW::Register>,
    _padding5236: [u8; 12],
    /// DATX8 0-1 Oscillator, Delay Line Test, PHY FIFO and High Speed Reset, Loopback, and Gated Clock Control Register
    pub dx8sl2osc: Aliased<u32, Dx8sl2oscR::Register, Dx8sl2oscW::Register>,
    /// DAXT8 0-1 PLL Control Register 0
    pub dx8sl2pllcr0: Aliased<u32, Dx8sl2pllcr0R::Register, Dx8sl2pllcr0W::Register>,
    /// DAXT8 0-1 PLL Control Register 1 (Type B PLL Only)
    pub dx8sl2pllcr1: Aliased<u32, Dx8sl2pllcr1R::Register, Dx8sl2pllcr1W::Register>,
    /// DAXT8 0-1 PLL Control Register 2 (Type B PLL Only)
    pub dx8sl2pllcr2: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 3 (Type B PLL Only)
    pub dx8sl2pllcr3: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 4 (Type B PLL Only)
    pub dx8sl2pllcr4: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 5 (Type B PLL Only)
    pub dx8sl2pllcr5: Aliased<u32, Dx8sl2pllcr5R::Register, Dx8sl2pllcr5W::Register>,
    /// DATX8 0-1 DQS Control Register
    pub dx8sl2dqsctl: Aliased<u32, Dx8sl2dqsctlR::Register, Dx8sl2dqsctlW::Register>,
    /// DATX8 0-1 Training Control Register
    pub dx8sl2trnctl: ReadOnly<u32>,
    /// DATX8 0-1 DDL Control Register
    pub dx8sl2ddlctl: Aliased<u32, Dx8sl2ddlctlR::Register, Dx8sl2ddlctlW::Register>,
    /// DATX8 0-1 DX Control Register 1
    pub dx8sl2dxctl1: Aliased<u32, Dx8sl2dxctl1R::Register, Dx8sl2dxctl1W::Register>,
    /// DATX8 0-1 DX Control Register 2
    pub dx8sl2dxctl2: Aliased<u32, Dx8sl2dxctl2R::Register, Dx8sl2dxctl2W::Register>,
    /// DATX8 0-1 I/O Configuration Register
    pub dx8sl2iocr: Aliased<u32, Dx8sl2iocrR::Register, Dx8sl2iocrW::Register>,
    _padding5300: [u8; 12],
    /// DATX8 0-1 Oscillator, Delay Line Test, PHY FIFO and High Speed Reset, Loopback, and Gated Clock Control Register
    pub dx8sl3osc: Aliased<u32, Dx8sl3oscR::Register, Dx8sl3oscW::Register>,
    /// DAXT8 0-1 PLL Control Register 0
    pub dx8sl3pllcr0: Aliased<u32, Dx8sl3pllcr0R::Register, Dx8sl3pllcr0W::Register>,
    /// DAXT8 0-1 PLL Control Register 1 (Type B PLL Only)
    pub dx8sl3pllcr1: Aliased<u32, Dx8sl3pllcr1R::Register, Dx8sl3pllcr1W::Register>,
    /// DAXT8 0-1 PLL Control Register 2 (Type B PLL Only)
    pub dx8sl3pllcr2: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 3 (Type B PLL Only)
    pub dx8sl3pllcr3: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 4 (Type B PLL Only)
    pub dx8sl3pllcr4: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 5 (Type B PLL Only)
    pub dx8sl3pllcr5: Aliased<u32, Dx8sl3pllcr5R::Register, Dx8sl3pllcr5W::Register>,
    /// DATX8 0-1 DQS Control Register
    pub dx8sl3dqsctl: Aliased<u32, Dx8sl3dqsctlR::Register, Dx8sl3dqsctlW::Register>,
    /// DATX8 0-1 Training Control Register
    pub dx8sl3trnctl: ReadOnly<u32>,
    /// DATX8 0-1 DDL Control Register
    pub dx8sl3ddlctl: Aliased<u32, Dx8sl3ddlctlR::Register, Dx8sl3ddlctlW::Register>,
    /// DATX8 0-1 DX Control Register 1
    pub dx8sl3dxctl1: Aliased<u32, Dx8sl3dxctl1R::Register, Dx8sl3dxctl1W::Register>,
    /// DATX8 0-1 DX Control Register 2
    pub dx8sl3dxctl2: Aliased<u32, Dx8sl3dxctl2R::Register, Dx8sl3dxctl2W::Register>,
    /// DATX8 0-1 I/O Configuration Register
    pub dx8sl3iocr: Aliased<u32, Dx8sl3iocrR::Register, Dx8sl3iocrW::Register>,
    _padding5364: [u8; 12],
    /// DATX8 0-1 Oscillator, Delay Line Test, PHY FIFO and High Speed Reset, Loopback, and Gated Clock Control Register
    pub dx8sl4osc: Aliased<u32, Dx8sl4oscR::Register, Dx8sl4oscW::Register>,
    /// DAXT8 0-1 PLL Control Register 0
    pub dx8sl4pllcr0: Aliased<u32, Dx8sl4pllcr0R::Register, Dx8sl4pllcr0W::Register>,
    /// DAXT8 0-1 PLL Control Register 1 (Type B PLL Only)
    pub dx8sl4pllcr1: Aliased<u32, Dx8sl4pllcr1R::Register, Dx8sl4pllcr1W::Register>,
    /// DAXT8 0-1 PLL Control Register 2 (Type B PLL Only)
    pub dx8sl4pllcr2: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 3 (Type B PLL Only)
    pub dx8sl4pllcr3: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 4 (Type B PLL Only)
    pub dx8sl4pllcr4: ReadWrite<u32>,
    /// DAXT8 0-1 PLL Control Register 5 (Type B PLL Only)
    pub dx8sl4pllcr5: Aliased<u32, Dx8sl4pllcr5R::Register, Dx8sl4pllcr5W::Register>,
    /// DATX8 0-1 DQS Control Register
    pub dx8sl4dqsctl: Aliased<u32, Dx8sl4dqsctlR::Register, Dx8sl4dqsctlW::Register>,
    /// DATX8 0-1 Training Control Register
    pub dx8sl4trnctl: ReadOnly<u32>,
    /// DATX8 0-1 DDL Control Register
    pub dx8sl4ddlctl: Aliased<u32, Dx8sl4ddlctlR::Register, Dx8sl4ddlctlW::Register>,
    /// DATX8 0-1 DX Control Register 1
    pub dx8sl4dxctl1: Aliased<u32, Dx8sl4dxctl1R::Register, Dx8sl4dxctl1W::Register>,
    /// DATX8 0-1 DX Control Register 2
    pub dx8sl4dxctl2: Aliased<u32, Dx8sl4dxctl2R::Register, Dx8sl4dxctl2W::Register>,
    /// DATX8 0-1 I/O Configuration Register
    pub dx8sl4iocr: Aliased<u32, Dx8sl4iocrR::Register, Dx8sl4iocrW::Register>,
    _padding5428: [u8; 652],
    /// DATX8 0-8 Oscillator, Delay Line Test, PHY FIFO and High Speed Reset, Loopback, and Gated Clock Control Register
    pub dx8slbosc: WriteOnly<u32, Dx8slbosc::Register>,
    /// DAXT8 0-8 PLL Control Register 0
    pub dx8slbpllcr0: WriteOnly<u32, Dx8slbpllcr0::Register>,
    /// DAXT8 0-8 PLL Control Register 1 (Type B PLL Only)
    pub dx8slbpllcr1: WriteOnly<u32, Dx8slbpllcr1::Register>,
    /// DAXT8 0-8 PLL Control Register 2 (Type B PLL Only)
    pub dx8slbpllcr2: WriteOnly<u32>,
    /// DAXT8 0-8 PLL Control Register 3 (Type B PLL Only)
    pub dx8slbpllcr3: WriteOnly<u32>,
    /// DAXT8 0-8 PLL Control Register 4 (Type B PLL Only)
    pub dx8slbpllcr4: WriteOnly<u32>,
    /// DAXT8 0-8 PLL Control Register 5 (Type B PLL Only)
    pub dx8slbpllcr5: WriteOnly<u32, Dx8slbpllcr5::Register>,
    /// DATX8 0-8 DQS Control Register
    pub dx8slbdqsctl: WriteOnly<u32, Dx8slbdqsctl::Register>,
    /// DATX8 0-8 Training Control Register
    pub dx8slbtrnctl: WriteOnly<u32>,
    /// DATX8 0-8 DDL Control Register
    pub dx8slbddlctl: WriteOnly<u32, Dx8slbddlctl::Register>,
    /// DATX8 0-8 DX Control Register 1
    pub dx8slbdxctl1: WriteOnly<u32, Dx8slbdxctl1::Register>,
    /// DATX8 0-8 DX Control Register 2
    pub dx8slbdxctl2: WriteOnly<u32, Dx8slbdxctl2::Register>,
    /// DATX8 0-8 I/O Configuration Register
    pub dx8slbiocr: WriteOnly<u32, Dx8slbiocr::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub PirR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        ZCALBYP OFFSET(30) NUMBITS(1) [],
        DCALPSE OFFSET(29) NUMBITS(1) [],
        RESERVED1 OFFSET(21) NUMBITS(8) [],
        DQS2DQ OFFSET(20) NUMBITS(1) [],
        RDIMMINIT OFFSET(19) NUMBITS(1) [],
        CTLDINIT OFFSET(18) NUMBITS(1) [],
        VREF OFFSET(17) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(1) [],
        WREYE OFFSET(15) NUMBITS(1) [],
        RDEYE OFFSET(14) NUMBITS(1) [],
        WRDSKW OFFSET(13) NUMBITS(1) [],
        RDDSKW OFFSET(12) NUMBITS(1) [],
        WLADJ OFFSET(11) NUMBITS(1) [],
        QSGATE OFFSET(10) NUMBITS(1) [],
        WL OFFSET(9) NUMBITS(1) [],
        DRAMINIT OFFSET(8) NUMBITS(1) [],
        DRAMRST OFFSET(7) NUMBITS(1) [],
        PHYRST OFFSET(6) NUMBITS(1) [],
        DCAL OFFSET(5) NUMBITS(1) [],
        PLLINIT OFFSET(4) NUMBITS(1) [],
        RESERVED3 OFFSET(3) NUMBITS(1) [],
        CA OFFSET(2) NUMBITS(1) [],
        ZCAL OFFSET(1) NUMBITS(1) [],
        INIT OFFSET(0) NUMBITS(1) [],
    ],
    pub PirW [
        ZCALBYP OFFSET(30) NUMBITS(1) [],
        DCALPSE OFFSET(29) NUMBITS(1) [],
        DQS2DQ OFFSET(20) NUMBITS(1) [],
        RDIMMINIT OFFSET(19) NUMBITS(1) [],
        CTLDINIT OFFSET(18) NUMBITS(1) [],
        VREF OFFSET(17) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(1) [],
        WREYE OFFSET(15) NUMBITS(1) [],
        RDEYE OFFSET(14) NUMBITS(1) [],
        WRDSKW OFFSET(13) NUMBITS(1) [],
        RDDSKW OFFSET(12) NUMBITS(1) [],
        WLADJ OFFSET(11) NUMBITS(1) [],
        QSGATE OFFSET(10) NUMBITS(1) [],
        WL OFFSET(9) NUMBITS(1) [],
        DRAMINIT OFFSET(8) NUMBITS(1) [],
        DRAMRST OFFSET(7) NUMBITS(1) [],
        PHYRST OFFSET(6) NUMBITS(1) [],
        DCAL OFFSET(5) NUMBITS(1) [],
        PLLINIT OFFSET(4) NUMBITS(1) [],
        CA OFFSET(2) NUMBITS(1) [],
        ZCAL OFFSET(1) NUMBITS(1) [],
        INIT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgcr0R [
        ADCP OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(27) NUMBITS(4) [],
        PHYFRST OFFSET(26) NUMBITS(1) [],
        OSCACDL OFFSET(24) NUMBITS(2) [],
        RESERVED1 OFFSET(19) NUMBITS(5) [],
        DTOSEL OFFSET(14) NUMBITS(5) [],
        RESERVED2 OFFSET(13) NUMBITS(1) [],
        OSCDIV OFFSET(9) NUMBITS(4) [],
        OSCEN OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(8) [],
    ],
    pub Pgcr0W [
        ADCP OFFSET(31) NUMBITS(1) [],
        PHYFRST OFFSET(26) NUMBITS(1) [],
        OSCACDL OFFSET(24) NUMBITS(2) [],
        DTOSEL OFFSET(14) NUMBITS(5) [],
        OSCDIV OFFSET(9) NUMBITS(4) [],
        OSCEN OFFSET(8) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgcr2 [
        CLRTSTAT OFFSET(31) NUMBITS(1) [],
        CLRZCAL OFFSET(30) NUMBITS(1) [],
        CLRPERR OFFSET(29) NUMBITS(1) [],
        ICPC OFFSET(28) NUMBITS(1) [],
        DTPMXTMR OFFSET(20) NUMBITS(8) [],
        INITFSMBYP OFFSET(19) NUMBITS(1) [],
        PLLFSMBYP OFFSET(18) NUMBITS(1) [],
        TREFPRD OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgcr3R [
        CKNEN OFFSET(24) NUMBITS(4) [],
        CKEN OFFSET(16) NUMBITS(4) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        GATEACRDCLK OFFSET(13) NUMBITS(2) [],
        GATEACDDRCLK OFFSET(11) NUMBITS(2) [],
        GATEACCTLCLK OFFSET(9) NUMBITS(2) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        DDLBYPMODE OFFSET(6) NUMBITS(2) [],
        IOLB OFFSET(5) NUMBITS(1) [],
        RDMODE OFFSET(3) NUMBITS(2) [],
        DISRST OFFSET(2) NUMBITS(1) [],
        CLKLEVEL OFFSET(0) NUMBITS(2) [],
    ],
    pub Pgcr3W [
        CKNEN OFFSET(24) NUMBITS(4) [],
        CKEN OFFSET(16) NUMBITS(4) [],
        GATEACRDCLK OFFSET(13) NUMBITS(2) [],
        GATEACDDRCLK OFFSET(11) NUMBITS(2) [],
        GATEACCTLCLK OFFSET(9) NUMBITS(2) [],
        DDLBYPMODE OFFSET(6) NUMBITS(2) [],
        IOLB OFFSET(5) NUMBITS(1) [],
        RDMODE OFFSET(3) NUMBITS(2) [],
        DISRST OFFSET(2) NUMBITS(1) [],
        CLKLEVEL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgcr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        ACDDLLD OFFSET(29) NUMBITS(1) [],
        ACDDLBYP OFFSET(24) NUMBITS(5) [],
        OEDDLBYP OFFSET(23) NUMBITS(1) [],
        TEDDLBYP OFFSET(22) NUMBITS(1) [],
        PDRDDLBYP OFFSET(21) NUMBITS(1) [],
        RRRMODE OFFSET(20) NUMBITS(1) [],
        WRRMODE OFFSET(19) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(11) [],
        LPWAKEUP_THRSH OFFSET(4) NUMBITS(4) [],
        RESERVED2 OFFSET(2) NUMBITS(2) [],
        LPPLLPD OFFSET(1) NUMBITS(1) [],
        LPIOPD OFFSET(0) NUMBITS(1) [],
    ],
    pub Pgcr4W [
        ACDDLLD OFFSET(29) NUMBITS(1) [],
        ACDDLBYP OFFSET(24) NUMBITS(5) [],
        OEDDLBYP OFFSET(23) NUMBITS(1) [],
        TEDDLBYP OFFSET(22) NUMBITS(1) [],
        PDRDDLBYP OFFSET(21) NUMBITS(1) [],
        RRRMODE OFFSET(20) NUMBITS(1) [],
        WRRMODE OFFSET(19) NUMBITS(1) [],
        LPWAKEUP_THRSH OFFSET(4) NUMBITS(4) [],
        LPPLLPD OFFSET(1) NUMBITS(1) [],
        LPIOPD OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgcr5R [
        FRQBT OFFSET(24) NUMBITS(8) [],
        FRQAT OFFSET(16) NUMBITS(8) [],
        DISCNPERIOD OFFSET(8) NUMBITS(8) [],
        VREF_RBCTRL OFFSET(4) NUMBITS(4) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        DXREFISELRANGE OFFSET(2) NUMBITS(1) [],
        DDLPGACT OFFSET(1) NUMBITS(1) [],
        DDLPGRW OFFSET(0) NUMBITS(1) [],
    ],
    pub Pgcr5W [
        FRQBT OFFSET(24) NUMBITS(8) [],
        FRQAT OFFSET(16) NUMBITS(8) [],
        DISCNPERIOD OFFSET(8) NUMBITS(8) [],
        VREF_RBCTRL OFFSET(4) NUMBITS(4) [],
        DXREFISELRANGE OFFSET(2) NUMBITS(1) [],
        DDLPGACT OFFSET(1) NUMBITS(1) [],
        DDLPGRW OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgcr6R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        DLDLMT OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        ACDLVT OFFSET(13) NUMBITS(1) [],
        ACBVT OFFSET(12) NUMBITS(1) [],
        ODTBVT OFFSET(11) NUMBITS(1) [],
        CKEBVT OFFSET(10) NUMBITS(1) [],
        CSNBVT OFFSET(9) NUMBITS(1) [],
        CKBVT OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(7) [],
        INHVT OFFSET(0) NUMBITS(1) [],
    ],
    pub Pgcr6W [
        DLDLMT OFFSET(16) NUMBITS(8) [],
        ACDLVT OFFSET(13) NUMBITS(1) [],
        ACBVT OFFSET(12) NUMBITS(1) [],
        ODTBVT OFFSET(11) NUMBITS(1) [],
        CKEBVT OFFSET(10) NUMBITS(1) [],
        CSNBVT OFFSET(9) NUMBITS(1) [],
        CKBVT OFFSET(8) NUMBITS(1) [],
        INHVT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgcr7R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        ACRSVD_7_6 OFFSET(6) NUMBITS(2) [],
        ACCALCLK OFFSET(5) NUMBITS(1) [],
        ACRCLKMD OFFSET(4) NUMBITS(1) [],
        ACDLDT OFFSET(3) NUMBITS(1) [],
        ACRSVD_2 OFFSET(2) NUMBITS(1) [],
        ACDTOSEL OFFSET(1) NUMBITS(1) [],
        ACTMODE OFFSET(0) NUMBITS(1) [],
    ],
    pub Pgcr7W [
        ACRSVD_7_6 OFFSET(6) NUMBITS(2) [],
        ACCALCLK OFFSET(5) NUMBITS(1) [],
        ACRCLKMD OFFSET(4) NUMBITS(1) [],
        ACDLDT OFFSET(3) NUMBITS(1) [],
        ACRSVD_2 OFFSET(2) NUMBITS(1) [],
        ACDTOSEL OFFSET(1) NUMBITS(1) [],
        ACTMODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgsr0 [
        APLOCK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(30) NUMBITS(1) [],
        CAWRN OFFSET(29) NUMBITS(1) [],
        CAERR OFFSET(28) NUMBITS(1) [],
        WEERR OFFSET(27) NUMBITS(1) [],
        REERR OFFSET(26) NUMBITS(1) [],
        WDERR OFFSET(25) NUMBITS(1) [],
        RDERR OFFSET(24) NUMBITS(1) [],
        WLAERR OFFSET(23) NUMBITS(1) [],
        QSGERR OFFSET(22) NUMBITS(1) [],
        WLERR OFFSET(21) NUMBITS(1) [],
        ZCERR OFFSET(20) NUMBITS(1) [],
        VERR OFFSET(19) NUMBITS(1) [],
        DQS2DQERR OFFSET(18) NUMBITS(1) [],
        RESERVED1 OFFSET(16) NUMBITS(2) [],
        DQS2DQDONE OFFSET(15) NUMBITS(1) [],
        VDONE OFFSET(14) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(1) [],
        CADONE OFFSET(12) NUMBITS(1) [],
        WEDONE OFFSET(11) NUMBITS(1) [],
        REDONE OFFSET(10) NUMBITS(1) [],
        WDDONE OFFSET(9) NUMBITS(1) [],
        RDDONE OFFSET(8) NUMBITS(1) [],
        WLADONE OFFSET(7) NUMBITS(1) [],
        QSGDONE OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        DIDONE OFFSET(4) NUMBITS(1) [],
        ZCDONE OFFSET(3) NUMBITS(1) [],
        DCDONE OFFSET(2) NUMBITS(1) [],
        PLDONE OFFSET(1) NUMBITS(1) [],
        IDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgsr1 [
        PARERR OFFSET(31) NUMBITS(1) [],
        VTSTOP OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(25) NUMBITS(5) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pgsr2 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ptr0 [
        TPLLPD OFFSET(21) NUMBITS(11) [],
        TPLLGS OFFSET(6) NUMBITS(15) [],
        TPHYRST OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ptr1R [
        TPLLLOCK OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(13) NUMBITS(3) [],
        TPLLRST OFFSET(0) NUMBITS(13) [],
    ],
    pub Ptr1W [
        TPLLLOCK OFFSET(16) NUMBITS(16) [],
        TPLLRST OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ptr2R [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        TWLDLYS OFFSET(15) NUMBITS(5) [],
        TCALH OFFSET(10) NUMBITS(5) [],
        TCALS OFFSET(5) NUMBITS(5) [],
        TCALON OFFSET(0) NUMBITS(5) [],
    ],
    pub Ptr2W [
        TWLDLYS OFFSET(15) NUMBITS(5) [],
        TCALH OFFSET(10) NUMBITS(5) [],
        TCALS OFFSET(5) NUMBITS(5) [],
        TCALON OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ptr3R [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        TDINIT0 OFFSET(0) NUMBITS(23) [],
    ],
    pub Ptr3W [
        TDINIT0 OFFSET(0) NUMBITS(23) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ptr4R [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        TDINIT1 OFFSET(0) NUMBITS(13) [],
    ],
    pub Ptr4W [
        TDINIT1 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ptr5R [
        RESERVED0 OFFSET(19) NUMBITS(13) [],
        TDINIT2 OFFSET(0) NUMBITS(19) [],
    ],
    pub Ptr5W [
        TDINIT2 OFFSET(0) NUMBITS(19) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ptr6R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        TDINIT4 OFFSET(20) NUMBITS(7) [],
        RESERVED1 OFFSET(12) NUMBITS(8) [],
        TDINIT3 OFFSET(0) NUMBITS(12) [],
    ],
    pub Ptr6W [
        TDINIT4 OFFSET(20) NUMBITS(7) [],
        TDINIT3 OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pllcr0R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(4) [],
        RESERVED4 OFFSET(0) NUMBITS(4) [],
    ],
    pub Pllcr0W [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        RESERVED3 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pllcr1R [
        PLLPROG OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(6) NUMBITS(10) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ],
    pub Pllcr1W [
        PLLPROG OFFSET(16) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pllcr5R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ],
    pub Pllcr5W [
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DxccrR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RKLOOP OFFSET(29) NUMBITS(1) [],
        RESERVED1 OFFSET(7) NUMBITS(22) [],
        DQS2DQMPER OFFSET(3) NUMBITS(4) [],
        RESERVED2 OFFSET(0) NUMBITS(3) [],
    ],
    pub DxccrW [
        RKLOOP OFFSET(29) NUMBITS(1) [],
        DQS2DQMPER OFFSET(3) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DsgcrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RDBICLSEL OFFSET(27) NUMBITS(1) [],
        RDBICL OFFSET(24) NUMBITS(3) [],
        PHYZUEN OFFSET(23) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(1) [],
        RSTOE OFFSET(21) NUMBITS(1) [],
        SDRMODE OFFSET(19) NUMBITS(2) [],
        RESERVED2 OFFSET(18) NUMBITS(1) [],
        ATOAE OFFSET(17) NUMBITS(1) [],
        DTOOE OFFSET(16) NUMBITS(1) [],
        DTOIOM OFFSET(15) NUMBITS(1) [],
        DTOPDR OFFSET(14) NUMBITS(1) [],
        RESERVED3 OFFSET(13) NUMBITS(1) [],
        DTOODT OFFSET(12) NUMBITS(1) [],
        PUAD OFFSET(6) NUMBITS(6) [],
        CUAEN OFFSET(5) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(2) [],
        CTLZUEN OFFSET(2) NUMBITS(1) [],
        RESERVED5 OFFSET(1) NUMBITS(1) [],
        PUREN OFFSET(0) NUMBITS(1) [],
    ],
    pub DsgcrW [
        RDBICLSEL OFFSET(27) NUMBITS(1) [],
        RDBICL OFFSET(24) NUMBITS(3) [],
        PHYZUEN OFFSET(23) NUMBITS(1) [],
        RSTOE OFFSET(21) NUMBITS(1) [],
        SDRMODE OFFSET(19) NUMBITS(2) [],
        ATOAE OFFSET(17) NUMBITS(1) [],
        DTOOE OFFSET(16) NUMBITS(1) [],
        DTOIOM OFFSET(15) NUMBITS(1) [],
        DTOPDR OFFSET(14) NUMBITS(1) [],
        DTOODT OFFSET(12) NUMBITS(1) [],
        PUAD OFFSET(6) NUMBITS(6) [],
        CUAEN OFFSET(5) NUMBITS(1) [],
        CTLZUEN OFFSET(2) NUMBITS(1) [],
        PUREN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OdtcrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        WRODT_RSVD OFFSET(18) NUMBITS(10) [],
        WRODT OFFSET(16) NUMBITS(2) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        RDODT_RSVD OFFSET(2) NUMBITS(10) [],
        RDODT OFFSET(0) NUMBITS(2) [],
    ],
    pub OdtcrW [
        WRODT OFFSET(16) NUMBITS(2) [],
        RDODT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aacr [
        AAOENC OFFSET(31) NUMBITS(1) [],
        AAENC OFFSET(30) NUMBITS(1) [],
        AATR OFFSET(0) NUMBITS(30) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gpr0 [
        GPR0 OFFSET(1) NUMBITS(31) [],
        WDQSEXT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gpr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RESERVED1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcrR [
        GEARDN OFFSET(31) NUMBITS(1) [],
        UBG OFFSET(30) NUMBITS(1) [],
        UDIMM OFFSET(29) NUMBITS(1) [],
        DDR2T OFFSET(28) NUMBITS(1) [],
        NOSRA OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(18) NUMBITS(9) [],
        BYTEMASK OFFSET(10) NUMBITS(8) [],
        DDRTYPE OFFSET(8) NUMBITS(2) [],
        MPRDQ OFFSET(7) NUMBITS(1) [],
        PDQ OFFSET(4) NUMBITS(3) [],
        DDR8BNK OFFSET(3) NUMBITS(1) [],
        DDRMD OFFSET(0) NUMBITS(3) [],
    ],
    pub DcrW [
        GEARDN OFFSET(31) NUMBITS(1) [],
        UBG OFFSET(30) NUMBITS(1) [],
        UDIMM OFFSET(29) NUMBITS(1) [],
        DDR2T OFFSET(28) NUMBITS(1) [],
        NOSRA OFFSET(27) NUMBITS(1) [],
        BYTEMASK OFFSET(10) NUMBITS(8) [],
        DDRTYPE OFFSET(8) NUMBITS(2) [],
        MPRDQ OFFSET(7) NUMBITS(1) [],
        PDQ OFFSET(4) NUMBITS(3) [],
        DDR8BNK OFFSET(3) NUMBITS(1) [],
        DDRMD OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtpr0R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        TRRD OFFSET(24) NUMBITS(5) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        TRAS OFFSET(16) NUMBITS(7) [],
        RESERVED2 OFFSET(15) NUMBITS(1) [],
        TRP OFFSET(8) NUMBITS(7) [],
        RESERVED3 OFFSET(5) NUMBITS(3) [],
        TRTP OFFSET(0) NUMBITS(5) [],
    ],
    pub Dtpr0W [
        TRRD OFFSET(24) NUMBITS(5) [],
        TRAS OFFSET(16) NUMBITS(7) [],
        TRP OFFSET(8) NUMBITS(7) [],
        TRTP OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtpr1R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        TWLMRD OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        TFAW OFFSET(16) NUMBITS(7) [],
        RESERVED2 OFFSET(11) NUMBITS(5) [],
        TMOD OFFSET(8) NUMBITS(3) [],
        RESERVED3 OFFSET(5) NUMBITS(3) [],
        TMRD OFFSET(0) NUMBITS(5) [],
    ],
    pub Dtpr1W [
        TWLMRD OFFSET(24) NUMBITS(7) [],
        TFAW OFFSET(16) NUMBITS(7) [],
        TMOD OFFSET(8) NUMBITS(3) [],
        TMRD OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtpr2R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        TRTW OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(25) NUMBITS(3) [],
        TRTODT OFFSET(24) NUMBITS(1) [],
        RESERVED2 OFFSET(20) NUMBITS(4) [],
        TCKE OFFSET(16) NUMBITS(4) [],
        RESERVED3 OFFSET(10) NUMBITS(6) [],
        TXS OFFSET(0) NUMBITS(10) [],
    ],
    pub Dtpr2W [
        TRTW OFFSET(28) NUMBITS(1) [],
        TRTODT OFFSET(24) NUMBITS(1) [],
        TCKE OFFSET(16) NUMBITS(4) [],
        TXS OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtpr3R [
        TOFDX OFFSET(29) NUMBITS(3) [],
        TCCD OFFSET(26) NUMBITS(3) [],
        TDLLK OFFSET(16) NUMBITS(10) [],
        RESERVED0 OFFSET(12) NUMBITS(4) [],
        TDQSCKMAX OFFSET(8) NUMBITS(4) [],
        RESERVED1 OFFSET(3) NUMBITS(5) [],
        TDQSCK OFFSET(0) NUMBITS(3) [],
    ],
    pub Dtpr3W [
        TOFDX OFFSET(29) NUMBITS(3) [],
        TCCD OFFSET(26) NUMBITS(3) [],
        TDLLK OFFSET(16) NUMBITS(10) [],
        TDQSCKMAX OFFSET(8) NUMBITS(4) [],
        TDQSCK OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtpr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        TAOND_TAOFD OFFSET(28) NUMBITS(2) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        TRFC OFFSET(16) NUMBITS(10) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        TWLO OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(5) NUMBITS(3) [],
        TXP OFFSET(0) NUMBITS(5) [],
    ],
    pub Dtpr4W [
        TAOND_TAOFD OFFSET(28) NUMBITS(2) [],
        TRFC OFFSET(16) NUMBITS(10) [],
        TWLO OFFSET(8) NUMBITS(6) [],
        TXP OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtpr5R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        TRC OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(15) NUMBITS(1) [],
        TRCD OFFSET(8) NUMBITS(7) [],
        RESERVED2 OFFSET(5) NUMBITS(3) [],
        TWTR OFFSET(0) NUMBITS(5) [],
    ],
    pub Dtpr5W [
        TRC OFFSET(16) NUMBITS(8) [],
        TRCD OFFSET(8) NUMBITS(7) [],
        TWTR OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtpr6R [
        PUBWLEN OFFSET(31) NUMBITS(1) [],
        PUBRLEN OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(14) NUMBITS(16) [],
        PUBWL OFFSET(8) NUMBITS(6) [],
        RESERVED1 OFFSET(6) NUMBITS(2) [],
        PUBRL OFFSET(0) NUMBITS(6) [],
    ],
    pub Dtpr6W [
        PUBWLEN OFFSET(31) NUMBITS(1) [],
        PUBRLEN OFFSET(30) NUMBITS(1) [],
        PUBWL OFFSET(8) NUMBITS(6) [],
        PUBRL OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rdimmgcr0R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        QCSEN OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(28) NUMBITS(2) [],
        RDIMMIOM OFFSET(27) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(3) [],
        ERROUTOE OFFSET(23) NUMBITS(1) [],
        ERROUTIOM OFFSET(22) NUMBITS(1) [],
        ERROUTPDR OFFSET(21) NUMBITS(1) [],
        RESERVED3 OFFSET(20) NUMBITS(1) [],
        ERROUTODT OFFSET(19) NUMBITS(1) [],
        RESERVED4 OFFSET(18) NUMBITS(1) [],
        PARINIOM OFFSET(17) NUMBITS(1) [],
        RESERVED5 OFFSET(8) NUMBITS(9) [],
        RNKMRREN_RSVD OFFSET(6) NUMBITS(2) [],
        RNKMRREN OFFSET(4) NUMBITS(2) [],
        RESERVED6 OFFSET(3) NUMBITS(1) [],
        SOPERR OFFSET(2) NUMBITS(1) [],
        ERRNOREG OFFSET(1) NUMBITS(1) [],
        RDIMM OFFSET(0) NUMBITS(1) [],
    ],
    pub Rdimmgcr0W [
        QCSEN OFFSET(30) NUMBITS(1) [],
        RDIMMIOM OFFSET(27) NUMBITS(1) [],
        ERROUTOE OFFSET(23) NUMBITS(1) [],
        ERROUTIOM OFFSET(22) NUMBITS(1) [],
        ERROUTPDR OFFSET(21) NUMBITS(1) [],
        ERROUTODT OFFSET(19) NUMBITS(1) [],
        RESERVED0 OFFSET(18) NUMBITS(1) [],
        PARINIOM OFFSET(17) NUMBITS(1) [],
        RNKMRREN OFFSET(4) NUMBITS(2) [],
        SOPERR OFFSET(2) NUMBITS(1) [],
        ERRNOREG OFFSET(1) NUMBITS(1) [],
        RDIMM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rdimmgcr1R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        A17BID OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(27) NUMBITS(1) [],
        TBCMRD_L2 OFFSET(24) NUMBITS(3) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        TBCMRD_L OFFSET(20) NUMBITS(3) [],
        RESERVED3 OFFSET(19) NUMBITS(1) [],
        TBCMRD OFFSET(16) NUMBITS(3) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        TBCSTAB OFFSET(0) NUMBITS(14) [],
    ],
    pub Rdimmgcr1W [
        A17BID OFFSET(28) NUMBITS(1) [],
        TBCMRD_L2 OFFSET(24) NUMBITS(3) [],
        TBCMRD_L OFFSET(20) NUMBITS(3) [],
        TBCMRD OFFSET(16) NUMBITS(3) [],
        TBCSTAB OFFSET(0) NUMBITS(14) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rdimmcr0 [
        RC7 OFFSET(28) NUMBITS(4) [],
        RC6 OFFSET(24) NUMBITS(4) [],
        RC5 OFFSET(20) NUMBITS(4) [],
        RC4 OFFSET(16) NUMBITS(4) [],
        RC3 OFFSET(12) NUMBITS(4) [],
        RC2 OFFSET(8) NUMBITS(4) [],
        RC1 OFFSET(4) NUMBITS(4) [],
        RC0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rdimmcr1 [
        RC15 OFFSET(28) NUMBITS(4) [],
        RC14 OFFSET(24) NUMBITS(4) [],
        RC13 OFFSET(20) NUMBITS(4) [],
        RC12 OFFSET(16) NUMBITS(4) [],
        RC11 OFFSET(12) NUMBITS(4) [],
        RC10 OFFSET(8) NUMBITS(4) [],
        RC9 OFFSET(4) NUMBITS(4) [],
        RC8 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rdimmcr2 [
        RC4X OFFSET(24) NUMBITS(8) [],
        RC3X OFFSET(16) NUMBITS(8) [],
        RC2X OFFSET(8) NUMBITS(8) [],
        RC1X OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rdimmcr3 [
        RC8X OFFSET(24) NUMBITS(8) [],
        RC7X OFFSET(16) NUMBITS(8) [],
        RC6X OFFSET(8) NUMBITS(8) [],
        RC5X OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rdimmcr4 [
        RCXX OFFSET(24) NUMBITS(8) [],
        RCBX OFFSET(16) NUMBITS(8) [],
        RCAX OFFSET(8) NUMBITS(8) [],
        RC9X OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Schcr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        SCHDQV OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        SP_CMD OFFSET(8) NUMBITS(4) [],
        CMD OFFSET(4) NUMBITS(4) [],
        SCHTRIG OFFSET(0) NUMBITS(4) [],
    ],
    pub Schcr0W [
        SCHDQV OFFSET(16) NUMBITS(9) [],
        SP_CMD OFFSET(8) NUMBITS(4) [],
        CMD OFFSET(4) NUMBITS(4) [],
        SCHTRIG OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Schcr1R [
        SCRNK OFFSET(28) NUMBITS(4) [],
        SCADDR OFFSET(8) NUMBITS(20) [],
        SCBG OFFSET(6) NUMBITS(2) [],
        SCBK OFFSET(4) NUMBITS(2) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        ALLRANK OFFSET(2) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(2) [],
    ],
    pub Schcr1W [
        SCRNK OFFSET(28) NUMBITS(4) [],
        SCADDR OFFSET(8) NUMBITS(20) [],
        SCBG OFFSET(6) NUMBITS(2) [],
        SCBK OFFSET(4) NUMBITS(2) [],
        ALLRANK OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr0R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RSVD_15_8 OFFSET(8) NUMBITS(8) [],
        CATR OFFSET(7) NUMBITS(1) [],
        RSVD_6_5 OFFSET(5) NUMBITS(2) [],
        RZQI OFFSET(3) NUMBITS(2) [],
        RSVD_2_0 OFFSET(0) NUMBITS(3) [],
    ],
    pub Mr0W [
        CATR OFFSET(7) NUMBITS(1) [],
        RSVD_6_5 OFFSET(5) NUMBITS(2) [],
        RZQI OFFSET(3) NUMBITS(2) [],
        RSVD_2_0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr1R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RSVD OFFSET(8) NUMBITS(8) [],
        RDPST OFFSET(7) NUMBITS(1) [],
        NWR OFFSET(4) NUMBITS(3) [],
        RDPRE OFFSET(3) NUMBITS(1) [],
        WRPRE OFFSET(2) NUMBITS(1) [],
        BL OFFSET(0) NUMBITS(2) [],
    ],
    pub Mr1W [
        RDPST OFFSET(7) NUMBITS(1) [],
        NWR OFFSET(4) NUMBITS(3) [],
        RDPRE OFFSET(3) NUMBITS(1) [],
        WRPRE OFFSET(2) NUMBITS(1) [],
        BL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr2R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RSVD OFFSET(8) NUMBITS(8) [],
        WRL OFFSET(7) NUMBITS(1) [],
        WLS OFFSET(6) NUMBITS(1) [],
        WL OFFSET(3) NUMBITS(3) [],
        RL OFFSET(0) NUMBITS(3) [],
    ],
    pub Mr2W [
        WRL OFFSET(7) NUMBITS(1) [],
        WLS OFFSET(6) NUMBITS(1) [],
        WL OFFSET(3) NUMBITS(3) [],
        RL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr3R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        DBIWR OFFSET(7) NUMBITS(1) [],
        DBIRD OFFSET(6) NUMBITS(1) [],
        PDDS OFFSET(3) NUMBITS(3) [],
        RSVD OFFSET(2) NUMBITS(1) [],
        WRPST OFFSET(1) NUMBITS(1) [],
        PUCAL OFFSET(0) NUMBITS(1) [],
    ],
    pub Mr3W [
        DBIWR OFFSET(7) NUMBITS(1) [],
        DBIRD OFFSET(6) NUMBITS(1) [],
        PDDS OFFSET(3) NUMBITS(3) [],
        RSVD OFFSET(2) NUMBITS(1) [],
        WRPST OFFSET(1) NUMBITS(1) [],
        PUCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr4R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RSVD OFFSET(0) NUMBITS(8) [],
    ],
    pub Mr4W [
        RSVD OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr5R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RSVD OFFSET(0) NUMBITS(8) [],
    ],
    pub Mr5W [
        RSVD OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr6R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RSVD OFFSET(0) NUMBITS(8) [],
    ],
    pub Mr6W [
        RSVD OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr7R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RSVD OFFSET(0) NUMBITS(8) [],
    ],
    pub Mr7W [
        RSVD OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr11R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RSVD_15_8 OFFSET(8) NUMBITS(8) [],
        RSVD_7 OFFSET(7) NUMBITS(1) [],
        CAODT OFFSET(4) NUMBITS(3) [],
        RSVD_3 OFFSET(3) NUMBITS(1) [],
        DQODT OFFSET(0) NUMBITS(3) [],
    ],
    pub Mr11W [
        RSVD_7 OFFSET(7) NUMBITS(1) [],
        CAODT OFFSET(4) NUMBITS(3) [],
        RSVD_3 OFFSET(3) NUMBITS(1) [],
        DQODT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr12R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RSVD OFFSET(7) NUMBITS(1) [],
        VR_CA OFFSET(6) NUMBITS(1) [],
        VREF_CA OFFSET(0) NUMBITS(6) [],
    ],
    pub Mr12W [
        RSVD OFFSET(7) NUMBITS(1) [],
        VR_CA OFFSET(6) NUMBITS(1) [],
        VREF_CA OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr13R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        FSPOP OFFSET(7) NUMBITS(1) [],
        FSPWR OFFSET(6) NUMBITS(1) [],
        DMD OFFSET(5) NUMBITS(1) [],
        RRO OFFSET(4) NUMBITS(1) [],
        VRCG OFFSET(3) NUMBITS(1) [],
        VRO OFFSET(2) NUMBITS(1) [],
        RPT OFFSET(1) NUMBITS(1) [],
        CBT OFFSET(0) NUMBITS(1) [],
    ],
    pub Mr13W [
        FSPOP OFFSET(7) NUMBITS(1) [],
        FSPWR OFFSET(6) NUMBITS(1) [],
        DMD OFFSET(5) NUMBITS(1) [],
        RRO OFFSET(4) NUMBITS(1) [],
        VRCG OFFSET(3) NUMBITS(1) [],
        VRO OFFSET(2) NUMBITS(1) [],
        RPT OFFSET(1) NUMBITS(1) [],
        CBT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr14R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RSVD OFFSET(7) NUMBITS(1) [],
        VR_DQ OFFSET(6) NUMBITS(1) [],
        VREF_DQ OFFSET(0) NUMBITS(6) [],
    ],
    pub Mr14W [
        RSVD OFFSET(7) NUMBITS(1) [],
        VR_DQ OFFSET(6) NUMBITS(1) [],
        VREF_DQ OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mr22R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RSVD OFFSET(6) NUMBITS(2) [],
        ODTD_CA OFFSET(5) NUMBITS(1) [],
        ODTE_CS OFFSET(4) NUMBITS(1) [],
        ODTE_CK OFFSET(3) NUMBITS(1) [],
        CODT OFFSET(0) NUMBITS(3) [],
    ],
    pub Mr22W [
        RSVD OFFSET(6) NUMBITS(2) [],
        ODTD_CA OFFSET(5) NUMBITS(1) [],
        ODTE_CS OFFSET(4) NUMBITS(1) [],
        ODTE_CK OFFSET(3) NUMBITS(1) [],
        CODT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtcr0R [
        RFSHDT OFFSET(28) NUMBITS(4) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DTDRS OFFSET(24) NUMBITS(2) [],
        DTEXG OFFSET(23) NUMBITS(1) [],
        DTEXD OFFSET(22) NUMBITS(1) [],
        DTDSTP OFFSET(21) NUMBITS(1) [],
        DTDEN OFFSET(20) NUMBITS(1) [],
        DTDBS OFFSET(16) NUMBITS(4) [],
        DTRDBITR OFFSET(14) NUMBITS(2) [],
        RESERVED1 OFFSET(13) NUMBITS(1) [],
        DTWBDDM OFFSET(12) NUMBITS(1) [],
        RFSHEN OFFSET(8) NUMBITS(4) [],
        DTCMPD OFFSET(7) NUMBITS(1) [],
        DTMPR OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        INCWEYE OFFSET(4) NUMBITS(1) [],
        DTRPTN OFFSET(0) NUMBITS(4) [],
    ],
    pub Dtcr0W [
        RFSHDT OFFSET(28) NUMBITS(4) [],
        DTDRS OFFSET(24) NUMBITS(2) [],
        DTEXG OFFSET(23) NUMBITS(1) [],
        DTEXD OFFSET(22) NUMBITS(1) [],
        DTDSTP OFFSET(21) NUMBITS(1) [],
        DTDEN OFFSET(20) NUMBITS(1) [],
        DTDBS OFFSET(16) NUMBITS(4) [],
        DTRDBITR OFFSET(14) NUMBITS(2) [],
        DTWBDDM OFFSET(12) NUMBITS(1) [],
        RFSHEN OFFSET(8) NUMBITS(4) [],
        DTCMPD OFFSET(7) NUMBITS(1) [],
        DTMPR OFFSET(6) NUMBITS(1) [],
        INCWEYE OFFSET(4) NUMBITS(1) [],
        DTRPTN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtcr1R [
        RANKEN_RSVD OFFSET(18) NUMBITS(14) [],
        RANKEN OFFSET(16) NUMBITS(2) [],
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        DTRANK OFFSET(12) NUMBITS(2) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        RDLVLGDIFF OFFSET(8) NUMBITS(3) [],
        RESERVED2 OFFSET(7) NUMBITS(1) [],
        RDLVLGS OFFSET(4) NUMBITS(3) [],
        RESERVED3 OFFSET(3) NUMBITS(1) [],
        RDPRMVL_TRN OFFSET(2) NUMBITS(1) [],
        RDLVLEN OFFSET(1) NUMBITS(1) [],
        BSTEN OFFSET(0) NUMBITS(1) [],
    ],
    pub Dtcr1W [
        RANKEN OFFSET(16) NUMBITS(2) [],
        DTRANK OFFSET(12) NUMBITS(2) [],
        RDLVLGDIFF OFFSET(8) NUMBITS(3) [],
        RDLVLGS OFFSET(4) NUMBITS(3) [],
        RDPRMVL_TRN OFFSET(2) NUMBITS(1) [],
        RDLVLEN OFFSET(1) NUMBITS(1) [],
        BSTEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtar0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        MPRLOC OFFSET(28) NUMBITS(2) [],
        DTBGBK1 OFFSET(24) NUMBITS(4) [],
        DTBGBK0 OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(18) NUMBITS(2) [],
        DTROW OFFSET(0) NUMBITS(18) [],
    ],
    pub Dtar0W [
        MPRLOC OFFSET(28) NUMBITS(2) [],
        DTBGBK1 OFFSET(24) NUMBITS(4) [],
        DTBGBK0 OFFSET(20) NUMBITS(4) [],
        DTROW OFFSET(0) NUMBITS(18) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtar1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DTCOL1 OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        DTCOL0 OFFSET(0) NUMBITS(9) [],
    ],
    pub Dtar1W [
        DTCOL1 OFFSET(16) NUMBITS(9) [],
        DTCOL0 OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtar2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DTCOL3 OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        DTCOL2 OFFSET(0) NUMBITS(9) [],
    ],
    pub Dtar2W [
        DTCOL3 OFFSET(16) NUMBITS(9) [],
        DTCOL2 OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtdr0 [
        DTBYTE3 OFFSET(24) NUMBITS(8) [],
        DTBYTE2 OFFSET(16) NUMBITS(8) [],
        DTBYTE1 OFFSET(8) NUMBITS(8) [],
        DTBYTE0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtdr1 [
        DTBYTE7 OFFSET(24) NUMBITS(8) [],
        DTBYTE6 OFFSET(16) NUMBITS(8) [],
        DTBYTE5 OFFSET(8) NUMBITS(8) [],
        DTBYTE4 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtedr0 [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        WDQBMX OFFSET(24) NUMBITS(6) [],
        WDQBMN OFFSET(18) NUMBITS(6) [],
        RESERVED1 OFFSET(9) NUMBITS(9) [],
        RESERVED2 OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtedr1 [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDQSBMX OFFSET(24) NUMBITS(6) [],
        RDQSBMN OFFSET(18) NUMBITS(6) [],
        RDQSLMX OFFSET(9) NUMBITS(9) [],
        RDQSLMN OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtedr2 [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDQSNBMX OFFSET(24) NUMBITS(6) [],
        RDQSNBMN OFFSET(18) NUMBITS(6) [],
        RDQSNLMX OFFSET(9) NUMBITS(9) [],
        RDQSNLMN OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vtdr [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        HVREFMX OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        HVREFMN OFFSET(16) NUMBITS(7) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DVREFMX OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DVREFMN OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Catr0R [
        RESERVED0 OFFSET(21) NUMBITS(11) [],
        CACD OFFSET(16) NUMBITS(5) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        CAADR OFFSET(8) NUMBITS(5) [],
        CA1BYTE1 OFFSET(4) NUMBITS(4) [],
        CA1BYTE0 OFFSET(0) NUMBITS(4) [],
    ],
    pub Catr0W [
        CACD OFFSET(16) NUMBITS(5) [],
        CAADR OFFSET(8) NUMBITS(5) [],
        CA1BYTE1 OFFSET(4) NUMBITS(4) [],
        CA1BYTE0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Catr1R [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        CA0BYTE1 OFFSET(24) NUMBITS(4) [],
        CA0BYTE0 OFFSET(20) NUMBITS(4) [],
        CAMRZ OFFSET(16) NUMBITS(4) [],
        CACKEH OFFSET(12) NUMBITS(4) [],
        CACKEL OFFSET(8) NUMBITS(4) [],
        CAEXT OFFSET(4) NUMBITS(4) [],
        CAENT OFFSET(0) NUMBITS(4) [],
    ],
    pub Catr1W [
        CA0BYTE1 OFFSET(24) NUMBITS(4) [],
        CA0BYTE0 OFFSET(20) NUMBITS(4) [],
        CAMRZ OFFSET(16) NUMBITS(4) [],
        CACKEH OFFSET(12) NUMBITS(4) [],
        CACKEL OFFSET(8) NUMBITS(4) [],
        CAEXT OFFSET(4) NUMBITS(4) [],
        CAENT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dqsdr0R [
        DFTDLY OFFSET(28) NUMBITS(4) [],
        DFTZQUP OFFSET(27) NUMBITS(1) [],
        DFTDDLUP OFFSET(26) NUMBITS(1) [],
        RESERVED0 OFFSET(22) NUMBITS(4) [],
        DFTRDSPC OFFSET(20) NUMBITS(2) [],
        DFTB2BRD OFFSET(16) NUMBITS(4) [],
        DFTIDLRD OFFSET(12) NUMBITS(4) [],
        RESERVED1 OFFSET(8) NUMBITS(4) [],
        DFTGPULSE OFFSET(4) NUMBITS(4) [],
        DFTUPMODE OFFSET(2) NUMBITS(2) [],
        DFTDTMODE OFFSET(1) NUMBITS(1) [],
        DFTDTEN OFFSET(0) NUMBITS(1) [],
    ],
    pub Dqsdr0W [
        DFTDLY OFFSET(28) NUMBITS(4) [],
        DFTZQUP OFFSET(27) NUMBITS(1) [],
        DFTDDLUP OFFSET(26) NUMBITS(1) [],
        DFTRDSPC OFFSET(20) NUMBITS(2) [],
        DFTB2BRD OFFSET(16) NUMBITS(4) [],
        DFTIDLRD OFFSET(12) NUMBITS(4) [],
        DFTGPULSE OFFSET(4) NUMBITS(4) [],
        DFTUPMODE OFFSET(2) NUMBITS(2) [],
        DFTDTMODE OFFSET(1) NUMBITS(1) [],
        DFTDTEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dqsdr1 [
        DFTUPDACKF OFFSET(29) NUMBITS(3) [],
        DFTUPDACKC OFFSET(24) NUMBITS(5) [],
        DFTRDB2BF OFFSET(20) NUMBITS(4) [],
        DFTRDIDLF OFFSET(16) NUMBITS(4) [],
        DFTRDB2BC OFFSET(8) NUMBITS(8) [],
        DFTRDIDLC OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dqsdr2R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        DFTTHRSH OFFSET(16) NUMBITS(8) [],
        DFTMNTPRD OFFSET(0) NUMBITS(16) [],
    ],
    pub Dqsdr2W [
        DFTTHRSH OFFSET(16) NUMBITS(8) [],
        DFTMNTPRD OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dtedr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQLMX OFFSET(16) NUMBITS(11) [],
        RESERVED1 OFFSET(11) NUMBITS(5) [],
        WDQLMN OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcuarR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        CSADDR_R OFFSET(16) NUMBITS(4) [],
        CWADDR_R OFFSET(12) NUMBITS(4) [],
        ATYPE OFFSET(11) NUMBITS(1) [],
        INCA OFFSET(10) NUMBITS(1) [],
        CSEL OFFSET(8) NUMBITS(2) [],
        CSADDR_W OFFSET(4) NUMBITS(4) [],
        CWADDR_W OFFSET(0) NUMBITS(4) [],
    ],
    pub DcuarW [
        CSADDR_R OFFSET(16) NUMBITS(4) [],
        CWADDR_R OFFSET(12) NUMBITS(4) [],
        ATYPE OFFSET(11) NUMBITS(1) [],
        INCA OFFSET(10) NUMBITS(1) [],
        CSEL OFFSET(8) NUMBITS(2) [],
        CSADDR_W OFFSET(4) NUMBITS(4) [],
        CWADDR_W OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcurrR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        XCEN OFFSET(23) NUMBITS(1) [],
        RCEN OFFSET(22) NUMBITS(1) [],
        SCOF OFFSET(21) NUMBITS(1) [],
        SONF OFFSET(20) NUMBITS(1) [],
        NFAIL OFFSET(12) NUMBITS(8) [],
        EADDR OFFSET(8) NUMBITS(4) [],
        SADDR OFFSET(4) NUMBITS(4) [],
        DINST OFFSET(0) NUMBITS(4) [],
    ],
    pub DcurrW [
        XCEN OFFSET(23) NUMBITS(1) [],
        RCEN OFFSET(22) NUMBITS(1) [],
        SCOF OFFSET(21) NUMBITS(1) [],
        SONF OFFSET(20) NUMBITS(1) [],
        NFAIL OFFSET(12) NUMBITS(8) [],
        EADDR OFFSET(8) NUMBITS(4) [],
        SADDR OFFSET(4) NUMBITS(4) [],
        DINST OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DculrR [
        XLEADDR OFFSET(28) NUMBITS(4) [],
        RESERVED0 OFFSET(18) NUMBITS(10) [],
        IDA OFFSET(17) NUMBITS(1) [],
        LINF OFFSET(16) NUMBITS(1) [],
        LCNT OFFSET(8) NUMBITS(8) [],
        LEADDR OFFSET(4) NUMBITS(4) [],
        LSADDR OFFSET(0) NUMBITS(4) [],
    ],
    pub DculrW [
        XLEADDR OFFSET(28) NUMBITS(4) [],
        IDA OFFSET(17) NUMBITS(1) [],
        LINF OFFSET(16) NUMBITS(1) [],
        LCNT OFFSET(8) NUMBITS(8) [],
        LEADDR OFFSET(4) NUMBITS(4) [],
        LSADDR OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcugcrR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RCSW OFFSET(0) NUMBITS(16) [],
    ],
    pub DcugcrW [
        RCSW OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dcutpr [
        TDCUT2 OFFSET(16) NUMBITS(16) [],
        TDCUT1 OFFSET(8) NUMBITS(8) [],
        TDCUT0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dcusr0 [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        CFULL OFFSET(2) NUMBITS(1) [],
        CFAIL OFFSET(1) NUMBITS(1) [],
        RDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dcusr1 [
        LPCNT OFFSET(24) NUMBITS(8) [],
        FLCNT OFFSET(16) NUMBITS(8) [],
        RDCNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RankidrR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        RANKRID OFFSET(16) NUMBITS(4) [],
        RESERVED1 OFFSET(4) NUMBITS(12) [],
        RANKWID OFFSET(0) NUMBITS(4) [],
    ],
    pub RankidrW [
        RANKRID OFFSET(16) NUMBITS(4) [],
        RANKWID OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Riocr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        COEMODE_RSVD OFFSET(26) NUMBITS(4) [],
        COEMODE OFFSET(24) NUMBITS(2) [],
        CSOEMODE_RSVD OFFSET(4) NUMBITS(20) [],
        CSOEMODE OFFSET(0) NUMBITS(4) [],
    ],
    pub Riocr2W [
        COEMODE OFFSET(24) NUMBITS(2) [],
        CSOEMODE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Riocr4R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CKEOEMODE_RSVD OFFSET(4) NUMBITS(12) [],
        CKEOEMODE OFFSET(0) NUMBITS(4) [],
    ],
    pub Riocr4W [
        CKEOEMODE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Riocr5R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ODTOEMODE_RSVD OFFSET(4) NUMBITS(12) [],
        ODTOEMODE OFFSET(0) NUMBITS(4) [],
    ],
    pub Riocr5W [
        ODTOEMODE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aciocr0R [
        ACSR OFFSET(30) NUMBITS(2) [],
        RSTIOM OFFSET(29) NUMBITS(1) [],
        RSTPDR OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(27) NUMBITS(1) [],
        RSTODT OFFSET(26) NUMBITS(1) [],
        RESERVED1 OFFSET(10) NUMBITS(16) [],
        CKDCC OFFSET(6) NUMBITS(4) [],
        ACPDRMODE OFFSET(4) NUMBITS(2) [],
        ACODTMODE OFFSET(2) NUMBITS(2) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        ACRANKCLKSEL OFFSET(0) NUMBITS(1) [],
    ],
    pub Aciocr0W [
        ACSR OFFSET(30) NUMBITS(2) [],
        RSTIOM OFFSET(29) NUMBITS(1) [],
        RSTPDR OFFSET(28) NUMBITS(1) [],
        RSTODT OFFSET(26) NUMBITS(1) [],
        CKDCC OFFSET(6) NUMBITS(4) [],
        ACPDRMODE OFFSET(4) NUMBITS(2) [],
        ACODTMODE OFFSET(2) NUMBITS(2) [],
        ACRANKCLKSEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aciocr2 [
        CLKGENCLKGATE OFFSET(31) NUMBITS(1) [],
        ACOECLKGATE0 OFFSET(30) NUMBITS(1) [],
        ACPDRCLKGATE0 OFFSET(29) NUMBITS(1) [],
        ACTECLKGATE0 OFFSET(28) NUMBITS(1) [],
        CKNCLKGATE0 OFFSET(26) NUMBITS(2) [],
        CKCLKGATE0 OFFSET(24) NUMBITS(2) [],
        ACCLKGATE0 OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aciocr3R [
        PAROEMODE OFFSET(30) NUMBITS(2) [],
        BGOEMODE OFFSET(26) NUMBITS(4) [],
        BAOEMODE OFFSET(22) NUMBITS(4) [],
        A17OEMODE OFFSET(20) NUMBITS(2) [],
        A16OEMODE OFFSET(18) NUMBITS(2) [],
        ACTOEMODE OFFSET(16) NUMBITS(2) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        CKOEMODE_RSVD OFFSET(4) NUMBITS(4) [],
        CKOEMODE OFFSET(0) NUMBITS(4) [],
    ],
    pub Aciocr3W [
        PAROEMODE OFFSET(30) NUMBITS(2) [],
        BGOEMODE OFFSET(26) NUMBITS(4) [],
        BAOEMODE OFFSET(22) NUMBITS(4) [],
        A17OEMODE OFFSET(20) NUMBITS(2) [],
        A16OEMODE OFFSET(18) NUMBITS(2) [],
        ACTOEMODE OFFSET(16) NUMBITS(2) [],
        CKOEMODE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aciocr4 [
        LBCLKGATE OFFSET(31) NUMBITS(1) [],
        ACOECLKGATE1 OFFSET(30) NUMBITS(1) [],
        ACPDRCLKGATE1 OFFSET(29) NUMBITS(1) [],
        ACTECLKGATE1 OFFSET(28) NUMBITS(1) [],
        CKNCLKGATE1 OFFSET(26) NUMBITS(2) [],
        CKCLKGATE1 OFFSET(24) NUMBITS(2) [],
        ACCLKGATE1 OFFSET(0) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Aciocr5R [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(25) NUMBITS(3) [],
        ACXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(11) NUMBITS(11) [],
        RESERVED3 OFFSET(0) NUMBITS(11) [],
    ],
    pub Aciocr5W [
        RESERVED0 OFFSET(25) NUMBITS(3) [],
        ACXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED1 OFFSET(11) NUMBITS(11) [],
        RESERVED2 OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Iovcr0R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        ACREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        ACREFSEN OFFSET(25) NUMBITS(1) [],
        ACREFIEN OFFSET(24) NUMBITS(1) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        ACREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        ACREFSSEL OFFSET(8) NUMBITS(7) [],
        ACVREFISELRANGE OFFSET(7) NUMBITS(1) [],
        ACVREFISEL OFFSET(0) NUMBITS(7) [],
    ],
    pub Iovcr0W [
        ACREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        ACREFSEN OFFSET(25) NUMBITS(1) [],
        ACREFIEN OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        ACREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        ACREFSSEL OFFSET(8) NUMBITS(7) [],
        ACVREFISELRANGE OFFSET(7) NUMBITS(1) [],
        ACVREFISEL OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vtcr0R [
        TVREF OFFSET(29) NUMBITS(3) [],
        DVEN OFFSET(28) NUMBITS(1) [],
        PDAEN OFFSET(27) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(1) [],
        VWCR OFFSET(22) NUMBITS(4) [],
        DVSS OFFSET(18) NUMBITS(4) [],
        DVMAX OFFSET(12) NUMBITS(6) [],
        DVMIN OFFSET(6) NUMBITS(6) [],
        DVINIT OFFSET(0) NUMBITS(6) [],
    ],
    pub Vtcr0W [
        TVREF OFFSET(29) NUMBITS(3) [],
        DVEN OFFSET(28) NUMBITS(1) [],
        PDAEN OFFSET(27) NUMBITS(1) [],
        VWCR OFFSET(22) NUMBITS(4) [],
        DVSS OFFSET(18) NUMBITS(4) [],
        DVMAX OFFSET(12) NUMBITS(6) [],
        DVMIN OFFSET(6) NUMBITS(6) [],
        DVINIT OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Vtcr1R [
        HVSS OFFSET(28) NUMBITS(4) [],
        RESERVED0 OFFSET(27) NUMBITS(1) [],
        HVMAX OFFSET(20) NUMBITS(7) [],
        RESERVED1 OFFSET(19) NUMBITS(1) [],
        HVMIN OFFSET(12) NUMBITS(7) [],
        RESERVED2 OFFSET(11) NUMBITS(1) [],
        SHRNK OFFSET(9) NUMBITS(2) [],
        SHREN OFFSET(8) NUMBITS(1) [],
        TVREFIO OFFSET(5) NUMBITS(3) [],
        RESERVED3 OFFSET(3) NUMBITS(2) [],
        RESERVED4 OFFSET(2) NUMBITS(1) [],
        HVEN OFFSET(1) NUMBITS(1) [],
        HVIO OFFSET(0) NUMBITS(1) [],
    ],
    pub Vtcr1W [
        HVSS OFFSET(28) NUMBITS(4) [],
        HVMAX OFFSET(20) NUMBITS(7) [],
        HVMIN OFFSET(12) NUMBITS(7) [],
        SHRNK OFFSET(9) NUMBITS(2) [],
        SHREN OFFSET(8) NUMBITS(1) [],
        TVREFIO OFFSET(5) NUMBITS(3) [],
        RESERVED0 OFFSET(3) NUMBITS(2) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        HVEN OFFSET(1) NUMBITS(1) [],
        HVIO OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        CK1BD OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        CK0BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr0W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        CK1BD OFFSET(8) NUMBITS(6) [],
        CK0BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        PARBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        A16BD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        A17BD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        ACTBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr1W [
        PARBD OFFSET(24) NUMBITS(6) [],
        A16BD OFFSET(16) NUMBITS(6) [],
        A17BD OFFSET(8) NUMBITS(6) [],
        ACTBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        BG1BD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        BG0BD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        BA1BD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        BA0BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr2W [
        BG1BD OFFSET(24) NUMBITS(6) [],
        BG0BD OFFSET(16) NUMBITS(6) [],
        BA1BD OFFSET(8) NUMBITS(6) [],
        BA0BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        CS1BD OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        CS0BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr3W [
        CS1BD OFFSET(8) NUMBITS(6) [],
        CS0BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        ODT1BD OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        ODT0BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr4W [
        ODT1BD OFFSET(8) NUMBITS(6) [],
        ODT0BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr5R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        CKE1BD OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        CKE0BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr5W [
        CKE1BD OFFSET(8) NUMBITS(6) [],
        CKE0BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        A03BD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        A02BD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        A01BD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        A00BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr6W [
        A03BD OFFSET(24) NUMBITS(6) [],
        A02BD OFFSET(16) NUMBITS(6) [],
        A01BD OFFSET(8) NUMBITS(6) [],
        A00BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr7R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        A07BD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        A06BD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        A05BD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        A04BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr7W [
        A07BD OFFSET(24) NUMBITS(6) [],
        A06BD OFFSET(16) NUMBITS(6) [],
        A05BD OFFSET(8) NUMBITS(6) [],
        A04BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr8R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        A11BD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        A10BD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        A09BD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        A08BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr8W [
        A11BD OFFSET(24) NUMBITS(6) [],
        A10BD OFFSET(16) NUMBITS(6) [],
        A09BD OFFSET(8) NUMBITS(6) [],
        A08BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr9R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        A15BD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        A14BD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        A13BD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        A12BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr9W [
        A15BD OFFSET(24) NUMBITS(6) [],
        A14BD OFFSET(16) NUMBITS(6) [],
        A13BD OFFSET(8) NUMBITS(6) [],
        A12BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr15R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        OEBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        TEBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(6) NUMBITS(2) [],
        PDRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr15W [
        OEBD OFFSET(16) NUMBITS(6) [],
        TEBD OFFSET(8) NUMBITS(6) [],
        PDRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acbdlr16R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        CKN1BD OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        CKN0BD OFFSET(0) NUMBITS(6) [],
    ],
    pub Acbdlr16W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        CKN1BD OFFSET(8) NUMBITS(6) [],
        CKN0BD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AclcdlrR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        ACD1 OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        ACD OFFSET(0) NUMBITS(9) [],
    ],
    pub AclcdlrW [
        ACD1 OFFSET(16) NUMBITS(9) [],
        ACD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acmdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Acmdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Acmdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        MDLD1 OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Acmdlr1W [
        MDLD1 OFFSET(16) NUMBITS(9) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ZqcrR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        ZQREFISELRANGE OFFSET(25) NUMBITS(1) [],
        PGWAIT_FRQB OFFSET(19) NUMBITS(6) [],
        PGWAIT_FRQA OFFSET(13) NUMBITS(6) [],
        ZQREFPEN OFFSET(12) NUMBITS(1) [],
        ZQREFIEN OFFSET(11) NUMBITS(1) [],
        ODT_MODE OFFSET(9) NUMBITS(2) [],
        FORCE_ZCAL_VT_UPDATE OFFSET(8) NUMBITS(1) [],
        IODLMT OFFSET(5) NUMBITS(3) [],
        AVGEN OFFSET(4) NUMBITS(1) [],
        AVGMAX OFFSET(2) NUMBITS(2) [],
        ZCALT OFFSET(1) NUMBITS(1) [],
        ZQPD OFFSET(0) NUMBITS(1) [],
    ],
    pub ZqcrW [
        ZQREFISELRANGE OFFSET(25) NUMBITS(1) [],
        PGWAIT_FRQB OFFSET(19) NUMBITS(6) [],
        PGWAIT_FRQA OFFSET(13) NUMBITS(6) [],
        ZQREFPEN OFFSET(12) NUMBITS(1) [],
        ZQREFIEN OFFSET(11) NUMBITS(1) [],
        ODT_MODE OFFSET(9) NUMBITS(2) [],
        FORCE_ZCAL_VT_UPDATE OFFSET(8) NUMBITS(1) [],
        IODLMT OFFSET(5) NUMBITS(3) [],
        AVGEN OFFSET(4) NUMBITS(1) [],
        AVGMAX OFFSET(2) NUMBITS(2) [],
        ZCALT OFFSET(1) NUMBITS(1) [],
        ZQPD OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq0pr0 [
        PD_DRV_ZDEN OFFSET(31) NUMBITS(1) [],
        PU_DRV_ZDEN OFFSET(30) NUMBITS(1) [],
        PD_ODT_ZDEN OFFSET(29) NUMBITS(1) [],
        PU_ODT_ZDEN OFFSET(28) NUMBITS(1) [],
        ZSEGBYP OFFSET(27) NUMBITS(1) [],
        ZLE_MODE OFFSET(25) NUMBITS(2) [],
        ODT_ADJUST OFFSET(22) NUMBITS(3) [],
        PD_DRV_ADJUST OFFSET(19) NUMBITS(3) [],
        PU_DRV_ADJUST OFFSET(16) NUMBITS(3) [],
        ZPROG_DRAM_ODT OFFSET(12) NUMBITS(4) [],
        ZPROG_HOST_ODT OFFSET(8) NUMBITS(4) [],
        ZPROG_ASYM_DRV_PD OFFSET(4) NUMBITS(4) [],
        ZPROG_ASYM_DRV_PU OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq0pr1R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        PU_REFSEL OFFSET(8) NUMBITS(7) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        PD_REFSEL OFFSET(0) NUMBITS(7) [],
    ],
    pub Zq0pr1W [
        PU_REFSEL OFFSET(8) NUMBITS(7) [],
        PD_REFSEL OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq0dr0 [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        ZDATA_PU_DRV_RESULT OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ZDATA_PD_DRV_RESULT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq0dr1 [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        ZDATA_PU_ODT_RESULT OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ZDATA_PD_ODT_RESULT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq0or0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        ZDATA_PU_DRV_OVRD OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ZDATA_PD_DRV_OVRD OFFSET(0) NUMBITS(10) [],
    ],
    pub Zq0or0W [
        ZDATA_PU_DRV_OVRD OFFSET(16) NUMBITS(10) [],
        ZDATA_PD_DRV_OVRD OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq0or1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        ZDATA_PU_ODT_OVRD OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ZDATA_PD_ODT_OVRD OFFSET(0) NUMBITS(10) [],
    ],
    pub Zq0or1W [
        ZDATA_PU_ODT_OVRD OFFSET(16) NUMBITS(10) [],
        ZDATA_PD_ODT_OVRD OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq0sr [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        PD_ODT_SAT OFFSET(13) NUMBITS(1) [],
        PU_ODT_SAT OFFSET(12) NUMBITS(1) [],
        PD_DRV_SAT OFFSET(11) NUMBITS(1) [],
        PU_DRV_SAT OFFSET(10) NUMBITS(1) [],
        ZDONE OFFSET(9) NUMBITS(1) [],
        ZERR OFFSET(8) NUMBITS(1) [],
        OPU OFFSET(6) NUMBITS(2) [],
        OPD OFFSET(4) NUMBITS(2) [],
        ZPU OFFSET(2) NUMBITS(2) [],
        ZPD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq1pr0 [
        PD_DRV_ZDEN OFFSET(31) NUMBITS(1) [],
        PU_DRV_ZDEN OFFSET(30) NUMBITS(1) [],
        PD_ODT_ZDEN OFFSET(29) NUMBITS(1) [],
        PU_ODT_ZDEN OFFSET(28) NUMBITS(1) [],
        ZSEGBYP OFFSET(27) NUMBITS(1) [],
        ZLE_MODE OFFSET(25) NUMBITS(2) [],
        ODT_ADJUST OFFSET(22) NUMBITS(3) [],
        PD_DRV_ADJUST OFFSET(19) NUMBITS(3) [],
        PU_DRV_ADJUST OFFSET(16) NUMBITS(3) [],
        ZPROG_DRAM_ODT OFFSET(12) NUMBITS(4) [],
        ZPROG_HOST_ODT OFFSET(8) NUMBITS(4) [],
        ZPROG_ASYM_DRV_PD OFFSET(4) NUMBITS(4) [],
        ZPROG_ASYM_DRV_PU OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq1pr1R [
        RESERVED0 OFFSET(15) NUMBITS(17) [],
        PU_REFSEL OFFSET(8) NUMBITS(7) [],
        RESERVED1 OFFSET(7) NUMBITS(1) [],
        PD_REFSEL OFFSET(0) NUMBITS(7) [],
    ],
    pub Zq1pr1W [
        PU_REFSEL OFFSET(8) NUMBITS(7) [],
        PD_REFSEL OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq1dr0 [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        ZDATA_PU_DRV_RESULT OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ZDATA_PD_DRV_RESULT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq1dr1 [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        ZDATA_PU_ODT_RESULT OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ZDATA_PD_ODT_RESULT OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq1or0R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        ZDATA_PU_DRV_OVRD OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ZDATA_PD_DRV_OVRD OFFSET(0) NUMBITS(10) [],
    ],
    pub Zq1or0W [
        ZDATA_PU_DRV_OVRD OFFSET(16) NUMBITS(10) [],
        ZDATA_PD_DRV_OVRD OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq1or1R [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        ZDATA_PU_ODT_OVRD OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ZDATA_PD_ODT_OVRD OFFSET(0) NUMBITS(10) [],
    ],
    pub Zq1or1W [
        ZDATA_PU_ODT_OVRD OFFSET(16) NUMBITS(10) [],
        ZDATA_PD_ODT_OVRD OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Zq1sr [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        PD_ODT_SAT OFFSET(13) NUMBITS(1) [],
        PU_ODT_SAT OFFSET(12) NUMBITS(1) [],
        PD_DRV_SAT OFFSET(11) NUMBITS(1) [],
        PU_DRV_SAT OFFSET(10) NUMBITS(1) [],
        ZDONE OFFSET(9) NUMBITS(1) [],
        ZERR OFFSET(8) NUMBITS(1) [],
        OPU OFFSET(6) NUMBITS(2) [],
        OPD OFFSET(4) NUMBITS(2) [],
        ZPU OFFSET(2) NUMBITS(2) [],
        ZPD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gcr0R [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(6) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx0gcr0W [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gcr1R [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx0gcr1W [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gcr2 [
        DXOEMODE OFFSET(16) NUMBITS(16) [],
        DXTEMODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gcr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx0gcr3W [
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gcr4R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(23) NUMBITS(1) [],
        RESERVED4 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx0gcr4W [
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gcr5R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(7) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(7) NUMBITS(1) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ],
    pub Dx0gcr5W [
        RESERVED0 OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(7) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gcr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx0gcr6W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0bdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx0bdlr0W [
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0bdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx0bdlr1W [
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0bdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSNWBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx0bdlr2W [
        DSNWBD OFFSET(24) NUMBITS(6) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0bdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx0bdlr3W [
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0bdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx0bdlr4W [
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0bdlr5R [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DMRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx0bdlr5W [
        DMRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0bdlr6R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        TERBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx0bdlr6W [
        TERBD OFFSET(16) NUMBITS(6) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0lcdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx0lcdlr0W [
        WLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0lcdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WDQD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx0lcdlr1W [
        WDQD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0lcdlr2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx0lcdlr2W [
        DQSGD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0lcdlr3R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx0lcdlr3W [
        RDQSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0lcdlr4R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSND OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx0lcdlr4W [
        RDQSND OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0lcdlr5R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx0lcdlr5W [
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0mdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx0mdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0mdlr1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx0mdlr1W [
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gtr0R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQSL OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        RESERVED3 OFFSET(8) NUMBITS(5) [],
        RESERVED4 OFFSET(5) NUMBITS(3) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ],
    pub Dx0gtr0W [
        WDQSL OFFSET(24) NUMBITS(3) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0rsr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RDLVLERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0rsr2 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAWN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0rsr3 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gsr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        WLDQ OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(4) [],
        GDQSPRD OFFSET(17) NUMBITS(9) [],
        DPLOCK OFFSET(16) NUMBITS(1) [],
        WLPRD OFFSET(7) NUMBITS(9) [],
        WLERR OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        WLCAL OFFSET(4) NUMBITS(1) [],
        GDQSCAL OFFSET(3) NUMBITS(1) [],
        RDQSNCAL OFFSET(2) NUMBITS(1) [],
        RDQSCAL OFFSET(1) NUMBITS(1) [],
        WDQCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gsr1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gsr2 [
        GSDQSPRD OFFSET(23) NUMBITS(9) [],
        GSDQSCAL OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        DQS2DQERR OFFSET(12) NUMBITS(4) [],
        ESTAT OFFSET(8) NUMBITS(4) [],
        WEWN OFFSET(7) NUMBITS(1) [],
        WEERR OFFSET(6) NUMBITS(1) [],
        REWN OFFSET(5) NUMBITS(1) [],
        REERR OFFSET(4) NUMBITS(1) [],
        WDWN OFFSET(3) NUMBITS(1) [],
        WDERR OFFSET(2) NUMBITS(1) [],
        RDWN OFFSET(1) NUMBITS(1) [],
        RDERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx0gsr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ESTAT OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(2) [],
        DVERR OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        HVERR OFFSET(8) NUMBITS(2) [],
        RESERVED3 OFFSET(2) NUMBITS(6) [],
        RESERVED4 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gcr0R [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(6) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx1gcr0W [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gcr1R [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx1gcr1W [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gcr2 [
        DXOEMODE OFFSET(16) NUMBITS(16) [],
        DXTEMODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gcr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx1gcr3W [
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gcr4R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(23) NUMBITS(1) [],
        RESERVED4 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx1gcr4W [
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gcr5R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(7) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(7) NUMBITS(1) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ],
    pub Dx1gcr5W [
        RESERVED0 OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(7) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gcr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx1gcr6W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1bdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx1bdlr0W [
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1bdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx1bdlr1W [
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1bdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSNWBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx1bdlr2W [
        DSNWBD OFFSET(24) NUMBITS(6) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1bdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx1bdlr3W [
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1bdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx1bdlr4W [
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1bdlr5R [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DMRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx1bdlr5W [
        DMRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1bdlr6R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        TERBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx1bdlr6W [
        TERBD OFFSET(16) NUMBITS(6) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1lcdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx1lcdlr0W [
        WLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1lcdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WDQD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx1lcdlr1W [
        WDQD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1lcdlr2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx1lcdlr2W [
        DQSGD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1lcdlr3R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx1lcdlr3W [
        RDQSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1lcdlr4R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSND OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx1lcdlr4W [
        RDQSND OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1lcdlr5R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx1lcdlr5W [
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1mdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx1mdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1mdlr1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx1mdlr1W [
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gtr0R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQSL OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        RESERVED3 OFFSET(8) NUMBITS(5) [],
        RESERVED4 OFFSET(5) NUMBITS(3) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ],
    pub Dx1gtr0W [
        WDQSL OFFSET(24) NUMBITS(3) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1rsr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RDLVLERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1rsr2 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAWN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1rsr3 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gsr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        WLDQ OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(4) [],
        GDQSPRD OFFSET(17) NUMBITS(9) [],
        RESERVED2 OFFSET(16) NUMBITS(1) [],
        WLPRD OFFSET(7) NUMBITS(9) [],
        WLERR OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        WLCAL OFFSET(4) NUMBITS(1) [],
        GDQSCAL OFFSET(3) NUMBITS(1) [],
        RDQSNCAL OFFSET(2) NUMBITS(1) [],
        RDQSCAL OFFSET(1) NUMBITS(1) [],
        WDQCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gsr1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gsr2 [
        GSDQSPRD OFFSET(23) NUMBITS(9) [],
        GSDQSCAL OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        DQS2DQERR OFFSET(12) NUMBITS(4) [],
        ESTAT OFFSET(8) NUMBITS(4) [],
        WEWN OFFSET(7) NUMBITS(1) [],
        WEERR OFFSET(6) NUMBITS(1) [],
        REWN OFFSET(5) NUMBITS(1) [],
        REERR OFFSET(4) NUMBITS(1) [],
        WDWN OFFSET(3) NUMBITS(1) [],
        WDERR OFFSET(2) NUMBITS(1) [],
        RDWN OFFSET(1) NUMBITS(1) [],
        RDERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx1gsr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ESTAT OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(2) [],
        DVERR OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        HVERR OFFSET(8) NUMBITS(2) [],
        RESERVED3 OFFSET(2) NUMBITS(6) [],
        RESERVED4 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gcr0R [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(6) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx2gcr0W [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gcr1R [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx2gcr1W [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gcr2 [
        DXOEMODE OFFSET(16) NUMBITS(16) [],
        DXTEMODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gcr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx2gcr3W [
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gcr4R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(23) NUMBITS(1) [],
        RESERVED4 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx2gcr4W [
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gcr5R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(7) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(7) NUMBITS(1) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ],
    pub Dx2gcr5W [
        RESERVED0 OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(7) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gcr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx2gcr6W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2bdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx2bdlr0W [
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2bdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx2bdlr1W [
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2bdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSNWBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx2bdlr2W [
        DSNWBD OFFSET(24) NUMBITS(6) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2bdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx2bdlr3W [
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2bdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx2bdlr4W [
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2bdlr5R [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DMRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx2bdlr5W [
        DMRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2bdlr6R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        TERBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx2bdlr6W [
        TERBD OFFSET(16) NUMBITS(6) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2lcdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx2lcdlr0W [
        WLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2lcdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WDQD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx2lcdlr1W [
        WDQD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2lcdlr2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx2lcdlr2W [
        DQSGD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2lcdlr3R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx2lcdlr3W [
        RDQSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2lcdlr4R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSND OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx2lcdlr4W [
        RDQSND OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2lcdlr5R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx2lcdlr5W [
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2mdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx2mdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2mdlr1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx2mdlr1W [
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gtr0R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQSL OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        RESERVED3 OFFSET(8) NUMBITS(5) [],
        RESERVED4 OFFSET(5) NUMBITS(3) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ],
    pub Dx2gtr0W [
        WDQSL OFFSET(24) NUMBITS(3) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2rsr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RDLVLERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2rsr2 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAWN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2rsr3 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gsr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        WLDQ OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(4) [],
        GDQSPRD OFFSET(17) NUMBITS(9) [],
        DPLOCK OFFSET(16) NUMBITS(1) [],
        WLPRD OFFSET(7) NUMBITS(9) [],
        WLERR OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        WLCAL OFFSET(4) NUMBITS(1) [],
        GDQSCAL OFFSET(3) NUMBITS(1) [],
        RDQSNCAL OFFSET(2) NUMBITS(1) [],
        RDQSCAL OFFSET(1) NUMBITS(1) [],
        WDQCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gsr1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gsr2 [
        GSDQSPRD OFFSET(23) NUMBITS(9) [],
        GSDQSCAL OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        DQS2DQERR OFFSET(12) NUMBITS(4) [],
        ESTAT OFFSET(8) NUMBITS(4) [],
        WEWN OFFSET(7) NUMBITS(1) [],
        WEERR OFFSET(6) NUMBITS(1) [],
        REWN OFFSET(5) NUMBITS(1) [],
        REERR OFFSET(4) NUMBITS(1) [],
        WDWN OFFSET(3) NUMBITS(1) [],
        WDERR OFFSET(2) NUMBITS(1) [],
        RDWN OFFSET(1) NUMBITS(1) [],
        RDERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx2gsr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ESTAT OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(2) [],
        DVERR OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        HVERR OFFSET(8) NUMBITS(2) [],
        RESERVED3 OFFSET(2) NUMBITS(6) [],
        RESERVED4 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gcr0R [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(6) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx3gcr0W [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gcr1R [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx3gcr1W [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gcr2 [
        DXOEMODE OFFSET(16) NUMBITS(16) [],
        DXTEMODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gcr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx3gcr3W [
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gcr4R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(23) NUMBITS(1) [],
        RESERVED4 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx3gcr4W [
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gcr5R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(7) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(7) NUMBITS(1) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ],
    pub Dx3gcr5W [
        RESERVED0 OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(7) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gcr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx3gcr6W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3bdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx3bdlr0W [
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3bdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx3bdlr1W [
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3bdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSNWBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx3bdlr2W [
        DSNWBD OFFSET(24) NUMBITS(6) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3bdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx3bdlr3W [
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3bdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx3bdlr4W [
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3bdlr5R [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DMRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx3bdlr5W [
        DMRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3bdlr6R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        TERBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx3bdlr6W [
        TERBD OFFSET(16) NUMBITS(6) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3lcdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx3lcdlr0W [
        WLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3lcdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WDQD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx3lcdlr1W [
        WDQD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3lcdlr2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx3lcdlr2W [
        DQSGD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3lcdlr3R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx3lcdlr3W [
        RDQSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3lcdlr4R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSND OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx3lcdlr4W [
        RDQSND OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3lcdlr5R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx3lcdlr5W [
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3mdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx3mdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3mdlr1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx3mdlr1W [
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gtr0R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQSL OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        RESERVED3 OFFSET(8) NUMBITS(5) [],
        RESERVED4 OFFSET(5) NUMBITS(3) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ],
    pub Dx3gtr0W [
        WDQSL OFFSET(24) NUMBITS(3) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3rsr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RDLVLERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3rsr2 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAWN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3rsr3 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gsr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        WLDQ OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(4) [],
        GDQSPRD OFFSET(17) NUMBITS(9) [],
        RESERVED2 OFFSET(16) NUMBITS(1) [],
        WLPRD OFFSET(7) NUMBITS(9) [],
        WLERR OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        WLCAL OFFSET(4) NUMBITS(1) [],
        GDQSCAL OFFSET(3) NUMBITS(1) [],
        RDQSNCAL OFFSET(2) NUMBITS(1) [],
        RDQSCAL OFFSET(1) NUMBITS(1) [],
        WDQCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gsr1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gsr2 [
        GSDQSPRD OFFSET(23) NUMBITS(9) [],
        GSDQSCAL OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        DQS2DQERR OFFSET(12) NUMBITS(4) [],
        ESTAT OFFSET(8) NUMBITS(4) [],
        WEWN OFFSET(7) NUMBITS(1) [],
        WEERR OFFSET(6) NUMBITS(1) [],
        REWN OFFSET(5) NUMBITS(1) [],
        REERR OFFSET(4) NUMBITS(1) [],
        WDWN OFFSET(3) NUMBITS(1) [],
        WDERR OFFSET(2) NUMBITS(1) [],
        RDWN OFFSET(1) NUMBITS(1) [],
        RDERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx3gsr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ESTAT OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(2) [],
        DVERR OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        HVERR OFFSET(8) NUMBITS(2) [],
        RESERVED3 OFFSET(2) NUMBITS(6) [],
        RESERVED4 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gcr0R [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(6) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx4gcr0W [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gcr1R [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx4gcr1W [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gcr2 [
        DXOEMODE OFFSET(16) NUMBITS(16) [],
        DXTEMODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gcr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx4gcr3W [
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gcr4R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(23) NUMBITS(1) [],
        RESERVED4 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx4gcr4W [
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gcr5R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(7) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(7) NUMBITS(1) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ],
    pub Dx4gcr5W [
        RESERVED0 OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(7) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gcr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx4gcr6W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4bdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx4bdlr0W [
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4bdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx4bdlr1W [
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4bdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSNWBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx4bdlr2W [
        DSNWBD OFFSET(24) NUMBITS(6) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4bdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx4bdlr3W [
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4bdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx4bdlr4W [
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4bdlr5R [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DMRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx4bdlr5W [
        DMRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4bdlr6R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        TERBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx4bdlr6W [
        TERBD OFFSET(16) NUMBITS(6) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4lcdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx4lcdlr0W [
        WLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4lcdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WDQD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx4lcdlr1W [
        WDQD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4lcdlr2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx4lcdlr2W [
        DQSGD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4lcdlr3R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx4lcdlr3W [
        RDQSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4lcdlr4R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSND OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx4lcdlr4W [
        RDQSND OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4lcdlr5R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx4lcdlr5W [
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4mdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx4mdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4mdlr1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx4mdlr1W [
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gtr0R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQSL OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        RESERVED3 OFFSET(8) NUMBITS(5) [],
        RESERVED4 OFFSET(5) NUMBITS(3) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ],
    pub Dx4gtr0W [
        WDQSL OFFSET(24) NUMBITS(3) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4rsr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RDLVLERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4rsr2 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAWN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4rsr3 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gsr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        WLDQ OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(4) [],
        GDQSPRD OFFSET(17) NUMBITS(9) [],
        DPLOCK OFFSET(16) NUMBITS(1) [],
        WLPRD OFFSET(7) NUMBITS(9) [],
        WLERR OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        WLCAL OFFSET(4) NUMBITS(1) [],
        GDQSCAL OFFSET(3) NUMBITS(1) [],
        RDQSNCAL OFFSET(2) NUMBITS(1) [],
        RDQSCAL OFFSET(1) NUMBITS(1) [],
        WDQCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gsr1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gsr2 [
        GSDQSPRD OFFSET(23) NUMBITS(9) [],
        GSDQSCAL OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        RESERVED2 OFFSET(12) NUMBITS(4) [],
        ESTAT OFFSET(8) NUMBITS(4) [],
        WEWN OFFSET(7) NUMBITS(1) [],
        WEERR OFFSET(6) NUMBITS(1) [],
        REWN OFFSET(5) NUMBITS(1) [],
        REERR OFFSET(4) NUMBITS(1) [],
        WDWN OFFSET(3) NUMBITS(1) [],
        WDERR OFFSET(2) NUMBITS(1) [],
        RDWN OFFSET(1) NUMBITS(1) [],
        RDERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx4gsr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ESTAT OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(2) [],
        DVERR OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        HVERR OFFSET(8) NUMBITS(2) [],
        RESERVED3 OFFSET(2) NUMBITS(6) [],
        RESERVED4 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gcr0R [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(6) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx5gcr0W [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gcr1R [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx5gcr1W [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gcr2 [
        DXOEMODE OFFSET(16) NUMBITS(16) [],
        DXTEMODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gcr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx5gcr3W [
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gcr4R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(23) NUMBITS(1) [],
        RESERVED4 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx5gcr4W [
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gcr5R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(7) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(7) NUMBITS(1) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ],
    pub Dx5gcr5W [
        RESERVED0 OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(7) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gcr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx5gcr6W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5bdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx5bdlr0W [
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5bdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx5bdlr1W [
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5bdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSNWBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx5bdlr2W [
        DSNWBD OFFSET(24) NUMBITS(6) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5bdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx5bdlr3W [
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5bdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx5bdlr4W [
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5bdlr5R [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DMRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx5bdlr5W [
        DMRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5bdlr6R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        TERBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx5bdlr6W [
        TERBD OFFSET(16) NUMBITS(6) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5lcdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx5lcdlr0W [
        WLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5lcdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WDQD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx5lcdlr1W [
        WDQD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5lcdlr2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx5lcdlr2W [
        DQSGD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5lcdlr3R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx5lcdlr3W [
        RDQSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5lcdlr4R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSND OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx5lcdlr4W [
        RDQSND OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5lcdlr5R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx5lcdlr5W [
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5mdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx5mdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5mdlr1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx5mdlr1W [
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gtr0R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQSL OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        RESERVED3 OFFSET(8) NUMBITS(5) [],
        RESERVED4 OFFSET(5) NUMBITS(3) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ],
    pub Dx5gtr0W [
        WDQSL OFFSET(24) NUMBITS(3) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5rsr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RDLVLERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5rsr2 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAWN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5rsr3 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gsr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        WLDQ OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(4) [],
        GDQSPRD OFFSET(17) NUMBITS(9) [],
        RESERVED2 OFFSET(16) NUMBITS(1) [],
        WLPRD OFFSET(7) NUMBITS(9) [],
        WLERR OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        WLCAL OFFSET(4) NUMBITS(1) [],
        GDQSCAL OFFSET(3) NUMBITS(1) [],
        RDQSNCAL OFFSET(2) NUMBITS(1) [],
        RDQSCAL OFFSET(1) NUMBITS(1) [],
        WDQCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gsr1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gsr2 [
        GSDQSPRD OFFSET(23) NUMBITS(9) [],
        GSDQSCAL OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        RESERVED2 OFFSET(12) NUMBITS(4) [],
        ESTAT OFFSET(8) NUMBITS(4) [],
        WEWN OFFSET(7) NUMBITS(1) [],
        WEERR OFFSET(6) NUMBITS(1) [],
        REWN OFFSET(5) NUMBITS(1) [],
        REERR OFFSET(4) NUMBITS(1) [],
        WDWN OFFSET(3) NUMBITS(1) [],
        WDERR OFFSET(2) NUMBITS(1) [],
        RDWN OFFSET(1) NUMBITS(1) [],
        RDERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx5gsr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ESTAT OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(2) [],
        DVERR OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        HVERR OFFSET(8) NUMBITS(2) [],
        RESERVED3 OFFSET(2) NUMBITS(6) [],
        RESERVED4 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gcr0R [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(6) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx6gcr0W [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gcr1R [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx6gcr1W [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gcr2 [
        DXOEMODE OFFSET(16) NUMBITS(16) [],
        DXTEMODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gcr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx6gcr3W [
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gcr4R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(23) NUMBITS(1) [],
        RESERVED4 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx6gcr4W [
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gcr5R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(7) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(7) NUMBITS(1) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ],
    pub Dx6gcr5W [
        RESERVED0 OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(7) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gcr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx6gcr6W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6bdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx6bdlr0W [
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6bdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx6bdlr1W [
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6bdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSNWBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx6bdlr2W [
        DSNWBD OFFSET(24) NUMBITS(6) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6bdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx6bdlr3W [
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6bdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx6bdlr4W [
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6bdlr5R [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DMRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx6bdlr5W [
        DMRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6bdlr6R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        TERBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx6bdlr6W [
        TERBD OFFSET(16) NUMBITS(6) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6lcdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx6lcdlr0W [
        WLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6lcdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WDQD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx6lcdlr1W [
        WDQD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6lcdlr2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx6lcdlr2W [
        DQSGD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6lcdlr3R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx6lcdlr3W [
        RDQSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6lcdlr4R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSND OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx6lcdlr4W [
        RDQSND OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6lcdlr5R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx6lcdlr5W [
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6mdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx6mdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6mdlr1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx6mdlr1W [
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gtr0R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQSL OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        RESERVED3 OFFSET(8) NUMBITS(5) [],
        RESERVED4 OFFSET(5) NUMBITS(3) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ],
    pub Dx6gtr0W [
        WDQSL OFFSET(24) NUMBITS(3) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6rsr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RDLVLERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6rsr2 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAWN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6rsr3 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gsr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        WLDQ OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(4) [],
        GDQSPRD OFFSET(17) NUMBITS(9) [],
        DPLOCK OFFSET(16) NUMBITS(1) [],
        WLPRD OFFSET(7) NUMBITS(9) [],
        WLERR OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        WLCAL OFFSET(4) NUMBITS(1) [],
        GDQSCAL OFFSET(3) NUMBITS(1) [],
        RDQSNCAL OFFSET(2) NUMBITS(1) [],
        RDQSCAL OFFSET(1) NUMBITS(1) [],
        WDQCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gsr1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gsr2 [
        GSDQSPRD OFFSET(23) NUMBITS(9) [],
        GSDQSCAL OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        RESERVED2 OFFSET(12) NUMBITS(4) [],
        ESTAT OFFSET(8) NUMBITS(4) [],
        WEWN OFFSET(7) NUMBITS(1) [],
        WEERR OFFSET(6) NUMBITS(1) [],
        REWN OFFSET(5) NUMBITS(1) [],
        REERR OFFSET(4) NUMBITS(1) [],
        WDWN OFFSET(3) NUMBITS(1) [],
        WDERR OFFSET(2) NUMBITS(1) [],
        RDWN OFFSET(1) NUMBITS(1) [],
        RDERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx6gsr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ESTAT OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(2) [],
        DVERR OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        HVERR OFFSET(8) NUMBITS(2) [],
        RESERVED3 OFFSET(2) NUMBITS(6) [],
        RESERVED4 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gcr0R [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(6) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx7gcr0W [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gcr1R [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx7gcr1W [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gcr2 [
        DXOEMODE OFFSET(16) NUMBITS(16) [],
        DXTEMODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gcr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx7gcr3W [
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gcr4R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(23) NUMBITS(1) [],
        RESERVED4 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx7gcr4W [
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gcr5R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(7) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(7) NUMBITS(1) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ],
    pub Dx7gcr5W [
        RESERVED0 OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(7) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gcr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx7gcr6W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7bdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx7bdlr0W [
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7bdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx7bdlr1W [
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7bdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSNWBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx7bdlr2W [
        DSNWBD OFFSET(24) NUMBITS(6) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7bdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx7bdlr3W [
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7bdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx7bdlr4W [
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7bdlr5R [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DMRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx7bdlr5W [
        DMRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7bdlr6R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        TERBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx7bdlr6W [
        TERBD OFFSET(16) NUMBITS(6) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7lcdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx7lcdlr0W [
        WLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7lcdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WDQD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx7lcdlr1W [
        WDQD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7lcdlr2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx7lcdlr2W [
        DQSGD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7lcdlr3R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx7lcdlr3W [
        RDQSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7lcdlr4R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSND OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx7lcdlr4W [
        RDQSND OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7lcdlr5R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx7lcdlr5W [
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7mdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx7mdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7mdlr1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx7mdlr1W [
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gtr0R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQSL OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        RESERVED3 OFFSET(8) NUMBITS(5) [],
        RESERVED4 OFFSET(5) NUMBITS(3) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ],
    pub Dx7gtr0W [
        WDQSL OFFSET(24) NUMBITS(3) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7rsr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RDLVLERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7rsr2 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAWN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7rsr3 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gsr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        WLDQ OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(4) [],
        GDQSPRD OFFSET(17) NUMBITS(9) [],
        RESERVED2 OFFSET(16) NUMBITS(1) [],
        WLPRD OFFSET(7) NUMBITS(9) [],
        WLERR OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        WLCAL OFFSET(4) NUMBITS(1) [],
        GDQSCAL OFFSET(3) NUMBITS(1) [],
        RDQSNCAL OFFSET(2) NUMBITS(1) [],
        RDQSCAL OFFSET(1) NUMBITS(1) [],
        WDQCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gsr1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gsr2 [
        GSDQSPRD OFFSET(23) NUMBITS(9) [],
        GSDQSCAL OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        RESERVED2 OFFSET(12) NUMBITS(4) [],
        ESTAT OFFSET(8) NUMBITS(4) [],
        WEWN OFFSET(7) NUMBITS(1) [],
        WEERR OFFSET(6) NUMBITS(1) [],
        REWN OFFSET(5) NUMBITS(1) [],
        REERR OFFSET(4) NUMBITS(1) [],
        WDWN OFFSET(3) NUMBITS(1) [],
        WDERR OFFSET(2) NUMBITS(1) [],
        RDWN OFFSET(1) NUMBITS(1) [],
        RDERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx7gsr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ESTAT OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(2) [],
        DVERR OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        HVERR OFFSET(8) NUMBITS(2) [],
        RESERVED3 OFFSET(2) NUMBITS(6) [],
        RESERVED4 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gcr0R [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        RESERVED1 OFFSET(14) NUMBITS(6) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx8gcr0W [
        CALBYP OFFSET(31) NUMBITS(1) [],
        MDLEN OFFSET(30) NUMBITS(1) [],
        CODTSHFT OFFSET(28) NUMBITS(2) [],
        RESERVED0 OFFSET(24) NUMBITS(4) [],
        RDDLY OFFSET(20) NUMBITS(4) [],
        DQSNSEPDR OFFSET(13) NUMBITS(1) [],
        DQSSEPDR OFFSET(12) NUMBITS(1) [],
        RTTOAL OFFSET(11) NUMBITS(1) [],
        RTTOH OFFSET(9) NUMBITS(2) [],
        CPDRSHFT OFFSET(7) NUMBITS(2) [],
        DQSRPD OFFSET(6) NUMBITS(1) [],
        DQSGPDR OFFSET(5) NUMBITS(1) [],
        DQSGODT OFFSET(3) NUMBITS(1) [],
        DQSGOE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gcr1R [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(15) NUMBITS(1) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx8gcr1W [
        DXPDRMODE OFFSET(16) NUMBITS(16) [],
        QSNSEL OFFSET(14) NUMBITS(1) [],
        QSSEL OFFSET(13) NUMBITS(1) [],
        OEEN OFFSET(12) NUMBITS(1) [],
        PDREN OFFSET(11) NUMBITS(1) [],
        TEEN OFFSET(10) NUMBITS(1) [],
        DSEN OFFSET(9) NUMBITS(1) [],
        DMEN OFFSET(8) NUMBITS(1) [],
        DQEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gcr2 [
        DXOEMODE OFFSET(16) NUMBITS(16) [],
        DXTEMODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gcr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx8gcr3W [
        RDBVT OFFSET(29) NUMBITS(1) [],
        WDBVT OFFSET(28) NUMBITS(1) [],
        RGLVT OFFSET(27) NUMBITS(1) [],
        RDLVT OFFSET(26) NUMBITS(1) [],
        WDLVT OFFSET(25) NUMBITS(1) [],
        WLLVT OFFSET(24) NUMBITS(1) [],
        DSNOEMODE OFFSET(20) NUMBITS(2) [],
        DSNTEMODE OFFSET(18) NUMBITS(2) [],
        DSNPDRMODE OFFSET(16) NUMBITS(2) [],
        DMOEMODE OFFSET(14) NUMBITS(2) [],
        DMTEMODE OFFSET(12) NUMBITS(2) [],
        DMPDRMODE OFFSET(10) NUMBITS(2) [],
        DSOEMODE OFFSET(6) NUMBITS(2) [],
        DSTEMODE OFFSET(4) NUMBITS(2) [],
        DSPDRMODE OFFSET(2) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gcr4R [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED2 OFFSET(24) NUMBITS(1) [],
        RESERVED3 OFFSET(23) NUMBITS(1) [],
        RESERVED4 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx8gcr4W [
        DXREFPEN OFFSET(28) NUMBITS(1) [],
        RESERVED0 OFFSET(26) NUMBITS(2) [],
        DXREFSEN OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RESERVED2 OFFSET(16) NUMBITS(7) [],
        DXREFSSELRANGE OFFSET(15) NUMBITS(1) [],
        DXREFSSEL OFFSET(8) NUMBITS(7) [],
        DXREFIEN OFFSET(2) NUMBITS(4) [],
        DXREFIMON OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gcr5R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(24) NUMBITS(7) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        RESERVED3 OFFSET(16) NUMBITS(7) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        RESERVED5 OFFSET(7) NUMBITS(1) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ],
    pub Dx8gcr5W [
        RESERVED0 OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(7) [],
        DXREFISELR1 OFFSET(8) NUMBITS(7) [],
        DXREFISELR0 OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gcr6R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        RESERVED1 OFFSET(24) NUMBITS(6) [],
        RESERVED2 OFFSET(22) NUMBITS(2) [],
        RESERVED3 OFFSET(16) NUMBITS(6) [],
        RESERVED4 OFFSET(14) NUMBITS(2) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        RESERVED5 OFFSET(6) NUMBITS(2) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx8gcr6W [
        RESERVED0 OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(16) NUMBITS(6) [],
        DXDQVREFR1 OFFSET(8) NUMBITS(6) [],
        DXDQVREFR0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8bdlr0R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx8bdlr0W [
        DQ3WBD OFFSET(24) NUMBITS(6) [],
        DQ2WBD OFFSET(16) NUMBITS(6) [],
        DQ1WBD OFFSET(8) NUMBITS(6) [],
        DQ0WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8bdlr1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx8bdlr1W [
        DQ7WBD OFFSET(24) NUMBITS(6) [],
        DQ6WBD OFFSET(16) NUMBITS(6) [],
        DQ5WBD OFFSET(8) NUMBITS(6) [],
        DQ4WBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8bdlr2R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DSNWBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx8bdlr2W [
        DSNWBD OFFSET(24) NUMBITS(6) [],
        DSOEBD OFFSET(16) NUMBITS(6) [],
        DSWBD OFFSET(8) NUMBITS(6) [],
        DMWBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8bdlr3R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx8bdlr3W [
        DQ3RBD OFFSET(24) NUMBITS(6) [],
        DQ2RBD OFFSET(16) NUMBITS(6) [],
        DQ1RBD OFFSET(8) NUMBITS(6) [],
        DQ0RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8bdlr4R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        RESERVED2 OFFSET(14) NUMBITS(2) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx8bdlr4W [
        DQ7RBD OFFSET(24) NUMBITS(6) [],
        DQ6RBD OFFSET(16) NUMBITS(6) [],
        DQ5RBD OFFSET(8) NUMBITS(6) [],
        DQ4RBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8bdlr5R [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DMRBD OFFSET(0) NUMBITS(6) [],
    ],
    pub Dx8bdlr5W [
        DMRBD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8bdlr6R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        TERBD OFFSET(16) NUMBITS(6) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx8bdlr6W [
        TERBD OFFSET(16) NUMBITS(6) [],
        PDRBD OFFSET(8) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8lcdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx8lcdlr0W [
        WLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8lcdlr1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        WDQD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx8lcdlr1W [
        WDQD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8lcdlr2R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx8lcdlr2W [
        DQSGD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8lcdlr3R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx8lcdlr3W [
        RDQSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8lcdlr4R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        RDQSND OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx8lcdlr4W [
        RDQSND OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8lcdlr5R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RESERVED2 OFFSET(9) NUMBITS(7) [],
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx8lcdlr5W [
        DQSGSD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8mdlr0R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        TPRD OFFSET(16) NUMBITS(9) [],
        RESERVED1 OFFSET(9) NUMBITS(7) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx8mdlr0W [
        TPRD OFFSET(16) NUMBITS(9) [],
        IPRD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8mdlr1R [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        MDLD OFFSET(0) NUMBITS(9) [],
    ],
    pub Dx8mdlr1W [
        MDLD OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gtr0R [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        WDQSL OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        RESERVED2 OFFSET(13) NUMBITS(3) [],
        RESERVED3 OFFSET(8) NUMBITS(5) [],
        RESERVED4 OFFSET(5) NUMBITS(3) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ],
    pub Dx8gtr0W [
        WDQSL OFFSET(24) NUMBITS(3) [],
        WLSL OFFSET(16) NUMBITS(4) [],
        DGSL OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8rsr1 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RDLVLERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8rsr2 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAWN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8rsr3 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        WLAERR OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gsr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        WLDQ OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(4) [],
        GDQSPRD OFFSET(17) NUMBITS(9) [],
        DPLOCK OFFSET(16) NUMBITS(1) [],
        WLPRD OFFSET(7) NUMBITS(9) [],
        WLERR OFFSET(6) NUMBITS(1) [],
        WLDONE OFFSET(5) NUMBITS(1) [],
        WLCAL OFFSET(4) NUMBITS(1) [],
        GDQSCAL OFFSET(3) NUMBITS(1) [],
        RDQSNCAL OFFSET(2) NUMBITS(1) [],
        RDQSCAL OFFSET(1) NUMBITS(1) [],
        WDQCAL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gsr1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DLTCODE OFFSET(1) NUMBITS(24) [],
        DLTDONE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gsr2 [
        GSDQSPRD OFFSET(23) NUMBITS(9) [],
        GSDQSCAL OFFSET(22) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(1) [],
        DQS2DQERR OFFSET(12) NUMBITS(4) [],
        ESTAT OFFSET(8) NUMBITS(4) [],
        WEWN OFFSET(7) NUMBITS(1) [],
        WEERR OFFSET(6) NUMBITS(1) [],
        REWN OFFSET(5) NUMBITS(1) [],
        REERR OFFSET(4) NUMBITS(1) [],
        WDWN OFFSET(3) NUMBITS(1) [],
        WDERR OFFSET(2) NUMBITS(1) [],
        RDWN OFFSET(1) NUMBITS(1) [],
        RDERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8gsr3 [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        ESTAT OFFSET(24) NUMBITS(3) [],
        RESERVED1 OFFSET(20) NUMBITS(2) [],
        DVERR OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(12) NUMBITS(2) [],
        HVERR OFFSET(8) NUMBITS(2) [],
        RESERVED3 OFFSET(2) NUMBITS(6) [],
        RESERVED4 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl0oscR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED2 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl0oscW [
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED0 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED1 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl0pllcr0R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(4) [],
        RESERVED4 OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl0pllcr0W [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        RESERVED3 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl0pllcr1R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl0pllcr1W [
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl0pllcr5R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx8sl0pllcr5W [
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl0dqsctlR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RRRMODE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        RESERVED2 OFFSET(15) NUMBITS(2) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        RESERVED3 OFFSET(10) NUMBITS(3) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl0dqsctlW [
        RRRMODE OFFSET(24) NUMBITS(1) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl0ddlctlR [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(2) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx8sl0ddlctlW [
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl0dxctl1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(16) [],
    ],
    pub Dx8sl0dxctl1W [
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl0dxctl2R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        RESERVED1 OFFSET(17) NUMBITS(1) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(2) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl0dxctl2W [
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl0iocrR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED1 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(11) NUMBITS(11) [],
        RESERVED3 OFFSET(0) NUMBITS(11) [],
    ],
    pub Dx8sl0iocrW [
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED0 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED1 OFFSET(11) NUMBITS(11) [],
        RESERVED2 OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl1oscR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED2 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl1oscW [
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED0 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED1 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl1pllcr0R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(4) [],
        RESERVED4 OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl1pllcr0W [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        RESERVED3 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl1pllcr1R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl1pllcr1W [
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl1pllcr5R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx8sl1pllcr5W [
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl1dqsctlR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RRRMODE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        RESERVED2 OFFSET(15) NUMBITS(2) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        RESERVED3 OFFSET(10) NUMBITS(3) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl1dqsctlW [
        RRRMODE OFFSET(24) NUMBITS(1) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl1ddlctlR [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(2) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx8sl1ddlctlW [
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl1dxctl1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(16) [],
    ],
    pub Dx8sl1dxctl1W [
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl1dxctl2R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        RESERVED1 OFFSET(17) NUMBITS(1) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(2) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl1dxctl2W [
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl1iocrR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED1 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(11) NUMBITS(11) [],
        RESERVED3 OFFSET(0) NUMBITS(11) [],
    ],
    pub Dx8sl1iocrW [
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED0 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED1 OFFSET(11) NUMBITS(11) [],
        RESERVED2 OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl2oscR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED2 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl2oscW [
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED0 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED1 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl2pllcr0R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(4) [],
        RESERVED4 OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl2pllcr0W [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        RESERVED3 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl2pllcr1R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl2pllcr1W [
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl2pllcr5R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx8sl2pllcr5W [
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl2dqsctlR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RRRMODE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        RESERVED2 OFFSET(15) NUMBITS(2) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        RESERVED3 OFFSET(10) NUMBITS(3) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl2dqsctlW [
        RRRMODE OFFSET(24) NUMBITS(1) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl2ddlctlR [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(2) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx8sl2ddlctlW [
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl2dxctl1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(16) [],
    ],
    pub Dx8sl2dxctl1W [
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl2dxctl2R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        RESERVED1 OFFSET(17) NUMBITS(1) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(2) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl2dxctl2W [
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl2iocrR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED1 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(11) NUMBITS(11) [],
        RESERVED3 OFFSET(0) NUMBITS(11) [],
    ],
    pub Dx8sl2iocrW [
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED0 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED1 OFFSET(11) NUMBITS(11) [],
        RESERVED2 OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl3oscR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED2 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl3oscW [
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED0 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED1 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl3pllcr0R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(4) [],
        RESERVED4 OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl3pllcr0W [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        RESERVED3 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl3pllcr1R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl3pllcr1W [
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl3pllcr5R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx8sl3pllcr5W [
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl3dqsctlR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RRRMODE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        RESERVED2 OFFSET(15) NUMBITS(2) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        RESERVED3 OFFSET(10) NUMBITS(3) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl3dqsctlW [
        RRRMODE OFFSET(24) NUMBITS(1) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl3ddlctlR [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(2) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx8sl3ddlctlW [
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl3dxctl1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(16) [],
    ],
    pub Dx8sl3dxctl1W [
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl3dxctl2R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        RESERVED1 OFFSET(17) NUMBITS(1) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(2) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl3dxctl2W [
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl3iocrR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED1 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(11) NUMBITS(11) [],
        RESERVED3 OFFSET(0) NUMBITS(11) [],
    ],
    pub Dx8sl3iocrW [
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED0 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED1 OFFSET(11) NUMBITS(11) [],
        RESERVED2 OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl4oscR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED2 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl4oscW [
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED0 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED1 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl4pllcr0R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(4) [],
        RESERVED4 OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl4pllcr0W [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        RESERVED3 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl4pllcr1R [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl4pllcr1W [
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl4pllcr5R [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ],
    pub Dx8sl4pllcr5W [
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl4dqsctlR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RRRMODE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        RESERVED2 OFFSET(15) NUMBITS(2) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        RESERVED3 OFFSET(10) NUMBITS(3) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ],
    pub Dx8sl4dqsctlW [
        RRRMODE OFFSET(24) NUMBITS(1) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl4ddlctlR [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(2) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ],
    pub Dx8sl4ddlctlW [
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl4dxctl1R [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(16) [],
    ],
    pub Dx8sl4dxctl1W [
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl4dxctl2R [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        RESERVED1 OFFSET(17) NUMBITS(1) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(2) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ],
    pub Dx8sl4dxctl2W [
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8sl4iocrR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED1 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(11) NUMBITS(11) [],
        RESERVED3 OFFSET(0) NUMBITS(11) [],
    ],
    pub Dx8sl4iocrW [
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED0 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED1 OFFSET(11) NUMBITS(11) [],
        RESERVED2 OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8slbosc [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        GATEDXRDCLK OFFSET(28) NUMBITS(2) [],
        GATEDXDDRCLK OFFSET(26) NUMBITS(2) [],
        GATEDXCTLCLK OFFSET(24) NUMBITS(2) [],
        CLKLEVEL OFFSET(22) NUMBITS(2) [],
        LBMODE OFFSET(21) NUMBITS(1) [],
        LBGSDQS OFFSET(20) NUMBITS(1) [],
        LBGDQS OFFSET(18) NUMBITS(2) [],
        LBDQSS OFFSET(17) NUMBITS(1) [],
        PHYHRST OFFSET(16) NUMBITS(1) [],
        PHYFRST OFFSET(15) NUMBITS(1) [],
        DLTST OFFSET(14) NUMBITS(1) [],
        DLTMODE OFFSET(13) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(2) [],
        OSCWDDL OFFSET(9) NUMBITS(2) [],
        RESERVED2 OFFSET(7) NUMBITS(2) [],
        OSCWDL OFFSET(5) NUMBITS(2) [],
        OSCDIV OFFSET(1) NUMBITS(4) [],
        OSCEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8slbpllcr0 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        PLLRST OFFSET(30) NUMBITS(1) [],
        PLLPD OFFSET(29) NUMBITS(1) [],
        RSTOPM OFFSET(28) NUMBITS(1) [],
        FRQSEL OFFSET(24) NUMBITS(4) [],
        RLOCKM OFFSET(23) NUMBITS(1) [],
        CPPC OFFSET(17) NUMBITS(6) [],
        CPIC OFFSET(13) NUMBITS(4) [],
        GSHIFT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(3) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(4) [],
        RESERVED4 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8slbpllcr1 [
        RESERVED0 OFFSET(22) NUMBITS(10) [],
        PLLPROG OFFSET(6) NUMBITS(16) [],
        BYPVREGCP OFFSET(5) NUMBITS(1) [],
        BYPVREGDIG OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        LOCKPS OFFSET(2) NUMBITS(1) [],
        LOCKCS OFFSET(1) NUMBITS(1) [],
        LOCKDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8slbpllcr5 [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        PLLCTRL_103_96 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8slbdqsctl [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RRRMODE OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(2) [],
        WRRMODE OFFSET(21) NUMBITS(1) [],
        DQSGX OFFSET(19) NUMBITS(2) [],
        LPPLLPD OFFSET(18) NUMBITS(1) [],
        LPIOPD OFFSET(17) NUMBITS(1) [],
        RESERVED2 OFFSET(15) NUMBITS(2) [],
        QSCNTEN OFFSET(14) NUMBITS(1) [],
        UDQIOM OFFSET(13) NUMBITS(1) [],
        RESERVED3 OFFSET(10) NUMBITS(3) [],
        DXSR OFFSET(8) NUMBITS(2) [],
        DQSNRES OFFSET(4) NUMBITS(4) [],
        DQSRES OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8slbddlctl [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        DLYLDTM OFFSET(26) NUMBITS(1) [],
        DXDDLLDT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(23) NUMBITS(2) [],
        DXDDLLD OFFSET(18) NUMBITS(5) [],
        DXDDLBYP OFFSET(2) NUMBITS(16) [],
        DDLBYPMODE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8slbdxctl1 [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        DXCALCLK OFFSET(24) NUMBITS(1) [],
        DXRCLKMD OFFSET(23) NUMBITS(1) [],
        RESERVED1 OFFSET(22) NUMBITS(1) [],
        DXDTOSEL OFFSET(20) NUMBITS(2) [],
        DXGSMD OFFSET(19) NUMBITS(1) [],
        DXQSDBYP OFFSET(18) NUMBITS(1) [],
        DXGDBYP OFFSET(17) NUMBITS(1) [],
        DXTMODE OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8slbdxctl2 [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        CRDEN OFFSET(23) NUMBITS(1) [],
        POSOEX OFFSET(20) NUMBITS(3) [],
        PREOEX OFFSET(18) NUMBITS(2) [],
        RESERVED1 OFFSET(17) NUMBITS(1) [],
        IOAG OFFSET(16) NUMBITS(1) [],
        IOLB OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(13) NUMBITS(2) [],
        LPWAKEUP_THRSH OFFSET(9) NUMBITS(4) [],
        RDBI OFFSET(8) NUMBITS(1) [],
        WDBI OFFSET(7) NUMBITS(1) [],
        PRFBYP OFFSET(6) NUMBITS(1) [],
        RDMODE OFFSET(4) NUMBITS(2) [],
        DISRST OFFSET(3) NUMBITS(1) [],
        DQSGLB OFFSET(1) NUMBITS(2) [],
        RESERVED3 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dx8slbiocr [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        DXDACRANGE OFFSET(28) NUMBITS(3) [],
        RESERVED1 OFFSET(25) NUMBITS(3) [],
        DXIOM OFFSET(22) NUMBITS(3) [],
        RESERVED2 OFFSET(11) NUMBITS(11) [],
        RESERVED3 OFFSET(0) NUMBITS(11) [],
    ]
];

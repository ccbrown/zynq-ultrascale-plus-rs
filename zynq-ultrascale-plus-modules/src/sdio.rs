// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// SDIO Controller, SDIO 0 Controller
pub static mut SD0: *mut Registers = 0xff160000 as *mut Registers;
/// SDIO Controller, SDIO 1 Controller
pub static mut SD1: *mut Registers = 0xff170000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Dual purpose: low SDMA address, Auto CMD23 arg.
        (0x00000000 => pub reg_sdmasysaddrlo: ReadWrite<u16>),
        /// Dual purpose: high SDMA address, Auto CMD23 arg.
        (0x00000002 => pub reg_sdmasysaddrhi: ReadWrite<u16>),
        /// Configure the Number of Bytes in a Data Block.
        (0x00000004 => pub reg_blocksize: ReadWrite<u16, RegBlocksize::Register>),
        /// Configure the number of data blocks
        (0x00000006 => pub reg_blockcount: ReadWrite<u16>),
        /// Lower bits of SD Command Argument
        (0x00000008 => pub reg_argument1lo: ReadWrite<u16>),
        /// Upper bits of SD Command Argument
        (0x0000000a => pub reg_argument1hi: ReadWrite<u16>),
        /// Control the Data Transfer Operations.
        (0x0000000c => pub reg_transfermode: ReadWrite<u16, RegTransfermode::Register>),
        /// Controller Commands.
        (0x0000000e => pub reg_command: ReadWrite<u16, RegCommand::Register>),
        /// Response 0 from SD Card.
        (0x00000010 => pub reg_response0: ReadOnly<u16>),
        /// This register is used to store responses from SD Cards
        (0x00000012 => pub reg_response1: ReadOnly<u16>),
        /// This register is used to store responses from SD Cards
        (0x00000014 => pub reg_response2: ReadOnly<u16>),
        /// This register is used to store responses from SD Cards
        (0x00000016 => pub reg_response3: ReadOnly<u16>),
        /// This register is used to store responses from SD Cards
        (0x00000018 => pub reg_response4: ReadOnly<u16>),
        /// This register is used to store responses from SD Cards
        (0x0000001a => pub reg_response5: ReadOnly<u16>),
        /// This register is used to store responses from SD Cards
        (0x0000001c => pub reg_response6: ReadOnly<u16>),
        /// This register is used to store responses from SD Cards
        (0x0000001e => pub reg_response7: ReadOnly<u16>),
        /// Read/write internal buffer.
        (0x00000020 => pub reg_dataport: ReadWrite<u32>),
        /// SDIO Controller Status, read-only.
        (0x00000024 => pub reg_presentstate: ReadOnly<u32, RegPresentstate::Register>),
        /// Controller Configuration.
        (0x00000028 => pub reg_hostcontrol1: ReadWrite<u8, RegHostcontrol1::Register>),
        /// SD Bus Power and Voltage Level.
        (0x00000029 => pub reg_powercontrol: ReadWrite<u8, RegPowercontrol::Register>),
        /// This register is used to program the block gap request, read wait control and interrupt at block gap
        (0x0000002a => pub reg_blockgapcontrol: ReadWrite<u8, RegBlockgapcontrol::Register>),
        /// Wakeup Functionality Control.
        (0x0000002b => pub reg_wakeupcontrol: ReadWrite<u8, RegWakeupcontrol::Register>),
        /// Clock Frequency Control and State.
        (0x0000002c => pub reg_clockcontrol: Aliased<u16, RegClockcontrolR::Register, RegClockcontrolW::Register>),
        /// Set the Data Timeout Counter Value.
        (0x0000002e => pub reg_timeoutcontrol: ReadWrite<u8, RegTimeoutcontrol::Register>),
        /// Software reset for data, command and all.
        (0x0000002f => pub reg_softwarereset: ReadWrite<u8, RegSoftwarereset::Register>),
        /// Status of allInterrupts
        (0x00000030 => pub reg_normalintrsts: Aliased<u16, RegNormalintrstsR::Register, RegNormalintrstsW::Register>),
        /// Error Interrupts Status
        (0x00000032 => pub reg_errorintrsts: ReadWrite<u16, RegErrorintrsts::Register>),
        /// Normal-type Interrupts Status Enables.
        (0x00000034 => pub reg_normalintrstsena: Aliased<u16, RegNormalintrstsenaR::Register, RegNormalintrstsenaW::Register>),
        /// Error-type Interrupts Status Enables.
        (0x00000036 => pub reg_errorintrstsena: ReadWrite<u16, RegErrorintrstsena::Register>),
        /// Normal-type Interrupts Signal Enables.
        (0x00000038 => pub reg_normalintrsigena: Aliased<u16, RegNormalintrsigenaR::Register, RegNormalintrsigenaW::Register>),
        /// Error-type Interrupts Signal Enables.
        (0x0000003a => pub reg_errorintrsigena: Aliased<u16, RegErrorintrsigenaR::Register, RegErrorintrsigenaW::Register>),
        /// CMD12 response error of Auto CMD12 and CMD23.
        (0x0000003c => pub reg_autocmderrsts: ReadOnly<u16, RegAutocmderrsts::Register>),
        /// UHS Mode, I/O Drive, Tuning, Clocking, Intr, and Presets.
        (0x0000003e => pub reg_hostcontrol2: ReadWrite<u16, RegHostcontrol2::Register>),
        /// Host controller implementation.
        (0x00000040 => pub reg_capabilities: ReadOnly<u64, RegCapabilities::Register>),
        /// Maximum current capability for each voltage.
        (0x00000048 => pub reg_maxcurrentcap: ReadOnly<u64, RegMaxcurrentcap::Register>),
        /// Generate Auto CMD Error Status Interrupts, write-only.
        (0x00000050 => pub reg_forceeventforautocmderrorstatus: WriteOnly<u16, RegForceeventforautocmderrorstatus::Register>),
        /// Generate Error Interrupt Status Interrupts.
        (0x00000052 => pub reg_forceeventforerrintsts: Aliased<u16, RegForceeventforerrintstsR::Register, RegForceeventforerrintstsW::Register>),
        /// SDIO ADMA Error State and Address.
        (0x00000054 => pub reg_admaerrsts: ReadOnly<u8, RegAdmaerrsts::Register>),
        (0x00000055 => _padding85),
        /// Lower physical address for ADMA data transfer.
        (0x00000058 => pub reg_admasysaddr0: ReadWrite<u16>),
        /// ADMA Physical Address, 16 LSBs.
        (0x0000005a => pub reg_admasysaddr1: ReadWrite<u16>),
        /// ADMA Physical Address, 16 bits.
        (0x0000005c => pub reg_admasysaddr2: ReadWrite<u16>),
        /// ADMA Physical Address, 16 MSBs.
        (0x0000005e => pub reg_admasysaddr3: ReadWrite<u16>),
        /// This register is used to read the SDCLK Frequency Select Value,Clock Generator Select Value,Driver Strength Select Value
        (0x00000060 => pub reg_presetvalue0: ReadOnly<u16, RegPresetvalue0::Register>),
        /// Default Clock and I/O Drive Preset Values.Read clock select values and I/O drive.
        (0x00000062 => pub reg_presetvalue1: ReadOnly<u16, RegPresetvalue1::Register>),
        /// High-Speed Clock and I/O Drive Preset Values.Read clock select values and I/O drive.
        (0x00000064 => pub reg_presetvalue2: ReadOnly<u16, RegPresetvalue2::Register>),
        /// SDR12 Clock and I/O Drive Preset Values.
        (0x00000066 => pub reg_presetvalue3: ReadOnly<u16, RegPresetvalue3::Register>),
        /// SDR25 Clock and I/O Drive Preset Values.
        (0x00000068 => pub reg_presetvalue4: ReadOnly<u16, RegPresetvalue4::Register>),
        /// SDR50 Clock and I/O Drive Preset Values.
        (0x0000006a => pub reg_presetvalue5: ReadOnly<u16, RegPresetvalue5::Register>),
        /// SDR 104 Mode Clock and I/O Drive Preset Values.
        (0x0000006c => pub reg_presetvalue6: ReadOnly<u16, RegPresetvalue6::Register>),
        /// DDR50Clock and I/O Drive Preset Values.
        (0x0000006e => pub reg_presetvalue7: ReadOnly<u16, RegPresetvalue7::Register>),
        /// Program the boot timeout value counter.
        (0x00000070 => pub reg_boottimeoutcnt: ReadWrite<u32>),
        (0x00000074 => _padding116),
        /// Read the interrupt signal for each slot.
        (0x000000fc => pub reg_slotintrsts: ReadOnly<u16, RegSlotintrsts::Register>),
        /// Controller version and specification numbers.
        (0x000000fe => pub reg_hostcontrollerver: ReadOnly<u16, RegHostcontrollerver::Register>),
        (0x00000100 => @END),
    }
}
register_bitfields! [
    u16,
    pub RegBlocksize [
        SDMA_BUFBOUNDARY OFFSET(12) NUMBITS(3) [],
        XFER_BLOCKSIZE OFFSET(0) NUMBITS(12) [],
    ]
];
register_bitfields! [
    u16,
    pub RegTransfermode [
        XFERMODE_MULTIBLKSEL OFFSET(5) NUMBITS(1) [],
        XFERMODE_DATAXFERDIR OFFSET(4) NUMBITS(1) [],
        XFERMODE_AUTOCMDENA OFFSET(2) NUMBITS(2) [],
        XFERMODE_BLKCNTENA OFFSET(1) NUMBITS(1) [],
        XFERMODE_DMAENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegCommand [
        COMMAND_CMDINDEX OFFSET(8) NUMBITS(6) [],
        COMMAND_CMDTYPE OFFSET(6) NUMBITS(2) [],
        COMMAND_DATAPRESENT OFFSET(5) NUMBITS(1) [],
        COMMAND_INDEXCHKENA OFFSET(4) NUMBITS(1) [],
        COMMAND_CRCCHKENA OFFSET(3) NUMBITS(1) [],
        COMMAND_RESPONSETYPE OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub RegPresentstate [
        SDIF_DAT7IN_DSYNC OFFSET(28) NUMBITS(1) [],
        SDIF_DAT6IN_DSYNC OFFSET(27) NUMBITS(1) [],
        SDIF_DAT5IN_DSYNC OFFSET(26) NUMBITS(1) [],
        SDIF_DAT4IN_DSYNC OFFSET(25) NUMBITS(1) [],
        SDIF_CMDIN_DSYNC OFFSET(24) NUMBITS(1) [],
        SDIF_DAT3IN_DSYNC OFFSET(23) NUMBITS(1) [],
        SDIF_DAT2IN_DSYNC OFFSET(22) NUMBITS(1) [],
        SDIF_DAT1IN_DSYNC OFFSET(21) NUMBITS(1) [],
        SDIF_DAT0IN_DSYNC OFFSET(20) NUMBITS(1) [],
        SDIF_WP_DSYNC OFFSET(19) NUMBITS(1) [],
        SDIF_CD_N_DSYNC OFFSET(18) NUMBITS(1) [],
        SDHCCARDDET_STATESTABLE_DSYNC OFFSET(17) NUMBITS(1) [],
        SDHCCARDDET_INSERTED_DSYNC OFFSET(16) NUMBITS(1) [],
        SDHCDMACTRL_PIOBUFRDENA OFFSET(11) NUMBITS(1) [],
        SDHCDMACTRL_PIOBUFWRENA OFFSET(10) NUMBITS(1) [],
        SDHCDMACTRL_RDXFERACTIVE OFFSET(9) NUMBITS(1) [],
        SDHCDMACTRL_WRXFERACTIVE OFFSET(8) NUMBITS(1) [],
        SDHCSDCTRL_RETUNINGREQ_DSYNC OFFSET(3) NUMBITS(1) [],
        SDHCDMACTRL_DATALINEACTIVE OFFSET(2) NUMBITS(1) [],
        PRESENTSTATE_INHIBITDAT OFFSET(1) NUMBITS(1) [],
        PRESENTSTATE_INHIBITCMD OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RegHostcontrol1 [
        HOSTCTRL1_CDSIGSELECT OFFSET(7) NUMBITS(1) [],
        HOSTCTRL1_CDTESTLEVEL OFFSET(6) NUMBITS(1) [],
        HOSTCTRL1_EXTDATAWIDTH OFFSET(5) NUMBITS(1) [],
        HOSTCTRL1_DMASELECT OFFSET(3) NUMBITS(2) [],
        HOSTCTRL1_HIGHSPEEDENA OFFSET(2) NUMBITS(1) [],
        HOSTCTRL1_DATAWIDTH OFFSET(1) NUMBITS(1) [],
        HOSTCTRL1_LEDCONTROL OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RegPowercontrol [
        EMMC_HWRESET OFFSET(4) NUMBITS(1) [],
        PWRCTRL_SDBUSVOLTAGE OFFSET(1) NUMBITS(3) [],
        PWRCTRL_SDBUSPOWER OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RegBlockgapcontrol [
        BLKGAPCTRL_BOOTACKENA OFFSET(7) NUMBITS(1) [],
        BLKGAPCTRL_ALTBOOTMODE OFFSET(6) NUMBITS(1) [],
        BLKGAPCTRL_BOOTENABLE OFFSET(5) NUMBITS(1) [],
        BLKGAPCTRL_SPIMODE OFFSET(4) NUMBITS(1) [],
        BLKGAPCTRL_INTERRUPT OFFSET(3) NUMBITS(1) [],
        BLKGAPCTRL_RDWAITCTRL OFFSET(2) NUMBITS(1) [],
        BLKGAPCTRL_CONTINUE OFFSET(1) NUMBITS(1) [],
        BLKGAPCTRL_STOPATBLKGAP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RegWakeupcontrol [
        WKUPCTRL_CARDREMOVAL OFFSET(2) NUMBITS(1) [],
        WKUPCTRL_CARDINSERTION OFFSET(1) NUMBITS(1) [],
        WKUPCTRL_CARDINTERRUPT OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegClockcontrolR [
        CLKCTRL_SDCLKFREQSEL OFFSET(8) NUMBITS(8) [],
        CLKCTRL_SDCLKFREQSEL_UPPERBITS OFFSET(6) NUMBITS(2) [],
        CLKCTRL_CLKGENSEL OFFSET(5) NUMBITS(1) [],
        CLKCTRL_SDCLKENA OFFSET(2) NUMBITS(1) [],
        SDHCCLKGEN_INTCLKSTABLE_DSYNC OFFSET(1) NUMBITS(1) [],
        CLKCTRL_INTCLKENA OFFSET(0) NUMBITS(1) [],
    ],
    pub RegClockcontrolW [
        CLKCTRL_SDCLKFREQSEL OFFSET(8) NUMBITS(8) [],
        CLKCTRL_SDCLKFREQSEL_UPPERBITS OFFSET(6) NUMBITS(2) [],
        CLKCTRL_CLKGENSEL OFFSET(5) NUMBITS(1) [],
        CLKCTRL_SDCLKENA OFFSET(2) NUMBITS(1) [],
        CLKCTRL_INTCLKENA OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RegTimeoutcontrol [
        TIMEOUT_CTRVALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u8,
    pub RegSoftwarereset [
        SWRESET_FOR_DAT OFFSET(2) NUMBITS(1) [],
        SWRESET_FOR_CMD OFFSET(1) NUMBITS(1) [],
        SWRESET_FOR_ALL OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegNormalintrstsR [
        REG_ERRORINTRSTS OFFSET(15) NUMBITS(1) [],
        NORMALINTRSTS_BOOTCOMPLETE OFFSET(14) NUMBITS(1) [],
        NORMALINTRSTS_RCVBOOTACK OFFSET(13) NUMBITS(1) [],
        NORMALINTRSTS_RETUNINGEVENT OFFSET(12) NUMBITS(1) [],
        NORMALINTRSTS_INTC OFFSET(11) NUMBITS(1) [],
        NORMALINTRSTS_INTB OFFSET(10) NUMBITS(1) [],
        NORMALINTRSTS_INTA OFFSET(9) NUMBITS(1) [],
        NORMALINTRSTS_CARDINTSTS OFFSET(8) NUMBITS(1) [],
        NORMALINTRSTS_CARDREMSTS OFFSET(7) NUMBITS(1) [],
        NORMALINTRSTS_CARDINSSTS OFFSET(6) NUMBITS(1) [],
        NORMALINTRSTS_BUFRDREADY OFFSET(5) NUMBITS(1) [],
        NORMALINTRSTS_BUFWRREADY OFFSET(4) NUMBITS(1) [],
        NORMALINTRSTS_DMAINTERRUPT OFFSET(3) NUMBITS(1) [],
        NORMALINTRSTS_BLKGAPEVENT OFFSET(2) NUMBITS(1) [],
        NORMALINTRSTS_XFERCOMPLETE OFFSET(1) NUMBITS(1) [],
        NORMALINTRSTS_CMDCOMPLETE OFFSET(0) NUMBITS(1) [],
    ],
    pub RegNormalintrstsW [
        NORMALINTRSTS_BOOTCOMPLETE OFFSET(14) NUMBITS(1) [],
        NORMALINTRSTS_RCVBOOTACK OFFSET(13) NUMBITS(1) [],
        NORMALINTRSTS_CARDREMSTS OFFSET(7) NUMBITS(1) [],
        NORMALINTRSTS_CARDINSSTS OFFSET(6) NUMBITS(1) [],
        NORMALINTRSTS_BUFRDREADY OFFSET(5) NUMBITS(1) [],
        NORMALINTRSTS_BUFWRREADY OFFSET(4) NUMBITS(1) [],
        NORMALINTRSTS_DMAINTERRUPT OFFSET(3) NUMBITS(1) [],
        NORMALINTRSTS_BLKGAPEVENT OFFSET(2) NUMBITS(1) [],
        NORMALINTRSTS_XFERCOMPLETE OFFSET(1) NUMBITS(1) [],
        NORMALINTRSTS_CMDCOMPLETE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegErrorintrsts [
        ERRORINTRSTS_HOSTERROR OFFSET(12) NUMBITS(1) [],
        ERRORINTRSTS_ADMAERROR OFFSET(9) NUMBITS(1) [],
        ERRORINTRSTS_AUTOCMDERROR OFFSET(8) NUMBITS(1) [],
        ERRORINTRSTS_CURRLIMITERROR OFFSET(7) NUMBITS(1) [],
        ERRORINTRSTS_DATAENDBITERROR OFFSET(6) NUMBITS(1) [],
        ERRORINTRSTS_DATACRCERROR OFFSET(5) NUMBITS(1) [],
        ERRORINTRSTS_DATATIMEOUTERROR OFFSET(4) NUMBITS(1) [],
        ERRORINTRSTS_CMDINDEXERROR OFFSET(3) NUMBITS(1) [],
        ERRORINTRSTS_CMDENDBITERROR OFFSET(2) NUMBITS(1) [],
        ERRORINTRSTS_CMDCRCERROR OFFSET(1) NUMBITS(1) [],
        ERRORINTRSTS_CMDTIMEOUTERROR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegNormalintrstsenaR [
        NORMALINTRSTS_ENABLEREGBIT15 OFFSET(15) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT14 OFFSET(14) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT13 OFFSET(13) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT12 OFFSET(12) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT11 OFFSET(11) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT10 OFFSET(10) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT9 OFFSET(9) NUMBITS(1) [],
        SDHCREGSET_CARDINTSTSENA OFFSET(8) NUMBITS(1) [],
        SDHCREGSET_CARDREMSTSENA OFFSET(7) NUMBITS(1) [],
        SDHCREGSET_CARDINSSTSENA OFFSET(6) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT5 OFFSET(5) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT4 OFFSET(4) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT3 OFFSET(3) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT2 OFFSET(2) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT1 OFFSET(1) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT0 OFFSET(0) NUMBITS(1) [],
    ],
    pub RegNormalintrstsenaW [
        NORMALINTRSTS_ENABLEREGBIT14 OFFSET(14) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT13 OFFSET(13) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT12 OFFSET(12) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT11 OFFSET(11) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT10 OFFSET(10) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT9 OFFSET(9) NUMBITS(1) [],
        SDHCREGSET_CARDINTSTSENA OFFSET(8) NUMBITS(1) [],
        SDHCREGSET_CARDREMSTSENA OFFSET(7) NUMBITS(1) [],
        SDHCREGSET_CARDINSSTSENA OFFSET(6) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT5 OFFSET(5) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT4 OFFSET(4) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT3 OFFSET(3) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT2 OFFSET(2) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT1 OFFSET(1) NUMBITS(1) [],
        NORMALINTRSTS_ENABLEREGBIT0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegErrorintrstsena [
        ERRORINTRSTS_ENABLEREGBIT12 OFFSET(12) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT10 OFFSET(10) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT9 OFFSET(9) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT8 OFFSET(8) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT7 OFFSET(7) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT6 OFFSET(6) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT5 OFFSET(5) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT4 OFFSET(4) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT3 OFFSET(3) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT2 OFFSET(2) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT1 OFFSET(1) NUMBITS(1) [],
        ERRORINTRSTS_ENABLEREGBIT0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegNormalintrsigenaR [
        NORMALINTRSIG_ENABLEREGBIT15 OFFSET(15) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT14 OFFSET(14) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT13 OFFSET(13) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT12 OFFSET(12) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT11 OFFSET(11) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT10 OFFSET(10) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT9 OFFSET(9) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT8 OFFSET(8) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT7 OFFSET(7) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT6 OFFSET(6) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT5 OFFSET(5) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT4 OFFSET(4) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT3 OFFSET(3) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT2 OFFSET(2) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT1 OFFSET(1) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT0 OFFSET(0) NUMBITS(1) [],
    ],
    pub RegNormalintrsigenaW [
        NORMALINTRSIG_ENABLEREGBIT14 OFFSET(14) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT13 OFFSET(13) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT12 OFFSET(12) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT11 OFFSET(11) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT10 OFFSET(10) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT9 OFFSET(9) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT8 OFFSET(8) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT7 OFFSET(7) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT6 OFFSET(6) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT5 OFFSET(5) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT4 OFFSET(4) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT3 OFFSET(3) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT2 OFFSET(2) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT1 OFFSET(1) NUMBITS(1) [],
        NORMALINTRSIG_ENABLEREGBIT0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegErrorintrsigenaR [
        ERRORINTRSIG_ENABLEREGBIT12 OFFSET(12) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT10 OFFSET(10) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT9 OFFSET(9) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT8 OFFSET(8) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT7 OFFSET(7) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT6 OFFSET(6) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT5 OFFSET(5) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT4 OFFSET(4) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT3 OFFSET(3) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT2 OFFSET(2) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT1 OFFSET(1) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT0 OFFSET(0) NUMBITS(1) [],
    ],
    pub RegErrorintrsigenaW [
        ERRORINTRSIG_ENABLEREGBIT10 OFFSET(10) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT9 OFFSET(9) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT8 OFFSET(8) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT7 OFFSET(7) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT6 OFFSET(6) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT5 OFFSET(5) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT4 OFFSET(4) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT3 OFFSET(3) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT2 OFFSET(2) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT1 OFFSET(1) NUMBITS(1) [],
        ERRORINTRSIG_ENABLEREGBIT0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegAutocmderrsts [
        AUTOCMDERRSTS_NEXTERROR OFFSET(7) NUMBITS(1) [],
        AUTOCMDERRSTS_INDEXERROR OFFSET(4) NUMBITS(1) [],
        AUTOCMDERRSTS_ENDBITERROR OFFSET(3) NUMBITS(1) [],
        AUTOCMDERRSTS_CRCERROR OFFSET(2) NUMBITS(1) [],
        AUTOCMDERRSTS_TIMEOUTERROR OFFSET(1) NUMBITS(1) [],
        AUTOCMDERRSTS_NOTEXECERROR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegHostcontrol2 [
        HOSTCTRL2_PRESETVALUEENABLE OFFSET(15) NUMBITS(1) [],
        HOSTCTRL2_ASYNCHINTRENABLE OFFSET(14) NUMBITS(1) [],
        HOSTCTRL2_SAMPLINGCLKSELECT OFFSET(7) NUMBITS(1) [],
        HOSTCTRL2_EXECUTETUNING OFFSET(6) NUMBITS(1) [],
        RESERVED0 OFFSET(4) NUMBITS(2) [],
        HOSTCTRL2_1P8VSIGNALLINGENA OFFSET(3) NUMBITS(1) [],
        HOSTCTRL2_UHSMODESELECT OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u64,
    pub RegCapabilities [
        CORECFG_SPIBLKMODE OFFSET(57) NUMBITS(1) [],
        CORECFG_SPISUPPORT OFFSET(56) NUMBITS(1) [],
        CORECFG_CLOCKMULTIPLIER OFFSET(48) NUMBITS(8) [],
        CORECFG_RETUNINGMODES OFFSET(46) NUMBITS(2) [],
        CORECFG_TUNINGFORSDR50 OFFSET(45) NUMBITS(1) [],
        CORECFG_RETUNINGTIMERCNT OFFSET(40) NUMBITS(4) [],
        CORECFG_DDRIVERSUPPORT OFFSET(38) NUMBITS(1) [],
        CORECFG_CDRIVERSUPPORT OFFSET(37) NUMBITS(1) [],
        CORECFG_ADRIVERSUPPORT OFFSET(36) NUMBITS(1) [],
        CORECFG_DDR50SUPPORT OFFSET(34) NUMBITS(1) [],
        CORECFG_SDR104SUPPORT OFFSET(33) NUMBITS(1) [],
        CORECFG_SDR50SUPPORT OFFSET(32) NUMBITS(1) [],
        CORECFG_SLOTTYPE OFFSET(30) NUMBITS(2) [],
        CORECFG_ASYNCHINTRSUPPORT OFFSET(29) NUMBITS(1) [],
        CORECFG_64BITSUPPORT OFFSET(28) NUMBITS(1) [],
        CORECFG_1P8VOLTSUPPORT OFFSET(26) NUMBITS(1) [],
        CORECFG_3P0VOLTSUPPORT OFFSET(25) NUMBITS(1) [],
        CORECFG_3P3VOLTSUPPORT OFFSET(24) NUMBITS(1) [],
        CORECFG_SUSPRESSUPPORT OFFSET(23) NUMBITS(1) [],
        CORECFG_SDMASUPPORT OFFSET(22) NUMBITS(1) [],
        CORECFG_HIGHSPEEDSUPPORT OFFSET(21) NUMBITS(1) [],
        CORECFG_ADMA2SUPPORT OFFSET(19) NUMBITS(1) [],
        CORECFG_8BITSUPPORT OFFSET(18) NUMBITS(1) [],
        CORECFG_MAXBLKLENGTH OFFSET(16) NUMBITS(2) [],
        CORECFG_BASECLKFREQ OFFSET(8) NUMBITS(8) [],
        CORECFG_TIMEOUTCLKUNIT OFFSET(7) NUMBITS(1) [],
        CORECFG_TIMEOUTCLKFREQ OFFSET(0) NUMBITS(6) [],
    ]
];
register_bitfields! [
    u64,
    pub RegMaxcurrentcap [
        CORECFG_MAXCURRENT1P8V OFFSET(16) NUMBITS(8) [],
        CORECFG_MAXCURRENT3P0V OFFSET(8) NUMBITS(8) [],
        CORECFG_MAXCURRENT3P3V OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u16,
    pub RegForceeventforautocmderrorstatus [
        FORCECMDNOTISSUEDBYAUTOCMD12ERR OFFSET(7) NUMBITS(1) [],
        FORCEAUTOCMDINDEXERR OFFSET(4) NUMBITS(1) [],
        FORCEAUTOCMDENDBITERR OFFSET(3) NUMBITS(1) [],
        FORCEAUTOCMDCRCERR OFFSET(2) NUMBITS(1) [],
        FORCEAUTOCMDTIMEOUTERR OFFSET(1) NUMBITS(1) [],
        FORCEAUTOCMDNOTEXEC OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u16,
    pub RegForceeventforerrintstsR [
        FORCETUNINGERR OFFSET(10) NUMBITS(1) [],
    ],
    pub RegForceeventforerrintstsW [
        FORCEADMAERR OFFSET(9) NUMBITS(1) [],
        FORCEAUTOCMDERR OFFSET(8) NUMBITS(1) [],
        FORCECURRLIMERR OFFSET(7) NUMBITS(1) [],
        FORCEDATENDBITERR OFFSET(6) NUMBITS(1) [],
        FORCEDATCRCERR OFFSET(5) NUMBITS(1) [],
        FORCEDATTIMEOUTERR OFFSET(4) NUMBITS(1) [],
        FORCECMDINDEXERR OFFSET(3) NUMBITS(1) [],
        FORCECMDENDBITERR OFFSET(2) NUMBITS(1) [],
        FORCECMDCRCERR OFFSET(1) NUMBITS(1) [],
        FORCECMDTIMEOUTERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u8,
    pub RegAdmaerrsts [
        ADMAERRSTS_ADMALENMISMATCHERR OFFSET(2) NUMBITS(1) [],
        ADMAERRSTS_ADMAERRORSTATE OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u16,
    pub RegPresetvalue0 [
        DRIVERSTRENGTHSELECTVALUE OFFSET(14) NUMBITS(2) [],
        CLOCKGENERATORSELECTVALUE OFFSET(10) NUMBITS(1) [],
        SDCLKFREQUENCYSELECTVALUE OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u16,
    pub RegPresetvalue1 [
        DRIVERSTRENGTHSELECTVALUE OFFSET(14) NUMBITS(2) [],
        CLOCKGENERATORSELECTVALUE OFFSET(10) NUMBITS(1) [],
        SDCLKFREQUENCYSELECTVALUE OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u16,
    pub RegPresetvalue2 [
        DRIVERSTRENGTHSELECTVALUE OFFSET(14) NUMBITS(2) [],
        CLOCKGENERATORSELECTVALUE OFFSET(10) NUMBITS(1) [],
        SDCLKFREQUENCYSELECTVALUE OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u16,
    pub RegPresetvalue3 [
        DRIVERSTRENGTHSELECTVALUE OFFSET(14) NUMBITS(2) [],
        CLOCKGENERATORSELECTVALUE OFFSET(10) NUMBITS(1) [],
        SDCLKFREQUENCYSELECTVALUE OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u16,
    pub RegPresetvalue4 [
        DRIVERSTRENGTHSELECTVALUE OFFSET(14) NUMBITS(2) [],
        CLOCKGENERATORSELECTVALUE OFFSET(10) NUMBITS(1) [],
        SDCLKFREQUENCYSELECTVALUE OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u16,
    pub RegPresetvalue5 [
        DRIVERSTRENGTHSELECTVALUE OFFSET(14) NUMBITS(2) [],
        CLOCKGENERATORSELECTVALUE OFFSET(10) NUMBITS(1) [],
        SDCLKFREQUENCYSELECTVALUE OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u16,
    pub RegPresetvalue6 [
        DRIVERSTRENGTHSELECTVALUE OFFSET(14) NUMBITS(2) [],
        CLOCKGENERATORSELECTVALUE OFFSET(10) NUMBITS(1) [],
        SDCLKFREQUENCYSELECTVALUE OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u16,
    pub RegPresetvalue7 [
        DRIVERSTRENGTHSELECTVALUE OFFSET(14) NUMBITS(2) [],
        CLOCKGENERATORSELECTVALUE OFFSET(10) NUMBITS(1) [],
        SDCLKFREQUENCYSELECTVALUE OFFSET(0) NUMBITS(10) [],
    ]
];
register_bitfields! [
    u16,
    pub RegSlotintrsts [
        SDHCHOSTIF_SLOTINTRSTS OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u16,
    pub RegHostcontrollerver [
        SDHC_VENVERNUM OFFSET(8) NUMBITS(8) [],
        SPECIFICATIONVERSIONNUMBER OFFSET(0) NUMBITS(8) [],
    ]
];

// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// SATA AHCI Port Control, SATA AHCI Port 0 Control
pub static mut SATA_AHCI_PORT0_CNTRL: *mut Registers = 0xfd0c0100 as *mut Registers;
/// SATA AHCI Port Control, SATA AHCI Port 1 Control
pub static mut SATA_AHCI_PORT1_CNTRL: *mut Registers = 0xfd0c0180 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Port x Command List Base Address (PxCLB)
    pub pxclb: Aliased<u32, PxclbR::Register, PxclbW::Register>,
    /// Port x Command List Base Address Upper 32-bits (PxCLBU)
    pub pxclbu: ReadWrite<u32>,
    /// Port x FIS Base Address (PxFB)
    pub pxfb: Aliased<u32, PxfbR::Register, PxfbW::Register>,
    /// Port x FIS Base Address Upper 32-bits (PxFBU)
    pub pxfbu: ReadWrite<u32>,
    /// Port x Interupt Status (PxIS)
    pub pxis: Aliased<u32, PxisR::Register, PxisW::Register>,
    /// Port x Interrupt Enable (PxIE)
    pub pxie: Aliased<u32, PxieR::Register, PxieW::Register>,
    /// Port x Command and Status (PxCMD)
    pub pxcmd: Aliased<u32, PxcmdR::Register, PxcmdW::Register>,
    _padding28: [u8; 4],
    /// Port x Task File Data (PxTFD)
    pub pxtfd: ReadOnly<u32, Pxtfd::Register>,
    /// Port x Signature (PxSIG).
    pub pxsig: ReadOnly<u32>,
    /// Port x serial ATA Status (SCR0: Sstatus) (PxSSTS)
    pub pxssts: ReadOnly<u32, Pxssts::Register>,
    /// Port x Serial ATA Control (SCR2: SControl) (PxSCTL)
    pub pxsctl: Aliased<u32, PxsctlR::Register, PxsctlW::Register>,
    /// Port x Serial ATA Error (SCR1: SError)
    pub pxserr: ReadWrite<u32, Pxserr::Register>,
    /// Port x Serial ATA Active (SCR3: SActive)
    pub pxsact: ReadWrite<u32>,
    /// PxCI: Port x Command Issue
    pub pxci: ReadWrite<u32>,
    /// Port x Serial ATA Notification (SCR4: SNotification)
    pub pxsntf: Aliased<u32, PxsntfR::Register, PxsntfW::Register>,
    /// Port x FIS-based Switching Control
    pub pxfbs: Aliased<u32, PxfbsR::Register, PxfbsW::Register>,
    /// PxDEVSLP - Port x Device Sleep
    pub pxdevslp: Aliased<u32, PxdevslpR::Register, PxdevslpW::Register>,
    _padding72: [u8; 40],
    /// PBERR - Port 0/1 BIST Error.
    pub pberr: Aliased<u32, PberrR::Register, PberrW::Register>,
    /// CMDS - Port 0/1 Command Status Error.
    pub cmds: ReadOnly<u32, Cmds::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub PxclbR [
        CLB OFFSET(10) NUMBITS(22) [],
        RESERVED0 OFFSET(0) NUMBITS(10) [],
    ],
    pub PxclbW [
        CLB OFFSET(10) NUMBITS(22) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PxfbR [
        FB OFFSET(8) NUMBITS(24) [],
        RESERVED0 OFFSET(0) NUMBITS(8) [],
    ],
    pub PxfbW [
        FB OFFSET(8) NUMBITS(24) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PxisR [
        CPDS OFFSET(31) NUMBITS(1) [],
        TFES OFFSET(30) NUMBITS(1) [],
        HBFS OFFSET(29) NUMBITS(1) [],
        HBDS OFFSET(28) NUMBITS(1) [],
        IFS OFFSET(27) NUMBITS(1) [],
        INFS OFFSET(26) NUMBITS(1) [],
        RESERVED0 OFFSET(25) NUMBITS(1) [],
        OFS OFFSET(24) NUMBITS(1) [],
        IPMS OFFSET(23) NUMBITS(1) [],
        PRCS OFFSET(22) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(14) [],
        DMPS OFFSET(7) NUMBITS(1) [],
        PCS OFFSET(6) NUMBITS(1) [],
        DPS OFFSET(5) NUMBITS(1) [],
        UFS OFFSET(4) NUMBITS(1) [],
        SDBS OFFSET(3) NUMBITS(1) [],
        DSS OFFSET(2) NUMBITS(1) [],
        PSS OFFSET(1) NUMBITS(1) [],
        DHRS OFFSET(0) NUMBITS(1) [],
    ],
    pub PxisW [
        CPDS OFFSET(31) NUMBITS(1) [],
        TFES OFFSET(30) NUMBITS(1) [],
        HBFS OFFSET(29) NUMBITS(1) [],
        HBDS OFFSET(28) NUMBITS(1) [],
        IFS OFFSET(27) NUMBITS(1) [],
        INFS OFFSET(26) NUMBITS(1) [],
        OFS OFFSET(24) NUMBITS(1) [],
        IPMS OFFSET(23) NUMBITS(1) [],
        DMPS OFFSET(7) NUMBITS(1) [],
        DPS OFFSET(5) NUMBITS(1) [],
        SDBS OFFSET(3) NUMBITS(1) [],
        DSS OFFSET(2) NUMBITS(1) [],
        PSS OFFSET(1) NUMBITS(1) [],
        DHRS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PxieR [
        CPDE OFFSET(31) NUMBITS(1) [],
        TFEE OFFSET(30) NUMBITS(1) [],
        HBFE OFFSET(29) NUMBITS(1) [],
        HBDE OFFSET(28) NUMBITS(1) [],
        IFE OFFSET(27) NUMBITS(1) [],
        INFE OFFSET(26) NUMBITS(1) [],
        RESERVED0 OFFSET(25) NUMBITS(1) [],
        OFE OFFSET(24) NUMBITS(1) [],
        IPME OFFSET(23) NUMBITS(1) [],
        PRCE OFFSET(22) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(14) [],
        DMPE OFFSET(7) NUMBITS(1) [],
        PCE OFFSET(6) NUMBITS(1) [],
        DPE OFFSET(5) NUMBITS(1) [],
        UFE OFFSET(4) NUMBITS(1) [],
        SDBE OFFSET(3) NUMBITS(1) [],
        DSE OFFSET(2) NUMBITS(1) [],
        PSE OFFSET(1) NUMBITS(1) [],
        DHRE OFFSET(0) NUMBITS(1) [],
    ],
    pub PxieW [
        CPDE OFFSET(31) NUMBITS(1) [],
        TFEE OFFSET(30) NUMBITS(1) [],
        HBFE OFFSET(29) NUMBITS(1) [],
        HBDE OFFSET(28) NUMBITS(1) [],
        IFE OFFSET(27) NUMBITS(1) [],
        INFE OFFSET(26) NUMBITS(1) [],
        OFE OFFSET(24) NUMBITS(1) [],
        IPME OFFSET(23) NUMBITS(1) [],
        PRCE OFFSET(22) NUMBITS(1) [],
        DMPE OFFSET(7) NUMBITS(1) [],
        PCE OFFSET(6) NUMBITS(1) [],
        DPE OFFSET(5) NUMBITS(1) [],
        UFE OFFSET(4) NUMBITS(1) [],
        SDBE OFFSET(3) NUMBITS(1) [],
        DSE OFFSET(2) NUMBITS(1) [],
        PSE OFFSET(1) NUMBITS(1) [],
        DHRE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PxcmdR [
        ICC OFFSET(28) NUMBITS(4) [],
        ASP OFFSET(27) NUMBITS(1) [],
        ALPE OFFSET(26) NUMBITS(1) [],
        DLAE OFFSET(25) NUMBITS(1) [],
        ATAPI OFFSET(24) NUMBITS(1) [],
        APSTE OFFSET(23) NUMBITS(1) [],
        FBSCP OFFSET(22) NUMBITS(1) [],
        ESP OFFSET(21) NUMBITS(1) [],
        CPD OFFSET(20) NUMBITS(1) [],
        MPSP OFFSET(19) NUMBITS(1) [],
        HPCP OFFSET(18) NUMBITS(1) [],
        PMA OFFSET(17) NUMBITS(1) [],
        CPS OFFSET(16) NUMBITS(1) [],
        CR OFFSET(15) NUMBITS(1) [],
        FR OFFSET(14) NUMBITS(1) [],
        MPSS OFFSET(13) NUMBITS(1) [],
        CCS OFFSET(8) NUMBITS(5) [],
        RESERVED0 OFFSET(5) NUMBITS(3) [],
        FRE OFFSET(4) NUMBITS(1) [],
        CLO OFFSET(3) NUMBITS(1) [],
        POD OFFSET(2) NUMBITS(1) [],
        SUD OFFSET(1) NUMBITS(1) [],
        ST OFFSET(0) NUMBITS(1) [],
    ],
    pub PxcmdW [
        ICC OFFSET(28) NUMBITS(4) [],
        ASP OFFSET(27) NUMBITS(1) [],
        ALPE OFFSET(26) NUMBITS(1) [],
        DLAE OFFSET(25) NUMBITS(1) [],
        ATAPI OFFSET(24) NUMBITS(1) [],
        APSTE OFFSET(23) NUMBITS(1) [],
        PMA OFFSET(17) NUMBITS(1) [],
        FRE OFFSET(4) NUMBITS(1) [],
        CLO OFFSET(3) NUMBITS(1) [],
        ST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pxtfd [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ERR OFFSET(8) NUMBITS(8) [],
        STS_BSY OFFSET(7) NUMBITS(1) [],
        STS_CS1 OFFSET(4) NUMBITS(3) [],
        STS_DRQ OFFSET(3) NUMBITS(1) [],
        STS_CS OFFSET(1) NUMBITS(2) [],
        STS_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pxssts [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        IPM OFFSET(8) NUMBITS(4) [],
        SPD OFFSET(4) NUMBITS(4) [],
        DET OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PxsctlR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        PMP OFFSET(16) NUMBITS(4) [],
        SPM OFFSET(12) NUMBITS(4) [],
        IPM OFFSET(8) NUMBITS(4) [],
        SPD OFFSET(4) NUMBITS(4) [],
        DET OFFSET(0) NUMBITS(4) [],
    ],
    pub PxsctlW [
        IPM OFFSET(8) NUMBITS(4) [],
        SPD OFFSET(4) NUMBITS(4) [],
        DET OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pxserr [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        DIAG_X OFFSET(26) NUMBITS(1) [],
        DIAG_F OFFSET(25) NUMBITS(1) [],
        DIAG_T OFFSET(24) NUMBITS(1) [],
        DIAG_S OFFSET(23) NUMBITS(1) [],
        DIAG_H OFFSET(22) NUMBITS(1) [],
        DIAG_C OFFSET(21) NUMBITS(1) [],
        DIAG_D OFFSET(20) NUMBITS(1) [],
        DIAG_B OFFSET(19) NUMBITS(1) [],
        DIAG_W OFFSET(18) NUMBITS(1) [],
        DIAG_I OFFSET(17) NUMBITS(1) [],
        DIAG_N OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        ERR_E OFFSET(11) NUMBITS(1) [],
        ERR_P OFFSET(10) NUMBITS(1) [],
        ERR_C OFFSET(9) NUMBITS(1) [],
        ERR_T OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(2) NUMBITS(6) [],
        ERR_M OFFSET(1) NUMBITS(1) [],
        ERR_I OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PxsntfR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        PMN OFFSET(0) NUMBITS(16) [],
    ],
    pub PxsntfW [
        PMN OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PxfbsR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        DWE OFFSET(16) NUMBITS(4) [],
        ADO OFFSET(12) NUMBITS(4) [],
        DEV OFFSET(8) NUMBITS(4) [],
        RESERVED1 OFFSET(3) NUMBITS(5) [],
        SDE OFFSET(2) NUMBITS(1) [],
        DEC OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ],
    pub PxfbsW [
        DEV OFFSET(8) NUMBITS(4) [],
        DEC OFFSET(1) NUMBITS(1) [],
        EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PxdevslpR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        DM OFFSET(25) NUMBITS(4) [],
        DITO OFFSET(15) NUMBITS(10) [],
        MDAT OFFSET(10) NUMBITS(5) [],
        DETO OFFSET(2) NUMBITS(8) [],
        DSP OFFSET(1) NUMBITS(1) [],
        ADSE OFFSET(0) NUMBITS(1) [],
    ],
    pub PxdevslpW [
        DITO OFFSET(15) NUMBITS(10) [],
        MDAT OFFSET(10) NUMBITS(5) [],
        DETO OFFSET(2) NUMBITS(8) [],
        ADSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PberrR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        BEOS OFFSET(1) NUMBITS(1) [],
        BERR OFFSET(0) NUMBITS(1) [],
    ],
    pub PberrW [
        BEOS OFFSET(1) NUMBITS(1) [],
        BERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cmds [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        CQMS OFFSET(8) NUMBITS(4) [],
        CS OFFSET(0) NUMBITS(8) [],
    ]
];

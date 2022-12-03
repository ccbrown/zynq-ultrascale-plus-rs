// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// SATA AHCI Control and Status, SATA AHCI Vendor
pub static mut SATA_AHCI_VENDOR: *mut Registers = 0xfd0c00a0 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// SerDes control AHB control port.
    pub pctrl: Aliased<u32, PctrlR::Register, PctrlW::Register>,
    /// Dual lane port select, timer scalars, interrupt separation.
    pub pcfg: Aliased<u32, PcfgR::Register, PcfgW::Register>,
    /// Phy Control Layer 1 configuration for port 0 or 1.
    pub ppcfg: Aliased<u32, PpcfgR::Register, PpcfgW::Register>,
    /// Port Phy Configuration 2.
    pub pp2c: ReadWrite<u32, Pp2c::Register>,
    /// PP3C - Port Phy3 Configuration
    pub pp3c: ReadWrite<u32, Pp3c::Register>,
    /// Port Phy Configuration 4.
    pub pp4c: ReadWrite<u32, Pp4c::Register>,
    /// Port Phy Configuration 5.
    pub pp5c: ReadWrite<u32, Pp5c::Register>,
    /// AXI CACHE Control.
    pub axicc: Aliased<u32, AxiccR::Register, AxiccW::Register>,
    /// Port AXICfg
    pub paxic: Aliased<u32, PaxicR::Register, PaxicW::Register>,
    /// AXI PROT Control.
    pub axipc: Aliased<u32, AxipcR::Register, AxipcW::Register>,
    /// Port Transfer Configuration
    pub ptc: Aliased<u32, PtcR::Register, PtcW::Register>,
    /// Transport Layer Status (TransStatus).
    pub pts: ReadOnly<u32, Pts::Register>,
    /// Link Layer Configuration (LinkCfg).
    pub plc: ReadWrite<u32, Plc::Register>,
    /// Port LinkCfg1
    pub plc1: Aliased<u32, Plc1R::Register, Plc1W::Register>,
    /// Port LinkCfg2
    pub plc2: ReadWrite<u32>,
    /// Port Link-layer Status.
    pub pls: ReadOnly<u32, Pls::Register>,
    /// Port Link-layer Status 1.
    pub pls1: ReadWrite<u32, Pls1::Register>,
    /// Port CmdConfig
    pub pcmdc: Aliased<u32, PcmdcR::Register, PcmdcW::Register>,
    /// Port Phy Status (PhyControlStatus).
    pub ppcs: Aliased<u32, PpcsR::Register, PpcsW::Register>,
    /// AXI Master Status
    pub ams: ReadOnly<u32, Ams::Register>,
    /// Timer Control
    pub tcr: Aliased<u32, TcrR::Register, TcrW::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub PctrlR [
        BSY OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(25) NUMBITS(6) [],
        SRI OFFSET(24) NUMBITS(1) [],
        SRWD OFFSET(16) NUMBITS(8) [],
        SAV OFFSET(0) NUMBITS(16) [],
    ],
    pub PctrlW [
        SRI OFFSET(24) NUMBITS(1) [],
        SRWD OFFSET(16) NUMBITS(8) [],
        SAV OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcfgR [
        RESERVED0 OFFSET(23) NUMBITS(9) [],
        TPSS OFFSET(16) NUMBITS(7) [],
        RESERVED1 OFFSET(15) NUMBITS(1) [],
        TPRS OFFSET(12) NUMBITS(3) [],
        RESERVED2 OFFSET(9) NUMBITS(3) [],
        CISE OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(6) NUMBITS(2) [],
        PAD OFFSET(0) NUMBITS(6) [],
    ],
    pub PcfgW [
        TPSS OFFSET(16) NUMBITS(7) [],
        TPRS OFFSET(12) NUMBITS(3) [],
        CISE OFFSET(8) NUMBITS(1) [],
        PAD OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PpcfgR [
        ESDF OFFSET(31) NUMBITS(1) [],
        ERSN OFFSET(30) NUMBITS(1) [],
        PSS OFFSET(29) NUMBITS(1) [],
        PSSO OFFSET(28) NUMBITS(1) [],
        STB OFFSET(27) NUMBITS(1) [],
        PBPNA OFFSET(26) NUMBITS(1) [],
        PBCE OFFSET(25) NUMBITS(1) [],
        PBPE OFFSET(24) NUMBITS(1) [],
        PBPS OFFSET(21) NUMBITS(3) [],
        FPR OFFSET(20) NUMBITS(1) [],
        RESERVED0 OFFSET(19) NUMBITS(1) [],
        SNR OFFSET(18) NUMBITS(1) [],
        SNM OFFSET(17) NUMBITS(1) [],
        TTA OFFSET(0) NUMBITS(17) [],
    ],
    pub PpcfgW [
        ESDF OFFSET(31) NUMBITS(1) [],
        ERSN OFFSET(30) NUMBITS(1) [],
        PSS OFFSET(29) NUMBITS(1) [],
        PSSO OFFSET(28) NUMBITS(1) [],
        PBPNA OFFSET(26) NUMBITS(1) [],
        PBCE OFFSET(25) NUMBITS(1) [],
        PBPE OFFSET(24) NUMBITS(1) [],
        PBPS OFFSET(21) NUMBITS(3) [],
        FPR OFFSET(20) NUMBITS(1) [],
        SNR OFFSET(18) NUMBITS(1) [],
        SNM OFFSET(17) NUMBITS(1) [],
        TTA OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp2c [
        CINMP OFFSET(24) NUMBITS(8) [],
        CIBGN OFFSET(16) NUMBITS(8) [],
        CIBGMX OFFSET(8) NUMBITS(8) [],
        CIBGMN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp3c [
        CWNMP OFFSET(24) NUMBITS(8) [],
        CWBGN OFFSET(16) NUMBITS(8) [],
        CWBGMX OFFSET(8) NUMBITS(8) [],
        CWBGMN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp4c [
        PTST OFFSET(24) NUMBITS(8) [],
        SFD OFFSET(16) NUMBITS(8) [],
        BNM OFFSET(8) NUMBITS(8) [],
        BMX OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pp5c [
        RCT OFFSET(20) NUMBITS(12) [],
        RIT OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AxiccR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        EARC OFFSET(29) NUMBITS(1) [],
        AWCF OFFSET(24) NUMBITS(4) [],
        AWCD OFFSET(20) NUMBITS(4) [],
        AWCFD OFFSET(16) NUMBITS(4) [],
        ARCP OFFSET(12) NUMBITS(4) [],
        ARCH OFFSET(8) NUMBITS(4) [],
        ARCF OFFSET(4) NUMBITS(4) [],
        ARCA OFFSET(0) NUMBITS(4) [],
    ],
    pub AxiccW [
        EARC OFFSET(29) NUMBITS(1) [],
        AWCF OFFSET(24) NUMBITS(4) [],
        AWCD OFFSET(20) NUMBITS(4) [],
        AWCFD OFFSET(16) NUMBITS(4) [],
        ARCP OFFSET(12) NUMBITS(4) [],
        ARCH OFFSET(8) NUMBITS(4) [],
        ARCF OFFSET(4) NUMBITS(4) [],
        ARCA OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PaxicR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        ENZP OFFSET(28) NUMBITS(1) [],
        AXIPT OFFSET(27) NUMBITS(1) [],
        AXIPE OFFSET(26) NUMBITS(1) [],
        AAO OFFSET(25) NUMBITS(1) [],
        ECM OFFSET(24) NUMBITS(1) [],
        OTL OFFSET(20) NUMBITS(4) [],
        MARIDD OFFSET(16) NUMBITS(4) [],
        MARID OFFSET(12) NUMBITS(4) [],
        MAWIDD OFFSET(8) NUMBITS(4) [],
        MAWID OFFSET(4) NUMBITS(4) [],
        RESERVED1 OFFSET(2) NUMBITS(2) [],
        ADBW OFFSET(0) NUMBITS(2) [],
    ],
    pub PaxicW [
        ENZP OFFSET(28) NUMBITS(1) [],
        AXIPT OFFSET(27) NUMBITS(1) [],
        AXIPE OFFSET(26) NUMBITS(1) [],
        AAO OFFSET(25) NUMBITS(1) [],
        ECM OFFSET(24) NUMBITS(1) [],
        OTL OFFSET(20) NUMBITS(4) [],
        MARIDD OFFSET(16) NUMBITS(4) [],
        MARID OFFSET(12) NUMBITS(4) [],
        MAWIDD OFFSET(8) NUMBITS(4) [],
        MAWID OFFSET(4) NUMBITS(4) [],
        ADBW OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AxipcR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        EARP OFFSET(29) NUMBITS(1) [],
        EAWP OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(27) NUMBITS(1) [],
        AWPF OFFSET(24) NUMBITS(3) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        AWPD OFFSET(20) NUMBITS(3) [],
        RESERVED3 OFFSET(19) NUMBITS(1) [],
        AWPFD OFFSET(16) NUMBITS(3) [],
        RESERVED4 OFFSET(15) NUMBITS(1) [],
        ARPP OFFSET(12) NUMBITS(3) [],
        RESERVED5 OFFSET(11) NUMBITS(1) [],
        ARPH OFFSET(8) NUMBITS(3) [],
        RESERVED6 OFFSET(7) NUMBITS(1) [],
        ARPF OFFSET(4) NUMBITS(3) [],
        RESERVED7 OFFSET(3) NUMBITS(1) [],
        ARPD OFFSET(0) NUMBITS(3) [],
    ],
    pub AxipcW [
        EARP OFFSET(29) NUMBITS(1) [],
        EAWP OFFSET(28) NUMBITS(1) [],
        AWPF OFFSET(24) NUMBITS(3) [],
        AWPD OFFSET(20) NUMBITS(3) [],
        AWPFD OFFSET(16) NUMBITS(3) [],
        ARPP OFFSET(12) NUMBITS(3) [],
        ARPH OFFSET(8) NUMBITS(3) [],
        ARPF OFFSET(4) NUMBITS(3) [],
        ARPD OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PtcR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(27) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(17) [],
        ITM OFFSET(9) NUMBITS(1) [],
        ENBD OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(7) NUMBITS(1) [],
        RXWM OFFSET(0) NUMBITS(7) [],
    ],
    pub PtcW [
        ITM OFFSET(9) NUMBITS(1) [],
        ENBD OFFSET(8) NUMBITS(1) [],
        RXWM OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pts [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        TXSM OFFSET(4) NUMBITS(5) [],
        RXSM OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Plc [
        PMPRA OFFSET(27) NUMBITS(5) [],
        POE OFFSET(26) NUMBITS(1) [],
        PRT OFFSET(16) NUMBITS(10) [],
        AIR OFFSET(8) NUMBITS(8) [],
        EPNRT OFFSET(7) NUMBITS(1) [],
        S4A OFFSET(6) NUMBITS(1) [],
        RXSE OFFSET(5) NUMBITS(1) [],
        TXSE OFFSET(4) NUMBITS(1) [],
        TXPJ OFFSET(3) NUMBITS(1) [],
        TXC OFFSET(2) NUMBITS(1) [],
        RXBC OFFSET(1) NUMBITS(1) [],
        TXBC OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Plc1R [
        RESERVED0 OFFSET(7) NUMBITS(25) [],
        CD OFFSET(6) NUMBITS(1) [],
        POS OFFSET(0) NUMBITS(6) [],
    ],
    pub Plc1W [
        CD OFFSET(6) NUMBITS(1) [],
        POS OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pls [
        SVN OFFSET(28) NUMBITS(4) [],
        DMB OFFSET(24) NUMBITS(4) [],
        DMBW OFFSET(20) NUMBITS(4) [],
        SRRN OFFSET(12) NUMBITS(8) [],
        RESERVED0 OFFSET(6) NUMBITS(6) [],
        LLS OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pls1 [
        KCEC OFFSET(24) NUMBITS(8) [],
        PIEC OFFSET(16) NUMBITS(8) [],
        CEC OFFSET(8) NUMBITS(8) [],
        DEC OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcmdcR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        TSVIE OFFSET(29) NUMBITS(1) [],
        TSVI OFFSET(28) NUMBITS(1) [],
        TSVT OFFSET(12) NUMBITS(16) [],
        RESERVED1 OFFSET(2) NUMBITS(10) [],
        ETLL OFFSET(1) NUMBITS(1) [],
        ETLLB OFFSET(0) NUMBITS(1) [],
    ],
    pub PcmdcW [
        TSVIE OFFSET(29) NUMBITS(1) [],
        TSVI OFFSET(28) NUMBITS(1) [],
        ETLL OFFSET(1) NUMBITS(1) [],
        ETLLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PpcsR [
        PHYCE OFFSET(30) NUMBITS(2) [],
        PHYDE OFFSET(28) NUMBITS(2) [],
        PHYKC OFFSET(27) NUMBITS(1) [],
        PHYD OFFSET(11) NUMBITS(16) [],
        CCAC OFFSET(10) NUMBITS(1) [],
        CCA OFFSET(5) NUMBITS(5) [],
        PCTRLS OFFSET(0) NUMBITS(5) [],
    ],
    pub PpcsW [
        CCAC OFFSET(10) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ams [
        RESERVED0 OFFSET(12) NUMBITS(20) [],
        AMS1 OFFSET(7) NUMBITS(5) [],
        AMS0 OFFSET(2) NUMBITS(5) [],
        WAS OFFSET(1) NUMBITS(1) [],
        RAS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub TcrR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        TPS OFFSET(0) NUMBITS(13) [],
    ],
    pub TcrW [
        TPS OFFSET(0) NUMBITS(13) [],
    ]
];

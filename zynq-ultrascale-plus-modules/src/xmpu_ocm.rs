// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// OCM XMPU Protection Unit, OCM Memory Protection Unit Configuration
pub static mut OCM_XMPU_CFG: *mut Registers = 0xffa70000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Control and Implementation.
    pub ctrl: Aliased<u32, CtrlR::Register, CtrlW::Register>,
    /// Error Status, Reg 1.
    pub err_status1: ReadOnly<u32, ErrStatus1::Register>,
    /// Error Status, Reg 2.
    pub err_status2: ReadOnly<u32, ErrStatus2::Register>,
    /// Poison Configuration.
    pub poison: Aliased<u32, PoisonR::Register, PoisonW::Register>,
    /// Interrupt Status and Clear.
    pub isr: Aliased<u32, IsrR::Register, IsrW::Register>,
    /// Interrupt Mask.
    pub imr: ReadOnly<u32, Imr::Register>,
    /// Interrupt Enable.
    pub ien: Aliased<u32, IenR::Register, IenW::Register>,
    /// Interrupt Disable.
    pub ids: Aliased<u32, IdsR::Register, IdsW::Register>,
    /// Register Write Lock.
    pub lock: ReadWrite<u8, Lock::Register>,
    _padding33: [u8; 223],
    /// Region 0 Start Address.
    pub r00_start: Aliased<u32, R00StartR::Register, R00StartW::Register>,
    /// Region 0 End Address.
    pub r00_end: Aliased<u32, R00EndR::Register, R00EndW::Register>,
    /// Region 0 Master ID.
    pub r00_master: Aliased<u32, R00MasterR::Register, R00MasterW::Register>,
    /// Region 0 Configuration.
    pub r00_config: Aliased<u32, R00ConfigR::Register, R00ConfigW::Register>,
    /// Region 1 Start Address.
    pub r01_start: Aliased<u32, R01StartR::Register, R01StartW::Register>,
    /// Region 1 End Address.
    pub r01_end: Aliased<u32, R01EndR::Register, R01EndW::Register>,
    /// Region 1 Master ID.
    pub r01_master: Aliased<u32, R01MasterR::Register, R01MasterW::Register>,
    /// Region 1 Configuration.
    pub r01_config: Aliased<u32, R01ConfigR::Register, R01ConfigW::Register>,
    /// Region 2 Start Address.
    pub r02_start: Aliased<u32, R02StartR::Register, R02StartW::Register>,
    /// Region 2 End Address.
    pub r02_end: Aliased<u32, R02EndR::Register, R02EndW::Register>,
    /// Region 2 Master ID.
    pub r02_master: Aliased<u32, R02MasterR::Register, R02MasterW::Register>,
    /// Region 2 Configuration.
    pub r02_config: Aliased<u32, R02ConfigR::Register, R02ConfigW::Register>,
    /// Region 3 Start Address.
    pub r03_start: Aliased<u32, R03StartR::Register, R03StartW::Register>,
    /// Region 3 End Address.
    pub r03_end: Aliased<u32, R03EndR::Register, R03EndW::Register>,
    /// Region 3 Master ID.
    pub r03_master: Aliased<u32, R03MasterR::Register, R03MasterW::Register>,
    /// Region 3 Configuration.
    pub r03_config: Aliased<u32, R03ConfigR::Register, R03ConfigW::Register>,
    /// Region 4 Start Address.
    pub r04_start: Aliased<u32, R04StartR::Register, R04StartW::Register>,
    /// Region 4 End Address.
    pub r04_end: Aliased<u32, R04EndR::Register, R04EndW::Register>,
    /// Region 4 Master ID.
    pub r04_master: Aliased<u32, R04MasterR::Register, R04MasterW::Register>,
    /// Region 4 Configuration.
    pub r04_config: Aliased<u32, R04ConfigR::Register, R04ConfigW::Register>,
    /// Region 5 Start Address.
    pub r05_start: Aliased<u32, R05StartR::Register, R05StartW::Register>,
    /// Region 5 End Address.
    pub r05_end: Aliased<u32, R05EndR::Register, R05EndW::Register>,
    /// Region 5 Master ID.
    pub r05_master: Aliased<u32, R05MasterR::Register, R05MasterW::Register>,
    /// Region 5 Configuration.
    pub r05_config: Aliased<u32, R05ConfigR::Register, R05ConfigW::Register>,
    /// Region 6 Start Address.
    pub r06_start: Aliased<u32, R06StartR::Register, R06StartW::Register>,
    /// Region 6 End Address.
    pub r06_end: Aliased<u32, R06EndR::Register, R06EndW::Register>,
    /// Region 6 Master ID.
    pub r06_master: Aliased<u32, R06MasterR::Register, R06MasterW::Register>,
    /// Region 6 Configuration.
    pub r06_config: Aliased<u32, R06ConfigR::Register, R06ConfigW::Register>,
    /// Region 7 Start Address.
    pub r07_start: Aliased<u32, R07StartR::Register, R07StartW::Register>,
    /// Region 7 End Address.
    pub r07_end: Aliased<u32, R07EndR::Register, R07EndW::Register>,
    /// Region 7 Master ID.
    pub r07_master: Aliased<u32, R07MasterR::Register, R07MasterW::Register>,
    /// Region 7 Configuration.
    pub r07_config: Aliased<u32, R07ConfigR::Register, R07ConfigW::Register>,
    /// Region 8 Start Address.
    pub r08_start: Aliased<u32, R08StartR::Register, R08StartW::Register>,
    /// Region 8 End Address.
    pub r08_end: Aliased<u32, R08EndR::Register, R08EndW::Register>,
    /// Region 8 Master ID.
    pub r08_master: Aliased<u32, R08MasterR::Register, R08MasterW::Register>,
    /// Region 8 Configuration.
    pub r08_config: Aliased<u32, R08ConfigR::Register, R08ConfigW::Register>,
    /// Region 9 Start Address.
    pub r09_start: Aliased<u32, R09StartR::Register, R09StartW::Register>,
    /// Region 9 End Address.
    pub r09_end: Aliased<u32, R09EndR::Register, R09EndW::Register>,
    /// Region 9 Master ID.
    pub r09_master: Aliased<u32, R09MasterR::Register, R09MasterW::Register>,
    /// Region 9 Configuration.
    pub r09_config: Aliased<u32, R09ConfigR::Register, R09ConfigW::Register>,
    /// Region 10 Start Address.
    pub r10_start: Aliased<u32, R10StartR::Register, R10StartW::Register>,
    /// Region 10 End Address.
    pub r10_end: Aliased<u32, R10EndR::Register, R10EndW::Register>,
    /// Region 10 Master ID.
    pub r10_master: Aliased<u32, R10MasterR::Register, R10MasterW::Register>,
    /// Region 10 Configuration.
    pub r10_config: Aliased<u32, R10ConfigR::Register, R10ConfigW::Register>,
    /// Region 11 Start Address.
    pub r11_start: Aliased<u32, R11StartR::Register, R11StartW::Register>,
    /// Region 11 End Address.
    pub r11_end: Aliased<u32, R11EndR::Register, R11EndW::Register>,
    /// Region 11 Master ID.
    pub r11_master: Aliased<u32, R11MasterR::Register, R11MasterW::Register>,
    /// Region 11 Configuration.
    pub r11_config: Aliased<u32, R11ConfigR::Register, R11ConfigW::Register>,
    /// Region 12 Start Address.
    pub r12_start: Aliased<u32, R12StartR::Register, R12StartW::Register>,
    /// Region 12 End Address.
    pub r12_end: Aliased<u32, R12EndR::Register, R12EndW::Register>,
    /// Region 12 Master ID.
    pub r12_master: Aliased<u32, R12MasterR::Register, R12MasterW::Register>,
    /// Region 12 Configuration.
    pub r12_config: Aliased<u32, R12ConfigR::Register, R12ConfigW::Register>,
    /// Region 13 Start Address.
    pub r13_start: Aliased<u32, R13StartR::Register, R13StartW::Register>,
    /// Region 13 End Address.
    pub r13_end: Aliased<u32, R13EndR::Register, R13EndW::Register>,
    /// Region 13 Master ID.
    pub r13_master: Aliased<u32, R13MasterR::Register, R13MasterW::Register>,
    /// Region 13 Configuration.
    pub r13_config: Aliased<u32, R13ConfigR::Register, R13ConfigW::Register>,
    /// Region 14 Start Address.
    pub r14_start: Aliased<u32, R14StartR::Register, R14StartW::Register>,
    /// Region 14 End Address.
    pub r14_end: Aliased<u32, R14EndR::Register, R14EndW::Register>,
    /// Region 14 Master ID.
    pub r14_master: Aliased<u32, R14MasterR::Register, R14MasterW::Register>,
    /// Region 14 Configuration.
    pub r14_config: Aliased<u32, R14ConfigR::Register, R14ConfigW::Register>,
    /// Region 15 Start Address.
    pub r15_start: Aliased<u32, R15StartR::Register, R15StartW::Register>,
    /// Region 15 End Address.
    pub r15_end: Aliased<u32, R15EndR::Register, R15EndW::Register>,
    /// Region 15 Master ID.
    pub r15_master: Aliased<u32, R15MasterR::Register, R15MasterW::Register>,
    /// Region 15 Configuration.
    pub r15_config: Aliased<u32, R15ConfigR::Register, R15ConfigW::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub CtrlR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        ALIGNCFG OFFSET(3) NUMBITS(1) [],
        POISONCFG OFFSET(2) NUMBITS(1) [],
        DEFWRALLOWED OFFSET(1) NUMBITS(1) [],
        DEFRDALLOWED OFFSET(0) NUMBITS(1) [],
    ],
    pub CtrlW [
        POISONCFG OFFSET(2) NUMBITS(1) [],
        DEFWRALLOWED OFFSET(1) NUMBITS(1) [],
        DEFRDALLOWED OFFSET(0) NUMBITS(1) [],
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
    pub PoisonR [
        RESERVED0 OFFSET(21) NUMBITS(11) [],
        ATTRIB OFFSET(20) NUMBITS(1) [],
        BASE OFFSET(0) NUMBITS(20) [],
    ],
    pub PoisonW [
        ATTRIB OFFSET(20) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IsrR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        SECURITYVIO OFFSET(3) NUMBITS(1) [],
        WRPERMVIO OFFSET(2) NUMBITS(1) [],
        RDPERMVIO OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ],
    pub IsrW [
        SECURITYVIO OFFSET(3) NUMBITS(1) [],
        WRPERMVIO OFFSET(2) NUMBITS(1) [],
        RDPERMVIO OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imr [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        SECURITYVIO OFFSET(3) NUMBITS(1) [],
        WRPERMVIO OFFSET(2) NUMBITS(1) [],
        RDPERMVIO OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IenR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
    ],
    pub IenW [
        SECURITYVIO OFFSET(3) NUMBITS(1) [],
        WRPERMVIO OFFSET(2) NUMBITS(1) [],
        RDPERMVIO OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IdsR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
    ],
    pub IdsW [
        SECURITYVIO OFFSET(3) NUMBITS(1) [],
        WRPERMVIO OFFSET(2) NUMBITS(1) [],
        RDPERMVIO OFFSET(1) NUMBITS(1) [],
        INV_APB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub Lock [
        REGWRDIS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R00StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R00StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R00EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R00EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R00MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R00MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R00ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R00ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R01StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R01StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R01EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R01EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R01MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R01MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R01ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R01ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R02StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R02StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R02EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R02EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R02MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R02MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R02ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R02ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R03StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R03StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R03EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R03EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R03MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R03MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R03ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R03ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R04StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R04StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R04EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R04EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R04MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R04MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R04ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R04ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R05StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R05StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R05EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R05EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R05MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R05MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R05ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R05ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R06StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R06StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R06EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R06EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R06MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R06MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R06ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R06ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R07StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R07StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R07EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R07EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R07MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R07MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R07ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R07ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R08StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R08StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R08EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R08EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R08MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R08MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R08ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R08ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R09StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R09StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R09EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R09EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R09MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R09MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R09ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R09ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R10StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R10StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R10EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R10EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R10MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R10MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R10ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R10ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R11StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R11StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R11EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R11EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R11MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R11MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R11ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R11ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R12StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R12StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R12EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R12EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R12MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R12MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R12ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R12ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R13StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R13StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R13EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R13EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R13MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R13MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R13ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R13ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R14StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R14StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R14EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R14EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R14MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R14MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R14ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R14ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R15StartR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R15StartW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R15EndR [
        RESERVED0 OFFSET(20) NUMBITS(12) [],
        ADDR OFFSET(0) NUMBITS(20) [],
    ],
    pub R15EndW [
        ADDR OFFSET(0) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R15MasterR [
        RESERVED0 OFFSET(26) NUMBITS(6) [],
        MASK OFFSET(16) NUMBITS(10) [],
        RESERVED1 OFFSET(10) NUMBITS(6) [],
        ID OFFSET(0) NUMBITS(10) [],
    ],
    pub R15MasterW [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub R15ConfigR [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub R15ConfigW [
        NSCHECKTYPE OFFSET(4) NUMBITS(1) [],
        REGIONNS OFFSET(3) NUMBITS(1) [],
        WRALLOWED OFFSET(2) NUMBITS(1) [],
        RDALLOWED OFFSET(1) NUMBITS(1) [],
        ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];

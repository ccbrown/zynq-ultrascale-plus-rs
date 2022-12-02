// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// APU 0 Cross Trigger Interface, A53 Cross Trigger Interface with to/from broadcast
pub static mut CORESIGHT_A53_CTI_0: *mut Registers = 0xfec20000 as *mut Registers;
register_structs! {
    pub Registers {
        /// The CTI Control Register enables the CTI.
        (0x00000000 => pub cticontrol: ReadWrite<u32, Cticontrol::Register>),
        (0x00000004 => _padding4),
        /// The CTI Interrupt Acknowledge Register is write-only. Any bits written as a 1 cause the ctitrigout output signal to be acknowledged. The acknowledgement is cleared when MAPTRIGOUT is deactivated. This register is used when the ctitrigout is used as a sticky output, that is, no hardware acknowledge is supplied, and a software acknowledge is required.
        (0x00000010 => pub ctiintack: WriteOnly<u32, Ctiintack::Register>),
        /// The CTI Application Trigger Set Register is read/write. A write to this register causes a channel event to be raised, corresponding to the bit written to.
        (0x00000014 => pub ctiappset: WriteOnly<u32, Ctiappset::Register>),
        /// The CTI Interrupt Acknowledge Register is write-only. A write to this register causes a channel event to be cleared, corresponding to the bit written to.
        (0x00000018 => pub ctiappclear: WriteOnly<u32, Ctiappclear::Register>),
        /// The CTI Application Pulse Register is write-only. A write to this register causes a channel event pulse, one cticlk period, to be generated, corresponding to the bit written to. The pulse external to the ECT can be extended to multi-cycle by the handshaking interface circuits. This register clears itself immediately, so it can be repeatedly written to without software having to clear it.
        (0x0000001c => pub ctiapppulse: WriteOnly<u32, Ctiapppulse::Register>),
        /// The CTI Trigger 0 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
        (0x00000020 => pub ctiinen0: ReadWrite<u32, Ctiinen0::Register>),
        /// The CTI Trigger 1 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
        (0x00000024 => pub ctiinen1: ReadWrite<u32, Ctiinen1::Register>),
        /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
        (0x00000028 => pub ctiinen2: ReadWrite<u32, Ctiinen2::Register>),
        /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
        (0x0000002c => pub ctiinen3: ReadWrite<u32, Ctiinen3::Register>),
        /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
        (0x00000030 => pub ctiinen4: ReadWrite<u32, Ctiinen4::Register>),
        /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
        (0x00000034 => pub ctiinen5: ReadWrite<u32, Ctiinen5::Register>),
        /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
        (0x00000038 => pub ctiinen6: ReadWrite<u32, Ctiinen6::Register>),
        /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
        (0x0000003c => pub ctiinen7: ReadWrite<u32, Ctiinen7::Register>),
        (0x00000040 => _padding64),
        /// The CTI Channel to Trigger 0 Enable Registers define which channels can generate a ctitrigout[0] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
        (0x000000a0 => pub ctiouten0: ReadWrite<u32, Ctiouten0::Register>),
        /// The CTI Channel to Trigger 1 Enable Registers define which channels can generate a ctitrigout[1] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
        (0x000000a4 => pub ctiouten1: ReadWrite<u32, Ctiouten1::Register>),
        /// The CTI Channel to Trigger 2 Enable Registers define which channels can generate a ctitrigout[2] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
        (0x000000a8 => pub ctiouten2: ReadWrite<u32, Ctiouten2::Register>),
        /// The CTI Channel to Trigger 3 Enable Registers define which channels can generate a ctitrigout[3] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
        (0x000000ac => pub ctiouten3: ReadWrite<u32, Ctiouten3::Register>),
        /// The CTI Channel to Trigger 4 Enable Registers define which channels can generate a ctitrigout[4] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
        (0x000000b0 => pub ctiouten4: ReadWrite<u32, Ctiouten4::Register>),
        /// The CTI Channel to Trigger 5 Enable Registers define which channels can generate a ctitrigout[5] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
        (0x000000b4 => pub ctiouten5: ReadWrite<u32, Ctiouten5::Register>),
        /// The CTI Channel to Trigger 6 Enable Registers define which channels can generate a ctitrigout[6] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
        (0x000000b8 => pub ctiouten6: ReadWrite<u32, Ctiouten6::Register>),
        /// The CTI Channel to Trigger 7 Enable Registers define which channels can generate a ctitrigout[7] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
        (0x000000bc => pub ctiouten7: ReadWrite<u32, Ctiouten7::Register>),
        (0x000000c0 => _padding192),
        /// The CTI Trigger In Status Register provides the status of the ctitrigin inputs.
        (0x00000130 => pub ctitriginstatus: ReadOnly<u32, Ctitriginstatus::Register>),
        /// The CTI Trigger Out Status Register provides the status of the ctitrigout outputs.
        (0x00000134 => pub ctitrigoutstatus: ReadOnly<u32, Ctitrigoutstatus::Register>),
        /// The CTI Channel In Status Register provides the status of the ctichin inputs.
        (0x00000138 => pub ctichinstatus: ReadOnly<u32, Ctichinstatus::Register>),
        /// The CTI Channel Out Status Register provides the status of the CTI ctichout outputs.
        (0x0000013c => pub ctichoutstatus: ReadOnly<u32, Ctichoutstatus::Register>),
        /// The Gate Enable Register prevents the channels from propagating through the CTM to other CTIs. This enables local cross-triggering, for example for causing an interrupt when the ETM trigger occurs. It can be used effectively with CTIAPPSET, CTIAPPCLEAR, and CTIAPPPULSE for asserting trigger outputs by asserting channels, without affecting the rest of the system. On reset, this register is 0xF, and channel propagation is enabled.
        (0x00000140 => pub ctigate: ReadWrite<u32, Ctigate::Register>),
        /// Implementation-defined ASIC control, value written to the register is output on asicctl[7:0].
        (0x00000144 => pub asicctl: ReadWrite<u32, Asicctl::Register>),
        (0x00000148 => _padding328),
        /// CTI Integration mode Control Register
        (0x00000f00 => pub itctrl: ReadWrite<u32, Itctrl::Register>),
        (0x00000f04 => _padding3844),
        /// CTI Claim Set
        (0x00000fa0 => pub claimset: ReadWrite<u32>),
        /// CTI Claim Clear
        (0x00000fa4 => pub claimclr: ReadWrite<u32>),
        /// CTI Device Affinity Register 0
        (0x00000fa8 => pub devaff0: ReadOnly<u32>),
        /// CTI Device Affinity Register 1
        (0x00000fac => pub devaff1: ReadOnly<u32>),
        /// CTI Lock Access Register
        (0x00000fb0 => pub lar: WriteOnly<u32>),
        /// CTI Lock Status Register
        (0x00000fb4 => pub lsr: ReadOnly<u32, Lsr::Register>),
        /// CTI Authentication Status Register
        (0x00000fb8 => pub authstatus: ReadOnly<u32, Authstatus::Register>),
        /// CTI Device Architecture Register
        (0x00000fbc => pub devarch: ReadOnly<u32, Devarch::Register>),
        (0x00000fc0 => _padding4032),
        /// CTI Device ID Register 0
        (0x00000fc8 => pub devid: ReadOnly<u32, Devid::Register>),
        /// CTI Device Type Register
        (0x00000fcc => pub devtype: ReadOnly<u32, Devtype::Register>),
        /// CTI Peripheral Identification Register 4
        (0x00000fd0 => pub pidr4: ReadOnly<u32, Pidr4::Register>),
        /// CTI Peripheral Identification Register 5
        (0x00000fd4 => pub pidr5: ReadOnly<u32>),
        /// CTI Peripheral Identification Register 6
        (0x00000fd8 => pub pidr6: ReadOnly<u32>),
        /// CTI Peripheral Identification Register 7
        (0x00000fdc => pub pidr7: ReadOnly<u32>),
        /// CTI Peripheral Identification Register 0
        (0x00000fe0 => pub pidr0: ReadOnly<u32, Pidr0::Register>),
        /// CTI Peripheral Identification Register 1
        (0x00000fe4 => pub pidr1: ReadOnly<u32, Pidr1::Register>),
        /// CTI Peripheral Identification Register 2
        (0x00000fe8 => pub pidr2: ReadOnly<u32, Pidr2::Register>),
        /// CTI Peripheral Identification Register 3
        (0x00000fec => pub pidr3: ReadOnly<u32, Pidr3::Register>),
        /// CTI Component Identification Register 0
        (0x00000ff0 => pub cidr0: ReadOnly<u32, Cidr0::Register>),
        /// CTI Component Identification Register 1
        (0x00000ff4 => pub cidr1: ReadOnly<u32, Cidr1::Register>),
        /// CTI Component Identification Register 2
        (0x00000ff8 => pub cidr2: ReadOnly<u32, Cidr2::Register>),
        /// CTI Component Identification Register 3
        (0x00000ffc => pub cidr3: ReadOnly<u32, Cidr3::Register>),
        (0x00001000 => @END),
    }
}
register_bitfields! [
    u32,
    pub Cticontrol [
        GLBEN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiintack [
        INTACK OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiappset [
        APPSET OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiappclear [
        APPCLEAR OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiapppulse [
        APPULSE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiinen0 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiinen1 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiinen2 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiinen3 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiinen4 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiinen5 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiinen6 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiinen7 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiouten0 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiouten1 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiouten2 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiouten3 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiouten4 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiouten5 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiouten6 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctiouten7 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctitriginstatus [
        TRIGINSTATUS OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctitrigoutstatus [
        TRIGOUTSTATUS OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctichinstatus [
        CTICHINSTATUS OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctichoutstatus [
        CTICHOUTSTATUS OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Ctigate [
        CTIGATEEN OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Asicctl [
        ASICCTL OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Itctrl [
        IME OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Lsr [
        NTT OFFSET(2) NUMBITS(1) [],
        SLK OFFSET(1) NUMBITS(1) [],
        SLI OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Authstatus [
        NSNID OFFSET(2) NUMBITS(2) [],
        NSID OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub Devarch [
        ARCHITECT OFFSET(21) NUMBITS(11) [],
        PRESENT OFFSET(20) NUMBITS(1) [],
        REVISION OFFSET(16) NUMBITS(4) [],
        ARCHID OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub Devid [
        INOUT OFFSET(24) NUMBITS(2) [],
        NUMCHAN OFFSET(16) NUMBITS(6) [],
        NUMTRIG OFFSET(8) NUMBITS(6) [],
        EXTMUXNUM OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u32,
    pub Devtype [
        SUB OFFSET(4) NUMBITS(4) [],
        MAJOR OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr4 [
        SIZE OFFSET(4) NUMBITS(4) [],
        DES_2 OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr0 [
        PART_0 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr1 [
        DES_0 OFFSET(4) NUMBITS(4) [],
        PART_1 OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC OFFSET(3) NUMBITS(1) [],
        DES_1 OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub Pidr3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        CMOD OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr0 [
        PRMBL_0 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr1 [
        CLASS OFFSET(4) NUMBITS(4) [],
        PRMBL_1 OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr2 [
        PRMBL_2 OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Cidr3 [
        PRMBL_3 OFFSET(0) NUMBITS(8) [],
    ]
];

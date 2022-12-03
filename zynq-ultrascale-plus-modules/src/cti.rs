// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// CoreSight Cross Trigger Interface, R5 Cross Trigger Interface with to/from broadcast
pub static mut CORESIGHT_R5_CTI_0: *mut Registers = 0xfebf8000 as *mut Registers;
/// CoreSight Cross Trigger Interface, R5 Cross Trigger Interface with to/from broadcast
pub static mut CORESIGHT_R5_CTI_1: *mut Registers = 0xfebf9000 as *mut Registers;
/// CoreSight Cross Trigger Interface, SoC Cross Trigger Interface with to/from broadcast
pub static mut CORESIGHT_SOC_CTI_0: *mut Registers = 0xfe990000 as *mut Registers;
/// CoreSight Cross Trigger Interface, SoC Cross Trigger Interface with to/from broadcast
pub static mut CORESIGHT_SOC_CTI_1: *mut Registers = 0xfe9a0000 as *mut Registers;
/// CoreSight Cross Trigger Interface, SoC Cross Trigger Interface with to/from broadcast
pub static mut CORESIGHT_SOC_CTI_2: *mut Registers = 0xfe9b0000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// The CTI Control Register enables the CTI.
    pub cticontrol: ReadWrite<u32, Cticontrol::Register>,
    _padding4: [u8; 12],
    /// The CTI Interrupt Acknowledge Register is write-only. Any bits written as a 1 cause the ctitrigout output signal to be acknowledged. The acknowledgement is cleared when MAPTRIGOUT is deactivated. This register is used when the ctitrigout is used as a sticky output, that is, no hardware acknowledge is supplied, and a software acknowledge is required.
    pub ctiintack: WriteOnly<u32, Ctiintack::Register>,
    /// The CTI Application Trigger Set Register is read/write. A write to this register causes a channel event to be raised, corresponding to the bit written to.
    pub ctiappset: WriteOnly<u32, Ctiappset::Register>,
    /// The CTI Interrupt Acknowledge Register is write-only. A write to this register causes a channel event to be cleared, corresponding to the bit written to.
    pub ctiappclear: WriteOnly<u32, Ctiappclear::Register>,
    /// The CTI Application Pulse Register is write-only. A write to this register causes a channel event pulse, one cticlk period, to be generated, corresponding to the bit written to. The pulse external to the ECT can be extended to multi-cycle by the handshaking interface circuits. This register clears itself immediately, so it can be repeatedly written to without software having to clear it.
    pub ctiapppulse: WriteOnly<u32, Ctiapppulse::Register>,
    /// The CTI Trigger 0 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
    pub ctiinen0: ReadWrite<u32, Ctiinen0::Register>,
    /// The CTI Trigger 1 to Channel Enable Register enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
    pub ctiinen1: ReadWrite<u32, Ctiinen1::Register>,
    /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
    pub ctiinen2: ReadWrite<u32, Ctiinen2::Register>,
    /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
    pub ctiinen3: ReadWrite<u32, Ctiinen3::Register>,
    /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
    pub ctiinen4: ReadWrite<u32, Ctiinen4::Register>,
    /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
    pub ctiinen5: ReadWrite<u32, Ctiinen5::Register>,
    /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
    pub ctiinen6: ReadWrite<u32, Ctiinen6::Register>,
    /// The CTI Trigger to Channel Enable Register 0 enables the signalling of an event on CTM channels when the core issues a trigger, ctitrigin, to the CTI. Within this register there is one bit for each of the four channels implemented. This register does not affect the application trigger operations.
    pub ctiinen7: ReadWrite<u32, Ctiinen7::Register>,
    _padding64: [u8; 96],
    /// The CTI Channel to Trigger 0 Enable Registers define which channels can generate a ctitrigout[0] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
    pub ctiouten0: ReadWrite<u32, Ctiouten0::Register>,
    /// The CTI Channel to Trigger 1 Enable Registers define which channels can generate a ctitrigout[1] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
    pub ctiouten1: ReadWrite<u32, Ctiouten1::Register>,
    /// The CTI Channel to Trigger 2 Enable Registers define which channels can generate a ctitrigout[2] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
    pub ctiouten2: ReadWrite<u32, Ctiouten2::Register>,
    /// The CTI Channel to Trigger 3 Enable Registers define which channels can generate a ctitrigout[3] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
    pub ctiouten3: ReadWrite<u32, Ctiouten3::Register>,
    /// The CTI Channel to Trigger 4 Enable Registers define which channels can generate a ctitrigout[4] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
    pub ctiouten4: ReadWrite<u32, Ctiouten4::Register>,
    /// The CTI Channel to Trigger 5 Enable Registers define which channels can generate a ctitrigout[5] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
    pub ctiouten5: ReadWrite<u32, Ctiouten5::Register>,
    /// The CTI Channel to Trigger 6 Enable Registers define which channels can generate a ctitrigout[6] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
    pub ctiouten6: ReadWrite<u32, Ctiouten6::Register>,
    /// The CTI Channel to Trigger 7 Enable Registers define which channels can generate a ctitrigout[7] output. Within this register there is one bit for each of the four channels implemented. This register affects the mapping from application trigger to trigger outputs.
    pub ctiouten7: ReadWrite<u32, Ctiouten7::Register>,
    _padding192: [u8; 112],
    /// The CTI Trigger In Status Register provides the status of the ctitrigin inputs.
    pub ctitriginstatus: ReadOnly<u32, Ctitriginstatus::Register>,
    /// The CTI Trigger Out Status Register provides the status of the ctitrigout outputs.
    pub ctitrigoutstatus: ReadOnly<u32, Ctitrigoutstatus::Register>,
    /// The CTI Channel In Status Register provides the status of the ctichin inputs.
    pub ctichinstatus: ReadOnly<u32, Ctichinstatus::Register>,
    /// The CTI Channel Out Status Register provides the status of the CTI ctichout outputs.
    pub ctichoutstatus: ReadOnly<u32, Ctichoutstatus::Register>,
    /// The Gate Enable Register prevents the channels from propagating through the CTM to other CTIs. This enables local cross-triggering, for example for causing an interrupt when the ETM trigger occurs. It can be used effectively with CTIAPPSET, CTIAPPCLEAR, and CTIAPPPULSE for asserting trigger outputs by asserting channels, without affecting the rest of the system. On reset, this register is 0xF, and channel propagation is enabled.
    pub ctigate: ReadWrite<u32, Ctigate::Register>,
    /// Implementation-defined ASIC control, value written to the register is output on asicctl[7:0].
    pub asicctl: ReadWrite<u32, Asicctl::Register>,
    _padding328: [u8; 3476],
    /// This register is a write-only register. It can be used to set the value of the CTCHINACK outputs.
    pub itchinack: WriteOnly<u32, Itchinack::Register>,
    /// This register is a write-only register. It can be used to set the value of the CTTRIGINACK outputs.
    pub ittriginack: WriteOnly<u32, Ittriginack::Register>,
    /// This register is a write-only register. It can be used to set the value of the CTCHOUT outputs.
    pub itchout: WriteOnly<u32, Itchout::Register>,
    /// This register is a write-only register. It can be used to set the value of the CTTRIGOUT outputs.
    pub ittrigout: WriteOnly<u32, Ittrigout::Register>,
    /// This register is a read-only register. It can be used to read the values of the CTCHOUTACK inputs.
    pub itchoutack: ReadOnly<u32, Itchoutack::Register>,
    /// This register is a read-only register. It can be used to read the values of the CTTRIGOUTACK inputs.
    pub ittrigoutack: ReadOnly<u32, Ittrigoutack::Register>,
    /// This register is a read-only register. It can be used to read the values of the CTCHIN inputs.
    pub itchin: ReadOnly<u32, Itchin::Register>,
    /// This register is a read-only register. It can be used to read the values of the CTTRIGIN inputs.
    pub ittrigin: ReadOnly<u32, Ittrigin::Register>,
    _padding3836: [u8; 4],
    /// This register is used to enable topology detection. For more information see the CoreSight Architecture Specification. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purpose of integration testing and topology solving. Note: When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection.
    pub itctrl: ReadWrite<u32, Itctrl::Register>,
    _padding3844: [u8; 156],
    /// This is used in conjunction with Claim Tag Clear Register, CLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read.
    pub claimset: ReadWrite<u32, Claimset::Register>,
    /// This register is used in conjunction with Claim Tag Set Register, CLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read.
    pub claimclr: ReadWrite<u32, Claimclr::Register>,
    _padding4008: [u8; 8],
    /// This is used to enable write access to device registers. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. A debugger does not have to unlock the component in order to write and modify the registers in the component.
    pub lar: WriteOnly<u32>,
    /// This indicates the status of the Lock control mechanism. This lock prevents accidental writes by code under debug. When locked, write access is blocked to all registers, except the Lock Access Register.External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. This register reads as 0 when read from an external debugger (paddrdbg31 = 1).
    pub lsr: ReadOnly<u32, Lsr::Register>,
    /// Reports what functionality is currently permitted by the authentication interface.
    pub authstatus: ReadOnly<u32, Authstatus::Register>,
    _padding4028: [u8; 12],
    /// This register indicates the capabilities of the CTI.
    pub devid: ReadOnly<u32, Devid::Register>,
    /// It provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information.
    pub devtype: ReadOnly<u32, Devtype::Register>,
    /// Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.
    pub pidr4: ReadOnly<u32, Pidr4::Register>,
    /// Reserved
    pub pidr5: ReadOnly<u32>,
    /// Reserved
    pub pidr6: ReadOnly<u32>,
    /// Reserved
    pub pidr7: ReadOnly<u32>,
    /// Part of the set of Peripheral Identification registers. Contains part of the designer specific part number.
    pub pidr0: ReadOnly<u32, Pidr0::Register>,
    /// Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity.
    pub pidr1: ReadOnly<u32, Pidr1::Register>,
    /// Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.
    pub pidr2: ReadOnly<u32, Pidr2::Register>,
    /// Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields.
    pub pidr3: ReadOnly<u32, Pidr3::Register>,
    /// A component identification register, that indicates that the identification registers are present.
    pub cidr0: ReadOnly<u32, Cidr0::Register>,
    /// A component identification register, that indicates that the identification registers are present. This register also indicates the component class.
    pub cidr1: ReadOnly<u32, Cidr1::Register>,
    /// A component identification register, that indicates that the identification registers are present.
    pub cidr2: ReadOnly<u32, Cidr2::Register>,
    /// A component identification register, that indicates that the identification registers are present.
    pub cidr3: ReadOnly<u32, Cidr3::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Cticontrol [
        GLBEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiintack [
        INTACK OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiappset [
        APPSET OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiappclear [
        APPCLEAR OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiapppulse [
        APPULSE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiinen0 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiinen1 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiinen2 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiinen3 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiinen4 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiinen5 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiinen6 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiinen7 [
        TRIGINEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiouten0 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiouten1 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiouten2 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiouten3 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiouten4 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiouten5 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiouten6 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctiouten7 [
        TRIGOUTEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctitriginstatus [
        TRIGINSTATUS OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctitrigoutstatus [
        TRIGOUTSTATUS OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctichinstatus [
        CTICHINSTATUS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctichoutstatus [
        CTICHOUTSTATUS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctigate [
        CTIGATEEN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Asicctl [
        ASICCTL OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itchinack [
        CTCHINACK OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ittriginack [
        CTTRIGINACK OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itchout [
        CTCHOUT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ittrigout [
        CTTRIGOUT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itchoutack [
        CTCHOUTACK OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ittrigoutack [
        CTTRIGOUTACK OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itchin [
        CTCHIN OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ittrigin [
        CTTRIGIN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itctrl [
        IME OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Claimset [
        CLAIMSET OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Claimclr [
        CLAIMCLR OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Lsr [
        LOCKTYPE OFFSET(2) NUMBITS(1) [],
        LOCKGRANT OFFSET(1) NUMBITS(1) [],
        LOCKEXIST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Authstatus [
        SNID OFFSET(6) NUMBITS(2) [],
        SID OFFSET(4) NUMBITS(2) [],
        NSNID OFFSET(2) NUMBITS(2) [],
        NSID OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devid [
        NUMCH OFFSET(16) NUMBITS(4) [],
        NUMTRIG OFFSET(8) NUMBITS(8) [],
        EXTMUXNUM OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Devtype [
        SUB_TYPE OFFSET(4) NUMBITS(4) [],
        MAJOR_TYPE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr4 [
        SIZE OFFSET(4) NUMBITS(4) [],
        DES_2 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr0 [
        PART_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr1 [
        DES_0 OFFSET(4) NUMBITS(4) [],
        PART_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC OFFSET(3) NUMBITS(1) [],
        DES_1 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        CMOD OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr0 [
        PRMBL_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr1 [
        CLASS OFFSET(4) NUMBITS(4) [],
        PRMBL_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr2 [
        PRMBL_2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr3 [
        PRMBL_3 OFFSET(0) NUMBITS(8) [],
    ]
];

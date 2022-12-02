// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// Trace Port Interface Unit, Test Port Interface Unit bridge to on-chip trace data
pub static mut CORESIGHT_SOC_TPIU: *mut Registers = 0xfe980000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Each bit location represents a single port size that is supported on the device, that is, 32-1 in bit locations [31:0].
        (0x00000000 => pub supported_port_sizes: ReadOnly<u32, SupportedPortSizes::Register>),
        /// The Current Port Size Register has the same format as the Supported Port Sizes register but only one bit is set, and all others must be zero. Writing values with more than one bit set or setting a bit that is not indicated as supported is not supported and causes unpredictable behavior.On reset this defaults to the smallest possible port size, 1 bit, and so reads as 0x00000001.Note: Do not modify the value while the Trace Port is still active, or without correctly stopping the formatter (see Formatter and Flush Control Register, 0x304). This can result in data not being aligned to the port width. For example, data on an 8-bit Trace Port might not be byte aligned.
        (0x00000004 => pub current_port_size: ReadWrite<u32, CurrentPortSize::Register>),
        (0x00000008 => _padding8),
        /// This register indicates the implemented Trigger Counter multipliers and other supported features of the trigger system.
        (0x00000100 => pub supported_trigger_modes: ReadOnly<u32, SupportedTriggerModes::Register>),
        /// The Trigger Counter Register enables delaying the indication of triggers to any external connected trace capture or storage devices. This counter is only eight bits wide and is intended to only be used with the counter multipliers in the Trigger Multiplier Register, 0x108. When a trigger is started, this value, in combination with the multiplier, is the number of words before the trigger is indicated. When the trigger counter reaches zero, the value written here is reloaded. Writing to this register causes the trigger counter value to reset but not reset any values on the multiplier. Reading this register returns the preset value not the current count.
        (0x00000104 => pub trigger_counter_value: ReadWrite<u32, TriggerCounterValue::Register>),
        /// This register contains the selectors for the Trigger Counter Multiplier. Several multipliers can be selected to create the required multiplier value, that is, any value between 1 and approximately 2x10^9. The default value is multiplied by 1, 0x0.Writing to this register causes the internal trigger counter and the state in the multipliers to be reset to initial count position, that is, trigger counter is reloaded with the Trigger Counter Register value and all multipliers are reset.
        (0x00000108 => pub trigger_multiplier: ReadWrite<u32, TriggerMultiplier::Register>),
        (0x0000010c => _padding268),
        /// The pattern generator unit provides a set of known bit sequences or patterns that can be output over the Trace Port and be detected by the TPA or other associated trace capture device.
        (0x00000200 => pub supported_test_pattern_modes: ReadOnly<u32, SupportedTestPatternModes::Register>),
        /// This register indicates the current test pattern/mode selected. Only one of the modes can be set, using bits 17-16, but a multiple number of bits for the patterns can be set using bits 3-0. If Timed Mode is chosen, then after the allotted number of cycles has been reached, the mode automatically switches to Off Mode. On reset this register is set to 18h00000, Off Mode with no selected patterns.
        (0x00000204 => pub current_test_pattern_mode: ReadWrite<u32, CurrentTestPatternMode::Register>),
        /// This is an eight-bit counter start value that is decremented. A write sets the initial counter value and a read returns the programmed value. On reset this value is set to 0.
        (0x00000208 => pub tprcr: ReadWrite<u32, Tprcr::Register>),
        (0x0000020c => _padding524),
        /// This register indicates the current status of the formatter and flush features available in the TPIU.
        (0x00000300 => pub ffsr: ReadOnly<u32, Ffsr::Register>),
        /// This register controls the generation of stop, trigger, and flush events.To disable formatting and put the formatter into bypass mode, bits 1 and 0 must be clear. Setting both bits is the same as setting bit 1.All three flush generating conditions can be enabled together. However, if a second or third flush event is generated from another condition then the current flush completes before the next flush is serviced. Flush from flushin takes priority over flush from Trigger, which in turn completes before a manually activated flush. All Trigger indication conditions can be enabled simultaneously although this can cause the appearance of multiple triggers if flush using trigger is also enabled.Both Stop On settings can be enabled, although if flush on trigger is set up then none of the flushed data is stored. When the system stops, it returns atreadys and does not store the accepted data packets. This is to avoid stalling of any other devices that are connected to a Trace Replicator.If an event in the Formatter and Flush Control Register is required, it must be enabled before the originating event starts. Because requests from flushes and triggers can originate in an asynchronous clock domain, the exact time the component acts on the request cannot be determined with respect to configuring the control.Note - It is recommended that the Trace Port width is changed without enabling continuous mode. Enabling continuous mode causes data to be output from the Trace Port and modifying the port size can result in data not being aligned for power2 port widths.- To perform a stop on flush completion through a manually-generated flush request, two write operations to the register are required: one to enable the stop event, if it is not already enabled; one to generate the manual flush.
        (0x00000304 => pub ffcr: ReadWrite<u32, Ffcr::Register>),
        /// The Formatter Synchronization Counter Register enables effective use on different sized Trace Port Analyzers (TPAs) without wasting large amounts of the storage capacity of the capture device.This counter is the number of formatter frames since the last synchronization packet of 128 bits, and is a 12-bit counter with a maximum count value of 4096. This equates to synchronization every 65536 bytes, that is, 4096 packets x 16 bytes per packet. The default is set up for a synchronization packet every 1024 bytes, that is, every 64 formatter frames.If the formatter has been configured for continuous mode, full and half-word sync frames are inserted during normal operation. Under these circumstances the count value represents the maximum number of complete frames between full synchronization packets.
        (0x00000308 => pub fscr: ReadWrite<u32, Fscr::Register>),
        (0x0000030c => _padding780),
        /// Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexors or other solutions that might be added to the trace output pins either for pin control or a high speed trace port solution. These ports are raw register banks that sample or export the corresponding external pins. The output register bank is set to all zeros on reset. The input registers sample the incoming signals and as such are undefined.
        (0x00000400 => pub extctl_in_port: ReadOnly<u32, ExtctlInPort::Register>),
        /// Two ports can be used as a control and feedback mechanism for any serializers, pin sharing multiplexors or other solutions that might be added to the trace output pins either for pin control or a high speed trace port solution. These ports are raw register banks that sample or export the corresponding external pins. The output register bank is set to all zeros on reset. The input registers sample the incoming signals and as such are undefined.
        (0x00000404 => pub extctl_out_port: ReadWrite<u32, ExtctlOutPort::Register>),
        (0x00000408 => _padding1032),
        /// The Integration Test Trigger In and Flush In Acknowledge Register enables control of the triginack and flushinack outputs from the TPIU.
        (0x00000ee4 => pub ittrflinack: WriteOnly<u32, Ittrflinack::Register>),
        /// The Integration Test Trigger In and Flush In Register contains the values of the flushin and trigin inputs to the TPIU.
        (0x00000ee8 => pub ittrflin: ReadOnly<u32, Ittrflin::Register>),
        /// The Integration Test ATB Data Register 0 contains the value of the atdatas inputs to the TPIU. The values are only valid when atvalids is HIGH.
        (0x00000eec => pub itatbdata0: ReadOnly<u32, Itatbdata0::Register>),
        /// The Integration Test ATB Control Register 2 enables control of the atreadys and afvalids outputs of the TPIU.
        (0x00000ef0 => pub itatbctr2: WriteOnly<u32, Itatbctr2::Register>),
        /// The Integration Test ATB Control Register 1 contains the value of the atids input to the TPIU. This is only valid when atvalids is HIGH.
        (0x00000ef4 => pub itatbctr1: ReadOnly<u32, Itatbctr1::Register>),
        /// The Integration Test ATB Control Register 0 captures the values of the atvalids, afreadys, and atbytess inputs to the TPIU. To ensure the integration registers work correctly in a system, the value of atbytess is only valid when atvalids, bit [0], is HIGH.
        (0x00000ef8 => pub itatbctr0: ReadOnly<u32, Itatbctr0::Register>),
        (0x00000efc => _padding3836),
        /// This register is used to enable topology detection. For more information see the CoreSight Architecture Specification. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purpose of integration testing and topology solving. Note: When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection.The registers in the TPIU enable the system to set the flushinack and triginack output pins. The flushin and trigin inputs to the TPIU can also be read. The other Integration Test Registers are for testing the integration of the ATB slave interface on the TPIU.
        (0x00000f00 => pub itctrl: ReadWrite<u32, Itctrl::Register>),
        (0x00000f04 => _padding3844),
        /// This is used in conjunction with Claim Tag Clear Register, CLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read.
        (0x00000fa0 => pub claimset: ReadWrite<u32, Claimset::Register>),
        /// This register is used in conjunction with Claim Tag Set Register, CLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read.
        (0x00000fa4 => pub claimclr: ReadWrite<u32, Claimclr::Register>),
        (0x00000fa8 => _padding4008),
        /// This is used to enable write access to device registers. External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. A debugger does not have to unlock the component in order to write and modify the registers in the component.
        (0x00000fb0 => pub lar: WriteOnly<u32>),
        /// This indicates the status of the Lock control mechanism. This lock prevents accidental writes by code under debug. When locked, write access is blocked to all registers, except the Lock Access Register.External accesses from a debugger (paddrdbg31 = 1) are not subject to the Lock Registers. This register reads as 0 when read from an external debugger (paddrdbg31 = 1).
        (0x00000fb4 => pub lsr: ReadOnly<u32, Lsr::Register>),
        /// Reports what functionality is currently permitted by the authentication interface.
        (0x00000fb8 => pub authstatus: ReadOnly<u32, Authstatus::Register>),
        (0x00000fbc => _padding4028),
        /// This register indicates the capabilities of the TPIU.
        (0x00000fc8 => pub devid: ReadOnly<u32, Devid::Register>),
        /// It provides a debugger with information about the component when the Part Number field is not recognized. The debugger can then report this information.
        (0x00000fcc => pub devtype: ReadOnly<u32, Devtype::Register>),
        /// Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.
        (0x00000fd0 => pub pidr4: ReadOnly<u32, Pidr4::Register>),
        /// Reserved
        (0x00000fd4 => pub pidr5: ReadWrite<u32>),
        /// Reserved
        (0x00000fd8 => pub pidr6: ReadWrite<u32>),
        /// Reserved
        (0x00000fdc => pub pidr7: ReadWrite<u32>),
        /// Part of the set of Peripheral Identification registers. Contains part of the designer specific part number.
        (0x00000fe0 => pub pidr0: ReadOnly<u32, Pidr0::Register>),
        /// Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity.
        (0x00000fe4 => pub pidr1: ReadOnly<u32, Pidr1::Register>),
        /// Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.
        (0x00000fe8 => pub pidr2: ReadOnly<u32, Pidr2::Register>),
        /// Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields.
        (0x00000fec => pub pidr3: ReadOnly<u32, Pidr3::Register>),
        /// A component identification register, that indicates that the identification registers are present.
        (0x00000ff0 => pub cidr0: ReadOnly<u32, Cidr0::Register>),
        /// A component identification register, that indicates that the identification registers are present. This register also indicates the component class.
        (0x00000ff4 => pub cidr1: ReadOnly<u32, Cidr1::Register>),
        /// A component identification register, that indicates that the identification registers are present.
        (0x00000ff8 => pub cidr2: ReadOnly<u32, Cidr2::Register>),
        /// A component identification register, that indicates that the identification registers are present.
        (0x00000ffc => pub cidr3: ReadOnly<u32, Cidr3::Register>),
        (0x00001000 => @END),
    }
}
register_bitfields! [
    u32,
    pub SupportedPortSizes [
        PORT_SIZE_32 OFFSET(31) NUMBITS(1) [],
        PORT_SIZE_31 OFFSET(30) NUMBITS(1) [],
        PORT_SIZE_30 OFFSET(29) NUMBITS(1) [],
        PORT_SIZE_29 OFFSET(28) NUMBITS(1) [],
        PORT_SIZE_28 OFFSET(27) NUMBITS(1) [],
        PORT_SIZE_27 OFFSET(26) NUMBITS(1) [],
        PORT_SIZE_26 OFFSET(25) NUMBITS(1) [],
        PORT_SIZE_25 OFFSET(24) NUMBITS(1) [],
        PORT_SIZE_24 OFFSET(23) NUMBITS(1) [],
        PORT_SIZE_23 OFFSET(22) NUMBITS(1) [],
        PORT_SIZE_22 OFFSET(21) NUMBITS(1) [],
        PORT_SIZE_21 OFFSET(20) NUMBITS(1) [],
        PORT_SIZE_20 OFFSET(19) NUMBITS(1) [],
        PORT_SIZE_19 OFFSET(18) NUMBITS(1) [],
        PORT_SIZE_18 OFFSET(17) NUMBITS(1) [],
        PORT_SIZE_17 OFFSET(16) NUMBITS(1) [],
        PORT_SIZE_16 OFFSET(15) NUMBITS(1) [],
        PORT_SIZE_15 OFFSET(14) NUMBITS(1) [],
        PORT_SIZE_14 OFFSET(13) NUMBITS(1) [],
        PORT_SIZE_13 OFFSET(12) NUMBITS(1) [],
        PORT_SIZE_12 OFFSET(11) NUMBITS(1) [],
        PORT_SIZE_11 OFFSET(10) NUMBITS(1) [],
        PORT_SIZE_10 OFFSET(9) NUMBITS(1) [],
        PORT_SIZE_9 OFFSET(8) NUMBITS(1) [],
        PORT_SIZE_8 OFFSET(7) NUMBITS(1) [],
        PORT_SIZE_7 OFFSET(6) NUMBITS(1) [],
        PORT_SIZE_6 OFFSET(5) NUMBITS(1) [],
        PORT_SIZE_5 OFFSET(4) NUMBITS(1) [],
        PORT_SIZE_4 OFFSET(3) NUMBITS(1) [],
        PORT_SIZE_3 OFFSET(2) NUMBITS(1) [],
        PORT_SIZE_2 OFFSET(1) NUMBITS(1) [],
        PORT_SIZE_1 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub CurrentPortSize [
        PORT_SIZE_32 OFFSET(31) NUMBITS(1) [],
        PORT_SIZE_31 OFFSET(30) NUMBITS(1) [],
        PORT_SIZE_30 OFFSET(29) NUMBITS(1) [],
        PORT_SIZE_29 OFFSET(28) NUMBITS(1) [],
        PORT_SIZE_28 OFFSET(27) NUMBITS(1) [],
        PORT_SIZE_27 OFFSET(26) NUMBITS(1) [],
        PORT_SIZE_26 OFFSET(25) NUMBITS(1) [],
        PORT_SIZE_25 OFFSET(24) NUMBITS(1) [],
        PORT_SIZE_24 OFFSET(23) NUMBITS(1) [],
        PORT_SIZE_23 OFFSET(22) NUMBITS(1) [],
        PORT_SIZE_22 OFFSET(21) NUMBITS(1) [],
        PORT_SIZE_21 OFFSET(20) NUMBITS(1) [],
        PORT_SIZE_20 OFFSET(19) NUMBITS(1) [],
        PORT_SIZE_19 OFFSET(18) NUMBITS(1) [],
        PORT_SIZE_18 OFFSET(17) NUMBITS(1) [],
        PORT_SIZE_17 OFFSET(16) NUMBITS(1) [],
        PORT_SIZE_16 OFFSET(15) NUMBITS(1) [],
        PORT_SIZE_15 OFFSET(14) NUMBITS(1) [],
        PORT_SIZE_14 OFFSET(13) NUMBITS(1) [],
        PORT_SIZE_13 OFFSET(12) NUMBITS(1) [],
        PORT_SIZE_12 OFFSET(11) NUMBITS(1) [],
        PORT_SIZE_11 OFFSET(10) NUMBITS(1) [],
        PORT_SIZE_10 OFFSET(9) NUMBITS(1) [],
        PORT_SIZE_9 OFFSET(8) NUMBITS(1) [],
        PORT_SIZE_8 OFFSET(7) NUMBITS(1) [],
        PORT_SIZE_7 OFFSET(6) NUMBITS(1) [],
        PORT_SIZE_6 OFFSET(5) NUMBITS(1) [],
        PORT_SIZE_5 OFFSET(4) NUMBITS(1) [],
        PORT_SIZE_4 OFFSET(3) NUMBITS(1) [],
        PORT_SIZE_3 OFFSET(2) NUMBITS(1) [],
        PORT_SIZE_2 OFFSET(1) NUMBITS(1) [],
        PORT_SIZE_1 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub SupportedTriggerModes [
        TRGRUN OFFSET(17) NUMBITS(1) [],
        TRIGGERED OFFSET(16) NUMBITS(1) [],
        TCOUNT8 OFFSET(8) NUMBITS(1) [],
        MULT64K OFFSET(4) NUMBITS(1) [],
        MULT256 OFFSET(3) NUMBITS(1) [],
        MULT16 OFFSET(2) NUMBITS(1) [],
        MULT4 OFFSET(1) NUMBITS(1) [],
        MULT2 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TriggerCounterValue [
        TRIGCOUNT OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TriggerMultiplier [
        MULT64K OFFSET(4) NUMBITS(1) [],
        MULT256 OFFSET(3) NUMBITS(1) [],
        MULT16 OFFSET(2) NUMBITS(1) [],
        MULT4 OFFSET(1) NUMBITS(1) [],
        MULT2 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub SupportedTestPatternModes [
        PCONTEN OFFSET(17) NUMBITS(1) [],
        PTIMEEN OFFSET(16) NUMBITS(1) [],
        PATF0 OFFSET(3) NUMBITS(1) [],
        PATA5 OFFSET(2) NUMBITS(1) [],
        PATW0 OFFSET(1) NUMBITS(1) [],
        PATW1 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub CurrentTestPatternMode [
        PCONTEN OFFSET(17) NUMBITS(1) [],
        PTIMEEN OFFSET(16) NUMBITS(1) [],
        PATF0 OFFSET(3) NUMBITS(1) [],
        PATA5 OFFSET(2) NUMBITS(1) [],
        PATW0 OFFSET(1) NUMBITS(1) [],
        PATW1 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Tprcr [
        PATTCOUNT OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Ffsr [
        TCPRESENT OFFSET(2) NUMBITS(1) [],
        FTSTOPPED OFFSET(1) NUMBITS(1) [],
        FLINPROG OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Ffcr [
        STOPTRIG OFFSET(13) NUMBITS(1) [],
        STOPFL OFFSET(12) NUMBITS(1) [],
        TRIGFL OFFSET(10) NUMBITS(1) [],
        TRIGEVT OFFSET(9) NUMBITS(1) [],
        TRIGIN OFFSET(8) NUMBITS(1) [],
        FONMAN OFFSET(6) NUMBITS(1) [],
        FONTRIG OFFSET(5) NUMBITS(1) [],
        FONFLIN OFFSET(4) NUMBITS(1) [],
        ENFCONT OFFSET(1) NUMBITS(1) [],
        ENFTC OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Fscr [
        CYCCOUNT OFFSET(0) NUMBITS(12) [],
    ]
];
register_bitfields! [
    u32,
    pub ExtctlInPort [
        EXTCTLIN OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub ExtctlOutPort [
        EXTCTLOUT OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub Ittrflinack [
        FLUSHINACK OFFSET(1) NUMBITS(1) [],
        TRIGINACK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Ittrflin [
        FLUSHIN OFFSET(1) NUMBITS(1) [],
        TRIGIN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Itatbdata0 [
        ATDATA_31 OFFSET(4) NUMBITS(1) [],
        ATDATA_23 OFFSET(3) NUMBITS(1) [],
        ATDATA_15 OFFSET(2) NUMBITS(1) [],
        ATDATA_7 OFFSET(1) NUMBITS(1) [],
        ATDATA_0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Itatbctr2 [
        AFVALID OFFSET(1) NUMBITS(1) [],
        ATREADY OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Itatbctr1 [
        ATID OFFSET(0) NUMBITS(7) [],
    ]
];
register_bitfields! [
    u32,
    pub Itatbctr0 [
        ATBYTES OFFSET(8) NUMBITS(2) [],
        AFREADY OFFSET(1) NUMBITS(1) [],
        ATVALID OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Itctrl [
        INTEGRATION_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Claimset [
        CLAIMSET OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Claimclr [
        CLAIMCLR OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub Lsr [
        LOCKTYPE OFFSET(2) NUMBITS(1) [],
        LOCKGRANT OFFSET(1) NUMBITS(1) [],
        LOCKEXIST OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Authstatus [
        SNID OFFSET(6) NUMBITS(2) [],
        SID OFFSET(4) NUMBITS(2) [],
        NSNID OFFSET(2) NUMBITS(2) [],
        NSID OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub Devid [
        SWOUARTNRZ OFFSET(11) NUMBITS(1) [],
        SWOMAN OFFSET(10) NUMBITS(1) [],
        TCLKDATA OFFSET(9) NUMBITS(1) [],
        FIFOSIZE OFFSET(6) NUMBITS(3) [],
        CLKRELAT OFFSET(5) NUMBITS(1) [],
        MUXNUM OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u32,
    pub Devtype [
        SUB_TYPE OFFSET(4) NUMBITS(4) [],
        MAJOR_TYPE OFFSET(0) NUMBITS(4) [],
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

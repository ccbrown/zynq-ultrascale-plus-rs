// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// 4K Embedded Trace FIFO, Embedded Trace FIFO
pub static mut CORESIGHT_SOC_ETF_1: *mut Registers = 0xfe940000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 4],
    /// RAM size Register
    pub rsz: ReadOnly<u32>,
    _padding8: [u8; 4],
    /// TMC Status register
    pub sts: ReadOnly<u32, Sts::Register>,
    /// Reading this register enables data to be read from the trace memory. When the memory width given in the DEVID register is greater than 32 bits, multiple reads to this register must be performed together to read a full memory width of data. For example, if the memory width is 128 bits, then reads from this register must be performed four at a time. When a full memory width of data has been read, the RAM Read Pointer is incremented to the next memory word.When no data is available, this register returns 0xFFFFFFFF. This value is chosen because it cannot be generated as part of the trace data when the formatter is enabled.Trace Capture disabled: When in Disabled state (TraceCaptEn=0 and TMCReady=1), the TMC mode is ignored. Reading this register returns the contents of the Local RAM buffer or AXI memory at the location addressed by the RAM Read Pointer Register.Circular Buffer mode: When in Stopped state in Circular Buffer mode and the buffer is not empty, reading this register returns the next word of data from the trace buffer. When all of the trace buffer has been read, the Empty bit in the STS Register is set, and more reads return 0xFFFFFFF. Reading this register when not in Stopped state returns 0xFFFFFFFF.Software FIFO mode: Reading this register returns data from the FIFO. If this register is read when the FIFO is empty, the data returned is 0xFFFFFFFF.Hardware FIFO mode: Reading this register returns 0xFFFFFFFF.Reading this register alters the internal state of the TMC, and can only be performed if the device is unlocked. Reading this register when the device is locked returns 0xFFFFFFFF.In the ETR configuration, when the MemErr bit in the STS Register is set, reading this register returns an error response on the APB slave interface.
    pub rrd: ReadOnly<u32>,
    /// The RAM Read Pointer Register contains the value of the read pointer that is used to read entries from the trace memory over the APB interface.The value written to this register must be a byte-address aligned to the width of the trace memory databus and to a frame boundary. For example, for 64_bit wide trace memory and 128_bit wide trace memory, the four LSBs must be 0s. For 256_bit wide trace memory, the five LSBs must be 0s. The width of the trace memory can be obtained by reading the MEMWIDTH field in the DEVID Register, 0xFC8. When one complete buffer or FIFO entry has been read through the RRD Register, the RAM Read Pointer Register is incremented by the number of bytes per memory width of data. For example, for 64_bit wide memory, it is incremented by eight. For 128_bit wide memory, it is incremented by 16 for every complete memory entry read. When the RAM Read Pointer is incremented after having reached its maximum value, it wraps around. The width of this register in the ETB or ETF configurations is log2(MEM_SIZE*4). In the ETR configuration, this register is 32 bits wide, and the contents of this register represents the lower 32 bits of the 40-bit AXI address used to access trace memory. If scatter-gather operation is enabled, this register represents the next address in trace memory to be read, not the address of a page table entry. When in Disabled state (TraceCaptEn=0 and TMCReady=1), a write to this register sets the value of the trace memory address from which data is fetched on a subsequent RRD read. A write to this register when not in Disabled state results in Unpredictable behavior. This register can be read:<ul><li>when in Disabled state</li><li>when in Stopped state (TraceCaptEn=1 and TMCReady=1), in Circular Buffer mode</li><li>when in Running (TraceCaptEn=1 and TMCReady=0), Stopping or Stopped states, in Software FIFO mode.</li></ul>When entering Disabled state in Circular Buffer mode with scatter-gather mode disabled, this register points to the next location in the trace buffer to be read. This is for backwards compatibility purposes, so that the buffer can be read while in Disabled state. It is recommended that you read the buffer contents while in Stopped state instead, because the pointers are managed automatically.
    pub rrp: ReadWrite<u32, Rrp::Register>,
    /// The value written to this register must be a byte-address aligned to the width of the trace memory databus and to a frame boundary. For example, for 64_bit wide trace memory and 128_bit wide trace memory, the four LSBs must be 0s. For 256_bit wide trace memory, the five LSBs must be 0s. The width of the trace memory can be obtained by reading the MEMWIDTH field in the DEVID Register, 0xFC8. Reading this register returns the current memory location being referenced, to which the next write would occur. When one complete buffer or FIFO entry has been written to the RWD Register, the RAM Write Pointer Register is incremented by the number of bytes per memory width of data. For example, for 64_bit wide memory, it is incremented by eight. For 128_bit wide memory it is incremented by 16 for every complete memory entry write. When this register wraps around its maximum value, the Full flag in the Status Register, STS, 0x00C, is set. In the ETB or ETF configurations, the width of this register is log2(MEM_SIZE*4). In the ETR configuration, this register is 32 bits wide, and the contents of this register represent the lower 32 bits of the 40-bit AXI address used to access trace memory. In ETB and ETF configurations, when in Circular Buffer mode, this register can be used to set the address to start capturing data from. This is for backwards compatibility purposes, to enable the FULL signal to be generated before the buffer becomes full. In other configurations and modes, the RAM Write Pointer is reset to the start of trace memory when exiting Disabled state (TraceCaptEn=0 and TMCReady=1). This register can be read:<ul><li>when in Disabled state</li><li>when in Stopped state, in Circular Buffer mode</li><li>when in Running, Stopping or Stopped states, in Software FIFO mode.</li></ul>
    pub rwp: ReadWrite<u32, Rwp::Register>,
    /// In the Circular-buffer mode, the Trigger Counter Register specifies the number of 32_bit words to capture in the Trace RAM following the detection of either a rising edge on the TRIGIN input or a trigger packet in the incoming trace stream (ATID = 7h7D). On capturing the specified number of datawords, a Trigger Event is said to have occurred. The effect of a Trigger Event on TMC behavior is controlled by the FFCR register, 0x304.The number of 32_bit words written into the Trace RAM following the trigger is the value stored in this register+1. This counter is disabled when the TMC is in Software-read-FIFO mode or Hardware-read-FIFO mode.Once the trigger counter has started counting, any further triggers, either on TRIGIN or in the incoming trace stream, are ignored till the counter reaches 0. Once the counter has reached 0, it remains at 0 till it is re-programmed with a write to this register. This register is cleared when TMCReady goes HIGH, so that the state of the counter when trace capture has stopped does not affect a subsequent trace capture cycle.Attempting to write to this register while not in Disabled state (TMCReady=0 or TraceCaptEn=1) will result in Unpredictable behavior. A read access to this register is permitted even if trace capture enabled. A read access returns the current value of the Trigger counter. The width of this register and the Trigger counter depends on the size of the trace memory. In ETB and ETF configurations, the width of the counter is log2(MEMSIZE). The width of this register in ETR configuration is 32 bits.
    pub trg: ReadWrite<u32, Trg::Register>,
    /// This register controls trace stream capture.Setting the TraceCaptEn bit to 1 enables the TMC to capture trace data. When trace capture is enabled, Formatter behavior is controlled by the FFCR register.When trace capture is disabled, any remaining data in the formatter is stored to RAM. If the TMC is programmed for Software-read-FIFO mode or hardware-read-FIFO mode and and TraceCaptEn is cleared before TMCReady=1, trace data may get corrupted. In the Hardware-read-FIFO mode, the unformatter drains any trace data in its internal pipelines on to the ATB Master interface, but discards any data that remains in the trace FIFO. Trace capture is fully disabled, or complete, when TMCReady goes HIGH. See Formatter and Flush Status Register, FFSR, 0x300.It is recommended that stopping trace capture be initiated only by programming stop conditions in FFCR register bits. Stopping trace capture by clearing TraceCaptEn is deprecated and is supported only for backwards compatibility with earlier versions of the ETB. Features in the TMC such as the DrainBuffer bit (FFCR register) and the Empty bit (STS register) that are not part of the earlier versions of the ETB do not support stopping trace capture by clearing TraceCaptEn. If trace capture stopping is initiated by clearing this bit, then the DrainBuffer feature (ETF configuration) cannot be invoked. Also, in the ETR configuration, if the TMC is programmed for Scatter_Gather operation and Circular-Buffer mode, clearing TraceCaptEn prevents reading trace data from memory.
    pub ctl: ReadWrite<u32, Ctl::Register>,
    /// Enables testing of Trace RAM connectivity to the TMC. When in Disabled state (TraceCaptEn=0 and TMCReady=1), a write to this register stores data at the location pointed to by the RWP. Writes to this register when not in Disabled state are ignored. When the memory width given in the DEVID register is greater than 32_bit, multiple writes to this register must be performed together to read a full memory width of data. For example, if the memory width is 128 bits, then writes to this register must be performed four at a time. When a full memory width of data has been written, the data is written to memory and the RAM Write Pointer is incremented to the next memory word. In ETR configuration, when the MemErr bit in the STS Register is set, writing to this register returns an error response on the APB slave interface and the write data is discarded. Writing to this register other than when in Disabled state and in integration mode results in Unpredictable behavior.
    pub rwd: WriteOnly<u32>,
    /// Controls TMC operating mode. When configured as an ETB or ETR, the TMC can operate in the following modes:<ul><li>Software FIFO mode</li><li>Circular Buffer mode.</li></ul> When configured as an ETF, the TMC has an additional mode of operation, Hardware FIFO mode. The operating mode can be changed only when the TMC is disabled. Attempting to write to this register while not in Disabled state (TraceCaptEn=0 and TMCReady=1) results in Unpredictable behavior. The operating mode is ignored when in Disabled state.
    pub mode: ReadWrite<u32, Mode::Register>,
    /// Reading this register returns the maximum fill level of the trace memory in 32_bit words since this register was last read. Reading this register also results in its contents being updated to the current fill level.This register does not apply when the TMC is programmed for Scatter Gather operation in ETR configuration. In this case, reading this register will return 0x00000000.When the TraceCaptEn bit is cleared, this register retains it last value. If TraceCaptEn is 0 and a 1 is written to it, the LBUFLEVEL register is cleared.The fill level information is mainly useful for performance analysis.Reading this register alters the internal state of the TMC, hence this register can be read only if the device is unlocked. Reading this register when the device is locked will return 0x00000000.The width of this register in the ETB/ETF configurations is 1+log2(MEM_SIZE). In the ETR configuration, the width of this register is 31 bits. Note that an extra bit is needed to return the correct fill level when full.
    pub lbuflevel: ReadOnly<u32, Lbuflevel::Register>,
    /// When TraceCaptEn=1, this register indicates the current fill level of the trace memory in 32_bit words. When TraceCaptEn=0, reading this register returns a value of 0x00000000. This register is not available if the TMC is programmed for Scatter_Gather operation in ETR configuration - in this case, reading this register returns a value of 0x00000000. This register is particularly useful in the FIFO modes, in which the pointers change dynamically due to read and write of trace data into trace memory through hardware. It would not be possible for the debugger to deduce the fill level of the trace memory merely by reading the pointer registers. The fill level information is mainly useful for performance analysis.
    pub cbuflevel: ReadOnly<u32, Cbuflevel::Register>,
    /// The value programmed into this register indicates the desired threshold vacancy level in 32_bit words in the trace memory. When the space into FIFO is less than or equal to this value (ie Fill level >= MEM_SIZE - BUFWM), the FULL output is pulled HIGH and the Full bit in the STS register is set. This register is used only in the Software FIFO mode and the Hardware FIFO mode. In the Circular Buffer mode, this functionality can be obtained by programming the RWP to the desired vacancy trigger level, so that when the pointer wraps around, the Full bit is set indicating that the vacancy level has fallen below the desired level. The maximum value that can be written into this register is MEM_SIZE - 1 (in which case the Full bit is set after the first 32_bit word is written to trace memory.).Writing to this register other than when TMCReady=1 and TraceCaptEn=0 will result in Unpredictable behavior.This register is ignored when the TMC is programmed for scatter-gather operation in ETR configuration.
    pub bufwm: ReadWrite<u32, Bufwm::Register>,
    _padding56: [u8; 712],
    /// This register indicates the status of the Formatter and the status of Flush request.
    pub ffsr: ReadOnly<u32, Ffsr::Register>,
    /// This register allows user control of the stop, trigger, and flush events. When the EnFt bit is 0, no formatting information is inserted into the trace stream and the trace data is stored raw. When tracing is stopped, a byte of value 0x01 is appended to the trace buffer, followed by zero or more bytes of value 0x00 to align to a memory dataword. When data is later decompressed it is then possible to determine that a post-amble is present by back tracking the trailing zero data at the end of the trace stream until the last single bit at logic 1 is detected. All data preceding this first logic 1 is then treated as decompressible data. When all data has been stored in the RAM, FtStopped in the Formatter and Flush Status Register is set HIGH. Note: When the EnFt bit is 0, it is assumed that the source ID is not changing. Multiple flush generating conditions can be enabled together. However, if a second or third flush event is generated then the current flush completes before the next flush is serviced. Only one flush can be outstanding at a time. If two flushes are requested simultaneously, only one will be issued. If two flushes are requested while another is in progress, only one further flush will be issued when the in-progress flush completes.Multiple trigger indication conditions can be enabled simultaneously although this can cause the appearance of multiple triggers if flush using trigger is also enabled. If StopOnTrigEvt and FOnTrigEvt are both set then none of the flushed data is stored. When the system stops, it returns ATREADYS HIGH and does not store the accepted data packets. This is to stop stalling of any other devices connected using a Trace Replicator. StopOnTrigEvt, FOnTrigEvt and TrigOnTrigEvt bits are functional only in Circular Buffer mode. Setting these bits with the TMC enabled in FIFO modes will not have any effect on the TMC.Note: To perform a stop on flush completion through a manually-generated flush request, two write operations to the register are required - one to enable the stop event, if it is not already enabled and one to generate the manual flush.
    pub ffcr: ReadWrite<u32, Ffcr::Register>,
    /// This register determines the reload value of the Periodic Synchronization Counter. This counter enables the frequency of synchronization information to be optimized to the trace capture buffer size.When the TMC is enabled, the Periodic Synchronization counter counts the number of bytes of trace data stored into the trace memory (regardless of whether the trace data has been formatted by the TMC or not) since the occurrence of the last synchronization request on the ATB slave interface. The value programmed into this register determines the reload value of the Periodic Synchronization counter.This counter is enabled only when the TraceCaptEn bit in the Control Register, CTL, 0x020, is set. Writing to this register other than when TraceCaptEn=0 and TMCReady=1 will result in Unpredictable behavior.
    pub pscr: ReadWrite<u32, Pscr::Register>,
    _padding780: [u8; 3012],
    /// The Integration Test ATB Master Data Register 0 enables control of the ATDATAM output of the TMC. Writing to this register other than when in Disabled state (TraceCaptEn=0 and TMCReady=1) and in integration mode results in Unpredictable behavior.
    pub itatbmdata0: WriteOnly<u32, Itatbmdata0::Register>,
    /// The Integration Test ATB Master Interface Control Register 2 captures the values of the SYNCREQM, AFVALIDM and ATREADYM inputs to the TMC.
    pub itatbmctr2: ReadOnly<u32, Itatbmctr2::Register>,
    /// The Integration Test ATB Master Control Register 1 enables control of the ATIDM outputs of the TMC. Writing to this register other than when in Disabled state (TraceCaptEn=0 and TMCReady=1) and in integration mode results in Unpredictable behavior.
    pub itatbmctr1: WriteOnly<u32, Itatbmctr1::Register>,
    /// The Integration Test ATB Master Interface Control Register 0 enables control of the ATBYTESM, AFREADYM and ATVALIDM outputs of the TMC. Writing to this register other than when in Disabled state (TraceCaptEn=0 and TMCReady=1) and in integration mode results in Unpredictable behavior.
    pub itatbmctr0: WriteOnly<u32, Itatbmctr0::Register>,
    /// The Integration Test Miscellaneous Output Register 0 controls the values of some outputs from the TMC. Writing to this register other than when in Disabled state (TraceCaptEn=0 and TMCReady=1) and in integration mode results in Unpredictable behavior.
    pub itmiscop0: WriteOnly<u32, Itmiscop0::Register>,
    _padding3812: [u8; 4],
    /// This register returns the values of the FLUSHIN and TRIGIN inputs to the TMC. Writing to this register other than when in Disabled state (TraceCaptEn=0 and TMCReady=1) and in integration mode results in Unpredictable behavior.
    pub ittrflin: ReadOnly<u32, Ittrflin::Register>,
    /// The Integration Test ATB Data Register 0 returns the values on the ATDATAS input to the TMC.
    pub itatbdata0: ReadOnly<u32, Itatbdata0::Register>,
    /// The Integration Test ATB Control Register 2 enables control of the ATREADYS and AFVALIDS outputs of the TMC. Writing to this register other than when in Disabled state (TraceCaptEn=0 and TMCReady=1) and in integration mode results in Unpredictable behavior.
    pub itatbctr2: WriteOnly<u32, Itatbctr2::Register>,
    /// The Integration Test ATB Control Register 1 captures the value of the ATIDS input to the TMC.
    pub itatbctr1: ReadOnly<u32, Itatbctr1::Register>,
    /// The Integration Test ATB Control Register 0 captures the values of the ATVALIDS, AFREADYS and ATBYTESS inputs to the TMC.
    pub itatbctr0: ReadOnly<u32, Itatbctr0::Register>,
    _padding3836: [u8; 4],
    /// This register is used to enable topology detection. For more information see the CoreSight Architecture Specification. This register enables the component to switch from a functional mode, the default behavior, to integration mode where the inputs and outputs of the component can be directly controlled for the purpose of integration testing and topology solving. Note: When a device has been in integration mode, it might not function with the original behavior. After performing integration or topology detection, you must reset the system to ensure correct behavior of CoreSight and other connected system components that are affected by the integration or topology detection.Writing to this register other than when in Disabled state (TMCReady=1 and TraceCaptEn=0) results in Unpredictable behavior.
    pub itctrl: ReadWrite<u32, Itctrl::Register>,
    _padding3844: [u8; 156],
    /// This is used in conjunction with Claim Tag Clear Register, CLAIMCLR. This register forms one half of the Claim Tag value. This location allows individual bits to be set, write, and returns the number of bits that can be set, read.
    pub claimset: ReadWrite<u32, Claimset::Register>,
    /// This register is used in conjunction with Claim Tag Set Register, CLAIMSET. This register forms one half of the Claim Tag value. This location enables individual bits to be cleared, write, and returns the current Claim Tag value, read.
    pub claimclr: ReadWrite<u32, Claimclr::Register>,
    _padding4008: [u8; 8],
    /// This is used to enable write access to device registers. External accesses from a debugger (PADDRDBG31 = 1) are not subject to the Lock Registers. A debugger does not have to unlock the component in order to write and modify the registers in the component.
    pub lar: WriteOnly<u32>,
    /// This indicates the status of the Lock control mechanism. This lock prevents accidental writes by code under debug. When locked, write access is blocked to all registers, except the Lock Access Register.External accesses from a debugger (PADDRDBG31 = 1) are not subject to the Lock Registers. This register reads as 0 when read from an external debugger (PADDRDBG31 = 1).
    pub lsr: ReadOnly<u32, Lsr::Register>,
    /// Reports what functionality is currently permitted by the authentication interface.
    pub authstatus: ReadOnly<u32, Authstatus::Register>,
    _padding4028: [u8; 12],
    /// This register indicates the capabilities of the CoreSight TMC.
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
    pub Sts [
        EMPTY OFFSET(4) NUMBITS(1) [],
        FTEMPTY OFFSET(3) NUMBITS(1) [],
        TMCREADY OFFSET(2) NUMBITS(1) [],
        TRIGGERED OFFSET(1) NUMBITS(1) [],
        FULL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rrp [
        RRP OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rwp [
        RWP OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Trg [
        TRG OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ctl [
        TRACECAPTEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mode [
        MODE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Lbuflevel [
        LBUFLEVEL OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cbuflevel [
        CBUFLEVEL OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Bufwm [
        BUFWM OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ffsr [
        FTSTOPPED OFFSET(1) NUMBITS(1) [],
        FLINPROG OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ffcr [
        DRAINBUFFER OFFSET(14) NUMBITS(1) [],
        STOPONTRIGEVT OFFSET(13) NUMBITS(1) [],
        STOPONFL OFFSET(12) NUMBITS(1) [],
        TRIGONFL OFFSET(10) NUMBITS(1) [],
        TRIGONTRIGEVT OFFSET(9) NUMBITS(1) [],
        TRIGONTRIGIN OFFSET(8) NUMBITS(1) [],
        FLUSHMAN OFFSET(6) NUMBITS(1) [],
        FONTRIGEVT OFFSET(5) NUMBITS(1) [],
        FONFLIN OFFSET(4) NUMBITS(1) [],
        ENTI OFFSET(1) NUMBITS(1) [],
        ENFT OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pscr [
        PSCOUNT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbmdata0 [
        ATDATAMBIT31 OFFSET(4) NUMBITS(1) [],
        ATDATAMBIT23 OFFSET(3) NUMBITS(1) [],
        ATDATAMBIT15 OFFSET(2) NUMBITS(1) [],
        ATDATAMBIT7 OFFSET(1) NUMBITS(1) [],
        ATDATAMBIT0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbmctr2 [
        SYNCREQM OFFSET(2) NUMBITS(1) [],
        AFVALIDM OFFSET(1) NUMBITS(1) [],
        ATREADYM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbmctr1 [
        ATIDM OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbmctr0 [
        ATBYTESM OFFSET(8) NUMBITS(2) [],
        AFREADYM OFFSET(1) NUMBITS(1) [],
        ATVALIDM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itmiscop0 [
        FULL OFFSET(1) NUMBITS(1) [],
        ACQCOMP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ittrflin [
        FLUSHIN OFFSET(1) NUMBITS(1) [],
        TRIGIN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbdata0 [
        ATDATASBIT31 OFFSET(4) NUMBITS(1) [],
        ATDATASBIT23 OFFSET(3) NUMBITS(1) [],
        ATDATASBIT15 OFFSET(2) NUMBITS(1) [],
        ATDATASBIT7 OFFSET(1) NUMBITS(1) [],
        ATDATASBIT0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbctr2 [
        SYNCREQS OFFSET(2) NUMBITS(1) [],
        AFVALIDS OFFSET(1) NUMBITS(1) [],
        ATREADYS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbctr1 [
        ATIDS OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itatbctr0 [
        ATBYTESS OFFSET(8) NUMBITS(2) [],
        AFREADYS OFFSET(1) NUMBITS(1) [],
        ATVALIDS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Itctrl [
        INTEGRATION_MODE OFFSET(0) NUMBITS(1) [],
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
        MEMWIDTH OFFSET(8) NUMBITS(3) [],
        CONFIGTYPE OFFSET(6) NUMBITS(2) [],
        CLKSCHEME OFFSET(5) NUMBITS(1) [],
        ATBINPORTCOUNT OFFSET(0) NUMBITS(5) [],
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
        FOURKB_COUNT OFFSET(4) NUMBITS(4) [],
        JEP106_CONT OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr0 [
        PART_NUMBER_BITS7TO0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr1 [
        JEP106_BITS3TO0 OFFSET(4) NUMBITS(4) [],
        PART_NUMBER_BITS11TO8 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC OFFSET(3) NUMBITS(1) [],
        JEP106_BITS6TO4 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pidr3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        CUSTOMER_MODIFIED OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr0 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr1 [
        CLASS OFFSET(4) NUMBITS(4) [],
        PREAMBLE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr2 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cidr3 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];

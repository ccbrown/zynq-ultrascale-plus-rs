// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// VCU Decoder, VCU Decoder
pub static mut VCU_DECODE: *mut Registers = 0xa0020000 as *mut Registers;
register_structs! {
    pub Registers {
        (0x00000000 => _padding0),
        /// MCU Subsystem Reset
        (0x00009000 => pub mcu_reset: Aliased<u32, McuResetR::Register, McuResetW::Register>),
        /// MCU Reset Mode
        (0x00009004 => pub mcu_reset_mode: Aliased<u32, McuResetModeR::Register, McuResetModeW::Register>),
        /// MCU Status
        (0x00009008 => pub mcu_sta: ReadOnly<u32, McuSta::Register>),
        /// MCU Wake-up
        (0x0000900c => pub mcu_wakeup: Aliased<u32, McuWakeupR::Register, McuWakeupW::Register>),
        /// MCU Instruction Cache Address Offset 0
        (0x00009010 => pub mcu_addr_offset_ic0: ReadWrite<u32>),
        /// MCU Instruction Cache Address Offset 1
        (0x00009014 => pub mcu_addr_offset_ic1: ReadWrite<u32>),
        /// MCU Data Cache Address Offset 0
        (0x00009018 => pub mcu_addr_offset_dc0: ReadWrite<u32>),
        /// MCU Data Cache Address Offset 1
        (0x0000901c => pub mcu_addr_offset_dc1: ReadWrite<u32>),
        (0x00009020 => _padding36896),
        /// MCU Interrupt Trigger
        (0x00009100 => pub itc_mcu_irq: Aliased<u32, ItcMcuIrqR::Register, ItcMcuIrqW::Register>),
        /// CPU Interrupt Mask
        (0x00009104 => pub itc_cpu_irq_msk: ReadWrite<u32, ItcCpuIrqMsk::Register>),
        /// CPU Interrupt Clear
        (0x00009108 => pub itc_cpu_irq_clr: Aliased<u32, ItcCpuIrqClrR::Register, ItcCpuIrqClrW::Register>),
        /// CPU Interrupt Status
        (0x0000910c => pub itc_cpu_irq_sta: ReadOnly<u32, ItcCpuIrqSta::Register>),
        (0x00009110 => _padding37136),
        /// AXI Bandwidth Measurement Window
        (0x00009204 => pub axi_bw: ReadWrite<u32>),
        /// Video Data Address Offset
        (0x00009208 => pub axi_addr_offset_ip: ReadWrite<u32>),
        (0x0000920c => _padding37388),
        /// AXI Read Bandwidth Status 0
        (0x00009210 => pub axi_rbw0: ReadOnly<u32>),
        /// AXI Read Bandwidth Status 1
        (0x00009214 => pub axi_rbw1: ReadOnly<u32>),
        /// AXI Write Bandwidth Status 0
        (0x00009218 => pub axi_wbw0: ReadOnly<u32>),
        /// AXI Write Bandwidth Status 1
        (0x0000921c => pub axi_wbw1: ReadOnly<u32>),
        /// AXI Read Bandwidth Limiter 0
        (0x00009220 => pub axi_rbl0: ReadWrite<u32, AxiRbl0::Register>),
        /// AXI Read Bandwidth Limiter 1
        (0x00009224 => pub axi_rbl1: ReadWrite<u32, AxiRbl1::Register>),
        (0x00009228 => @END),
    }
}
register_bitfields! [
    u32,
    pub McuResetR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
    ],
    pub McuResetW [
        MCUREGSOFTRESET OFFSET(1) NUMBITS(1) [],
        MCUSOFTRESET OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub McuResetModeR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        MCURESETMODE OFFSET(0) NUMBITS(2) [],
    ],
    pub McuResetModeW [
        MCURESETMODE OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub McuSta [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        MCUSLEEPSTATUS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub McuWakeupR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        MCUWAKEUP OFFSET(0) NUMBITS(1) [],
    ],
    pub McuWakeupW [
        MCUWAKEUP OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ItcMcuIrqR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub ItcMcuIrqW [
        MCUINTERRUPTTRIGGER OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ItcCpuIrqMsk [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        EXTIRQ1INTERRUPTMASK OFFSET(7) NUMBITS(1) [],
        EXTIRQ0INTERRUPTMASK OFFSET(6) NUMBITS(1) [],
        RRESP1INTERRUPTMASK OFFSET(5) NUMBITS(1) [],
        BRESP1INTERRUPTMASK OFFSET(4) NUMBITS(1) [],
        RRESP0INTERRUPTMASK OFFSET(3) NUMBITS(1) [],
        BRESP0INTERRUPTMASK OFFSET(2) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(1) [],
        MCUTOCPUINTERRUPTMASK OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ItcCpuIrqClrR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        RESERVED1 OFFSET(1) NUMBITS(1) [],
    ],
    pub ItcCpuIrqClrW [
        EXTIRQ1INTERRUPTCLEAR OFFSET(7) NUMBITS(1) [],
        EXTIRQ0INTERRUPTCLEAR OFFSET(6) NUMBITS(1) [],
        RRESP1INTERRUPTCLEAR OFFSET(5) NUMBITS(1) [],
        BRESP1INTERRUPTCLEAR OFFSET(4) NUMBITS(1) [],
        RRESP0INTERRUPTCLEAR OFFSET(3) NUMBITS(1) [],
        BRESP0INTERRUPTCLEAR OFFSET(2) NUMBITS(1) [],
        MCUTOCPUINTERRUPTCLEAR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ItcCpuIrqSta [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        EXTIRQ1INTERRUPTSTATUS OFFSET(7) NUMBITS(1) [],
        EXTIRQ0INTERRUPTSTATUS OFFSET(6) NUMBITS(1) [],
        RRESP1INTERRUPTSTATUS OFFSET(5) NUMBITS(1) [],
        BRESP1INTERRUPTSTATUS OFFSET(4) NUMBITS(1) [],
        RRESP0INTERRUPTSTATUS OFFSET(3) NUMBITS(1) [],
        BRESP0INTERRUPTSTATUS OFFSET(2) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(1) [],
        MCUTOCPUINTERRUPTSTATUS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub AxiRbl0 [
        AXIREADBWLIMTHR0 OFFSET(16) NUMBITS(16) [],
        AXIREADBWLIMWIN0 OFFSET(0) NUMBITS(16) [],
    ]
];
register_bitfields! [
    u32,
    pub AxiRbl1 [
        AXIREADBWLIMTHR1 OFFSET(16) NUMBITS(16) [],
        AXIREADBWLIMWIN1 OFFSET(0) NUMBITS(16) [],
    ]
];

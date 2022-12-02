// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// Inter Processor Interrupts, Inter-Processor Interrupts Control and Status
pub static mut IPI: *mut Registers = 0xff300000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Ch 0 Interrupt Trigger (sender).Default APU MPCore.
        (0x00000000 => pub apu_trig: Aliased<u32, ApuTrigR::Register, ApuTrigW::Register>),
        /// Ch 0 Interrupt Observation (sender).
        (0x00000004 => pub apu_obs: ReadOnly<u32, ApuObs::Register>),
        (0x00000008 => _padding8),
        /// Ch 0 Interrupt Status and Clear (receiver). Default APU MPCore.
        (0x00000010 => pub apu_isr: Aliased<u32, ApuIsrR::Register, ApuIsrW::Register>),
        /// Ch 0 Interrupt Mask (receiver).
        (0x00000014 => pub apu_imr: ReadOnly<u32, ApuImr::Register>),
        /// Ch 0 Interrupt Enable (receiver).
        (0x00000018 => pub apu_ier: Aliased<u32, ApuIerR::Register, ApuIerW::Register>),
        /// Ch 0 Interrupt Disable (receiver).
        (0x0000001c => pub apu_idr: Aliased<u32, ApuIdrR::Register, ApuIdrW::Register>),
        (0x00000020 => _padding32),
        /// Ch 1 Interrupt Trigger (sender). Default RPU0.
        (0x00010000 => pub rpu_0_trig: Aliased<u32, Rpu0TrigR::Register, Rpu0TrigW::Register>),
        /// Ch 1 Interrupt Observation (sender).
        (0x00010004 => pub rpu_0_obs: ReadOnly<u32, Rpu0Obs::Register>),
        (0x00010008 => _padding65544),
        /// Ch 1 Interrupt Status and Clear (receiver). Default RPU0.
        (0x00010010 => pub rpu_0_isr: Aliased<u32, Rpu0IsrR::Register, Rpu0IsrW::Register>),
        /// Ch 1 Interrupt Mask (receiver).
        (0x00010014 => pub rpu_0_imr: ReadOnly<u32, Rpu0Imr::Register>),
        /// Ch 1 Interrupt Enable (receiver).
        (0x00010018 => pub rpu_0_ier: Aliased<u32, Rpu0IerR::Register, Rpu0IerW::Register>),
        /// Ch 1 Interrupt Disable (receiver).
        (0x0001001c => pub rpu_0_idr: Aliased<u32, Rpu0IdrR::Register, Rpu0IdrW::Register>),
        (0x00010020 => _padding65568),
        /// Ch 2 Interrupt Trigger (sender). Default RPU1.
        (0x00020000 => pub rpu_1_trig: Aliased<u32, Rpu1TrigR::Register, Rpu1TrigW::Register>),
        /// Ch 2 Interrupt Observation (sender).
        (0x00020004 => pub rpu_1_obs: ReadOnly<u32, Rpu1Obs::Register>),
        (0x00020008 => _padding131080),
        /// Ch 2 Interrupt Status and Clear (receiver). Default RPU1.
        (0x00020010 => pub rpu_1_isr: Aliased<u32, Rpu1IsrR::Register, Rpu1IsrW::Register>),
        /// Ch 2 Interrupt Mask (receiver).
        (0x00020014 => pub rpu_1_imr: ReadOnly<u32, Rpu1Imr::Register>),
        /// Ch 2 Interrupt Enable (receiver).
        (0x00020018 => pub rpu_1_ier: Aliased<u32, Rpu1IerR::Register, Rpu1IerW::Register>),
        /// Ch 1 Interrupt Disable (receiver).
        (0x0002001c => pub rpu_1_idr: Aliased<u32, Rpu1IdrR::Register, Rpu1IdrW::Register>),
        (0x00020020 => _padding131104),
        /// PMU 0 Interrupt Trigger (sender).
        (0x00030000 => pub pmu_0_trig: Aliased<u32, Pmu0TrigR::Register, Pmu0TrigW::Register>),
        /// PMU 0 Interrupt Observation (sender).
        (0x00030004 => pub pmu_0_obs: ReadOnly<u32, Pmu0Obs::Register>),
        (0x00030008 => _padding196616),
        /// PMU 0 Interrupt Status and Clear (receiver).
        (0x00030010 => pub pmu_0_isr: Aliased<u32, Pmu0IsrR::Register, Pmu0IsrW::Register>),
        /// PMU 0 Interrupt Mask (receiver).
        (0x00030014 => pub pmu_0_imr: ReadOnly<u32, Pmu0Imr::Register>),
        /// PMU 0 Interrupt Enable (receiver).
        (0x00030018 => pub pmu_0_ier: Aliased<u32, Pmu0IerR::Register, Pmu0IerW::Register>),
        /// PMU 0 Interrupt Disable (receiver).
        (0x0003001c => pub pmu_0_idr: Aliased<u32, Pmu0IdrR::Register, Pmu0IdrW::Register>),
        (0x00030020 => _padding196640),
        /// PMU 1 Interrupt Trigger (sender).
        (0x00031000 => pub pmu_1_trig: Aliased<u32, Pmu1TrigR::Register, Pmu1TrigW::Register>),
        /// PMU 1 Interrupt Observation (sender).
        (0x00031004 => pub pmu_1_obs: ReadOnly<u32, Pmu1Obs::Register>),
        (0x00031008 => _padding200712),
        /// PMU 1 Interrupt Status and Clear (receiver).
        (0x00031010 => pub pmu_1_isr: Aliased<u32, Pmu1IsrR::Register, Pmu1IsrW::Register>),
        /// PMU 1 Interrupt Mask (receiver).
        (0x00031014 => pub pmu_1_imr: ReadOnly<u32, Pmu1Imr::Register>),
        /// PMU 1 Interrupt Enable (receiver).
        (0x00031018 => pub pmu_1_ier: Aliased<u32, Pmu1IerR::Register, Pmu1IerW::Register>),
        /// PMU 1 Interrupt Disable (receiver).
        (0x0003101c => pub pmu_1_idr: Aliased<u32, Pmu1IdrR::Register, Pmu1IdrW::Register>),
        (0x00031020 => _padding200736),
        /// PMU 2 Interrupt Trigger (sender).
        (0x00032000 => pub pmu_2_trig: Aliased<u32, Pmu2TrigR::Register, Pmu2TrigW::Register>),
        /// PMU 2 Interrupt Observation (sender).
        (0x00032004 => pub pmu_2_obs: ReadOnly<u32, Pmu2Obs::Register>),
        (0x00032008 => _padding204808),
        /// PMU 2 Interrupt Status and Clear (receiver).
        (0x00032010 => pub pmu_2_isr: Aliased<u32, Pmu2IsrR::Register, Pmu2IsrW::Register>),
        /// PMU 2 Interrupt Mask (receiver).
        (0x00032014 => pub pmu_2_imr: ReadOnly<u32, Pmu2Imr::Register>),
        /// PMU 2 Interrupt Enable (receiver).
        (0x00032018 => pub pmu_2_ier: Aliased<u32, Pmu2IerR::Register, Pmu2IerW::Register>),
        /// PMU 2 Interrupt Disable (receiver).
        (0x0003201c => pub pmu_2_idr: Aliased<u32, Pmu2IdrR::Register, Pmu2IdrW::Register>),
        (0x00032020 => _padding204832),
        /// PMU 3 Interrupt Trigger (sender).
        (0x00033000 => pub pmu_3_trig: Aliased<u32, Pmu3TrigR::Register, Pmu3TrigW::Register>),
        /// PMU 3 Interrupt Observation (sender).
        (0x00033004 => pub pmu_3_obs: ReadOnly<u32, Pmu3Obs::Register>),
        (0x00033008 => _padding208904),
        /// PMU 3 Interrupt Status and Clear (receiver).
        (0x00033010 => pub pmu_3_isr: Aliased<u32, Pmu3IsrR::Register, Pmu3IsrW::Register>),
        /// PMU 3 Interrupt Mask (receiver).
        (0x00033014 => pub pmu_3_imr: ReadOnly<u32, Pmu3Imr::Register>),
        /// PMU 3 Interrupt Enable (receiver).
        (0x00033018 => pub pmu_3_ier: Aliased<u32, Pmu3IerR::Register, Pmu3IerW::Register>),
        /// PMU 3 Interrupt Disable (receiver).
        (0x0003301c => pub pmu_3_idr: Aliased<u32, Pmu3IdrR::Register, Pmu3IdrW::Register>),
        (0x00033020 => _padding208928),
        /// Ch 7 Interrupt Trigger (sender). Default PL 0.
        (0x00040000 => pub pl_0_trig: Aliased<u32, Pl0TrigR::Register, Pl0TrigW::Register>),
        /// Ch 7 Interrupt Observation (sender).
        (0x00040004 => pub pl_0_obs: ReadOnly<u32, Pl0Obs::Register>),
        (0x00040008 => _padding262152),
        /// Ch 7 Interrupt Status and Clear (receiver). Default PL 0.
        (0x00040010 => pub pl_0_isr: Aliased<u32, Pl0IsrR::Register, Pl0IsrW::Register>),
        /// Ch 7 Interrupt Mask (receiver).
        (0x00040014 => pub pl_0_imr: ReadOnly<u32, Pl0Imr::Register>),
        /// Ch 7 Interrupt Enable (receiver).
        (0x00040018 => pub pl_0_ier: Aliased<u32, Pl0IerR::Register, Pl0IerW::Register>),
        /// Ch 7 Interrupt Disable (receiver).
        (0x0004001c => pub pl_0_idr: Aliased<u32, Pl0IdrR::Register, Pl0IdrW::Register>),
        (0x00040020 => _padding262176),
        /// Ch 8 Interrupt Trigger (sender). Default PL 1.
        (0x00050000 => pub pl_1_trig: Aliased<u32, Pl1TrigR::Register, Pl1TrigW::Register>),
        /// Ch 8 Interrupt Observation (sender).
        (0x00050004 => pub pl_1_obs: ReadOnly<u32, Pl1Obs::Register>),
        (0x00050008 => _padding327688),
        /// Ch 8 Interrupt Status and Clear (receiver). Default PL 1.
        (0x00050010 => pub pl_1_isr: Aliased<u32, Pl1IsrR::Register, Pl1IsrW::Register>),
        /// Ch 8 Interrupt Mask (receiver).
        (0x00050014 => pub pl_1_imr: ReadOnly<u32, Pl1Imr::Register>),
        /// Ch 8 Interrupt Enable (receiver).
        (0x00050018 => pub pl_1_ier: Aliased<u32, Pl1IerR::Register, Pl1IerW::Register>),
        /// Ch 8 Interrupt Disable (receiver).
        (0x0005001c => pub pl_1_idr: Aliased<u32, Pl1IdrR::Register, Pl1IdrW::Register>),
        (0x00050020 => _padding327712),
        /// Ch 9 Interrupt Trigger (sender). Default PL 2.
        (0x00060000 => pub pl_2_trig: Aliased<u32, Pl2TrigR::Register, Pl2TrigW::Register>),
        /// Ch 9 Interrupt Observation (sender).
        (0x00060004 => pub pl_2_obs: ReadOnly<u32, Pl2Obs::Register>),
        (0x00060008 => _padding393224),
        /// Ch 9 Interrupt Status and Clear (receiver). Default PL 2.
        (0x00060010 => pub pl_2_isr: Aliased<u32, Pl2IsrR::Register, Pl2IsrW::Register>),
        /// Ch 9 Interrupt Mask (receiver).
        (0x00060014 => pub pl_2_imr: ReadOnly<u32, Pl2Imr::Register>),
        /// Ch 9 Interrupt Enable (receiver).
        (0x00060018 => pub pl_2_ier: Aliased<u32, Pl2IerR::Register, Pl2IerW::Register>),
        /// Ch 9 Interrupt Disable (receiver).
        (0x0006001c => pub pl_2_idr: Aliased<u32, Pl2IdrR::Register, Pl2IdrW::Register>),
        (0x00060020 => _padding393248),
        /// Ch 10 Interrupt Trigger (sender). Default PL 3.
        (0x00070000 => pub pl_3_trig: Aliased<u32, Pl3TrigR::Register, Pl3TrigW::Register>),
        /// Ch 10 Interrupt Observation (sender).
        (0x00070004 => pub pl_3_obs: ReadOnly<u32, Pl3Obs::Register>),
        (0x00070008 => _padding458760),
        /// Ch 10 Interrupt Status and Clear (receiver). Default PL 3.
        (0x00070010 => pub pl_3_isr: Aliased<u32, Pl3IsrR::Register, Pl3IsrW::Register>),
        /// Ch 10 Interrupt Mask (receiver).
        (0x00070014 => pub pl_3_imr: ReadOnly<u32, Pl3Imr::Register>),
        /// Ch 10 Interrupt Enable (receiver).
        (0x00070018 => pub pl_3_ier: Aliased<u32, Pl3IerR::Register, Pl3IerW::Register>),
        /// Ch 10 Interrupt Disable (receiver).
        (0x0007001c => pub pl_3_idr: Aliased<u32, Pl3IdrR::Register, Pl3IdrW::Register>),
        (0x00070020 => _padding458784),
        /// IPI Controller Error Signal Control.
        (0x00080000 => pub ipi_ctrl: ReadWrite<u32, IpiCtrl::Register>),
        (0x00080004 => _padding524292),
        /// IPI Controller Interrupt Status and Clear.
        (0x00080010 => pub ipi_isr: Aliased<u32, IpiIsrR::Register, IpiIsrW::Register>),
        /// IPI Controller Interrupt Mask.
        (0x00080014 => pub ipi_imr: ReadOnly<u32, IpiImr::Register>),
        /// IPI Controller Interrupt Enable.
        (0x00080018 => pub ipi_ier: Aliased<u32, IpiIerR::Register, IpiIerW::Register>),
        (0x0008001c => _padding524316),
        /// Scratch register for interconnect data path checking
        (0x00080030 => pub safety_chk: ReadWrite<u32>),
        (0x00080034 => _padding524340),
        /// IPI Controller Interrupt Disable.
        (0x000c001c => pub ipi_idr: Aliased<u32, IpiIdrR::Register, IpiIdrW::Register>),
        (0x000c0020 => @END),
    }
}
register_bitfields! [
    u32,
    pub ApuTrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub ApuTrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ApuObs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ApuIsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub ApuIsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ApuImr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ApuIerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub ApuIerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ApuIdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub ApuIdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu0TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Rpu0TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu0Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu0IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu0IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu0Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu0IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Rpu0IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu0IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Rpu0IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu1TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Rpu1TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu1Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu1IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Rpu1IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu1Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu1IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Rpu1IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Rpu1IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Rpu1IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu0TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu0TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu0Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu0IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Pmu0IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu0Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu0IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu0IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu0IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu0IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu1TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu1TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu1Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu1IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Pmu1IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu1Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu1IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu1IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu1IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu1IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu2TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu2TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu2Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu2IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Pmu2IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu2Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu2IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu2IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu2IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu2IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu3TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu3TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu3Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu3IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Pmu3IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu3Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu3IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu3IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pmu3IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pmu3IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl0TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl0TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl0Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl0IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Pl0IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl0Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl0IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl0IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl0IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl0IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl1TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl1TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl1Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl1IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Pl1IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl1Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl1IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl1IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl1IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl1IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl2TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl2TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl2Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl2IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Pl2IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl2Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl2IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl2IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl2IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl2IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl3TrigR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl3TrigW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl3Obs [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl3IsrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ],
    pub Pl3IsrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl3Imr [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl3IerR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl3IerW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Pl3IdrR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        RESERVED1 OFFSET(20) NUMBITS(4) [],
        RESERVED2 OFFSET(10) NUMBITS(6) [],
        RESERVED3 OFFSET(1) NUMBITS(7) [],
    ],
    pub Pl3IdrW [
        PL_3 OFFSET(27) NUMBITS(1) [],
        PL_2 OFFSET(26) NUMBITS(1) [],
        PL_1 OFFSET(25) NUMBITS(1) [],
        PL_0 OFFSET(24) NUMBITS(1) [],
        PMU_3 OFFSET(19) NUMBITS(1) [],
        PMU_2 OFFSET(18) NUMBITS(1) [],
        PMU_1 OFFSET(17) NUMBITS(1) [],
        PMU_0 OFFSET(16) NUMBITS(1) [],
        RPU_1 OFFSET(9) NUMBITS(1) [],
        RPU_0 OFFSET(8) NUMBITS(1) [],
        APU OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IpiCtrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IpiIsrR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ],
    pub IpiIsrW [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IpiImr [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IpiIerR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub IpiIerW [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub IpiIdrR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub IpiIdrW [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];

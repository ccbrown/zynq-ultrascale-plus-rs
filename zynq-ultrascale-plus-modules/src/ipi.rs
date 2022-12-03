// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// Inter Processor Interrupts, Inter-Processor Interrupts Control and Status
pub static mut IPI: *mut Registers = 0xff300000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Ch 0 Interrupt Trigger (sender).Default APU MPCore.
    pub apu_trig: Aliased<u32, ApuTrigR::Register, ApuTrigW::Register>,
    /// Ch 0 Interrupt Observation (sender).
    pub apu_obs: ReadOnly<u32, ApuObs::Register>,
    _padding8: [u8; 8],
    /// Ch 0 Interrupt Status and Clear (receiver). Default APU MPCore.
    pub apu_isr: Aliased<u32, ApuIsrR::Register, ApuIsrW::Register>,
    /// Ch 0 Interrupt Mask (receiver).
    pub apu_imr: ReadOnly<u32, ApuImr::Register>,
    /// Ch 0 Interrupt Enable (receiver).
    pub apu_ier: Aliased<u32, ApuIerR::Register, ApuIerW::Register>,
    /// Ch 0 Interrupt Disable (receiver).
    pub apu_idr: Aliased<u32, ApuIdrR::Register, ApuIdrW::Register>,
    _padding32: [u8; 65504],
    /// Ch 1 Interrupt Trigger (sender). Default RPU0.
    pub rpu_0_trig: Aliased<u32, Rpu0TrigR::Register, Rpu0TrigW::Register>,
    /// Ch 1 Interrupt Observation (sender).
    pub rpu_0_obs: ReadOnly<u32, Rpu0Obs::Register>,
    _padding65544: [u8; 8],
    /// Ch 1 Interrupt Status and Clear (receiver). Default RPU0.
    pub rpu_0_isr: Aliased<u32, Rpu0IsrR::Register, Rpu0IsrW::Register>,
    /// Ch 1 Interrupt Mask (receiver).
    pub rpu_0_imr: ReadOnly<u32, Rpu0Imr::Register>,
    /// Ch 1 Interrupt Enable (receiver).
    pub rpu_0_ier: Aliased<u32, Rpu0IerR::Register, Rpu0IerW::Register>,
    /// Ch 1 Interrupt Disable (receiver).
    pub rpu_0_idr: Aliased<u32, Rpu0IdrR::Register, Rpu0IdrW::Register>,
    _padding65568: [u8; 65504],
    /// Ch 2 Interrupt Trigger (sender). Default RPU1.
    pub rpu_1_trig: Aliased<u32, Rpu1TrigR::Register, Rpu1TrigW::Register>,
    /// Ch 2 Interrupt Observation (sender).
    pub rpu_1_obs: ReadOnly<u32, Rpu1Obs::Register>,
    _padding131080: [u8; 8],
    /// Ch 2 Interrupt Status and Clear (receiver). Default RPU1.
    pub rpu_1_isr: Aliased<u32, Rpu1IsrR::Register, Rpu1IsrW::Register>,
    /// Ch 2 Interrupt Mask (receiver).
    pub rpu_1_imr: ReadOnly<u32, Rpu1Imr::Register>,
    /// Ch 2 Interrupt Enable (receiver).
    pub rpu_1_ier: Aliased<u32, Rpu1IerR::Register, Rpu1IerW::Register>,
    /// Ch 1 Interrupt Disable (receiver).
    pub rpu_1_idr: Aliased<u32, Rpu1IdrR::Register, Rpu1IdrW::Register>,
    _padding131104: [u8; 65504],
    /// PMU 0 Interrupt Trigger (sender).
    pub pmu_0_trig: Aliased<u32, Pmu0TrigR::Register, Pmu0TrigW::Register>,
    /// PMU 0 Interrupt Observation (sender).
    pub pmu_0_obs: ReadOnly<u32, Pmu0Obs::Register>,
    _padding196616: [u8; 8],
    /// PMU 0 Interrupt Status and Clear (receiver).
    pub pmu_0_isr: Aliased<u32, Pmu0IsrR::Register, Pmu0IsrW::Register>,
    /// PMU 0 Interrupt Mask (receiver).
    pub pmu_0_imr: ReadOnly<u32, Pmu0Imr::Register>,
    /// PMU 0 Interrupt Enable (receiver).
    pub pmu_0_ier: Aliased<u32, Pmu0IerR::Register, Pmu0IerW::Register>,
    /// PMU 0 Interrupt Disable (receiver).
    pub pmu_0_idr: Aliased<u32, Pmu0IdrR::Register, Pmu0IdrW::Register>,
    _padding196640: [u8; 4064],
    /// PMU 1 Interrupt Trigger (sender).
    pub pmu_1_trig: Aliased<u32, Pmu1TrigR::Register, Pmu1TrigW::Register>,
    /// PMU 1 Interrupt Observation (sender).
    pub pmu_1_obs: ReadOnly<u32, Pmu1Obs::Register>,
    _padding200712: [u8; 8],
    /// PMU 1 Interrupt Status and Clear (receiver).
    pub pmu_1_isr: Aliased<u32, Pmu1IsrR::Register, Pmu1IsrW::Register>,
    /// PMU 1 Interrupt Mask (receiver).
    pub pmu_1_imr: ReadOnly<u32, Pmu1Imr::Register>,
    /// PMU 1 Interrupt Enable (receiver).
    pub pmu_1_ier: Aliased<u32, Pmu1IerR::Register, Pmu1IerW::Register>,
    /// PMU 1 Interrupt Disable (receiver).
    pub pmu_1_idr: Aliased<u32, Pmu1IdrR::Register, Pmu1IdrW::Register>,
    _padding200736: [u8; 4064],
    /// PMU 2 Interrupt Trigger (sender).
    pub pmu_2_trig: Aliased<u32, Pmu2TrigR::Register, Pmu2TrigW::Register>,
    /// PMU 2 Interrupt Observation (sender).
    pub pmu_2_obs: ReadOnly<u32, Pmu2Obs::Register>,
    _padding204808: [u8; 8],
    /// PMU 2 Interrupt Status and Clear (receiver).
    pub pmu_2_isr: Aliased<u32, Pmu2IsrR::Register, Pmu2IsrW::Register>,
    /// PMU 2 Interrupt Mask (receiver).
    pub pmu_2_imr: ReadOnly<u32, Pmu2Imr::Register>,
    /// PMU 2 Interrupt Enable (receiver).
    pub pmu_2_ier: Aliased<u32, Pmu2IerR::Register, Pmu2IerW::Register>,
    /// PMU 2 Interrupt Disable (receiver).
    pub pmu_2_idr: Aliased<u32, Pmu2IdrR::Register, Pmu2IdrW::Register>,
    _padding204832: [u8; 4064],
    /// PMU 3 Interrupt Trigger (sender).
    pub pmu_3_trig: Aliased<u32, Pmu3TrigR::Register, Pmu3TrigW::Register>,
    /// PMU 3 Interrupt Observation (sender).
    pub pmu_3_obs: ReadOnly<u32, Pmu3Obs::Register>,
    _padding208904: [u8; 8],
    /// PMU 3 Interrupt Status and Clear (receiver).
    pub pmu_3_isr: Aliased<u32, Pmu3IsrR::Register, Pmu3IsrW::Register>,
    /// PMU 3 Interrupt Mask (receiver).
    pub pmu_3_imr: ReadOnly<u32, Pmu3Imr::Register>,
    /// PMU 3 Interrupt Enable (receiver).
    pub pmu_3_ier: Aliased<u32, Pmu3IerR::Register, Pmu3IerW::Register>,
    /// PMU 3 Interrupt Disable (receiver).
    pub pmu_3_idr: Aliased<u32, Pmu3IdrR::Register, Pmu3IdrW::Register>,
    _padding208928: [u8; 53216],
    /// Ch 7 Interrupt Trigger (sender). Default PL 0.
    pub pl_0_trig: Aliased<u32, Pl0TrigR::Register, Pl0TrigW::Register>,
    /// Ch 7 Interrupt Observation (sender).
    pub pl_0_obs: ReadOnly<u32, Pl0Obs::Register>,
    _padding262152: [u8; 8],
    /// Ch 7 Interrupt Status and Clear (receiver). Default PL 0.
    pub pl_0_isr: Aliased<u32, Pl0IsrR::Register, Pl0IsrW::Register>,
    /// Ch 7 Interrupt Mask (receiver).
    pub pl_0_imr: ReadOnly<u32, Pl0Imr::Register>,
    /// Ch 7 Interrupt Enable (receiver).
    pub pl_0_ier: Aliased<u32, Pl0IerR::Register, Pl0IerW::Register>,
    /// Ch 7 Interrupt Disable (receiver).
    pub pl_0_idr: Aliased<u32, Pl0IdrR::Register, Pl0IdrW::Register>,
    _padding262176: [u8; 65504],
    /// Ch 8 Interrupt Trigger (sender). Default PL 1.
    pub pl_1_trig: Aliased<u32, Pl1TrigR::Register, Pl1TrigW::Register>,
    /// Ch 8 Interrupt Observation (sender).
    pub pl_1_obs: ReadOnly<u32, Pl1Obs::Register>,
    _padding327688: [u8; 8],
    /// Ch 8 Interrupt Status and Clear (receiver). Default PL 1.
    pub pl_1_isr: Aliased<u32, Pl1IsrR::Register, Pl1IsrW::Register>,
    /// Ch 8 Interrupt Mask (receiver).
    pub pl_1_imr: ReadOnly<u32, Pl1Imr::Register>,
    /// Ch 8 Interrupt Enable (receiver).
    pub pl_1_ier: Aliased<u32, Pl1IerR::Register, Pl1IerW::Register>,
    /// Ch 8 Interrupt Disable (receiver).
    pub pl_1_idr: Aliased<u32, Pl1IdrR::Register, Pl1IdrW::Register>,
    _padding327712: [u8; 65504],
    /// Ch 9 Interrupt Trigger (sender). Default PL 2.
    pub pl_2_trig: Aliased<u32, Pl2TrigR::Register, Pl2TrigW::Register>,
    /// Ch 9 Interrupt Observation (sender).
    pub pl_2_obs: ReadOnly<u32, Pl2Obs::Register>,
    _padding393224: [u8; 8],
    /// Ch 9 Interrupt Status and Clear (receiver). Default PL 2.
    pub pl_2_isr: Aliased<u32, Pl2IsrR::Register, Pl2IsrW::Register>,
    /// Ch 9 Interrupt Mask (receiver).
    pub pl_2_imr: ReadOnly<u32, Pl2Imr::Register>,
    /// Ch 9 Interrupt Enable (receiver).
    pub pl_2_ier: Aliased<u32, Pl2IerR::Register, Pl2IerW::Register>,
    /// Ch 9 Interrupt Disable (receiver).
    pub pl_2_idr: Aliased<u32, Pl2IdrR::Register, Pl2IdrW::Register>,
    _padding393248: [u8; 65504],
    /// Ch 10 Interrupt Trigger (sender). Default PL 3.
    pub pl_3_trig: Aliased<u32, Pl3TrigR::Register, Pl3TrigW::Register>,
    /// Ch 10 Interrupt Observation (sender).
    pub pl_3_obs: ReadOnly<u32, Pl3Obs::Register>,
    _padding458760: [u8; 8],
    /// Ch 10 Interrupt Status and Clear (receiver). Default PL 3.
    pub pl_3_isr: Aliased<u32, Pl3IsrR::Register, Pl3IsrW::Register>,
    /// Ch 10 Interrupt Mask (receiver).
    pub pl_3_imr: ReadOnly<u32, Pl3Imr::Register>,
    /// Ch 10 Interrupt Enable (receiver).
    pub pl_3_ier: Aliased<u32, Pl3IerR::Register, Pl3IerW::Register>,
    /// Ch 10 Interrupt Disable (receiver).
    pub pl_3_idr: Aliased<u32, Pl3IdrR::Register, Pl3IdrW::Register>,
    _padding458784: [u8; 65504],
    /// IPI Controller Error Signal Control.
    pub ipi_ctrl: ReadWrite<u32, IpiCtrl::Register>,
    _padding524292: [u8; 12],
    /// IPI Controller Interrupt Status and Clear.
    pub ipi_isr: Aliased<u32, IpiIsrR::Register, IpiIsrW::Register>,
    /// IPI Controller Interrupt Mask.
    pub ipi_imr: ReadOnly<u32, IpiImr::Register>,
    /// IPI Controller Interrupt Enable.
    pub ipi_ier: Aliased<u32, IpiIerR::Register, IpiIerW::Register>,
    _padding524316: [u8; 20],
    /// Scratch register for interconnect data path checking
    pub safety_chk: ReadWrite<u32>,
    _padding524340: [u8; 262120],
    /// IPI Controller Interrupt Disable.
    pub ipi_idr: Aliased<u32, IpiIdrR::Register, IpiIdrW::Register>,
}
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
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
tock_registers::register_bitfields! [
    u32,
    pub IpiCtrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IpiIsrR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ],
    pub IpiIsrW [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IpiImr [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IpiIerR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub IpiIerW [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IpiIdrR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
    ],
    pub IpiIdrW [
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];

// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// Cache Coherent Interconnect GPV, CCI-400 GPV
pub static mut CCI_GPV: *mut Registers = 0xfd6e0000 as *mut Registers;
register_structs! {
    pub Registers {
        /// Control_Override_Register
        (0x00000000 => pub control_override: ReadWrite<u32, ControlOverride::Register>),
        /// Speculation_Control_Register
        (0x00000004 => pub speculation_control: ReadWrite<u32, SpeculationControl::Register>),
        /// Secure_Access_Register
        (0x00000008 => pub secure_access: ReadWrite<u32, SecureAccess::Register>),
        /// Status_Register
        (0x0000000c => pub status: ReadOnly<u32, Status::Register>),
        /// Imprecise_Error_Register
        (0x00000010 => pub imprecise_error: ReadWrite<u32, ImpreciseError::Register>),
        (0x00000014 => _padding20),
        /// Performance_Monitor_Control_Register
        (0x00000100 => pub performance_monitor_control: Aliased<u32, PerformanceMonitorControlR::Register, PerformanceMonitorControlW::Register>),
        (0x00000104 => _padding260),
        /// Snoop_Control_Register_S0
        (0x00001000 => pub snoop_control_register_s0: Aliased<u32, SnoopControlRegisterS0R::Register, SnoopControlRegisterS0W::Register>),
        /// Shareable_Override_Register_S0
        (0x00001004 => pub shareable_override_register_s0: ReadWrite<u32, ShareableOverrideRegisterS0::Register>),
        (0x00001008 => _padding4104),
        /// Read_Qos_Override_Register_S0
        (0x00001100 => pub read_qos_override_register_s0: Aliased<u32, ReadQosOverrideRegisterS0R::Register, ReadQosOverrideRegisterS0W::Register>),
        /// Write_Qos_Override_Register_S0
        (0x00001104 => pub write_qos_override_register_s0: Aliased<u32, WriteQosOverrideRegisterS0R::Register, WriteQosOverrideRegisterS0W::Register>),
        (0x00001108 => _padding4360),
        /// Qos_Control_Register_S0
        (0x0000110c => pub qos_control_register_s0: Aliased<u32, QosControlRegisterS0R::Register, QosControlRegisterS0W::Register>),
        /// Max_OT_Register_S0
        (0x00001110 => pub max_ot_register_s0: ReadWrite<u32, MaxOtRegisterS0::Register>),
        (0x00001114 => _padding4372),
        /// Target_Latency_Register_S0
        (0x00001130 => pub target_latency_register_s0: ReadWrite<u32, TargetLatencyRegisterS0::Register>),
        /// Latency_Regulation_Register_S0
        (0x00001134 => pub latency_regulation_register_s0: ReadWrite<u32, LatencyRegulationRegisterS0::Register>),
        /// Qos_Range_Register_S0
        (0x00001138 => pub qos_range_register_s0: ReadWrite<u32, QosRangeRegisterS0::Register>),
        (0x0000113c => _padding4412),
        /// Snoop_Control_Register_S1
        (0x00002000 => pub snoop_control_register_s1: ReadOnly<u32, SnoopControlRegisterS1::Register>),
        /// Shareable_Override_Register_S1
        (0x00002004 => pub shareable_override_register_s1: ReadWrite<u32, ShareableOverrideRegisterS1::Register>),
        (0x00002008 => _padding8200),
        /// Read_Qos_Override_Register_S1
        (0x00002100 => pub read_qos_override_register_s1: Aliased<u32, ReadQosOverrideRegisterS1R::Register, ReadQosOverrideRegisterS1W::Register>),
        /// Write_Qos_Override_Register_S1
        (0x00002104 => pub write_qos_override_register_s1: Aliased<u32, WriteQosOverrideRegisterS1R::Register, WriteQosOverrideRegisterS1W::Register>),
        (0x00002108 => _padding8456),
        /// Qos_Control_Register_S1
        (0x0000210c => pub qos_control_register_s1: Aliased<u32, QosControlRegisterS1R::Register, QosControlRegisterS1W::Register>),
        /// Max_OT_Register_S1
        (0x00002110 => pub max_ot_register_s1: ReadWrite<u32, MaxOtRegisterS1::Register>),
        (0x00002114 => _padding8468),
        /// Target_Latency_Register_S1
        (0x00002130 => pub target_latency_register_s1: ReadWrite<u32, TargetLatencyRegisterS1::Register>),
        /// Latency_Regulation_Register_S1
        (0x00002134 => pub latency_regulation_register_s1: ReadWrite<u32, LatencyRegulationRegisterS1::Register>),
        /// Qos_Range_Register_S1
        (0x00002138 => pub qos_range_register_s1: ReadWrite<u32, QosRangeRegisterS1::Register>),
        (0x0000213c => _padding8508),
        /// Snoop_Control_Register_S2
        (0x00003000 => pub snoop_control_register_s2: ReadOnly<u32, SnoopControlRegisterS2::Register>),
        /// Shareable_Override_Register_S2
        (0x00003004 => pub shareable_override_register_s2: ReadWrite<u32, ShareableOverrideRegisterS2::Register>),
        (0x00003008 => _padding12296),
        /// Read_Qos_Override_Register_S2
        (0x00003100 => pub read_qos_override_register_s2: Aliased<u32, ReadQosOverrideRegisterS2R::Register, ReadQosOverrideRegisterS2W::Register>),
        /// Write_Qos_Override_Register_S2
        (0x00003104 => pub write_qos_override_register_s2: Aliased<u32, WriteQosOverrideRegisterS2R::Register, WriteQosOverrideRegisterS2W::Register>),
        (0x00003108 => _padding12552),
        /// Qos_Control_Register_S2
        (0x0000310c => pub qos_control_register_s2: Aliased<u32, QosControlRegisterS2R::Register, QosControlRegisterS2W::Register>),
        /// Max_OT_Register_S2
        (0x00003110 => pub max_ot_register_s2: ReadWrite<u32, MaxOtRegisterS2::Register>),
        (0x00003114 => _padding12564),
        /// Target_Latency_Register_S2
        (0x00003130 => pub target_latency_register_s2: ReadWrite<u32, TargetLatencyRegisterS2::Register>),
        /// Latency_Regulation_Register_S2
        (0x00003134 => pub latency_regulation_register_s2: ReadWrite<u32, LatencyRegulationRegisterS2::Register>),
        /// Qos_Range_Register_S2
        (0x00003138 => pub qos_range_register_s2: ReadWrite<u32, QosRangeRegisterS2::Register>),
        (0x0000313c => _padding12604),
        /// Snoop_Control_Register_S3
        (0x00004000 => pub snoop_control_register_s3: Aliased<u32, SnoopControlRegisterS3R::Register, SnoopControlRegisterS3W::Register>),
        (0x00004004 => _padding16388),
        /// Read_Qos_Override_Register_S3
        (0x00004100 => pub read_qos_override_register_s3: Aliased<u32, ReadQosOverrideRegisterS3R::Register, ReadQosOverrideRegisterS3W::Register>),
        /// Write_Qos_Override_Register_S3
        (0x00004104 => pub write_qos_override_register_s3: Aliased<u32, WriteQosOverrideRegisterS3R::Register, WriteQosOverrideRegisterS3W::Register>),
        (0x00004108 => _padding16648),
        /// Qos_Control_Register_S3
        (0x0000410c => pub qos_control_register_s3: Aliased<u32, QosControlRegisterS3R::Register, QosControlRegisterS3W::Register>),
        (0x00004110 => _padding16656),
        /// Target_Latency_Register_S3
        (0x00004130 => pub target_latency_register_s3: ReadWrite<u32, TargetLatencyRegisterS3::Register>),
        /// Latency_Regulation_Register_S3
        (0x00004134 => pub latency_regulation_register_s3: ReadWrite<u32, LatencyRegulationRegisterS3::Register>),
        /// Qos_Range_Register_S3
        (0x00004138 => pub qos_range_register_s3: ReadWrite<u32, QosRangeRegisterS3::Register>),
        (0x0000413c => _padding16700),
        /// Snoop_Control_Register_S4
        (0x00005000 => pub snoop_control_register_s4: Aliased<u32, SnoopControlRegisterS4R::Register, SnoopControlRegisterS4W::Register>),
        (0x00005004 => _padding20484),
        /// Read_Qos_Override_Register_S4
        (0x00005100 => pub read_qos_override_register_s4: Aliased<u32, ReadQosOverrideRegisterS4R::Register, ReadQosOverrideRegisterS4W::Register>),
        /// Write_Qos_Override_Register_S4
        (0x00005104 => pub write_qos_override_register_s4: Aliased<u32, WriteQosOverrideRegisterS4R::Register, WriteQosOverrideRegisterS4W::Register>),
        (0x00005108 => _padding20744),
        /// Qos_Control_Register_S4
        (0x0000510c => pub qos_control_register_s4: Aliased<u32, QosControlRegisterS4R::Register, QosControlRegisterS4W::Register>),
        (0x00005110 => _padding20752),
        /// Target_Latency_Register_S4
        (0x00005130 => pub target_latency_register_s4: ReadWrite<u32, TargetLatencyRegisterS4::Register>),
        /// Latency_Regulation_Register_S4
        (0x00005134 => pub latency_regulation_register_s4: ReadWrite<u32, LatencyRegulationRegisterS4::Register>),
        /// Qos_Range_Register_S4
        (0x00005138 => pub qos_range_register_s4: ReadWrite<u32, QosRangeRegisterS4::Register>),
        (0x0000513c => _padding20796),
        /// Cycle_Counter
        (0x00009004 => pub cycle_counter: ReadWrite<u32>),
        /// Cycle_Counter_Control
        (0x00009008 => pub cycle_counter_control: ReadWrite<u32, CycleCounterControl::Register>),
        /// Cycle_Count_Overflow
        (0x0000900c => pub cycle_count_overflow: ReadWrite<u32, CycleCountOverflow::Register>),
        (0x00009010 => _padding36880),
        /// ESR0
        (0x0000a000 => pub esr0: ReadWrite<u32, Esr0::Register>),
        /// Event_Counter0
        (0x0000a004 => pub event_counter0: ReadWrite<u32>),
        /// Event_Counter0_Control
        (0x0000a008 => pub event_counter0_control: ReadWrite<u32, EventCounter0Control::Register>),
        /// Event_Counter0_Overflow
        (0x0000a00c => pub event_counter0_overflow: ReadWrite<u32, EventCounter0Overflow::Register>),
        (0x0000a010 => _padding40976),
        /// ESR1
        (0x0000b000 => pub esr1: ReadWrite<u32, Esr1::Register>),
        /// Event_Counter1
        (0x0000b004 => pub event_counter1: ReadWrite<u32>),
        /// Event_Counter1_Control
        (0x0000b008 => pub event_counter1_control: ReadWrite<u32, EventCounter1Control::Register>),
        /// Event_Counter1_Overflow
        (0x0000b00c => pub event_counter1_overflow: ReadWrite<u32, EventCounter1Overflow::Register>),
        (0x0000b010 => _padding45072),
        /// ESR2
        (0x0000c000 => pub esr2: ReadWrite<u32, Esr2::Register>),
        /// Event_Counter2
        (0x0000c004 => pub event_counter2: ReadWrite<u32>),
        /// Event_Counter2_Control
        (0x0000c008 => pub event_counter2_control: ReadWrite<u32, EventCounter2Control::Register>),
        /// Event_Counter2_Overflow
        (0x0000c00c => pub event_counter2_overflow: ReadWrite<u32, EventCounter2Overflow::Register>),
        (0x0000c010 => _padding49168),
        /// ESR3
        (0x0000d000 => pub esr3: ReadWrite<u32, Esr3::Register>),
        /// Event_Counter3
        (0x0000d004 => pub event_counter3: ReadWrite<u32>),
        /// Event_Counter3_Control
        (0x0000d008 => pub event_counter3_control: ReadWrite<u32, EventCounter3Control::Register>),
        /// Event_Counter3_Overflow
        (0x0000d00c => pub event_counter3_overflow: ReadWrite<u32, EventCounter3Overflow::Register>),
        (0x0000d010 => @END),
    }
}
register_bitfields! [
    u32,
    pub ControlOverride [
        DISABLE_RETRY_REDUCTION_BUFFERS OFFSET(5) NUMBITS(1) [],
        DISABLE_PRIORITY_PROMOTION OFFSET(4) NUMBITS(1) [],
        TERMINATE_BARRIERS OFFSET(3) NUMBITS(1) [],
        DISABLE_SPECULATIVE_FETCHES OFFSET(2) NUMBITS(1) [],
        DVM_MESSAGE_DISABLE OFFSET(1) NUMBITS(1) [],
        SNOOP_DISABLE OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub SpeculationControl [
        DISABLE_SPECULATIVE_FETCHES_S4 OFFSET(20) NUMBITS(1) [],
        DISABLE_SPECULATIVE_FETCHES_S3 OFFSET(19) NUMBITS(1) [],
        DISABLE_SPECULATIVE_FETCHES_S2 OFFSET(18) NUMBITS(1) [],
        DISABLE_SPECULATIVE_FETCHES_S1 OFFSET(17) NUMBITS(1) [],
        DISABLE_SPECULATIVE_FETCHES_S0 OFFSET(16) NUMBITS(1) [],
        DISABLE_SPECULATIVE_FETCHES_M2 OFFSET(2) NUMBITS(1) [],
        DISABLE_SPECULATIVE_FETCHES_M1 OFFSET(1) NUMBITS(1) [],
        DISABLE_SPECULATIVE_FETCHES_M0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub SecureAccess [
        SECURE_ACCESS_CONTROL OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Status [
        CCI_STATUS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ImpreciseError [
        IMP_ERR_S4 OFFSET(20) NUMBITS(1) [],
        IMP_ERR_S3 OFFSET(19) NUMBITS(1) [],
        IMP_ERR_S2 OFFSET(18) NUMBITS(1) [],
        IMP_ERR_S1 OFFSET(17) NUMBITS(1) [],
        IMP_ERR_S0 OFFSET(16) NUMBITS(1) [],
        IMP_ERR_M2 OFFSET(2) NUMBITS(1) [],
        IMP_ERR_M1 OFFSET(1) NUMBITS(1) [],
        IMP_ERR_M0 OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub PerformanceMonitorControlR [
        PMU_COUNT_NUM OFFSET(11) NUMBITS(5) [],
        DP OFFSET(5) NUMBITS(1) [],
        EX OFFSET(4) NUMBITS(1) [],
        CCD OFFSET(3) NUMBITS(1) [],
        CEN OFFSET(0) NUMBITS(1) [],
    ],
    pub PerformanceMonitorControlW [
        DP OFFSET(5) NUMBITS(1) [],
        EX OFFSET(4) NUMBITS(1) [],
        CCD OFFSET(3) NUMBITS(1) [],
        CCR OFFSET(2) NUMBITS(1) [],
        RST OFFSET(1) NUMBITS(1) [],
        CEN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub SnoopControlRegisterS0R [
        SUPPORT_DVMS OFFSET(31) NUMBITS(1) [],
        SUPPORT_SNOOPS OFFSET(30) NUMBITS(1) [],
        ENABLE_DVMS OFFSET(1) NUMBITS(1) [],
    ],
    pub SnoopControlRegisterS0W [
        ENABLE_DVMS OFFSET(1) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ShareableOverrideRegisterS0 [
        AXDOMAIN_OVERRIDE OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub ReadQosOverrideRegisterS0R [
        ARQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub ReadQosOverrideRegisterS0W [
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub WriteQosOverrideRegisterS0R [
        AWQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub WriteQosOverrideRegisterS0W [
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub QosControlRegisterS0R [
        QOS_REGULATION_DISABLED OFFSET(31) NUMBITS(1) [],
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        AR_OT_REGULATION OFFSET(3) NUMBITS(1) [],
        AW_OT_REGULATION OFFSET(2) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ],
    pub QosControlRegisterS0W [
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        AR_OT_REGULATION OFFSET(3) NUMBITS(1) [],
        AW_OT_REGULATION OFFSET(2) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub MaxOtRegisterS0 [
        INT_OT_AR OFFSET(24) NUMBITS(6) [],
        FRAC_OT_AR OFFSET(16) NUMBITS(8) [],
        INT_OT_AW OFFSET(8) NUMBITS(6) [],
        FRAC_OT_AW OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TargetLatencyRegisterS0 [
        AR_LAT OFFSET(16) NUMBITS(12) [],
        AW_LAT OFFSET(0) NUMBITS(12) [],
    ]
];
register_bitfields! [
    u32,
    pub LatencyRegulationRegisterS0 [
        AR_SCALE_FACT OFFSET(8) NUMBITS(3) [],
        AW_SCALE_FACT OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub QosRangeRegisterS0 [
        MAX_ARQOS OFFSET(24) NUMBITS(4) [],
        MIN_ARQOS OFFSET(16) NUMBITS(4) [],
        MAX_AWQOS OFFSET(8) NUMBITS(4) [],
        MIN_AWQOS OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub SnoopControlRegisterS1 [
        SUPPORT_DVMS OFFSET(31) NUMBITS(1) [],
        SUPPORT_SNOOPS OFFSET(30) NUMBITS(1) [],
        ENABLE_DVMS OFFSET(1) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ShareableOverrideRegisterS1 [
        AXDOMAIN_OVERRIDE OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub ReadQosOverrideRegisterS1R [
        ARQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub ReadQosOverrideRegisterS1W [
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub WriteQosOverrideRegisterS1R [
        AWQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub WriteQosOverrideRegisterS1W [
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub QosControlRegisterS1R [
        QOS_REGULATION_DISABLED OFFSET(31) NUMBITS(1) [],
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        AR_OT_REGULATION OFFSET(3) NUMBITS(1) [],
        AW_OT_REGULATION OFFSET(2) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ],
    pub QosControlRegisterS1W [
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        AR_OT_REGULATION OFFSET(3) NUMBITS(1) [],
        AW_OT_REGULATION OFFSET(2) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub MaxOtRegisterS1 [
        INT_OT_AR OFFSET(24) NUMBITS(6) [],
        FRAC_OT_AR OFFSET(16) NUMBITS(8) [],
        INT_OT_AW OFFSET(8) NUMBITS(6) [],
        FRAC_OT_AW OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TargetLatencyRegisterS1 [
        AR_LAT OFFSET(16) NUMBITS(12) [],
        AW_LAT OFFSET(0) NUMBITS(12) [],
    ]
];
register_bitfields! [
    u32,
    pub LatencyRegulationRegisterS1 [
        AR_SCALE_FACT OFFSET(8) NUMBITS(3) [],
        AW_SCALE_FACT OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub QosRangeRegisterS1 [
        MAX_ARQOS OFFSET(24) NUMBITS(4) [],
        MIN_ARQOS OFFSET(16) NUMBITS(4) [],
        MAX_AWQOS OFFSET(8) NUMBITS(4) [],
        MIN_AWQOS OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub SnoopControlRegisterS2 [
        SUPPORT_DVMS OFFSET(31) NUMBITS(1) [],
        SUPPORT_SNOOPS OFFSET(30) NUMBITS(1) [],
        ENABLE_DVMS OFFSET(1) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ShareableOverrideRegisterS2 [
        AXDOMAIN_OVERRIDE OFFSET(0) NUMBITS(2) [],
    ]
];
register_bitfields! [
    u32,
    pub ReadQosOverrideRegisterS2R [
        ARQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub ReadQosOverrideRegisterS2W [
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub WriteQosOverrideRegisterS2R [
        AWQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub WriteQosOverrideRegisterS2W [
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub QosControlRegisterS2R [
        QOS_REGULATION_DISABLED OFFSET(31) NUMBITS(1) [],
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        AR_OT_REGULATION OFFSET(3) NUMBITS(1) [],
        AW_OT_REGULATION OFFSET(2) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ],
    pub QosControlRegisterS2W [
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        AR_OT_REGULATION OFFSET(3) NUMBITS(1) [],
        AW_OT_REGULATION OFFSET(2) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub MaxOtRegisterS2 [
        INT_OT_AR OFFSET(24) NUMBITS(6) [],
        FRAC_OT_AR OFFSET(16) NUMBITS(8) [],
        INT_OT_AW OFFSET(8) NUMBITS(6) [],
        FRAC_OT_AW OFFSET(0) NUMBITS(8) [],
    ]
];
register_bitfields! [
    u32,
    pub TargetLatencyRegisterS2 [
        AR_LAT OFFSET(16) NUMBITS(12) [],
        AW_LAT OFFSET(0) NUMBITS(12) [],
    ]
];
register_bitfields! [
    u32,
    pub LatencyRegulationRegisterS2 [
        AR_SCALE_FACT OFFSET(8) NUMBITS(3) [],
        AW_SCALE_FACT OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub QosRangeRegisterS2 [
        MAX_ARQOS OFFSET(24) NUMBITS(4) [],
        MIN_ARQOS OFFSET(16) NUMBITS(4) [],
        MAX_AWQOS OFFSET(8) NUMBITS(4) [],
        MIN_AWQOS OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub SnoopControlRegisterS3R [
        SUPPORT_DVMS OFFSET(31) NUMBITS(1) [],
        SUPPORT_SNOOPS OFFSET(30) NUMBITS(1) [],
        ENABLE_DVMS OFFSET(1) NUMBITS(1) [],
        ENABLE_SNOOPS OFFSET(0) NUMBITS(1) [],
    ],
    pub SnoopControlRegisterS3W [
        ENABLE_DVMS OFFSET(1) NUMBITS(1) [],
        ENABLE_SNOOPS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ReadQosOverrideRegisterS3R [
        ARQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub ReadQosOverrideRegisterS3W [
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub WriteQosOverrideRegisterS3R [
        AWQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub WriteQosOverrideRegisterS3W [
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub QosControlRegisterS3R [
        QOS_REGULATION_DISABLED OFFSET(31) NUMBITS(1) [],
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ],
    pub QosControlRegisterS3W [
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TargetLatencyRegisterS3 [
        AR_LAT OFFSET(16) NUMBITS(12) [],
        AW_LAT OFFSET(0) NUMBITS(12) [],
    ]
];
register_bitfields! [
    u32,
    pub LatencyRegulationRegisterS3 [
        AR_SCALE_FACT OFFSET(8) NUMBITS(3) [],
        AW_SCALE_FACT OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub QosRangeRegisterS3 [
        MAX_ARQOS OFFSET(24) NUMBITS(4) [],
        MIN_ARQOS OFFSET(16) NUMBITS(4) [],
        MAX_AWQOS OFFSET(8) NUMBITS(4) [],
        MIN_AWQOS OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub SnoopControlRegisterS4R [
        SUPPORT_DVMS OFFSET(31) NUMBITS(1) [],
        SUPPORT_SNOOPS OFFSET(30) NUMBITS(1) [],
        ENABLE_DVMS OFFSET(1) NUMBITS(1) [],
        ENABLE_SNOOPS OFFSET(0) NUMBITS(1) [],
    ],
    pub SnoopControlRegisterS4W [
        ENABLE_DVMS OFFSET(1) NUMBITS(1) [],
        ENABLE_SNOOPS OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub ReadQosOverrideRegisterS4R [
        ARQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub ReadQosOverrideRegisterS4W [
        ARQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub WriteQosOverrideRegisterS4R [
        AWQOS_OVERRIDE_READBACK OFFSET(8) NUMBITS(4) [],
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ],
    pub WriteQosOverrideRegisterS4W [
        AWQOS_VALUE OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub QosControlRegisterS4R [
        QOS_REGULATION_DISABLED OFFSET(31) NUMBITS(1) [],
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ],
    pub QosControlRegisterS4W [
        BANDWIDTH_REGULATION_MODE OFFSET(21) NUMBITS(1) [],
        ARQOS_REGULATION_MODE OFFSET(20) NUMBITS(1) [],
        AWQOS_REGULATION_MODE OFFSET(16) NUMBITS(1) [],
        ARQOS_REGULATION OFFSET(1) NUMBITS(1) [],
        AWQOS_REGULATION OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub TargetLatencyRegisterS4 [
        AR_LAT OFFSET(16) NUMBITS(12) [],
        AW_LAT OFFSET(0) NUMBITS(12) [],
    ]
];
register_bitfields! [
    u32,
    pub LatencyRegulationRegisterS4 [
        AR_SCALE_FACT OFFSET(8) NUMBITS(3) [],
        AW_SCALE_FACT OFFSET(0) NUMBITS(3) [],
    ]
];
register_bitfields! [
    u32,
    pub QosRangeRegisterS4 [
        MAX_ARQOS OFFSET(24) NUMBITS(4) [],
        MIN_ARQOS OFFSET(16) NUMBITS(4) [],
        MAX_AWQOS OFFSET(8) NUMBITS(4) [],
        MIN_AWQOS OFFSET(0) NUMBITS(4) [],
    ]
];
register_bitfields! [
    u32,
    pub CycleCounterControl [
        CCNT_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub CycleCountOverflow [
        CCNT_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Esr0 [
        EVT_IF0 OFFSET(5) NUMBITS(3) [],
        EVT_CNT0 OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u32,
    pub EventCounter0Control [
        CNT0_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EventCounter0Overflow [
        CNT0_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Esr1 [
        EVT_IF1 OFFSET(5) NUMBITS(3) [],
        EVT_CNT1 OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u32,
    pub EventCounter1Control [
        CNT1_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EventCounter1Overflow [
        CNT1_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Esr2 [
        EVT_IF2 OFFSET(5) NUMBITS(3) [],
        EVT_CNT2 OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u32,
    pub EventCounter2Control [
        CNT2_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EventCounter2Overflow [
        CNT2_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub Esr3 [
        EVT_IF3 OFFSET(5) NUMBITS(3) [],
        EVT_CNT3 OFFSET(0) NUMBITS(5) [],
    ]
];
register_bitfields! [
    u32,
    pub EventCounter3Control [
        CNT3_EN OFFSET(0) NUMBITS(1) [],
    ]
];
register_bitfields! [
    u32,
    pub EventCounter3Overflow [
        CNT3_OVERFLOW OFFSET(0) NUMBITS(1) [],
    ]
];

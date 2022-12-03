// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// PCIe Controller Attributes, PCIe Controller Attributes
pub static mut PCIE_ATTRIB: *mut Registers = 0xfd480000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// ATTR_0
    pub attr_0: ReadWrite<u32, Attr0::Register>,
    /// ATTR_1
    pub attr_1: ReadWrite<u32, Attr1::Register>,
    /// ATTR_2
    pub attr_2: ReadWrite<u32, Attr2::Register>,
    /// ATTR_3
    pub attr_3: ReadWrite<u32, Attr3::Register>,
    /// ATTR_4
    pub attr_4: ReadWrite<u32, Attr4::Register>,
    /// ATTR_5
    pub attr_5: ReadWrite<u32, Attr5::Register>,
    /// ATTR_6
    pub attr_6: ReadWrite<u32, Attr6::Register>,
    /// ATTR_7
    pub attr_7: ReadWrite<u32, Attr7::Register>,
    /// ATTR_8
    pub attr_8: ReadWrite<u32, Attr8::Register>,
    /// ATTR_9
    pub attr_9: ReadWrite<u32, Attr9::Register>,
    /// ATTR_10
    pub attr_10: ReadWrite<u32, Attr10::Register>,
    /// ATTR_11
    pub attr_11: ReadWrite<u32, Attr11::Register>,
    /// ATTR_12
    pub attr_12: ReadWrite<u32, Attr12::Register>,
    /// ATTR_13
    pub attr_13: ReadWrite<u32, Attr13::Register>,
    /// ATTR_14
    pub attr_14: ReadWrite<u32, Attr14::Register>,
    /// ATTR_15
    pub attr_15: ReadWrite<u32, Attr15::Register>,
    /// ATTR_16
    pub attr_16: ReadWrite<u32, Attr16::Register>,
    /// ATTR_17
    pub attr_17: ReadWrite<u32, Attr17::Register>,
    /// ATTR_18
    pub attr_18: ReadWrite<u32, Attr18::Register>,
    /// ATTR_19
    pub attr_19: ReadWrite<u32, Attr19::Register>,
    /// ATTR_20
    pub attr_20: ReadWrite<u32, Attr20::Register>,
    /// ATTR_21
    pub attr_21: ReadWrite<u32, Attr21::Register>,
    /// ATTR_22
    pub attr_22: ReadWrite<u32, Attr22::Register>,
    /// ATTR_23
    pub attr_23: ReadWrite<u32, Attr23::Register>,
    /// ATTR_24
    pub attr_24: ReadWrite<u32, Attr24::Register>,
    /// ATTR_25
    pub attr_25: ReadWrite<u32, Attr25::Register>,
    /// ATTR_26
    pub attr_26: ReadWrite<u32, Attr26::Register>,
    /// ATTR_27
    pub attr_27: ReadWrite<u32, Attr27::Register>,
    /// ATTR_28
    pub attr_28: ReadWrite<u32, Attr28::Register>,
    /// ATTR_29
    pub attr_29: ReadWrite<u32, Attr29::Register>,
    /// ATTR_30
    pub attr_30: ReadWrite<u32, Attr30::Register>,
    /// ATTR_31
    pub attr_31: ReadWrite<u32, Attr31::Register>,
    /// ATTR_32
    pub attr_32: ReadWrite<u32, Attr32::Register>,
    /// ATTR_33
    pub attr_33: ReadWrite<u32, Attr33::Register>,
    /// ATTR_34
    pub attr_34: ReadWrite<u32, Attr34::Register>,
    /// ATTR_35
    pub attr_35: ReadWrite<u32, Attr35::Register>,
    /// ATTR_36
    pub attr_36: ReadWrite<u32, Attr36::Register>,
    /// ATTR_37
    pub attr_37: ReadWrite<u32, Attr37::Register>,
    /// ATTR_38
    pub attr_38: ReadWrite<u32, Attr38::Register>,
    /// ATTR_39
    pub attr_39: ReadWrite<u32, Attr39::Register>,
    /// ATTR_40
    pub attr_40: ReadWrite<u32, Attr40::Register>,
    /// ATTR_41
    pub attr_41: ReadWrite<u32, Attr41::Register>,
    /// ATTR_42
    pub attr_42: ReadWrite<u32, Attr42::Register>,
    /// ATTR_43
    pub attr_43: ReadWrite<u32, Attr43::Register>,
    /// ATTR_44
    pub attr_44: ReadWrite<u32, Attr44::Register>,
    /// ATTR_45
    pub attr_45: ReadWrite<u32, Attr45::Register>,
    /// ATTR_46
    pub attr_46: ReadWrite<u32, Attr46::Register>,
    /// ATTR_47
    pub attr_47: ReadWrite<u32, Attr47::Register>,
    /// ATTR_48
    pub attr_48: ReadWrite<u32, Attr48::Register>,
    /// ATTR_49
    pub attr_49: ReadWrite<u32, Attr49::Register>,
    /// ATTR_50
    pub attr_50: ReadWrite<u32, Attr50::Register>,
    /// ATTR_51
    pub attr_51: ReadWrite<u32, Attr51::Register>,
    /// ATTR_52
    pub attr_52: ReadWrite<u32, Attr52::Register>,
    /// ATTR_53
    pub attr_53: ReadWrite<u32, Attr53::Register>,
    /// ATTR_54
    pub attr_54: ReadWrite<u32, Attr54::Register>,
    /// ATTR_55
    pub attr_55: ReadWrite<u32, Attr55::Register>,
    /// ATTR_56
    pub attr_56: ReadWrite<u32, Attr56::Register>,
    /// ATTR_57
    pub attr_57: ReadWrite<u32, Attr57::Register>,
    /// ATTR_58
    pub attr_58: ReadWrite<u32, Attr58::Register>,
    /// ATTR_59
    pub attr_59: ReadWrite<u32, Attr59::Register>,
    /// ATTR_60
    pub attr_60: ReadWrite<u32, Attr60::Register>,
    /// ATTR_61
    pub attr_61: ReadWrite<u32, Attr61::Register>,
    /// ATTR_62
    pub attr_62: ReadWrite<u32, Attr62::Register>,
    /// ATTR_63
    pub attr_63: ReadWrite<u32, Attr63::Register>,
    /// ATTR_64
    pub attr_64: ReadWrite<u32, Attr64::Register>,
    /// ATTR_65
    pub attr_65: ReadWrite<u32, Attr65::Register>,
    /// ATTR_66
    pub attr_66: ReadWrite<u32, Attr66::Register>,
    /// ATTR_67
    pub attr_67: ReadWrite<u32, Attr67::Register>,
    /// ATTR_68
    pub attr_68: ReadWrite<u32, Attr68::Register>,
    /// ATTR_69
    pub attr_69: ReadWrite<u32, Attr69::Register>,
    /// ATTR_70
    pub attr_70: ReadWrite<u32, Attr70::Register>,
    /// ATTR_71
    pub attr_71: ReadWrite<u32, Attr71::Register>,
    /// ATTR_72
    pub attr_72: ReadWrite<u32, Attr72::Register>,
    /// ATTR_73
    pub attr_73: ReadWrite<u32, Attr73::Register>,
    /// ATTR_74
    pub attr_74: ReadWrite<u32, Attr74::Register>,
    /// ATTR_75
    pub attr_75: ReadWrite<u32, Attr75::Register>,
    /// ATTR_76
    pub attr_76: ReadWrite<u32, Attr76::Register>,
    /// ATTR_77
    pub attr_77: ReadWrite<u32, Attr77::Register>,
    /// ATTR_78
    pub attr_78: ReadWrite<u32, Attr78::Register>,
    /// ATTR_79
    pub attr_79: ReadWrite<u32, Attr79::Register>,
    /// ATTR_80
    pub attr_80: ReadWrite<u32, Attr80::Register>,
    /// ATTR_81
    pub attr_81: ReadWrite<u32, Attr81::Register>,
    /// ATTR_82
    pub attr_82: ReadWrite<u32, Attr82::Register>,
    /// ATTR_83
    pub attr_83: ReadWrite<u32, Attr83::Register>,
    /// ATTR_84
    pub attr_84: ReadWrite<u32, Attr84::Register>,
    /// ATTR_85
    pub attr_85: ReadWrite<u32, Attr85::Register>,
    /// ATTR_86
    pub attr_86: ReadWrite<u32, Attr86::Register>,
    /// ATTR_87
    pub attr_87: ReadWrite<u32, Attr87::Register>,
    /// ATTR_88
    pub attr_88: ReadWrite<u32, Attr88::Register>,
    /// ATTR_89
    pub attr_89: ReadWrite<u32, Attr89::Register>,
    /// ATTR_90
    pub attr_90: ReadWrite<u32, Attr90::Register>,
    /// ATTR_91
    pub attr_91: ReadWrite<u32, Attr91::Register>,
    /// ATTR_92
    pub attr_92: ReadWrite<u32, Attr92::Register>,
    /// ATTR_93
    pub attr_93: ReadWrite<u32, Attr93::Register>,
    /// ATTR_94
    pub attr_94: ReadWrite<u32, Attr94::Register>,
    /// ATTR_95
    pub attr_95: ReadWrite<u32, Attr95::Register>,
    /// ATTR_96
    pub attr_96: ReadWrite<u32, Attr96::Register>,
    /// ATTR_97
    pub attr_97: ReadWrite<u32, Attr97::Register>,
    /// ATTR_98
    pub attr_98: ReadWrite<u32, Attr98::Register>,
    /// ATTR_99
    pub attr_99: ReadWrite<u32, Attr99::Register>,
    /// ATTR_100
    pub attr_100: ReadWrite<u32, Attr100::Register>,
    /// ATTR_101
    pub attr_101: ReadWrite<u32, Attr101::Register>,
    /// ATTR_102
    pub attr_102: ReadWrite<u32, Attr102::Register>,
    /// ATTR_103
    pub attr_103: ReadWrite<u32, Attr103::Register>,
    /// ATTR_104
    pub attr_104: ReadWrite<u32, Attr104::Register>,
    /// ATTR_105
    pub attr_105: ReadWrite<u32, Attr105::Register>,
    /// ATTR_106
    pub attr_106: ReadWrite<u32, Attr106::Register>,
    /// ATTR_107
    pub attr_107: ReadWrite<u32, Attr107::Register>,
    /// ATTR_108
    pub attr_108: ReadWrite<u32, Attr108::Register>,
    /// ATTR_109
    pub attr_109: ReadWrite<u32, Attr109::Register>,
    /// ATTR_110
    pub attr_110: ReadWrite<u32, Attr110::Register>,
    _padding444: [u8; 68],
    /// Device and Vendor ID register
    pub id: ReadWrite<u32, Id::Register>,
    /// Subsystem ID and Vendor ID register
    pub subsys_id: ReadWrite<u32, SubsysId::Register>,
    /// Revision ID register
    pub rev_id: ReadWrite<u32, RevId::Register>,
    /// DSN Register 0
    pub dsn_0: ReadWrite<u32>,
    /// DSN Register 1
    pub dsn_1: ReadWrite<u32>,
    _padding532: [u8; 4],
    /// CFG PM Control Register
    pub pm_ctrl: Aliased<u32, PmCtrlR::Register, PmCtrlW::Register>,
    _padding540: [u8; 20],
    /// PL End Point Mode Control Register
    pub ep_ctrl: Aliased<u32, EpCtrlR::Register, EpCtrlW::Register>,
    /// PL Root Port Mode Control Register
    pub rp_ctrl: ReadWrite<u32, RpCtrl::Register>,
    /// PL Root Port Mode Control Register
    pub pcie_status: ReadOnly<u32, PcieStatus::Register>,
    _padding572: [u8; 196],
    /// MISC_CTRL
    pub misc_ctrl: ReadWrite<u8, MiscCtrl::Register>,
    _padding769: [u8; 3],
    /// Interrupt Status Register. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub isr: ReadWrite<u32, Isr::Register>,
    /// Interrupt Mask Register. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub imr: ReadOnly<u32, Imr::Register>,
    /// Interrupt Enable Register. A write of to this location will unmask the interrupt. (IMR: 0)
    pub ier: WriteOnly<u32, Ier::Register>,
    /// Interrupt Disable Register. A write of one to this location will mask the interrupt. (IMR: 1)
    pub idr: WriteOnly<u32, Idr::Register>,
    _padding788: [u8; 8],
    /// DT Enables
    pub cb: ReadWrite<u32, Cb::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Attr0 [
        ATTR_AER_CAP_ECRC_GEN_CAPABLE OFFSET(1) NUMBITS(1) [],
        ATTR_AER_CAP_ECRC_CHECK_CAPABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr1 [
        ATTR_AER_CAP_ID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr2 [
        ATTR_AER_CAP_VERSION OFFSET(1) NUMBITS(4) [],
        ATTR_AER_CAP_PERMIT_ROOTERR_UPDATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr3 [
        ATTR_AER_BASE_PTR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr4 [
        ATTR_AER_CAP_ON OFFSET(12) NUMBITS(1) [],
        ATTR_AER_CAP_NEXTPTR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr5 [
        ATTR_AER_CAP_OPTIONAL_ERR_SUPPORT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr6 [
        ATTR_AER_CAP_MULTIHEADER OFFSET(8) NUMBITS(1) [],
        ATTR_AER_CAP_OPTIONAL_ERR_SUPPORT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr7 [
        ATTR_BAR0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr8 [
        ATTR_BAR0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr9 [
        ATTR_BAR1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr10 [
        ATTR_BAR1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr11 [
        ATTR_BAR2 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr12 [
        ATTR_BAR2 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr13 [
        ATTR_BAR3 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr14 [
        ATTR_BAR3 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr15 [
        ATTR_BAR4 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr16 [
        ATTR_BAR4 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr17 [
        ATTR_BAR5 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr18 [
        ATTR_BAR5 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr19 [
        ATTR_EXPANSION_ROM OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr20 [
        ATTR_EXPANSION_ROM OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr21 [
        ATTR_CAPABILITIES_PTR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr22 [
        ATTR_CARDBUS_CIS_POINTER OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr23 [
        ATTR_CARDBUS_CIS_POINTER OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr24 [
        ATTR_CLASS_CODE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr25 [
        ATTR_DEV_CAP2_ATOMICOP_ROUTING_SUPPORTED OFFSET(15) NUMBITS(1) [],
        ATTR_DEV_CAP2_ARI_FORWARDING_SUPPORTED OFFSET(14) NUMBITS(1) [],
        ATTR_CPL_TIMEOUT_RANGES_SUPPORTED OFFSET(10) NUMBITS(4) [],
        ATTR_CPL_TIMEOUT_DISABLE_SUPPORTED OFFSET(9) NUMBITS(1) [],
        ATTR_CMD_INTX_IMPLEMENTED OFFSET(8) NUMBITS(1) [],
        ATTR_CLASS_CODE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr26 [
        ATTR_DEV_CAP_ENABLE_SLOT_PWR_LIMIT_VALUE OFFSET(13) NUMBITS(1) [],
        ATTR_DEV_CAP_ENABLE_SLOT_PWR_LIMIT_SCALE OFFSET(12) NUMBITS(1) [],
        RESERVED0 OFFSET(11) NUMBITS(1) [],
        RESERVED1 OFFSET(9) NUMBITS(2) [],
        RESERVED2 OFFSET(8) NUMBITS(1) [],
        RESERVED3 OFFSET(7) NUMBITS(1) [],
        RESERVED4 OFFSET(5) NUMBITS(2) [],
        RESERVED5 OFFSET(4) NUMBITS(1) [],
        RESERVED6 OFFSET(3) NUMBITS(1) [],
        RESERVED7 OFFSET(2) NUMBITS(1) [],
        RESERVED8 OFFSET(1) NUMBITS(1) [],
        RESERVED9 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr27 [
        ATTR_DEV_CAP_ROLE_BASED_ERROR OFFSET(13) NUMBITS(1) [],
        ATTR_DEV_CAP_PHANTOM_FUNCTIONS_SUPPORT OFFSET(11) NUMBITS(2) [],
        ATTR_DEV_CAP_MAX_PAYLOAD_SUPPORTED OFFSET(8) NUMBITS(3) [],
        ATTR_DEV_CAP_FUNCTION_LEVEL_RESET_CAPABLE OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        ATTR_DEV_CAP_ENDPOINT_L1_LATENCY OFFSET(3) NUMBITS(3) [],
        ATTR_DEV_CAP_ENDPOINT_L0S_LATENCY OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr28 [
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        ATTR_DEV_CONTROL_AUX_POWER_SUPPORTED OFFSET(8) NUMBITS(1) [],
        ATTR_DEV_CAP_RSVD_31_29 OFFSET(5) NUMBITS(3) [],
        ATTR_DEV_CAP_RSVD_17_16 OFFSET(3) NUMBITS(2) [],
        ATTR_DEV_CAP_RSVD_14_12 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr29 [
        ATTR_DSN_BASE_PTR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr30 [
        ATTR_DSN_CAP_ID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr31 [
        ATTR_DSN_CAP_ON OFFSET(12) NUMBITS(1) [],
        ATTR_DSN_CAP_NEXTPTR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr32 [
        RESERVED0 OFFSET(4) NUMBITS(6) [],
        ATTR_DSN_CAP_VERSION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr33 [
        RESERVED0 OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr34 [
        ATTR_INTERRUPT_PIN OFFSET(8) NUMBITS(8) [],
        ATTR_HEADER_TYPE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr35 [
        ATTR_LINK_CAP_DLL_LINK_ACTIVE_REPORTING_CAP OFFSET(15) NUMBITS(1) [],
        ATTR_LINK_CAP_CLOCK_POWER_MANAGEMENT OFFSET(14) NUMBITS(1) [],
        ATTR_LINK_CAP_ASPM_SUPPORT OFFSET(12) NUMBITS(2) [],
        RESERVED0 OFFSET(2) NUMBITS(10) [],
        RESERVED1 OFFSET(1) NUMBITS(1) [],
        ATTR_INTERRUPT_STAT_AUTO OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr36 [
        ATTR_LINK_CAP_L1_EXIT_LATENCY_COMCLK_GEN1 OFFSET(12) NUMBITS(3) [],
        ATTR_LINK_CAP_L0S_EXIT_LATENCY_GEN2 OFFSET(9) NUMBITS(3) [],
        ATTR_LINK_CAP_L0S_EXIT_LATENCY_GEN1 OFFSET(6) NUMBITS(3) [],
        ATTR_LINK_CAP_L0S_EXIT_LATENCY_COMCLK_GEN2 OFFSET(3) NUMBITS(3) [],
        ATTR_LINK_CAP_L0S_EXIT_LATENCY_COMCLK_GEN1 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr37 [
        ATTR_LINK_CAP_RSVD_23 OFFSET(15) NUMBITS(1) [],
        ATTR_LINK_CAP_ASPM_OPTIONALITY OFFSET(14) NUMBITS(1) [],
        ATTR_LINK_CAP_MAX_LINK_SPEED OFFSET(10) NUMBITS(4) [],
        ATTR_LINK_CAP_LINK_BANDWIDTH_NOTIFICATION_CAP OFFSET(9) NUMBITS(1) [],
        ATTR_LINK_CAP_L1_EXIT_LATENCY_GEN2 OFFSET(6) NUMBITS(3) [],
        ATTR_LINK_CAP_L1_EXIT_LATENCY_GEN1 OFFSET(3) NUMBITS(3) [],
        ATTR_LINK_CAP_L1_EXIT_LATENCY_COMCLK_GEN2 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr38 [
        ATTR_MPS_FORCE OFFSET(9) NUMBITS(1) [],
        ATTR_LINK_STATUS_SLOT_CLOCK_CONFIG OFFSET(8) NUMBITS(1) [],
        ATTR_LINK_CTRL2_TARGET_LINK_SPEED OFFSET(4) NUMBITS(4) [],
        ATTR_LINK_CTRL2_HW_AUTONOMOUS_SPEED_DISABLE OFFSET(3) NUMBITS(1) [],
        ATTR_LINK_CTRL2_DEEMPHASIS OFFSET(2) NUMBITS(1) [],
        ATTR_LINK_CONTROL_RCB OFFSET(1) NUMBITS(1) [],
        ATTR_LINK_CAP_SURPRISE_DOWN_ERROR_CAPABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr39 [
        ATTR_MSI_CAP_64_BIT_ADDR_CAPABLE OFFSET(8) NUMBITS(1) [],
        ATTR_MSI_BASE_PTR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr40 [
        ATTR_MSI_CAP_MULTIMSGCAP OFFSET(9) NUMBITS(3) [],
        ATTR_MSI_CAP_MULTIMSG_EXTENSION OFFSET(8) NUMBITS(1) [],
        ATTR_MSI_CAP_ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr41 [
        ATTR_MSI_CAP_PER_VECTOR_MASKING_CAPABLE OFFSET(9) NUMBITS(1) [],
        ATTR_MSI_CAP_ON OFFSET(8) NUMBITS(1) [],
        ATTR_MSI_CAP_NEXTPTR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr42 [
        ATTR_MSIX_CAP_ID OFFSET(8) NUMBITS(8) [],
        ATTR_MSIX_BASE_PTR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr43 [
        ATTR_MSIX_CAP_PBA_BIR OFFSET(9) NUMBITS(3) [],
        ATTR_MSIX_CAP_ON OFFSET(8) NUMBITS(1) [],
        ATTR_MSIX_CAP_NEXTPTR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr44 [
        ATTR_MSIX_CAP_PBA_OFFSET OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr45 [
        ATTR_MSIX_CAP_PBA_OFFSET OFFSET(3) NUMBITS(13) [],
        ATTR_MSIX_CAP_TABLE_BIR OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr46 [
        ATTR_MSIX_CAP_TABLE_OFFSET OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr47 [
        ATTR_MSIX_CAP_TABLE_OFFSET OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr48 [
        ATTR_MSIX_CAP_TABLE_SIZE OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr49 [
        ATTR_PCIE_CAP_CAPABILITY_ID OFFSET(8) NUMBITS(8) [],
        ATTR_PCIE_BASE_PTR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr50 [
        ATTR_PCIE_CAP_NEXTPTR OFFSET(8) NUMBITS(8) [],
        ATTR_PCIE_CAP_DEVICE_PORT_TYPE OFFSET(4) NUMBITS(4) [],
        ATTR_PCIE_CAP_CAPABILITY_VERSION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr51 [
        ATTR_PM_BASE_PTR OFFSET(8) NUMBITS(8) [],
        ATTR_PCIE_REVISION OFFSET(4) NUMBITS(4) [],
        ATTR_PCIE_CAP_SLOT_IMPLEMENTED OFFSET(3) NUMBITS(1) [],
        ATTR_PCIE_CAP_RSVD_15_14 OFFSET(1) NUMBITS(2) [],
        ATTR_PCIE_CAP_ON OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr52 [
        ATTR_PM_CAP_ID OFFSET(6) NUMBITS(8) [],
        ATTR_PM_CAP_DSI OFFSET(5) NUMBITS(1) [],
        ATTR_PM_CAP_D2SUPPORT OFFSET(4) NUMBITS(1) [],
        ATTR_PM_CAP_D1SUPPORT OFFSET(3) NUMBITS(1) [],
        ATTR_PM_CAP_AUXCURRENT OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr53 [
        ATTR_PM_CAP_RSVD_04 OFFSET(15) NUMBITS(1) [],
        ATTR_PM_CAP_PMESUPPORT OFFSET(10) NUMBITS(5) [],
        ATTR_PM_CAP_PME_CLOCK OFFSET(9) NUMBITS(1) [],
        ATTR_PM_CAP_ON OFFSET(8) NUMBITS(1) [],
        ATTR_PM_CAP_NEXTPTR OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr54 [
        ATTR_PM_DATA_SCALE4 OFFSET(14) NUMBITS(2) [],
        ATTR_PM_DATA_SCALE3 OFFSET(12) NUMBITS(2) [],
        ATTR_PM_DATA_SCALE2 OFFSET(10) NUMBITS(2) [],
        ATTR_PM_DATA_SCALE1 OFFSET(8) NUMBITS(2) [],
        ATTR_PM_DATA_SCALE0 OFFSET(6) NUMBITS(2) [],
        ATTR_PM_CSR_NOSOFTRST OFFSET(5) NUMBITS(1) [],
        ATTR_PM_CSR_BPCCEN OFFSET(4) NUMBITS(1) [],
        ATTR_PM_CSR_B2B3 OFFSET(3) NUMBITS(1) [],
        ATTR_PM_CAP_VERSION OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr55 [
        ATTR_PM_DATA0 OFFSET(6) NUMBITS(8) [],
        ATTR_PM_DATA_SCALE7 OFFSET(4) NUMBITS(2) [],
        ATTR_PM_DATA_SCALE6 OFFSET(2) NUMBITS(2) [],
        ATTR_PM_DATA_SCALE5 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr56 [
        ATTR_PM_DATA2 OFFSET(8) NUMBITS(8) [],
        ATTR_PM_DATA1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr57 [
        ATTR_PM_DATA4 OFFSET(8) NUMBITS(8) [],
        ATTR_PM_DATA3 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr58 [
        ATTR_PM_DATA6 OFFSET(8) NUMBITS(8) [],
        ATTR_PM_DATA5 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr59 [
        ATTR_PM_DATA7 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr60 [
        ATTR_RBAR_BASE_PTR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr61 [
        ATTR_RBAR_CAP_ON OFFSET(12) NUMBITS(1) [],
        ATTR_RBAR_CAP_NEXTPTR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr62 [
        ATTR_RBAR_CAP_ID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr63 [
        ATTR_RBAR_NUM OFFSET(4) NUMBITS(3) [],
        ATTR_RBAR_CAP_VERSION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr64 [
        ATTR_RBAR_CAP_SUP0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr65 [
        ATTR_RBAR_CAP_SUP0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr66 [
        ATTR_RBAR_CAP_SUP1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr67 [
        ATTR_RBAR_CAP_SUP1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr68 [
        ATTR_RBAR_CAP_SUP2 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr69 [
        ATTR_RBAR_CAP_SUP2 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr70 [
        ATTR_RBAR_CAP_SUP3 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr71 [
        ATTR_RBAR_CAP_SUP3 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr72 [
        ATTR_RBAR_CAP_SUP4 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr73 [
        ATTR_RBAR_CAP_SUP4 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr74 [
        ATTR_RBAR_CAP_SUP5 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr75 [
        ATTR_RBAR_CAP_SUP5 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr76 [
        ATTR_RBAR_CAP_INDEX4 OFFSET(12) NUMBITS(3) [],
        ATTR_RBAR_CAP_INDEX3 OFFSET(9) NUMBITS(3) [],
        ATTR_RBAR_CAP_INDEX2 OFFSET(6) NUMBITS(3) [],
        ATTR_RBAR_CAP_INDEX1 OFFSET(3) NUMBITS(3) [],
        ATTR_RBAR_CAP_INDEX0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr77 [
        ATTR_RBAR_CAP_CONTROL_ENCODEDBAR1 OFFSET(8) NUMBITS(5) [],
        ATTR_RBAR_CAP_CONTROL_ENCODEDBAR0 OFFSET(3) NUMBITS(5) [],
        ATTR_RBAR_CAP_INDEX5 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr78 [
        ATTR_RBAR_CAP_CONTROL_ENCODEDBAR4 OFFSET(10) NUMBITS(5) [],
        ATTR_RBAR_CAP_CONTROL_ENCODEDBAR3 OFFSET(5) NUMBITS(5) [],
        ATTR_RBAR_CAP_CONTROL_ENCODEDBAR2 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr79 [
        ATTR_SLOT_CAP_NO_CMD_COMPLETED_SUPPORT OFFSET(13) NUMBITS(1) [],
        ATTR_SLOT_CAP_MRL_SENSOR_PRESENT OFFSET(12) NUMBITS(1) [],
        ATTR_SLOT_CAP_HOTPLUG_SURPRISE OFFSET(11) NUMBITS(1) [],
        ATTR_SLOT_CAP_HOTPLUG_CAPABLE OFFSET(10) NUMBITS(1) [],
        ATTR_SLOT_CAP_ELEC_INTERLOCK_PRESENT OFFSET(9) NUMBITS(1) [],
        ATTR_SLOT_CAP_ATT_INDICATOR_PRESENT OFFSET(8) NUMBITS(1) [],
        ATTR_SLOT_CAP_ATT_BUTTON_PRESENT OFFSET(7) NUMBITS(1) [],
        RESERVED0 OFFSET(6) NUMBITS(1) [],
        ATTR_ROOT_CAP_CRS_SW_VISIBILITY OFFSET(5) NUMBITS(1) [],
        ATTR_RBAR_CAP_CONTROL_ENCODEDBAR5 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr80 [
        ATTR_SLOT_CAP_POWER_INDICATOR_PRESENT OFFSET(14) NUMBITS(1) [],
        ATTR_SLOT_CAP_POWER_CONTROLLER_PRESENT OFFSET(13) NUMBITS(1) [],
        ATTR_SLOT_CAP_PHYSICAL_SLOT_NUM OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr81 [
        RESERVED0 OFFSET(10) NUMBITS(1) [],
        ATTR_SLOT_CAP_SLOT_POWER_LIMIT_VALUE OFFSET(2) NUMBITS(8) [],
        ATTR_SLOT_CAP_SLOT_POWER_LIMIT_SCALE OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr82 [
        ATTR_VC_BASE_PTR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr83 [
        ATTR_VC_CAP_ON OFFSET(12) NUMBITS(1) [],
        ATTR_VC_CAP_NEXTPTR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr84 [
        ATTR_VC_CAP_ID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr85 [
        ATTR_VSEC_BASE_PTR OFFSET(1) NUMBITS(12) [],
        ATTR_VC_CAP_REJECT_SNOOP_TRANSACTIONS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr86 [
        ATTR_VSEC_CAP_HDR_ID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr87 [
        ATTR_VSEC_CAP_HDR_REVISION OFFSET(12) NUMBITS(4) [],
        ATTR_VSEC_CAP_HDR_LENGTH OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr88 [
        ATTR_VSEC_CAP_ID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr89 [
        ATTR_VSEC_CAP_ON OFFSET(13) NUMBITS(1) [],
        ATTR_VSEC_CAP_NEXTPTR OFFSET(1) NUMBITS(12) [],
        ATTR_VSEC_CAP_IS_LINK_VISIBLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr90 [
        RESERVED0 OFFSET(7) NUMBITS(7) [],
        RESERVED1 OFFSET(4) NUMBITS(3) [],
        ATTR_VSEC_CAP_VERSION OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr91 [
        ATTR_LL_ACK_TIMEOUT_EN OFFSET(15) NUMBITS(1) [],
        ATTR_LL_ACK_TIMEOUT OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr92 [
        ATTR_LL_ACK_TIMEOUT_FUNC OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr93 [
        ATTR_LL_REPLAY_TIMEOUT_EN OFFSET(15) NUMBITS(1) [],
        ATTR_LL_REPLAY_TIMEOUT OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr94 [
        ATTR_LL_REPLAY_TIMEOUT_FUNC OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr95 [
        ATTR_PM_ASPML0S_TIMEOUT_EN OFFSET(15) NUMBITS(1) [],
        ATTR_PM_ASPML0S_TIMEOUT OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr96 [
        ATTR_INFER_EI OFFSET(6) NUMBITS(5) [],
        ATTR_ENTER_RVRY_EI_L0 OFFSET(5) NUMBITS(1) [],
        ATTR_DISABLE_SCRAMBLING OFFSET(4) NUMBITS(1) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        ATTR_PM_ASPM_FASTEXIT OFFSET(2) NUMBITS(1) [],
        ATTR_PM_ASPML0S_TIMEOUT_FUNC OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr97 [
        ATTR_LTSSM_MAX_LINK_WIDTH OFFSET(6) NUMBITS(6) [],
        ATTR_LINK_CAP_MAX_LINK_WIDTH OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr98 [
        ATTR_N_FTS_COMCLK_GEN2 OFFSET(8) NUMBITS(8) [],
        ATTR_N_FTS_COMCLK_GEN1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr99 [
        ATTR_N_FTS_GEN2 OFFSET(8) NUMBITS(8) [],
        ATTR_N_FTS_GEN1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr100 [
        ATTR_DNSTREAM_LINK_NUM OFFSET(8) NUMBITS(8) [],
        ATTR_EXIT_LOOPBACK_ON_EI OFFSET(7) NUMBITS(1) [],
        ATTR_UPSTREAM_FACING OFFSET(6) NUMBITS(1) [],
        ATTR_UPCONFIG_CAPABLE OFFSET(5) NUMBITS(1) [],
        RESERVED0 OFFSET(4) NUMBITS(1) [],
        ATTR_PL_AUTO_CONFIG OFFSET(1) NUMBITS(3) [],
        RESERVED1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr101 [
        ATTR_ENABLE_MSG_ROUTE OFFSET(5) NUMBITS(11) [],
        ATTR_DISABLE_RX_POISONED_RESP OFFSET(4) NUMBITS(1) [],
        ATTR_DISABLE_RX_TC_FILTER OFFSET(3) NUMBITS(1) [],
        ATTR_DISABLE_ID_CHECK OFFSET(2) NUMBITS(1) [],
        ATTR_DISABLE_BAR_FILTERING OFFSET(1) NUMBITS(1) [],
        ATTR_DISABLE_ASPM_L1_TIMER OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr102 [
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        RESERVED1 OFFSET(13) NUMBITS(1) [],
        RESERVED2 OFFSET(12) NUMBITS(1) [],
        RESERVED3 OFFSET(11) NUMBITS(1) [],
        RESERVED4 OFFSET(10) NUMBITS(1) [],
        RESERVED5 OFFSET(9) NUMBITS(1) [],
        RESERVED6 OFFSET(8) NUMBITS(1) [],
        RESERVED7 OFFSET(7) NUMBITS(1) [],
        RESERVED8 OFFSET(6) NUMBITS(1) [],
        RESERVED9 OFFSET(5) NUMBITS(1) [],
        RESERVED10 OFFSET(4) NUMBITS(1) [],
        RESERVED11 OFFSET(2) NUMBITS(2) [],
        RESERVED12 OFFSET(1) NUMBITS(1) [],
        ATTR_ENABLE_RX_TD_ECRC_TRIM OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr103 [
        ATTR_VC0_CPL_INFINITE OFFSET(5) NUMBITS(1) [],
        ATTR_VC_CAP_VERSION OFFSET(1) NUMBITS(4) [],
        ATTR_TL_TX_RAM_WRITE_LATENCY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr104 [
        ATTR_VC0_RX_RAM_LIMIT OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr105 [
        ATTR_VC0_TOTAL_CREDITS_CD OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr106 [
        ATTR_VC0_TOTAL_CREDITS_NPH OFFSET(7) NUMBITS(7) [],
        ATTR_VC0_TOTAL_CREDITS_CH OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr107 [
        ATTR_VC0_TOTAL_CREDITS_NPD OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr108 [
        ATTR_VC0_TOTAL_CREDITS_PD OFFSET(0) NUMBITS(11) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr109 [
        ATTR_TECRC_EP_INV OFFSET(15) NUMBITS(1) [],
        ATTR_RECRC_CHK_TRIM OFFSET(14) NUMBITS(1) [],
        ATTR_RECRC_CHK OFFSET(12) NUMBITS(2) [],
        ATTR_VC0_TX_LASTPACKET OFFSET(7) NUMBITS(5) [],
        ATTR_VC0_TOTAL_CREDITS_PH OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Attr110 [
        ATTR_RP_AUTO_SPD_LOOPCNT OFFSET(11) NUMBITS(5) [],
        ATTR_RP_AUTO_SPD OFFSET(9) NUMBITS(2) [],
        RESERVED0 OFFSET(8) NUMBITS(1) [],
        ATTR_TRN_NP_FC OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(1) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(3) NUMBITS(1) [],
        ATTR_UR_INV_REQ OFFSET(2) NUMBITS(1) [],
        ATTR_CFG_ECRC_ERR_CPLSTAT OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Id [
        CFG_VEND_ID OFFSET(16) NUMBITS(16) [],
        CFG_DEV_ID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SubsysId [
        CFG_SUBSYS_VEND_ID OFFSET(16) NUMBITS(16) [],
        CFG_SUBSYS_ID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RevId [
        CFG_REV_ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PmCtrlR [
        CFG_TRN_PENDING OFFSET(3) NUMBITS(1) [],
        CFG_PM_SEND_PME_TO OFFSET(2) NUMBITS(1) [],
        CFG_PM_TURNOFF_OK OFFSET(1) NUMBITS(1) [],
        CFG_PM_WAKE OFFSET(0) NUMBITS(1) [],
    ],
    pub PmCtrlW [
        CFG_PM_SEND_PME_TO OFFSET(2) NUMBITS(1) [],
        CFG_PM_TURNOFF_OK OFFSET(1) NUMBITS(1) [],
        CFG_PM_WAKE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EpCtrlR [
        PL_UPSTREAM_DEEMPH_SOURCE OFFSET(1) NUMBITS(1) [],
        PL_RECEIVED_HOT_RST OFFSET(0) NUMBITS(1) [],
    ],
    pub EpCtrlW [
        PL_UPSTREAM_DEEMPH_SOURCE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub RpCtrl [
        PL_DOWNSTREAM_DEEMPH_SOURCE OFFSET(1) NUMBITS(1) [],
        PL_TRANSMIT_HOT_RST OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PcieStatus [
        PHY_RDY OFFSET(1) NUMBITS(1) [],
        PCIE_LINK_UP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub MiscCtrl [
        SLVERR_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Isr [
        PCIE_RESET OFFSET(1) NUMBITS(1) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imr [
        PCIE_RESET OFFSET(1) NUMBITS(1) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ier [
        PCIE_RESET OFFSET(1) NUMBITS(1) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Idr [
        PCIE_RESET OFFSET(1) NUMBITS(1) [],
        ADDR_DECODE_ERR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Cb [
        CB1 OFFSET(1) NUMBITS(1) [],
        RESERVED0 OFFSET(0) NUMBITS(1) [],
    ]
];

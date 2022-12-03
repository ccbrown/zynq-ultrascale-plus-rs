// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// IOP GPV, GPV
pub static mut IOU_GPV: *mut Registers = 0xfe000000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 8144],
    /// 4KB count, JEP106 continuation code
    pub periph_id_4: ReadOnly<u32, PeriphId4::Register>,
    /// Reserved
    pub periph_id_5: ReadOnly<u32, PeriphId5::Register>,
    /// Reserved
    pub periph_id_6: ReadOnly<u32, PeriphId6::Register>,
    /// Reserved
    pub periph_id_7: ReadOnly<u32, PeriphId7::Register>,
    /// Part Number [7:0]
    pub periph_id_0: ReadOnly<u32, PeriphId0::Register>,
    /// JEP106[3:0], part number [11:8]
    pub periph_id_1: ReadOnly<u32, PeriphId1::Register>,
    /// Revision, JEP106 code flag, JEP106[6:4]
    pub periph_id_2: ReadOnly<u32, PeriphId2::Register>,
    /// You can set this using the AMBA Designer Graphical User Interface (GUI)
    pub periph_id_3: ReadOnly<u32, PeriphId3::Register>,
    /// Preamble
    pub comp_id_0: ReadOnly<u32, CompId0::Register>,
    /// Generic IP component class, preamble
    pub comp_id_1: ReadOnly<u32, CompId1::Register>,
    /// Preamble
    pub comp_id_2: ReadOnly<u32, CompId2::Register>,
    /// Preamble
    pub comp_id_3: ReadOnly<u32, CompId3::Register>,
    _padding8192: [u8; 8],
    /// Bus matrix issuing functionality modification register
    pub intiou_intlpd_fn_mod_iss_bm: ReadWrite<u32, IntiouIntlpdFnModIssBm::Register>,
    _padding8204: [u8; 20476],
    /// Bus matrix issuing functionality modification register
    pub apb_ns_0_ib_fn_mod_iss_bm: ReadWrite<u32, ApbNs0IbFnModIssBm::Register>,
    _padding28684: [u8; 4092],
    /// Bus matrix issuing functionality modification register
    pub apb_ns_1_ib_fn_mod_iss_bm: ReadWrite<u32, ApbNs1IbFnModIssBm::Register>,
    _padding32780: [u8; 237820],
    /// Issuing functionality modification register
    pub intlpd_intiou_fn_mod: ReadWrite<u32, IntlpdIntiouFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub intlpd_intiou_qos_cntl: ReadWrite<u32, IntlpdIntiouQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub intlpd_intiou_max_ot: ReadWrite<u32, IntlpdIntiouMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub intlpd_intiou_max_comb_ot: ReadWrite<u32, IntlpdIntiouMaxCombOt::Register>,
    /// AW channel peak rate
    pub intlpd_intiou_aw_p: ReadWrite<u32, IntlpdIntiouAwP::Register>,
    /// AW channel burstiness allowance
    pub intlpd_intiou_aw_b: ReadWrite<u32, IntlpdIntiouAwB::Register>,
    /// AW channel average rate
    pub intlpd_intiou_aw_r: ReadWrite<u32, IntlpdIntiouAwR::Register>,
    /// AR channel peak rate
    pub intlpd_intiou_ar_p: ReadWrite<u32, IntlpdIntiouArP::Register>,
    /// AR channel burstiness allowance
    pub intlpd_intiou_ar_b: ReadWrite<u32, IntlpdIntiouArB::Register>,
    /// AR channel average rate
    pub intlpd_intiou_ar_r: ReadWrite<u32, IntlpdIntiouArR::Register>,
    _padding270640: [u8; 4048],
    /// Read channel QoS value
    pub gem0m_intiou_read_qos: ReadWrite<u32, Gem0mIntiouReadQos::Register>,
    /// Write channel QoS value
    pub gem0m_intiou_write_qos: ReadWrite<u32, Gem0mIntiouWriteQos::Register>,
    /// Issuing functionality modification register
    pub gem0m_intiou_fn_mod: ReadWrite<u32, Gem0mIntiouFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub gem0m_intiou_qos_cntl: ReadWrite<u32, Gem0mIntiouQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub gem0m_intiou_max_ot: ReadWrite<u32, Gem0mIntiouMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub gem0m_intiou_max_comb_ot: ReadWrite<u32, Gem0mIntiouMaxCombOt::Register>,
    /// AW channel peak rate
    pub gem0m_intiou_aw_p: ReadWrite<u32, Gem0mIntiouAwP::Register>,
    /// AW channel burstiness allowance
    pub gem0m_intiou_aw_b: ReadWrite<u32, Gem0mIntiouAwB::Register>,
    /// AW channel average rate
    pub gem0m_intiou_aw_r: ReadWrite<u32, Gem0mIntiouAwR::Register>,
    /// AR channel peak rate
    pub gem0m_intiou_ar_p: ReadWrite<u32, Gem0mIntiouArP::Register>,
    /// AR channel burstiness allowance
    pub gem0m_intiou_ar_b: ReadWrite<u32, Gem0mIntiouArB::Register>,
    /// AR channel average rate
    pub gem0m_intiou_ar_r: ReadWrite<u32, Gem0mIntiouArR::Register>,
    _padding274736: [u8; 4048],
    /// Read channel QoS value
    pub gem1m_intiou_read_qos: ReadWrite<u32, Gem1mIntiouReadQos::Register>,
    /// Write channel QoS value
    pub gem1m_intiou_write_qos: ReadWrite<u32, Gem1mIntiouWriteQos::Register>,
    /// Issuing functionality modification register
    pub gem1m_intiou_fn_mod: ReadWrite<u32, Gem1mIntiouFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub gem1m_intiou_qos_cntl: ReadWrite<u32, Gem1mIntiouQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub gem1m_intiou_max_ot: ReadWrite<u32, Gem1mIntiouMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub gem1m_intiou_max_comb_ot: ReadWrite<u32, Gem1mIntiouMaxCombOt::Register>,
    /// AW channel peak rate
    pub gem1m_intiou_aw_p: ReadWrite<u32, Gem1mIntiouAwP::Register>,
    /// AW channel burstiness allowance
    pub gem1m_intiou_aw_b: ReadWrite<u32, Gem1mIntiouAwB::Register>,
    /// AW channel average rate
    pub gem1m_intiou_aw_r: ReadWrite<u32, Gem1mIntiouAwR::Register>,
    /// AR channel peak rate
    pub gem1m_intiou_ar_p: ReadWrite<u32, Gem1mIntiouArP::Register>,
    /// AR channel burstiness allowance
    pub gem1m_intiou_ar_b: ReadWrite<u32, Gem1mIntiouArB::Register>,
    /// AR channel average rate
    pub gem1m_intiou_ar_r: ReadWrite<u32, Gem1mIntiouArR::Register>,
    _padding278832: [u8; 4048],
    /// Read channel QoS value
    pub gem2m_intiou_read_qos: ReadWrite<u32, Gem2mIntiouReadQos::Register>,
    /// Write channel QoS value
    pub gem2m_intiou_write_qos: ReadWrite<u32, Gem2mIntiouWriteQos::Register>,
    /// Issuing functionality modification register
    pub gem2m_intiou_fn_mod: ReadWrite<u32, Gem2mIntiouFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub gem2m_intiou_qos_cntl: ReadWrite<u32, Gem2mIntiouQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub gem2m_intiou_max_ot: ReadWrite<u32, Gem2mIntiouMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub gem2m_intiou_max_comb_ot: ReadWrite<u32, Gem2mIntiouMaxCombOt::Register>,
    /// AW channel peak rate
    pub gem2m_intiou_aw_p: ReadWrite<u32, Gem2mIntiouAwP::Register>,
    /// AW channel burstiness allowance
    pub gem2m_intiou_aw_b: ReadWrite<u32, Gem2mIntiouAwB::Register>,
    /// AW channel average rate
    pub gem2m_intiou_aw_r: ReadWrite<u32, Gem2mIntiouAwR::Register>,
    /// AR channel peak rate
    pub gem2m_intiou_ar_p: ReadWrite<u32, Gem2mIntiouArP::Register>,
    /// AR channel burstiness allowance
    pub gem2m_intiou_ar_b: ReadWrite<u32, Gem2mIntiouArB::Register>,
    /// AR channel average rate
    pub gem2m_intiou_ar_r: ReadWrite<u32, Gem2mIntiouArR::Register>,
    _padding282928: [u8; 4048],
    /// Read channel QoS value
    pub gem3m_intiou_read_qos: ReadWrite<u32, Gem3mIntiouReadQos::Register>,
    /// Write channel QoS value
    pub gem3m_intiou_write_qos: ReadWrite<u32, Gem3mIntiouWriteQos::Register>,
    /// Issuing functionality modification register
    pub gem3m_intiou_fn_mod: ReadWrite<u32, Gem3mIntiouFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub gem3m_intiou_qos_cntl: ReadWrite<u32, Gem3mIntiouQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub gem3m_intiou_max_ot: ReadWrite<u32, Gem3mIntiouMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub gem3m_intiou_max_comb_ot: ReadWrite<u32, Gem3mIntiouMaxCombOt::Register>,
    /// AW channel peak rate
    pub gem3m_intiou_aw_p: ReadWrite<u32, Gem3mIntiouAwP::Register>,
    /// AW channel burstiness allowance
    pub gem3m_intiou_aw_b: ReadWrite<u32, Gem3mIntiouAwB::Register>,
    /// AW channel average rate
    pub gem3m_intiou_aw_r: ReadWrite<u32, Gem3mIntiouAwR::Register>,
    /// AR channel peak rate
    pub gem3m_intiou_ar_p: ReadWrite<u32, Gem3mIntiouArP::Register>,
    /// AR channel burstiness allowance
    pub gem3m_intiou_ar_b: ReadWrite<u32, Gem3mIntiouArB::Register>,
    /// AR channel average rate
    pub gem3m_intiou_ar_r: ReadWrite<u32, Gem3mIntiouArR::Register>,
    _padding287024: [u8; 3828],
    /// This register is only present if upsizing or downsizing happens
    pub nandm_intiou_ib_fn_mod2: ReadWrite<u32, NandmIntiouIbFnMod2::Register>,
    _padding290856: [u8; 216],
    /// Read channel QoS value
    pub nandm_intiou_ib_read_qos: ReadWrite<u32, NandmIntiouIbReadQos::Register>,
    /// Write channel QoS value
    pub nandm_intiou_ib_write_qos: ReadWrite<u32, NandmIntiouIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub nandm_intiou_ib_fn_mod: ReadWrite<u32, NandmIntiouIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub nandm_intiou_ib_qos_cntl: ReadWrite<u32, NandmIntiouIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub nandm_intiou_ib_max_ot: ReadWrite<u32, NandmIntiouIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub nandm_intiou_ib_max_comb_ot: ReadWrite<u32, NandmIntiouIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub nandm_intiou_ib_aw_p: ReadWrite<u32, NandmIntiouIbAwP::Register>,
    /// AW channel burstiness allowance
    pub nandm_intiou_ib_aw_b: ReadWrite<u32, NandmIntiouIbAwB::Register>,
    /// AW channel average rate
    pub nandm_intiou_ib_aw_r: ReadWrite<u32, NandmIntiouIbAwR::Register>,
    /// AR channel peak rate
    pub nandm_intiou_ib_ar_p: ReadWrite<u32, NandmIntiouIbArP::Register>,
    /// AR channel burstiness allowance
    pub nandm_intiou_ib_ar_b: ReadWrite<u32, NandmIntiouIbArB::Register>,
    /// AR channel average rate
    pub nandm_intiou_ib_ar_r: ReadWrite<u32, NandmIntiouIbArR::Register>,
    _padding291120: [u8; 3828],
    /// This register is only present if upsizing or downsizing happens
    pub sd0m_intiou_ib_fn_mod2: ReadWrite<u32, Sd0mIntiouIbFnMod2::Register>,
    _padding294952: [u8; 216],
    /// Read channel QoS value
    pub sd0m_intiou_ib_read_qos: ReadWrite<u32, Sd0mIntiouIbReadQos::Register>,
    /// Write channel QoS value
    pub sd0m_intiou_ib_write_qos: ReadWrite<u32, Sd0mIntiouIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub sd0m_intiou_ib_fn_mod: ReadWrite<u32, Sd0mIntiouIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub sd0m_intiou_ib_qos_cntl: ReadWrite<u32, Sd0mIntiouIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub sd0m_intiou_ib_max_ot: ReadWrite<u32, Sd0mIntiouIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub sd0m_intiou_ib_max_comb_ot: ReadWrite<u32, Sd0mIntiouIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub sd0m_intiou_ib_aw_p: ReadWrite<u32, Sd0mIntiouIbAwP::Register>,
    /// AW channel burstiness allowance
    pub sd0m_intiou_ib_aw_b: ReadWrite<u32, Sd0mIntiouIbAwB::Register>,
    /// AW channel average rate
    pub sd0m_intiou_ib_aw_r: ReadWrite<u32, Sd0mIntiouIbAwR::Register>,
    /// AR channel peak rate
    pub sd0m_intiou_ib_ar_p: ReadWrite<u32, Sd0mIntiouIbArP::Register>,
    /// AR channel burstiness allowance
    pub sd0m_intiou_ib_ar_b: ReadWrite<u32, Sd0mIntiouIbArB::Register>,
    /// AR channel average rate
    pub sd0m_intiou_ib_ar_r: ReadWrite<u32, Sd0mIntiouIbArR::Register>,
    _padding295216: [u8; 3828],
    /// This register is only present if upsizing or downsizing happens
    pub sd1m_intiou_ib_fn_mod2: ReadWrite<u32, Sd1mIntiouIbFnMod2::Register>,
    _padding299048: [u8; 216],
    /// Read channel QoS value
    pub sd1m_intiou_ib_read_qos: ReadWrite<u32, Sd1mIntiouIbReadQos::Register>,
    /// Write channel QoS value
    pub sd1m_intiou_ib_write_qos: ReadWrite<u32, Sd1mIntiouIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub sd1m_intiou_ib_fn_mod: ReadWrite<u32, Sd1mIntiouIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub sd1m_intiou_ib_qos_cntl: ReadWrite<u32, Sd1mIntiouIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub sd1m_intiou_ib_max_ot: ReadWrite<u32, Sd1mIntiouIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub sd1m_intiou_ib_max_comb_ot: ReadWrite<u32, Sd1mIntiouIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub sd1m_intiou_ib_aw_p: ReadWrite<u32, Sd1mIntiouIbAwP::Register>,
    /// AW channel burstiness allowance
    pub sd1m_intiou_ib_aw_b: ReadWrite<u32, Sd1mIntiouIbAwB::Register>,
    /// AW channel average rate
    pub sd1m_intiou_ib_aw_r: ReadWrite<u32, Sd1mIntiouIbAwR::Register>,
    /// AR channel peak rate
    pub sd1m_intiou_ib_ar_p: ReadWrite<u32, Sd1mIntiouIbArP::Register>,
    /// AR channel burstiness allowance
    pub sd1m_intiou_ib_ar_b: ReadWrite<u32, Sd1mIntiouIbArB::Register>,
    /// AR channel average rate
    pub sd1m_intiou_ib_ar_r: ReadWrite<u32, Sd1mIntiouIbArR::Register>,
    _padding299312: [u8; 3828],
    /// This register is only present if upsizing or downsizing happens
    pub qspim_intiou_ib_fn_mod2: ReadWrite<u32, QspimIntiouIbFnMod2::Register>,
    _padding303144: [u8; 216],
    /// Read channel QoS value
    pub qspim_intiou_ib_read_qos: ReadWrite<u32, QspimIntiouIbReadQos::Register>,
    /// Write channel QoS value
    pub qspim_intiou_ib_write_qos: ReadWrite<u32, QspimIntiouIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub qspim_intiou_ib_fn_mod: ReadWrite<u32, QspimIntiouIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub qspim_intiou_ib_qos_cntl: ReadWrite<u32, QspimIntiouIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub qspim_intiou_ib_max_ot: ReadWrite<u32, QspimIntiouIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub qspim_intiou_ib_max_comb_ot: ReadWrite<u32, QspimIntiouIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub qspim_intiou_ib_aw_p: ReadWrite<u32, QspimIntiouIbAwP::Register>,
    /// AW channel burstiness allowance
    pub qspim_intiou_ib_aw_b: ReadWrite<u32, QspimIntiouIbAwB::Register>,
    /// AW channel average rate
    pub qspim_intiou_ib_aw_r: ReadWrite<u32, QspimIntiouIbAwR::Register>,
    /// AR channel peak rate
    pub qspim_intiou_ib_ar_p: ReadWrite<u32, QspimIntiouIbArP::Register>,
    /// AR channel burstiness allowance
    pub qspim_intiou_ib_ar_b: ReadWrite<u32, QspimIntiouIbArB::Register>,
    /// AR channel average rate
    pub qspim_intiou_ib_ar_r: ReadWrite<u32, QspimIntiouIbArR::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub PeriphId4 [
        PERIPH_ID_4 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PeriphId5 [
        PERIPH_ID_5 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PeriphId6 [
        PERIPH_ID_6 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PeriphId7 [
        PERIPH_ID_7 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PeriphId0 [
        PERIPH_ID_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PeriphId1 [
        PERIPH_ID_1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PeriphId2 [
        PERIPH_ID_2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PeriphId3 [
        REV_AND OFFSET(4) NUMBITS(4) [],
        CUST_MOD_NUM OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CompId0 [
        COMP_ID_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CompId1 [
        COMP_ID_1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CompId2 [
        COMP_ID_2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CompId3 [
        COMP_ID_3 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdFnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ApbNs0IbFnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ApbNs1IbFnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouQosCntl [
        EN_AWAR_OT OFFSET(7) NUMBITS(1) [],
        EN_AR_OT OFFSET(6) NUMBITS(1) [],
        EN_AW_OT OFFSET(5) NUMBITS(1) [],
        EN_AWAR_RATE OFFSET(2) NUMBITS(1) [],
        EN_AR_RATE OFFSET(1) NUMBITS(1) [],
        EN_AW_RATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouQosCntl [
        EN_AWAR_OT OFFSET(7) NUMBITS(1) [],
        EN_AR_OT OFFSET(6) NUMBITS(1) [],
        EN_AW_OT OFFSET(5) NUMBITS(1) [],
        EN_AWAR_RATE OFFSET(2) NUMBITS(1) [],
        EN_AR_RATE OFFSET(1) NUMBITS(1) [],
        EN_AW_RATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem0mIntiouArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouQosCntl [
        EN_AWAR_OT OFFSET(7) NUMBITS(1) [],
        EN_AR_OT OFFSET(6) NUMBITS(1) [],
        EN_AW_OT OFFSET(5) NUMBITS(1) [],
        EN_AWAR_RATE OFFSET(2) NUMBITS(1) [],
        EN_AR_RATE OFFSET(1) NUMBITS(1) [],
        EN_AW_RATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem1mIntiouArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouQosCntl [
        EN_AWAR_OT OFFSET(7) NUMBITS(1) [],
        EN_AR_OT OFFSET(6) NUMBITS(1) [],
        EN_AW_OT OFFSET(5) NUMBITS(1) [],
        EN_AWAR_RATE OFFSET(2) NUMBITS(1) [],
        EN_AR_RATE OFFSET(1) NUMBITS(1) [],
        EN_AW_RATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem2mIntiouArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouQosCntl [
        EN_AWAR_OT OFFSET(7) NUMBITS(1) [],
        EN_AR_OT OFFSET(6) NUMBITS(1) [],
        EN_AW_OT OFFSET(5) NUMBITS(1) [],
        EN_AWAR_RATE OFFSET(2) NUMBITS(1) [],
        EN_AR_RATE OFFSET(1) NUMBITS(1) [],
        EN_AW_RATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gem3mIntiouArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbQosCntl [
        EN_AWAR_OT OFFSET(7) NUMBITS(1) [],
        EN_AR_OT OFFSET(6) NUMBITS(1) [],
        EN_AW_OT OFFSET(5) NUMBITS(1) [],
        EN_AWAR_RATE OFFSET(2) NUMBITS(1) [],
        EN_AR_RATE OFFSET(1) NUMBITS(1) [],
        EN_AW_RATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub NandmIntiouIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbQosCntl [
        EN_AWAR_OT OFFSET(7) NUMBITS(1) [],
        EN_AR_OT OFFSET(6) NUMBITS(1) [],
        EN_AW_OT OFFSET(5) NUMBITS(1) [],
        EN_AWAR_RATE OFFSET(2) NUMBITS(1) [],
        EN_AR_RATE OFFSET(1) NUMBITS(1) [],
        EN_AW_RATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd0mIntiouIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbQosCntl [
        EN_AWAR_OT OFFSET(7) NUMBITS(1) [],
        EN_AR_OT OFFSET(6) NUMBITS(1) [],
        EN_AW_OT OFFSET(5) NUMBITS(1) [],
        EN_AWAR_RATE OFFSET(2) NUMBITS(1) [],
        EN_AR_RATE OFFSET(1) NUMBITS(1) [],
        EN_AW_RATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Sd1mIntiouIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbQosCntl [
        EN_AWAR_OT OFFSET(7) NUMBITS(1) [],
        EN_AR_OT OFFSET(6) NUMBITS(1) [],
        EN_AW_OT OFFSET(5) NUMBITS(1) [],
        EN_AWAR_RATE OFFSET(2) NUMBITS(1) [],
        EN_AR_RATE OFFSET(1) NUMBITS(1) [],
        EN_AW_RATE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub QspimIntiouIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];

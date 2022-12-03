// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// LPD GPV, GPV
pub static mut LPD_GPV: *mut Registers = 0xfe100000 as *mut Registers;
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
    pub intlpd_ocm_fn_mod_iss_bm: ReadWrite<u32, IntlpdOcmFnModIssBm::Register>,
    _padding8204: [u8; 12284],
    /// Bus matrix issuing functionality modification register
    pub intlpd_rpus0_fn_mod_iss_bm: ReadWrite<u32, IntlpdRpus0FnModIssBm::Register>,
    _padding20492: [u8; 4092],
    /// Bus matrix issuing functionality modification register
    pub intlpd_rpus1_fn_mod_iss_bm: ReadWrite<u32, IntlpdRpus1FnModIssBm::Register>,
    _padding24588: [u8; 4092],
    /// Bus matrix issuing functionality modification register
    pub intlpd_usb0s_fn_mod_iss_bm: ReadWrite<u32, IntlpdUsb0sFnModIssBm::Register>,
    _padding28684: [u8; 4092],
    /// Bus matrix issuing functionality modification register
    pub intlpd_usb1s_fn_mod_iss_bm: ReadWrite<u32, IntlpdUsb1sFnModIssBm::Register>,
    _padding32780: [u8; 4092],
    /// Bus matrix issuing functionality modification register
    pub intlpd_afifs2_fn_mod_iss_bm: ReadWrite<u32, IntlpdAfifs2FnModIssBm::Register>,
    _padding36876: [u8; 4092],
    /// Bus matrix issuing functionality modification register
    pub intlpd_intiou_ib_fn_mod_iss_bm: ReadWrite<u32, IntlpdIntiouIbFnModIssBm::Register>,
    _padding40972: [u8; 252],
    /// Issuing functionality modification register
    pub intlpd_intiou_ib_fn_mod: ReadWrite<u32, IntlpdIntiouIbFnMod::Register>,
    _padding41228: [u8; 12028],
    /// Bus matrix issuing functionality modification register
    pub slave_11_ib_fn_mod_iss_bm: ReadWrite<u32, Slave11IbFnModIssBm::Register>,
    _padding53260: [u8; 217332],
    /// Read channel QoS value
    pub rpum0_intlpd_read_qos: ReadWrite<u32, Rpum0IntlpdReadQos::Register>,
    /// Write channel QoS value
    pub rpum0_intlpd_write_qos: ReadWrite<u32, Rpum0IntlpdWriteQos::Register>,
    /// Issuing functionality modification register
    pub rpum0_intlpd_fn_mod: ReadWrite<u32, Rpum0IntlpdFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub rpum0_intlpd_qos_cntl: ReadWrite<u32, Rpum0IntlpdQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub rpum0_intlpd_max_ot: ReadWrite<u32, Rpum0IntlpdMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub rpum0_intlpd_max_comb_ot: ReadWrite<u32, Rpum0IntlpdMaxCombOt::Register>,
    /// AW channel peak rate
    pub rpum0_intlpd_aw_p: ReadWrite<u32, Rpum0IntlpdAwP::Register>,
    /// AW channel burstiness allowance
    pub rpum0_intlpd_aw_b: ReadWrite<u32, Rpum0IntlpdAwB::Register>,
    /// AW channel average rate
    pub rpum0_intlpd_aw_r: ReadWrite<u32, Rpum0IntlpdAwR::Register>,
    /// AR channel peak rate
    pub rpum0_intlpd_ar_p: ReadWrite<u32, Rpum0IntlpdArP::Register>,
    /// AR channel burstiness allowance
    pub rpum0_intlpd_ar_b: ReadWrite<u32, Rpum0IntlpdArB::Register>,
    /// AR channel average rate
    pub rpum0_intlpd_ar_r: ReadWrite<u32, Rpum0IntlpdArR::Register>,
    _padding270640: [u8; 4048],
    /// Read channel QoS value
    pub rpum1_intlpd_read_qos: ReadWrite<u32, Rpum1IntlpdReadQos::Register>,
    /// Write channel QoS value
    pub rpum1_intlpd_write_qos: ReadWrite<u32, Rpum1IntlpdWriteQos::Register>,
    /// Issuing functionality modification register
    pub rpum1_intlpd_fn_mod: ReadWrite<u32, Rpum1IntlpdFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub rpum1_intlpd_qos_cntl: ReadWrite<u32, Rpum1IntlpdQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub rpum1_intlpd_max_ot: ReadWrite<u32, Rpum1IntlpdMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub rpum1_intlpd_max_comb_ot: ReadWrite<u32, Rpum1IntlpdMaxCombOt::Register>,
    /// AW channel peak rate
    pub rpum1_intlpd_aw_p: ReadWrite<u32, Rpum1IntlpdAwP::Register>,
    /// AW channel burstiness allowance
    pub rpum1_intlpd_aw_b: ReadWrite<u32, Rpum1IntlpdAwB::Register>,
    /// AW channel average rate
    pub rpum1_intlpd_aw_r: ReadWrite<u32, Rpum1IntlpdAwR::Register>,
    /// AR channel peak rate
    pub rpum1_intlpd_ar_p: ReadWrite<u32, Rpum1IntlpdArP::Register>,
    /// AR channel burstiness allowance
    pub rpum1_intlpd_ar_b: ReadWrite<u32, Rpum1IntlpdArB::Register>,
    /// AR channel average rate
    pub rpum1_intlpd_ar_r: ReadWrite<u32, Rpum1IntlpdArR::Register>,
    _padding274736: [u8; 3828],
    /// This register is only present if upsizing or downsizing happens
    pub admam_intlpd_ib_fn_mod2: ReadWrite<u32, AdmamIntlpdIbFnMod2::Register>,
    _padding278568: [u8; 224],
    /// Issuing functionality modification register
    pub admam_intlpd_ib_fn_mod: ReadWrite<u32, AdmamIntlpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub admam_intlpd_ib_qos_cntl: ReadWrite<u32, AdmamIntlpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub admam_intlpd_ib_max_ot: ReadWrite<u32, AdmamIntlpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub admam_intlpd_ib_max_comb_ot: ReadWrite<u32, AdmamIntlpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub admam_intlpd_ib_aw_p: ReadWrite<u32, AdmamIntlpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub admam_intlpd_ib_aw_b: ReadWrite<u32, AdmamIntlpdIbAwB::Register>,
    /// AW channel average rate
    pub admam_intlpd_ib_aw_r: ReadWrite<u32, AdmamIntlpdIbAwR::Register>,
    /// AR channel peak rate
    pub admam_intlpd_ib_ar_p: ReadWrite<u32, AdmamIntlpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub admam_intlpd_ib_ar_b: ReadWrite<u32, AdmamIntlpdIbArB::Register>,
    /// AR channel average rate
    pub admam_intlpd_ib_ar_r: ReadWrite<u32, AdmamIntlpdIbArR::Register>,
    _padding278832: [u8; 4056],
    /// Issuing functionality modification register
    pub afifm6m_intlpd_ib_fn_mod: ReadWrite<u32, Afifm6mIntlpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub afifm6m_intlpd_ib_qos_cntl: ReadWrite<u32, Afifm6mIntlpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub afifm6m_intlpd_ib_max_ot: ReadWrite<u32, Afifm6mIntlpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub afifm6m_intlpd_ib_max_comb_ot: ReadWrite<u32, Afifm6mIntlpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub afifm6m_intlpd_ib_aw_p: ReadWrite<u32, Afifm6mIntlpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub afifm6m_intlpd_ib_aw_b: ReadWrite<u32, Afifm6mIntlpdIbAwB::Register>,
    /// AW channel average rate
    pub afifm6m_intlpd_ib_aw_r: ReadWrite<u32, Afifm6mIntlpdIbAwR::Register>,
    /// AR channel peak rate
    pub afifm6m_intlpd_ib_ar_p: ReadWrite<u32, Afifm6mIntlpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub afifm6m_intlpd_ib_ar_b: ReadWrite<u32, Afifm6mIntlpdIbArB::Register>,
    /// AR channel average rate
    pub afifm6m_intlpd_ib_ar_r: ReadWrite<u32, Afifm6mIntlpdIbArR::Register>,
    _padding282928: [u8; 7924],
    /// This register is only present if upsizing or downsizing happens
    pub dap_intlpd_ib_fn_mod2: ReadWrite<u32, DapIntlpdIbFnMod2::Register>,
    _padding290856: [u8; 216],
    /// Read channel QoS value
    pub dap_intlpd_ib_read_qos: ReadWrite<u32, DapIntlpdIbReadQos::Register>,
    /// Write channel QoS value
    pub dap_intlpd_ib_write_qos: ReadWrite<u32, DapIntlpdIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub dap_intlpd_ib_fn_mod: ReadWrite<u32, DapIntlpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub dap_intlpd_ib_qos_cntl: ReadWrite<u32, DapIntlpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub dap_intlpd_ib_max_ot: ReadWrite<u32, DapIntlpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub dap_intlpd_ib_max_comb_ot: ReadWrite<u32, DapIntlpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub dap_intlpd_ib_aw_p: ReadWrite<u32, DapIntlpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub dap_intlpd_ib_aw_b: ReadWrite<u32, DapIntlpdIbAwB::Register>,
    /// AW channel average rate
    pub dap_intlpd_ib_aw_r: ReadWrite<u32, DapIntlpdIbAwR::Register>,
    /// AR channel peak rate
    pub dap_intlpd_ib_ar_p: ReadWrite<u32, DapIntlpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub dap_intlpd_ib_ar_b: ReadWrite<u32, DapIntlpdIbArB::Register>,
    /// AR channel average rate
    pub dap_intlpd_ib_ar_r: ReadWrite<u32, DapIntlpdIbArR::Register>,
    _padding291120: [u8; 4048],
    /// Read channel QoS value
    pub usb0m_intlpd_ib_read_qos: ReadWrite<u32, Usb0mIntlpdIbReadQos::Register>,
    /// Write channel QoS value
    pub usb0m_intlpd_ib_write_qos: ReadWrite<u32, Usb0mIntlpdIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub usb0m_intlpd_ib_fn_mod: ReadWrite<u32, Usb0mIntlpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub usb0m_intlpd_ib_qos_cntl: ReadWrite<u32, Usb0mIntlpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub usb0m_intlpd_ib_max_ot: ReadWrite<u32, Usb0mIntlpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub usb0m_intlpd_ib_max_comb_ot: ReadWrite<u32, Usb0mIntlpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub usb0m_intlpd_ib_aw_p: ReadWrite<u32, Usb0mIntlpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub usb0m_intlpd_ib_aw_b: ReadWrite<u32, Usb0mIntlpdIbAwB::Register>,
    /// AW channel average rate
    pub usb0m_intlpd_ib_aw_r: ReadWrite<u32, Usb0mIntlpdIbAwR::Register>,
    /// AR channel peak rate
    pub usb0m_intlpd_ib_ar_p: ReadWrite<u32, Usb0mIntlpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub usb0m_intlpd_ib_ar_b: ReadWrite<u32, Usb0mIntlpdIbArB::Register>,
    /// AR channel average rate
    pub usb0m_intlpd_ib_ar_r: ReadWrite<u32, Usb0mIntlpdIbArR::Register>,
    _padding295216: [u8; 4048],
    /// Read channel QoS value
    pub usb1m_intlpd_ib_read_qos: ReadWrite<u32, Usb1mIntlpdIbReadQos::Register>,
    /// Write channel QoS value
    pub usb1m_intlpd_ib_write_qos: ReadWrite<u32, Usb1mIntlpdIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub usb1m_intlpd_ib_fn_mod: ReadWrite<u32, Usb1mIntlpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub usb1m_intlpd_ib_qos_cntl: ReadWrite<u32, Usb1mIntlpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub usb1m_intlpd_ib_max_ot: ReadWrite<u32, Usb1mIntlpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub usb1m_intlpd_ib_max_comb_ot: ReadWrite<u32, Usb1mIntlpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub usb1m_intlpd_ib_aw_p: ReadWrite<u32, Usb1mIntlpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub usb1m_intlpd_ib_aw_b: ReadWrite<u32, Usb1mIntlpdIbAwB::Register>,
    /// AW channel average rate
    pub usb1m_intlpd_ib_aw_r: ReadWrite<u32, Usb1mIntlpdIbAwR::Register>,
    /// AR channel peak rate
    pub usb1m_intlpd_ib_ar_p: ReadWrite<u32, Usb1mIntlpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub usb1m_intlpd_ib_ar_b: ReadWrite<u32, Usb1mIntlpdIbArB::Register>,
    /// AR channel average rate
    pub usb1m_intlpd_ib_ar_r: ReadWrite<u32, Usb1mIntlpdIbArR::Register>,
    _padding299312: [u8; 4056],
    /// Issuing functionality modification register
    pub intiou_intlpd_ib_fn_mod: ReadWrite<u32, IntiouIntlpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub intiou_intlpd_ib_qos_cntl: ReadWrite<u32, IntiouIntlpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub intiou_intlpd_ib_max_ot: ReadWrite<u32, IntiouIntlpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub intiou_intlpd_ib_max_comb_ot: ReadWrite<u32, IntiouIntlpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub intiou_intlpd_ib_aw_p: ReadWrite<u32, IntiouIntlpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub intiou_intlpd_ib_aw_b: ReadWrite<u32, IntiouIntlpdIbAwB::Register>,
    /// AW channel average rate
    pub intiou_intlpd_ib_aw_r: ReadWrite<u32, IntiouIntlpdIbAwR::Register>,
    /// AR channel peak rate
    pub intiou_intlpd_ib_ar_p: ReadWrite<u32, IntiouIntlpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub intiou_intlpd_ib_ar_b: ReadWrite<u32, IntiouIntlpdIbArB::Register>,
    /// AR channel average rate
    pub intiou_intlpd_ib_ar_r: ReadWrite<u32, IntiouIntlpdIbArR::Register>,
    _padding303408: [u8; 4056],
    /// Issuing functionality modification register
    pub intcsupmu_intlpd_ib_fn_mod: ReadWrite<u32, IntcsupmuIntlpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub intcsupmu_intlpd_ib_qos_cntl: ReadWrite<u32, IntcsupmuIntlpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub intcsupmu_intlpd_ib_max_ot: ReadWrite<u32, IntcsupmuIntlpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub intcsupmu_intlpd_ib_max_comb_ot: ReadWrite<u32, IntcsupmuIntlpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub intcsupmu_intlpd_ib_aw_p: ReadWrite<u32, IntcsupmuIntlpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub intcsupmu_intlpd_ib_aw_b: ReadWrite<u32, IntcsupmuIntlpdIbAwB::Register>,
    /// AW channel average rate
    pub intcsupmu_intlpd_ib_aw_r: ReadWrite<u32, IntcsupmuIntlpdIbAwR::Register>,
    /// AR channel peak rate
    pub intcsupmu_intlpd_ib_ar_p: ReadWrite<u32, IntcsupmuIntlpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub intcsupmu_intlpd_ib_ar_b: ReadWrite<u32, IntcsupmuIntlpdIbArB::Register>,
    /// AR channel average rate
    pub intcsupmu_intlpd_ib_ar_r: ReadWrite<u32, IntcsupmuIntlpdIbArR::Register>,
    _padding307504: [u8; 4056],
    /// Issuing functionality modification register
    pub intlpdinbound_intlpdmain_fn_mod: ReadWrite<u32, IntlpdinboundIntlpdmainFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub intlpdinbound_intlpdmain_qos_cntl: ReadWrite<u32, IntlpdinboundIntlpdmainQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub intlpdinbound_intlpdmain_max_ot: ReadWrite<u32, IntlpdinboundIntlpdmainMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub intlpdinbound_intlpdmain_max_comb_ot:
        ReadWrite<u32, IntlpdinboundIntlpdmainMaxCombOt::Register>,
    /// AW channel peak rate
    pub intlpdinbound_intlpdmain_aw_p: ReadWrite<u32, IntlpdinboundIntlpdmainAwP::Register>,
    /// AW channel burstiness allowance
    pub intlpdinbound_intlpdmain_aw_b: ReadWrite<u32, IntlpdinboundIntlpdmainAwB::Register>,
    /// AW channel average rate
    pub intlpdinbound_intlpdmain_aw_r: ReadWrite<u32, IntlpdinboundIntlpdmainAwR::Register>,
    /// AR channel peak rate
    pub intlpdinbound_intlpdmain_ar_p: ReadWrite<u32, IntlpdinboundIntlpdmainArP::Register>,
    /// AR channel burstiness allowance
    pub intlpdinbound_intlpdmain_ar_b: ReadWrite<u32, IntlpdinboundIntlpdmainArB::Register>,
    /// AR channel average rate
    pub intlpdinbound_intlpdmain_ar_r: ReadWrite<u32, IntlpdinboundIntlpdmainArR::Register>,
    _padding311600: [u8; 4056],
    /// Issuing functionality modification register
    pub intfpd_intlpdocm_fn_mod: ReadWrite<u32, IntfpdIntlpdocmFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub intfpd_intlpdocm_qos_cntl: ReadWrite<u32, IntfpdIntlpdocmQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub intfpd_intlpdocm_max_ot: ReadWrite<u32, IntfpdIntlpdocmMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub intfpd_intlpdocm_max_comb_ot: ReadWrite<u32, IntfpdIntlpdocmMaxCombOt::Register>,
    /// AW channel peak rate
    pub intfpd_intlpdocm_aw_p: ReadWrite<u32, IntfpdIntlpdocmAwP::Register>,
    /// AW channel burstiness allowance
    pub intfpd_intlpdocm_aw_b: ReadWrite<u32, IntfpdIntlpdocmAwB::Register>,
    /// AW channel average rate
    pub intfpd_intlpdocm_aw_r: ReadWrite<u32, IntfpdIntlpdocmAwR::Register>,
    /// AR channel peak rate
    pub intfpd_intlpdocm_ar_p: ReadWrite<u32, IntfpdIntlpdocmArP::Register>,
    /// AR channel burstiness allowance
    pub intfpd_intlpdocm_ar_b: ReadWrite<u32, IntfpdIntlpdocmArB::Register>,
    /// AR channel average rate
    pub intfpd_intlpdocm_ar_r: ReadWrite<u32, IntfpdIntlpdocmArR::Register>,
    _padding315696: [u8; 478936],
    /// Bus matrix issuing functionality modification register
    pub ib9_fn_mod_iss_bm: ReadWrite<u32, Ib9FnModIssBm::Register>,
    _padding794636: [u8; 252],
    /// Issuing functionality modification register
    pub ib9_fn_mod: ReadWrite<u32, Ib9FnMod::Register>,
    _padding794892: [u8; 3836],
    /// Bus matrix issuing functionality modification register
    pub ib5_fn_mod_iss_bm: ReadWrite<u32, Ib5FnModIssBm::Register>,
    _padding798732: [u8; 24],
    /// This register is only present if upsizing or downsizing happens
    pub ib5_fn_mod2: ReadWrite<u32, Ib5FnMod2::Register>,
    _padding798760: [u8; 224],
    /// Issuing functionality modification register
    pub ib5_fn_mod: ReadWrite<u32, Ib5FnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub ib5_qos_cntl: ReadWrite<u32, Ib5QosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub ib5_max_ot: ReadWrite<u32, Ib5MaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub ib5_max_comb_ot: ReadWrite<u32, Ib5MaxCombOt::Register>,
    /// AW channel peak rate
    pub ib5_aw_p: ReadWrite<u32, Ib5AwP::Register>,
    /// AW channel burstiness allowance
    pub ib5_aw_b: ReadWrite<u32, Ib5AwB::Register>,
    /// AW channel average rate
    pub ib5_aw_r: ReadWrite<u32, Ib5AwR::Register>,
    /// AR channel peak rate
    pub ib5_ar_p: ReadWrite<u32, Ib5ArP::Register>,
    /// AR channel burstiness allowance
    pub ib5_ar_b: ReadWrite<u32, Ib5ArB::Register>,
    /// AR channel average rate
    pub ib5_ar_r: ReadWrite<u32, Ib5ArR::Register>,
    _padding799024: [u8; 3800],
    /// Bus matrix issuing functionality modification register
    pub ib6_fn_mod_iss_bm: ReadWrite<u32, Ib6FnModIssBm::Register>,
    _padding802828: [u8; 24],
    /// This register is only present if upsizing or downsizing happens
    pub ib6_fn_mod2: ReadWrite<u32, Ib6FnMod2::Register>,
    _padding802856: [u8; 224],
    /// Issuing functionality modification register
    pub ib6_fn_mod: ReadWrite<u32, Ib6FnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub ib6_qos_cntl: ReadWrite<u32, Ib6QosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub ib6_max_ot: ReadWrite<u32, Ib6MaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub ib6_max_comb_ot: ReadWrite<u32, Ib6MaxCombOt::Register>,
    /// AW channel peak rate
    pub ib6_aw_p: ReadWrite<u32, Ib6AwP::Register>,
    /// AW channel burstiness allowance
    pub ib6_aw_b: ReadWrite<u32, Ib6AwB::Register>,
    /// AW channel average rate
    pub ib6_aw_r: ReadWrite<u32, Ib6AwR::Register>,
    /// AR channel peak rate
    pub ib6_ar_p: ReadWrite<u32, Ib6ArP::Register>,
    /// AR channel burstiness allowance
    pub ib6_ar_b: ReadWrite<u32, Ib6ArB::Register>,
    /// AR channel average rate
    pub ib6_ar_r: ReadWrite<u32, Ib6ArR::Register>,
    _padding803120: [u8; 3800],
    /// Bus matrix issuing functionality modification register
    pub ib8_fn_mod_iss_bm: ReadWrite<u32, Ib8FnModIssBm::Register>,
    _padding806924: [u8; 24],
    /// This register is only present if upsizing or downsizing happens
    pub ib8_fn_mod2: ReadWrite<u32, Ib8FnMod2::Register>,
    _padding806952: [u8; 224],
    /// Issuing functionality modification register
    pub ib8_fn_mod: ReadWrite<u32, Ib8FnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub ib8_qos_cntl: ReadWrite<u32, Ib8QosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub ib8_max_ot: ReadWrite<u32, Ib8MaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub ib8_max_comb_ot: ReadWrite<u32, Ib8MaxCombOt::Register>,
    /// AW channel peak rate
    pub ib8_aw_p: ReadWrite<u32, Ib8AwP::Register>,
    /// AW channel burstiness allowance
    pub ib8_aw_b: ReadWrite<u32, Ib8AwB::Register>,
    /// AW channel average rate
    pub ib8_aw_r: ReadWrite<u32, Ib8AwR::Register>,
    /// AR channel peak rate
    pub ib8_ar_p: ReadWrite<u32, Ib8ArP::Register>,
    /// AR channel burstiness allowance
    pub ib8_ar_b: ReadWrite<u32, Ib8ArB::Register>,
    /// AR channel average rate
    pub ib8_ar_r: ReadWrite<u32, Ib8ArR::Register>,
    _padding807216: [u8; 3800],
    /// Bus matrix issuing functionality modification register
    pub ib0_fn_mod_iss_bm: ReadWrite<u32, Ib0FnModIssBm::Register>,
    _padding811020: [u8; 24],
    /// This register is only present if upsizing or downsizing happens
    pub ib0_fn_mod2: ReadWrite<u32, Ib0FnMod2::Register>,
    _padding811048: [u8; 224],
    /// Issuing functionality modification register
    pub ib0_fn_mod: ReadWrite<u32, Ib0FnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub ib0_qos_cntl: ReadWrite<u32, Ib0QosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub ib0_max_ot: ReadWrite<u32, Ib0MaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub ib0_max_comb_ot: ReadWrite<u32, Ib0MaxCombOt::Register>,
    /// AW channel peak rate
    pub ib0_aw_p: ReadWrite<u32, Ib0AwP::Register>,
    /// AW channel burstiness allowance
    pub ib0_aw_b: ReadWrite<u32, Ib0AwB::Register>,
    /// AW channel average rate
    pub ib0_aw_r: ReadWrite<u32, Ib0AwR::Register>,
    /// AR channel peak rate
    pub ib0_ar_p: ReadWrite<u32, Ib0ArP::Register>,
    /// AR channel burstiness allowance
    pub ib0_ar_b: ReadWrite<u32, Ib0ArB::Register>,
    /// AR channel average rate
    pub ib0_ar_r: ReadWrite<u32, Ib0ArR::Register>,
    _padding811312: [u8; 3800],
    /// Bus matrix issuing functionality modification register
    pub ib11_fn_mod_iss_bm: ReadWrite<u32, Ib11FnModIssBm::Register>,
    _padding815116: [u8; 24],
    /// This register is only present if upsizing or downsizing happens
    pub ib11_fn_mod2: ReadWrite<u32, Ib11FnMod2::Register>,
    _padding815144: [u8; 224],
    /// Issuing functionality modification register
    pub ib11_fn_mod: ReadWrite<u32, Ib11FnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub ib11_qos_cntl: ReadWrite<u32, Ib11QosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub ib11_max_ot: ReadWrite<u32, Ib11MaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub ib11_max_comb_ot: ReadWrite<u32, Ib11MaxCombOt::Register>,
    /// AW channel peak rate
    pub ib11_aw_p: ReadWrite<u32, Ib11AwP::Register>,
    /// AW channel burstiness allowance
    pub ib11_aw_b: ReadWrite<u32, Ib11AwB::Register>,
    /// AW channel average rate
    pub ib11_aw_r: ReadWrite<u32, Ib11AwR::Register>,
    /// AR channel peak rate
    pub ib11_ar_p: ReadWrite<u32, Ib11ArP::Register>,
    /// AR channel burstiness allowance
    pub ib11_ar_b: ReadWrite<u32, Ib11ArB::Register>,
    /// AR channel average rate
    pub ib11_ar_r: ReadWrite<u32, Ib11ArR::Register>,
    _padding815408: [u8; 3800],
    /// Bus matrix issuing functionality modification register
    pub ib12_fn_mod_iss_bm: ReadWrite<u32, Ib12FnModIssBm::Register>,
    _padding819212: [u8; 24],
    /// This register is only present if upsizing or downsizing happens
    pub ib12_fn_mod2: ReadWrite<u32, Ib12FnMod2::Register>,
    _padding819240: [u8; 224],
    /// Issuing functionality modification register
    pub ib12_fn_mod: ReadWrite<u32, Ib12FnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub ib12_qos_cntl: ReadWrite<u32, Ib12QosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub ib12_max_ot: ReadWrite<u32, Ib12MaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub ib12_max_comb_ot: ReadWrite<u32, Ib12MaxCombOt::Register>,
    /// AW channel peak rate
    pub ib12_aw_p: ReadWrite<u32, Ib12AwP::Register>,
    /// AW channel burstiness allowance
    pub ib12_aw_b: ReadWrite<u32, Ib12AwB::Register>,
    /// AW channel average rate
    pub ib12_aw_r: ReadWrite<u32, Ib12AwR::Register>,
    /// AR channel peak rate
    pub ib12_ar_p: ReadWrite<u32, Ib12ArP::Register>,
    /// AR channel burstiness allowance
    pub ib12_ar_b: ReadWrite<u32, Ib12ArB::Register>,
    /// AR channel average rate
    pub ib12_ar_r: ReadWrite<u32, Ib12ArR::Register>,
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
    pub IntlpdOcmFnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdRpus0FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdRpus1FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdUsb0sFnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdUsb1sFnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdAfifs2FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouIbFnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdIntiouIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Slave11IbFnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdQosCntl [
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
    pub Rpum0IntlpdMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum0IntlpdArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdQosCntl [
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
    pub Rpum1IntlpdMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rpum1IntlpdArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbQosCntl [
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
    pub AdmamIntlpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdmamIntlpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm6mIntlpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm6mIntlpdIbQosCntl [
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
    pub Afifm6mIntlpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm6mIntlpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm6mIntlpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm6mIntlpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm6mIntlpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm6mIntlpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm6mIntlpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm6mIntlpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbQosCntl [
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
    pub DapIntlpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DapIntlpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbQosCntl [
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
    pub Usb0mIntlpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb0mIntlpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbQosCntl [
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
    pub Usb1mIntlpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Usb1mIntlpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdIbQosCntl [
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
    pub IntiouIntlpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntiouIntlpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntcsupmuIntlpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntcsupmuIntlpdIbQosCntl [
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
    pub IntcsupmuIntlpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntcsupmuIntlpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntcsupmuIntlpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntcsupmuIntlpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntcsupmuIntlpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntcsupmuIntlpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntcsupmuIntlpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntcsupmuIntlpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdinboundIntlpdmainFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdinboundIntlpdmainQosCntl [
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
    pub IntlpdinboundIntlpdmainMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdinboundIntlpdmainMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdinboundIntlpdmainAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdinboundIntlpdmainAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdinboundIntlpdmainAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdinboundIntlpdmainArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdinboundIntlpdmainArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntlpdinboundIntlpdmainArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdocmFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdocmQosCntl [
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
    pub IntfpdIntlpdocmMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdocmMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdocmAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdocmAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdocmAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdocmArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdocmArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdocmArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib9FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib9FnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5FnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5FnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5QosCntl [
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
    pub Ib5MaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5MaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5AwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5AwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5AwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5ArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5ArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib5ArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6FnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6FnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6QosCntl [
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
    pub Ib6MaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6MaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6AwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6AwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6AwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6ArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6ArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib6ArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8FnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8FnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8QosCntl [
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
    pub Ib8MaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8MaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8AwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8AwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8AwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8ArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8ArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib8ArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0FnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0FnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0QosCntl [
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
    pub Ib0MaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0MaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0AwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0AwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0AwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0ArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0ArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib0ArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11FnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11FnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11QosCntl [
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
    pub Ib11MaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11MaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11AwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11AwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11AwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11ArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11ArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib11ArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12FnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12FnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12QosCntl [
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
    pub Ib12MaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12MaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12AwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12AwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12AwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12ArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12ArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib12ArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];

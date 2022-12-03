// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite};
/// FPD GPV Module, FPD GPV
pub static mut FPD_GPV: *mut Registers = 0xfd700000 as *mut Registers;
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
    pub intfpd_intlpd_ib_fn_mod_iss_bm: ReadWrite<u32, IntfpdIntlpdIbFnModIssBm::Register>,
    _padding8204: [u8; 24],
    /// This register is only present if upsizing or downsizing happens
    pub intfpd_intlpd_ib_fn_mod2: ReadWrite<u32, IntfpdIntlpdIbFnMod2::Register>,
    _padding8232: [u8; 224],
    /// Issuing functionality modification register
    pub intfpd_intlpd_ib_fn_mod: ReadWrite<u32, IntfpdIntlpdIbFnMod::Register>,
    _padding8460: [u8; 262140],
    /// Issuing functionality modification register
    pub intfpdcci_intfpdmain_ib_fn_mod: ReadWrite<u32, IntfpdcciIntfpdmainIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub intfpdcci_intfpdmain_ib_qos_cntl: ReadWrite<u32, IntfpdcciIntfpdmainIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub intfpdcci_intfpdmain_ib_max_ot: ReadWrite<u32, IntfpdcciIntfpdmainIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub intfpdcci_intfpdmain_ib_max_comb_ot:
        ReadWrite<u32, IntfpdcciIntfpdmainIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub intfpdcci_intfpdmain_ib_aw_p: ReadWrite<u32, IntfpdcciIntfpdmainIbAwP::Register>,
    /// AW channel burstiness allowance
    pub intfpdcci_intfpdmain_ib_aw_b: ReadWrite<u32, IntfpdcciIntfpdmainIbAwB::Register>,
    /// AW channel average rate
    pub intfpdcci_intfpdmain_ib_aw_r: ReadWrite<u32, IntfpdcciIntfpdmainIbAwR::Register>,
    /// AR channel peak rate
    pub intfpdcci_intfpdmain_ib_ar_p: ReadWrite<u32, IntfpdcciIntfpdmainIbArP::Register>,
    /// AR channel burstiness allowance
    pub intfpdcci_intfpdmain_ib_ar_b: ReadWrite<u32, IntfpdcciIntfpdmainIbArB::Register>,
    /// AR channel average rate
    pub intfpdcci_intfpdmain_ib_ar_r: ReadWrite<u32, IntfpdcciIntfpdmainIbArR::Register>,
    _padding270640: [u8; 4056],
    /// Issuing functionality modification register
    pub intfpdsmmutbu3_intfpdmain_fn_mod: ReadWrite<u32, Intfpdsmmutbu3IntfpdmainFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub intfpdsmmutbu3_intfpdmain_qos_cntl:
        ReadWrite<u32, Intfpdsmmutbu3IntfpdmainQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub intfpdsmmutbu3_intfpdmain_max_ot: ReadWrite<u32, Intfpdsmmutbu3IntfpdmainMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub intfpdsmmutbu3_intfpdmain_max_comb_ot:
        ReadWrite<u32, Intfpdsmmutbu3IntfpdmainMaxCombOt::Register>,
    /// AW channel peak rate
    pub intfpdsmmutbu3_intfpdmain_aw_p: ReadWrite<u32, Intfpdsmmutbu3IntfpdmainAwP::Register>,
    /// AW channel burstiness allowance
    pub intfpdsmmutbu3_intfpdmain_aw_b: ReadWrite<u32, Intfpdsmmutbu3IntfpdmainAwB::Register>,
    /// AW channel average rate
    pub intfpdsmmutbu3_intfpdmain_aw_r: ReadWrite<u32, Intfpdsmmutbu3IntfpdmainAwR::Register>,
    /// AR channel peak rate
    pub intfpdsmmutbu3_intfpdmain_ar_p: ReadWrite<u32, Intfpdsmmutbu3IntfpdmainArP::Register>,
    /// AR channel burstiness allowance
    pub intfpdsmmutbu3_intfpdmain_ar_b: ReadWrite<u32, Intfpdsmmutbu3IntfpdmainArB::Register>,
    /// AR channel average rate
    pub intfpdsmmutbu3_intfpdmain_ar_r: ReadWrite<u32, Intfpdsmmutbu3IntfpdmainArR::Register>,
    _padding274736: [u8; 4056],
    /// Issuing functionality modification register
    pub intfpdsmmutbu4_intfpdmain_fn_mod: ReadWrite<u32, Intfpdsmmutbu4IntfpdmainFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub intfpdsmmutbu4_intfpdmain_qos_cntl:
        ReadWrite<u32, Intfpdsmmutbu4IntfpdmainQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub intfpdsmmutbu4_intfpdmain_max_ot: ReadWrite<u32, Intfpdsmmutbu4IntfpdmainMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub intfpdsmmutbu4_intfpdmain_max_comb_ot:
        ReadWrite<u32, Intfpdsmmutbu4IntfpdmainMaxCombOt::Register>,
    /// AW channel peak rate
    pub intfpdsmmutbu4_intfpdmain_aw_p: ReadWrite<u32, Intfpdsmmutbu4IntfpdmainAwP::Register>,
    /// AW channel burstiness allowance
    pub intfpdsmmutbu4_intfpdmain_aw_b: ReadWrite<u32, Intfpdsmmutbu4IntfpdmainAwB::Register>,
    /// AW channel average rate
    pub intfpdsmmutbu4_intfpdmain_aw_r: ReadWrite<u32, Intfpdsmmutbu4IntfpdmainAwR::Register>,
    /// AR channel peak rate
    pub intfpdsmmutbu4_intfpdmain_ar_p: ReadWrite<u32, Intfpdsmmutbu4IntfpdmainArP::Register>,
    /// AR channel burstiness allowance
    pub intfpdsmmutbu4_intfpdmain_ar_b: ReadWrite<u32, Intfpdsmmutbu4IntfpdmainArB::Register>,
    /// AR channel average rate
    pub intfpdsmmutbu4_intfpdmain_ar_r: ReadWrite<u32, Intfpdsmmutbu4IntfpdmainArR::Register>,
    _padding278832: [u8; 4056],
    /// Issuing functionality modification register
    pub afifm0m_intfpd_fn_mod: ReadWrite<u32, Afifm0mIntfpdFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub afifm0m_intfpd_qos_cntl: ReadWrite<u32, Afifm0mIntfpdQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub afifm0m_intfpd_max_ot: ReadWrite<u32, Afifm0mIntfpdMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub afifm0m_intfpd_max_comb_ot: ReadWrite<u32, Afifm0mIntfpdMaxCombOt::Register>,
    /// AW channel peak rate
    pub afifm0m_intfpd_aw_p: ReadWrite<u32, Afifm0mIntfpdAwP::Register>,
    /// AW channel burstiness allowance
    pub afifm0m_intfpd_aw_b: ReadWrite<u32, Afifm0mIntfpdAwB::Register>,
    /// AW channel average rate
    pub afifm0m_intfpd_aw_r: ReadWrite<u32, Afifm0mIntfpdAwR::Register>,
    /// AR channel peak rate
    pub afifm0m_intfpd_ar_p: ReadWrite<u32, Afifm0mIntfpdArP::Register>,
    /// AR channel burstiness allowance
    pub afifm0m_intfpd_ar_b: ReadWrite<u32, Afifm0mIntfpdArB::Register>,
    /// AR channel average rate
    pub afifm0m_intfpd_ar_r: ReadWrite<u32, Afifm0mIntfpdArR::Register>,
    _padding282928: [u8; 4056],
    /// Issuing functionality modification register
    pub afifm1m_intfpd_fn_mod: ReadWrite<u32, Afifm1mIntfpdFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub afifm1m_intfpd_qos_cntl: ReadWrite<u32, Afifm1mIntfpdQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub afifm1m_intfpd_max_ot: ReadWrite<u32, Afifm1mIntfpdMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub afifm1m_intfpd_max_comb_ot: ReadWrite<u32, Afifm1mIntfpdMaxCombOt::Register>,
    /// AW channel peak rate
    pub afifm1m_intfpd_aw_p: ReadWrite<u32, Afifm1mIntfpdAwP::Register>,
    /// AW channel burstiness allowance
    pub afifm1m_intfpd_aw_b: ReadWrite<u32, Afifm1mIntfpdAwB::Register>,
    /// AW channel average rate
    pub afifm1m_intfpd_aw_r: ReadWrite<u32, Afifm1mIntfpdAwR::Register>,
    /// AR channel peak rate
    pub afifm1m_intfpd_ar_p: ReadWrite<u32, Afifm1mIntfpdArP::Register>,
    /// AR channel burstiness allowance
    pub afifm1m_intfpd_ar_b: ReadWrite<u32, Afifm1mIntfpdArB::Register>,
    /// AR channel average rate
    pub afifm1m_intfpd_ar_r: ReadWrite<u32, Afifm1mIntfpdArR::Register>,
    _padding287024: [u8; 4056],
    /// Issuing functionality modification register
    pub afifm2m_intfpd_fn_mod: ReadWrite<u32, Afifm2mIntfpdFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub afifm2m_intfpd_qos_cntl: ReadWrite<u32, Afifm2mIntfpdQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub afifm2m_intfpd_max_ot: ReadWrite<u32, Afifm2mIntfpdMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub afifm2m_intfpd_max_comb_ot: ReadWrite<u32, Afifm2mIntfpdMaxCombOt::Register>,
    /// AW channel peak rate
    pub afifm2m_intfpd_aw_p: ReadWrite<u32, Afifm2mIntfpdAwP::Register>,
    /// AW channel burstiness allowance
    pub afifm2m_intfpd_aw_b: ReadWrite<u32, Afifm2mIntfpdAwB::Register>,
    /// AW channel average rate
    pub afifm2m_intfpd_aw_r: ReadWrite<u32, Afifm2mIntfpdAwR::Register>,
    /// AR channel peak rate
    pub afifm2m_intfpd_ar_p: ReadWrite<u32, Afifm2mIntfpdArP::Register>,
    /// AR channel burstiness allowance
    pub afifm2m_intfpd_ar_b: ReadWrite<u32, Afifm2mIntfpdArB::Register>,
    /// AR channel average rate
    pub afifm2m_intfpd_ar_r: ReadWrite<u32, Afifm2mIntfpdArR::Register>,
    _padding291120: [u8; 4056],
    /// Issuing functionality modification register
    pub intfpdsmmutbu5_intfpdmain_fn_mod: ReadWrite<u32, Intfpdsmmutbu5IntfpdmainFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub intfpdsmmutbu5_intfpdmain_qos_cntl:
        ReadWrite<u32, Intfpdsmmutbu5IntfpdmainQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub intfpdsmmutbu5_intfpdmain_max_ot: ReadWrite<u32, Intfpdsmmutbu5IntfpdmainMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub intfpdsmmutbu5_intfpdmain_max_comb_ot:
        ReadWrite<u32, Intfpdsmmutbu5IntfpdmainMaxCombOt::Register>,
    /// AW channel peak rate
    pub intfpdsmmutbu5_intfpdmain_aw_p: ReadWrite<u32, Intfpdsmmutbu5IntfpdmainAwP::Register>,
    /// AW channel burstiness allowance
    pub intfpdsmmutbu5_intfpdmain_aw_b: ReadWrite<u32, Intfpdsmmutbu5IntfpdmainAwB::Register>,
    /// AW channel average rate
    pub intfpdsmmutbu5_intfpdmain_aw_r: ReadWrite<u32, Intfpdsmmutbu5IntfpdmainAwR::Register>,
    /// AR channel peak rate
    pub intfpdsmmutbu5_intfpdmain_ar_p: ReadWrite<u32, Intfpdsmmutbu5IntfpdmainArP::Register>,
    /// AR channel burstiness allowance
    pub intfpdsmmutbu5_intfpdmain_ar_b: ReadWrite<u32, Intfpdsmmutbu5IntfpdmainArB::Register>,
    /// AR channel average rate
    pub intfpdsmmutbu5_intfpdmain_ar_r: ReadWrite<u32, Intfpdsmmutbu5IntfpdmainArR::Register>,
    _padding295216: [u8; 4056],
    /// Issuing functionality modification register
    pub dp_intfpd_ib_fn_mod: ReadWrite<u32, DpIntfpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub dp_intfpd_ib_qos_cntl: ReadWrite<u32, DpIntfpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub dp_intfpd_ib_max_ot: ReadWrite<u32, DpIntfpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub dp_intfpd_ib_max_comb_ot: ReadWrite<u32, DpIntfpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub dp_intfpd_ib_aw_p: ReadWrite<u32, DpIntfpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub dp_intfpd_ib_aw_b: ReadWrite<u32, DpIntfpdIbAwB::Register>,
    /// AW channel average rate
    pub dp_intfpd_ib_aw_r: ReadWrite<u32, DpIntfpdIbAwR::Register>,
    /// AR channel peak rate
    pub dp_intfpd_ib_ar_p: ReadWrite<u32, DpIntfpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub dp_intfpd_ib_ar_b: ReadWrite<u32, DpIntfpdIbArB::Register>,
    /// AR channel average rate
    pub dp_intfpd_ib_ar_r: ReadWrite<u32, DpIntfpdIbArR::Register>,
    _padding299312: [u8; 4056],
    /// Issuing functionality modification register
    pub afifm3m_intfpd_fn_mod: ReadWrite<u32, Afifm3mIntfpdFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub afifm3m_intfpd_qos_cntl: ReadWrite<u32, Afifm3mIntfpdQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub afifm3m_intfpd_max_ot: ReadWrite<u32, Afifm3mIntfpdMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub afifm3m_intfpd_max_comb_ot: ReadWrite<u32, Afifm3mIntfpdMaxCombOt::Register>,
    /// AW channel peak rate
    pub afifm3m_intfpd_aw_p: ReadWrite<u32, Afifm3mIntfpdAwP::Register>,
    /// AW channel burstiness allowance
    pub afifm3m_intfpd_aw_b: ReadWrite<u32, Afifm3mIntfpdAwB::Register>,
    /// AW channel average rate
    pub afifm3m_intfpd_aw_r: ReadWrite<u32, Afifm3mIntfpdAwR::Register>,
    /// AR channel peak rate
    pub afifm3m_intfpd_ar_p: ReadWrite<u32, Afifm3mIntfpdArP::Register>,
    /// AR channel burstiness allowance
    pub afifm3m_intfpd_ar_b: ReadWrite<u32, Afifm3mIntfpdArB::Register>,
    /// AR channel average rate
    pub afifm3m_intfpd_ar_r: ReadWrite<u32, Afifm3mIntfpdArR::Register>,
    _padding303408: [u8; 4056],
    /// Issuing functionality modification register
    pub afifm4m_intfpd_fn_mod: ReadWrite<u32, Afifm4mIntfpdFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub afifm4m_intfpd_qos_cntl: ReadWrite<u32, Afifm4mIntfpdQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub afifm4m_intfpd_max_ot: ReadWrite<u32, Afifm4mIntfpdMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub afifm4m_intfpd_max_comb_ot: ReadWrite<u32, Afifm4mIntfpdMaxCombOt::Register>,
    /// AW channel peak rate
    pub afifm4m_intfpd_aw_p: ReadWrite<u32, Afifm4mIntfpdAwP::Register>,
    /// AW channel burstiness allowance
    pub afifm4m_intfpd_aw_b: ReadWrite<u32, Afifm4mIntfpdAwB::Register>,
    /// AW channel average rate
    pub afifm4m_intfpd_aw_r: ReadWrite<u32, Afifm4mIntfpdAwR::Register>,
    /// AR channel peak rate
    pub afifm4m_intfpd_ar_p: ReadWrite<u32, Afifm4mIntfpdArP::Register>,
    /// AR channel burstiness allowance
    pub afifm4m_intfpd_ar_b: ReadWrite<u32, Afifm4mIntfpdArB::Register>,
    /// AR channel average rate
    pub afifm4m_intfpd_ar_r: ReadWrite<u32, Afifm4mIntfpdArR::Register>,
    _padding307504: [u8; 4056],
    /// Issuing functionality modification register
    pub afifm5m_intfpd_fn_mod: ReadWrite<u32, Afifm5mIntfpdFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub afifm5m_intfpd_qos_cntl: ReadWrite<u32, Afifm5mIntfpdQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub afifm5m_intfpd_max_ot: ReadWrite<u32, Afifm5mIntfpdMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub afifm5m_intfpd_max_comb_ot: ReadWrite<u32, Afifm5mIntfpdMaxCombOt::Register>,
    /// AW channel peak rate
    pub afifm5m_intfpd_aw_p: ReadWrite<u32, Afifm5mIntfpdAwP::Register>,
    /// AW channel burstiness allowance
    pub afifm5m_intfpd_aw_b: ReadWrite<u32, Afifm5mIntfpdAwB::Register>,
    /// AW channel average rate
    pub afifm5m_intfpd_aw_r: ReadWrite<u32, Afifm5mIntfpdAwR::Register>,
    /// AR channel peak rate
    pub afifm5m_intfpd_ar_p: ReadWrite<u32, Afifm5mIntfpdArP::Register>,
    /// AR channel burstiness allowance
    pub afifm5m_intfpd_ar_b: ReadWrite<u32, Afifm5mIntfpdArB::Register>,
    /// AR channel average rate
    pub afifm5m_intfpd_ar_r: ReadWrite<u32, Afifm5mIntfpdArR::Register>,
    _padding311600: [u8; 4048],
    /// Read channel QoS value
    pub gpu_intfpd_ib_read_qos: ReadWrite<u32, GpuIntfpdIbReadQos::Register>,
    /// Write channel QoS value
    pub gpu_intfpd_ib_write_qos: ReadWrite<u32, GpuIntfpdIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub gpu_intfpd_ib_fn_mod: ReadWrite<u32, GpuIntfpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub gpu_intfpd_ib_qos_cntl: ReadWrite<u32, GpuIntfpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub gpu_intfpd_ib_max_ot: ReadWrite<u32, GpuIntfpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub gpu_intfpd_ib_max_comb_ot: ReadWrite<u32, GpuIntfpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub gpu_intfpd_ib_aw_p: ReadWrite<u32, GpuIntfpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub gpu_intfpd_ib_aw_b: ReadWrite<u32, GpuIntfpdIbAwB::Register>,
    /// AW channel average rate
    pub gpu_intfpd_ib_aw_r: ReadWrite<u32, GpuIntfpdIbAwR::Register>,
    /// AR channel peak rate
    pub gpu_intfpd_ib_ar_p: ReadWrite<u32, GpuIntfpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub gpu_intfpd_ib_ar_b: ReadWrite<u32, GpuIntfpdIbArB::Register>,
    /// AR channel average rate
    pub gpu_intfpd_ib_ar_r: ReadWrite<u32, GpuIntfpdIbArR::Register>,
    _padding315696: [u8; 3828],
    /// This register is only present if upsizing or downsizing happens
    pub pciem_intfpd_ib_fn_mod2: ReadWrite<u32, PciemIntfpdIbFnMod2::Register>,
    _padding319528: [u8; 216],
    /// Read channel QoS value
    pub pciem_intfpd_ib_read_qos: ReadWrite<u32, PciemIntfpdIbReadQos::Register>,
    /// Write channel QoS value
    pub pciem_intfpd_ib_write_qos: ReadWrite<u32, PciemIntfpdIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub pciem_intfpd_ib_fn_mod: ReadWrite<u32, PciemIntfpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub pciem_intfpd_ib_qos_cntl: ReadWrite<u32, PciemIntfpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub pciem_intfpd_ib_max_ot: ReadWrite<u32, PciemIntfpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub pciem_intfpd_ib_max_comb_ot: ReadWrite<u32, PciemIntfpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub pciem_intfpd_ib_aw_p: ReadWrite<u32, PciemIntfpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub pciem_intfpd_ib_aw_b: ReadWrite<u32, PciemIntfpdIbAwB::Register>,
    /// AW channel average rate
    pub pciem_intfpd_ib_aw_r: ReadWrite<u32, PciemIntfpdIbAwR::Register>,
    /// AR channel peak rate
    pub pciem_intfpd_ib_ar_p: ReadWrite<u32, PciemIntfpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub pciem_intfpd_ib_ar_b: ReadWrite<u32, PciemIntfpdIbArB::Register>,
    /// AR channel average rate
    pub pciem_intfpd_ib_ar_r: ReadWrite<u32, PciemIntfpdIbArR::Register>,
    _padding319792: [u8; 3828],
    /// This register is only present if upsizing or downsizing happens
    pub gdma_intfpd_ib_fn_mod2: ReadWrite<u32, GdmaIntfpdIbFnMod2::Register>,
    _padding323624: [u8; 224],
    /// Issuing functionality modification register
    pub gdma_intfpd_ib_fn_mod: ReadWrite<u32, GdmaIntfpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub gdma_intfpd_ib_qos_cntl: ReadWrite<u32, GdmaIntfpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub gdma_intfpd_ib_max_ot: ReadWrite<u32, GdmaIntfpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub gdma_intfpd_ib_max_comb_ot: ReadWrite<u32, GdmaIntfpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub gdma_intfpd_ib_aw_p: ReadWrite<u32, GdmaIntfpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub gdma_intfpd_ib_aw_b: ReadWrite<u32, GdmaIntfpdIbAwB::Register>,
    /// AW channel average rate
    pub gdma_intfpd_ib_aw_r: ReadWrite<u32, GdmaIntfpdIbAwR::Register>,
    /// AR channel peak rate
    pub gdma_intfpd_ib_ar_p: ReadWrite<u32, GdmaIntfpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub gdma_intfpd_ib_ar_b: ReadWrite<u32, GdmaIntfpdIbArB::Register>,
    /// AR channel average rate
    pub gdma_intfpd_ib_ar_r: ReadWrite<u32, GdmaIntfpdIbArR::Register>,
    _padding323888: [u8; 3828],
    /// This register is only present if upsizing or downsizing happens
    pub satam_intfpd_ib_fn_mod2: ReadWrite<u32, SatamIntfpdIbFnMod2::Register>,
    _padding327720: [u8; 216],
    /// Read channel QoS value
    pub satam_intfpd_ib_read_qos: ReadWrite<u32, SatamIntfpdIbReadQos::Register>,
    /// Write channel QoS value
    pub satam_intfpd_ib_write_qos: ReadWrite<u32, SatamIntfpdIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub satam_intfpd_ib_fn_mod: ReadWrite<u32, SatamIntfpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub satam_intfpd_ib_qos_cntl: ReadWrite<u32, SatamIntfpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub satam_intfpd_ib_max_ot: ReadWrite<u32, SatamIntfpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub satam_intfpd_ib_max_comb_ot: ReadWrite<u32, SatamIntfpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub satam_intfpd_ib_aw_p: ReadWrite<u32, SatamIntfpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub satam_intfpd_ib_aw_b: ReadWrite<u32, SatamIntfpdIbAwB::Register>,
    /// AW channel average rate
    pub satam_intfpd_ib_aw_r: ReadWrite<u32, SatamIntfpdIbAwR::Register>,
    /// AR channel peak rate
    pub satam_intfpd_ib_ar_p: ReadWrite<u32, SatamIntfpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub satam_intfpd_ib_ar_b: ReadWrite<u32, SatamIntfpdIbArB::Register>,
    /// AR channel average rate
    pub satam_intfpd_ib_ar_r: ReadWrite<u32, SatamIntfpdIbArR::Register>,
    _padding327984: [u8; 7924],
    /// This register is only present if upsizing or downsizing happens
    pub coresightm_intfpd_ib_fn_mod2: ReadWrite<u32, CoresightmIntfpdIbFnMod2::Register>,
    _padding335912: [u8; 216],
    /// Read channel QoS value
    pub coresightm_intfpd_ib_read_qos: ReadWrite<u32, CoresightmIntfpdIbReadQos::Register>,
    /// Write channel QoS value
    pub coresightm_intfpd_ib_write_qos: ReadWrite<u32, CoresightmIntfpdIbWriteQos::Register>,
    /// Issuing functionality modification register
    pub coresightm_intfpd_ib_fn_mod: ReadWrite<u32, CoresightmIntfpdIbFnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub coresightm_intfpd_ib_qos_cntl: ReadWrite<u32, CoresightmIntfpdIbQosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub coresightm_intfpd_ib_max_ot: ReadWrite<u32, CoresightmIntfpdIbMaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub coresightm_intfpd_ib_max_comb_ot: ReadWrite<u32, CoresightmIntfpdIbMaxCombOt::Register>,
    /// AW channel peak rate
    pub coresightm_intfpd_ib_aw_p: ReadWrite<u32, CoresightmIntfpdIbAwP::Register>,
    /// AW channel burstiness allowance
    pub coresightm_intfpd_ib_aw_b: ReadWrite<u32, CoresightmIntfpdIbAwB::Register>,
    /// AW channel average rate
    pub coresightm_intfpd_ib_aw_r: ReadWrite<u32, CoresightmIntfpdIbAwR::Register>,
    /// AR channel peak rate
    pub coresightm_intfpd_ib_ar_p: ReadWrite<u32, CoresightmIntfpdIbArP::Register>,
    /// AR channel burstiness allowance
    pub coresightm_intfpd_ib_ar_b: ReadWrite<u32, CoresightmIntfpdIbArB::Register>,
    /// AR channel average rate
    pub coresightm_intfpd_ib_ar_r: ReadWrite<u32, CoresightmIntfpdIbArR::Register>,
    _padding336176: [u8; 458456],
    /// Bus matrix issuing functionality modification register
    pub ib2_fn_mod_iss_bm: ReadWrite<u32, Ib2FnModIssBm::Register>,
    _padding794636: [u8; 252],
    /// Issuing functionality modification register
    pub ib2_fn_mod: ReadWrite<u32, Ib2FnMod::Register>,
    /// The QoS control register contains the enable bits for all the regulators.
    pub ib2_qos_cntl: ReadWrite<u32, Ib2QosCntl::Register>,
    /// Maximum number of outstanding transactions
    pub ib2_max_ot: ReadWrite<u32, Ib2MaxOt::Register>,
    /// Maximum number of combined outstanding transactions
    pub ib2_max_comb_ot: ReadWrite<u32, Ib2MaxCombOt::Register>,
    /// AW channel peak rate
    pub ib2_aw_p: ReadWrite<u32, Ib2AwP::Register>,
    /// AW channel burstiness allowance
    pub ib2_aw_b: ReadWrite<u32, Ib2AwB::Register>,
    /// AW channel average rate
    pub ib2_aw_r: ReadWrite<u32, Ib2AwR::Register>,
    /// AR channel peak rate
    pub ib2_ar_p: ReadWrite<u32, Ib2ArP::Register>,
    /// AR channel burstiness allowance
    pub ib2_ar_b: ReadWrite<u32, Ib2ArB::Register>,
    /// AR channel average rate
    pub ib2_ar_r: ReadWrite<u32, Ib2ArR::Register>,
    _padding794928: [u8; 3800],
    /// Bus matrix issuing functionality modification register
    pub ib6_fn_mod_iss_bm: ReadWrite<u32, Ib6FnModIssBm::Register>,
    _padding798732: [u8; 24],
    /// This register is only present if upsizing or downsizing happens
    pub ib6_fn_mod2: ReadWrite<u32, Ib6FnMod2::Register>,
    _padding798760: [u8; 224],
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
    pub IntfpdIntlpdIbFnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdIntlpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdcciIntfpdmainIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdcciIntfpdmainIbQosCntl [
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
    pub IntfpdcciIntfpdmainIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdcciIntfpdmainIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdcciIntfpdmainIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdcciIntfpdmainIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdcciIntfpdmainIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdcciIntfpdmainIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdcciIntfpdmainIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub IntfpdcciIntfpdmainIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu3IntfpdmainFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu3IntfpdmainQosCntl [
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
    pub Intfpdsmmutbu3IntfpdmainMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu3IntfpdmainMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu3IntfpdmainAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu3IntfpdmainAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu3IntfpdmainAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu3IntfpdmainArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu3IntfpdmainArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu3IntfpdmainArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu4IntfpdmainFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu4IntfpdmainQosCntl [
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
    pub Intfpdsmmutbu4IntfpdmainMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu4IntfpdmainMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu4IntfpdmainAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu4IntfpdmainAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu4IntfpdmainAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu4IntfpdmainArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu4IntfpdmainArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu4IntfpdmainArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm0mIntfpdFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm0mIntfpdQosCntl [
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
    pub Afifm0mIntfpdMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm0mIntfpdMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm0mIntfpdAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm0mIntfpdAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm0mIntfpdAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm0mIntfpdArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm0mIntfpdArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm0mIntfpdArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm1mIntfpdFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm1mIntfpdQosCntl [
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
    pub Afifm1mIntfpdMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm1mIntfpdMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm1mIntfpdAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm1mIntfpdAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm1mIntfpdAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm1mIntfpdArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm1mIntfpdArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm1mIntfpdArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm2mIntfpdFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm2mIntfpdQosCntl [
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
    pub Afifm2mIntfpdMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm2mIntfpdMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm2mIntfpdAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm2mIntfpdAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm2mIntfpdAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm2mIntfpdArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm2mIntfpdArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm2mIntfpdArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu5IntfpdmainFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu5IntfpdmainQosCntl [
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
    pub Intfpdsmmutbu5IntfpdmainMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu5IntfpdmainMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu5IntfpdmainAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu5IntfpdmainAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu5IntfpdmainAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu5IntfpdmainArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu5IntfpdmainArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Intfpdsmmutbu5IntfpdmainArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntfpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntfpdIbQosCntl [
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
    pub DpIntfpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntfpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntfpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntfpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntfpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntfpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntfpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DpIntfpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm3mIntfpdFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm3mIntfpdQosCntl [
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
    pub Afifm3mIntfpdMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm3mIntfpdMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm3mIntfpdAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm3mIntfpdAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm3mIntfpdAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm3mIntfpdArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm3mIntfpdArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm3mIntfpdArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm4mIntfpdFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm4mIntfpdQosCntl [
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
    pub Afifm4mIntfpdMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm4mIntfpdMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm4mIntfpdAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm4mIntfpdAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm4mIntfpdAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm4mIntfpdArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm4mIntfpdArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm4mIntfpdArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm5mIntfpdFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm5mIntfpdQosCntl [
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
    pub Afifm5mIntfpdMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm5mIntfpdMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm5mIntfpdAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm5mIntfpdAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm5mIntfpdAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm5mIntfpdArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm5mIntfpdArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Afifm5mIntfpdArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbQosCntl [
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
    pub GpuIntfpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpuIntfpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbQosCntl [
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
    pub PciemIntfpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PciemIntfpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbQosCntl [
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
    pub GdmaIntfpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmaIntfpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbQosCntl [
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
    pub SatamIntfpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SatamIntfpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbFnMod2 [
        BYPASS_MERGE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbReadQos [
        AR_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbWriteQos [
        AW_QOS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbFnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbQosCntl [
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
    pub CoresightmIntfpdIbMaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbMaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbAwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbAwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbAwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CoresightmIntfpdIbArR [
        AR_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2FnModIssBm [
        FN_MOD_ISS_BM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2FnMod [
        FN_MOD OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2QosCntl [
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
    pub Ib2MaxOt [
        AR_MAX_OTI OFFSET(24) NUMBITS(6) [],
        AR_MAX_OTF OFFSET(16) NUMBITS(8) [],
        AW_MAX_OTI OFFSET(8) NUMBITS(6) [],
        AW_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2MaxCombOt [
        AWAR_MAX_OTI OFFSET(8) NUMBITS(7) [],
        AWAR_MAX_OTF OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2AwP [
        AW_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2AwB [
        AW_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2AwR [
        AW_R OFFSET(20) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2ArP [
        AR_P OFFSET(24) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2ArB [
        AR_B OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ib2ArR [
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

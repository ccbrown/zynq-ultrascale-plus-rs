// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// VCU System-Level Control, VCU System-Level Control
pub static mut VCU_SLCR: *mut Registers = 0xa0040000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Enable/Disable a error response
    pub vcu_err_ctrl: Aliased<u32, VcuErrCtrlR::Register, VcuErrCtrlW::Register>,
    _padding4: [u8; 16],
    /// VCU Version Control Register
    pub vcu_version: ReadOnly<u32, VcuVersion::Register>,
    _padding24: [u8; 8],
    /// CRL SLCR Write protection register
    pub crl_wprot: ReadWrite<u8, CrlWprot::Register>,
    _padding33: [u8; 3],
    /// PLL Basic Control
    pub vcu_pll_ctrl: Aliased<u32, VcuPllCtrlR::Register, VcuPllCtrlW::Register>,
    /// Helper data. Values are to be looked up in a table from Data Sheet
    pub vcu_pll_cfg: Aliased<u32, VcuPllCfgR::Register, VcuPllCfgW::Register>,
    _padding44: [u8; 4],
    /// Reference clock control.
    pub enc_core_ctrl: Aliased<u32, EncCoreCtrlR::Register, EncCoreCtrlW::Register>,
    /// This register controls this reference clock
    pub enc_mcu_ctrl: Aliased<u32, EncMcuCtrlR::Register, EncMcuCtrlW::Register>,
    /// This register controls this reference clock
    pub dec_core_ctrl: Aliased<u32, DecCoreCtrlR::Register, DecCoreCtrlW::Register>,
    /// This register controls this reference clock
    pub dec_mcu_ctrl: Aliased<u32, DecMcuCtrlR::Register, DecMcuCtrlW::Register>,
    /// This register controls this reference clock
    pub vcu_axi_ctrl: Aliased<u32, VcuAxiCtrlR::Register, VcuAxiCtrlW::Register>,
    _padding68: [u8; 12],
    /// Caution! The fields in this regsiter must not be changed while the VCU is active and AXI traffic is being produced. It is recommended that this field only change during 'idle/inactive' periods of the VCU
    pub vcu_enc_axi_prot: Aliased<u32, VcuEncAxiProtR::Register, VcuEncAxiProtW::Register>,
    /// Caution! The fields in this regsiter must not be changed while the VCU is active and AXI traffic is being produced. It is recommended that this field only change during 'idle/inactive' periods of the VCU
    pub vcu_dec_axi_prot: Aliased<u32, VcuDecAxiProtR::Register, VcuDecAxiProtW::Register>,
    /// Caution! The fields in this regsiter must not be changed while the VCU is active and AXI traffic is being produced. It is recommended that this field only change during 'idle/inactive' periods of the VCU
    pub vcu_enc_cache_axi_prot:
        Aliased<u32, VcuEncCacheAxiProtR::Register, VcuEncCacheAxiProtW::Register>,
    /// Caution! The fields in this regsiter must not be changed while the VCU is active and AXI traffic is being produced. It is recommended that this field only change during 'idle/inactive' periods of the VCU
    pub vcu_dec_cache_axi_prot:
        Aliased<u32, VcuDecCacheAxiProtR::Register, VcuDecCacheAxiProtW::Register>,
    /// Tells the status of the PLLs
    pub pll_status: ReadOnly<u32, PllStatus::Register>,
    _padding100: [u8; 12],
    /// Interrupt Status Register for intrN. This is a sticky register that holds the value of the interrupt until cleared by a value of 1.
    pub vcu_isr: ReadWrite<u32, VcuIsr::Register>,
    /// Interrupt Mask Register for intrN. This is a read-only location and can be atomically altered by either the IDR or the IER.
    pub vcu_imr: ReadOnly<u32, VcuImr::Register>,
    /// Interrupt Enable Register. A write of 1 to this location will unmask the interrupt. (IMR: 0)
    pub vcu_ien: WriteOnly<u32, VcuIen::Register>,
    /// Interrupt Disable Register. A write of 1 one to this location will mask the interrupt. (IMR: 1)
    pub vcu_ids: WriteOnly<u32, VcuIds::Register>,
    _padding128: [u8; 128],
    /// APM0_CFG
    pub apm0_cfg: Aliased<u32, Apm0CfgR::Register, Apm0CfgW::Register>,
    /// APM0_TIMER
    pub apm0_timer: ReadWrite<u32>,
    /// APM0_TRG
    pub apm0_trg: Aliased<u32, Apm0TrgR::Register, Apm0TrgW::Register>,
    /// APM0_RESULT0
    pub apm0_result0: ReadOnly<u32>,
    /// APM0_RESULT1
    pub apm0_result1: ReadOnly<u32>,
    /// APM0_RESULT2
    pub apm0_result2: ReadOnly<u32>,
    /// APM0_RESULT3
    pub apm0_result3: ReadOnly<u32>,
    /// APM0_RESULT4
    pub apm0_result4: ReadOnly<u32, Apm0Result4::Register>,
    /// APM0_RESULT5
    pub apm0_result5: ReadOnly<u32, Apm0Result5::Register>,
    /// APM0_RESULT6
    pub apm0_result6: ReadOnly<u32, Apm0Result6::Register>,
    /// APM0_RESULT7
    pub apm0_result7: ReadOnly<u32, Apm0Result7::Register>,
    /// APM0_RESULT8
    pub apm0_result8: ReadOnly<u32, Apm0Result8::Register>,
    /// APM0_RESULT9
    pub apm0_result9: ReadOnly<u32, Apm0Result9::Register>,
    /// APM0_RESULT10
    pub apm0_result10: ReadOnly<u32, Apm0Result10::Register>,
    /// APM0_RESULT11
    pub apm0_result11: ReadOnly<u32, Apm0Result11::Register>,
    /// APM0_RESULT12
    pub apm0_result12: ReadOnly<u32, Apm0Result12::Register>,
    /// APM0_RESULT13
    pub apm0_result13: ReadOnly<u32, Apm0Result13::Register>,
    /// APM0_RESULT14
    pub apm0_result14: ReadOnly<u32, Apm0Result14::Register>,
    /// APM0_RESULT15
    pub apm0_result15: ReadOnly<u32, Apm0Result15::Register>,
    /// APM0_RESULT16
    pub apm0_result16: ReadOnly<u32, Apm0Result16::Register>,
    /// APM0_RESULT17
    pub apm0_result17: ReadOnly<u32, Apm0Result17::Register>,
    /// APM0_RESULT18
    pub apm0_result18: ReadOnly<u32, Apm0Result18::Register>,
    /// APM0_RESULT19
    pub apm0_result19: ReadOnly<u32, Apm0Result19::Register>,
    /// APM0_RESULT20
    pub apm0_result20: ReadOnly<u32, Apm0Result20::Register>,
    /// APM0_RESULT21
    pub apm0_result21: ReadOnly<u32, Apm0Result21::Register>,
    /// APM0_RESULT22
    pub apm0_result22: ReadOnly<u32, Apm0Result22::Register>,
    /// APM0_RESULT23
    pub apm0_result23: ReadOnly<u32, Apm0Result23::Register>,
    /// APM0_RESULT24
    pub apm0_result24: ReadOnly<u32, Apm0Result24::Register>,
    _padding368: [u8; 144],
    /// APM1_CFG
    pub apm1_cfg: Aliased<u32, Apm1CfgR::Register, Apm1CfgW::Register>,
    /// APM1_TIMER
    pub apm1_timer: ReadWrite<u32>,
    /// APM1_TRG
    pub apm1_trg: Aliased<u32, Apm1TrgR::Register, Apm1TrgW::Register>,
    /// APM1_RESULT0
    pub apm1_result0: ReadOnly<u32>,
    /// APM1_RESULT1
    pub apm1_result1: ReadOnly<u32>,
    /// APM1_RESULT2
    pub apm1_result2: ReadOnly<u32>,
    /// APM1_RESULT3
    pub apm1_result3: ReadOnly<u32>,
    /// APM1_RESULT4
    pub apm1_result4: ReadOnly<u32, Apm1Result4::Register>,
    /// APM1_RESULT5
    pub apm1_result5: ReadOnly<u32, Apm1Result5::Register>,
    /// APM1_RESULT6
    pub apm1_result6: ReadOnly<u32, Apm1Result6::Register>,
    /// APM1_RESULT7
    pub apm1_result7: ReadOnly<u32, Apm1Result7::Register>,
    /// APM1_RESULT8
    pub apm1_result8: ReadOnly<u32, Apm1Result8::Register>,
    /// APM1_RESULT9
    pub apm1_result9: ReadOnly<u32, Apm1Result9::Register>,
    /// APM1_RESULT10
    pub apm1_result10: ReadOnly<u32, Apm1Result10::Register>,
    /// APM1_RESULT11
    pub apm1_result11: ReadOnly<u32, Apm1Result11::Register>,
    /// APM1_RESULT12
    pub apm1_result12: ReadOnly<u32, Apm1Result12::Register>,
    /// APM1_RESULT13
    pub apm1_result13: ReadOnly<u32, Apm1Result13::Register>,
    /// APM1_RESULT14
    pub apm1_result14: ReadOnly<u32, Apm1Result14::Register>,
    /// APM1_RESULT15
    pub apm1_result15: ReadOnly<u32, Apm1Result15::Register>,
    /// APM1_RESULT16
    pub apm1_result16: ReadOnly<u32, Apm1Result16::Register>,
    /// APM1_RESULT17
    pub apm1_result17: ReadOnly<u32, Apm1Result17::Register>,
    /// APM1_RESULT18
    pub apm1_result18: ReadOnly<u32, Apm1Result18::Register>,
    /// APM1_RESULT19
    pub apm1_result19: ReadOnly<u32, Apm1Result19::Register>,
    /// APM1_RESULT20
    pub apm1_result20: ReadOnly<u32, Apm1Result20::Register>,
    /// APM1_RESULT21
    pub apm1_result21: ReadOnly<u32, Apm1Result21::Register>,
    /// APM1_RESULT22
    pub apm1_result22: ReadOnly<u32, Apm1Result22::Register>,
    /// APM1_RESULT23
    pub apm1_result23: ReadOnly<u32, Apm1Result23::Register>,
    /// APM1_RESULT24
    pub apm1_result24: ReadOnly<u32, Apm1Result24::Register>,
    _padding624: [u8; 144],
    /// APM2_CFG
    pub apm2_cfg: Aliased<u32, Apm2CfgR::Register, Apm2CfgW::Register>,
    /// APM2_TIMER
    pub apm2_timer: ReadWrite<u32>,
    /// APM2_TRG
    pub apm2_trg: Aliased<u32, Apm2TrgR::Register, Apm2TrgW::Register>,
    /// APM2_RESULT0
    pub apm2_result0: ReadOnly<u32>,
    /// APM2_RESULT1
    pub apm2_result1: ReadOnly<u32>,
    /// APM2_RESULT2
    pub apm2_result2: ReadOnly<u32>,
    /// APM2_RESULT3
    pub apm2_result3: ReadOnly<u32>,
    /// APM2_RESULT4
    pub apm2_result4: ReadOnly<u32, Apm2Result4::Register>,
    /// APM2_RESULT5
    pub apm2_result5: ReadOnly<u32, Apm2Result5::Register>,
    /// APM2_RESULT6
    pub apm2_result6: ReadOnly<u32, Apm2Result6::Register>,
    /// APM2_RESULT7
    pub apm2_result7: ReadOnly<u32, Apm2Result7::Register>,
    /// APM2_RESULT8
    pub apm2_result8: ReadOnly<u32, Apm2Result8::Register>,
    /// APM2_RESULT9
    pub apm2_result9: ReadOnly<u32, Apm2Result9::Register>,
    /// APM2_RESULT10
    pub apm2_result10: ReadOnly<u32, Apm2Result10::Register>,
    /// APM2_RESULT11
    pub apm2_result11: ReadOnly<u32, Apm2Result11::Register>,
    /// APM2_RESULT12
    pub apm2_result12: ReadOnly<u32, Apm2Result12::Register>,
    /// APM2_RESULT13
    pub apm2_result13: ReadOnly<u32, Apm2Result13::Register>,
    /// APM2_RESULT14
    pub apm2_result14: ReadOnly<u32, Apm2Result14::Register>,
    /// APM2_RESULT15
    pub apm2_result15: ReadOnly<u32, Apm2Result15::Register>,
    /// APM2_RESULT16
    pub apm2_result16: ReadOnly<u32, Apm2Result16::Register>,
    /// APM2_RESULT17
    pub apm2_result17: ReadOnly<u32, Apm2Result17::Register>,
    /// APM2_RESULT18
    pub apm2_result18: ReadOnly<u32, Apm2Result18::Register>,
    /// APM2_RESULT19
    pub apm2_result19: ReadOnly<u32, Apm2Result19::Register>,
    /// APM2_RESULT20
    pub apm2_result20: ReadOnly<u32, Apm2Result20::Register>,
    /// APM2_RESULT21
    pub apm2_result21: ReadOnly<u32, Apm2Result21::Register>,
    /// APM2_RESULT22
    pub apm2_result22: ReadOnly<u32, Apm2Result22::Register>,
    /// APM2_RESULT23
    pub apm2_result23: ReadOnly<u32, Apm2Result23::Register>,
    /// APM2_RESULT24
    pub apm2_result24: ReadOnly<u32, Apm2Result24::Register>,
    _padding880: [u8; 144],
    /// APM3_CFG
    pub apm3_cfg: Aliased<u32, Apm3CfgR::Register, Apm3CfgW::Register>,
    /// APM3_TIMER
    pub apm3_timer: ReadWrite<u32>,
    /// APM3_TRG
    pub apm3_trg: Aliased<u32, Apm3TrgR::Register, Apm3TrgW::Register>,
    /// APM3_RESULT0
    pub apm3_result0: ReadOnly<u32>,
    /// APM3_RESULT1
    pub apm3_result1: ReadOnly<u32>,
    /// APM3_RESULT2
    pub apm3_result2: ReadOnly<u32>,
    /// APM3_RESULT3
    pub apm3_result3: ReadOnly<u32>,
    /// APM3_RESULT4
    pub apm3_result4: ReadOnly<u32, Apm3Result4::Register>,
    /// APM3_RESULT5
    pub apm3_result5: ReadOnly<u32, Apm3Result5::Register>,
    /// APM3_RESULT6
    pub apm3_result6: ReadOnly<u32, Apm3Result6::Register>,
    /// APM3_RESULT7
    pub apm3_result7: ReadOnly<u32, Apm3Result7::Register>,
    /// APM3_RESULT8
    pub apm3_result8: ReadOnly<u32, Apm3Result8::Register>,
    /// APM3_RESULT9
    pub apm3_result9: ReadOnly<u32, Apm3Result9::Register>,
    /// APM3_RESULT10
    pub apm3_result10: ReadOnly<u32, Apm3Result10::Register>,
    /// APM3_RESULT11
    pub apm3_result11: ReadOnly<u32, Apm3Result11::Register>,
    /// APM3_RESULT12
    pub apm3_result12: ReadOnly<u32, Apm3Result12::Register>,
    /// APM3_RESULT13
    pub apm3_result13: ReadOnly<u32, Apm3Result13::Register>,
    /// APM3_RESULT14
    pub apm3_result14: ReadOnly<u32, Apm3Result14::Register>,
    /// APM3_RESULT15
    pub apm3_result15: ReadOnly<u32, Apm3Result15::Register>,
    /// APM3_RESULT16
    pub apm3_result16: ReadOnly<u32, Apm3Result16::Register>,
    /// APM3_RESULT17
    pub apm3_result17: ReadOnly<u32, Apm3Result17::Register>,
    /// APM3_RESULT18
    pub apm3_result18: ReadOnly<u32, Apm3Result18::Register>,
    /// APM3_RESULT19
    pub apm3_result19: ReadOnly<u32, Apm3Result19::Register>,
    /// APM3_RESULT20
    pub apm3_result20: ReadOnly<u32, Apm3Result20::Register>,
    /// APM3_RESULT21
    pub apm3_result21: ReadOnly<u32, Apm3Result21::Register>,
    /// APM3_RESULT22
    pub apm3_result22: ReadOnly<u32, Apm3Result22::Register>,
    /// APM3_RESULT23
    pub apm3_result23: ReadOnly<u32, Apm3Result23::Register>,
    /// APM3_RESULT24
    pub apm3_result24: ReadOnly<u32, Apm3Result24::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub VcuErrCtrlR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ],
    pub VcuErrCtrlW [
        APB_ERR_RES OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuVersion [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        CTRL OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub CrlWprot [
        ACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuPllCtrlR [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        RESERVED1 OFFSET(18) NUMBITS(9) [],
        CLKOUTDIV OFFSET(16) NUMBITS(2) [],
        RESERVED2 OFFSET(15) NUMBITS(1) [],
        FBDIV OFFSET(8) NUMBITS(7) [],
        RESERVED3 OFFSET(4) NUMBITS(4) [],
        BYPASS OFFSET(3) NUMBITS(1) [],
        PCTRL_POR_IN OFFSET(2) NUMBITS(1) [],
        PSS_PWR_POR OFFSET(1) NUMBITS(1) [],
        RESET OFFSET(0) NUMBITS(1) [],
    ],
    pub VcuPllCtrlW [
        CLKOUTDIV OFFSET(16) NUMBITS(2) [],
        FBDIV OFFSET(8) NUMBITS(7) [],
        BYPASS OFFSET(3) NUMBITS(1) [],
        PCTRL_POR_IN OFFSET(2) NUMBITS(1) [],
        PSS_PWR_POR OFFSET(1) NUMBITS(1) [],
        RESET OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuPllCfgR [
        LOCK_DLY OFFSET(25) NUMBITS(7) [],
        RESERVED0 OFFSET(23) NUMBITS(2) [],
        LOCK_CNT OFFSET(13) NUMBITS(10) [],
        RESERVED1 OFFSET(12) NUMBITS(1) [],
        LFHF OFFSET(10) NUMBITS(2) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        CP OFFSET(5) NUMBITS(4) [],
        RESERVED3 OFFSET(4) NUMBITS(1) [],
        RES OFFSET(0) NUMBITS(4) [],
    ],
    pub VcuPllCfgW [
        LOCK_DLY OFFSET(25) NUMBITS(7) [],
        LOCK_CNT OFFSET(13) NUMBITS(10) [],
        LFHF OFFSET(10) NUMBITS(2) [],
        CP OFFSET(5) NUMBITS(4) [],
        RES OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EncCoreCtrlR [
        CLKACT OFFSET(12) NUMBITS(1) [],
        RESERVED0 OFFSET(10) NUMBITS(2) [],
        DIVISOR0 OFFSET(4) NUMBITS(6) [],
        RESERVED1 OFFSET(1) NUMBITS(3) [],
        SRCSEL OFFSET(0) NUMBITS(1) [],
    ],
    pub EncCoreCtrlW [
        CLKACT OFFSET(12) NUMBITS(1) [],
        DIVISOR0 OFFSET(4) NUMBITS(6) [],
        SRCSEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EncMcuCtrlR [
        CLKACT OFFSET(12) NUMBITS(1) [],
        RESERVED0 OFFSET(10) NUMBITS(2) [],
        DIVISOR0 OFFSET(4) NUMBITS(6) [],
        RESERVED1 OFFSET(1) NUMBITS(3) [],
        SRCSEL OFFSET(0) NUMBITS(1) [],
    ],
    pub EncMcuCtrlW [
        CLKACT OFFSET(12) NUMBITS(1) [],
        DIVISOR0 OFFSET(4) NUMBITS(6) [],
        SRCSEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DecCoreCtrlR [
        CLKACT OFFSET(12) NUMBITS(1) [],
        RESERVED0 OFFSET(10) NUMBITS(2) [],
        DIVISOR0 OFFSET(4) NUMBITS(6) [],
        RESERVED1 OFFSET(1) NUMBITS(3) [],
        SRCSEL OFFSET(0) NUMBITS(1) [],
    ],
    pub DecCoreCtrlW [
        CLKACT OFFSET(12) NUMBITS(1) [],
        DIVISOR0 OFFSET(4) NUMBITS(6) [],
        SRCSEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DecMcuCtrlR [
        CLKACT OFFSET(12) NUMBITS(1) [],
        RESERVED0 OFFSET(10) NUMBITS(2) [],
        DIVISOR0 OFFSET(4) NUMBITS(6) [],
        RESERVED1 OFFSET(1) NUMBITS(3) [],
        SRCSEL OFFSET(0) NUMBITS(1) [],
    ],
    pub DecMcuCtrlW [
        CLKACT OFFSET(12) NUMBITS(1) [],
        DIVISOR0 OFFSET(4) NUMBITS(6) [],
        SRCSEL OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuAxiCtrlR [
        RESERVED0 OFFSET(18) NUMBITS(1) [],
        MCU_CLK_SEL OFFSET(17) NUMBITS(1) [],
        CORE_CLK_SEL OFFSET(16) NUMBITS(1) [],
        MCU_CLKACT OFFSET(15) NUMBITS(1) [],
        DEC_CLKACT OFFSET(14) NUMBITS(1) [],
        ENC_CACHE_CLKACT OFFSET(13) NUMBITS(1) [],
        ENC_CLKACT OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(0) NUMBITS(12) [],
    ],
    pub VcuAxiCtrlW [
        MCU_CLK_SEL OFFSET(17) NUMBITS(1) [],
        CORE_CLK_SEL OFFSET(16) NUMBITS(1) [],
        MCU_CLKACT OFFSET(15) NUMBITS(1) [],
        DEC_CLKACT OFFSET(14) NUMBITS(1) [],
        ENC_CACHE_CLKACT OFFSET(13) NUMBITS(1) [],
        ENC_CLKACT OFFSET(12) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuEncAxiProtR [
        AWQOS1 OFFSET(20) NUMBITS(4) [],
        ARQOS1 OFFSET(16) NUMBITS(4) [],
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        ARPROT1 OFFSET(13) NUMBITS(1) [],
        AWPROT1 OFFSET(12) NUMBITS(1) [],
        AWQOS0 OFFSET(8) NUMBITS(4) [],
        ARQOS0 OFFSET(4) NUMBITS(4) [],
        RESERVED1 OFFSET(2) NUMBITS(2) [],
        ARPROT0 OFFSET(1) NUMBITS(1) [],
        AWPROT0 OFFSET(0) NUMBITS(1) [],
    ],
    pub VcuEncAxiProtW [
        AWQOS1 OFFSET(20) NUMBITS(4) [],
        ARQOS1 OFFSET(16) NUMBITS(4) [],
        ARPROT1 OFFSET(13) NUMBITS(1) [],
        AWPROT1 OFFSET(12) NUMBITS(1) [],
        AWQOS0 OFFSET(8) NUMBITS(4) [],
        ARQOS0 OFFSET(4) NUMBITS(4) [],
        ARPROT0 OFFSET(1) NUMBITS(1) [],
        AWPROT0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuDecAxiProtR [
        AWQOS1 OFFSET(20) NUMBITS(4) [],
        ARQOS1 OFFSET(16) NUMBITS(4) [],
        RESERVED0 OFFSET(14) NUMBITS(2) [],
        ARPROT1 OFFSET(13) NUMBITS(1) [],
        AWPROT1 OFFSET(12) NUMBITS(1) [],
        AWQOS0 OFFSET(8) NUMBITS(4) [],
        ARQOS0 OFFSET(4) NUMBITS(4) [],
        RESERVED1 OFFSET(2) NUMBITS(2) [],
        ARPROT0 OFFSET(1) NUMBITS(1) [],
        AWPROT0 OFFSET(0) NUMBITS(1) [],
    ],
    pub VcuDecAxiProtW [
        AWQOS1 OFFSET(20) NUMBITS(4) [],
        ARQOS1 OFFSET(16) NUMBITS(4) [],
        ARPROT1 OFFSET(13) NUMBITS(1) [],
        AWPROT1 OFFSET(12) NUMBITS(1) [],
        AWQOS0 OFFSET(8) NUMBITS(4) [],
        ARQOS0 OFFSET(4) NUMBITS(4) [],
        ARPROT0 OFFSET(1) NUMBITS(1) [],
        AWPROT0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuEncCacheAxiProtR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        AWCACHE1 OFFSET(12) NUMBITS(4) [],
        ARCACHE1 OFFSET(8) NUMBITS(4) [],
        AWCACHE0 OFFSET(4) NUMBITS(4) [],
        ARCACHE0 OFFSET(0) NUMBITS(4) [],
    ],
    pub VcuEncCacheAxiProtW [
        AWCACHE1 OFFSET(12) NUMBITS(4) [],
        ARCACHE1 OFFSET(8) NUMBITS(4) [],
        AWCACHE0 OFFSET(4) NUMBITS(4) [],
        ARCACHE0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuDecCacheAxiProtR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        AWCACHE1 OFFSET(12) NUMBITS(4) [],
        ARCACHE1 OFFSET(8) NUMBITS(4) [],
        AWCACHE0 OFFSET(4) NUMBITS(4) [],
        ARCACHE0 OFFSET(0) NUMBITS(4) [],
    ],
    pub VcuDecCacheAxiProtW [
        AWCACHE1 OFFSET(12) NUMBITS(4) [],
        ARCACHE1 OFFSET(8) NUMBITS(4) [],
        AWCACHE0 OFFSET(4) NUMBITS(4) [],
        ARCACHE0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PllStatus [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        PLL_STABLE OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(1) NUMBITS(2) [],
        PLL_LOCK OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuIsr [
        APM3_FIFO3_OVFL OFFSET(19) NUMBITS(1) [],
        APM3_FIFO2_OVFL OFFSET(18) NUMBITS(1) [],
        APM3_FIFO1_OVFL OFFSET(17) NUMBITS(1) [],
        APM3_FIFO0_OVFL OFFSET(16) NUMBITS(1) [],
        APM3_RESULT_VALID OFFSET(15) NUMBITS(1) [],
        APM2_FIFO3_OVFL OFFSET(14) NUMBITS(1) [],
        APM2_FIFO2_OVFL OFFSET(13) NUMBITS(1) [],
        APM2_FIFO1_OVFL OFFSET(12) NUMBITS(1) [],
        APM2_FIFO0_OVFL OFFSET(11) NUMBITS(1) [],
        APM2_RESULT_VALID OFFSET(10) NUMBITS(1) [],
        APM1_FIFO3_OVFL OFFSET(9) NUMBITS(1) [],
        APM1_FIFO2_OVFL OFFSET(8) NUMBITS(1) [],
        APM1_FIFO1_OVFL OFFSET(7) NUMBITS(1) [],
        APM1_FIFO0_OVFL OFFSET(6) NUMBITS(1) [],
        APM1_RESULT_VALID OFFSET(5) NUMBITS(1) [],
        APM0_FIFO3_OVFL OFFSET(4) NUMBITS(1) [],
        APM0_FIFO2_OVFL OFFSET(3) NUMBITS(1) [],
        APM0_FIFO1_OVFL OFFSET(2) NUMBITS(1) [],
        APM0_FIFO0_OVFL OFFSET(1) NUMBITS(1) [],
        APM0_RESULT_VALID OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuImr [
        APM3_FIFO3_OVFL OFFSET(19) NUMBITS(1) [],
        APM3_FIFO2_OVFL OFFSET(18) NUMBITS(1) [],
        APM3_FIFO1_OVFL OFFSET(17) NUMBITS(1) [],
        APM3_FIFO0_OVFL OFFSET(16) NUMBITS(1) [],
        APM3_RESULT_VALID OFFSET(15) NUMBITS(1) [],
        APM2_FIFO3_OVFL OFFSET(14) NUMBITS(1) [],
        APM2_FIFO2_OVFL OFFSET(13) NUMBITS(1) [],
        APM2_FIFO1_OVFL OFFSET(12) NUMBITS(1) [],
        APM2_FIFO0_OVFL OFFSET(11) NUMBITS(1) [],
        APM2_RESULT_VALID OFFSET(10) NUMBITS(1) [],
        APM1_FIFO3_OVFL OFFSET(9) NUMBITS(1) [],
        APM1_FIFO2_OVFL OFFSET(8) NUMBITS(1) [],
        APM1_FIFO1_OVFL OFFSET(7) NUMBITS(1) [],
        APM1_FIFO0_OVFL OFFSET(6) NUMBITS(1) [],
        APM1_RESULT_VALID OFFSET(5) NUMBITS(1) [],
        APM0_FIFO3_OVFL OFFSET(4) NUMBITS(1) [],
        APM0_FIFO2_OVFL OFFSET(3) NUMBITS(1) [],
        APM0_FIFO1_OVFL OFFSET(2) NUMBITS(1) [],
        APM0_FIFO0_OVFL OFFSET(1) NUMBITS(1) [],
        APM0_RESULT_VALID OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuIen [
        APM3_FIFO3_OVFL OFFSET(19) NUMBITS(1) [],
        APM3_FIFO2_OVFL OFFSET(18) NUMBITS(1) [],
        APM3_FIFO1_OVFL OFFSET(17) NUMBITS(1) [],
        APM3_FIFO0_OVFL OFFSET(16) NUMBITS(1) [],
        APM3_RESULT_VALID OFFSET(15) NUMBITS(1) [],
        APM2_FIFO3_OVFL OFFSET(14) NUMBITS(1) [],
        APM2_FIFO2_OVFL OFFSET(13) NUMBITS(1) [],
        APM2_FIFO1_OVFL OFFSET(12) NUMBITS(1) [],
        APM2_FIFO0_OVFL OFFSET(11) NUMBITS(1) [],
        APM2_RESULT_VALID OFFSET(10) NUMBITS(1) [],
        APM1_FIFO3_OVFL OFFSET(9) NUMBITS(1) [],
        APM1_FIFO2_OVFL OFFSET(8) NUMBITS(1) [],
        APM1_FIFO1_OVFL OFFSET(7) NUMBITS(1) [],
        APM1_FIFO0_OVFL OFFSET(6) NUMBITS(1) [],
        APM1_RESULT_VALID OFFSET(5) NUMBITS(1) [],
        APM0_FIFO3_OVFL OFFSET(4) NUMBITS(1) [],
        APM0_FIFO2_OVFL OFFSET(3) NUMBITS(1) [],
        APM0_FIFO1_OVFL OFFSET(2) NUMBITS(1) [],
        APM0_FIFO0_OVFL OFFSET(1) NUMBITS(1) [],
        APM0_RESULT_VALID OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub VcuIds [
        APM3_FIFO3_OVFL OFFSET(19) NUMBITS(1) [],
        APM3_FIFO2_OVFL OFFSET(18) NUMBITS(1) [],
        APM3_FIFO1_OVFL OFFSET(17) NUMBITS(1) [],
        APM3_FIFO0_OVFL OFFSET(16) NUMBITS(1) [],
        APM3_RESULT_VALID OFFSET(15) NUMBITS(1) [],
        APM2_FIFO3_OVFL OFFSET(14) NUMBITS(1) [],
        APM2_FIFO2_OVFL OFFSET(13) NUMBITS(1) [],
        APM2_FIFO1_OVFL OFFSET(12) NUMBITS(1) [],
        APM2_FIFO0_OVFL OFFSET(11) NUMBITS(1) [],
        APM2_RESULT_VALID OFFSET(10) NUMBITS(1) [],
        APM1_FIFO3_OVFL OFFSET(9) NUMBITS(1) [],
        APM1_FIFO2_OVFL OFFSET(8) NUMBITS(1) [],
        APM1_FIFO1_OVFL OFFSET(7) NUMBITS(1) [],
        APM1_FIFO0_OVFL OFFSET(6) NUMBITS(1) [],
        APM1_RESULT_VALID OFFSET(5) NUMBITS(1) [],
        APM0_FIFO3_OVFL OFFSET(4) NUMBITS(1) [],
        APM0_FIFO2_OVFL OFFSET(3) NUMBITS(1) [],
        APM0_FIFO1_OVFL OFFSET(2) NUMBITS(1) [],
        APM0_FIFO0_OVFL OFFSET(1) NUMBITS(1) [],
        APM0_RESULT_VALID OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0CfgR [
        RD_ID_LAT_FF1 OFFSET(28) NUMBITS(4) [],
        WR_ID_LAT_FF1 OFFSET(24) NUMBITS(4) [],
        RD_ID_LAT_FF0 OFFSET(20) NUMBITS(4) [],
        WR_ID_LAT_FF0 OFFSET(16) NUMBITS(4) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        SEL_RD_ID_LAT_FF1 OFFSET(7) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF1 OFFSET(6) NUMBITS(1) [],
        SEL_RD_ID_LAT_FF0 OFFSET(5) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF0 OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        MODE OFFSET(2) NUMBITS(1) [],
        ENABLE OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub Apm0CfgW [
        RD_ID_LAT_FF1 OFFSET(28) NUMBITS(4) [],
        WR_ID_LAT_FF1 OFFSET(24) NUMBITS(4) [],
        RD_ID_LAT_FF0 OFFSET(20) NUMBITS(4) [],
        WR_ID_LAT_FF0 OFFSET(16) NUMBITS(4) [],
        SEL_RD_ID_LAT_FF1 OFFSET(7) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF1 OFFSET(6) NUMBITS(1) [],
        SEL_RD_ID_LAT_FF0 OFFSET(5) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF0 OFFSET(4) NUMBITS(1) [],
        MODE OFFSET(2) NUMBITS(1) [],
        ENABLE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0TrgR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        START_STOP OFFSET(0) NUMBITS(1) [],
    ],
    pub Apm0TrgW [
        START_STOP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result4 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_WR_LAT0 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result5 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_WR_LAT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result6 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result7 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result8 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_RD_LAT0 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result9 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_RD_LAT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result10 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result11 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result12 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_WR_LAT1 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result13 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_WR_LAT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result14 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result15 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result16 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_RD_LAT1 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result17 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_RD_LAT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result18 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result19 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result20 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_WR_LAT0 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_WR_LAT0 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result21 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_RD_LAT0 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_RD_LAT0 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result22 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_WR_LAT1 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_WR_LAT1 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result23 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_RD_LAT1 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_RD_LAT1 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm0Result24 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RD_FIFO1_SAMPLE_PRESENT OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RD_FIFO0_SAMPLE_PRESENT OFFSET(16) NUMBITS(7) [],
        RESERVED2 OFFSET(15) NUMBITS(1) [],
        WR_FIFO1_SAMPLE_PRESENT OFFSET(8) NUMBITS(7) [],
        RESERVED3 OFFSET(7) NUMBITS(1) [],
        WR_FIFO0_SAMPLE_PRESENT OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1CfgR [
        RD_ID_LAT_FF1 OFFSET(28) NUMBITS(4) [],
        WR_ID_LAT_FF1 OFFSET(24) NUMBITS(4) [],
        RD_ID_LAT_FF0 OFFSET(20) NUMBITS(4) [],
        WR_ID_LAT_FF0 OFFSET(16) NUMBITS(4) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        SEL_RD_ID_LAT_FF1 OFFSET(7) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF1 OFFSET(6) NUMBITS(1) [],
        SEL_RD_ID_LAT_FF0 OFFSET(5) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF0 OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        MODE OFFSET(2) NUMBITS(1) [],
        ENABLE OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub Apm1CfgW [
        RD_ID_LAT_FF1 OFFSET(28) NUMBITS(4) [],
        WR_ID_LAT_FF1 OFFSET(24) NUMBITS(4) [],
        RD_ID_LAT_FF0 OFFSET(20) NUMBITS(4) [],
        WR_ID_LAT_FF0 OFFSET(16) NUMBITS(4) [],
        SEL_RD_ID_LAT_FF1 OFFSET(7) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF1 OFFSET(6) NUMBITS(1) [],
        SEL_RD_ID_LAT_FF0 OFFSET(5) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF0 OFFSET(4) NUMBITS(1) [],
        MODE OFFSET(2) NUMBITS(1) [],
        ENABLE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1TrgR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        START_STOP OFFSET(0) NUMBITS(1) [],
    ],
    pub Apm1TrgW [
        START_STOP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result4 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_WR_LAT0 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result5 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_WR_LAT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result6 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result7 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result8 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_RD_LAT0 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result9 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_RD_LAT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result10 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result11 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result12 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_WR_LAT1 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result13 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_WR_LAT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result14 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result15 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result16 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_RD_LAT1 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result17 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_RD_LAT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result18 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result19 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result20 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_WR_LAT0 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_WR_LAT0 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result21 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_RD_LAT0 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_RD_LAT0 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result22 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_WR_LAT1 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_WR_LAT1 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result23 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_RD_LAT1 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_RD_LAT1 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm1Result24 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RD_FIFO1_SAMPLE_PRESENT OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RD_FIFO0_SAMPLE_PRESENT OFFSET(16) NUMBITS(7) [],
        RESERVED2 OFFSET(15) NUMBITS(1) [],
        WR_FIFO1_SAMPLE_PRESENT OFFSET(8) NUMBITS(7) [],
        RESERVED3 OFFSET(7) NUMBITS(1) [],
        WR_FIFO0_SAMPLE_PRESENT OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2CfgR [
        RD_ID_LAT_FF1 OFFSET(28) NUMBITS(4) [],
        WR_ID_LAT_FF1 OFFSET(24) NUMBITS(4) [],
        RD_ID_LAT_FF0 OFFSET(20) NUMBITS(4) [],
        WR_ID_LAT_FF0 OFFSET(16) NUMBITS(4) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        SEL_RD_ID_LAT_FF1 OFFSET(7) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF1 OFFSET(6) NUMBITS(1) [],
        SEL_RD_ID_LAT_FF0 OFFSET(5) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF0 OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        MODE OFFSET(2) NUMBITS(1) [],
        ENABLE OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub Apm2CfgW [
        RD_ID_LAT_FF1 OFFSET(28) NUMBITS(4) [],
        WR_ID_LAT_FF1 OFFSET(24) NUMBITS(4) [],
        RD_ID_LAT_FF0 OFFSET(20) NUMBITS(4) [],
        WR_ID_LAT_FF0 OFFSET(16) NUMBITS(4) [],
        SEL_RD_ID_LAT_FF1 OFFSET(7) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF1 OFFSET(6) NUMBITS(1) [],
        SEL_RD_ID_LAT_FF0 OFFSET(5) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF0 OFFSET(4) NUMBITS(1) [],
        MODE OFFSET(2) NUMBITS(1) [],
        ENABLE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2TrgR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        START_STOP OFFSET(0) NUMBITS(1) [],
    ],
    pub Apm2TrgW [
        START_STOP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result4 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_WR_LAT0 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result5 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_WR_LAT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result6 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result7 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result8 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_RD_LAT0 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result9 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_RD_LAT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result10 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result11 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result12 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_WR_LAT1 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result13 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_WR_LAT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result14 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result15 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result16 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_RD_LAT1 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result17 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_RD_LAT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result18 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result19 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result20 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_WR_LAT0 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_WR_LAT0 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result21 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_RD_LAT0 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_RD_LAT0 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result22 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_WR_LAT1 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_WR_LAT1 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result23 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_RD_LAT1 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_RD_LAT1 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm2Result24 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RD_FIFO1_SAMPLE_PRESENT OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RD_FIFO0_SAMPLE_PRESENT OFFSET(16) NUMBITS(7) [],
        RESERVED2 OFFSET(15) NUMBITS(1) [],
        WR_FIFO1_SAMPLE_PRESENT OFFSET(8) NUMBITS(7) [],
        RESERVED3 OFFSET(7) NUMBITS(1) [],
        WR_FIFO0_SAMPLE_PRESENT OFFSET(0) NUMBITS(7) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3CfgR [
        RD_ID_LAT_FF1 OFFSET(28) NUMBITS(4) [],
        WR_ID_LAT_FF1 OFFSET(24) NUMBITS(4) [],
        RD_ID_LAT_FF0 OFFSET(20) NUMBITS(4) [],
        WR_ID_LAT_FF0 OFFSET(16) NUMBITS(4) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        SEL_RD_ID_LAT_FF1 OFFSET(7) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF1 OFFSET(6) NUMBITS(1) [],
        SEL_RD_ID_LAT_FF0 OFFSET(5) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF0 OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(3) NUMBITS(1) [],
        MODE OFFSET(2) NUMBITS(1) [],
        ENABLE OFFSET(1) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub Apm3CfgW [
        RD_ID_LAT_FF1 OFFSET(28) NUMBITS(4) [],
        WR_ID_LAT_FF1 OFFSET(24) NUMBITS(4) [],
        RD_ID_LAT_FF0 OFFSET(20) NUMBITS(4) [],
        WR_ID_LAT_FF0 OFFSET(16) NUMBITS(4) [],
        SEL_RD_ID_LAT_FF1 OFFSET(7) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF1 OFFSET(6) NUMBITS(1) [],
        SEL_RD_ID_LAT_FF0 OFFSET(5) NUMBITS(1) [],
        SEL_WR_ID_LAT_FF0 OFFSET(4) NUMBITS(1) [],
        MODE OFFSET(2) NUMBITS(1) [],
        ENABLE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3TrgR [
        RESERVED0 OFFSET(1) NUMBITS(31) [],
        START_STOP OFFSET(0) NUMBITS(1) [],
    ],
    pub Apm3TrgW [
        START_STOP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result4 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_WR_LAT0 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result5 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_WR_LAT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result6 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result7 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result8 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_RD_LAT0 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result9 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_RD_LAT0 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result10 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result11 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result12 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_WR_LAT1 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result13 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_WR_LAT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result14 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result15 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_WR_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result16 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        ACCUM_RD_LAT1 OFFSET(0) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result17 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(17) NUMBITS(14) [],
        ACCUM_RD_LAT1 OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result18 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result19 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        COUNT_RD_LAT1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result20 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_WR_LAT0 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_WR_LAT0 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result21 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_RD_LAT0 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_RD_LAT0 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result22 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_WR_LAT1 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_WR_LAT1 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result23 [
        VALIDITY_CHECK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(2) [],
        MIN_RD_LAT1 OFFSET(16) NUMBITS(13) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        MAX_RD_LAT1 OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Apm3Result24 [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RD_FIFO1_SAMPLE_PRESENT OFFSET(24) NUMBITS(7) [],
        RESERVED1 OFFSET(23) NUMBITS(1) [],
        RD_FIFO0_SAMPLE_PRESENT OFFSET(16) NUMBITS(7) [],
        RESERVED2 OFFSET(15) NUMBITS(1) [],
        WR_FIFO1_SAMPLE_PRESENT OFFSET(8) NUMBITS(7) [],
        RESERVED3 OFFSET(7) NUMBITS(1) [],
        WR_FIFO0_SAMPLE_PRESENT OFFSET(0) NUMBITS(7) [],
    ]
];

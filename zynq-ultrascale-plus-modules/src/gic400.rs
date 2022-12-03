// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// APU GIC Interrupt Controller, APU GIC Interrupt Controller; GICv2
pub static mut ACPU_GIC: *mut Registers = 0xf9000000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    _padding0: [u8; 65536],
    /// Distributor Control Register
    pub gicd_ctlr: ReadWrite<u32>,
    /// Interrupt Controller Type Register
    pub gicd_typer: ReadOnly<u32>,
    /// Distributor Implementer Identification Register
    pub gicd_iidr: ReadOnly<u32>,
    _padding65548: [u8; 116],
    /// Interrupt Group Registers
    pub gicd_igroupr0: ReadWrite<u32>,
    /// Interrupt Group Registers
    pub gicd_igroupr1: ReadWrite<u32>,
    /// Interrupt Group Registers
    pub gicd_igroupr2: ReadWrite<u32>,
    /// Interrupt Group Registers
    pub gicd_igroupr3: ReadWrite<u32>,
    /// Interrupt Group Registers
    pub gicd_igroupr4: ReadWrite<u32>,
    /// Interrupt Group Registers
    pub gicd_igroupr5: ReadWrite<u32>,
    _padding65688: [u8; 104],
    /// Interrupt Set-Enable Registers
    pub gicd_isenabler0: ReadWrite<u32>,
    /// Interrupt Set-Enable Registers
    pub gicd_isenabler1: ReadWrite<u32>,
    /// Interrupt Set-Enable Registers
    pub gicd_isenabler2: ReadWrite<u32>,
    /// Interrupt Set-Enable Registers
    pub gicd_isenabler3: ReadWrite<u32>,
    /// Interrupt Set-Enable Registers
    pub gicd_isenabler4: ReadWrite<u32>,
    /// Interrupt Set-Enable Registers
    pub gicd_isenabler5: ReadWrite<u32>,
    _padding65816: [u8; 104],
    /// Interrupt Clear-Enable Registers
    pub gicd_icenabler0: ReadWrite<u32>,
    /// Interrupt Clear-Enable Registers
    pub gicd_icenabler1: ReadWrite<u32>,
    /// Interrupt Clear-Enable Registers
    pub gicd_icenabler2: ReadWrite<u32>,
    /// Interrupt Clear-Enable Registers
    pub gicd_icenabler3: ReadWrite<u32>,
    /// Interrupt Clear-Enable Registers
    pub gicd_icenabler4: ReadWrite<u32>,
    /// Interrupt Clear-Enable Registers
    pub gicd_icenabler5: ReadWrite<u32>,
    _padding65944: [u8; 104],
    /// Interrupt Set-Pending Registers
    pub gicd_ispendr0: ReadWrite<u32>,
    /// Interrupt Set-Pending Registers
    pub gicd_ispendr1: ReadWrite<u32>,
    /// Interrupt Set-Pending Registers
    pub gicd_ispendr2: ReadWrite<u32>,
    /// Interrupt Set-Pending Registers
    pub gicd_ispendr3: ReadWrite<u32>,
    /// Interrupt Set-Pending Registers
    pub gicd_ispendr4: ReadWrite<u32>,
    /// Interrupt Set-Pending Registers
    pub gicd_ispendr5: ReadWrite<u32>,
    _padding66072: [u8; 104],
    /// Interrupt Clear-Pending Registers
    pub gicd_icpendr0: ReadWrite<u32>,
    /// Interrupt Clear-Pending Registers
    pub gicd_icpendr1: ReadWrite<u32>,
    /// Interrupt Clear-Pending Registers
    pub gicd_icpendr2: ReadWrite<u32>,
    /// Interrupt Clear-Pending Registers
    pub gicd_icpendr3: ReadWrite<u32>,
    /// Interrupt Clear-Pending Registers
    pub gicd_icpendr4: ReadWrite<u32>,
    /// Interrupt Clear-Pending Registers
    pub gicd_icpendr5: ReadWrite<u32>,
    _padding66200: [u8; 104],
    /// Interrupt Set-Active Registers
    pub gicd_isactiver0: ReadWrite<u32>,
    /// Interrupt Set-Active Registers
    pub gicd_isactiver1: ReadWrite<u32>,
    /// Interrupt Set-Active Registers
    pub gicd_isactiver2: ReadWrite<u32>,
    /// Interrupt Set-Active Registers
    pub gicd_isactiver3: ReadWrite<u32>,
    /// Interrupt Set-Active Registers
    pub gicd_isactiver4: ReadWrite<u32>,
    /// Interrupt Set-Active Registers
    pub gicd_isactiver5: ReadWrite<u32>,
    _padding66328: [u8; 104],
    /// Interrupt Clear-Active Registers
    pub gicd_icactiver0: ReadWrite<u32>,
    /// Interrupt Clear-Active Registers
    pub gicd_icactiver1: ReadWrite<u32>,
    /// Interrupt Clear-Active Registers
    pub gicd_icactiver2: ReadWrite<u32>,
    /// Interrupt Clear-Active Registers
    pub gicd_icactiver3: ReadWrite<u32>,
    /// Interrupt Clear-Active Registers
    pub gicd_icactiver4: ReadWrite<u32>,
    /// Interrupt Clear-Active Registers
    pub gicd_icactiver5: ReadWrite<u32>,
    _padding66456: [u8; 104],
    /// Interrupt Priority Registers
    pub gicd_ipriorityr0: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr1: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr2: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr3: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr4: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr5: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr6: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr7: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr8: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr9: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr10: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr11: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr12: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr13: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr14: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr15: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr16: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr17: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr18: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr19: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr20: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr21: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr22: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr23: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr24: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr25: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr26: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr27: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr28: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr29: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr30: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr31: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr32: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr33: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr34: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr35: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr36: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr37: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr38: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr39: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr40: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr41: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr42: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr43: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr44: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr45: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr46: ReadWrite<u32>,
    /// Interrupt Priority Registers
    pub gicd_ipriorityr47: ReadWrite<u32>,
    _padding66752: [u8; 832],
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr0: ReadOnly<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr1: ReadOnly<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr2: ReadOnly<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr3: ReadOnly<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr4: ReadOnly<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr5: ReadOnly<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr6: ReadOnly<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr7: ReadOnly<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr8: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr9: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr10: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr11: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr12: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr13: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr14: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr15: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr16: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr17: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr18: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr19: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr20: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr21: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr22: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr23: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr24: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr25: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr26: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr27: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr28: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr29: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr30: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr31: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr32: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr33: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr34: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr35: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr36: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr37: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr38: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr39: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr40: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr41: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr42: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr43: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr44: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr45: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr46: ReadWrite<u32>,
    /// Interrupt Processor Targets Registers
    pub gicd_itargetsr47: ReadWrite<u32>,
    _padding67776: [u8; 832],
    /// Interrupt Configuration Registers
    pub gicd_icfgr0: ReadOnly<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr1: ReadOnly<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr2: ReadWrite<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr3: ReadWrite<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr4: ReadWrite<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr5: ReadWrite<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr6: ReadWrite<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr7: ReadWrite<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr8: ReadWrite<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr9: ReadWrite<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr10: ReadWrite<u32>,
    /// Interrupt Configuration Registers
    pub gicd_icfgr11: ReadWrite<u32>,
    _padding68656: [u8; 208],
    /// Private Peripheral Interrupt Status Register
    pub gicd_ppisr: ReadOnly<u32>,
    /// Shared Peripheral Interrupt Status Registers
    pub gicd_spisr0: ReadOnly<u32>,
    /// Shared Peripheral Interrupt Status Registers
    pub gicd_spisr1: ReadOnly<u32>,
    /// Shared Peripheral Interrupt Status Registers
    pub gicd_spisr2: ReadOnly<u32>,
    /// Shared Peripheral Interrupt Status Registers
    pub gicd_spisr3: ReadOnly<u32>,
    /// Shared Peripheral Interrupt Status Registers
    pub gicd_spisr4: ReadOnly<u32>,
    _padding68888: [u8; 488],
    /// Software Generated Interrupt Register
    pub gicd_sgir: WriteOnly<u32>,
    _padding69380: [u8; 12],
    /// SGI Clear-Pending Registers
    pub gicd_cpendsgir0: ReadWrite<u32>,
    /// SGI Clear-Pending Registers
    pub gicd_cpendsgir1: ReadWrite<u32>,
    /// SGI Clear-Pending Registers
    pub gicd_cpendsgir2: ReadWrite<u32>,
    /// SGI Clear-Pending Registers
    pub gicd_cpendsgir3: ReadWrite<u32>,
    /// SGI Set-Pending Registers
    pub gicd_spendsgir0: ReadWrite<u32>,
    /// SGI Set-Pending Registers
    pub gicd_spendsgir1: ReadWrite<u32>,
    /// SGI Set-Pending Registers
    pub gicd_spendsgir2: ReadWrite<u32>,
    /// SGI Set-Pending Registers
    pub gicd_spendsgir3: ReadWrite<u32>,
    _padding69424: [u8; 160],
    /// Peripheral ID4 Register
    pub gicd_pidr4: ReadOnly<u32>,
    /// Peripheral ID5 Register
    pub gicd_pidr5: ReadOnly<u32>,
    /// Peripheral ID6 Register
    pub gicd_pidr6: ReadOnly<u32>,
    /// Peripheral ID7 Register
    pub gicd_pidr7: ReadOnly<u32>,
    /// Peripheral ID0 Register
    pub gicd_pidr0: ReadOnly<u32>,
    /// Peripheral ID1 Register
    pub gicd_pidr1: ReadOnly<u32>,
    /// Peripheral ID2 Register
    pub gicd_pidr2: ReadOnly<u32>,
    /// Peripheral ID3 Register
    pub gicd_pidr3: ReadOnly<u32>,
    /// Component ID0 Register
    pub gicd_cidr0: ReadOnly<u32>,
    /// Component ID1 Register
    pub gicd_cidr1: ReadOnly<u32>,
    /// Component ID2 Register
    pub gicd_cidr2: ReadOnly<u32>,
    /// Component ID3 Register
    pub gicd_cidr3: ReadOnly<u32>,
    _padding69632: [u8; 61440],
    /// CPU Interface Control Register
    pub gicc_ctlr: ReadWrite<u32>,
    /// Interrupt Priority Mask Register
    pub gicc_pmr: ReadWrite<u32>,
    /// Binary Point Register
    pub gicc_bpr: ReadWrite<u32>,
    /// Interrupt Acknowledge Register
    pub gicc_iar: ReadOnly<u32>,
    /// End of Interrupt Register
    pub gicc_eoir: WriteOnly<u32>,
    /// Running Priority Register
    pub gicc_rpr: ReadOnly<u32>,
    /// Highest Priority Pending Interrupt Register
    pub gicc_hppir: ReadOnly<u32>,
    /// Aliased Binary Point Register
    pub gicc_abpr: ReadWrite<u32>,
    /// Aliased Interrupt Acknowledge Register
    pub gicc_aiar: ReadOnly<u32>,
    /// Aliased End of Interrupt Register
    pub gicc_aeoir: WriteOnly<u32>,
    /// Aliased Highest Priority Pending Interrupt Register
    pub gicc_ahppir: ReadOnly<u32>,
    _padding131116: [u8; 164],
    /// Active Priority Register
    pub gicc_apr0: ReadWrite<u32>,
    _padding131284: [u8; 12],
    /// Non-Secure Active Priority Register
    pub gicc_nsapr0: ReadWrite<u32>,
    _padding131300: [u8; 24],
    /// CPU Interface Identification Register
    pub gicc_iidr: ReadOnly<u32>,
    _padding131328: [u8; 65280],
    /// Deactivate Interrupt Register
    pub gicc_dir: WriteOnly<u32>,
    _padding196612: [u8; 65532],
    /// Hypervisor Control Register
    pub gich_hcr: ReadWrite<u32>,
    /// VGIC Type Register
    pub gich_vtr: ReadOnly<u32>,
    /// Virtual Machine Control Register
    pub gich_vmcr: ReadWrite<u32>,
    _padding262156: [u8; 4],
    /// Maintenance Interrupt Status Register
    pub gich_misr: ReadOnly<u32>,
    _padding262164: [u8; 12],
    /// End of Interrupt Status Register
    pub gich_eisr0: ReadOnly<u32>,
    _padding262180: [u8; 12],
    /// Empty List register Status Register
    pub gich_elsr0: ReadOnly<u32>,
    _padding262196: [u8; 188],
    /// Active Priority Register
    pub gich_apr0: ReadWrite<u32>,
    _padding262388: [u8; 12],
    /// List Register 0
    pub gich_lr0: ReadWrite<u32>,
    /// List Register 1
    pub gich_lr1: ReadWrite<u32>,
    /// List Register 2
    pub gich_lr2: ReadWrite<u32>,
    /// List Register 3
    pub gich_lr3: ReadWrite<u32>,
    _padding262416: [u8; 65264],
    /// Hypervisor Control Register
    pub gich_hcr_alias0: ReadWrite<u32>,
    /// VGIC Type Register
    pub gich_vtr_alias0: ReadOnly<u32>,
    /// Virtual Machine Control Register
    pub gich_vmcr_alias0: ReadWrite<u32>,
    _padding327692: [u8; 4],
    /// Maintenance Interrupt Status Register
    pub gich_misr_alias0: ReadOnly<u32>,
    _padding327700: [u8; 12],
    /// End of Interrupt Status Register
    pub gich_eisr0_alias0: ReadOnly<u32>,
    _padding327716: [u8; 12],
    /// Empty List register Status Register
    pub gich_elsr0_alias0: ReadOnly<u32>,
    _padding327732: [u8; 188],
    /// Active Priority Register
    pub gich_apr0_alias0: ReadWrite<u32>,
    _padding327924: [u8; 12],
    /// List Register 0
    pub gich_lr0_alias0: ReadWrite<u32>,
    /// List Register 1
    pub gich_lr1_alias0: ReadWrite<u32>,
    /// List Register 2
    pub gich_lr2_alias0: ReadWrite<u32>,
    /// List Register 3
    pub gich_lr3_alias0: ReadWrite<u32>,
    _padding327952: [u8; 240],
    /// Hypervisor Control Register
    pub gich_hcr_alias1: ReadWrite<u32>,
    /// VGIC Type Register
    pub gich_vtr_alias1: ReadOnly<u32>,
    /// Virtual Machine Control Register
    pub gich_vmcr_alias1: ReadWrite<u32>,
    _padding328204: [u8; 4],
    /// Maintenance Interrupt Status Register
    pub gich_misr_alias1: ReadOnly<u32>,
    _padding328212: [u8; 12],
    /// End of Interrupt Status Register
    pub gich_eisr0_alias1: ReadOnly<u32>,
    _padding328228: [u8; 12],
    /// Empty List register Status Register
    pub gich_elsr0_alias1: ReadOnly<u32>,
    _padding328244: [u8; 188],
    /// Active Priority Register
    pub gich_apr0_alias1: ReadWrite<u32>,
    _padding328436: [u8; 12],
    /// List Register 0
    pub gich_lr0_alias1: ReadWrite<u32>,
    /// List Register 1
    pub gich_lr1_alias1: ReadWrite<u32>,
    /// List Register 2
    pub gich_lr2_alias1: ReadWrite<u32>,
    /// List Register 3
    pub gich_lr3_alias1: ReadWrite<u32>,
    _padding328464: [u8; 240],
    /// Hypervisor Control Register
    pub gich_hcr_alias2: ReadWrite<u32>,
    /// VGIC Type Register
    pub gich_vtr_alias2: ReadOnly<u32>,
    /// Virtual Machine Control Register
    pub gich_vmcr_alias2: ReadWrite<u32>,
    _padding328716: [u8; 4],
    /// Maintenance Interrupt Status Register
    pub gich_misr_alias2: ReadOnly<u32>,
    _padding328724: [u8; 12],
    /// End of Interrupt Status Register
    pub gich_eisr0_alias2: ReadOnly<u32>,
    _padding328740: [u8; 12],
    /// Empty List register Status Register
    pub gich_elsr0_alias2: ReadOnly<u32>,
    _padding328756: [u8; 188],
    /// Active Priority Register
    pub gich_apr0_alias2: ReadWrite<u32>,
    _padding328948: [u8; 12],
    /// List Register 0
    pub gich_lr0_alias2: ReadWrite<u32>,
    /// List Register 1
    pub gich_lr1_alias2: ReadWrite<u32>,
    /// List Register 2
    pub gich_lr2_alias2: ReadWrite<u32>,
    /// List Register 3
    pub gich_lr3_alias2: ReadWrite<u32>,
    _padding328976: [u8; 240],
    /// Hypervisor Control Register
    pub gich_hcr_alias3: ReadWrite<u32>,
    /// VGIC Type Register
    pub gich_vtr_alias3: ReadOnly<u32>,
    /// Virtual Machine Control Register
    pub gich_vmcr_alias3: ReadWrite<u32>,
    _padding329228: [u8; 4],
    /// Maintenance Interrupt Status Register
    pub gich_misr_alias3: ReadOnly<u32>,
    _padding329236: [u8; 12],
    /// End of Interrupt Status Register
    pub gich_eisr0_alias3: ReadOnly<u32>,
    _padding329252: [u8; 12],
    /// Empty List register Status Register
    pub gich_elsr0_alias3: ReadOnly<u32>,
    _padding329268: [u8; 188],
    /// Active Priority Register
    pub gich_apr0_alias3: ReadWrite<u32>,
    _padding329460: [u8; 12],
    /// List Register 0
    pub gich_lr0_alias3: ReadWrite<u32>,
    /// List Register 1
    pub gich_lr1_alias3: ReadWrite<u32>,
    /// List Register 2
    pub gich_lr2_alias3: ReadWrite<u32>,
    /// List Register 3
    pub gich_lr3_alias3: ReadWrite<u32>,
    _padding329488: [u8; 240],
    /// Hypervisor Control Register
    pub gich_hcr_alias4: ReadWrite<u32>,
    /// VGIC Type Register
    pub gich_vtr_alias4: ReadOnly<u32>,
    /// Virtual Machine Control Register
    pub gich_vmcr_alias4: ReadWrite<u32>,
    _padding329740: [u8; 4],
    /// Maintenance Interrupt Status Register
    pub gich_misr_alias4: ReadOnly<u32>,
    _padding329748: [u8; 12],
    /// End of Interrupt Status Register
    pub gich_eisr0_alias4: ReadOnly<u32>,
    _padding329764: [u8; 12],
    /// Empty List register Status Register
    pub gich_elsr0_alias4: ReadOnly<u32>,
    _padding329780: [u8; 188],
    /// Active Priority Register
    pub gich_apr0_alias4: ReadWrite<u32>,
    _padding329972: [u8; 12],
    /// List Register 0
    pub gich_lr0_alias4: ReadWrite<u32>,
    /// List Register 1
    pub gich_lr1_alias4: ReadWrite<u32>,
    /// List Register 2
    pub gich_lr2_alias4: ReadWrite<u32>,
    /// List Register 3
    pub gich_lr3_alias4: ReadWrite<u32>,
    _padding330000: [u8; 240],
    /// Hypervisor Control Register
    pub gich_hcr_alias5: ReadWrite<u32>,
    /// VGIC Type Register
    pub gich_vtr_alias5: ReadOnly<u32>,
    /// Virtual Machine Control Register
    pub gich_vmcr_alias5: ReadWrite<u32>,
    _padding330252: [u8; 4],
    /// Maintenance Interrupt Status Register
    pub gich_misr_alias5: ReadOnly<u32>,
    _padding330260: [u8; 12],
    /// End of Interrupt Status Register
    pub gich_eisr0_alias5: ReadOnly<u32>,
    _padding330276: [u8; 12],
    /// Empty List register Status Register
    pub gich_elsr0_alias5: ReadOnly<u32>,
    _padding330292: [u8; 188],
    /// Active Priority Register
    pub gich_apr0_alias5: ReadWrite<u32>,
    _padding330484: [u8; 12],
    /// List Register 0
    pub gich_lr0_alias5: ReadWrite<u32>,
    /// List Register 1
    pub gich_lr1_alias5: ReadWrite<u32>,
    /// List Register 2
    pub gich_lr2_alias5: ReadWrite<u32>,
    /// List Register 3
    pub gich_lr3_alias5: ReadWrite<u32>,
    _padding330512: [u8; 240],
    /// Hypervisor Control Register
    pub gich_hcr_alias6: ReadWrite<u32>,
    /// VGIC Type Register
    pub gich_vtr_alias6: ReadOnly<u32>,
    /// Virtual Machine Control Register
    pub gich_vmcr_alias6: ReadWrite<u32>,
    _padding330764: [u8; 4],
    /// Maintenance Interrupt Status Register
    pub gich_misr_alias6: ReadOnly<u32>,
    _padding330772: [u8; 12],
    /// End of Interrupt Status Register
    pub gich_eisr0_alias6: ReadOnly<u32>,
    _padding330788: [u8; 12],
    /// Empty List register Status Register
    pub gich_elsr0_alias6: ReadOnly<u32>,
    _padding330804: [u8; 188],
    /// Active Priority Register
    pub gich_apr0_alias6: ReadWrite<u32>,
    _padding330996: [u8; 12],
    /// List Register 0
    pub gich_lr0_alias6: ReadWrite<u32>,
    /// List Register 1
    pub gich_lr1_alias6: ReadWrite<u32>,
    /// List Register 2
    pub gich_lr2_alias6: ReadWrite<u32>,
    /// List Register 3
    pub gich_lr3_alias6: ReadWrite<u32>,
    _padding331024: [u8; 240],
    /// Hypervisor Control Register
    pub gich_hcr_alias7: ReadWrite<u32>,
    /// VGIC Type Register
    pub gich_vtr_alias7: ReadOnly<u32>,
    /// Virtual Machine Control Register
    pub gich_vmcr_alias7: ReadWrite<u32>,
    _padding331276: [u8; 4],
    /// Maintenance Interrupt Status Register
    pub gich_misr_alias7: ReadOnly<u32>,
    _padding331284: [u8; 12],
    /// End of Interrupt Status Register
    pub gich_eisr0_alias7: ReadOnly<u32>,
    _padding331300: [u8; 12],
    /// Empty List register Status Register
    pub gich_elsr0_alias7: ReadOnly<u32>,
    _padding331316: [u8; 188],
    /// Active Priority Register
    pub gich_apr0_alias7: ReadWrite<u32>,
    _padding331508: [u8; 12],
    /// List Register 0
    pub gich_lr0_alias7: ReadWrite<u32>,
    /// List Register 1
    pub gich_lr1_alias7: ReadWrite<u32>,
    /// List Register 2
    pub gich_lr2_alias7: ReadWrite<u32>,
    /// List Register 3
    pub gich_lr3_alias7: ReadWrite<u32>,
    _padding331536: [u8; 61680],
    /// Virtual Machine Control Register
    pub gicv_ctlr: ReadWrite<u32>,
    /// VM Priority Mask Register
    pub gicv_pmr: ReadWrite<u32>,
    /// VM Binary Point Register
    pub gicv_bpr: ReadWrite<u32>,
    /// VM Interrupt Acknowledge Register
    pub gicv_iar: ReadOnly<u32>,
    /// VM End of Interrupt Register
    pub gicv_eoir: WriteOnly<u32>,
    /// VM Running Priority Register
    pub gicv_rpr: ReadOnly<u32>,
    /// VM Highest Priority Pending Interrupt Register
    pub gicv_hppir: ReadOnly<u32>,
    /// VM Aliased Binary Point Register
    pub gicv_abpr: ReadWrite<u32>,
    /// VM Aliased Interrupt Acknowledge Register
    pub gicv_aiar: ReadOnly<u32>,
    /// VM Aliased End of Interrupt Register
    pub gicv_aeoir: WriteOnly<u32>,
    /// VM Aliased Highest Priority Pending Interrupt Register
    pub gicv_ahppir: ReadOnly<u32>,
    _padding393260: [u8; 164],
    /// VM Active Priority Register
    pub gicv_apr0: ReadWrite<u32>,
    _padding393428: [u8; 40],
    /// VM CPU Interface Identification Register
    pub gicv_iidr: ReadOnly<u32>,
    _padding393472: [u8; 65280],
    /// VM Deactivate Interrupt Register
    pub gicv_dir: WriteOnly<u32>,
}

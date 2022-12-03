// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite, WriteOnly};
/// FPD System Memory Management Unit (GPV), SMMU-500 GPV
pub static mut SMMU_GPV: *mut Registers = 0xfd800000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Provides top-level control of the SMMU.
    pub smmu_scr0: Aliased<u32, SmmuScr0R::Register, SmmuScr0W::Register>,
    /// Provides top-level Secure control of the SMMU.
    pub smmu_scr1: Aliased<u32, SmmuScr1R::Register, SmmuScr1W::Register>,
    _padding8: [u8; 8],
    /// Provides IMPLEMENTATION DEFINED functionality.
    pub smmu_sacr: ReadWrite<u32, SmmuSacr::Register>,
    _padding20: [u8; 12],
    /// Provides SMMU capability information.
    pub smmu_sidr0: ReadOnly<u32, SmmuSidr0::Register>,
    /// Provides SMMU capability information.
    pub smmu_sidr1: ReadOnly<u32, SmmuSidr1::Register>,
    /// Provides SMMU capability information.
    pub smmu_sidr2: ReadOnly<u32, SmmuSidr2::Register>,
    _padding44: [u8; 16],
    /// Provides SMMU capability information.
    pub smmu_sidr7: ReadOnly<u32, SmmuSidr7::Register>,
    /// Contains the input address of an erroneous request reported by SMMU_sGFSR.
    pub smmu_sgfar_low: ReadWrite<u32>,
    /// Contains the input address of an erroneous request reported by SMMU_sGFSR.
    pub smmu_sgfar_high: ReadWrite<u32, SmmuSgfarHigh::Register>,
    /// Gives the fault status for each of the following possible faults.
    pub smmu_sgfsr: WriteOnly<u32, SmmuSgfsr::Register>,
    /// Restores the state of SMMU_sGFSR, after a reset, for example.
    pub smmu_sgfsrrestore: WriteOnly<u32, SmmuSgfsrrestore::Register>,
    /// Contains fault syndrome information relating to SMMU_sGFSR.
    pub smmu_sgfsynr0: Aliased<u32, SmmuSgfsynr0R::Register, SmmuSgfsynr0W::Register>,
    /// Contains fault syndrome information relating to SMMU_sGFSR.
    pub smmu_sgfsynr1: ReadWrite<u32, SmmuSgfsynr1::Register>,
    _padding88: [u8; 8],
    /// Invalidates all unlocked Secure entries in the TLB.
    pub smmu_stlbiall: WriteOnly<u32>,
    /// Invalidates all Non-secure non-Hyp TLB entries having the specified VMID.
    pub smmu_tlbivmid: WriteOnly<u32, SmmuTlbivmid::Register>,
    /// Invalidates all Non-secure non-Hyp tagged entries in the TLB.
    pub smmu_tlbiallnsnh: WriteOnly<u32>,
    _padding108: [u8; 4],
    /// Starts a global synchronization operation that ensures the completion of any previously accepted TLB Invalidate operation. As a minimum, the operation applies to the specified security state, and includes all TLB Invalidate operations initiated in context banks associated with that security state.
    pub smmu_stlbgsync: WriteOnly<u32>,
    /// Gives the status of a TLB maintenance operation.
    pub smmu_stlbgstatus: ReadOnly<u32, SmmuStlbgstatus::Register>,
    _padding120: [u8; 8],
    /// Address of TLB entry in a specific TBU.
    pub smmu_dbgrptrtbu: ReadWrite<u32, SmmuDbgrptrtbu::Register>,
    /// TLB entry data addressed by TBU debug read pointer.
    pub smmu_dbgrdatatbu: ReadOnly<u32>,
    /// Address of an entry from a specific cache in TCU.
    pub smmu_dbgrptrtcu: ReadWrite<u32, SmmuDbgrptrtcu::Register>,
    /// Cache entry data addressed by TCU debug read pointer.
    pub smmu_dbgrdatatcu: ReadOnly<u32>,
    _padding144: [u8; 16],
    /// Invalidates all unlocked entries associated with MONC banks, that match the specified virtual address.
    pub smmu_stlbivalm_low: WriteOnly<u32>,
    /// Invalidates all unlocked entries associated with MONC banks, that match the specified virtual address.
    pub smmu_stlbivalm_high: WriteOnly<u32, SmmuStlbivalmHigh::Register>,
    /// Invalidates all unlocked entries associated with MONC banks, that match the specified virtual address.
    pub smmu_stlbivam_low: WriteOnly<u32>,
    /// Invalidates all unlocked entries associated with MONC banks, that match the specified virtual address.
    pub smmu_stlbivam_high: WriteOnly<u32, SmmuStlbivamHigh::Register>,
    _padding176: [u8; 12],
    /// Invalidates all unlocked entries associated with MONC banks in the TLB.
    pub smmu_stlbiallm: WriteOnly<u32>,
    _padding192: [u8; 832],
    /// Provides top-level control of the SMMU.
    pub smmu_nscr0: Aliased<u32, SmmuNscr0R::Register, SmmuNscr0W::Register>,
    _padding1028: [u8; 12],
    /// Provides IMPLEMENTATION DEFINED functionality.
    pub smmu_nsacr: ReadWrite<u32, SmmuNsacr::Register>,
    _padding1044: [u8; 44],
    /// Contains the input address of an erroneous request reported by SMMU_GFSR.
    pub smmu_nsgfar_low: ReadWrite<u32>,
    /// Contains the input address of an erroneous request reported by SMMU_GFSR.
    pub smmu_nsgfar_high: ReadWrite<u32, SmmuNsgfarHigh::Register>,
    /// Gives the fault status for each of the following possible faults.
    pub smmu_nsgfsr: WriteOnly<u32, SmmuNsgfsr::Register>,
    /// Restores the state of SMMU_GFSR, after a reset, for example.
    pub smmu_nsgfsrrestore: WriteOnly<u32, SmmuNsgfsrrestore::Register>,
    /// Contains fault syndrome information relating to SMMU_GFSR.
    pub smmu_nsgfsynr0: Aliased<u32, SmmuNsgfsynr0R::Register, SmmuNsgfsynr0W::Register>,
    /// Contains fault syndrome information relating to SMMU_GFSR.
    pub smmu_nsgfsyndr1: Aliased<u32, SmmuNsgfsyndr1R::Register, SmmuNsgfsyndr1W::Register>,
    _padding1112: [u8; 24],
    /// Starts a global synchronization operation that ensures the completion of any previously accepted TLB Invalidate operation. As a minimum, the operation applies to the specified security state, and includes all TLB Invalidate operations initiated in context banks associated with that security state.
    pub smmu_nstlbgsync: WriteOnly<u32>,
    /// Gives the status of a TLB maintenance operation.
    pub smmu_nstlbgstatus: ReadOnly<u32, SmmuNstlbgstatus::Register>,
    _padding1144: [u8; 904],
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr0: ReadWrite<u32, SmmuSmr0::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr1: ReadWrite<u32, SmmuSmr1::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr2: ReadWrite<u32, SmmuSmr2::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr3: ReadWrite<u32, SmmuSmr3::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr4: ReadWrite<u32, SmmuSmr4::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr5: ReadWrite<u32, SmmuSmr5::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr6: ReadWrite<u32, SmmuSmr6::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr7: ReadWrite<u32, SmmuSmr7::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr8: ReadWrite<u32, SmmuSmr8::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr9: ReadWrite<u32, SmmuSmr9::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr10: ReadWrite<u32, SmmuSmr10::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr11: ReadWrite<u32, SmmuSmr11::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr12: ReadWrite<u32, SmmuSmr12::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr13: ReadWrite<u32, SmmuSmr13::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr14: ReadWrite<u32, SmmuSmr14::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr15: ReadWrite<u32, SmmuSmr15::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr16: ReadWrite<u32, SmmuSmr16::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr17: ReadWrite<u32, SmmuSmr17::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr18: ReadWrite<u32, SmmuSmr18::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr19: ReadWrite<u32, SmmuSmr19::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr20: ReadWrite<u32, SmmuSmr20::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr21: ReadWrite<u32, SmmuSmr21::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr22: ReadWrite<u32, SmmuSmr22::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr23: ReadWrite<u32, SmmuSmr23::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr24: ReadWrite<u32, SmmuSmr24::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr25: ReadWrite<u32, SmmuSmr25::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr26: ReadWrite<u32, SmmuSmr26::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr27: ReadWrite<u32, SmmuSmr27::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr28: ReadWrite<u32, SmmuSmr28::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr29: ReadWrite<u32, SmmuSmr29::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr30: ReadWrite<u32, SmmuSmr30::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr31: ReadWrite<u32, SmmuSmr31::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr32: ReadWrite<u32, SmmuSmr32::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr33: ReadWrite<u32, SmmuSmr33::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr34: ReadWrite<u32, SmmuSmr34::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr35: ReadWrite<u32, SmmuSmr35::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr36: ReadWrite<u32, SmmuSmr36::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr37: ReadWrite<u32, SmmuSmr37::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr38: ReadWrite<u32, SmmuSmr38::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr39: ReadWrite<u32, SmmuSmr39::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr40: ReadWrite<u32, SmmuSmr40::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr41: ReadWrite<u32, SmmuSmr41::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr42: ReadWrite<u32, SmmuSmr42::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr43: ReadWrite<u32, SmmuSmr43::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr44: ReadWrite<u32, SmmuSmr44::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr45: ReadWrite<u32, SmmuSmr45::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr46: ReadWrite<u32, SmmuSmr46::Register>,
    /// Matches a transaction with a particular Stream mapping register group.
    pub smmu_smr47: ReadWrite<u32, SmmuSmr47::Register>,
    _padding2240: [u8; 832],
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr0: ReadWrite<u32, SmmuS2cr0::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr1: ReadWrite<u32, SmmuS2cr1::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr2: ReadWrite<u32, SmmuS2cr2::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr3: ReadWrite<u32, SmmuS2cr3::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr4: ReadWrite<u32, SmmuS2cr4::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr5: ReadWrite<u32, SmmuS2cr5::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr6: ReadWrite<u32, SmmuS2cr6::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr7: ReadWrite<u32, SmmuS2cr7::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr8: ReadWrite<u32, SmmuS2cr8::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr9: ReadWrite<u32, SmmuS2cr9::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr10: ReadWrite<u32, SmmuS2cr10::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr11: ReadWrite<u32, SmmuS2cr11::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr12: ReadWrite<u32, SmmuS2cr12::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr13: ReadWrite<u32, SmmuS2cr13::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr14: ReadWrite<u32, SmmuS2cr14::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr15: ReadWrite<u32, SmmuS2cr15::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr16: ReadWrite<u32, SmmuS2cr16::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr17: ReadWrite<u32, SmmuS2cr17::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr18: ReadWrite<u32, SmmuS2cr18::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr19: ReadWrite<u32, SmmuS2cr19::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr20: ReadWrite<u32, SmmuS2cr20::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr21: ReadWrite<u32, SmmuS2cr21::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr22: ReadWrite<u32, SmmuS2cr22::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr23: ReadWrite<u32, SmmuS2cr23::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr24: ReadWrite<u32, SmmuS2cr24::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr25: ReadWrite<u32, SmmuS2cr25::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr26: ReadWrite<u32, SmmuS2cr26::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr27: ReadWrite<u32, SmmuS2cr27::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr28: ReadWrite<u32, SmmuS2cr28::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr29: ReadWrite<u32, SmmuS2cr29::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr30: ReadWrite<u32, SmmuS2cr30::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr31: ReadWrite<u32, SmmuS2cr31::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr32: ReadWrite<u32, SmmuS2cr32::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr33: ReadWrite<u32, SmmuS2cr33::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr34: ReadWrite<u32, SmmuS2cr34::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr35: ReadWrite<u32, SmmuS2cr35::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr36: ReadWrite<u32, SmmuS2cr36::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr37: ReadWrite<u32, SmmuS2cr37::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr38: ReadWrite<u32, SmmuS2cr38::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr39: ReadWrite<u32, SmmuS2cr39::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr40: ReadWrite<u32, SmmuS2cr40::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr41: ReadWrite<u32, SmmuS2cr41::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr42: ReadWrite<u32, SmmuS2cr42::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr43: ReadWrite<u32, SmmuS2cr43::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr44: ReadWrite<u32, SmmuS2cr44::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr45: ReadWrite<u32, SmmuS2cr45::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr46: ReadWrite<u32, SmmuS2cr46::Register>,
    /// Specifies an initial context for processing a transaction, where the transaction matches the Stream mapping group that this register belongs to.
    pub smmu_s2cr47: ReadWrite<u32, SmmuS2cr47::Register>,
    _padding3264: [u8; 784],
    /// Peripheral Identificaation register 4
    pub smmu_pidr4: ReadOnly<u32, SmmuPidr4::Register>,
    /// Peripheral Identificaation register 5
    pub smmu_pidr5: ReadOnly<u32>,
    /// Peripheral Identificaation register 6
    pub smmu_pidr6: ReadOnly<u32>,
    /// Peripheral Identificaation register 7
    pub smmu_pidr7: ReadOnly<u32>,
    /// Peripheral Identificaation register 0
    pub smmu_pidr0: ReadOnly<u32, SmmuPidr0::Register>,
    /// Peripheral Identificaation register 1
    pub smmu_pidr1: ReadOnly<u32, SmmuPidr1::Register>,
    /// Peripheral Identificaation register 2
    pub smmu_pidr2: ReadOnly<u32, SmmuPidr2::Register>,
    /// Peripheral Identificaation register 3
    pub smmu_pidr3: ReadOnly<u32, SmmuPidr3::Register>,
    /// Component Identification register 0
    pub smmu_cidr0: ReadOnly<u32, SmmuCidr0::Register>,
    /// Component Identification register 1
    pub smmu_cidr1: ReadOnly<u32, SmmuCidr1::Register>,
    /// Component Identification register 2
    pub smmu_cidr2: ReadOnly<u32, SmmuCidr2::Register>,
    /// Component Identification register 3
    pub smmu_cidr3: ReadOnly<u32, SmmuCidr3::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar0: Aliased<u32, SmmuCbar0R::Register, SmmuCbar0W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar1: Aliased<u32, SmmuCbar1R::Register, SmmuCbar1W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar2: Aliased<u32, SmmuCbar2R::Register, SmmuCbar2W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar3: Aliased<u32, SmmuCbar3R::Register, SmmuCbar3W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar4: Aliased<u32, SmmuCbar4R::Register, SmmuCbar4W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar5: Aliased<u32, SmmuCbar5R::Register, SmmuCbar5W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar6: Aliased<u32, SmmuCbar6R::Register, SmmuCbar6W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar7: Aliased<u32, SmmuCbar7R::Register, SmmuCbar7W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar8: Aliased<u32, SmmuCbar8R::Register, SmmuCbar8W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar9: Aliased<u32, SmmuCbar9R::Register, SmmuCbar9W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar10: Aliased<u32, SmmuCbar10R::Register, SmmuCbar10W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar11: Aliased<u32, SmmuCbar11R::Register, SmmuCbar11W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar12: Aliased<u32, SmmuCbar12R::Register, SmmuCbar12W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar13: Aliased<u32, SmmuCbar13R::Register, SmmuCbar13W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar14: Aliased<u32, SmmuCbar14R::Register, SmmuCbar14W::Register>,
    /// Specifies configuration attributes for translation context bank.
    pub smmu_cbar15: Aliased<u32, SmmuCbar15R::Register, SmmuCbar15W::Register>,
    _padding4160: [u8; 960],
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra0: Aliased<u32, SmmuCbfrsynra0R::Register, SmmuCbfrsynra0W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra1: Aliased<u32, SmmuCbfrsynra1R::Register, SmmuCbfrsynra1W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra2: Aliased<u32, SmmuCbfrsynra2R::Register, SmmuCbfrsynra2W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra3: Aliased<u32, SmmuCbfrsynra3R::Register, SmmuCbfrsynra3W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra4: Aliased<u32, SmmuCbfrsynra4R::Register, SmmuCbfrsynra4W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra5: Aliased<u32, SmmuCbfrsynra5R::Register, SmmuCbfrsynra5W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra6: Aliased<u32, SmmuCbfrsynra6R::Register, SmmuCbfrsynra6W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra7: Aliased<u32, SmmuCbfrsynra7R::Register, SmmuCbfrsynra7W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra8: Aliased<u32, SmmuCbfrsynra8R::Register, SmmuCbfrsynra8W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra9: Aliased<u32, SmmuCbfrsynra9R::Register, SmmuCbfrsynra9W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra10: Aliased<u32, SmmuCbfrsynra10R::Register, SmmuCbfrsynra10W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra11: Aliased<u32, SmmuCbfrsynra11R::Register, SmmuCbfrsynra11W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra12: Aliased<u32, SmmuCbfrsynra12R::Register, SmmuCbfrsynra12W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra13: Aliased<u32, SmmuCbfrsynra13R::Register, SmmuCbfrsynra13W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra14: Aliased<u32, SmmuCbfrsynra14R::Register, SmmuCbfrsynra14W::Register>,
    /// Gives fault syndrome information about the access that caused an exception in the associated translation context bank.
    pub smmu_cbfrsynra15: Aliased<u32, SmmuCbfrsynra15R::Register, SmmuCbfrsynra15W::Register>,
    _padding5184: [u8; 960],
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r0: ReadWrite<u32, SmmuCba2r0::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r1: ReadWrite<u32, SmmuCba2r1::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r2: ReadWrite<u32, SmmuCba2r2::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r3: ReadWrite<u32, SmmuCba2r3::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r4: ReadWrite<u32, SmmuCba2r4::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r5: ReadWrite<u32, SmmuCba2r5::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r6: ReadWrite<u32, SmmuCba2r6::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r7: ReadWrite<u32, SmmuCba2r7::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r8: ReadWrite<u32, SmmuCba2r8::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r9: ReadWrite<u32, SmmuCba2r9::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r10: ReadWrite<u32, SmmuCba2r10::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r11: ReadWrite<u32, SmmuCba2r11::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r12: ReadWrite<u32, SmmuCba2r12::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r13: ReadWrite<u32, SmmuCba2r13::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r14: ReadWrite<u32, SmmuCba2r14::Register>,
    /// Extends the configuration attributes for the translation context bank that SMMU_CBARn specifies.
    pub smmu_cba2r15: ReadWrite<u32, SmmuCba2r15::Register>,
    _padding6208: [u8; 1984],
    /// This register enables the component to switch from functional mode to integration mode. You can directly control the inputs and outputs in integration mode.
    pub smmu_itctrl: ReadWrite<u32, SmmuItctrl::Register>,
    /// Enables the MMU-500 to read the status of the spniden signal.
    pub smmu_itip: ReadOnly<u32, SmmuItip::Register>,
    /// For integration test purposes, allows to enable or disable secure and nonsecure interrupts and write or read most significant bits of TCU MTLB and IPA RAMS.
    pub smmu_itop_glbl: Aliased<u32, SmmuItopGlblR::Register, SmmuItopGlblW::Register>,
    /// Enables TBU performance interrupts.
    pub smmu_itop_perf_index: WriteOnly<u32, SmmuItopPerfIndex::Register>,
    /// Enable the context performance interrupts.
    pub smmu_itop_cxt0to31_ram0: WriteOnly<u32>,
    _padding8212: [u8; 236],
    /// Specify the QoS for TBUs,when the TBUn is in the range of 0-7.
    pub smmu_tbuqos0: ReadWrite<u32, SmmuTbuqos0::Register>,
    _padding8452: [u8; 252],
    /// Checks for parity errors in TCU and TBU RAMs.
    pub smmu_per: ReadOnly<u32, SmmuPer::Register>,
    /// Provides the power status of TBUs.
    pub smmu_tbu_pwr_status: ReadOnly<u32>,
    _padding8712: [u8; 3576],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr3: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr4: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr5: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr6: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr7: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr8: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr9: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr10: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr11: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr12: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr13: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr14: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr15: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr16: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr17: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr18: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr19: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr20: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr21: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr22: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub pmevcntr23: ReadWrite<u32>,
    _padding12384: [u8; 928],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper0: ReadWrite<u32, Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper1: ReadWrite<u32, Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper2: ReadWrite<u32, Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper3: ReadWrite<u32, Pmevtyper3::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper4: ReadWrite<u32, Pmevtyper4::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper5: ReadWrite<u32, Pmevtyper5::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper6: ReadWrite<u32, Pmevtyper6::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper7: ReadWrite<u32, Pmevtyper7::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper8: ReadWrite<u32, Pmevtyper8::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper9: ReadWrite<u32, Pmevtyper9::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper10: ReadWrite<u32, Pmevtyper10::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper11: ReadWrite<u32, Pmevtyper11::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper12: ReadWrite<u32, Pmevtyper12::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper13: ReadWrite<u32, Pmevtyper13::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper14: ReadWrite<u32, Pmevtyper14::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper15: ReadWrite<u32, Pmevtyper15::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper16: ReadWrite<u32, Pmevtyper16::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper17: ReadWrite<u32, Pmevtyper17::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper18: ReadWrite<u32, Pmevtyper18::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper19: ReadWrite<u32, Pmevtyper19::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper20: ReadWrite<u32, Pmevtyper20::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper21: ReadWrite<u32, Pmevtyper21::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper22: ReadWrite<u32, Pmevtyper22::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub pmevtyper23: ReadWrite<u32, Pmevtyper23::Register>,
    _padding13408: [u8; 928],
    /// Controls Counter group behavior.
    pub pmcgcr0: Aliased<u32, Pmcgcr0R::Register, Pmcgcr0W::Register>,
    /// Controls Counter group behavior.
    pub pmcgcr1: Aliased<u32, Pmcgcr1R::Register, Pmcgcr1W::Register>,
    /// Controls Counter group behavior.
    pub pmcgcr2: Aliased<u32, Pmcgcr2R::Register, Pmcgcr2W::Register>,
    /// Controls Counter group behavior.
    pub pmcgcr3: Aliased<u32, Pmcgcr3R::Register, Pmcgcr3W::Register>,
    /// Controls Counter group behavior.
    pub pmcgcr4: Aliased<u32, Pmcgcr4R::Register, Pmcgcr4W::Register>,
    /// Controls Counter group behavior.
    pub pmcgcr5: Aliased<u32, Pmcgcr5R::Register, Pmcgcr5W::Register>,
    _padding14360: [u8; 488],
    /// Specifies StreamID filtering of the events counted in a Counter group
    pub pmcgsmr0: ReadWrite<u32, Pmcgsmr0::Register>,
    /// Specifies StreamID filtering of the events counted in a Counter group
    pub pmcgsmr1: ReadWrite<u32, Pmcgsmr1::Register>,
    /// Specifies StreamID filtering of the events counted in a Counter group
    pub pmcgsmr2: ReadWrite<u32, Pmcgsmr2::Register>,
    /// Specifies StreamID filtering of the events counted in a Counter group
    pub pmcgsmr3: ReadWrite<u32, Pmcgsmr3::Register>,
    /// Specifies StreamID filtering of the events counted in a Counter group
    pub pmcgsmr4: ReadWrite<u32, Pmcgsmr4::Register>,
    /// Specifies StreamID filtering of the events counted in a Counter group
    pub pmcgsmr5: ReadWrite<u32, Pmcgsmr5::Register>,
    _padding14872: [u8; 488],
    /// Performance Monitor Counter Enable Set registers are used to enable the event counters PMEVCNTRxx.
    pub pmcntenset: WriteOnly<u32, Pmcntenset::Register>,
    _padding15364: [u8; 28],
    /// Performance Monitor Counter Enable Clear registers are used to disable the event counters PMEVCNTRxx.
    pub pmcntenclr: WriteOnly<u32, Pmcntenclr::Register>,
    _padding15396: [u8; 28],
    /// Performance Monitor Interrupt Enable Set registers are used enable the generation of interrupts on overflows of the event counters.
    pub pmintenset: WriteOnly<u32, Pmintenset::Register>,
    _padding15428: [u8; 28],
    /// Performance Monitor Interrupt Enable Clear registers are used disable the generation of interrupts on overflows of the event counters.
    pub pmintenclr: WriteOnly<u32, Pmintenclr::Register>,
    _padding15460: [u8; 28],
    /// Performance Monitor Overflow Status Clear registers are used to clear the overflow status of the event registers.
    pub pmovsclr: WriteOnly<u32, Pmovsclr::Register>,
    _padding15492: [u8; 60],
    /// Performance Monitor Overflow Status Set registers contain overflow status for the event counters.
    pub pmovsset: WriteOnly<u32, Pmovsset::Register>,
    _padding15556: [u8; 316],
    /// Performance Monitor Configuration register containss PMU specific configuration data.
    pub pmcfgr: ReadOnly<u32, Pmcfgr::Register>,
    /// Performance Monitor Configuration register controls the behaviour of the event counters.
    pub pmcr: Aliased<u32, PmcrR::Register, PmcrW::Register>,
    _padding15880: [u8; 24],
    /// Performance Monitor Common Event Identification register 0 describes the event classes supported by the SMMU implementation.
    pub pmceid0: ReadOnly<u32, Pmceid0::Register>,
    _padding15908: [u8; 404],
    /// Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions.
    pub pmauthstatus: ReadOnly<u32, Pmauthstatus::Register>,
    _padding16316: [u8; 16],
    /// Performance Monitor Device Type register provides the Coresight device type information for the PerformanceMonitors.
    pub pmdevtype: ReadOnly<u32, Pmdevtype::Register>,
    _padding16336: [u8; 49200],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb0_sctlr: Aliased<u32, SmmuCb0SctlrR::Register, SmmuCb0SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb0_actlr: ReadWrite<u32, SmmuCb0Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb0_resume: WriteOnly<u32, SmmuCb0Resume::Register>,
    _padding65548: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb0_tcr2: Aliased<u32, SmmuCb0Tcr2R::Register, SmmuCb0Tcr2W::Register>,
    _padding65556: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb0_ttbr0_low: Aliased<u32, SmmuCb0Ttbr0LowR::Register, SmmuCb0Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb0_ttbr0_high: ReadWrite<u32, SmmuCb0Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb0_ttbr1_low: ReadWrite<u32, SmmuCb0Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb0_ttbr1_high: ReadWrite<u32, SmmuCb0Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb0_tcr_lpae: ReadWrite<u32, SmmuCb0TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb0_contextidr: ReadWrite<u32, SmmuCb0Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb0_prrr_mair0: ReadWrite<u32, SmmuCb0PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb0_nmrr_mair1: ReadWrite<u32, SmmuCb0NmrrMair1::Register>,
    _padding65600: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb0_fsr: WriteOnly<u32, SmmuCb0Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb0_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb0_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb0_far_high: ReadWrite<u32, SmmuCb0FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb0_fsynr0: Aliased<u32, SmmuCb0Fsynr0R::Register, SmmuCb0Fsynr0W::Register>,
    _padding65644: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb0_ipafar_low: Aliased<u32, SmmuCb0IpafarLowR::Register, SmmuCb0IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb0_ipafar_high: ReadWrite<u32, SmmuCb0IpafarHigh::Register>,
    _padding65656: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb0_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb0_tlbiva_high: WriteOnly<u32, SmmuCb0TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb0_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb0_tlbivaa_high: WriteOnly<u32, SmmuCb0TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb0_tlbiasid: WriteOnly<u32, SmmuCb0Tlbiasid::Register>,
    _padding67092: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb0_tlbiall: WriteOnly<u32>,
    _padding67100: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb0_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb0_tlbival_high: WriteOnly<u32, SmmuCb0TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb0_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb0_tlbivaal_high: WriteOnly<u32, SmmuCb0TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb0_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb0_tlbiipas2_high: WriteOnly<u32, SmmuCb0Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb0_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb0_tlbiipas2l_high: WriteOnly<u32, SmmuCb0Tlbiipas2lHigh::Register>,
    _padding67136: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb0_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb0_tlbstatus: ReadOnly<u32, SmmuCb0Tlbstatus::Register>,
    _padding67576: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb0_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb0_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb0_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb0_pmevcntr3: ReadWrite<u32>,
    _padding69136: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb0_pmevtyper0: ReadWrite<u32, SmmuCb0Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb0_pmevtyper1: ReadWrite<u32, SmmuCb0Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb0_pmevtyper2: ReadWrite<u32, SmmuCb0Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb0_pmevtyper3: ReadWrite<u32, SmmuCb0Pmevtyper3::Register>,
    _padding69264: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb0_pmcfgr: ReadOnly<u32, SmmuCb0Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb0_pmcr: Aliased<u32, SmmuCb0PmcrR::Register, SmmuCb0PmcrW::Register>,
    _padding69384: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb0_pmceid: ReadOnly<u32, SmmuCb0Pmceid::Register>,
    _padding69412: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb0_pmcntense: WriteOnly<u32, SmmuCb0Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb0_pmcntenclr: WriteOnly<u32, SmmuCb0Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb0_pmcntenset: WriteOnly<u32, SmmuCb0Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb0_pmintenclr: WriteOnly<u32, SmmuCb0Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb0_pmovsclr: WriteOnly<u32, SmmuCb0Pmovsclr::Register>,
    _padding69460: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb0_pmovsset: WriteOnly<u32, SmmuCb0Pmovsset::Register>,
    _padding69468: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb0_pmauthstatus: ReadOnly<u32, SmmuCb0Pmauthstatus::Register>,
    _padding69564: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb1_sctlr: Aliased<u32, SmmuCb1SctlrR::Register, SmmuCb1SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb1_actlr: ReadWrite<u32, SmmuCb1Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb1_resume: WriteOnly<u32, SmmuCb1Resume::Register>,
    _padding69644: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb1_tcr2: Aliased<u32, SmmuCb1Tcr2R::Register, SmmuCb1Tcr2W::Register>,
    _padding69652: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb1_ttbr0_low: Aliased<u32, SmmuCb1Ttbr0LowR::Register, SmmuCb1Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb1_ttbr0_high: ReadWrite<u32, SmmuCb1Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb1_ttbr1_low: ReadWrite<u32, SmmuCb1Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb1_ttbr1_high: ReadWrite<u32, SmmuCb1Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb1_tcr_lpae: ReadWrite<u32, SmmuCb1TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb1_contextidr: ReadWrite<u32, SmmuCb1Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb1_prrr_mair0: ReadWrite<u32, SmmuCb1PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb1_nmrr_mair1: ReadWrite<u32, SmmuCb1NmrrMair1::Register>,
    _padding69696: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb1_fsr: WriteOnly<u32, SmmuCb1Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb1_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb1_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb1_far_high: ReadWrite<u32, SmmuCb1FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb1_fsynr0: Aliased<u32, SmmuCb1Fsynr0R::Register, SmmuCb1Fsynr0W::Register>,
    _padding69740: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb1_ipafar_low: Aliased<u32, SmmuCb1IpafarLowR::Register, SmmuCb1IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb1_ipafar_high: ReadWrite<u32, SmmuCb1IpafarHigh::Register>,
    _padding69752: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb1_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb1_tlbiva_high: WriteOnly<u32, SmmuCb1TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb1_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb1_tlbivaa_high: WriteOnly<u32, SmmuCb1TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb1_tlbiasid: WriteOnly<u32, SmmuCb1Tlbiasid::Register>,
    _padding71188: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb1_tlbiall: WriteOnly<u32>,
    _padding71196: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb1_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb1_tlbival_high: WriteOnly<u32, SmmuCb1TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb1_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb1_tlbivaal_high: WriteOnly<u32, SmmuCb1TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb1_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb1_tlbiipas2_high: WriteOnly<u32, SmmuCb1Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb1_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb1_tlbiipas2l_high: WriteOnly<u32, SmmuCb1Tlbiipas2lHigh::Register>,
    _padding71232: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb1_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb1_tlbstatus: ReadOnly<u32, SmmuCb1Tlbstatus::Register>,
    _padding71672: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb1_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb1_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb1_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb1_pmevcntr3: ReadWrite<u32>,
    _padding73232: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb1_pmevtyper0: ReadWrite<u32, SmmuCb1Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb1_pmevtyper1: ReadWrite<u32, SmmuCb1Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb1_pmevtyper2: ReadWrite<u32, SmmuCb1Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb1_pmevtyper3: ReadWrite<u32, SmmuCb1Pmevtyper3::Register>,
    _padding73360: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb1_pmcfgr: ReadOnly<u32, SmmuCb1Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb1_pmcr: Aliased<u32, SmmuCb1PmcrR::Register, SmmuCb1PmcrW::Register>,
    _padding73480: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb1_pmceid: ReadOnly<u32, SmmuCb1Pmceid::Register>,
    _padding73508: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb1_pmcntense: WriteOnly<u32, SmmuCb1Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb1_pmcntenclr: WriteOnly<u32, SmmuCb1Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb1_pmcntenset: WriteOnly<u32, SmmuCb1Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb1_pmintenclr: WriteOnly<u32, SmmuCb1Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb1_pmovsclr: WriteOnly<u32, SmmuCb1Pmovsclr::Register>,
    _padding73556: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb1_pmovsset: WriteOnly<u32, SmmuCb1Pmovsset::Register>,
    _padding73564: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb1_pmauthstatus: ReadOnly<u32, SmmuCb1Pmauthstatus::Register>,
    _padding73660: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb2_sctlr: Aliased<u32, SmmuCb2SctlrR::Register, SmmuCb2SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb2_actlr: ReadWrite<u32, SmmuCb2Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb2_resume: WriteOnly<u32, SmmuCb2Resume::Register>,
    _padding73740: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb2_tcr2: Aliased<u32, SmmuCb2Tcr2R::Register, SmmuCb2Tcr2W::Register>,
    _padding73748: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb2_ttbr0_low: Aliased<u32, SmmuCb2Ttbr0LowR::Register, SmmuCb2Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb2_ttbr0_high: ReadWrite<u32, SmmuCb2Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb2_ttbr1_low: ReadWrite<u32, SmmuCb2Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb2_ttbr1_high: ReadWrite<u32, SmmuCb2Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb2_tcr_lpae: ReadWrite<u32, SmmuCb2TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb2_contextidr: ReadWrite<u32, SmmuCb2Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb2_prrr_mair0: ReadWrite<u32, SmmuCb2PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb2_nmrr_mair1: ReadWrite<u32, SmmuCb2NmrrMair1::Register>,
    _padding73792: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb2_fsr: WriteOnly<u32, SmmuCb2Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb2_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb2_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb2_far_high: ReadWrite<u32, SmmuCb2FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb2_fsynr0: Aliased<u32, SmmuCb2Fsynr0R::Register, SmmuCb2Fsynr0W::Register>,
    _padding73836: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb2_ipafar_low: Aliased<u32, SmmuCb2IpafarLowR::Register, SmmuCb2IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb2_ipafar_high: ReadWrite<u32, SmmuCb2IpafarHigh::Register>,
    _padding73848: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb2_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb2_tlbiva_high: WriteOnly<u32, SmmuCb2TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb2_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb2_tlbivaa_high: WriteOnly<u32, SmmuCb2TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb2_tlbiasid: WriteOnly<u32, SmmuCb2Tlbiasid::Register>,
    _padding75284: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb2_tlbiall: WriteOnly<u32>,
    _padding75292: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb2_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb2_tlbival_high: WriteOnly<u32, SmmuCb2TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb2_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb2_tlbivaal_high: WriteOnly<u32, SmmuCb2TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb2_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb2_tlbiipas2_high: WriteOnly<u32, SmmuCb2Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb2_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb2_tlbiipas2l_high: WriteOnly<u32, SmmuCb2Tlbiipas2lHigh::Register>,
    _padding75328: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb2_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb2_tlbstatus: ReadOnly<u32, SmmuCb2Tlbstatus::Register>,
    _padding75768: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb2_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb2_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb2_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb2_pmevcntr3: ReadWrite<u32>,
    _padding77328: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb2_pmevtyper0: ReadWrite<u32, SmmuCb2Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb2_pmevtyper1: ReadWrite<u32, SmmuCb2Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb2_pmevtyper2: ReadWrite<u32, SmmuCb2Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb2_pmevtyper3: ReadWrite<u32, SmmuCb2Pmevtyper3::Register>,
    _padding77456: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb2_pmcfgr: ReadOnly<u32, SmmuCb2Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb2_pmcr: Aliased<u32, SmmuCb2PmcrR::Register, SmmuCb2PmcrW::Register>,
    _padding77576: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb2_pmceid: ReadOnly<u32, SmmuCb2Pmceid::Register>,
    _padding77604: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb2_pmcntense: WriteOnly<u32, SmmuCb2Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb2_pmcntenclr: WriteOnly<u32, SmmuCb2Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb2_pmcntenset: WriteOnly<u32, SmmuCb2Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb2_pmintenclr: WriteOnly<u32, SmmuCb2Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb2_pmovsclr: WriteOnly<u32, SmmuCb2Pmovsclr::Register>,
    _padding77652: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb2_pmovsset: WriteOnly<u32, SmmuCb2Pmovsset::Register>,
    _padding77660: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb2_pmauthstatus: ReadOnly<u32, SmmuCb2Pmauthstatus::Register>,
    _padding77756: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb3_sctlr: Aliased<u32, SmmuCb3SctlrR::Register, SmmuCb3SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb3_actlr: ReadWrite<u32, SmmuCb3Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb3_resume: WriteOnly<u32, SmmuCb3Resume::Register>,
    _padding77836: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb3_tcr2: Aliased<u32, SmmuCb3Tcr2R::Register, SmmuCb3Tcr2W::Register>,
    _padding77844: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb3_ttbr0_low: Aliased<u32, SmmuCb3Ttbr0LowR::Register, SmmuCb3Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb3_ttbr0_high: ReadWrite<u32, SmmuCb3Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb3_ttbr1_low: ReadWrite<u32, SmmuCb3Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb3_ttbr1_high: ReadWrite<u32, SmmuCb3Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb3_tcr_lpae: ReadWrite<u32, SmmuCb3TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb3_contextidr: ReadWrite<u32, SmmuCb3Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb3_prrr_mair0: ReadWrite<u32, SmmuCb3PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb3_nmrr_mair1: ReadWrite<u32, SmmuCb3NmrrMair1::Register>,
    _padding77888: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb3_fsr: WriteOnly<u32, SmmuCb3Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb3_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb3_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb3_far_high: ReadWrite<u32, SmmuCb3FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb3_fsynr0: Aliased<u32, SmmuCb3Fsynr0R::Register, SmmuCb3Fsynr0W::Register>,
    _padding77932: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb3_ipafar_low: Aliased<u32, SmmuCb3IpafarLowR::Register, SmmuCb3IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb3_ipafar_high: ReadWrite<u32, SmmuCb3IpafarHigh::Register>,
    _padding77944: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb3_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb3_tlbiva_high: WriteOnly<u32, SmmuCb3TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb3_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb3_tlbivaa_high: WriteOnly<u32, SmmuCb3TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb3_tlbiasid: WriteOnly<u32, SmmuCb3Tlbiasid::Register>,
    _padding79380: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb3_tlbiall: WriteOnly<u32>,
    _padding79388: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb3_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb3_tlbival_high: WriteOnly<u32, SmmuCb3TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb3_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb3_tlbivaal_high: WriteOnly<u32, SmmuCb3TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb3_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb3_tlbiipas2_high: WriteOnly<u32, SmmuCb3Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb3_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb3_tlbiipas2l_high: WriteOnly<u32, SmmuCb3Tlbiipas2lHigh::Register>,
    _padding79424: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb3_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb3_tlbstatus: ReadOnly<u32, SmmuCb3Tlbstatus::Register>,
    _padding79864: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb3_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb3_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb3_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb3_pmevcntr3: ReadWrite<u32>,
    _padding81424: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb3_pmevtyper0: ReadWrite<u32, SmmuCb3Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb3_pmevtyper1: ReadWrite<u32, SmmuCb3Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb3_pmevtyper2: ReadWrite<u32, SmmuCb3Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb3_pmevtyper3: ReadWrite<u32, SmmuCb3Pmevtyper3::Register>,
    _padding81552: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb3_pmcfgr: ReadOnly<u32, SmmuCb3Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb3_pmcr: Aliased<u32, SmmuCb3PmcrR::Register, SmmuCb3PmcrW::Register>,
    _padding81672: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb3_pmceid: ReadOnly<u32, SmmuCb3Pmceid::Register>,
    _padding81700: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb3_pmcntense: WriteOnly<u32, SmmuCb3Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb3_pmcntenclr: WriteOnly<u32, SmmuCb3Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb3_pmcntenset: WriteOnly<u32, SmmuCb3Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb3_pmintenclr: WriteOnly<u32, SmmuCb3Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb3_pmovsclr: WriteOnly<u32, SmmuCb3Pmovsclr::Register>,
    _padding81748: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb3_pmovsset: WriteOnly<u32, SmmuCb3Pmovsset::Register>,
    _padding81756: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb3_pmauthstatus: ReadOnly<u32, SmmuCb3Pmauthstatus::Register>,
    _padding81852: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb4_sctlr: Aliased<u32, SmmuCb4SctlrR::Register, SmmuCb4SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb4_actlr: ReadWrite<u32, SmmuCb4Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb4_resume: WriteOnly<u32, SmmuCb4Resume::Register>,
    _padding81932: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb4_tcr2: Aliased<u32, SmmuCb4Tcr2R::Register, SmmuCb4Tcr2W::Register>,
    _padding81940: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb4_ttbr0_low: Aliased<u32, SmmuCb4Ttbr0LowR::Register, SmmuCb4Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb4_ttbr0_high: ReadWrite<u32, SmmuCb4Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb4_ttbr1_low: ReadWrite<u32, SmmuCb4Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb4_ttbr1_high: ReadWrite<u32, SmmuCb4Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb4_tcr_lpae: ReadWrite<u32, SmmuCb4TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb4_contextidr: ReadWrite<u32, SmmuCb4Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb4_prrr_mair0: ReadWrite<u32, SmmuCb4PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb4_nmrr_mair1: ReadWrite<u32, SmmuCb4NmrrMair1::Register>,
    _padding81984: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb4_fsr: WriteOnly<u32, SmmuCb4Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb4_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb4_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb4_far_high: ReadWrite<u32, SmmuCb4FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb4_fsynr0: Aliased<u32, SmmuCb4Fsynr0R::Register, SmmuCb4Fsynr0W::Register>,
    _padding82028: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb4_ipafar_low: Aliased<u32, SmmuCb4IpafarLowR::Register, SmmuCb4IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb4_ipafar_high: ReadWrite<u32, SmmuCb4IpafarHigh::Register>,
    _padding82040: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb4_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb4_tlbiva_high: WriteOnly<u32, SmmuCb4TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb4_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb4_tlbivaa_high: WriteOnly<u32, SmmuCb4TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb4_tlbiasid: WriteOnly<u32, SmmuCb4Tlbiasid::Register>,
    _padding83476: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb4_tlbiall: WriteOnly<u32>,
    _padding83484: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb4_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb4_tlbival_high: WriteOnly<u32, SmmuCb4TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb4_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb4_tlbivaal_high: WriteOnly<u32, SmmuCb4TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb4_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb4_tlbiipas2_high: WriteOnly<u32, SmmuCb4Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb4_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb4_tlbiipas2l_high: WriteOnly<u32, SmmuCb4Tlbiipas2lHigh::Register>,
    _padding83520: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb4_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb4_tlbstatus: ReadOnly<u32, SmmuCb4Tlbstatus::Register>,
    _padding83960: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb4_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb4_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb4_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb4_pmevcntr3: ReadWrite<u32>,
    _padding85520: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb4_pmevtyper0: ReadWrite<u32, SmmuCb4Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb4_pmevtyper1: ReadWrite<u32, SmmuCb4Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb4_pmevtyper2: ReadWrite<u32, SmmuCb4Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb4_pmevtyper3: ReadWrite<u32, SmmuCb4Pmevtyper3::Register>,
    _padding85648: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb4_pmcfgr: ReadOnly<u32, SmmuCb4Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb4_pmcr: Aliased<u32, SmmuCb4PmcrR::Register, SmmuCb4PmcrW::Register>,
    _padding85768: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb4_pmceid: ReadOnly<u32, SmmuCb4Pmceid::Register>,
    _padding85796: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb4_pmcntense: WriteOnly<u32, SmmuCb4Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb4_pmcntenclr: WriteOnly<u32, SmmuCb4Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb4_pmcntenset: WriteOnly<u32, SmmuCb4Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb4_pmintenclr: WriteOnly<u32, SmmuCb4Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb4_pmovsclr: WriteOnly<u32, SmmuCb4Pmovsclr::Register>,
    _padding85844: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb4_pmovsset: WriteOnly<u32, SmmuCb4Pmovsset::Register>,
    _padding85852: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb4_pmauthstatus: ReadOnly<u32, SmmuCb4Pmauthstatus::Register>,
    _padding85948: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb5_sctlr: Aliased<u32, SmmuCb5SctlrR::Register, SmmuCb5SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb5_actlr: ReadWrite<u32, SmmuCb5Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb5_resume: WriteOnly<u32, SmmuCb5Resume::Register>,
    _padding86028: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb5_tcr2: Aliased<u32, SmmuCb5Tcr2R::Register, SmmuCb5Tcr2W::Register>,
    _padding86036: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb5_ttbr0_low: Aliased<u32, SmmuCb5Ttbr0LowR::Register, SmmuCb5Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb5_ttbr0_high: ReadWrite<u32, SmmuCb5Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb5_ttbr1_low: ReadWrite<u32, SmmuCb5Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb5_ttbr1_high: ReadWrite<u32, SmmuCb5Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb5_tcr_lpae: ReadWrite<u32, SmmuCb5TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb5_contextidr: ReadWrite<u32, SmmuCb5Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb5_prrr_mair0: ReadWrite<u32, SmmuCb5PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb5_nmrr_mair1: ReadWrite<u32, SmmuCb5NmrrMair1::Register>,
    _padding86080: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb5_fsr: WriteOnly<u32, SmmuCb5Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb5_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb5_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb5_far_high: ReadWrite<u32, SmmuCb5FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb5_fsynr0: Aliased<u32, SmmuCb5Fsynr0R::Register, SmmuCb5Fsynr0W::Register>,
    _padding86124: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb5_ipafar_low: Aliased<u32, SmmuCb5IpafarLowR::Register, SmmuCb5IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb5_ipafar_high: ReadWrite<u32, SmmuCb5IpafarHigh::Register>,
    _padding86136: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb5_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb5_tlbiva_high: WriteOnly<u32, SmmuCb5TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb5_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb5_tlbivaa_high: WriteOnly<u32, SmmuCb5TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb5_tlbiasid: WriteOnly<u32, SmmuCb5Tlbiasid::Register>,
    _padding87572: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb5_tlbiall: WriteOnly<u32>,
    _padding87580: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb5_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb5_tlbival_high: WriteOnly<u32, SmmuCb5TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb5_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb5_tlbivaal_high: WriteOnly<u32, SmmuCb5TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb5_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb5_tlbiipas2_high: WriteOnly<u32, SmmuCb5Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb5_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb5_tlbiipas2l_high: WriteOnly<u32, SmmuCb5Tlbiipas2lHigh::Register>,
    _padding87616: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb5_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb5_tlbstatus: ReadOnly<u32, SmmuCb5Tlbstatus::Register>,
    _padding88056: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb5_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb5_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb5_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb5_pmevcntr3: ReadWrite<u32>,
    _padding89616: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb5_pmevtyper0: ReadWrite<u32, SmmuCb5Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb5_pmevtyper1: ReadWrite<u32, SmmuCb5Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb5_pmevtyper2: ReadWrite<u32, SmmuCb5Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb5_pmevtyper3: ReadWrite<u32, SmmuCb5Pmevtyper3::Register>,
    _padding89744: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb5_pmcfgr: ReadOnly<u32, SmmuCb5Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb5_pmcr: Aliased<u32, SmmuCb5PmcrR::Register, SmmuCb5PmcrW::Register>,
    _padding89864: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb5_pmceid: ReadOnly<u32, SmmuCb5Pmceid::Register>,
    _padding89892: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb5_pmcntense: WriteOnly<u32, SmmuCb5Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb5_pmcntenclr: WriteOnly<u32, SmmuCb5Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb5_pmcntenset: WriteOnly<u32, SmmuCb5Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb5_pmintenclr: WriteOnly<u32, SmmuCb5Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb5_pmovsclr: WriteOnly<u32, SmmuCb5Pmovsclr::Register>,
    _padding89940: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb5_pmovsset: WriteOnly<u32, SmmuCb5Pmovsset::Register>,
    _padding89948: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb5_pmauthstatus: ReadOnly<u32, SmmuCb5Pmauthstatus::Register>,
    _padding90044: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb6_sctlr: Aliased<u32, SmmuCb6SctlrR::Register, SmmuCb6SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb6_actlr: ReadWrite<u32, SmmuCb6Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb6_resume: WriteOnly<u32, SmmuCb6Resume::Register>,
    _padding90124: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb6_tcr2: Aliased<u32, SmmuCb6Tcr2R::Register, SmmuCb6Tcr2W::Register>,
    _padding90132: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb6_ttbr0_low: Aliased<u32, SmmuCb6Ttbr0LowR::Register, SmmuCb6Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb6_ttbr0_high: ReadWrite<u32, SmmuCb6Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb6_ttbr1_low: ReadWrite<u32, SmmuCb6Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb6_ttbr1_high: ReadWrite<u32, SmmuCb6Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb6_tcr_lpae: ReadWrite<u32, SmmuCb6TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb6_contextidr: ReadWrite<u32, SmmuCb6Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb6_prrr_mair0: ReadWrite<u32, SmmuCb6PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb6_nmrr_mair1: ReadWrite<u32, SmmuCb6NmrrMair1::Register>,
    _padding90176: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb6_fsr: WriteOnly<u32, SmmuCb6Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb6_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb6_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb6_far_high: ReadWrite<u32, SmmuCb6FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb6_fsynr0: Aliased<u32, SmmuCb6Fsynr0R::Register, SmmuCb6Fsynr0W::Register>,
    _padding90220: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb6_ipafar_low: Aliased<u32, SmmuCb6IpafarLowR::Register, SmmuCb6IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb6_ipafar_high: ReadWrite<u32, SmmuCb6IpafarHigh::Register>,
    _padding90232: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb6_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb6_tlbiva_high: WriteOnly<u32, SmmuCb6TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb6_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb6_tlbivaa_high: WriteOnly<u32, SmmuCb6TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb6_tlbiasid: WriteOnly<u32, SmmuCb6Tlbiasid::Register>,
    _padding91668: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb6_tlbiall: WriteOnly<u32>,
    _padding91676: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb6_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb6_tlbival_high: WriteOnly<u32, SmmuCb6TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb6_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb6_tlbivaal_high: WriteOnly<u32, SmmuCb6TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb6_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb6_tlbiipas2_high: WriteOnly<u32, SmmuCb6Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb6_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb6_tlbiipas2l_high: WriteOnly<u32, SmmuCb6Tlbiipas2lHigh::Register>,
    _padding91712: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb6_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb6_tlbstatus: ReadOnly<u32, SmmuCb6Tlbstatus::Register>,
    _padding92152: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb6_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb6_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb6_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb6_pmevcntr3: ReadWrite<u32>,
    _padding93712: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb6_pmevtyper0: ReadWrite<u32, SmmuCb6Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb6_pmevtyper1: ReadWrite<u32, SmmuCb6Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb6_pmevtyper2: ReadWrite<u32, SmmuCb6Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb6_pmevtyper3: ReadWrite<u32, SmmuCb6Pmevtyper3::Register>,
    _padding93840: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb6_pmcfgr: ReadOnly<u32, SmmuCb6Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb6_pmcr: Aliased<u32, SmmuCb6PmcrR::Register, SmmuCb6PmcrW::Register>,
    _padding93960: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb6_pmceid: ReadOnly<u32, SmmuCb6Pmceid::Register>,
    _padding93988: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb6_pmcntense: WriteOnly<u32, SmmuCb6Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb6_pmcntenclr: WriteOnly<u32, SmmuCb6Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb6_pmcntenset: WriteOnly<u32, SmmuCb6Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb6_pmintenclr: WriteOnly<u32, SmmuCb6Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb6_pmovsclr: WriteOnly<u32, SmmuCb6Pmovsclr::Register>,
    _padding94036: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb6_pmovsset: WriteOnly<u32, SmmuCb6Pmovsset::Register>,
    _padding94044: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb6_pmauthstatus: ReadOnly<u32, SmmuCb6Pmauthstatus::Register>,
    _padding94140: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb7_sctlr: Aliased<u32, SmmuCb7SctlrR::Register, SmmuCb7SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb7_actlr: ReadWrite<u32, SmmuCb7Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb7_resume: WriteOnly<u32, SmmuCb7Resume::Register>,
    _padding94220: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb7_tcr2: Aliased<u32, SmmuCb7Tcr2R::Register, SmmuCb7Tcr2W::Register>,
    _padding94228: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb7_ttbr0_low: Aliased<u32, SmmuCb7Ttbr0LowR::Register, SmmuCb7Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb7_ttbr0_high: ReadWrite<u32, SmmuCb7Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb7_ttbr1_low: ReadWrite<u32, SmmuCb7Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb7_ttbr1_high: ReadWrite<u32, SmmuCb7Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb7_tcr_lpae: ReadWrite<u32, SmmuCb7TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb7_contextidr: ReadWrite<u32, SmmuCb7Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb7_prrr_mair0: ReadWrite<u32, SmmuCb7PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb7_nmrr_mair1: ReadWrite<u32, SmmuCb7NmrrMair1::Register>,
    _padding94272: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb7_fsr: WriteOnly<u32, SmmuCb7Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb7_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb7_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb7_far_high: ReadWrite<u32, SmmuCb7FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb7_fsynr0: Aliased<u32, SmmuCb7Fsynr0R::Register, SmmuCb7Fsynr0W::Register>,
    _padding94316: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb7_ipafar_low: Aliased<u32, SmmuCb7IpafarLowR::Register, SmmuCb7IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb7_ipafar_high: ReadWrite<u32, SmmuCb7IpafarHigh::Register>,
    _padding94328: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb7_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb7_tlbiva_high: WriteOnly<u32, SmmuCb7TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb7_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb7_tlbivaa_high: WriteOnly<u32, SmmuCb7TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb7_tlbiasid: WriteOnly<u32, SmmuCb7Tlbiasid::Register>,
    _padding95764: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb7_tlbiall: WriteOnly<u32>,
    _padding95772: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb7_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb7_tlbival_high: WriteOnly<u32, SmmuCb7TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb7_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb7_tlbivaal_high: WriteOnly<u32, SmmuCb7TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb7_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb7_tlbiipas2_high: WriteOnly<u32, SmmuCb7Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb7_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb7_tlbiipas2l_high: WriteOnly<u32, SmmuCb7Tlbiipas2lHigh::Register>,
    _padding95808: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb7_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb7_tlbstatus: ReadOnly<u32, SmmuCb7Tlbstatus::Register>,
    _padding96248: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb7_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb7_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb7_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb7_pmevcntr3: ReadWrite<u32>,
    _padding97808: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb7_pmevtyper0: ReadWrite<u32, SmmuCb7Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb7_pmevtyper1: ReadWrite<u32, SmmuCb7Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb7_pmevtyper2: ReadWrite<u32, SmmuCb7Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb7_pmevtyper3: ReadWrite<u32, SmmuCb7Pmevtyper3::Register>,
    _padding97936: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb7_pmcfgr: ReadOnly<u32, SmmuCb7Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb7_pmcr: Aliased<u32, SmmuCb7PmcrR::Register, SmmuCb7PmcrW::Register>,
    _padding98056: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb7_pmceid: ReadOnly<u32, SmmuCb7Pmceid::Register>,
    _padding98084: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb7_pmcntense: WriteOnly<u32, SmmuCb7Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb7_pmcntenclr: WriteOnly<u32, SmmuCb7Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb7_pmcntenset: WriteOnly<u32, SmmuCb7Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb7_pmintenclr: WriteOnly<u32, SmmuCb7Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb7_pmovsclr: WriteOnly<u32, SmmuCb7Pmovsclr::Register>,
    _padding98132: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb7_pmovsset: WriteOnly<u32, SmmuCb7Pmovsset::Register>,
    _padding98140: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb7_pmauthstatus: ReadOnly<u32, SmmuCb7Pmauthstatus::Register>,
    _padding98236: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb8_sctlr: Aliased<u32, SmmuCb8SctlrR::Register, SmmuCb8SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb8_actlr: ReadWrite<u32, SmmuCb8Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb8_resume: WriteOnly<u32, SmmuCb8Resume::Register>,
    _padding98316: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb8_tcr2: Aliased<u32, SmmuCb8Tcr2R::Register, SmmuCb8Tcr2W::Register>,
    _padding98324: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb8_ttbr0_low: Aliased<u32, SmmuCb8Ttbr0LowR::Register, SmmuCb8Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb8_ttbr0_high: ReadWrite<u32, SmmuCb8Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb8_ttbr1_low: ReadWrite<u32, SmmuCb8Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb8_ttbr1_high: ReadWrite<u32, SmmuCb8Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb8_tcr_lpae: ReadWrite<u32, SmmuCb8TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb8_contextidr: ReadWrite<u32, SmmuCb8Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb8_prrr_mair0: ReadWrite<u32, SmmuCb8PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb8_nmrr_mair1: ReadWrite<u32, SmmuCb8NmrrMair1::Register>,
    _padding98368: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb8_fsr: WriteOnly<u32, SmmuCb8Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb8_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb8_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb8_far_high: ReadWrite<u32, SmmuCb8FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb8_fsynr0: Aliased<u32, SmmuCb8Fsynr0R::Register, SmmuCb8Fsynr0W::Register>,
    _padding98412: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb8_ipafar_low: Aliased<u32, SmmuCb8IpafarLowR::Register, SmmuCb8IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb8_ipafar_high: ReadWrite<u32, SmmuCb8IpafarHigh::Register>,
    _padding98424: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb8_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb8_tlbiva_high: WriteOnly<u32, SmmuCb8TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb8_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb8_tlbivaa_high: WriteOnly<u32, SmmuCb8TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb8_tlbiasid: WriteOnly<u32, SmmuCb8Tlbiasid::Register>,
    _padding99860: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb8_tlbiall: WriteOnly<u32>,
    _padding99868: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb8_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb8_tlbival_high: WriteOnly<u32, SmmuCb8TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb8_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb8_tlbivaal_high: WriteOnly<u32, SmmuCb8TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb8_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb8_tlbiipas2_high: WriteOnly<u32, SmmuCb8Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb8_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb8_tlbiipas2l_high: WriteOnly<u32, SmmuCb8Tlbiipas2lHigh::Register>,
    _padding99904: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb8_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb8_tlbstatus: ReadOnly<u32, SmmuCb8Tlbstatus::Register>,
    _padding100344: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb8_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb8_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb8_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb8_pmevcntr3: ReadWrite<u32>,
    _padding101904: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb8_pmevtyper0: ReadWrite<u32, SmmuCb8Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb8_pmevtyper1: ReadWrite<u32, SmmuCb8Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb8_pmevtyper2: ReadWrite<u32, SmmuCb8Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb8_pmevtyper3: ReadWrite<u32, SmmuCb8Pmevtyper3::Register>,
    _padding102032: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb8_pmcfgr: ReadOnly<u32, SmmuCb8Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb8_pmcr: Aliased<u32, SmmuCb8PmcrR::Register, SmmuCb8PmcrW::Register>,
    _padding102152: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb8_pmceid: ReadOnly<u32, SmmuCb8Pmceid::Register>,
    _padding102180: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb8_pmcntense: WriteOnly<u32, SmmuCb8Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb8_pmcntenclr: WriteOnly<u32, SmmuCb8Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb8_pmcntenset: WriteOnly<u32, SmmuCb8Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb8_pmintenclr: WriteOnly<u32, SmmuCb8Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb8_pmovsclr: WriteOnly<u32, SmmuCb8Pmovsclr::Register>,
    _padding102228: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb8_pmovsset: WriteOnly<u32, SmmuCb8Pmovsset::Register>,
    _padding102236: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb8_pmauthstatus: ReadOnly<u32, SmmuCb8Pmauthstatus::Register>,
    _padding102332: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb9_sctlr: Aliased<u32, SmmuCb9SctlrR::Register, SmmuCb9SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb9_actlr: ReadWrite<u32, SmmuCb9Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb9_resume: WriteOnly<u32, SmmuCb9Resume::Register>,
    _padding102412: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb9_tcr2: Aliased<u32, SmmuCb9Tcr2R::Register, SmmuCb9Tcr2W::Register>,
    _padding102420: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb9_ttbr0_low: Aliased<u32, SmmuCb9Ttbr0LowR::Register, SmmuCb9Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb9_ttbr0_high: ReadWrite<u32, SmmuCb9Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb9_ttbr1_low: ReadWrite<u32, SmmuCb9Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb9_ttbr1_high: ReadWrite<u32, SmmuCb9Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb9_tcr_lpae: ReadWrite<u32, SmmuCb9TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb9_contextidr: ReadWrite<u32, SmmuCb9Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb9_prrr_mair0: ReadWrite<u32, SmmuCb9PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb9_nmrr_mair1: ReadWrite<u32, SmmuCb9NmrrMair1::Register>,
    _padding102464: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb9_fsr: WriteOnly<u32, SmmuCb9Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb9_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb9_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb9_far_high: ReadWrite<u32, SmmuCb9FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb9_fsynr0: Aliased<u32, SmmuCb9Fsynr0R::Register, SmmuCb9Fsynr0W::Register>,
    _padding102508: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb9_ipafar_low: Aliased<u32, SmmuCb9IpafarLowR::Register, SmmuCb9IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb9_ipafar_high: ReadWrite<u32, SmmuCb9IpafarHigh::Register>,
    _padding102520: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb9_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb9_tlbiva_high: WriteOnly<u32, SmmuCb9TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb9_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb9_tlbivaa_high: WriteOnly<u32, SmmuCb9TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb9_tlbiasid: WriteOnly<u32, SmmuCb9Tlbiasid::Register>,
    _padding103956: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb9_tlbiall: WriteOnly<u32>,
    _padding103964: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb9_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb9_tlbival_high: WriteOnly<u32, SmmuCb9TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb9_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb9_tlbivaal_high: WriteOnly<u32, SmmuCb9TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb9_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb9_tlbiipas2_high: WriteOnly<u32, SmmuCb9Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb9_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb9_tlbiipas2l_high: WriteOnly<u32, SmmuCb9Tlbiipas2lHigh::Register>,
    _padding104000: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb9_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb9_tlbstatus: ReadOnly<u32, SmmuCb9Tlbstatus::Register>,
    _padding104440: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb9_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb9_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb9_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb9_pmevcntr3: ReadWrite<u32>,
    _padding106000: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb9_pmevtyper0: ReadWrite<u32, SmmuCb9Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb9_pmevtyper1: ReadWrite<u32, SmmuCb9Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb9_pmevtyper2: ReadWrite<u32, SmmuCb9Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb9_pmevtyper3: ReadWrite<u32, SmmuCb9Pmevtyper3::Register>,
    _padding106128: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb9_pmcfgr: ReadOnly<u32, SmmuCb9Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb9_pmcr: Aliased<u32, SmmuCb9PmcrR::Register, SmmuCb9PmcrW::Register>,
    _padding106248: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb9_pmceid: ReadOnly<u32, SmmuCb9Pmceid::Register>,
    _padding106276: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb9_pmcntense: WriteOnly<u32, SmmuCb9Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb9_pmcntenclr: WriteOnly<u32, SmmuCb9Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb9_pmcntenset: WriteOnly<u32, SmmuCb9Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb9_pmintenclr: WriteOnly<u32, SmmuCb9Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb9_pmovsclr: WriteOnly<u32, SmmuCb9Pmovsclr::Register>,
    _padding106324: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb9_pmovsset: WriteOnly<u32, SmmuCb9Pmovsset::Register>,
    _padding106332: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb9_pmauthstatus: ReadOnly<u32, SmmuCb9Pmauthstatus::Register>,
    _padding106428: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb10_sctlr: Aliased<u32, SmmuCb10SctlrR::Register, SmmuCb10SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb10_actlr: ReadWrite<u32, SmmuCb10Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb10_resume: WriteOnly<u32, SmmuCb10Resume::Register>,
    _padding106508: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb10_tcr2: Aliased<u32, SmmuCb10Tcr2R::Register, SmmuCb10Tcr2W::Register>,
    _padding106516: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb10_ttbr0_low: Aliased<u32, SmmuCb10Ttbr0LowR::Register, SmmuCb10Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb10_ttbr0_high: ReadWrite<u32, SmmuCb10Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb10_ttbr1_low: ReadWrite<u32, SmmuCb10Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb10_ttbr1_high: ReadWrite<u32, SmmuCb10Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb10_tcr_lpae: ReadWrite<u32, SmmuCb10TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb10_contextidr: ReadWrite<u32, SmmuCb10Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb10_prrr_mair0: ReadWrite<u32, SmmuCb10PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb10_nmrr_mair1: ReadWrite<u32, SmmuCb10NmrrMair1::Register>,
    _padding106560: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb10_fsr: WriteOnly<u32, SmmuCb10Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb10_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb10_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb10_far_high: ReadWrite<u32, SmmuCb10FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb10_fsynr0: Aliased<u32, SmmuCb10Fsynr0R::Register, SmmuCb10Fsynr0W::Register>,
    _padding106604: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb10_ipafar_low:
        Aliased<u32, SmmuCb10IpafarLowR::Register, SmmuCb10IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb10_ipafar_high: ReadWrite<u32, SmmuCb10IpafarHigh::Register>,
    _padding106616: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb10_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb10_tlbiva_high: WriteOnly<u32, SmmuCb10TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb10_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb10_tlbivaa_high: WriteOnly<u32, SmmuCb10TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb10_tlbiasid: WriteOnly<u32, SmmuCb10Tlbiasid::Register>,
    _padding108052: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb10_tlbiall: WriteOnly<u32>,
    _padding108060: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb10_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb10_tlbival_high: WriteOnly<u32, SmmuCb10TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb10_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb10_tlbivaal_high: WriteOnly<u32, SmmuCb10TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb10_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb10_tlbiipas2_high: WriteOnly<u32, SmmuCb10Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb10_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb10_tlbiipas2l_high: WriteOnly<u32, SmmuCb10Tlbiipas2lHigh::Register>,
    _padding108096: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb10_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb10_tlbstatus: ReadOnly<u32, SmmuCb10Tlbstatus::Register>,
    _padding108536: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb10_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb10_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb10_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb10_pmevcntr3: ReadWrite<u32>,
    _padding110096: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb10_pmevtyper0: ReadWrite<u32, SmmuCb10Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb10_pmevtyper1: ReadWrite<u32, SmmuCb10Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb10_pmevtyper2: ReadWrite<u32, SmmuCb10Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb10_pmevtyper3: ReadWrite<u32, SmmuCb10Pmevtyper3::Register>,
    _padding110224: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb10_pmcfgr: ReadOnly<u32, SmmuCb10Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb10_pmcr: Aliased<u32, SmmuCb10PmcrR::Register, SmmuCb10PmcrW::Register>,
    _padding110344: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb10_pmceid: ReadOnly<u32, SmmuCb10Pmceid::Register>,
    _padding110372: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb10_pmcntense: WriteOnly<u32, SmmuCb10Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb10_pmcntenclr: WriteOnly<u32, SmmuCb10Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb10_pmcntenset: WriteOnly<u32, SmmuCb10Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb10_pmintenclr: WriteOnly<u32, SmmuCb10Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb10_pmovsclr: WriteOnly<u32, SmmuCb10Pmovsclr::Register>,
    _padding110420: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb10_pmovsset: WriteOnly<u32, SmmuCb10Pmovsset::Register>,
    _padding110428: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb10_pmauthstatus: ReadOnly<u32, SmmuCb10Pmauthstatus::Register>,
    _padding110524: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb11_sctlr: Aliased<u32, SmmuCb11SctlrR::Register, SmmuCb11SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb11_actlr: ReadWrite<u32, SmmuCb11Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb11_resume: WriteOnly<u32, SmmuCb11Resume::Register>,
    _padding110604: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb11_tcr2: Aliased<u32, SmmuCb11Tcr2R::Register, SmmuCb11Tcr2W::Register>,
    _padding110612: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb11_ttbr0_low: Aliased<u32, SmmuCb11Ttbr0LowR::Register, SmmuCb11Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb11_ttbr0_high: ReadWrite<u32, SmmuCb11Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb11_ttbr1_low: ReadWrite<u32, SmmuCb11Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb11_ttbr1_high: ReadWrite<u32, SmmuCb11Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb11_tcr_lpae: ReadWrite<u32, SmmuCb11TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb11_contextidr: ReadWrite<u32, SmmuCb11Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb11_prrr_mair0: ReadWrite<u32, SmmuCb11PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb11_nmrr_mair1: ReadWrite<u32, SmmuCb11NmrrMair1::Register>,
    _padding110656: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb11_fsr: WriteOnly<u32, SmmuCb11Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb11_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb11_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb11_far_high: ReadWrite<u32, SmmuCb11FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb11_fsynr0: Aliased<u32, SmmuCb11Fsynr0R::Register, SmmuCb11Fsynr0W::Register>,
    _padding110700: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb11_ipafar_low:
        Aliased<u32, SmmuCb11IpafarLowR::Register, SmmuCb11IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb11_ipafar_high: ReadWrite<u32, SmmuCb11IpafarHigh::Register>,
    _padding110712: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb11_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb11_tlbiva_high: WriteOnly<u32, SmmuCb11TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb11_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb11_tlbivaa_high: WriteOnly<u32, SmmuCb11TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb11_tlbiasid: WriteOnly<u32, SmmuCb11Tlbiasid::Register>,
    _padding112148: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb11_tlbiall: WriteOnly<u32>,
    _padding112156: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb11_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb11_tlbival_high: WriteOnly<u32, SmmuCb11TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb11_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb11_tlbivaal_high: WriteOnly<u32, SmmuCb11TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb11_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb11_tlbiipas2_high: WriteOnly<u32, SmmuCb11Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb11_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb11_tlbiipas2l_high: WriteOnly<u32, SmmuCb11Tlbiipas2lHigh::Register>,
    _padding112192: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb11_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb11_tlbstatus: ReadOnly<u32, SmmuCb11Tlbstatus::Register>,
    _padding112632: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb11_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb11_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb11_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb11_pmevcntr3: ReadWrite<u32>,
    _padding114192: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb11_pmevtyper0: ReadWrite<u32, SmmuCb11Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb11_pmevtyper1: ReadWrite<u32, SmmuCb11Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb11_pmevtyper2: ReadWrite<u32, SmmuCb11Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb11_pmevtyper3: ReadWrite<u32, SmmuCb11Pmevtyper3::Register>,
    _padding114320: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb11_pmcfgr: ReadOnly<u32, SmmuCb11Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb11_pmcr: Aliased<u32, SmmuCb11PmcrR::Register, SmmuCb11PmcrW::Register>,
    _padding114440: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb11_pmceid: ReadOnly<u32, SmmuCb11Pmceid::Register>,
    _padding114468: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb11_pmcntense: WriteOnly<u32, SmmuCb11Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb11_pmcntenclr: WriteOnly<u32, SmmuCb11Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb11_pmcntenset: WriteOnly<u32, SmmuCb11Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb11_pmintenclr: WriteOnly<u32, SmmuCb11Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb11_pmovsclr: WriteOnly<u32, SmmuCb11Pmovsclr::Register>,
    _padding114516: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb11_pmovsset: WriteOnly<u32, SmmuCb11Pmovsset::Register>,
    _padding114524: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb11_pmauthstatus: ReadOnly<u32, SmmuCb11Pmauthstatus::Register>,
    _padding114620: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb12_sctlr: Aliased<u32, SmmuCb12SctlrR::Register, SmmuCb12SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb12_actlr: ReadWrite<u32, SmmuCb12Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb12_resume: WriteOnly<u32, SmmuCb12Resume::Register>,
    _padding114700: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb12_tcr2: Aliased<u32, SmmuCb12Tcr2R::Register, SmmuCb12Tcr2W::Register>,
    _padding114708: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb12_ttbr0_low: Aliased<u32, SmmuCb12Ttbr0LowR::Register, SmmuCb12Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb12_ttbr0_high: ReadWrite<u32, SmmuCb12Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb12_ttbr1_low: ReadWrite<u32, SmmuCb12Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb12_ttbr1_high: ReadWrite<u32, SmmuCb12Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb12_tcr_lpae: ReadWrite<u32, SmmuCb12TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb12_contextidr: ReadWrite<u32, SmmuCb12Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb12_prrr_mair0: ReadWrite<u32, SmmuCb12PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb12_nmrr_mair1: ReadWrite<u32, SmmuCb12NmrrMair1::Register>,
    _padding114752: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb12_fsr: WriteOnly<u32, SmmuCb12Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb12_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb12_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb12_far_high: ReadWrite<u32, SmmuCb12FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb12_fsynr0: Aliased<u32, SmmuCb12Fsynr0R::Register, SmmuCb12Fsynr0W::Register>,
    _padding114796: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb12_ipafar_low:
        Aliased<u32, SmmuCb12IpafarLowR::Register, SmmuCb12IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb12_ipafar_high: ReadWrite<u32, SmmuCb12IpafarHigh::Register>,
    _padding114808: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb12_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb12_tlbiva_high: WriteOnly<u32, SmmuCb12TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb12_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb12_tlbivaa_high: WriteOnly<u32, SmmuCb12TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb12_tlbiasid: WriteOnly<u32, SmmuCb12Tlbiasid::Register>,
    _padding116244: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb12_tlbiall: WriteOnly<u32>,
    _padding116252: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb12_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb12_tlbival_high: WriteOnly<u32, SmmuCb12TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb12_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb12_tlbivaal_high: WriteOnly<u32, SmmuCb12TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb12_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb12_tlbiipas2_high: WriteOnly<u32, SmmuCb12Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb12_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb12_tlbiipas2l_high: WriteOnly<u32, SmmuCb12Tlbiipas2lHigh::Register>,
    _padding116288: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb12_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb12_tlbstatus: ReadOnly<u32, SmmuCb12Tlbstatus::Register>,
    _padding116728: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb12_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb12_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb12_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb12_pmevcntr3: ReadWrite<u32>,
    _padding118288: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb12_pmevtyper0: ReadWrite<u32, SmmuCb12Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb12_pmevtyper1: ReadWrite<u32, SmmuCb12Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb12_pmevtyper2: ReadWrite<u32, SmmuCb12Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb12_pmevtyper3: ReadWrite<u32, SmmuCb12Pmevtyper3::Register>,
    _padding118416: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb12_pmcfgr: ReadOnly<u32, SmmuCb12Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb12_pmcr: Aliased<u32, SmmuCb12PmcrR::Register, SmmuCb12PmcrW::Register>,
    _padding118536: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb12_pmceid: ReadOnly<u32, SmmuCb12Pmceid::Register>,
    _padding118564: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb12_pmcntense: WriteOnly<u32, SmmuCb12Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb12_pmcntenclr: WriteOnly<u32, SmmuCb12Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb12_pmcntenset: WriteOnly<u32, SmmuCb12Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb12_pmintenclr: WriteOnly<u32, SmmuCb12Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb12_pmovsclr: WriteOnly<u32, SmmuCb12Pmovsclr::Register>,
    _padding118612: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb12_pmovsset: WriteOnly<u32, SmmuCb12Pmovsset::Register>,
    _padding118620: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb12_pmauthstatus: ReadOnly<u32, SmmuCb12Pmauthstatus::Register>,
    _padding118716: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb13_sctlr: Aliased<u32, SmmuCb13SctlrR::Register, SmmuCb13SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb13_actlr: ReadWrite<u32, SmmuCb13Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb13_resume: WriteOnly<u32, SmmuCb13Resume::Register>,
    _padding118796: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb13_tcr2: Aliased<u32, SmmuCb13Tcr2R::Register, SmmuCb13Tcr2W::Register>,
    _padding118804: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb13_ttbr0_low: Aliased<u32, SmmuCb13Ttbr0LowR::Register, SmmuCb13Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb13_ttbr0_high: ReadWrite<u32, SmmuCb13Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb13_ttbr1_low: ReadWrite<u32, SmmuCb13Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb13_ttbr1_high: ReadWrite<u32, SmmuCb13Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb13_tcr_lpae: ReadWrite<u32, SmmuCb13TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb13_contextidr: ReadWrite<u32, SmmuCb13Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb13_prrr_mair0: ReadWrite<u32, SmmuCb13PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb13_nmrr_mair1: ReadWrite<u32, SmmuCb13NmrrMair1::Register>,
    _padding118848: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb13_fsr: WriteOnly<u32, SmmuCb13Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb13_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb13_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb13_far_high: ReadWrite<u32, SmmuCb13FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb13_fsynr0: Aliased<u32, SmmuCb13Fsynr0R::Register, SmmuCb13Fsynr0W::Register>,
    _padding118892: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb13_ipafar_low:
        Aliased<u32, SmmuCb13IpafarLowR::Register, SmmuCb13IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb13_ipafar_high: ReadWrite<u32, SmmuCb13IpafarHigh::Register>,
    _padding118904: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb13_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb13_tlbiva_high: WriteOnly<u32, SmmuCb13TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb13_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb13_tlbivaa_high: WriteOnly<u32, SmmuCb13TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb13_tlbiasid: WriteOnly<u32, SmmuCb13Tlbiasid::Register>,
    _padding120340: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb13_tlbiall: WriteOnly<u32>,
    _padding120348: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb13_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb13_tlbival_high: WriteOnly<u32, SmmuCb13TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb13_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb13_tlbivaal_high: WriteOnly<u32, SmmuCb13TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb13_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb13_tlbiipas2_high: WriteOnly<u32, SmmuCb13Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb13_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb13_tlbiipas2l_high: WriteOnly<u32, SmmuCb13Tlbiipas2lHigh::Register>,
    _padding120384: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb13_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb13_tlbstatus: ReadOnly<u32, SmmuCb13Tlbstatus::Register>,
    _padding120824: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb13_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb13_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb13_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb13_pmevcntr3: ReadWrite<u32>,
    _padding122384: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb13_pmevtyper0: ReadWrite<u32, SmmuCb13Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb13_pmevtyper1: ReadWrite<u32, SmmuCb13Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb13_pmevtyper2: ReadWrite<u32, SmmuCb13Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb13_pmevtyper3: ReadWrite<u32, SmmuCb13Pmevtyper3::Register>,
    _padding122512: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb13_pmcfgr: ReadOnly<u32, SmmuCb13Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb13_pmcr: Aliased<u32, SmmuCb13PmcrR::Register, SmmuCb13PmcrW::Register>,
    _padding122632: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb13_pmceid: ReadOnly<u32, SmmuCb13Pmceid::Register>,
    _padding122660: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb13_pmcntense: WriteOnly<u32, SmmuCb13Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb13_pmcntenclr: WriteOnly<u32, SmmuCb13Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb13_pmcntenset: WriteOnly<u32, SmmuCb13Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb13_pmintenclr: WriteOnly<u32, SmmuCb13Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb13_pmovsclr: WriteOnly<u32, SmmuCb13Pmovsclr::Register>,
    _padding122708: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb13_pmovsset: WriteOnly<u32, SmmuCb13Pmovsset::Register>,
    _padding122716: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb13_pmauthstatus: ReadOnly<u32, SmmuCb13Pmauthstatus::Register>,
    _padding122812: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb14_sctlr: Aliased<u32, SmmuCb14SctlrR::Register, SmmuCb14SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb14_actlr: ReadWrite<u32, SmmuCb14Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb14_resume: WriteOnly<u32, SmmuCb14Resume::Register>,
    _padding122892: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb14_tcr2: Aliased<u32, SmmuCb14Tcr2R::Register, SmmuCb14Tcr2W::Register>,
    _padding122900: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb14_ttbr0_low: Aliased<u32, SmmuCb14Ttbr0LowR::Register, SmmuCb14Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb14_ttbr0_high: ReadWrite<u32, SmmuCb14Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb14_ttbr1_low: ReadWrite<u32, SmmuCb14Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb14_ttbr1_high: ReadWrite<u32, SmmuCb14Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb14_tcr_lpae: ReadWrite<u32, SmmuCb14TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb14_contextidr: ReadWrite<u32, SmmuCb14Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb14_prrr_mair0: ReadWrite<u32, SmmuCb14PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb14_nmrr_mair1: ReadWrite<u32, SmmuCb14NmrrMair1::Register>,
    _padding122944: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb14_fsr: WriteOnly<u32, SmmuCb14Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb14_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb14_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb14_far_high: ReadWrite<u32, SmmuCb14FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb14_fsynr0: Aliased<u32, SmmuCb14Fsynr0R::Register, SmmuCb14Fsynr0W::Register>,
    _padding122988: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb14_ipafar_low:
        Aliased<u32, SmmuCb14IpafarLowR::Register, SmmuCb14IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb14_ipafar_high: ReadWrite<u32, SmmuCb14IpafarHigh::Register>,
    _padding123000: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb14_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb14_tlbiva_high: WriteOnly<u32, SmmuCb14TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb14_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb14_tlbivaa_high: WriteOnly<u32, SmmuCb14TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb14_tlbiasid: WriteOnly<u32, SmmuCb14Tlbiasid::Register>,
    _padding124436: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb14_tlbiall: WriteOnly<u32>,
    _padding124444: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb14_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb14_tlbival_high: WriteOnly<u32, SmmuCb14TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb14_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb14_tlbivaal_high: WriteOnly<u32, SmmuCb14TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb14_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb14_tlbiipas2_high: WriteOnly<u32, SmmuCb14Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb14_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb14_tlbiipas2l_high: WriteOnly<u32, SmmuCb14Tlbiipas2lHigh::Register>,
    _padding124480: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb14_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb14_tlbstatus: ReadOnly<u32, SmmuCb14Tlbstatus::Register>,
    _padding124920: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb14_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb14_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb14_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb14_pmevcntr3: ReadWrite<u32>,
    _padding126480: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb14_pmevtyper0: ReadWrite<u32, SmmuCb14Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb14_pmevtyper1: ReadWrite<u32, SmmuCb14Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb14_pmevtyper2: ReadWrite<u32, SmmuCb14Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb14_pmevtyper3: ReadWrite<u32, SmmuCb14Pmevtyper3::Register>,
    _padding126608: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb14_pmcfgr: ReadOnly<u32, SmmuCb14Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb14_pmcr: Aliased<u32, SmmuCb14PmcrR::Register, SmmuCb14PmcrW::Register>,
    _padding126728: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb14_pmceid: ReadOnly<u32, SmmuCb14Pmceid::Register>,
    _padding126756: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb14_pmcntense: WriteOnly<u32, SmmuCb14Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb14_pmcntenclr: WriteOnly<u32, SmmuCb14Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb14_pmcntenset: WriteOnly<u32, SmmuCb14Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb14_pmintenclr: WriteOnly<u32, SmmuCb14Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb14_pmovsclr: WriteOnly<u32, SmmuCb14Pmovsclr::Register>,
    _padding126804: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb14_pmovsset: WriteOnly<u32, SmmuCb14Pmovsset::Register>,
    _padding126812: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb14_pmauthstatus: ReadOnly<u32, SmmuCb14Pmauthstatus::Register>,
    _padding126908: [u8; 68],
    /// The System Control register provides the top level control of the translation system for the related Context bank.
    pub smmu_cb15_sctlr: Aliased<u32, SmmuCb15SctlrR::Register, SmmuCb15SctlrW::Register>,
    /// The Auxillary Control register provides implementation specific configuration and control options.
    pub smmu_cb15_actlr: ReadWrite<u32, SmmuCb15Actlr::Register>,
    /// The Transaction Resume is used to resume operation of a transaction that is stalled because of an existing fault condition.
    pub smmu_cb15_resume: WriteOnly<u32, SmmuCb15Resume::Register>,
    _padding126988: [u8; 4],
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb15_tcr2: Aliased<u32, SmmuCb15Tcr2R::Register, SmmuCb15Tcr2W::Register>,
    _padding126996: [u8; 12],
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb15_ttbr0_low: Aliased<u32, SmmuCb15Ttbr0LowR::Register, SmmuCb15Ttbr0LowW::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 0.
    pub smmu_cb15_ttbr0_high: ReadWrite<u32, SmmuCb15Ttbr0High::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb15_ttbr1_low: ReadWrite<u32, SmmuCb15Ttbr1Low::Register>,
    /// The Translation Table Base register 0 holds the base address of the translation table 1.
    pub smmu_cb15_ttbr1_high: ReadWrite<u32, SmmuCb15Ttbr1High::Register>,
    /// The Translation Table base control register determines which of the TTBRs(SMMU_CBn_TTBR0 or SMMU_CBn_TTBR1) defines the base address for the translation table walk that is required when the input address is not found in the TLB.
    pub smmu_cb15_tcr_lpae: ReadWrite<u32, SmmuCb15TcrLpae::Register>,
    /// Identifies the current process identifier and the current address space identifier
    pub smmu_cb15_contextidr: ReadWrite<u32, SmmuCb15Contextidr::Register>,
    /// Primary region remap register if AArch32 short descriptor scheme is selected. Controls top-level mapping of the TEX, C, and B memory region attributes. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb15_prrr_mair0: ReadWrite<u32, SmmuCb15PrrrMair0::Register>,
    /// Normal memory remap register if AArch32 short descriptor scheme is selected. Provides additional mapping controls for memory regions that are mapped as Normal memory by their entry in SMMU_CBn_PRRR. Memory attribute indirection register when AArch32 Long descriptor scheme or AArch64 translation scheme is selected. Provide a revised version of the TEX-Remap system to redirect the selection of memory attributes from the translation table entries.
    pub smmu_cb15_nmrr_mair1: ReadWrite<u32, SmmuCb15NmrrMair1::Register>,
    _padding127040: [u8; 24],
    /// Provides memory system fault status information.
    pub smmu_cb15_fsr: WriteOnly<u32, SmmuCb15Fsr::Register>,
    /// Restores the state of SMMU_CBn_FSR, after a reset, for example.
    pub smmu_cb15_fsrrestore: WriteOnly<u32>,
    /// Holds the Lower input address bits [31:0] of the memory access that caused a synchronous abort exception.
    pub smmu_cb15_far_low: ReadWrite<u32>,
    /// Holds the Upper input address bits [63:32] of the memory access that caused a synchronous abort exception.
    pub smmu_cb15_far_high: ReadWrite<u32, SmmuCb15FarHigh::Register>,
    /// Holds fault syndrome information about the memory access that caused a synchronous abort exception
    pub smmu_cb15_fsynr0: Aliased<u32, SmmuCb15Fsynr0R::Register, SmmuCb15Fsynr0W::Register>,
    _padding127084: [u8; 4],
    /// The stage 1 IPA Fault Address Lower bits [31:0] Register.
    pub smmu_cb15_ipafar_low:
        Aliased<u32, SmmuCb15IpafarLowR::Register, SmmuCb15IpafarLowW::Register>,
    /// The stage 1 IPA Fault Address Upper bits [63:32] Register
    pub smmu_cb15_ipafar_high: ReadWrite<u32, SmmuCb15IpafarHigh::Register>,
    _padding127096: [u8; 1416],
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate.
    pub smmu_cb15_tlbiva_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match both the VA provided and the TLB tagging scheme of the context bank, including any global entries if appropriate
    pub smmu_cb15_tlbiva_high: WriteOnly<u32, SmmuCb15TlbivaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb15_tlbivaa_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.
    pub smmu_cb15_tlbivaa_high: WriteOnly<u32, SmmuCb15TlbivaaHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the ASID provided as an argument
    pub smmu_cb15_tlbiasid: WriteOnly<u32, SmmuCb15Tlbiasid::Register>,
    _padding128532: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that are tagged as: i) Hypervisor, for HYPC banks, ii)Non-secure, using the VMID of the context bank, for Non-secure, non-HYPC context banks,iii) Secure, using any ASID, for Secure context banks.
    pub smmu_cb15_tlbiall: WriteOnly<u32>,
    _padding128540: [u8; 4],
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb15_tlbival_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA and ASID provided as arguments, and the VMID of the context bank. This register is similar to SMMU_CBn_TLBIVA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation.
    pub smmu_cb15_tlbival_high: WriteOnly<u32, SmmuCb15TlbivalHigh::Register>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb15_tlbivaal_low: WriteOnly<u32>,
    /// Invalidates all of the unlocked TLB entries that match the VA provided as an argument, and the VMID of the context bank, regardless of the ASID. This operation includes global entries if appropriate.This register is similar to SMMU_CBn_TLBIVAA, but it is only required to invalidate cached copies of the last level of translation table walk of the first stage of translation
    pub smmu_cb15_tlbivaal_high: WriteOnly<u32, SmmuCb15TlbivaalHigh::Register>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb15_tlbiipas2_low: WriteOnly<u32>,
    /// Invalidates all unlocked TLB entries that match the IPA provided
    pub smmu_cb15_tlbiipas2_high: WriteOnly<u32, SmmuCb15Tlbiipas2High::Register>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb15_tlbiipas2l_low: WriteOnly<u32>,
    /// Invalidates any unlocked TLB entries that match the IPA provided and that correspond to the final level of translation table lookup
    pub smmu_cb15_tlbiipas2l_high: WriteOnly<u32, SmmuCb15Tlbiipas2lHigh::Register>,
    _padding128576: [u8; 432],
    /// Initiates a synchronization operation that ensures the completion of any TLB invalidate operations previously accepted in the corresponding translation context bank.
    pub smmu_cb15_tlbsync: WriteOnly<u32>,
    /// Indicates the status of any TLB maintenance operations issued before the most recent SMMU_CBn_TLBSYNC operation
    pub smmu_cb15_tlbstatus: ReadOnly<u32, SmmuCb15Tlbstatus::Register>,
    _padding129016: [u8; 1544],
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb15_pmevcntr0: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb15_pmevcntr1: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb15_pmevcntr2: ReadWrite<u32>,
    /// Provides event counter resources in the register map of a translation context bank. Reads or writes the value of the selected event counter.
    pub smmu_cb15_pmevcntr3: ReadWrite<u32>,
    _padding130576: [u8; 112],
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb15_pmevtyper0: ReadWrite<u32, SmmuCb15Pmevtyper0::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb15_pmevtyper1: ReadWrite<u32, SmmuCb15Pmevtyper1::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb15_pmevtyper2: ReadWrite<u32, SmmuCb15Pmevtyper2::Register>,
    /// Provides event type resources in the register map of a translation context bank. Controls which events are counted by the corresponding event counter
    pub smmu_cb15_pmevtyper3: ReadWrite<u32, SmmuCb15Pmevtyper3::Register>,
    _padding130704: [u8; 112],
    /// Provides a performance monitoring configuration register in the register map of a translation context bank. Provides Performance Monitoring Unit (PMU) configuration data.
    pub smmu_cb15_pmcfgr: ReadOnly<u32, SmmuCb15Pmcfgr::Register>,
    /// Provides the equivalent of the PMCR register, in the register map of a translation context bank. PMCR provides controls for the Performance Monitors.
    pub smmu_cb15_pmcr: Aliased<u32, SmmuCb15PmcrR::Register, SmmuCb15PmcrW::Register>,
    _padding130824: [u8; 24],
    /// Provide the equivalent of the SMMU performance monitoring register map PMCEID0 register, in the register map of a translation context bank. Describes the event classes supported by the SMMU implementation.
    pub smmu_cb15_pmceid: ReadOnly<u32, SmmuCb15Pmceid::Register>,
    _padding130852: [u8; 28],
    /// Provides the equivalent of the PMCNTENSETx register, in the register map of a translation context bank. Enables any implemented event counters.
    pub smmu_cb15_pmcntense: WriteOnly<u32, SmmuCb15Pmcntense::Register>,
    /// Provides the equivalent of the PMCNTENCLRx register, in the register map of a translation context bank. Disables any implemented event counter.
    pub smmu_cb15_pmcntenclr: WriteOnly<u32, SmmuCb15Pmcntenclr::Register>,
    /// Provides the equivalent of the PMINTENSETx in the register map of a translation context bank. Enables the generation of interrupt requests on overflows from each implemented event counter
    pub smmu_cb15_pmcntenset: WriteOnly<u32, SmmuCb15Pmcntenset::Register>,
    /// Provides the equivalent of the PMINTENCLRx in the register map of a translation context bank. Disables the generation of interrupt requests on overflows from each implemented event counter.
    pub smmu_cb15_pmintenclr: WriteOnly<u32, SmmuCb15Pmintenclr::Register>,
    /// Provides the equivalent of the PMOVSCLRx register, in the register map of a translation context bank. Clears the state of the overflow bit for each implemented event counter.
    pub smmu_cb15_pmovsclr: WriteOnly<u32, SmmuCb15Pmovsclr::Register>,
    _padding130900: [u8; 4],
    /// Provides the equivalent of PMOVSSETx, in the register map of a translation context bank. Sets the state of the overflow bit for each of the implemented event counters.
    pub smmu_cb15_pmovsset: WriteOnly<u32, SmmuCb15Pmovsset::Register>,
    _padding130908: [u8; 92],
    /// Provides the equivalent of the PMAUTHSTATUS register, in the register map of a translation context bank. Indicates the implemented debug features and provides the current values of the configuration inputs that determine the debug permissions
    pub smmu_cb15_pmauthstatus: ReadOnly<u32, SmmuCb15Pmauthstatus::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub SmmuScr0R [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        SMCFCFG OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        BSU OFFSET(14) NUMBITS(2) [],
        FB OFFSET(13) NUMBITS(1) [],
        PTM OFFSET(12) NUMBITS(1) [],
        USFCFG OFFSET(10) NUMBITS(1) [],
        GSE OFFSET(9) NUMBITS(1) [],
        STALLD OFFSET(8) NUMBITS(1) [],
        TRANSIENTCFG OFFSET(6) NUMBITS(2) [],
        GCFGFIE OFFSET(5) NUMBITS(1) [],
        GCFGFRE OFFSET(4) NUMBITS(1) [],
        GFIE OFFSET(2) NUMBITS(1) [],
        GFRE OFFSET(1) NUMBITS(1) [],
        CLIENTPD OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuScr0W [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        BSU OFFSET(14) NUMBITS(2) [],
        FB OFFSET(13) NUMBITS(1) [],
        PTM OFFSET(12) NUMBITS(1) [],
        USFCFG OFFSET(10) NUMBITS(1) [],
        TRANSIENTCFG OFFSET(6) NUMBITS(2) [],
        GFIE OFFSET(2) NUMBITS(1) [],
        GFRE OFFSET(1) NUMBITS(1) [],
        CLIENTPD OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuScr1R [
        NSCAFRO OFFSET(28) NUMBITS(1) [],
        SPMEN OFFSET(27) NUMBITS(1) [],
        SIF OFFSET(26) NUMBITS(1) [],
        GEFRO OFFSET(25) NUMBITS(1) [],
        GASRAE OFFSET(24) NUMBITS(1) [],
        NSNUMIRPTO OFFSET(16) NUMBITS(8) [],
        NSNUMSMRGO OFFSET(8) NUMBITS(6) [],
        NSNUMCBO OFFSET(0) NUMBITS(5) [],
    ],
    pub SmmuScr1W [
        SPMEN OFFSET(27) NUMBITS(1) [],
        SIF OFFSET(26) NUMBITS(1) [],
        GEFRO OFFSET(25) NUMBITS(1) [],
        GASRAE OFFSET(24) NUMBITS(1) [],
        NSNUMSMRGO OFFSET(8) NUMBITS(6) [],
        NSNUMCBO OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSacr [
        NORMALIZE OFFSET(27) NUMBITS(1) [],
        CACHE_LOCK OFFSET(26) NUMBITS(1) [],
        PAGESIZE OFFSET(16) NUMBITS(1) [],
        S2CRB_TLBEN OFFSET(10) NUMBITS(1) [],
        MMUDISB_TLBEN OFFSET(9) NUMBITS(1) [],
        SMTNMB_TLBEN OFFSET(8) NUMBITS(1) [],
        S1WC2EN OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSidr0 [
        SES OFFSET(31) NUMBITS(1) [],
        S1TS OFFSET(30) NUMBITS(1) [],
        S2TS OFFSET(29) NUMBITS(1) [],
        NTS OFFSET(28) NUMBITS(1) [],
        SMS OFFSET(27) NUMBITS(1) [],
        ATOSNS OFFSET(26) NUMBITS(1) [],
        PTFS OFFSET(24) NUMBITS(2) [],
        NUMIRPT OFFSET(16) NUMBITS(8) [],
        CTTW OFFSET(14) NUMBITS(1) [],
        BTM OFFSET(13) NUMBITS(1) [],
        NUMSIDB OFFSET(9) NUMBITS(4) [],
        NUMSMRG OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSidr1 [
        PAGESIZE OFFSET(31) NUMBITS(1) [],
        NUMPAGENDXB OFFSET(28) NUMBITS(3) [],
        NUMS2CB OFFSET(16) NUMBITS(8) [],
        SMCD OFFSET(15) NUMBITS(1) [],
        SSDTP OFFSET(12) NUMBITS(1) [],
        NUMSSDNDXB OFFSET(8) NUMBITS(4) [],
        NUMCB OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSidr2 [
        PTFSV8_64KB OFFSET(14) NUMBITS(1) [],
        PTFSV8_16KB OFFSET(13) NUMBITS(1) [],
        TFSV8_4KB OFFSET(12) NUMBITS(1) [],
        UBS OFFSET(8) NUMBITS(4) [],
        OAS OFFSET(4) NUMBITS(4) [],
        IAS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSidr7 [
        MAJOR OFFSET(4) NUMBITS(4) [],
        MINOR OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSgfarHigh [
        FADDR OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSgfsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        UUT OFFSET(8) NUMBITS(1) [],
        PF OFFSET(7) NUMBITS(1) [],
        EF OFFSET(6) NUMBITS(1) [],
        CAF OFFSET(5) NUMBITS(1) [],
        UCIF OFFSET(4) NUMBITS(1) [],
        UCBF OFFSET(3) NUMBITS(1) [],
        SMCF OFFSET(2) NUMBITS(1) [],
        USF OFFSET(1) NUMBITS(1) [],
        ICF OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSgfsrrestore [
        MULTI OFFSET(31) NUMBITS(1) [],
        UUT OFFSET(8) NUMBITS(1) [],
        PF OFFSET(7) NUMBITS(1) [],
        EF OFFSET(6) NUMBITS(1) [],
        CAF OFFSET(5) NUMBITS(1) [],
        UCIF OFFSET(4) NUMBITS(1) [],
        UCBF OFFSET(3) NUMBITS(1) [],
        SMCF OFFSET(2) NUMBITS(1) [],
        USF OFFSET(1) NUMBITS(1) [],
        ICF OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSgfsynr0R [
        ATS OFFSET(6) NUMBITS(1) [],
        NSATTR OFFSET(5) NUMBITS(1) [],
        NSSTATE OFFSET(4) NUMBITS(1) [],
        IND OFFSET(3) NUMBITS(1) [],
        PNU OFFSET(2) NUMBITS(1) [],
        WNR OFFSET(1) NUMBITS(1) [],
    ],
    pub SmmuSgfsynr0W [
        NSATTR OFFSET(5) NUMBITS(1) [],
        NSSTATE OFFSET(4) NUMBITS(1) [],
        IND OFFSET(3) NUMBITS(1) [],
        PNU OFFSET(2) NUMBITS(1) [],
        WNR OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSgfsynr1 [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuTlbivmid [
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuStlbgstatus [
        GSACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuDbgrptrtbu [
        TBU_ID OFFSET(24) NUMBITS(3) [],
        TLB_POINTER OFFSET(4) NUMBITS(12) [],
        TLB_ENTRY_POINTER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuDbgrptrtcu [
        DATASRC OFFSET(26) NUMBITS(2) [],
        WAY_RAM OFFSET(24) NUMBITS(2) [],
        TLB_POINTER OFFSET(4) NUMBITS(9) [],
        TLB_ENTRY_POINTER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuStlbivalmHigh [
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuStlbivamHigh [
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuNscr0R [
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        SMCFCFG OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        BSU OFFSET(14) NUMBITS(2) [],
        FB OFFSET(13) NUMBITS(1) [],
        PTM OFFSET(12) NUMBITS(1) [],
        VMIDPNE OFFSET(11) NUMBITS(1) [],
        USFCFG OFFSET(10) NUMBITS(1) [],
        GSE OFFSET(9) NUMBITS(1) [],
        STALLD OFFSET(8) NUMBITS(1) [],
        TRANSIENTCFG OFFSET(6) NUMBITS(2) [],
        GCFGFIE OFFSET(5) NUMBITS(1) [],
        GCFGFRE OFFSET(4) NUMBITS(1) [],
        GFIE OFFSET(2) NUMBITS(1) [],
        GFRE OFFSET(1) NUMBITS(1) [],
        CLIENTPD OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuNscr0W [
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        BSU OFFSET(14) NUMBITS(2) [],
        FB OFFSET(13) NUMBITS(1) [],
        PTM OFFSET(12) NUMBITS(1) [],
        VMIDPNE OFFSET(11) NUMBITS(1) [],
        USFCFG OFFSET(10) NUMBITS(1) [],
        TRANSIENTCFG OFFSET(6) NUMBITS(2) [],
        GFIE OFFSET(2) NUMBITS(1) [],
        GFRE OFFSET(1) NUMBITS(1) [],
        CLIENTPD OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuNsacr [
        CACHE_LOCK OFFSET(26) NUMBITS(1) [],
        DP4K_TBUDISB OFFSET(25) NUMBITS(1) [],
        DP4K_TCUDISB OFFSET(24) NUMBITS(1) [],
        S2CRB_TLBEN OFFSET(10) NUMBITS(1) [],
        MMUDISB_TLBEN OFFSET(9) NUMBITS(1) [],
        SMTNMB_TLBEN OFFSET(8) NUMBITS(1) [],
        IPA2PA_CEN OFFSET(4) NUMBITS(1) [],
        S2WC2EN OFFSET(3) NUMBITS(1) [],
        S1WC2EN OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuNsgfarHigh [
        FADDR OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuNsgfsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        UUT OFFSET(8) NUMBITS(1) [],
        EF OFFSET(6) NUMBITS(1) [],
        CAF OFFSET(5) NUMBITS(1) [],
        UCIF OFFSET(4) NUMBITS(1) [],
        UCBF OFFSET(3) NUMBITS(1) [],
        SMCF OFFSET(2) NUMBITS(1) [],
        USF OFFSET(1) NUMBITS(1) [],
        ICF OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuNsgfsrrestore [
        MULTI OFFSET(31) NUMBITS(1) [],
        UUT OFFSET(8) NUMBITS(1) [],
        EF OFFSET(6) NUMBITS(1) [],
        CAF OFFSET(5) NUMBITS(1) [],
        UCIF OFFSET(4) NUMBITS(1) [],
        UCBF OFFSET(3) NUMBITS(1) [],
        SMCF OFFSET(2) NUMBITS(1) [],
        USF OFFSET(1) NUMBITS(1) [],
        ICF OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuNsgfsynr0R [
        ATS OFFSET(6) NUMBITS(1) [],
        IND OFFSET(3) NUMBITS(1) [],
        PNU OFFSET(2) NUMBITS(1) [],
        WNR OFFSET(1) NUMBITS(1) [],
        NESTED OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuNsgfsynr0W [
        IND OFFSET(3) NUMBITS(1) [],
        PNU OFFSET(2) NUMBITS(1) [],
        WNR OFFSET(1) NUMBITS(1) [],
        NESTED OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuNsgfsyndr1R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuNsgfsyndr1W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuNstlbgstatus [
        GSACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr0 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr1 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr2 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr3 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr4 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr5 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr6 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr7 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr8 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr9 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr10 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr11 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr12 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr13 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr14 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr15 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr16 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr17 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr18 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr19 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr20 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr21 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr22 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr23 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr24 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr25 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr26 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr27 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr28 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr29 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr30 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr31 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr32 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr33 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr34 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr35 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr36 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr37 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr38 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr39 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr40 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr41 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr42 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr43 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr44 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr45 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr46 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuSmr47 [
        VALID OFFSET(31) NUMBITS(1) [],
        MASK OFFSET(16) NUMBITS(15) [],
        ID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr0 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr1 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr2 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr3 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr4 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr5 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr6 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr7 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr8 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr9 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr10 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr11 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr12 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr13 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr14 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr15 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr16 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr17 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr18 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr19 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr20 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr21 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr22 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr23 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr24 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr25 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr26 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr27 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr28 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr29 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr30 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr31 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr32 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr33 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr34 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr35 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr36 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr37 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr38 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr39 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr40 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr41 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr42 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr43 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr44 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr45 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr46 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuS2cr47 [
        TRANSIENTCFG OFFSET(28) NUMBITS(2) [],
        INSTCFG_1 OFFSET(27) NUMBITS(1) [],
        INSTCFG_0_FB OFFSET(26) NUMBITS(1) [],
        PRIVCFG_BSU OFFSET(24) NUMBITS(2) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        NSCFG OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEM_ATTR OFFSET(12) NUMBITS(4) [],
        MTCFG OFFSET(11) NUMBITS(1) [],
        SHCFG OFFSET(8) NUMBITS(2) [],
        CBNDX_VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuPidr4 [
        FOURKB_COUNT OFFSET(4) NUMBITS(4) [],
        JEP106_CONTINUATION_CODE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuPidr0 [
        PARTNUMBER0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuPidr1 [
        JEP106_IDENTITY_CODE OFFSET(4) NUMBITS(4) [],
        PARTNUMBER1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuPidr2 [
        ARCHITECTURE_REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC OFFSET(3) NUMBITS(1) [],
        JEP106_IDENTITY_CODE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuPidr3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        CUSTOMER_MODIFIED OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCidr0 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCidr1 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCidr2 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCidr3 [
        PREAMBLE OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar0R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar0W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar1R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar1W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar2R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar2W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar3R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar3W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar4R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar4W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar5R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar5W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar6R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar6W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar7R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar7W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar8R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar8W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar9R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar9W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar10R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar10W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar11R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar11W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar12R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar12W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar13R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar13W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar14R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar14W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbar15R [
        IRPTNDX OFFSET(24) NUMBITS(8) [],
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ],
    pub SmmuCbar15W [
        WACFG OFFSET(22) NUMBITS(2) [],
        RACFG OFFSET(20) NUMBITS(2) [],
        BSU OFFSET(18) NUMBITS(2) [],
        TYPE OFFSET(16) NUMBITS(2) [],
        MEMATTR_CBNDX_7_4 OFFSET(12) NUMBITS(4) [],
        FB_CBNDX_3 OFFSET(11) NUMBITS(1) [],
        HYPC_CBNDX_2 OFFSET(10) NUMBITS(1) [],
        BPSHCFG_CBNDX_1_0 OFFSET(8) NUMBITS(2) [],
        VMID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra0R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra0W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra1R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra1W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra2R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra2W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra3R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra3W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra4R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra4W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra5R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra5W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra6R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra6W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra7R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra7W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra8R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra8W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra9R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra9W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra10R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra10W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra11R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra11W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra12R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra12W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra13R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra13W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra14R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra14W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCbfrsynra15R [
        SSD_INDEX OFFSET(16) NUMBITS(15) [],
        STREAMID OFFSET(0) NUMBITS(15) [],
    ],
    pub SmmuCbfrsynra15W [
        STREAMID OFFSET(0) NUMBITS(15) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r0 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r1 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r2 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r3 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r4 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r5 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r6 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r7 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r8 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r9 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r10 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r11 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r12 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r13 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r14 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCba2r15 [
        MONC OFFSET(1) NUMBITS(1) [],
        VA64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuItctrl [
        TBU_INDEX OFFSET(4) NUMBITS(3) [],
        MODULE OFFSET(3) NUMBITS(1) [],
        RAM_DATA OFFSET(2) NUMBITS(1) [],
        RAM_MODE OFFSET(1) NUMBITS(1) [],
        INTGMODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuItip [
        SPINDEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuItopGlblR [
        TCU_RAM_DATA OFFSET(16) NUMBITS(4) [],
        GLBLSF1 OFFSET(9) NUMBITS(1) [],
        GLBLNSF1 OFFSET(1) NUMBITS(1) [],
    ],
    pub SmmuItopGlblW [
        TCU_RAM_DATA OFFSET(16) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuItopPerfIndex [
        WAY_IPA2PA_PF OFFSET(30) NUMBITS(2) [],
        IPA2PA_PF_INDEX OFFSET(16) NUMBITS(7) [],
        WAY_MTLB_WC OFFSET(14) NUMBITS(2) [],
        MTLB_WC_INDEX OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuTbuqos0 [
        QOSTBU5 OFFSET(20) NUMBITS(4) [],
        QOSTBU4 OFFSET(16) NUMBITS(4) [],
        QOSTBU3 OFFSET(12) NUMBITS(4) [],
        QOSTBU2 OFFSET(8) NUMBITS(4) [],
        QOSTBU1 OFFSET(4) NUMBITS(4) [],
        QOSTBU0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuPer [
        PER_TCU OFFSET(8) NUMBITS(8) [],
        PER_TBU OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper4 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper5 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper6 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper7 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper8 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper9 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper10 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper11 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper12 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper13 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper14 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper15 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper16 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper17 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper18 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper19 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper20 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper21 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper22 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmevtyper23 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgcr0R [
        CGNC OFFSET(24) NUMBITS(4) [],
        SIDG OFFSET(16) NUMBITS(7) [],
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ],
    pub Pmcgcr0W [
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgcr1R [
        CGNC OFFSET(24) NUMBITS(4) [],
        SIDG OFFSET(16) NUMBITS(7) [],
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ],
    pub Pmcgcr1W [
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgcr2R [
        CGNC OFFSET(24) NUMBITS(4) [],
        SIDG OFFSET(16) NUMBITS(7) [],
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ],
    pub Pmcgcr2W [
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgcr3R [
        CGNC OFFSET(24) NUMBITS(4) [],
        SIDG OFFSET(16) NUMBITS(7) [],
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ],
    pub Pmcgcr3W [
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgcr4R [
        CGNC OFFSET(24) NUMBITS(4) [],
        SIDG OFFSET(16) NUMBITS(7) [],
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ],
    pub Pmcgcr4W [
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgcr5R [
        CGNC OFFSET(24) NUMBITS(4) [],
        SIDG OFFSET(16) NUMBITS(7) [],
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ],
    pub Pmcgcr5W [
        X OFFSET(12) NUMBITS(1) [],
        E OFFSET(11) NUMBITS(1) [],
        CBAEN OFFSET(10) NUMBITS(1) [],
        TCEFCFG OFFSET(8) NUMBITS(2) [],
        NDX OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgsmr0 [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgsmr1 [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgsmr2 [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgsmr3 [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgsmr4 [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcgsmr5 [
        MASK OFFSET(16) NUMBITS(10) [],
        ID OFFSET(0) NUMBITS(10) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcntenset [
        P23 OFFSET(23) NUMBITS(1) [],
        P22 OFFSET(22) NUMBITS(1) [],
        P21 OFFSET(21) NUMBITS(1) [],
        P20 OFFSET(20) NUMBITS(1) [],
        P19 OFFSET(19) NUMBITS(1) [],
        P18 OFFSET(18) NUMBITS(1) [],
        P17 OFFSET(17) NUMBITS(1) [],
        P16 OFFSET(16) NUMBITS(1) [],
        P15 OFFSET(15) NUMBITS(1) [],
        P14 OFFSET(14) NUMBITS(1) [],
        P13 OFFSET(13) NUMBITS(1) [],
        P12 OFFSET(12) NUMBITS(1) [],
        P11 OFFSET(11) NUMBITS(1) [],
        P10 OFFSET(10) NUMBITS(1) [],
        P9 OFFSET(9) NUMBITS(1) [],
        P8 OFFSET(8) NUMBITS(1) [],
        P7 OFFSET(7) NUMBITS(1) [],
        P6 OFFSET(6) NUMBITS(1) [],
        P5 OFFSET(5) NUMBITS(1) [],
        P4 OFFSET(4) NUMBITS(1) [],
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcntenclr [
        P23 OFFSET(23) NUMBITS(1) [],
        P22 OFFSET(22) NUMBITS(1) [],
        P21 OFFSET(21) NUMBITS(1) [],
        P20 OFFSET(20) NUMBITS(1) [],
        P19 OFFSET(19) NUMBITS(1) [],
        P18 OFFSET(18) NUMBITS(1) [],
        P17 OFFSET(17) NUMBITS(1) [],
        P16 OFFSET(16) NUMBITS(1) [],
        P15 OFFSET(15) NUMBITS(1) [],
        P14 OFFSET(14) NUMBITS(1) [],
        P13 OFFSET(13) NUMBITS(1) [],
        P12 OFFSET(12) NUMBITS(1) [],
        P11 OFFSET(11) NUMBITS(1) [],
        P10 OFFSET(10) NUMBITS(1) [],
        P9 OFFSET(9) NUMBITS(1) [],
        P8 OFFSET(8) NUMBITS(1) [],
        P7 OFFSET(7) NUMBITS(1) [],
        P6 OFFSET(6) NUMBITS(1) [],
        P5 OFFSET(5) NUMBITS(1) [],
        P4 OFFSET(4) NUMBITS(1) [],
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmintenset [
        P23 OFFSET(23) NUMBITS(1) [],
        P22 OFFSET(22) NUMBITS(1) [],
        P21 OFFSET(21) NUMBITS(1) [],
        P20 OFFSET(20) NUMBITS(1) [],
        P19 OFFSET(19) NUMBITS(1) [],
        P18 OFFSET(18) NUMBITS(1) [],
        P17 OFFSET(17) NUMBITS(1) [],
        P16 OFFSET(16) NUMBITS(1) [],
        P15 OFFSET(15) NUMBITS(1) [],
        P14 OFFSET(14) NUMBITS(1) [],
        P13 OFFSET(13) NUMBITS(1) [],
        P12 OFFSET(12) NUMBITS(1) [],
        P11 OFFSET(11) NUMBITS(1) [],
        P10 OFFSET(10) NUMBITS(1) [],
        P9 OFFSET(9) NUMBITS(1) [],
        P8 OFFSET(8) NUMBITS(1) [],
        P7 OFFSET(7) NUMBITS(1) [],
        P6 OFFSET(6) NUMBITS(1) [],
        P5 OFFSET(5) NUMBITS(1) [],
        P4 OFFSET(4) NUMBITS(1) [],
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmintenclr [
        P23 OFFSET(23) NUMBITS(1) [],
        P22 OFFSET(22) NUMBITS(1) [],
        P21 OFFSET(21) NUMBITS(1) [],
        P20 OFFSET(20) NUMBITS(1) [],
        P19 OFFSET(19) NUMBITS(1) [],
        P18 OFFSET(18) NUMBITS(1) [],
        P17 OFFSET(17) NUMBITS(1) [],
        P16 OFFSET(16) NUMBITS(1) [],
        P15 OFFSET(15) NUMBITS(1) [],
        P14 OFFSET(14) NUMBITS(1) [],
        P13 OFFSET(13) NUMBITS(1) [],
        P12 OFFSET(12) NUMBITS(1) [],
        P11 OFFSET(11) NUMBITS(1) [],
        P10 OFFSET(10) NUMBITS(1) [],
        P9 OFFSET(9) NUMBITS(1) [],
        P8 OFFSET(8) NUMBITS(1) [],
        P7 OFFSET(7) NUMBITS(1) [],
        P6 OFFSET(6) NUMBITS(1) [],
        P5 OFFSET(5) NUMBITS(1) [],
        P4 OFFSET(4) NUMBITS(1) [],
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmovsclr [
        P23 OFFSET(23) NUMBITS(1) [],
        P22 OFFSET(22) NUMBITS(1) [],
        P21 OFFSET(21) NUMBITS(1) [],
        P20 OFFSET(20) NUMBITS(1) [],
        P19 OFFSET(19) NUMBITS(1) [],
        P18 OFFSET(18) NUMBITS(1) [],
        P17 OFFSET(17) NUMBITS(1) [],
        P16 OFFSET(16) NUMBITS(1) [],
        P15 OFFSET(15) NUMBITS(1) [],
        P14 OFFSET(14) NUMBITS(1) [],
        P13 OFFSET(13) NUMBITS(1) [],
        P12 OFFSET(12) NUMBITS(1) [],
        P11 OFFSET(11) NUMBITS(1) [],
        P10 OFFSET(10) NUMBITS(1) [],
        P9 OFFSET(9) NUMBITS(1) [],
        P8 OFFSET(8) NUMBITS(1) [],
        P7 OFFSET(7) NUMBITS(1) [],
        P6 OFFSET(6) NUMBITS(1) [],
        P5 OFFSET(5) NUMBITS(1) [],
        P4 OFFSET(4) NUMBITS(1) [],
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmovsset [
        P23 OFFSET(23) NUMBITS(1) [],
        P22 OFFSET(22) NUMBITS(1) [],
        P21 OFFSET(21) NUMBITS(1) [],
        P20 OFFSET(20) NUMBITS(1) [],
        P19 OFFSET(19) NUMBITS(1) [],
        P18 OFFSET(18) NUMBITS(1) [],
        P17 OFFSET(17) NUMBITS(1) [],
        P16 OFFSET(16) NUMBITS(1) [],
        P15 OFFSET(15) NUMBITS(1) [],
        P14 OFFSET(14) NUMBITS(1) [],
        P13 OFFSET(13) NUMBITS(1) [],
        P12 OFFSET(12) NUMBITS(1) [],
        P11 OFFSET(11) NUMBITS(1) [],
        P10 OFFSET(10) NUMBITS(1) [],
        P9 OFFSET(9) NUMBITS(1) [],
        P8 OFFSET(8) NUMBITS(1) [],
        P7 OFFSET(7) NUMBITS(1) [],
        P6 OFFSET(6) NUMBITS(1) [],
        P5 OFFSET(5) NUMBITS(1) [],
        P4 OFFSET(4) NUMBITS(1) [],
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmceid0 [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pmdevtype [
        T OFFSET(4) NUMBITS(4) [],
        C OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb0SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb0Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb0Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb0Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb0IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb0PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb0Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb1SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb1Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb1Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb1Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb1IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb1PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb1Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb2SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb2Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb2Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb2Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb2IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb2PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb2Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb3SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb3Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb3Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb3Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb3IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb3PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb3Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb4SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb4Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb4Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb4Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb4IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb4PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb4Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb5SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb5Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb5Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb5Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb5IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb5PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb5Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb6SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb6Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb6Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb6Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb6IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb6PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb6Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb7SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb7Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb7Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb7Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb7IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb7PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb7Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb8SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb8Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb8Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb8Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb8IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb8PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb8Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb9SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb9Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb9Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb9Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb9IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb9PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb9Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb10SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb10Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb10Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb10Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb10IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb10PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb10Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb11SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb11Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb11Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb11Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb11IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb11PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb11Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb12SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb12Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb12Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb12Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb12IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb12PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb12Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb13SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb13Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb13Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb13Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb13IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb13PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb13Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb14SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb14Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb14Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb14Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb14IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb14PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb14Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15SctlrR [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        ASIDPNE OFFSET(12) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb15SctlrW [
        NSCFG OFFSET(28) NUMBITS(2) [],
        WACFG OFFSET(26) NUMBITS(2) [],
        RACFG OFFSET(24) NUMBITS(2) [],
        SHCFG OFFSET(22) NUMBITS(2) [],
        FB OFFSET(21) NUMBITS(1) [],
        MTCFG OFFSET(20) NUMBITS(1) [],
        MEMATTR OFFSET(16) NUMBITS(4) [],
        TRANSIENTCFG OFFSET(14) NUMBITS(2) [],
        PTW OFFSET(13) NUMBITS(1) [],
        UWXN OFFSET(10) NUMBITS(1) [],
        WXN OFFSET(9) NUMBITS(1) [],
        HUPCF OFFSET(8) NUMBITS(1) [],
        CFCFG OFFSET(7) NUMBITS(1) [],
        CFIE OFFSET(6) NUMBITS(1) [],
        CFRE OFFSET(5) NUMBITS(1) [],
        E OFFSET(4) NUMBITS(1) [],
        AFFD OFFSET(3) NUMBITS(1) [],
        AFE OFFSET(2) NUMBITS(1) [],
        TRE OFFSET(1) NUMBITS(1) [],
        M OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Actlr [
        CPRE OFFSET(1) NUMBITS(1) [],
        CMTLB OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Resume [
        TNR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Tcr2R [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        TBI1 OFFSET(6) NUMBITS(1) [],
        TBI0 OFFSET(5) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ],
    pub SmmuCb15Tcr2W [
        NSCFG1 OFFSET(30) NUMBITS(1) [],
        SEP OFFSET(15) NUMBITS(3) [],
        NSCFG0 OFFSET(14) NUMBITS(1) [],
        AS OFFSET(4) NUMBITS(1) [],
        PASIZE OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Ttbr0LowR [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb15Ttbr0LowW [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Ttbr0High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Ttbr1Low [
        ADDRESS_31_7 OFFSET(7) NUMBITS(25) [],
        ADDRESS_6_IRGN0 OFFSET(6) NUMBITS(1) [],
        ADDRESS_5_NOS OFFSET(5) NUMBITS(1) [],
        ADDRESS_4_3_RGN OFFSET(3) NUMBITS(2) [],
        ADDRESS_2 OFFSET(2) NUMBITS(1) [],
        ADDRESS_1_S OFFSET(1) NUMBITS(1) [],
        ADDRESS_0_IRGN1 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Ttbr1High [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15TcrLpae [
        EAE OFFSET(31) NUMBITS(1) [],
        NSCFG1_TG1 OFFSET(30) NUMBITS(1) [],
        SH1 OFFSET(28) NUMBITS(2) [],
        ORGN1 OFFSET(26) NUMBITS(2) [],
        IRGN1 OFFSET(24) NUMBITS(2) [],
        EPD1 OFFSET(23) NUMBITS(1) [],
        A1 OFFSET(22) NUMBITS(1) [],
        T1SZ_5_3 OFFSET(19) NUMBITS(3) [],
        T1SZ_2_0_PASIZE OFFSET(16) NUMBITS(3) [],
        NSCFG0_TG0 OFFSET(14) NUMBITS(1) [],
        SH0 OFFSET(12) NUMBITS(2) [],
        ORGN0 OFFSET(10) NUMBITS(2) [],
        IRGN0 OFFSET(8) NUMBITS(2) [],
        SL0_1_EPD0 OFFSET(7) NUMBITS(1) [],
        SL0_0 OFFSET(6) NUMBITS(1) [],
        PD1_T0SZ_5 OFFSET(5) NUMBITS(1) [],
        S_PD0_T0SZ_4 OFFSET(4) NUMBITS(1) [],
        T0SZ_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Contextidr [
        PROCID OFFSET(8) NUMBITS(24) [],
        ASID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15PrrrMair0 [
        NOS7 OFFSET(31) NUMBITS(1) [],
        NOS6 OFFSET(30) NUMBITS(1) [],
        NOS5 OFFSET(29) NUMBITS(1) [],
        NOS4 OFFSET(28) NUMBITS(1) [],
        NOS3 OFFSET(27) NUMBITS(1) [],
        NOS2 OFFSET(26) NUMBITS(1) [],
        NOS1 OFFSET(25) NUMBITS(1) [],
        NOS0 OFFSET(24) NUMBITS(1) [],
        NS1 OFFSET(19) NUMBITS(1) [],
        NS0 OFFSET(18) NUMBITS(1) [],
        DS1 OFFSET(17) NUMBITS(1) [],
        DS0 OFFSET(16) NUMBITS(1) [],
        TR7 OFFSET(14) NUMBITS(2) [],
        TR6 OFFSET(12) NUMBITS(2) [],
        TR5 OFFSET(10) NUMBITS(2) [],
        TR4 OFFSET(8) NUMBITS(2) [],
        TR3 OFFSET(6) NUMBITS(2) [],
        TR2 OFFSET(4) NUMBITS(2) [],
        TR1 OFFSET(2) NUMBITS(2) [],
        TR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15NmrrMair1 [
        OR7 OFFSET(30) NUMBITS(2) [],
        OR6 OFFSET(28) NUMBITS(2) [],
        OR5 OFFSET(26) NUMBITS(2) [],
        OR4 OFFSET(24) NUMBITS(2) [],
        OR3 OFFSET(22) NUMBITS(2) [],
        OR2 OFFSET(20) NUMBITS(2) [],
        OR1 OFFSET(18) NUMBITS(2) [],
        OR0 OFFSET(16) NUMBITS(2) [],
        IR7 OFFSET(14) NUMBITS(2) [],
        IR6 OFFSET(12) NUMBITS(2) [],
        IR5 OFFSET(10) NUMBITS(2) [],
        IR4 OFFSET(8) NUMBITS(2) [],
        IR3 OFFSET(6) NUMBITS(2) [],
        IR2 OFFSET(4) NUMBITS(2) [],
        IR1 OFFSET(2) NUMBITS(2) [],
        IR0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Fsr [
        MULTI OFFSET(31) NUMBITS(1) [],
        SS OFFSET(30) NUMBITS(1) [],
        FORMAT OFFSET(9) NUMBITS(2) [],
        UUT OFFSET(8) NUMBITS(1) [],
        ASF OFFSET(7) NUMBITS(1) [],
        TLBLKF OFFSET(6) NUMBITS(1) [],
        TLBMCF OFFSET(5) NUMBITS(1) [],
        EF OFFSET(4) NUMBITS(1) [],
        PF OFFSET(3) NUMBITS(1) [],
        AFF OFFSET(2) NUMBITS(1) [],
        TF OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15FarHigh [
        BITS OFFSET(0) NUMBITS(17) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Fsynr0R [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        ATOF OFFSET(9) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ],
    pub SmmuCb15Fsynr0W [
        S1CBNDX OFFSET(16) NUMBITS(4) [],
        AFR OFFSET(11) NUMBITS(1) [],
        PTWF OFFSET(10) NUMBITS(1) [],
        NSATTR OFFSET(8) NUMBITS(1) [],
        IND OFFSET(6) NUMBITS(1) [],
        PNU OFFSET(5) NUMBITS(1) [],
        WNR OFFSET(4) NUMBITS(1) [],
        PLVL OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15IpafarLowR [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
        FAR_RO OFFSET(0) NUMBITS(12) [],
    ],
    pub SmmuCb15IpafarLowW [
        IPAFAR_L OFFSET(12) NUMBITS(20) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15IpafarHigh [
        BITS OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15TlbivaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15TlbivaaHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Tlbiasid [
        ASID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15TlbivalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15TlbivaalHigh [
        ASID OFFSET(16) NUMBITS(16) [],
        ADDRESS OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Tlbiipas2High [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Tlbiipas2lHigh [
        ADDRESS OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Tlbstatus [
        SACTIVE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmevtyper0 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmevtyper1 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmevtyper2 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmevtyper3 [
        P OFFSET(31) NUMBITS(1) [],
        U OFFSET(30) NUMBITS(1) [],
        NSP OFFSET(29) NUMBITS(1) [],
        NSU OFFSET(28) NUMBITS(1) [],
        EVENT OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmcfgr [
        NCG OFFSET(24) NUMBITS(8) [],
        UEN OFFSET(19) NUMBITS(1) [],
        EX OFFSET(16) NUMBITS(1) [],
        CCD OFFSET(15) NUMBITS(1) [],
        CC OFFSET(14) NUMBITS(1) [],
        SIZE OFFSET(8) NUMBITS(6) [],
        N OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15PmcrR [
        IMP OFFSET(24) NUMBITS(8) [],
        X OFFSET(4) NUMBITS(1) [],
        P OFFSET(1) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ],
    pub SmmuCb15PmcrW [
        X OFFSET(4) NUMBITS(1) [],
        E OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmceid [
        EVENT0X12 OFFSET(17) NUMBITS(1) [],
        EVENT0X11 OFFSET(16) NUMBITS(1) [],
        EVENT0X10 OFFSET(15) NUMBITS(1) [],
        EVENT0X0A OFFSET(9) NUMBITS(1) [],
        EVENT0X09 OFFSET(8) NUMBITS(1) [],
        EVENT0X08 OFFSET(7) NUMBITS(1) [],
        EVENT0X01 OFFSET(1) NUMBITS(1) [],
        EVENT0X00 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmcntense [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmcntenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmcntenset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmintenclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmovsclr [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmovsset [
        P3 OFFSET(3) NUMBITS(1) [],
        P2 OFFSET(2) NUMBITS(1) [],
        P1 OFFSET(1) NUMBITS(1) [],
        P0 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub SmmuCb15Pmauthstatus [
        SNI OFFSET(7) NUMBITS(1) [],
        SNE OFFSET(6) NUMBITS(1) [],
        SI OFFSET(5) NUMBITS(1) [],
        SE OFFSET(4) NUMBITS(1) [],
        NSNI OFFSET(3) NUMBITS(1) [],
        NSNE OFFSET(2) NUMBITS(1) [],
        NSI OFFSET(1) NUMBITS(1) [],
        NSE OFFSET(0) NUMBITS(1) [],
    ]
];

// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
/// RPU GIC Interrupt Controller, RPU GIC Interrupt Controller; GICv1
pub static mut RCPU_GIC: *mut Registers = 0xf9000000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Interrupt Control Register (ICDICR)
    pub enable_enable: ReadWrite<u32>,
    /// Provides information about the configuration of the GIC.
    pub enable_ic_type: ReadOnly<u32, EnableIcType::Register>,
    /// Provides information about the implementor of the Distributor andthe revision of the GIC.
    pub enable_dist_ident: ReadOnly<u32, EnableDistIdent::Register>,
    _padding12: [u8; 116],
    /// Interrupt Security Registers (ICDISR)
    pub enable_sgi_security_if_n: ReadWrite<u16>,
    _padding130: [u8; 2],
    /// Interrupt Security Registers (ICDISR)
    pub enable_spi_security0: ReadWrite<u32>,
    /// Interrupt Security Registers (ICDISR)
    pub enable_spi_security1: ReadWrite<u32>,
    /// Interrupt Security Registers (ICDISR)
    pub enable_spi_security2: ReadWrite<u32>,
    /// Interrupt Security Registers (ICDISR)
    pub enable_spi_security3: ReadWrite<u32>,
    /// Interrupt Security Registers (ICDISR)
    pub enable_spi_security4: ReadWrite<u32>,
    _padding152: [u8; 108],
    /// Interrupt Set-Enable Registers (ICDISER)
    pub enable_spi_enable_set0: ReadWrite<u32>,
    /// Interrupt Set-Enable Registers (ICDISER)
    pub enable_spi_enable_set1: ReadWrite<u32>,
    /// Interrupt Set-Enable Registers (ICDISER)
    pub enable_spi_enable_set2: ReadWrite<u32>,
    /// Interrupt Set-Enable Registers (ICDISER)
    pub enable_spi_enable_set3: ReadWrite<u32>,
    /// Interrupt Set-Enable Registers (ICDISER)
    pub enable_spi_enable_set4: ReadWrite<u32>,
    _padding280: [u8; 108],
    /// Interrupt Clear-Enable Registers (ICDICER)
    pub enable_spi_enable_clr0: ReadWrite<u32>,
    /// Interrupt Clear-Enable Registers (ICDICER)
    pub enable_spi_enable_clr1: ReadWrite<u32>,
    /// Interrupt Clear-Enable Registers (ICDICER)
    pub enable_spi_enable_clr2: ReadWrite<u32>,
    /// Interrupt Clear-Enable Registers (ICDICER)
    pub enable_spi_enable_clr3: ReadWrite<u32>,
    /// Interrupt Clear-Enable Registers (ICDICER)
    pub enable_spi_enable_clr4: ReadWrite<u32>,
    _padding408: [u8; 104],
    /// Interrupt Set-Pending Registers (ICDISPR)
    pub enable_sgi_pending_set_if_n: ReadOnly<u16>,
    _padding514: [u8; 2],
    /// Interrupt Set-Pending Registers (ICDISPR)
    pub enable_spi_pending_set0: ReadWrite<u32>,
    /// Interrupt Set-Pending Registers (ICDISPR)
    pub enable_spi_pending_set1: ReadWrite<u32>,
    /// Interrupt Set-Pending Registers (ICDISPR)
    pub enable_spi_pending_set2: ReadWrite<u32>,
    /// Interrupt Set-Pending Registers (ICDISPR)
    pub enable_spi_pending_set3: ReadWrite<u32>,
    /// Interrupt Set-Pending Registers (ICDISPR)
    pub enable_spi_pending_set4: ReadWrite<u32>,
    _padding536: [u8; 104],
    /// Pending Clear Register (ICDICPR)
    pub enable_sgi_pending_clr_if_n: ReadOnly<u16>,
    _padding642: [u8; 2],
    /// Pending Clear Register (ICDICPR)
    pub enable_spi_pending_clr0: ReadWrite<u32>,
    /// Pending Clear Register (ICDICPR)
    pub enable_spi_pending_clr1: ReadWrite<u32>,
    /// Pending Clear Register (ICDICPR)
    pub enable_spi_pending_clr2: ReadWrite<u32>,
    /// Pending Clear Register (ICDICPR)
    pub enable_spi_pending_clr3: ReadWrite<u32>,
    /// Pending Clear Register (ICDICPR)
    pub enable_spi_pending_clr4: ReadWrite<u32>,
    _padding664: [u8; 104],
    /// Active Bit Registers (ICDABR)
    pub enable_sgi_active_if_n: ReadOnly<u16>,
    _padding770: [u8; 2],
    /// Active Bit Registers (ICDABR)
    pub enable_spi_active0: ReadOnly<u32>,
    /// Active Bit Registers (ICDABR)
    pub enable_spi_active1: ReadOnly<u32>,
    /// Active Bit Registers (ICDABR)
    pub enable_spi_active2: ReadOnly<u32>,
    /// Active Bit Registers (ICDABR)
    pub enable_spi_active3: ReadOnly<u32>,
    /// Active Bit Registers (ICDABR)
    pub enable_spi_active4: ReadOnly<u32>,
    _padding792: [u8; 232],
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n0: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n1: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n2: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n3: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n4: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n5: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n6: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n7: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n8: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n9: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n10: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n11: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n12: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n13: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n14: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_sgi_intid_if_n15: ReadWrite<u8>,
    _padding1040: [u8; 16],
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid0: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid1: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid2: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid3: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid4: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid5: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid6: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid7: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid8: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid9: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid10: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid11: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid12: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid13: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid14: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid15: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid16: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid17: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid18: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid19: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid20: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid21: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid22: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid23: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid24: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid25: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid26: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid27: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid28: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid29: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid30: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid31: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid32: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid33: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid34: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid35: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid36: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid37: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid38: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid39: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid40: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid41: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid42: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid43: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid44: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid45: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid46: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid47: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid48: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid49: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid50: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid51: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid52: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid53: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid54: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid55: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid56: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid57: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid58: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid59: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid60: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid61: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid62: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid63: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid64: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid65: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid66: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid67: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid68: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid69: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid70: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid71: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid72: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid73: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid74: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid75: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid76: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid77: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid78: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid79: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid80: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid81: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid82: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid83: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid84: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid85: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid86: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid87: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid88: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid89: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid90: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid91: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid92: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid93: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid94: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid95: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid96: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid97: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid98: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid99: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid100: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid101: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid102: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid103: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid104: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid105: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid106: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid107: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid108: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid109: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid110: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid111: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid112: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid113: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid114: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid115: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid116: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid117: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid118: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid119: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid120: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid121: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid122: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid123: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid124: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid125: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid126: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid127: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid128: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid129: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid130: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid131: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid132: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid133: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid134: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid135: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid136: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid137: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid138: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid139: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid140: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid141: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid142: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid143: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid144: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid145: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid146: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid147: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid148: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid149: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid150: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid151: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid152: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid153: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid154: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid155: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid156: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid157: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid158: ReadWrite<u8>,
    /// Interrupt Priority Registers (ICDIPR)
    pub enable_priority_spi_intid159: ReadWrite<u8>,
    _padding1216: [u8; 864],
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid0: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid1: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid2: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid3: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid4: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid5: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid6: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid7: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid8: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid9: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid10: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid11: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid12: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid13: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid14: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid15: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid16: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid17: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid18: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid19: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid20: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid21: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid22: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid23: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid24: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid25: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid26: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid27: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid28: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid29: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid30: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid31: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid32: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid33: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid34: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid35: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid36: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid37: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid38: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid39: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid40: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid41: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid42: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid43: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid44: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid45: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid46: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid47: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid48: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid49: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid50: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid51: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid52: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid53: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid54: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid55: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid56: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid57: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid58: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid59: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid60: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid61: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid62: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid63: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid64: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid65: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid66: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid67: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid68: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid69: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid70: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid71: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid72: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid73: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid74: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid75: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid76: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid77: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid78: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid79: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid80: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid81: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid82: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid83: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid84: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid85: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid86: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid87: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid88: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid89: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid90: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid91: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid92: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid93: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid94: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid95: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid96: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid97: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid98: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid99: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid100: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid101: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid102: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid103: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid104: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid105: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid106: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid107: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid108: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid109: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid110: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid111: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid112: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid113: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid114: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid115: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid116: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid117: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid118: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid119: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid120: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid121: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid122: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid123: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid124: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid125: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid126: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid127: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid128: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid129: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid130: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid131: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid132: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid133: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid134: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid135: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid136: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid137: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid138: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid139: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid140: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid141: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid142: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid143: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid144: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid145: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid146: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid147: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid148: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid149: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid150: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid151: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid152: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid153: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid154: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid155: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid156: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid157: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid158: ReadWrite<u8>,
    /// Interrupt Processor Targets Registers (ICDIPTR)
    pub enable_targets_spi_intid159: ReadWrite<u8>,
    _padding2240: [u8; 840],
    /// Interrupt Configuration Register (ICDICR)
    pub enable_spi_config0: ReadWrite<u32>,
    /// Interrupt Configuration Register (ICDICR)
    pub enable_spi_config1: ReadWrite<u32>,
    /// Interrupt Configuration Register (ICDICR)
    pub enable_spi_config2: ReadWrite<u32>,
    /// Interrupt Configuration Register (ICDICR)
    pub enable_spi_config3: ReadWrite<u32>,
    /// Interrupt Configuration Register (ICDICR)
    pub enable_spi_config4: ReadWrite<u32>,
    _padding3100: [u8; 232],
    /// Each bit provides the status of the SPI[987:0] inputs.
    pub enable_spi: ReadOnly<u32>,
    _padding3336: [u8; 204],
    /// Enables an external AMBA master to access the status of the:legacy_nirq_c<n> and legacy_nfiq_c<n> inputs for CPUInterface <n>,cfgsdisable tie-off signal.
    pub enable_legacy_int_n: ReadOnly<u32, EnableLegacyIntN::Register>,
    _padding3544: [u8; 8],
    /// Returns the status of the match_d<n> tie-off signals for CPUInterface <n>.
    pub enable_match_d_n: ReadOnly<u32>,
    /// Returns the status of the enable_d<n> tie-off signals for CPUInterface <n>.
    pub enable_enable_d_n: ReadOnly<u32>,
    _padding3560: [u8; 280],
    /// Software Generated Interrupt Register (ICDSGIR)
    pub enable_sgi_control: WriteOnly<u32>,
    _padding3844: [u8; 188],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub enable_periph_id_8: ReadOnly<u8, EnablePeriphId8::Register>,
    _padding4033: [u8; 15],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub enable_periph_id_0: ReadOnly<u8, EnablePeriphId0::Register>,
    _padding4049: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub enable_periph_id_1: ReadOnly<u8, EnablePeriphId1::Register>,
    _padding4053: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub enable_periph_id_2: ReadOnly<u8, EnablePeriphId2::Register>,
    _padding4057: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub enable_periph_id_3: ReadOnly<u8, EnablePeriphId3::Register>,
    _padding4061: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub enable_periph_id_4: ReadOnly<u8, EnablePeriphId4::Register>,
    _padding4065: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub enable_periph_id_5: ReadOnly<u8, EnablePeriphId5::Register>,
    _padding4069: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub enable_periph_id_6: ReadOnly<u8, EnablePeriphId6::Register>,
    _padding4073: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub enable_periph_id_7: ReadOnly<u8, EnablePeriphId7::Register>,
    _padding4077: [u8; 3],
    /// The component_id_[3:0] Registers are four eight-bit wide registers,that can conceptually be treated as a single register that holds a32-bit PrimeCell ID value.
    pub enable_component_id_0: ReadOnly<u8, EnableComponentId0::Register>,
    _padding4081: [u8; 3],
    /// The component_id_[3:0] Registers are four eight-bit wide registers,that can conceptually be treated as a single register that holds a32-bit PrimeCell ID value.
    pub enable_component_id_1: ReadOnly<u8, EnableComponentId1::Register>,
    _padding4085: [u8; 3],
    /// The component_id_[3:0] Registers are four eight-bit wide registers,that can conceptually be treated as a single register that holds a32-bit PrimeCell ID value.
    pub enable_component_id_2: ReadOnly<u8, EnableComponentId2::Register>,
    _padding4089: [u8; 3],
    /// The component_id_[3:0] Registers are four eight-bit wide registers,that can conceptually be treated as a single register that holds a32-bit PrimeCell ID value.
    pub enable_component_id_3: ReadOnly<u8, EnableComponentId3::Register>,
    _padding4093: [u8; 3],
    /// CPU Interface Control Register (ICCICR)
    pub control_n_control_n: ReadWrite<u32>,
    /// Priority Mask Register (ICCIPMR)
    pub control_n_pri_msk_c_n: ReadWrite<u32>,
    /// Binary Point Register (ICCBPR)
    pub control_n_bp_c_n: ReadWrite<u32>,
    /// Interrupt Acknowledge Register (ICCIAR)
    pub control_n_int_ack_n: ReadOnly<u32>,
    /// End of Interrupt Register (ICCEOIR)
    pub control_n_eoi_n: WriteOnly<u32>,
    /// Running Priority Register (ICCRPR)
    pub control_n_run_priority_n: ReadOnly<u32>,
    /// Highest Pending Interrupt Register (ICCHPIR)
    pub control_n_hi_pend_n: ReadOnly<u32>,
    /// Aliased Binary Point Register (ICCABPR)
    pub control_n_alias_nsbp_c_n: ReadWrite<u32>,
    _padding4128: [u8; 32],
    /// Enables the integration test logic to modify the status of thenfiq_c<n> and nirq_c<n> signals.
    pub control_n_integ_en_c_n: ReadWrite<u32, ControlNIntegEnCN::Register>,
    /// Enables the integration test logic to modify the status of thenfiq_c<n> and nirq_c<n> signals.
    pub control_n_interrupt_out_n: ReadWrite<u32, ControlNInterruptOutN::Register>,
    _padding4168: [u8; 8],
    /// Returns the status of the match_c<n> tie-off signals for CPUInterface <n>.
    pub control_n_match_c_n: ReadOnly<u32>,
    /// Returns the status of the enable_c<n> tie-off signals for CPUInterface <n>.
    pub control_n_enable_c_n: ReadOnly<u32>,
    _padding4184: [u8; 164],
    /// Returns the status of the enable_c<n> tie-off signals for CPUInterface <n>.
    pub control_n_cpu_if_ident: ReadOnly<u32, ControlNCpuIfIdent::Register>,
    _padding4352: [u8; 3776],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub control_n_periph_id_8: ReadOnly<u8, ControlNPeriphId8::Register>,
    _padding8129: [u8; 15],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub control_n_periph_id_0: ReadOnly<u8, ControlNPeriphId0::Register>,
    _padding8145: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub control_n_periph_id_1: ReadOnly<u8, ControlNPeriphId1::Register>,
    _padding8149: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub control_n_periph_id_2: ReadOnly<u8, ControlNPeriphId2::Register>,
    _padding8153: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub control_n_periph_id_3: ReadOnly<u8, ControlNPeriphId3::Register>,
    _padding8157: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub control_n_periph_id_4: ReadOnly<u8, ControlNPeriphId4::Register>,
    _padding8161: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub control_n_periph_id_5: ReadOnly<u8, ControlNPeriphId5::Register>,
    _padding8165: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub control_n_periph_id_6: ReadOnly<u8, ControlNPeriphId6::Register>,
    _padding8169: [u8; 3],
    /// The periph_id_[8:0] Registers provide information about theconfiguration of the peripheral. Note some fields span acrossadjacent registers.
    pub control_n_periph_id_7: ReadOnly<u8, ControlNPeriphId7::Register>,
    _padding8173: [u8; 3],
    /// The component_id_[3:0] Registers are four eight-bit wide registers,that can conceptually be treated as a single register that holds a32-bit PrimeCell ID value.
    pub control_n_component_id_0: ReadOnly<u8, ControlNComponentId0::Register>,
    _padding8177: [u8; 3],
    /// The component_id_[3:0] Registers are four eight-bit wide registers,that can conceptually be treated as a single register that holds a32-bit PrimeCell ID value.
    pub control_n_component_id_1: ReadOnly<u8, ControlNComponentId1::Register>,
    _padding8181: [u8; 3],
    /// The component_id_[3:0] Registers are four eight-bit wide registers,that can conceptually be treated as a single register that holds a32-bit PrimeCell ID value.
    pub control_n_component_id_2: ReadOnly<u8, ControlNComponentId2::Register>,
    _padding8185: [u8; 3],
    /// The component_id_[3:0] Registers are four eight-bit wide registers,that can conceptually be treated as a single register that holds a32-bit PrimeCell ID value.
    pub control_n_component_id_3: ReadOnly<u8, ControlNComponentId3::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub EnableIcType [
        LSPI OFFSET(11) NUMBITS(5) [],
        TZ OFFSET(10) NUMBITS(1) [],
        CPU_NUMBER OFFSET(5) NUMBITS(3) [],
        IT_LINES_NUMBER OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EnableDistIdent [
        IMPL_VER OFFSET(24) NUMBITS(8) [],
        REV_NUM OFFSET(12) NUMBITS(12) [],
        IMPLEMENTOR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub EnableLegacyIntN [
        CFGSDISABLE OFFSET(2) NUMBITS(1) [],
        LEGACY_NFIQ_IF_N OFFSET(1) NUMBITS(1) [],
        LEGACY_NIRQ_IF_N OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnablePeriphId8 [
        IDENTIFIER OFFSET(7) NUMBITS(1) [],
        IF_TYPE OFFSET(5) NUMBITS(2) [],
        CPU_IF OFFSET(2) NUMBITS(3) [],
        FIQ_LEGACY OFFSET(1) NUMBITS(1) [],
        IRQ_LEGACY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnablePeriphId0 [
        PART_NUMBER_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnablePeriphId1 [
        JEP106_ID_3_0 OFFSET(4) NUMBITS(4) [],
        PART_NUMBER_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnablePeriphId2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC_USED OFFSET(3) NUMBITS(1) [],
        JEP106_ID_6_4 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnablePeriphId3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        MOD_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnablePeriphId4 [
        FOURKB_COUNT OFFSET(4) NUMBITS(4) [],
        JEP106_C_CODE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnablePeriphId5 [
        PPI_NUMBER_0 OFFSET(5) NUMBITS(3) [],
        SGI_NUMBER OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnablePeriphId6 [
        SPI_NUMBER_0 OFFSET(2) NUMBITS(6) [],
        PPI_NUMBER_1 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnablePeriphId7 [
        TZ OFFSET(7) NUMBITS(1) [],
        PRIORITY OFFSET(4) NUMBITS(3) [],
        SPI_NUMBER_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnableComponentId0 [
        COMPONENT_ID_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnableComponentId1 [
        COMPONENT_ID_1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnableComponentId2 [
        COMPONENT_ID_2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub EnableComponentId3 [
        COMPONENT_ID_3 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ControlNIntegEnCN [
        INTEG_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ControlNInterruptOutN [
        SET_NFIQ_C OFFSET(1) NUMBITS(1) [],
        SET_NIRQ_C OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ControlNCpuIfIdent [
        PART_NUM OFFSET(20) NUMBITS(12) [],
        ARCH_NUM OFFSET(16) NUMBITS(4) [],
        REV_NUM OFFSET(12) NUMBITS(4) [],
        IMPLEMENTOR OFFSET(0) NUMBITS(12) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNPeriphId8 [
        IDENTIFIER OFFSET(7) NUMBITS(1) [],
        IF_TYPE OFFSET(5) NUMBITS(2) [],
        CPU_IF OFFSET(2) NUMBITS(3) [],
        FIQ_LEGACY OFFSET(1) NUMBITS(1) [],
        IRQ_LEGACY OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNPeriphId0 [
        PART_NUMBER_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNPeriphId1 [
        JEP106_ID_3_0 OFFSET(4) NUMBITS(4) [],
        PART_NUMBER_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNPeriphId2 [
        REVISION OFFSET(4) NUMBITS(4) [],
        JEDEC_USED OFFSET(3) NUMBITS(1) [],
        JEP106_ID_6_4 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNPeriphId3 [
        REVAND OFFSET(4) NUMBITS(4) [],
        MOD_NUMBER OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNPeriphId4 [
        FOURKB_COUNT OFFSET(4) NUMBITS(4) [],
        JEP106_C_CODE OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNPeriphId5 [
        PPI_NUMBER_0 OFFSET(5) NUMBITS(3) [],
        SGI_NUMBER OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNPeriphId6 [
        SPI_NUMBER_0 OFFSET(2) NUMBITS(6) [],
        PPI_NUMBER_1 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNPeriphId7 [
        TZ OFFSET(7) NUMBITS(1) [],
        PRIORITY OFFSET(4) NUMBITS(3) [],
        SPI_NUMBER_1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNComponentId0 [
        COMPONENT_ID_0 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNComponentId1 [
        COMPONENT_ID_1 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNComponentId2 [
        COMPONENT_ID_2 OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u8,
    pub ControlNComponentId3 [
        COMPONENT_ID_3 OFFSET(0) NUMBITS(8) [],
    ]
];

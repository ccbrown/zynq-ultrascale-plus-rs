// This file was automatically generated. Don't edit it directly!
use tock_registers::registers::{Aliased, ReadOnly, ReadWrite};
/// USB Extensible Host Controller Interface, USB Port 0 XHCI
pub static mut USB3_0_XHCI: *mut Registers = 0xfe200000 as *mut Registers;
/// USB Extensible Host Controller Interface, USB Port 1 XHCI
pub static mut USB3_1_XHCI: *mut Registers = 0xfe300000 as *mut Registers;
#[repr(C)]
pub struct Registers {
    /// Capability Registers LengthHost Controller Operational Registers = Base address + CAPLENGTHwhere CAPLENGTH is `DWC_USB3_HOST_CAP_REG_LEN whose default value is 20h.
    pub caplength: ReadOnly<u32, Caplength::Register>,
    /// Structural Parameters 1 RegisterFor register definitions, refer to the xHCI specification.
    pub hcsparams1: ReadOnly<u32, Hcsparams1::Register>,
    /// Structural Parameters 2 RegisterFor register definitions, refer to the xHCI specification.
    pub hcsparams2: ReadOnly<u32, Hcsparams2::Register>,
    /// Structural Parameters 3 RegisterFor register definitions, refer to the xHCI specification.
    pub hcsparams3: ReadOnly<u32, Hcsparams3::Register>,
    /// Capability Parameters 1 RegisterFor register definitions, refer to the xHCI specification.
    pub hccparams1: ReadOnly<u32, Hccparams1::Register>,
    /// Doorbell Offset RegisterFor register definitions, refer to the xHCI specification.
    pub dboff: ReadOnly<u32, Dboff::Register>,
    /// Runtime Register Space Offset Register
    pub rtsoff: ReadOnly<u32, Rtsoff::Register>,
    /// Host Controller Capability Parameters 2For register definitions, refer to the xHCI specification.
    pub hccparams2: ReadOnly<u32, Hccparams2::Register>,
    /// USB Command RegisterFor a description of this standard USB register field see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub usbcmd: Aliased<u32, UsbcmdR::Register, UsbcmdW::Register>,
    /// USB Status Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub usbsts: Aliased<u32, UsbstsR::Register, UsbstsW::Register>,
    /// Page Size Register Bit DefinitionsUse this register to enable or disable the reporting of specific USB Device Notification Transaction Packets being received.A Notification Enable (Nx, where x = 0 to 15) flag is defined for each of the 16 possible device notification types. If a flag is set for a specific notification type, a Device Notification Event is generated when the respective notification packet is received. After reset, all notifications are disabled.This register is written as a Dword. Byte writes produce undefined results.
    pub pagesize: ReadOnly<u32, Pagesize::Register>,
    _padding44: [u8; 8],
    /// Device Notification Register Bit DefinitionsFor a description of this standard USB register field see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub dnctrl: Aliased<u32, DnctrlR::Register, DnctrlW::Register>,
    /// CRCR_LOFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub crcr_lo: Aliased<u32, CrcrLoR::Register, CrcrLoW::Register>,
    /// CRCR_HI
    pub crcr_hi: ReadWrite<u32>,
    _padding64: [u8; 16],
    /// DCBAAP_LO
    pub dcbaap_lo: Aliased<u32, DcbaapLoR::Register, DcbaapLoW::Register>,
    /// DCBAAP_HIFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub dcbaap_hi: ReadWrite<u32>,
    /// Configure Register Bit DefinitionsThis register is in the Aux Power well. It is only reset by platform hardware during a cold reset or in response to a Host Controller Reset (HCRST).
    pub config: Aliased<u32, ConfigR::Register, ConfigW::Register>,
    _padding92: [u8; 964],
    /// Port Status and Control Register Bit DefinitionsThe PORTSC Register Access fails (Timeout) if the UTMI/ULPI clock is not running or one of the following bits is asserted.- PR- ORC
    pub portsc_20: Aliased<u32, Portsc20R::Register, Portsc20W::Register>,
    /// USB3 Port Power Management Status and Control Register Bit DefinitionsThis register is in the Aux Power well. It is only reset by platform hardware during a cold reset or in response to a Host Controller Reset (HCRST).Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub portpmsc_20: Aliased<u32, Portpmsc20R::Register, Portpmsc20W::Register>,
    /// Port Link Info RegisterProgramming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub portli_20: ReadOnly<u32, Portli20::Register>,
    /// USB2 Port Hardware LPM Control Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub porthlpmc_20: Aliased<u32, Porthlpmc20R::Register, Porthlpmc20W::Register>,
    /// Port Status and Control Register Bit DefinitionsThe PORTSC Register Access fails (Timeout) if the UTMI/ULPI clock is not running or one of the following bits is asserted.- PR- ORC- WPR
    pub portsc_30: Aliased<u32, Portsc30R::Register, Portsc30W::Register>,
    /// USB3 Port Power Management Status and Control Register Bit DefinitionsThis register is in the Aux Power well. It is only reset by platform hardware during a cold reset or in response to a Host Controller Reset (HCRST).Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub portpmsc_30: Aliased<u32, Portpmsc30R::Register, Portpmsc30W::Register>,
    /// Port Link Info RegisterProgramming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub portli_30: ReadOnly<u32, Portli30::Register>,
    /// USB2 Port Hardware LPM Control Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub porthlpmc_30: ReadOnly<u32>,
    /// Microframe Index Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub mfindex: ReadOnly<u32, Mfindex::Register>,
    /// RsvdZFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub rsvdz: ReadOnly<u32>,
    _padding1096: [u8; 24],
    /// Interrupter Management Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 0 of an array of 4.
    pub iman_0: Aliased<u32, Iman0R::Register, Iman0W::Register>,
    /// Interrupter Moderation RegisterThe software may use this register to pace (or even out) the delivery of interrupts to the host CPU.This register provides a guaranteed inter-interrupt delay between interrupts asserted by the xHC, regardless of USB traffic conditions.To independently validate configuration settings, software may use the following algorithm to convert the inter-interrupt Interval value to the common interrupts/sec performance metric.Instance 0 of an array of 4.
    pub imod_0: ReadWrite<u32, Imod0::Register>,
    /// Event Ring Segment Table Size Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 0 of an array of 4.
    pub erstsz_0: Aliased<u32, Erstsz0R::Register, Erstsz0W::Register>,
    /// RsvdP Instance 0 of an array of 4.
    pub rsvdp_0: ReadOnly<u32>,
    /// ERSTBA_LO Instance 0 of an array of 4.
    pub erstba_lo_0: Aliased<u32, ErstbaLo0R::Register, ErstbaLo0W::Register>,
    /// ERSTBA_HI Instance 0 of an array of 4.
    pub erstba_hi_0: ReadWrite<u32>,
    /// ERDP_LOFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 0 of an array of 4.
    pub erdp_lo_0: ReadWrite<u32, ErdpLo0::Register>,
    /// ERDP_HI Instance 0 of an array of 4.
    pub erdp_hi_0: ReadWrite<u32>,
    /// Interrupter Management Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 1 of an array of 4.
    pub iman_1: Aliased<u32, Iman1R::Register, Iman1W::Register>,
    /// Interrupter Moderation RegisterThe software may use this register to pace (or even out) the delivery of interrupts to the host CPU.This register provides a guaranteed inter-interrupt delay between interrupts asserted by the xHC, regardless of USB traffic conditions.To independently validate configuration settings, software may use the following algorithm to convert the inter-interrupt Interval value to the common interrupts/sec performance metric.Instance 1 of an array of 4.
    pub imod_1: ReadWrite<u32, Imod1::Register>,
    /// Event Ring Segment Table Size Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 1 of an array of 4.
    pub erstsz_1: Aliased<u32, Erstsz1R::Register, Erstsz1W::Register>,
    /// RsvdP Instance 1 of an array of 4.
    pub rsvdp_1: ReadOnly<u32>,
    /// ERSTBA_LO Instance 1 of an array of 4.
    pub erstba_lo_1: Aliased<u32, ErstbaLo1R::Register, ErstbaLo1W::Register>,
    /// ERSTBA_HI Instance 1 of an array of 4.
    pub erstba_hi_1: ReadWrite<u32>,
    /// ERDP_LOFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 1 of an array of 4.
    pub erdp_lo_1: ReadWrite<u32, ErdpLo1::Register>,
    /// ERDP_HI Instance 1 of an array of 4.
    pub erdp_hi_1: ReadWrite<u32>,
    /// Interrupter Management Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 2 of an array of 4.
    pub iman_2: Aliased<u32, Iman2R::Register, Iman2W::Register>,
    /// Interrupter Moderation RegisterThe software may use this register to pace (or even out) the delivery of interrupts to the host CPU.This register provides a guaranteed inter-interrupt delay between interrupts asserted by the xHC, regardless of USB traffic conditions.To independently validate configuration settings, software may use the following algorithm to convert the inter-interrupt Interval value to the common interrupts/sec performance metric.Instance 2 of an array of 4.
    pub imod_2: ReadWrite<u32, Imod2::Register>,
    /// Event Ring Segment Table Size Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 2 of an array of 4.
    pub erstsz_2: Aliased<u32, Erstsz2R::Register, Erstsz2W::Register>,
    /// RsvdP Instance 2 of an array of 4.
    pub rsvdp_2: ReadOnly<u32>,
    /// ERSTBA_LO Instance 2 of an array of 4.
    pub erstba_lo_2: Aliased<u32, ErstbaLo2R::Register, ErstbaLo2W::Register>,
    /// ERSTBA_HI Instance 2 of an array of 4.
    pub erstba_hi_2: ReadWrite<u32>,
    /// ERDP_LOFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 2 of an array of 4.
    pub erdp_lo_2: ReadWrite<u32, ErdpLo2::Register>,
    /// ERDP_HI Instance 2 of an array of 4.
    pub erdp_hi_2: ReadWrite<u32>,
    /// Interrupter Management Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 3 of an array of 4.
    pub iman_3: Aliased<u32, Iman3R::Register, Iman3W::Register>,
    /// Interrupter Moderation RegisterThe software may use this register to pace (or even out) the delivery of interrupts to the host CPU.This register provides a guaranteed inter-interrupt delay between interrupts asserted by the xHC, regardless of USB traffic conditions.To independently validate configuration settings, software may use the following algorithm to convert the inter-interrupt Interval value to the common interrupts/sec performance metric.Instance 3 of an array of 4.
    pub imod_3: ReadWrite<u32, Imod3::Register>,
    /// Event Ring Segment Table Size Register Bit DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 3 of an array of 4.
    pub erstsz_3: Aliased<u32, Erstsz3R::Register, Erstsz3W::Register>,
    /// RsvdP Instance 3 of an array of 4.
    pub rsvdp_3: ReadOnly<u32>,
    /// ERSTBA_LO Instance 3 of an array of 4.
    pub erstba_lo_3: Aliased<u32, ErstbaLo3R::Register, ErstbaLo3W::Register>,
    /// ERSTBA_HI Instance 3 of an array of 4.
    pub erstba_hi_3: ReadWrite<u32>,
    /// ERDP_LOFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Instance 3 of an array of 4.
    pub erdp_lo_3: ReadWrite<u32, ErdpLo3::Register>,
    /// ERDP_HI Instance 3 of an array of 4.
    pub erdp_hi_3: ReadWrite<u32>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db0: Aliased<u32, Db0R::Register, Db0W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db1: Aliased<u32, Db1R::Register, Db1W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db2: Aliased<u32, Db2R::Register, Db2W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db3: Aliased<u32, Db3R::Register, Db3W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db4: Aliased<u32, Db4R::Register, Db4W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db5: Aliased<u32, Db5R::Register, Db5W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db6: Aliased<u32, Db6R::Register, Db6W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db7: Aliased<u32, Db7R::Register, Db7W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db8: Aliased<u32, Db8R::Register, Db8W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db9: Aliased<u32, Db9R::Register, Db9W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db10: Aliased<u32, Db10R::Register, Db10W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db11: Aliased<u32, Db11R::Register, Db11W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db12: Aliased<u32, Db12R::Register, Db12W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db13: Aliased<u32, Db13R::Register, Db13W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db14: Aliased<u32, Db14R::Register, Db14W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db15: Aliased<u32, Db15R::Register, Db15W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db16: Aliased<u32, Db16R::Register, Db16W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db17: Aliased<u32, Db17R::Register, Db17W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db18: Aliased<u32, Db18R::Register, Db18W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db19: Aliased<u32, Db19R::Register, Db19W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db20: Aliased<u32, Db20R::Register, Db20W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db21: Aliased<u32, Db21R::Register, Db21W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db22: Aliased<u32, Db22R::Register, Db22W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db23: Aliased<u32, Db23R::Register, Db23W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db24: Aliased<u32, Db24R::Register, Db24W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db25: Aliased<u32, Db25R::Register, Db25W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db26: Aliased<u32, Db26R::Register, Db26W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db27: Aliased<u32, Db27R::Register, Db27W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db28: Aliased<u32, Db28R::Register, Db28W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db29: Aliased<u32, Db29R::Register, Db29W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db30: Aliased<u32, Db30R::Register, Db30W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db31: Aliased<u32, Db31R::Register, Db31W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db32: Aliased<u32, Db32R::Register, Db32W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db33: Aliased<u32, Db33R::Register, Db33W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db34: Aliased<u32, Db34R::Register, Db34W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db35: Aliased<u32, Db35R::Register, Db35W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db36: Aliased<u32, Db36R::Register, Db36W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db37: Aliased<u32, Db37R::Register, Db37W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db38: Aliased<u32, Db38R::Register, Db38W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db39: Aliased<u32, Db39R::Register, Db39W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db40: Aliased<u32, Db40R::Register, Db40W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db41: Aliased<u32, Db41R::Register, Db41W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db42: Aliased<u32, Db42R::Register, Db42W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db43: Aliased<u32, Db43R::Register, Db43W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db44: Aliased<u32, Db44R::Register, Db44W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db45: Aliased<u32, Db45R::Register, Db45W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db46: Aliased<u32, Db46R::Register, Db46W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db47: Aliased<u32, Db47R::Register, Db47W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db48: Aliased<u32, Db48R::Register, Db48W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db49: Aliased<u32, Db49R::Register, Db49W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db50: Aliased<u32, Db50R::Register, Db50W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db51: Aliased<u32, Db51R::Register, Db51W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db52: Aliased<u32, Db52R::Register, Db52W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db53: Aliased<u32, Db53R::Register, Db53W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db54: Aliased<u32, Db54R::Register, Db54W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db55: Aliased<u32, Db55R::Register, Db55W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db56: Aliased<u32, Db56R::Register, Db56W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db57: Aliased<u32, Db57R::Register, Db57W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db58: Aliased<u32, Db58R::Register, Db58W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db59: Aliased<u32, Db59R::Register, Db59W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db60: Aliased<u32, Db60R::Register, Db60W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db61: Aliased<u32, Db61R::Register, Db61W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db62: Aliased<u32, Db62R::Register, Db62W::Register>,
    /// Doorbell Register Bit Field DefinitionsFor a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.Programming this field with random data will cause side effect i.e. Register Access will fail (Timeout) if the pipe clock is not running or reset is asserted. Bit Bash register testing is not recommended.
    pub db63: Aliased<u32, Db63R::Register, Db63W::Register>,
    _padding1504: [u8; 768],
    /// USBLEGSUP
    pub usblegsup: Aliased<u32, UsblegsupR::Register, UsblegsupW::Register>,
    /// USBLEGCTLSTS
    pub usblegctlsts: Aliased<u32, UsblegctlstsR::Register, UsblegctlstsW::Register>,
    _padding2280: [u8; 8],
    /// SUPTPRT2_DW0
    pub suptprt2_dw0: ReadOnly<u32, Suptprt2Dw0::Register>,
    /// Register SUPTPRT2_DW1
    pub suptprt2_dw1: ReadOnly<u32>,
    /// xHCI Supported Protocol Capability_ Data Word 2For a description of other register fields, see section 7.2 of the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub suptprt2_dw2: ReadOnly<u32, Suptprt2Dw2::Register>,
    /// Register SUPTPRT2_DW3
    pub suptprt2_dw3: ReadOnly<u32, Suptprt2Dw3::Register>,
    /// Register SUPTPRT3_DW0
    pub suptprt3_dw0: ReadOnly<u32, Suptprt3Dw0::Register>,
    /// Register SUPTPRT3_DW1
    pub suptprt3_dw1: ReadOnly<u32>,
    /// SUPTPRT3_DW2For a description of this standard USB register field, see the eXtensible Host Controller Interface for Universal Serial Bus (USB) Specification 3.0.
    pub suptprt3_dw2: ReadOnly<u32, Suptprt3Dw2::Register>,
    /// SUPTPRT3_DW3
    pub suptprt3_dw3: ReadOnly<u32, Suptprt3Dw3::Register>,
    /// DCID
    pub dcid: ReadOnly<u32, Dcid::Register>,
    /// Register DCDB
    pub dcdb: Aliased<u32, DcdbR::Register, DcdbW::Register>,
    /// DCERSTSZ
    pub dcerstsz: Aliased<u32, DcerstszR::Register, DcerstszW::Register>,
    _padding2332: [u8; 4],
    /// DCERSTBA_LO
    pub dcerstba_lo: Aliased<u32, DcerstbaLoR::Register, DcerstbaLoW::Register>,
    /// Register DCERSTBA_HI
    pub dcerstba_hi: ReadWrite<u32>,
    /// DCERDP_LO
    pub dcerdp_lo: Aliased<u32, DcerdpLoR::Register, DcerdpLoW::Register>,
    /// DCERDP_HI
    pub dcerdp_hi: ReadWrite<u32>,
    /// DCCTRL
    pub dcctrl: Aliased<u32, DcctrlR::Register, DcctrlW::Register>,
    /// DCST
    pub dcst: ReadOnly<u32, Dcst::Register>,
    /// Register DCPORTSC
    pub dcportsc: Aliased<u32, DcportscR::Register, DcportscW::Register>,
    _padding2364: [u8; 4],
    /// DCCP_LO
    pub dccp_lo: Aliased<u32, DccpLoR::Register, DccpLoW::Register>,
    /// Register DCCP_HI
    pub dccp_hi: ReadWrite<u32>,
    /// Register DCDDI1
    pub dcddi1: ReadWrite<u32, Dcddi1::Register>,
    /// Register DCDDI2
    pub dcddi2: ReadWrite<u32, Dcddi2::Register>,
    _padding2384: [u8; 47024],
    /// Global SoC Bus Configuration Register 0This register configures system bus DMA options for the master bus, which may be configured as AHB, AXI, or Native. Options include burst length and cache type (bufferable/posted, cacheable/snoop, and so on). The application can program this register upon power-on, or a change in mode of operation after the DMA engine is halted.xHCI Register Power-On Value:The standard xHCI driver does not access this register.
    pub gsbuscfg0: Aliased<u32, Gsbuscfg0R::Register, Gsbuscfg0W::Register>,
    /// Global SoC Bus Configuration Register 1xHCI Register Power-On Value:The standard xHCI driver does not access this register.
    pub gsbuscfg1: Aliased<u32, Gsbuscfg1R::Register, Gsbuscfg1W::Register>,
    /// Global Tx Threshold Control RegisterFor more information on- Using this register, refer to Architecture Details chapter.- Selecting values for the fields of this register.Note:- All the fields in GTXTHRCFG register are valid only in Host mode.- GTXTHRCFG register is not applicable for Debug Target.- GTXTHRCFG register is not applicable in USB 2.0-only mode.
    pub gtxthrcfg: Aliased<u32, GtxthrcfgR::Register, GtxthrcfgW::Register>,
    /// Global Rx Threshold Control RegisterIn a normal case, a Tx burst starts as soon as one packet is prefetched; an Rx burst starts as soon as 1-packet space is available. This works well as long as the system bus is faster than the USB 3.0 bus (a 1024-bytes packet takes ~2.2 microseconds on the USB bus in SS mode).If the system bus latency is larger than 2.2 microseconds to access a 1024-byte packet, then starting a burst on 1-packet condition leads to an early abort of the burst causing unnecessary performance reduction.To avoid underrun and overrun during the burst, in a high-latency bus system (like USB), threshold and burst size control is provided through GTXTHRCFG and GRXTHRCFG registers. Bit [29] of the GTXTHRCFG and GRXTHRCFG registers enables this feature.For more information on- Using this register, refer to Architecture Details chapter.- Selecting values for the fields of this register.Note:- GRXTHRCFG register is not applicable for Debug Target.- GRXTHRCFG register is not applicable in USB 2.0-only mode.
    pub grxthrcfg: Aliased<u32, GrxthrcfgR::Register, GrxthrcfgW::Register>,
    /// Global Core Control RegisterNote:When Hibernation is not enabled, you can write any value to GblHibernationEn. It always returns 0 when read.
    pub gctl: ReadWrite<u32, Gctl::Register>,
    /// Global Power Management Status RegisterThis debug register gives information on which event caused the hibernation exit. It provides internal status and state machine information, and is foruse only for debugging purposes.This register is not applicable in USB 2.0-only mode.
    pub gpmsts: Aliased<u32, GpmstsR::Register, GpmstsW::Register>,
    /// Global Status Register
    pub gsts: Aliased<u32, GstsR::Register, GstsW::Register>,
    /// Global User Control Register 1
    pub guctl1: Aliased<u32, Guctl1R::Register, Guctl1W::Register>,
    /// GlobalID RegisterThis is a read-only register that contains the release number of the core.
    pub gsnpsid: ReadOnly<u32>,
    /// Global General Purpose Input/Output RegisterThe application can use this register for general purpose input and output ports or for debugging.
    pub ggpio: Aliased<u32, GgpioR::Register, GgpioW::Register>,
    /// Global User ID Register This is a read/write register containing the User ID. This register can be used in the following ways:- To store the version or revision of your system;- To store hardware configurations that are outside the core;- As a scratch register.
    pub guid: ReadWrite<u32>,
    /// Global User Control Register:This register provides a few options for the software to control the core behavior in the Host mode. Most of the options are used to improve host inter-operability with different devices.
    pub guctl: Aliased<u32, GuctlR::Register, GuctlW::Register>,
    /// Gobal SoC Bus Error Address Register - LowThis is an alternate register for the GBUSERRADDR register.
    pub gbuserraddrlo: ReadOnly<u32>,
    /// Gobal SoC Bus Error Address Register - HighThis is an alternate register for the GBUSERRADDR register.
    pub gbuserraddrhi: ReadOnly<u32>,
    /// Global SS Port to Bus Instance Mapping Register - LowThis is an alternate register for the GPRTBIMAP register.Note: For reset values, refer to the corresponding values in the GPRTBIMAP register.
    pub gprtbimaplo: Aliased<u32, GprtbimaploR::Register, GprtbimaploW::Register>,
    /// Global SS Port to Bus Instance Mapping Register - HighThis is an alternate register for the GPRTBIMAP register.Note: For reset values, refer to the corresponding values in the GPRTBIMAP register.
    pub gprtbimaphi: ReadOnly<u32, Gprtbimaphi::Register>,
    /// Global Hardware Parameter Register 0
    pub ghwparams0: ReadOnly<u32, Ghwparams0::Register>,
    /// Global Hardware Parameter Register 1
    pub ghwparams1: ReadOnly<u32, Ghwparams1::Register>,
    /// Global Hardware Parameter Register 2
    pub ghwparams2: ReadOnly<u32>,
    /// Global Hardware Parameter Register 3
    pub ghwparams3: ReadOnly<u32, Ghwparams3::Register>,
    /// Global Hardware Parameter Register 4
    pub ghwparams4: ReadOnly<u32, Ghwparams4::Register>,
    /// Global Hardware Parameter Register 5
    pub ghwparams5: ReadOnly<u32, Ghwparams5::Register>,
    /// Global Hardware Parameter Register 6
    pub ghwparams6: ReadOnly<u32, Ghwparams6::Register>,
    /// Global Hardware Parameter Register 7
    pub ghwparams7: ReadOnly<u32, Ghwparams7::Register>,
    /// Global Debug Queue/FIFO Space Available RegisterBit Bash test should not be done on this debug register.
    pub gdbgfifospace: Aliased<u32, GdbgfifospaceR::Register, GdbgfifospaceW::Register>,
    /// Global Debug LTSSM RegisterIn multi-port host configuration, the port-number is defined by Port-Select[3:0] field in the GDBGFIFOSPACE register.Note:- GDBGLTSSM register is not applicable for USB 2.0-only mode.- Bit Bash test should not be done on this debug register.
    pub gdbgltssm: ReadOnly<u32, Gdbgltssm::Register>,
    /// Global Debug LNMCC RegisterBit Bash test should not be done on this debug register.
    pub gdbglnmcc: ReadOnly<u32, Gdbglnmcc::Register>,
    /// Global Debug BMU RegisterBit Bash test should not be done on this debug register.
    pub gdbgbmu: ReadOnly<u32, Gdbgbmu::Register>,
    /// Global Debug LSP MUX Register - HostThis register is for internal use only.If DWC_USB3_PRESERVE_LOGIC_ANALYZER_SELECT is enabled during core configuration, then the default values readout is X (Undefined).Bit Bash test should not be done on this debug register.
    pub gdbglspmux_hst: Aliased<u32, GdbglspmuxHstR::Register, GdbglspmuxHstW::Register>,
    /// Global Debug LSP RegisterThis register is for internal debug purposes only.This register is for internal use only.If DWC_USB3_PRESERVE_LOGIC_ANALYZER_SELECT is enabled during core configuration, then the default values readout is X (Undefined).Bit Bash test should not be done on this debug register.
    pub gdbglsp: ReadOnly<u32>,
    /// Global Debug Endpoint Information Register 0This register is for internal use only.If DWC_USB3_PRESERVE_LOGIC_ANALYZER_SELECT is enabled during core configuration, then the default values readout is X (Undefined).Bit Bash test should not be done on this debug register.
    pub gdbgepinfo0: ReadOnly<u32>,
    /// Global Debug Endpoint Information Register 1This register is for internal use only.If DWC_USB3_PRESERVE_LOGIC_ANALYZER_SELECT is enabled during core configuration, then the default values readout is X (Undefined).Bit Bash test should not be done on this debug register.
    pub gdbgepinfo1: ReadOnly<u32>,
    /// Global High-Speed Port to Bus Instance Mapping Register - LowThis is an alternate register for the GPRTBIMAP_HS register.Note: For reset values, refer to the corresponding values in the GPRTBIMAP_HS register.
    pub gprtbimap_hslo: Aliased<u32, GprtbimapHsloR::Register, GprtbimapHsloW::Register>,
    /// Global High-Speed Port to Bus Instance Mapping Register - HighThis is an alternate register for the GPRTBIMAP_HS register.Note: For reset values, refer to the corresponding values in the GPRTBIMAP register.
    pub gprtbimap_hshi: ReadOnly<u32, GprtbimapHshi::Register>,
    /// Global Full-Speed Port to Bus Instance Mapping Register - LowThis is an alternate register for the GPRTBIMAP_FS register.Note: For reset values, refer to the corresponding values in the GPRTBIMAP_FS register.
    pub gprtbimap_fslo: Aliased<u32, GprtbimapFsloR::Register, GprtbimapFsloW::Register>,
    /// Global Full-Speed Port to Bus Instance Mapping Register - HighThis is an alternate register for the GPRTBIMAP_FS register.Note: For reset values, refer to the corresponding values in the GPRTBIMAP_FS register.
    pub gprtbimap_fshi: ReadOnly<u32, GprtbimapFshi::Register>,
    _padding49552: [u8; 4],
    /// Future Register
    pub reserved_94: ReadOnly<u32>,
    /// Future Register
    pub reserved_98: ReadOnly<u32>,
    _padding49564: [u8; 100],
    /// Global USB2 PHY Configuration RegisterThe application must program this register before starting any transactions on either the SoC bus or the USB.In Device-only configurations, only one register is needed.In Host mode, per-port registers are implemented.
    pub gusb2phycfg: Aliased<u32, Gusb2phycfgR::Register, Gusb2phycfgW::Register>,
    _padding49668: [u8; 60],
    /// Reserved Register
    pub gusb2i2cctl: ReadOnly<u32>,
    _padding49732: [u8; 60],
    /// Global USB 2.0 UTMI PHY Vendor Control RegisterThe application used this register to access PHY registers. For an ULPI PHY, the core uses the ULPI interface for PHY register access.The application sets the Vendor Control register for PHY register access and times the PHY register access. The application polls the VStatus Done bit in this register for the completion of the PHY register access.In Device-only configurations, only one register is needed. In Host mode, per-port registers are implemented
    pub gusb2phyacc_ulpi: Aliased<u32, Gusb2phyaccUlpiR::Register, Gusb2phyaccUlpiW::Register>,
    _padding49796: [u8; 60],
    /// Global USB 3.0 PIPE Control RegisterThe application uses this register to configure the USB3 PHY and PIPE interface.Device-only configuration requires only one register. In Host mode, registers are implemented for each port.Note:- GUSB3PIPECTLn registers are not applicable for USB 2.0-only mode.
    pub gusb3pipectl: Aliased<u32, Gusb3pipectlR::Register, Gusb3pipectlW::Register>,
    _padding49860: [u8; 60],
    /// Global Transmit FIFO Size RegisterThis register specifies the RAM start address and depth (both in MDWIDTH-bit words) for each implemented TxFIFO. The number of TxFIFOs depends on the configuration parameters including the number of Device IN Endpoints, number of Host Bus Instances, and presence of Debug Capability.The register default values for each mode are assigned based on the maximum packet size, number of packets to be buffered, speed of host bus instance, bus latency, and mode of operation (host, device, or, DBC). Upon reset and mode transitions, hardware automatically programs these registers to the default values. Consequently, there is typically no need for the software to modify the pre-defined default values.For the debug capability mode, the currently mapped EP0 IN and EP1 IN TxFIFO numbers can be read from the GFIFOPRIDBC register.For OTG mode of operation, when the core is transitioning to host mode, program GTXFIFOSIZ register to the correct value only after OCTL.PeriMode is programmed to 1b0.
    pub gtxfifosiz0: ReadWrite<u32, Gtxfifosiz0::Register>,
    /// Register GTXFIFOSIZ 1
    pub gtxfifosiz1: ReadWrite<u32, Gtxfifosiz1::Register>,
    /// Transmit FIFOn RAM Start AddressThis field contains the memory start address for TxFIFOn in MDWIDTH-bit words.
    pub gtxfifosiz2: ReadWrite<u32, Gtxfifosiz2::Register>,
    /// Register GTXFIFOSIZ 3
    pub gtxfifosiz3: ReadWrite<u32, Gtxfifosiz3::Register>,
    /// Register GTXFIFOSIZ 4
    pub gtxfifosiz4: ReadWrite<u32, Gtxfifosiz4::Register>,
    /// Register GTXFIFOSIZ 5
    pub gtxfifosiz5: ReadWrite<u32, Gtxfifosiz5::Register>,
    _padding49944: [u8; 104],
    /// Global Receive FIFO Size RegisterThis register specifies the RAM start address and depth (both in MDWIDTH-bit words) for each implemented RxFIFO. The number of RxFIFOs depends on the configuration parameters including the number of Host Bus Instances and presence of Debug Capability; device mode requires only one RxFIFO.The register default values for each mode are assigned based on the maximum packet size, number of packets to be buffered, speed of the host bus instance, bus latency, and mode of operation (host, device, or DBC). Upon reset and mode transitions, hardware automatically programs these registers to the default values. Consequently, there is typically no need for the software to modify the pre-defined default values.For the debug capability mode, the currently mapped RxFIFO number can be read from the GFIFOPRIDBC register.
    pub grxfifosiz0: ReadWrite<u32, Grxfifosiz0::Register>,
    /// Register
    pub grxfifosiz1: ReadWrite<u32, Grxfifosiz1::Register>,
    /// Register
    pub grxfifosiz2: ReadWrite<u32, Grxfifosiz2::Register>,
    _padding50060: [u8; 116],
    /// Global Event Buffer Address (Low) RegisterThis is an alternate register for the GEVNTADRn register. Instance 0 of an array of 4.
    pub gevntadrlo_0: ReadWrite<u32>,
    /// Global Event Buffer Address (High) RegisterThis is an alternate register for the GEVNTADRn register. Instance 0 of an array of 4.
    pub gevntadrhi_0: ReadWrite<u32>,
    /// Global Event Buffer Size RegisterThis register holds the Event Buffer Size and the Event Interrupt Mask bit. During power-on initialization, software must initialize the size with the number of bytes allocated for the Event Buffer. The Event Interrupt Mask will mask the interrupt, but events are still queued. After configuration, software must preserve the Event Buffer Size value when changing the Event Interrupt Mask. Instance 0 of an array of 4.
    pub gevntsiz_0: Aliased<u32, Gevntsiz0R::Register, Gevntsiz0W::Register>,
    /// Global Event Buffer Count RegisterThis register holds the number of valid bytes in the Event Buffer. During initialization, software must initialize the count by writing 0 to the Event Count field. Each time the hardware writes a new event to the Event Buffer, it increments this count. Most events are four bytes, but some events may span over multiple four byte entries. Whenever the count is greater than zero, the hardware raises the corresponding interrupt line (depending on the EvntIntMask bit in the GEVNTSIZn register). On an interrupt, software processes one or more events out of the Event Buffer. Afterwards, software must write the Event Count field with the number of bytes it processed.Clock crossing delays may result in the interrupts continual assertion after software acknowledges the last event. Therefore, when the interrupt line is asserted, software must read the GEVNTCOUNT register and only process events if the GEVNTCOUNT is greater than 0. Instance 0 of an array of 4.
    pub gevntcount_0: Aliased<u32, Gevntcount0R::Register, Gevntcount0W::Register>,
    /// Global Event Buffer Address (Low) RegisterThis is an alternate register for the GEVNTADRn register. Instance 1 of an array of 4.
    pub gevntadrlo_1: ReadWrite<u32>,
    /// Global Event Buffer Address (High) RegisterThis is an alternate register for the GEVNTADRn register. Instance 1 of an array of 4.
    pub gevntadrhi_1: ReadWrite<u32>,
    /// Global Event Buffer Size RegisterThis register holds the Event Buffer Size and the Event Interrupt Mask bit. During power-on initialization, software must initialize the size with the number of bytes allocated for the Event Buffer. The Event Interrupt Mask will mask the interrupt, but events are still queued. After configuration, software must preserve the Event Buffer Size value when changing the Event Interrupt Mask. Instance 1 of an array of 4.
    pub gevntsiz_1: Aliased<u32, Gevntsiz1R::Register, Gevntsiz1W::Register>,
    /// Global Event Buffer Count RegisterThis register holds the number of valid bytes in the Event Buffer. During initialization, software must initialize the count by writing 0 to the Event Count field. Each time the hardware writes a new event to the Event Buffer, it increments this count. Most events are four bytes, but some events may span over multiple four byte entries. Whenever the count is greater than zero, the hardware raises the corresponding interrupt line (depending on the EvntIntMask bit in the GEVNTSIZn register). On an interrupt, software processes one or more events out of the Event Buffer. Afterwards, software must write the Event Count field with the number of bytes it processed.Clock crossing delays may result in the interrupts continual assertion after software acknowledges the last event. Therefore, when the interrupt line is asserted, software must read the GEVNTCOUNT register and only process events if the GEVNTCOUNT is greater than 0. Instance 1 of an array of 4.
    pub gevntcount_1: Aliased<u32, Gevntcount1R::Register, Gevntcount1W::Register>,
    /// Global Event Buffer Address (Low) RegisterThis is an alternate register for the GEVNTADRn register. Instance 2 of an array of 4.
    pub gevntadrlo_2: ReadWrite<u32>,
    /// Global Event Buffer Address (High) RegisterThis is an alternate register for the GEVNTADRn register. Instance 2 of an array of 4.
    pub gevntadrhi_2: ReadWrite<u32>,
    /// Global Event Buffer Size RegisterThis register holds the Event Buffer Size and the Event Interrupt Mask bit. During power-on initialization, software must initialize the size with the number of bytes allocated for the Event Buffer. The Event Interrupt Mask will mask the interrupt, but events are still queued. After configuration, software must preserve the Event Buffer Size value when changing the Event Interrupt Mask. Instance 2 of an array of 4.
    pub gevntsiz_2: Aliased<u32, Gevntsiz2R::Register, Gevntsiz2W::Register>,
    /// Global Event Buffer Count RegisterThis register holds the number of valid bytes in the Event Buffer. During initialization, software must initialize the count by writing 0 to the Event Count field. Each time the hardware writes a new event to the Event Buffer, it increments this count. Most events are four bytes, but some events may span over multiple four byte entries. Whenever the count is greater than zero, the hardware raises the corresponding interrupt line (depending on the EvntIntMask bit in the GEVNTSIZn register). On an interrupt, software processes one or more events out of the Event Buffer. Afterwards, software must write the Event Count field with the number of bytes it processed.Clock crossing delays may result in the interrupts continual assertion after software acknowledges the last event. Therefore, when the interrupt line is asserted, software must read the GEVNTCOUNT register and only process events if the GEVNTCOUNT is greater than 0. Instance 2 of an array of 4.
    pub gevntcount_2: Aliased<u32, Gevntcount2R::Register, Gevntcount2W::Register>,
    /// Global Event Buffer Address (Low) RegisterThis is an alternate register for the GEVNTADRn register. Instance 3 of an array of 4.
    pub gevntadrlo_3: ReadWrite<u32>,
    /// Global Event Buffer Address (High) RegisterThis is an alternate register for the GEVNTADRn register. Instance 3 of an array of 4.
    pub gevntadrhi_3: ReadWrite<u32>,
    /// Global Event Buffer Size RegisterThis register holds the Event Buffer Size and the Event Interrupt Mask bit. During power-on initialization, software must initialize the size with the number of bytes allocated for the Event Buffer. The Event Interrupt Mask will mask the interrupt, but events are still queued. After configuration, software must preserve the Event Buffer Size value when changing the Event Interrupt Mask. Instance 3 of an array of 4.
    pub gevntsiz_3: Aliased<u32, Gevntsiz3R::Register, Gevntsiz3W::Register>,
    /// Global Event Buffer Count RegisterThis register holds the number of valid bytes in the Event Buffer. During initialization, software must initialize the count by writing 0 to the Event Count field. Each time the hardware writes a new event to the Event Buffer, it increments this count. Most events are four bytes, but some events may span over multiple four byte entries. Whenever the count is greater than zero, the hardware raises the corresponding interrupt line (depending on the EvntIntMask bit in the GEVNTSIZn register). On an interrupt, software processes one or more events out of the Event Buffer. Afterwards, software must write the Event Count field with the number of bytes it processed.Clock crossing delays may result in the interrupts continual assertion after software acknowledges the last event. Therefore, when the interrupt line is asserted, software must read the GEVNTCOUNT register and only process events if the GEVNTCOUNT is greater than 0. Instance 3 of an array of 4.
    pub gevntcount_3: Aliased<u32, Gevntcount3R::Register, Gevntcount3W::Register>,
    _padding50240: [u8; 448],
    /// Global Hardware ParametersThis register contains the hardware configuration options selected during implementation.
    pub ghwparams8: ReadOnly<u32>,
    _padding50692: [u8; 12],
    /// Global Device TX FIFO DMA Priority RegisterThis register specifies the relative DMA priority level among the Device TXFIFOs (one per IN endpoint). Each register bit[n] controls the priority (1: high, 0: low) of each TXFIFO[n]. When multiple TXFIFOs compete for DMA service at a given time (that is, multiple TXQs contain TX DMA requests and their corresponding TXFIFOs have space available), the TX DMA arbiter grants access on a packet-basis in the following manner:- 1. High-priority TXFIFOs are granted access using round-robin arbitration- 2. Low-priority TXFIFOs are granted access using round-robin arbitration only after the high-priority TXFIFOs have no further processing to do (that is, either the TXQs are empty or the corresponding TXFIFOs are full).For scatter-gather packets, the arbiter grants successive DMA requests to the same FIFO until the entire packet is completed.When configuring periodic IN endpoints, software must set register bit[n]=1, where n is the TXFIFO assignment. This ensures that the DMA for isochronous or interrupt IN endpoints are prioritized over bulk or control IN endpoints.This register is present only when the core is configured to operate in the device mode (includes DRD and OTG modes). The register size corresponds to the number of Device IN endpoints.Note- Since the device mode uses only one RXFIFO, there is no Device RXFIFO DMA Priority Register.
    pub gtxfifopridev: Aliased<u32, GtxfifopridevR::Register, GtxfifopridevW::Register>,
    _padding50708: [u8; 4],
    /// Global Host TX FIFO DMA Priority RegisterThis register specifies the relative DMA priority level among the Host TXFIFOs (one per USB bus instance) within the associated speed group (SS or HS/FSLS). Each register bit[n] controls the priority (1: high, 0: low) of TXFIFO[n] within a speed group. When multiple TXFIFOs compete for DMA service at a given time (i.e., multiple TXQs contain TX DMA requests and their corresponding TXFIFOs have space available), the TX DMA arbiter grants access on a packet-basis in the following manner:- 1. Among the FIFOs in the same speed group (SS or HS/FSLS):a. High-priority TXFIFOs are granted access using round-robin arbitrationb. Low-priority TXFIFOs are granted access using round-robin arbitration only after the high-priority TXFIFOs have no further processing to do (that is, either the TXQs are empty or the corresponding TXFIFOs are full).- 2. The TX DMA arbiter prioritizes the SS speed group or HS/FSLS speed group according to the ratio programmed in the GDMAHLRATIO register.For scatter-gather packets, the arbiter grants successive DMA requests to the same FIFO until the entire packet is completed.This register is present only when the core is configured to operate in the host mode (includes DRD and OTG modes). The register size corresponds to the number of configured USB bus instances; for example, in the default configuration, there are 3 USB bus instances (1 SS, 1 HS, and 1 FSLS).
    pub gtxfifoprihst: Aliased<u32, GtxfifoprihstR::Register, GtxfifoprihstW::Register>,
    /// Global Host RX FIFO DMA Priority RegisterThis register specifies the relative DMA priority level among the Host RXFIFOs (one per USB bus instance) within the associated speed group (SS or HS/FSLS). Each register bit[n] controls the priority (1: high, 0: low) of RXFIFO[n] within a speed group. When multiple RXFIFOs compete for DMA service at a given time (i.e., multiple RXQs contain RX DMA requests and their corresponding RXFIFOs have data available), the RX DMA arbiter grants access on a packet-basis in the following manner:- 1. Among the FIFOs in the same speed group (SS or HS/FSLS):a. High-priority RXFIFOs are granted access using round-robin arbitrationb. Low-priority RXFIFOs are granted access using round-robin arbitration only after high-priority RXFIFOs have no further processing to do (that is, either the RXQs are empty or the corresponding RXFIFOs do not have the required data).- 2. The RX DMA arbiter prioritizes the SS speed group or HS/FSLS speed group according to the ratio programmed in the GDMAHLRATIO register.For scatter-gather packets, the arbiter grants successive DMA requests to the same FIFO until the entire packet is completed.This register is present only when the core is configured to operate in the host mode (includes DRD and OTG modes). The register size corresponds to the number of configured USB bus instances; for example, in the default configuration, there are 3 USB bus instances (1 SS, 1 HS, and 1 FSLS).
    pub grxfifoprihst: Aliased<u32, GrxfifoprihstR::Register, GrxfifoprihstW::Register>,
    /// Global Host Debug Capability DMA Priority RegisterThis register specifies the relative priority of the RXFIFOs and TXFIFOs associated with the DbC mode. It overrides the priority assigned in the corresponding indexes of the Host RXFIFO and TXFIFO DMA priority registers, when the DbC mode is enabled.Priority settings are specified in relation to the low-priority SS speed group:- 1. Normal priority indicates that the DbC FIFOs are considered identical to the Host SS low-priority FIFOs.- 2. Low priority indicates that the DbC FIFOs are considered to have lower priority than all Host SS FIFOs.- 3. High priority indicates that the DbC FIFOs are considered higher priority than the Host SS low-priority FIFOs but lower priority than the Host SS high-priority FIFOs.This register is present only when the core is configured to operate in Host Debug Capability (DbC) mode.
    pub gfifopridbc: Aliased<u32, GfifopridbcR::Register, GfifopridbcW::Register>,
    /// Global Host FIFO DMA High-Low Priority Ratio RegisterThis register specifies the relative priority of the SS FIFOs with respect to the HS/FSLS FIFOs. The DMA arbiter prioritizes the HS/FSLS round-robin arbiter group every DMA High-Low Priority Ratio grants as indicated in the register separately for TX and RX.To illustrate, consider that all FIFOs are requesting access simultaneously, and the ratio is 4. SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, SS gets priority for 4 packets, HS/FSLS gets priority for 1 packet, and so on.If FIFOs from both speed groups are not requesting access simultaneously then,- if SS got grants 4 out of the last 4 times, then HS/FSLS get the priority on any future request.- if HS/FSLS got the grant last time, SS gets the priority on the next request.- if there is a valid request on either SS or HS/FSLS, a grant is always awarded; there is no idle.This register is present if the core is configured to operate in host mode (includes DRD and OTG).
    pub gdmahlratio: Aliased<u32, GdmahlratioR::Register, GdmahlratioW::Register>,
    _padding50728: [u8; 8],
    /// Global Frame Length Adjustment RegisterThis register provides options for the software to control the core behavior with respect to SOF (Start of Frame) and ITP (Isochronous Timestamp Packet) timers and frame timer functionality. It provides an option to override the fladj_30mhz_reg sideband signal. In addition, it enables running SOF or ITP frame timer counters completely from the ref_clk. This facilitates hardware LPM in host mode with the SOF or ITP counters being run from the ref_clk signal.
    pub gfladj: Aliased<u32, GfladjR::Register, GfladjW::Register>,
    _padding50740: [u8; 204],
    /// Device Configuration Register.This register configures the core in Device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.
    pub dcfg: Aliased<u32, DcfgR::Register, DcfgW::Register>,
    /// Device Control RegisterNote:When Hibernation is not enabled using GCTL.GblHibernationEn field,- you can write any value to CSS, CRS, L1HibernationEn, and KeepConnect fields- L1HibernationEn, and KeepConnect fields always return 0 when read in this hibernation-disabled state
    pub dctl: Aliased<u32, DctlR::Register, DctlW::Register>,
    /// Device Event Enable RegisterThis register controls the generation of device-specific events. If an enable bit is set to 0, the event will not be generated.
    pub devten: Aliased<u32, DevtenR::Register, DevtenW::Register>,
    /// Device Status RegisterThis register indicates the status of the device controller with respect to USB-related events.Note:When Hibernation is not enabled, RSS and SSS fields always return 0 when read.
    pub dsts: Aliased<u32, DstsR::Register, DstsW::Register>,
    /// Device Generic Command Parameter RegisterThis register indicates the device command parameter. This must be programmed before or along with the device command. The available device commands are listed in DGCMD register.
    pub dgcmdpar: ReadWrite<u32>,
    /// Device Generic Command RegisterThis register enables software to program the core using a single generic command interface to send link management packets and notifications. This register contains command, control, and status fields relevant to the current generic command, while the DGCMDPAR register provides the command parameter.
    pub dgcmd: Aliased<u32, DgcmdR::Register, DgcmdW::Register>,
    _padding50968: [u8; 8],
    /// Device Active USB Endpoint Enable Register.This register indicates whether a USB endpoint is active in a given configuration or interface.
    pub dalepena: ReadWrite<u32>,
    /// Reserved
    pub rsvd0: ReadOnly<u32>,
    /// Reserved
    pub rsvd1: ReadOnly<u32>,
    /// Reserved
    pub rsvd2: ReadOnly<u32>,
    /// Reserved
    pub rsvd3: ReadOnly<u32>,
    /// Reserved
    pub rsvd4: ReadOnly<u32>,
    /// Reserved
    pub rsvd5: ReadOnly<u32>,
    /// Reserved
    pub rsvd6: ReadOnly<u32>,
    /// Reserved
    pub rsvd7: ReadOnly<u32>,
    /// Reserved
    pub rsvd8: ReadOnly<u32>,
    /// Reserved
    pub rsvd9: ReadOnly<u32>,
    /// Reserved
    pub rsvd10: ReadOnly<u32>,
    /// Reserved
    pub rsvd11: ReadOnly<u32>,
    /// Reserved
    pub rsvd12: ReadOnly<u32>,
    /// Reserved
    pub rsvd13: ReadOnly<u32>,
    /// Reserved
    pub rsvd14: ReadOnly<u32>,
    /// Reserved
    pub rsvd15: ReadOnly<u32>,
    /// Reserved
    pub rsvd16: ReadOnly<u32>,
    /// Reserved
    pub rsvd17: ReadOnly<u32>,
    /// Reserved
    pub rsvd18: ReadOnly<u32>,
    /// Reserved
    pub rsvd19: ReadOnly<u32>,
    /// Reserved
    pub rsvd20: ReadOnly<u32>,
    /// Reserved
    pub rsvd21: ReadOnly<u32>,
    /// Reserved
    pub rsvd22: ReadOnly<u32>,
    /// Reserved
    pub rsvd23: ReadOnly<u32>,
    /// Reserved
    pub rsvd24: ReadOnly<u32>,
    /// Reserved
    pub rsvd25: ReadOnly<u32>,
    /// Reserved
    pub rsvd26: ReadOnly<u32>,
    /// Reserved
    pub rsvd27: ReadOnly<u32>,
    /// Reserved
    pub rsvd28: ReadOnly<u32>,
    /// Reserved
    pub rsvd29: ReadOnly<u32>,
    /// Reserved
    pub rsvd30: ReadOnly<u32>,
    /// Reserved
    pub rsvd31: ReadOnly<u32>,
    _padding51108: [u8; 92],
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 0 of an array of 12.
    pub depcmdpar2_0: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 0 of an array of 12.
    pub depcmdpar1_0: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 0 of an array of 12.
    pub depcmdpar0_0: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 0 of an array of 12.
    pub depcmd_0: Aliased<u32, Depcmd0R::Register, Depcmd0W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 1 of an array of 12.
    pub depcmdpar2_1: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 1 of an array of 12.
    pub depcmdpar1_1: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 1 of an array of 12.
    pub depcmdpar0_1: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 1 of an array of 12.
    pub depcmd_1: Aliased<u32, Depcmd1R::Register, Depcmd1W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 2 of an array of 12.
    pub depcmdpar2_2: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 2 of an array of 12.
    pub depcmdpar1_2: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 2 of an array of 12.
    pub depcmdpar0_2: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 2 of an array of 12.
    pub depcmd_2: Aliased<u32, Depcmd2R::Register, Depcmd2W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 3 of an array of 12.
    pub depcmdpar2_3: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 3 of an array of 12.
    pub depcmdpar1_3: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 3 of an array of 12.
    pub depcmdpar0_3: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 3 of an array of 12.
    pub depcmd_3: Aliased<u32, Depcmd3R::Register, Depcmd3W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 4 of an array of 12.
    pub depcmdpar2_4: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 4 of an array of 12.
    pub depcmdpar1_4: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 4 of an array of 12.
    pub depcmdpar0_4: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 4 of an array of 12.
    pub depcmd_4: Aliased<u32, Depcmd4R::Register, Depcmd4W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 5 of an array of 12.
    pub depcmdpar2_5: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 5 of an array of 12.
    pub depcmdpar1_5: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 5 of an array of 12.
    pub depcmdpar0_5: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 5 of an array of 12.
    pub depcmd_5: Aliased<u32, Depcmd5R::Register, Depcmd5W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 6 of an array of 12.
    pub depcmdpar2_6: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 6 of an array of 12.
    pub depcmdpar1_6: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 6 of an array of 12.
    pub depcmdpar0_6: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 6 of an array of 12.
    pub depcmd_6: Aliased<u32, Depcmd6R::Register, Depcmd6W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 7 of an array of 12.
    pub depcmdpar2_7: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 7 of an array of 12.
    pub depcmdpar1_7: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 7 of an array of 12.
    pub depcmdpar0_7: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 7 of an array of 12.
    pub depcmd_7: Aliased<u32, Depcmd7R::Register, Depcmd7W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 8 of an array of 12.
    pub depcmdpar2_8: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 8 of an array of 12.
    pub depcmdpar1_8: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 8 of an array of 12.
    pub depcmdpar0_8: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 8 of an array of 12.
    pub depcmd_8: Aliased<u32, Depcmd8R::Register, Depcmd8W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 9 of an array of 12.
    pub depcmdpar2_9: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 9 of an array of 12.
    pub depcmdpar1_9: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 9 of an array of 12.
    pub depcmdpar0_9: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 9 of an array of 12.
    pub depcmd_9: Aliased<u32, Depcmd9R::Register, Depcmd9W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 10 of an array of 12.
    pub depcmdpar2_10: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 10 of an array of 12.
    pub depcmdpar1_10: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 10 of an array of 12.
    pub depcmdpar0_10: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 10 of an array of 12.
    pub depcmd_10: Aliased<u32, Depcmd10R::Register, Depcmd10W::Register>,
    /// Device Physical Endpoint-n Command Parameter 2 Register (DEPCMDPAR2n)This register indicates the physical endpoint command Parameter 2. It must be programmed before issuing the command.Instance 11 of an array of 12.
    pub depcmdpar2_11: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 1 Register (DEPCMDPAR1n) Instance 11 of an array of 12.
    pub depcmdpar1_11: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command Parameter 0 Register (DEPCMDPAR0n) Instance 11 of an array of 12.
    pub depcmdpar0_11: ReadWrite<u32>,
    /// Device Physical Endpoint-n Command RegisterThis register enables software to issue physical endpoint-specific commands. This register contains command, control, and status fields relevant to the current generic command, while the DEPCMDPAR[2:0]n registers provide command parameters and return status information.Several fields (including Command Type) are write-only, so their read values are undefined. After power-on, prior to issuing the first endpoint command, the read value of this register is undefined. In particular, the CmdAct bit may be set after power-on. In this case, it is safe to issue an endpoint command. Instance 11 of an array of 12.
    pub depcmd_11: Aliased<u32, Depcmd11R::Register, Depcmd11W::Register>,
    _padding51392: [u8; 832],
    /// OTG Configuration RegisterThis register specifies the HNP and SRP capability of the controller
    pub ocfg: Aliased<u32, OcfgR::Register, OcfgW::Register>,
    /// OTG Control RegisterThe OTG Control register controls the behavior of the OTG function of the core.
    pub octl: Aliased<u32, OctlR::Register, OctlW::Register>,
    /// OTG Events RegisterAny event set in this register will cause otg_interrupt signal to go high. Writing 1b1 to the event information bit in this register clears the register bit and the associated interrupt. The otg_interrupt signal goes low when there are no more pending OTG events.
    pub oevt: Aliased<u32, OevtR::Register, OevtW::Register>,
    /// OTG Events Enable RegisterSetting a bit in this register enables the generation of corresponding events in OEVT and assertion of otg_interrupt due to this event. When the enable bit is 1b0, the event isnot be set in OEVT and otg_interrupt is not asserted due to this event.
    pub oevten: Aliased<u32, OevtenR::Register, OevtenW::Register>,
    /// OTG Status RegisterThe OTG Status Register reflects the status of the OTG function of the core.
    pub osts: ReadOnly<u32, Osts::Register>,
    _padding52244: [u8; 20],
    /// ADP Event Register
    pub adpevt: Aliased<u32, AdpevtR::Register, AdpevtW::Register>,
}
tock_registers::register_bitfields! [
    u32,
    pub Caplength [
        HCIVERSION OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        CAPLENGTH OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Hcsparams1 [
        MAXPORTS OFFSET(24) NUMBITS(8) [],
        RESERVED0 OFFSET(19) NUMBITS(5) [],
        MAXINTRS OFFSET(8) NUMBITS(11) [],
        MAXSLOTS OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Hcsparams2 [
        MAXSCRATCHPADBUFS OFFSET(27) NUMBITS(5) [],
        SPR OFFSET(26) NUMBITS(1) [],
        MAXSCRATCHPADBUFS_HI OFFSET(21) NUMBITS(5) [],
        RESERVED0 OFFSET(8) NUMBITS(13) [],
        ERSTMAX OFFSET(4) NUMBITS(4) [],
        IST OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Hcsparams3 [
        U2_DEVICE_EXIT_LAT OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        U1_DEVICE_EXIT_LAT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Hccparams1 [
        XECP OFFSET(16) NUMBITS(16) [],
        MAXPSASIZE OFFSET(12) NUMBITS(4) [],
        CFC OFFSET(11) NUMBITS(1) [],
        SEC OFFSET(10) NUMBITS(1) [],
        SPC OFFSET(9) NUMBITS(1) [],
        PAE OFFSET(8) NUMBITS(1) [],
        NSS OFFSET(7) NUMBITS(1) [],
        LTC OFFSET(6) NUMBITS(1) [],
        LHRC OFFSET(5) NUMBITS(1) [],
        PIND OFFSET(4) NUMBITS(1) [],
        PPC OFFSET(3) NUMBITS(1) [],
        CSZ OFFSET(2) NUMBITS(1) [],
        BNC OFFSET(1) NUMBITS(1) [],
        AC64 OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dboff [
        DOORBELL_ARRAY_OFFSET OFFSET(2) NUMBITS(30) [],
        RESERVED0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Rtsoff [
        RUNTIME_REG_SPACE_OFFSET OFFSET(5) NUMBITS(27) [],
        RESERVED0 OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Hccparams2 [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        CIC OFFSET(5) NUMBITS(1) [],
        LEC OFFSET(4) NUMBITS(1) [],
        CTC OFFSET(3) NUMBITS(1) [],
        FSC OFFSET(2) NUMBITS(1) [],
        CMC OFFSET(1) NUMBITS(1) [],
        U3C OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub UsbcmdR [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        CME OFFSET(13) NUMBITS(1) [],
        RESERVED1 OFFSET(12) NUMBITS(1) [],
        EU3S OFFSET(11) NUMBITS(1) [],
        EWE OFFSET(10) NUMBITS(1) [],
        CRS OFFSET(9) NUMBITS(1) [],
        CSS OFFSET(8) NUMBITS(1) [],
        LHCRST OFFSET(7) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(3) [],
        HSEE OFFSET(3) NUMBITS(1) [],
        INTE OFFSET(2) NUMBITS(1) [],
        HCRST OFFSET(1) NUMBITS(1) [],
        R_S OFFSET(0) NUMBITS(1) [],
    ],
    pub UsbcmdW [
        CME OFFSET(13) NUMBITS(1) [],
        EU3S OFFSET(11) NUMBITS(1) [],
        EWE OFFSET(10) NUMBITS(1) [],
        CRS OFFSET(9) NUMBITS(1) [],
        CSS OFFSET(8) NUMBITS(1) [],
        LHCRST OFFSET(7) NUMBITS(1) [],
        HSEE OFFSET(3) NUMBITS(1) [],
        INTE OFFSET(2) NUMBITS(1) [],
        HCRST OFFSET(1) NUMBITS(1) [],
        R_S OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub UsbstsR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        HCE OFFSET(12) NUMBITS(1) [],
        CNR OFFSET(11) NUMBITS(1) [],
        SRE OFFSET(10) NUMBITS(1) [],
        RSS OFFSET(9) NUMBITS(1) [],
        SSS OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(5) NUMBITS(3) [],
        PCD OFFSET(4) NUMBITS(1) [],
        EINT OFFSET(3) NUMBITS(1) [],
        HSE OFFSET(2) NUMBITS(1) [],
        RESERVED2 OFFSET(1) NUMBITS(1) [],
        HCH OFFSET(0) NUMBITS(1) [],
    ],
    pub UsbstsW [
        SRE OFFSET(10) NUMBITS(1) [],
        PCD OFFSET(4) NUMBITS(1) [],
        EINT OFFSET(3) NUMBITS(1) [],
        HSE OFFSET(2) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Pagesize [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        PAGE_SIZE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DnctrlR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        N0_N15 OFFSET(0) NUMBITS(16) [],
    ],
    pub DnctrlW [
        N0_N15 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub CrcrLoR [
        CMD_RING_PNTR OFFSET(6) NUMBITS(26) [],
        RESERVED0 OFFSET(4) NUMBITS(2) [],
        CRR OFFSET(3) NUMBITS(1) [],
        CA OFFSET(2) NUMBITS(1) [],
        CS OFFSET(1) NUMBITS(1) [],
        RCS OFFSET(0) NUMBITS(1) [],
    ],
    pub CrcrLoW [
        CMD_RING_PNTR OFFSET(6) NUMBITS(26) [],
        CA OFFSET(2) NUMBITS(1) [],
        CS OFFSET(1) NUMBITS(1) [],
        RCS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcbaapLoR [
        DEVICE_CONTEXT_BAAP OFFSET(6) NUMBITS(26) [],
        RESERVED0 OFFSET(0) NUMBITS(6) [],
    ],
    pub DcbaapLoW [
        DEVICE_CONTEXT_BAAP OFFSET(6) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ConfigR [
        RESERVED0 OFFSET(10) NUMBITS(22) [],
        CIE OFFSET(9) NUMBITS(1) [],
        U3E OFFSET(8) NUMBITS(1) [],
        MAXSLOTSEN OFFSET(0) NUMBITS(8) [],
    ],
    pub ConfigW [
        CIE OFFSET(9) NUMBITS(1) [],
        U3E OFFSET(8) NUMBITS(1) [],
        MAXSLOTSEN OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Portsc20R [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        DR OFFSET(30) NUMBITS(1) [],
        RESERVED1 OFFSET(28) NUMBITS(2) [],
        WOE OFFSET(27) NUMBITS(1) [],
        WDE OFFSET(26) NUMBITS(1) [],
        WCE OFFSET(25) NUMBITS(1) [],
        CAS OFFSET(24) NUMBITS(1) [],
        RESERVED2 OFFSET(23) NUMBITS(1) [],
        PLC OFFSET(22) NUMBITS(1) [],
        PRC OFFSET(21) NUMBITS(1) [],
        OCC OFFSET(20) NUMBITS(1) [],
        RESERVED3 OFFSET(19) NUMBITS(1) [],
        PEC OFFSET(18) NUMBITS(1) [],
        CSC OFFSET(17) NUMBITS(1) [],
        LWS OFFSET(16) NUMBITS(1) [],
        PIC OFFSET(14) NUMBITS(2) [],
        PORTSPEED OFFSET(10) NUMBITS(4) [],
        PP OFFSET(9) NUMBITS(1) [],
        PLS OFFSET(5) NUMBITS(4) [],
        PR OFFSET(4) NUMBITS(1) [],
        OCA OFFSET(3) NUMBITS(1) [],
        RESERVED4 OFFSET(2) NUMBITS(1) [],
        PED OFFSET(1) NUMBITS(1) [],
        CCS OFFSET(0) NUMBITS(1) [],
    ],
    pub Portsc20W [
        WOE OFFSET(27) NUMBITS(1) [],
        WDE OFFSET(26) NUMBITS(1) [],
        WCE OFFSET(25) NUMBITS(1) [],
        PLC OFFSET(22) NUMBITS(1) [],
        PRC OFFSET(21) NUMBITS(1) [],
        OCC OFFSET(20) NUMBITS(1) [],
        RESERVED0 OFFSET(19) NUMBITS(1) [],
        PEC OFFSET(18) NUMBITS(1) [],
        CSC OFFSET(17) NUMBITS(1) [],
        LWS OFFSET(16) NUMBITS(1) [],
        PIC OFFSET(14) NUMBITS(2) [],
        PP OFFSET(9) NUMBITS(1) [],
        PLS OFFSET(5) NUMBITS(4) [],
        PR OFFSET(4) NUMBITS(1) [],
        PED OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Portpmsc20R [
        PRTTSTCTRL OFFSET(28) NUMBITS(4) [],
        RESERVED0 OFFSET(17) NUMBITS(11) [],
        HLE OFFSET(16) NUMBITS(1) [],
        L1DSLOT OFFSET(8) NUMBITS(8) [],
        HIRD OFFSET(4) NUMBITS(4) [],
        RWE OFFSET(3) NUMBITS(1) [],
        L1S OFFSET(0) NUMBITS(3) [],
    ],
    pub Portpmsc20W [
        PRTTSTCTRL OFFSET(28) NUMBITS(4) [],
        HLE OFFSET(16) NUMBITS(1) [],
        L1DSLOT OFFSET(8) NUMBITS(8) [],
        HIRD OFFSET(4) NUMBITS(4) [],
        RWE OFFSET(3) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Portli20 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        RESERVED1 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Porthlpmc20R [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        HIRDD OFFSET(10) NUMBITS(4) [],
        L1_TIMEOUT OFFSET(2) NUMBITS(8) [],
        HIRDM OFFSET(0) NUMBITS(2) [],
    ],
    pub Porthlpmc20W [
        HIRDD OFFSET(10) NUMBITS(4) [],
        L1_TIMEOUT OFFSET(2) NUMBITS(8) [],
        HIRDM OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Portsc30R [
        WPR OFFSET(31) NUMBITS(1) [],
        DR OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(2) [],
        WOE OFFSET(27) NUMBITS(1) [],
        WDE OFFSET(26) NUMBITS(1) [],
        WCE OFFSET(25) NUMBITS(1) [],
        CAS OFFSET(24) NUMBITS(1) [],
        CEC OFFSET(23) NUMBITS(1) [],
        PLC OFFSET(22) NUMBITS(1) [],
        PRC OFFSET(21) NUMBITS(1) [],
        OCC OFFSET(20) NUMBITS(1) [],
        WRC OFFSET(19) NUMBITS(1) [],
        PEC OFFSET(18) NUMBITS(1) [],
        CSC OFFSET(17) NUMBITS(1) [],
        LWS OFFSET(16) NUMBITS(1) [],
        PIC OFFSET(14) NUMBITS(2) [],
        PORTSPEED OFFSET(10) NUMBITS(4) [],
        PP OFFSET(9) NUMBITS(1) [],
        PLS OFFSET(5) NUMBITS(4) [],
        PR OFFSET(4) NUMBITS(1) [],
        OCA OFFSET(3) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(1) [],
        PED OFFSET(1) NUMBITS(1) [],
        CCS OFFSET(0) NUMBITS(1) [],
    ],
    pub Portsc30W [
        WPR OFFSET(31) NUMBITS(1) [],
        WOE OFFSET(27) NUMBITS(1) [],
        WDE OFFSET(26) NUMBITS(1) [],
        WCE OFFSET(25) NUMBITS(1) [],
        CEC OFFSET(23) NUMBITS(1) [],
        PLC OFFSET(22) NUMBITS(1) [],
        PRC OFFSET(21) NUMBITS(1) [],
        OCC OFFSET(20) NUMBITS(1) [],
        WRC OFFSET(19) NUMBITS(1) [],
        PEC OFFSET(18) NUMBITS(1) [],
        CSC OFFSET(17) NUMBITS(1) [],
        LWS OFFSET(16) NUMBITS(1) [],
        PIC OFFSET(14) NUMBITS(2) [],
        PP OFFSET(9) NUMBITS(1) [],
        PLS OFFSET(5) NUMBITS(4) [],
        PR OFFSET(4) NUMBITS(1) [],
        PED OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Portpmsc30R [
        RESERVED0 OFFSET(17) NUMBITS(15) [],
        FLA OFFSET(16) NUMBITS(1) [],
        U2_TIMEOUT OFFSET(8) NUMBITS(8) [],
        U1_TIMEOUT OFFSET(0) NUMBITS(8) [],
    ],
    pub Portpmsc30W [
        FLA OFFSET(16) NUMBITS(1) [],
        U2_TIMEOUT OFFSET(8) NUMBITS(8) [],
        U1_TIMEOUT OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Portli30 [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        LINK_ERROR_COUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Mfindex [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        MICROFRAME_INDEX OFFSET(0) NUMBITS(14) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Iman0R [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        IE OFFSET(1) NUMBITS(1) [],
        IP OFFSET(0) NUMBITS(1) [],
    ],
    pub Iman0W [
        IE OFFSET(1) NUMBITS(1) [],
        IP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imod0 [
        IMODC OFFSET(16) NUMBITS(16) [],
        IMODI OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Erstsz0R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ],
    pub Erstsz0W [
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErstbaLo0R [
        ERS_TABLE_BAR OFFSET(6) NUMBITS(26) [],
        RESERVED0 OFFSET(0) NUMBITS(6) [],
    ],
    pub ErstbaLo0W [
        ERS_TABLE_BAR OFFSET(6) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErdpLo0 [
        ERD_PNTR OFFSET(4) NUMBITS(28) [],
        EHB OFFSET(3) NUMBITS(1) [],
        DESI OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Iman1R [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        IE OFFSET(1) NUMBITS(1) [],
        IP OFFSET(0) NUMBITS(1) [],
    ],
    pub Iman1W [
        IE OFFSET(1) NUMBITS(1) [],
        IP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imod1 [
        IMODC OFFSET(16) NUMBITS(16) [],
        IMODI OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Erstsz1R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ],
    pub Erstsz1W [
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErstbaLo1R [
        ERS_TABLE_BAR OFFSET(6) NUMBITS(26) [],
        RESERVED0 OFFSET(0) NUMBITS(6) [],
    ],
    pub ErstbaLo1W [
        ERS_TABLE_BAR OFFSET(6) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErdpLo1 [
        ERD_PNTR OFFSET(4) NUMBITS(28) [],
        EHB OFFSET(3) NUMBITS(1) [],
        DESI OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Iman2R [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        IE OFFSET(1) NUMBITS(1) [],
        IP OFFSET(0) NUMBITS(1) [],
    ],
    pub Iman2W [
        IE OFFSET(1) NUMBITS(1) [],
        IP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imod2 [
        IMODC OFFSET(16) NUMBITS(16) [],
        IMODI OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Erstsz2R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ],
    pub Erstsz2W [
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErstbaLo2R [
        ERS_TABLE_BAR OFFSET(6) NUMBITS(26) [],
        RESERVED0 OFFSET(0) NUMBITS(6) [],
    ],
    pub ErstbaLo2W [
        ERS_TABLE_BAR OFFSET(6) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErdpLo2 [
        ERD_PNTR OFFSET(4) NUMBITS(28) [],
        EHB OFFSET(3) NUMBITS(1) [],
        DESI OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Iman3R [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        IE OFFSET(1) NUMBITS(1) [],
        IP OFFSET(0) NUMBITS(1) [],
    ],
    pub Iman3W [
        IE OFFSET(1) NUMBITS(1) [],
        IP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Imod3 [
        IMODC OFFSET(16) NUMBITS(16) [],
        IMODI OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Erstsz3R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ],
    pub Erstsz3W [
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErstbaLo3R [
        ERS_TABLE_BAR OFFSET(6) NUMBITS(26) [],
        RESERVED0 OFFSET(0) NUMBITS(6) [],
    ],
    pub ErstbaLo3W [
        ERS_TABLE_BAR OFFSET(6) NUMBITS(26) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub ErdpLo3 [
        ERD_PNTR OFFSET(4) NUMBITS(28) [],
        EHB OFFSET(3) NUMBITS(1) [],
        DESI OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db0R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db0W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db1R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db1W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db2R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db2W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db3R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db3W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db4R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db4W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db5R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db5W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db6R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db6W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db7R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db7W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db8R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db8W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db9R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db9W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db10R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db10W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db11R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db11W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db12R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db12W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db13R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db13W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db14R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db14W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db15R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db15W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db16R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db16W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db17R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db17W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db18R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db18W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db19R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db19W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db20R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db20W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db21R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db21W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db22R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db22W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db23R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db23W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db24R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db24W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db25R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db25W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db26R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db26W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db27R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db27W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db28R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db28W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db29R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db29W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db30R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db30W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db31R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db31W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db32R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db32W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db33R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db33W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db34R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db34W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db35R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db35W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db36R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db36W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db37R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db37W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db38R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db38W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db39R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db39W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db40R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db40W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db41R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db41W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db42R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db42W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db43R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db43W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db44R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db44W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db45R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db45W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db46R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db46W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db47R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db47W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db48R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db48W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db49R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db49W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db50R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db50W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db51R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db51W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db52R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db52W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db53R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db53W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db54R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db54W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db55R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db55W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db56R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db56W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db57R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db57W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db58R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db58W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db59R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db59W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db60R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db60W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db61R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db61W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db62R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db62W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Db63R [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ],
    pub Db63W [
        DB_STREAM_ID OFFSET(16) NUMBITS(16) [],
        DB_TARGET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub UsblegsupR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        HC_OS_OWNED OFFSET(24) NUMBITS(1) [],
        RESERVED1 OFFSET(17) NUMBITS(7) [],
        HC_BIOS_OWNED OFFSET(16) NUMBITS(1) [],
        NEXT_CAPABILITY_POINTER OFFSET(8) NUMBITS(8) [],
        CAPABILITY_ID OFFSET(0) NUMBITS(8) [],
    ],
    pub UsblegsupW [
        HC_OS_OWNED OFFSET(24) NUMBITS(1) [],
        HC_BIOS_OWNED OFFSET(16) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub UsblegctlstsR [
        SMI_ON_BAR OFFSET(31) NUMBITS(1) [],
        SMI_ON_PCI OFFSET(30) NUMBITS(1) [],
        SMI_ON_OS OFFSET(29) NUMBITS(1) [],
        RESERVED0 OFFSET(21) NUMBITS(8) [],
        SMI_ON_HOST OFFSET(20) NUMBITS(1) [],
        RESERVED1 OFFSET(17) NUMBITS(3) [],
        SMI_ON_EVENT OFFSET(16) NUMBITS(1) [],
        SMI_ON_BAR_E OFFSET(15) NUMBITS(1) [],
        SMI_ON_PCI_E OFFSET(14) NUMBITS(1) [],
        SMI_ON_OS_E OFFSET(13) NUMBITS(1) [],
        RESERVED2 OFFSET(5) NUMBITS(8) [],
        SMI_ON_HOST_E OFFSET(4) NUMBITS(1) [],
        RESERVED3 OFFSET(1) NUMBITS(3) [],
        USB_SMI_ENABLE OFFSET(0) NUMBITS(1) [],
    ],
    pub UsblegctlstsW [
        SMI_ON_BAR OFFSET(31) NUMBITS(1) [],
        SMI_ON_PCI OFFSET(30) NUMBITS(1) [],
        SMI_ON_OS OFFSET(29) NUMBITS(1) [],
        SMI_ON_BAR_E OFFSET(15) NUMBITS(1) [],
        SMI_ON_PCI_E OFFSET(14) NUMBITS(1) [],
        SMI_ON_OS_E OFFSET(13) NUMBITS(1) [],
        SMI_ON_HOST_E OFFSET(4) NUMBITS(1) [],
        USB_SMI_ENABLE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Suptprt2Dw0 [
        MAJOR_REVISION OFFSET(24) NUMBITS(8) [],
        MINOR_REVISION OFFSET(16) NUMBITS(8) [],
        NEXT_CAPABILITY_POINTER OFFSET(8) NUMBITS(8) [],
        CAPABILITY_ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Suptprt2Dw2 [
        PSIC OFFSET(28) NUMBITS(4) [],
        MHD OFFSET(25) NUMBITS(3) [],
        BLC OFFSET(20) NUMBITS(1) [],
        HLC OFFSET(19) NUMBITS(1) [],
        IHI OFFSET(18) NUMBITS(1) [],
        HSO OFFSET(17) NUMBITS(1) [],
        COMPATIBLE_PORT_COUNT OFFSET(8) NUMBITS(8) [],
        COMPATIBLE_PORT_OFFSET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Suptprt2Dw3 [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        PROTCL_SLT_TY OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Suptprt3Dw0 [
        MAJOR_REVISION OFFSET(24) NUMBITS(8) [],
        MINOR_REVISION OFFSET(16) NUMBITS(8) [],
        NEXT_CAPABILITY_POINTER OFFSET(8) NUMBITS(8) [],
        CAPABILITY_ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Suptprt3Dw2 [
        PSIC OFFSET(28) NUMBITS(4) [],
        MHD OFFSET(25) NUMBITS(3) [],
        COMPATIBLE_PORT_COUNT OFFSET(8) NUMBITS(8) [],
        COMPATIBLE_PORT_OFFSET OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Suptprt3Dw3 [
        RESERVED0 OFFSET(5) NUMBITS(27) [],
        PROTCL_SLT_TY OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dcid [
        RESERVED0 OFFSET(21) NUMBITS(11) [],
        DCERSTMAX OFFSET(16) NUMBITS(5) [],
        NEXT_CAPABILITY_POINTER OFFSET(8) NUMBITS(8) [],
        CAPABILITY_ID OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcdbR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        DBTARGET OFFSET(8) NUMBITS(8) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ],
    pub DcdbW [
        DBTARGET OFFSET(8) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcerstszR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ],
    pub DcerstszW [
        ERS_TABLE_SIZE OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcerstbaLoR [
        ERS_TABLE_BAR OFFSET(4) NUMBITS(28) [],
        RESERVED0 OFFSET(0) NUMBITS(4) [],
    ],
    pub DcerstbaLoW [
        ERS_TABLE_BAR OFFSET(4) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcerdpLoR [
        DEQUEUE_POINTER OFFSET(4) NUMBITS(28) [],
        RESERVED0 OFFSET(3) NUMBITS(1) [],
        DESI OFFSET(0) NUMBITS(3) [],
    ],
    pub DcerdpLoW [
        DEQUEUE_POINTER OFFSET(4) NUMBITS(28) [],
        DESI OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcctrlR [
        DCE OFFSET(31) NUMBITS(1) [],
        DEVICE_ADDRESS OFFSET(24) NUMBITS(7) [],
        DEBUG_MAX_BURST_SIZE OFFSET(16) NUMBITS(8) [],
        RESERVED0 OFFSET(5) NUMBITS(11) [],
        DRC OFFSET(4) NUMBITS(1) [],
        HIT OFFSET(3) NUMBITS(1) [],
        HOT OFFSET(2) NUMBITS(1) [],
        LSE OFFSET(1) NUMBITS(1) [],
        DCR OFFSET(0) NUMBITS(1) [],
    ],
    pub DcctrlW [
        DCE OFFSET(31) NUMBITS(1) [],
        DRC OFFSET(4) NUMBITS(1) [],
        HIT OFFSET(3) NUMBITS(1) [],
        HOT OFFSET(2) NUMBITS(1) [],
        LSE OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dcst [
        DEBUG_PORT_NUMBER OFFSET(24) NUMBITS(8) [],
        RESERVED0 OFFSET(2) NUMBITS(22) [],
        SBR OFFSET(1) NUMBITS(1) [],
        ER OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcportscR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        CEC OFFSET(23) NUMBITS(1) [],
        PLC OFFSET(22) NUMBITS(1) [],
        PRC OFFSET(21) NUMBITS(1) [],
        RESERVED1 OFFSET(18) NUMBITS(3) [],
        CSC OFFSET(17) NUMBITS(1) [],
        RESERVED2 OFFSET(14) NUMBITS(3) [],
        PORTSPEED OFFSET(10) NUMBITS(4) [],
        RESERVED3 OFFSET(9) NUMBITS(1) [],
        PLS OFFSET(5) NUMBITS(4) [],
        PR OFFSET(4) NUMBITS(1) [],
        RESERVED4 OFFSET(2) NUMBITS(2) [],
        PED OFFSET(1) NUMBITS(1) [],
        CCS OFFSET(0) NUMBITS(1) [],
    ],
    pub DcportscW [
        CEC OFFSET(23) NUMBITS(1) [],
        PLC OFFSET(22) NUMBITS(1) [],
        PRC OFFSET(21) NUMBITS(1) [],
        CSC OFFSET(17) NUMBITS(1) [],
        PED OFFSET(1) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DccpLoR [
        DCCPR OFFSET(4) NUMBITS(28) [],
        RESERVED0 OFFSET(0) NUMBITS(4) [],
    ],
    pub DccpLoW [
        DCCPR OFFSET(4) NUMBITS(28) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dcddi1 [
        VENDORID OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(8) NUMBITS(8) [],
        DBCPROTOCOL OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Dcddi2 [
        DEVICEREV OFFSET(16) NUMBITS(16) [],
        PRODUCTID OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gsbuscfg0R [
        DATRDREQINFO OFFSET(28) NUMBITS(4) [],
        DESRDREQINFO OFFSET(24) NUMBITS(4) [],
        DATWRREQINFO OFFSET(20) NUMBITS(4) [],
        DESWRREQINFO OFFSET(16) NUMBITS(4) [],
        RESERVED0 OFFSET(12) NUMBITS(4) [],
        DATBIGEND OFFSET(11) NUMBITS(1) [],
        DESBIGEND OFFSET(10) NUMBITS(1) [],
        RESERVED1 OFFSET(8) NUMBITS(2) [],
        INCR256BRSTENA OFFSET(7) NUMBITS(1) [],
        INCR128BRSTENA OFFSET(6) NUMBITS(1) [],
        INCR64BRSTENA OFFSET(5) NUMBITS(1) [],
        INCR32BRSTENA OFFSET(4) NUMBITS(1) [],
        INCR16BRSTENA OFFSET(3) NUMBITS(1) [],
        INCR8BRSTENA OFFSET(2) NUMBITS(1) [],
        INCR4BRSTENA OFFSET(1) NUMBITS(1) [],
        INCRBRSTENA OFFSET(0) NUMBITS(1) [],
    ],
    pub Gsbuscfg0W [
        DATRDREQINFO OFFSET(28) NUMBITS(4) [],
        DESRDREQINFO OFFSET(24) NUMBITS(4) [],
        DATWRREQINFO OFFSET(20) NUMBITS(4) [],
        DESWRREQINFO OFFSET(16) NUMBITS(4) [],
        DATBIGEND OFFSET(11) NUMBITS(1) [],
        DESBIGEND OFFSET(10) NUMBITS(1) [],
        INCR256BRSTENA OFFSET(7) NUMBITS(1) [],
        INCR128BRSTENA OFFSET(6) NUMBITS(1) [],
        INCR64BRSTENA OFFSET(5) NUMBITS(1) [],
        INCR32BRSTENA OFFSET(4) NUMBITS(1) [],
        INCR16BRSTENA OFFSET(3) NUMBITS(1) [],
        INCR8BRSTENA OFFSET(2) NUMBITS(1) [],
        INCR4BRSTENA OFFSET(1) NUMBITS(1) [],
        INCRBRSTENA OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gsbuscfg1R [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        EN1KPAGE OFFSET(12) NUMBITS(1) [],
        PIPETRANSLIMIT OFFSET(8) NUMBITS(4) [],
        RESERVED1 OFFSET(0) NUMBITS(8) [],
    ],
    pub Gsbuscfg1W [
        EN1KPAGE OFFSET(12) NUMBITS(1) [],
        PIPETRANSLIMIT OFFSET(8) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GtxthrcfgR [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RESERVED1 OFFSET(30) NUMBITS(1) [],
        USBTXPKTCNTSEL OFFSET(29) NUMBITS(1) [],
        RESERVED2 OFFSET(28) NUMBITS(1) [],
        USBTXPKTCNT OFFSET(24) NUMBITS(4) [],
        USBMAXTXBURSTSIZE OFFSET(16) NUMBITS(8) [],
        RESERVED3 OFFSET(15) NUMBITS(1) [],
        RESERVED4 OFFSET(14) NUMBITS(1) [],
        RESERVED5 OFFSET(11) NUMBITS(3) [],
        RESERVED6 OFFSET(0) NUMBITS(11) [],
    ],
    pub GtxthrcfgW [
        USBTXPKTCNTSEL OFFSET(29) NUMBITS(1) [],
        USBTXPKTCNT OFFSET(24) NUMBITS(4) [],
        USBMAXTXBURSTSIZE OFFSET(16) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GrxthrcfgR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        USBRXPKTCNTSEL OFFSET(29) NUMBITS(1) [],
        RESERVED1 OFFSET(28) NUMBITS(1) [],
        USBRXPKTCNT OFFSET(24) NUMBITS(4) [],
        USBMAXRXBURSTSIZE OFFSET(19) NUMBITS(5) [],
        RESERVED2 OFFSET(16) NUMBITS(3) [],
        RESERVED3 OFFSET(15) NUMBITS(1) [],
        RESERVED4 OFFSET(13) NUMBITS(2) [],
        RESVISOCOUTSPC OFFSET(0) NUMBITS(13) [],
    ],
    pub GrxthrcfgW [
        USBRXPKTCNTSEL OFFSET(29) NUMBITS(1) [],
        USBRXPKTCNT OFFSET(24) NUMBITS(4) [],
        USBMAXRXBURSTSIZE OFFSET(19) NUMBITS(5) [],
        RESVISOCOUTSPC OFFSET(0) NUMBITS(13) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gctl [
        PWRDNSCALE OFFSET(19) NUMBITS(13) [],
        MASTERFILTBYPASS OFFSET(18) NUMBITS(1) [],
        BYPSSETADDR OFFSET(17) NUMBITS(1) [],
        U2RSTECN OFFSET(16) NUMBITS(1) [],
        FRMSCLDWN OFFSET(14) NUMBITS(2) [],
        PRTCAPDIR OFFSET(12) NUMBITS(2) [],
        CORESOFTRESET OFFSET(11) NUMBITS(1) [],
        SOFITPSYNC OFFSET(10) NUMBITS(1) [],
        U1U2TIMERSCALE OFFSET(9) NUMBITS(1) [],
        DEBUGATTACH OFFSET(8) NUMBITS(1) [],
        RAMCLKSEL OFFSET(6) NUMBITS(2) [],
        SCALEDOWN OFFSET(4) NUMBITS(2) [],
        DISSCRAMBLE OFFSET(3) NUMBITS(1) [],
        U2EXIT_LFPS OFFSET(2) NUMBITS(1) [],
        GBLHIBERNATIONEN OFFSET(1) NUMBITS(1) [],
        DSBLCLKGTNG OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GpmstsR [
        RESERVED0 OFFSET(17) NUMBITS(11) [],
        U3WAKEUP OFFSET(12) NUMBITS(5) [],
        RESERVED1 OFFSET(10) NUMBITS(2) [],
        U2WAKEUP OFFSET(0) NUMBITS(10) [],
    ],
    pub GpmstsW [
        PORTSEL OFFSET(28) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GstsR [
        CBELT OFFSET(20) NUMBITS(12) [],
        RESERVED0 OFFSET(12) NUMBITS(8) [],
        SSIC_IP OFFSET(11) NUMBITS(1) [],
        OTG_IP OFFSET(10) NUMBITS(1) [],
        BC_IP OFFSET(9) NUMBITS(1) [],
        ADP_IP OFFSET(8) NUMBITS(1) [],
        HOST_IP OFFSET(7) NUMBITS(1) [],
        DEVICE_IP OFFSET(6) NUMBITS(1) [],
        CSRTIMEOUT OFFSET(5) NUMBITS(1) [],
        BUSERRADDRVLD OFFSET(4) NUMBITS(1) [],
        RESERVED1 OFFSET(2) NUMBITS(2) [],
        CURMOD OFFSET(0) NUMBITS(2) [],
    ],
    pub GstsW [
        CSRTIMEOUT OFFSET(5) NUMBITS(1) [],
        BUSERRADDRVLD OFFSET(4) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Guctl1R [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        FILTER_SE0_FSLS_EOP OFFSET(29) NUMBITS(1) [],
        TX_IPGAP_LINECHECK_DIS OFFSET(28) NUMBITS(1) [],
        DEV_TRB_OUT_SPR_IND OFFSET(27) NUMBITS(1) [],
        DEV_FORCE_20_CLK_FOR_30_CLK OFFSET(26) NUMBITS(1) [],
        P3_IN_U2 OFFSET(25) NUMBITS(1) [],
        DEV_L1_EXIT_BY_HW OFFSET(24) NUMBITS(1) [],
        IP_GAP_ADD_ON OFFSET(21) NUMBITS(3) [],
        DEV_LSP_TAIL_LOCK_DIS OFFSET(20) NUMBITS(1) [],
        NAK_PER_ENH_FS OFFSET(19) NUMBITS(1) [],
        NAK_PER_ENH_HS OFFSET(18) NUMBITS(1) [],
        PARKMODE_DISABLE_SS OFFSET(17) NUMBITS(1) [],
        PARKMODE_DISABLE_HS OFFSET(16) NUMBITS(1) [],
        PARKMODE_DISABLE_FSLS OFFSET(15) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(4) [],
        RESUME_TERMSEL_XCVRSEL_UNIFY OFFSET(10) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        L1_SUSP_THRLD_EN_FOR_HOST OFFSET(8) NUMBITS(1) [],
        L1_SUSP_THRLD_FOR_HOST OFFSET(4) NUMBITS(4) [],
        HC_ERRATA_ENABLE OFFSET(3) NUMBITS(1) [],
        HC_PARCHK_DISABLE OFFSET(2) NUMBITS(1) [],
        OVRLD_L1_SUSP_COM OFFSET(1) NUMBITS(1) [],
        LOA_FILTER_EN OFFSET(0) NUMBITS(1) [],
    ],
    pub Guctl1W [
        FILTER_SE0_FSLS_EOP OFFSET(29) NUMBITS(1) [],
        TX_IPGAP_LINECHECK_DIS OFFSET(28) NUMBITS(1) [],
        DEV_TRB_OUT_SPR_IND OFFSET(27) NUMBITS(1) [],
        DEV_FORCE_20_CLK_FOR_30_CLK OFFSET(26) NUMBITS(1) [],
        P3_IN_U2 OFFSET(25) NUMBITS(1) [],
        DEV_L1_EXIT_BY_HW OFFSET(24) NUMBITS(1) [],
        IP_GAP_ADD_ON OFFSET(21) NUMBITS(3) [],
        DEV_LSP_TAIL_LOCK_DIS OFFSET(20) NUMBITS(1) [],
        NAK_PER_ENH_FS OFFSET(19) NUMBITS(1) [],
        NAK_PER_ENH_HS OFFSET(18) NUMBITS(1) [],
        PARKMODE_DISABLE_SS OFFSET(17) NUMBITS(1) [],
        PARKMODE_DISABLE_HS OFFSET(16) NUMBITS(1) [],
        PARKMODE_DISABLE_FSLS OFFSET(15) NUMBITS(1) [],
        RESUME_TERMSEL_XCVRSEL_UNIFY OFFSET(10) NUMBITS(1) [],
        L1_SUSP_THRLD_EN_FOR_HOST OFFSET(8) NUMBITS(1) [],
        L1_SUSP_THRLD_FOR_HOST OFFSET(4) NUMBITS(4) [],
        HC_ERRATA_ENABLE OFFSET(3) NUMBITS(1) [],
        HC_PARCHK_DISABLE OFFSET(2) NUMBITS(1) [],
        OVRLD_L1_SUSP_COM OFFSET(1) NUMBITS(1) [],
        LOA_FILTER_EN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GgpioR [
        GPO OFFSET(16) NUMBITS(16) [],
        GPI OFFSET(0) NUMBITS(16) [],
    ],
    pub GgpioW [
        GPO OFFSET(16) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GuctlR [
        REFCLKPER OFFSET(22) NUMBITS(10) [],
        NOEXTRDL OFFSET(21) NUMBITS(1) [],
        RESERVED0 OFFSET(18) NUMBITS(3) [],
        SPRSCTRLTRANSEN OFFSET(17) NUMBITS(1) [],
        RESBWHSEPS OFFSET(16) NUMBITS(1) [],
        CMDEVADDR OFFSET(15) NUMBITS(1) [],
        USBHSTINAUTORETRYEN OFFSET(14) NUMBITS(1) [],
        ENOVERLAPCHK OFFSET(13) NUMBITS(1) [],
        EXTCAPSUPPTEN OFFSET(12) NUMBITS(1) [],
        INSRTEXTRFSBODI OFFSET(11) NUMBITS(1) [],
        DTCT OFFSET(9) NUMBITS(2) [],
        DTFT OFFSET(0) NUMBITS(9) [],
    ],
    pub GuctlW [
        REFCLKPER OFFSET(22) NUMBITS(10) [],
        NOEXTRDL OFFSET(21) NUMBITS(1) [],
        SPRSCTRLTRANSEN OFFSET(17) NUMBITS(1) [],
        RESBWHSEPS OFFSET(16) NUMBITS(1) [],
        CMDEVADDR OFFSET(15) NUMBITS(1) [],
        USBHSTINAUTORETRYEN OFFSET(14) NUMBITS(1) [],
        ENOVERLAPCHK OFFSET(13) NUMBITS(1) [],
        EXTCAPSUPPTEN OFFSET(12) NUMBITS(1) [],
        INSRTEXTRFSBODI OFFSET(11) NUMBITS(1) [],
        DTCT OFFSET(9) NUMBITS(2) [],
        DTFT OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GprtbimaploR [
        BINUM8 OFFSET(28) NUMBITS(4) [],
        BINUM7 OFFSET(24) NUMBITS(4) [],
        BINUM6 OFFSET(20) NUMBITS(4) [],
        BINUM5 OFFSET(16) NUMBITS(4) [],
        BINUM4 OFFSET(12) NUMBITS(4) [],
        BINUM3 OFFSET(8) NUMBITS(4) [],
        BINUM2 OFFSET(4) NUMBITS(4) [],
        BINUM1 OFFSET(0) NUMBITS(4) [],
    ],
    pub GprtbimaploW [
        BINUM1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gprtbimaphi [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        BINUM15 OFFSET(24) NUMBITS(4) [],
        BINUM14 OFFSET(20) NUMBITS(4) [],
        BINUM13 OFFSET(16) NUMBITS(4) [],
        BINUM12 OFFSET(12) NUMBITS(4) [],
        BINUM11 OFFSET(8) NUMBITS(4) [],
        BINUM10 OFFSET(4) NUMBITS(4) [],
        BINUM9 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ghwparams0 [
        GHWPARAMS0_31_24 OFFSET(24) NUMBITS(8) [],
        GHWPARAMS0_23_16 OFFSET(16) NUMBITS(8) [],
        GHWPARAMS0_15_8 OFFSET(8) NUMBITS(8) [],
        GHWPARAMS0_7_6 OFFSET(6) NUMBITS(2) [],
        GHWPARAMS0_5_3 OFFSET(3) NUMBITS(3) [],
        GHWPARAMS0_2_0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ghwparams1 [
        GHWPARAMS1_31 OFFSET(31) NUMBITS(1) [],
        GHWPARAMS1_30 OFFSET(30) NUMBITS(1) [],
        GHWPARAMS1_29 OFFSET(29) NUMBITS(1) [],
        GHWPARAMS1_28 OFFSET(28) NUMBITS(1) [],
        GHWPARAMS1_27 OFFSET(27) NUMBITS(1) [],
        GHWPARAMS1_26 OFFSET(26) NUMBITS(1) [],
        GHWPARAMS1_25_24 OFFSET(24) NUMBITS(2) [],
        GHWPARAMS1_23 OFFSET(23) NUMBITS(1) [],
        GHWPARAMS1_22_21 OFFSET(21) NUMBITS(2) [],
        GHWPARAMS1_20_15 OFFSET(15) NUMBITS(6) [],
        GHWPARAMS1_14_12 OFFSET(12) NUMBITS(3) [],
        GHWPARAMS1_11_9 OFFSET(9) NUMBITS(3) [],
        GHWPARAMS1_8_6 OFFSET(6) NUMBITS(3) [],
        GHWPARAMS1_5_3 OFFSET(3) NUMBITS(3) [],
        GHWPARAMS1_2_0 OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ghwparams3 [
        GHWPARAMS3_31 OFFSET(31) NUMBITS(1) [],
        GHWPARAMS3_30_23 OFFSET(23) NUMBITS(8) [],
        GHWPARAMS3_22_18 OFFSET(18) NUMBITS(5) [],
        GHWPARAMS3_17_12 OFFSET(12) NUMBITS(6) [],
        GHWPARAMS3_11 OFFSET(11) NUMBITS(1) [],
        GHWPARAMS3_10 OFFSET(10) NUMBITS(1) [],
        GHWPARAMS3_9_8 OFFSET(8) NUMBITS(2) [],
        GHWPARAMS3_7_6 OFFSET(6) NUMBITS(2) [],
        GHWPARAMS3_5_4 OFFSET(4) NUMBITS(2) [],
        GHWPARAMS3_3_2 OFFSET(2) NUMBITS(2) [],
        GHWPARAMS3_1_0 OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ghwparams4 [
        GHWPARAMS4_31_28 OFFSET(28) NUMBITS(4) [],
        GHWPARAMS4_27_24 OFFSET(24) NUMBITS(4) [],
        GHWPARAMS4_23 OFFSET(23) NUMBITS(1) [],
        GHWPARAMS4_22 OFFSET(22) NUMBITS(1) [],
        GHWPARAMS4_21 OFFSET(21) NUMBITS(1) [],
        GHWPARAMS4_20_17 OFFSET(17) NUMBITS(4) [],
        GHWPARAMS4_16_13 OFFSET(13) NUMBITS(4) [],
        GHWPARAMS4_12 OFFSET(12) NUMBITS(1) [],
        GHWPARAMS4_11 OFFSET(11) NUMBITS(1) [],
        GHWPARAMS4_10_9 OFFSET(9) NUMBITS(2) [],
        GHWPARAMS4_8_7 OFFSET(7) NUMBITS(2) [],
        GHWPARAMS4_6 OFFSET(6) NUMBITS(1) [],
        GHWPARAMS4_5_0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ghwparams5 [
        GHWPARAMS5_31_28 OFFSET(28) NUMBITS(4) [],
        GHWPARAMS5_27_22 OFFSET(22) NUMBITS(6) [],
        GHWPARAMS5_21_16 OFFSET(16) NUMBITS(6) [],
        GHWPARAMS5_15_10 OFFSET(10) NUMBITS(6) [],
        GHWPARAMS5_9_4 OFFSET(4) NUMBITS(6) [],
        GHWPARAMS5_3_0 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ghwparams6 [
        GHWPARAMS6_31_16 OFFSET(16) NUMBITS(16) [],
        BUSFLTRSSUPPORT OFFSET(15) NUMBITS(1) [],
        BCSUPPORT OFFSET(14) NUMBITS(1) [],
        OTG_SS_SUPPORT OFFSET(13) NUMBITS(1) [],
        ADPSUPPORT OFFSET(12) NUMBITS(1) [],
        HNPSUPPORT OFFSET(11) NUMBITS(1) [],
        SRPSUPPORT OFFSET(10) NUMBITS(1) [],
        GHWPARAMS6_9_8 OFFSET(8) NUMBITS(2) [],
        GHWPARAMS6_7 OFFSET(7) NUMBITS(1) [],
        GHWPARAMS6_6 OFFSET(6) NUMBITS(1) [],
        GHWPARAMS6_5_0 OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Ghwparams7 [
        GHWPARAMS7_31_16 OFFSET(16) NUMBITS(16) [],
        GHWPARAMS7_15_0 OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdbgfifospaceR [
        SPACE_AVAILABLE OFFSET(16) NUMBITS(16) [],
        RESERVED0 OFFSET(9) NUMBITS(7) [],
        FIFO_QUEUE_SELECT OFFSET(0) NUMBITS(9) [],
    ],
    pub GdbgfifospaceW [
        FIFO_QUEUE_SELECT OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gdbgltssm [
        RESERVED0 OFFSET(31) NUMBITS(1) [],
        RXELECIDLE OFFSET(30) NUMBITS(1) [],
        X3_XS_SWAPPING OFFSET(29) NUMBITS(1) [],
        X3_DS_HOST_SHUTDOWN OFFSET(28) NUMBITS(1) [],
        PRTDIRECTION OFFSET(27) NUMBITS(1) [],
        LTDBTIMEOUT OFFSET(26) NUMBITS(1) [],
        LTDBLINKSTATE OFFSET(22) NUMBITS(4) [],
        LTDBSUBSTATE OFFSET(18) NUMBITS(4) [],
        ELASTICBUFFERMODE OFFSET(17) NUMBITS(1) [],
        TXELECLDLE OFFSET(16) NUMBITS(1) [],
        RXPOLARITY OFFSET(15) NUMBITS(1) [],
        TXDETRXLOOPBACK OFFSET(14) NUMBITS(1) [],
        LTDBPHYCMDSTATE OFFSET(11) NUMBITS(3) [],
        POWERDOWN OFFSET(9) NUMBITS(2) [],
        RXEQTRAIN OFFSET(8) NUMBITS(1) [],
        TXDEEMPHASIS OFFSET(6) NUMBITS(2) [],
        LTDBCLKSTATE OFFSET(3) NUMBITS(3) [],
        TXSWING OFFSET(2) NUMBITS(1) [],
        RXTERMINATION OFFSET(1) NUMBITS(1) [],
        TXONESZEROS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gdbglnmcc [
        RESERVED0 OFFSET(9) NUMBITS(23) [],
        LNMCC_BERC OFFSET(0) NUMBITS(9) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gdbgbmu [
        BMU_BCU OFFSET(8) NUMBITS(24) [],
        BMU_DCU OFFSET(4) NUMBITS(4) [],
        BMU_CCU OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdbglspmuxHstR [
        RESERVED0 OFFSET(24) NUMBITS(8) [],
        LOGIC_ANALYZER_TRACE OFFSET(16) NUMBITS(8) [],
        RESERVED1 OFFSET(14) NUMBITS(2) [],
        HOSTSELECT OFFSET(0) NUMBITS(14) [],
    ],
    pub GdbglspmuxHstW [
        LOGIC_ANALYZER_TRACE OFFSET(16) NUMBITS(8) [],
        HOSTSELECT OFFSET(0) NUMBITS(14) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GprtbimapHsloR [
        BINUM8 OFFSET(28) NUMBITS(4) [],
        BINUM7 OFFSET(24) NUMBITS(4) [],
        BINUM6 OFFSET(20) NUMBITS(4) [],
        BINUM5 OFFSET(16) NUMBITS(4) [],
        BINUM4 OFFSET(12) NUMBITS(4) [],
        BINUM3 OFFSET(8) NUMBITS(4) [],
        BINUM2 OFFSET(4) NUMBITS(4) [],
        BINUM1 OFFSET(0) NUMBITS(4) [],
    ],
    pub GprtbimapHsloW [
        BINUM1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GprtbimapHshi [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        BINUM15 OFFSET(24) NUMBITS(4) [],
        BINUM14 OFFSET(20) NUMBITS(4) [],
        BINUM13 OFFSET(16) NUMBITS(4) [],
        BINUM12 OFFSET(12) NUMBITS(4) [],
        BINUM11 OFFSET(8) NUMBITS(4) [],
        BINUM10 OFFSET(4) NUMBITS(4) [],
        BINUM9 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GprtbimapFsloR [
        BINUM8 OFFSET(28) NUMBITS(4) [],
        BINUM7 OFFSET(24) NUMBITS(4) [],
        BINUM6 OFFSET(20) NUMBITS(4) [],
        BINUM5 OFFSET(16) NUMBITS(4) [],
        BINUM4 OFFSET(12) NUMBITS(4) [],
        BINUM3 OFFSET(8) NUMBITS(4) [],
        BINUM2 OFFSET(4) NUMBITS(4) [],
        BINUM1 OFFSET(0) NUMBITS(4) [],
    ],
    pub GprtbimapFsloW [
        BINUM1 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GprtbimapFshi [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        BINUM15 OFFSET(24) NUMBITS(4) [],
        BINUM14 OFFSET(20) NUMBITS(4) [],
        BINUM13 OFFSET(16) NUMBITS(4) [],
        BINUM12 OFFSET(12) NUMBITS(4) [],
        BINUM11 OFFSET(8) NUMBITS(4) [],
        BINUM10 OFFSET(4) NUMBITS(4) [],
        BINUM9 OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gusb2phycfgR [
        PHYSOFTRST OFFSET(31) NUMBITS(1) [],
        U2_FREECLK_EXISTS OFFSET(30) NUMBITS(1) [],
        ULPI_LPM_WITH_OPMODE_CHK OFFSET(29) NUMBITS(1) [],
        HSIC_CON_WIDTH_ADJ OFFSET(27) NUMBITS(2) [],
        INV_SEL_HSIC OFFSET(26) NUMBITS(1) [],
        RESERVED0 OFFSET(25) NUMBITS(1) [],
        LSTRD OFFSET(22) NUMBITS(3) [],
        LSIPD OFFSET(19) NUMBITS(3) [],
        ULPIEXTVBUSINDIACTOR OFFSET(18) NUMBITS(1) [],
        ULPIEXTVBUSDRV OFFSET(17) NUMBITS(1) [],
        RESERVED1 OFFSET(16) NUMBITS(1) [],
        ULPIAUTORES OFFSET(15) NUMBITS(1) [],
        RESERVED2 OFFSET(14) NUMBITS(1) [],
        USBTRDTIM OFFSET(10) NUMBITS(4) [],
        XCVRDLY OFFSET(9) NUMBITS(1) [],
        ENBLSLPM OFFSET(8) NUMBITS(1) [],
        SUSPENDUSB20 OFFSET(6) NUMBITS(1) [],
        FSINTF OFFSET(5) NUMBITS(1) [],
        ULPI_UTMI_SEL OFFSET(4) NUMBITS(1) [],
        PHYIF OFFSET(3) NUMBITS(1) [],
        TOUTCAL OFFSET(0) NUMBITS(3) [],
    ],
    pub Gusb2phycfgW [
        PHYSOFTRST OFFSET(31) NUMBITS(1) [],
        U2_FREECLK_EXISTS OFFSET(30) NUMBITS(1) [],
        ULPI_LPM_WITH_OPMODE_CHK OFFSET(29) NUMBITS(1) [],
        LSTRD OFFSET(22) NUMBITS(3) [],
        LSIPD OFFSET(19) NUMBITS(3) [],
        ULPIEXTVBUSINDIACTOR OFFSET(18) NUMBITS(1) [],
        ULPIEXTVBUSDRV OFFSET(17) NUMBITS(1) [],
        ULPIAUTORES OFFSET(15) NUMBITS(1) [],
        USBTRDTIM OFFSET(10) NUMBITS(4) [],
        XCVRDLY OFFSET(9) NUMBITS(1) [],
        ENBLSLPM OFFSET(8) NUMBITS(1) [],
        PHYSEL OFFSET(7) NUMBITS(1) [],
        SUSPENDUSB20 OFFSET(6) NUMBITS(1) [],
        PHYIF OFFSET(3) NUMBITS(1) [],
        TOUTCAL OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gusb2phyaccUlpiR [
        RESERVED0 OFFSET(27) NUMBITS(5) [],
        DISUIPIDRVR OFFSET(26) NUMBITS(1) [],
        NEWREGREQ OFFSET(25) NUMBITS(1) [],
        VSTSDONE OFFSET(24) NUMBITS(1) [],
        VSTSBSY OFFSET(23) NUMBITS(1) [],
        REGWR OFFSET(22) NUMBITS(1) [],
        REGADDR OFFSET(16) NUMBITS(6) [],
        EXTREGADDR OFFSET(8) NUMBITS(8) [],
        REGDATA OFFSET(0) NUMBITS(8) [],
    ],
    pub Gusb2phyaccUlpiW [
        NEWREGREQ OFFSET(25) NUMBITS(1) [],
        VSTSBSY OFFSET(23) NUMBITS(1) [],
        REGWR OFFSET(22) NUMBITS(1) [],
        REGADDR OFFSET(16) NUMBITS(6) [],
        EXTREGADDR OFFSET(8) NUMBITS(8) [],
        REGDATA OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gusb3pipectlR [
        PHYSOFTRST OFFSET(31) NUMBITS(1) [],
        HSTPRTCMPL OFFSET(30) NUMBITS(1) [],
        U2SSINACTP3OK OFFSET(29) NUMBITS(1) [],
        DISRXDETP3 OFFSET(28) NUMBITS(1) [],
        UX_EXIT_IN_PX OFFSET(27) NUMBITS(1) [],
        PING_ENHANCEMENT_EN OFFSET(26) NUMBITS(1) [],
        U1U2EXITFAIL_TO_RECOV OFFSET(25) NUMBITS(1) [],
        REQUEST_P1P2P3 OFFSET(24) NUMBITS(1) [],
        DISRXDETU3RXDET OFFSET(22) NUMBITS(1) [],
        DELAYP1P2P3 OFFSET(19) NUMBITS(3) [],
        DELAYP1TRANS OFFSET(18) NUMBITS(1) [],
        SUSPENDENABLE OFFSET(17) NUMBITS(1) [],
        DATWIDTH OFFSET(15) NUMBITS(2) [],
        ABORTRXDETINU2 OFFSET(14) NUMBITS(1) [],
        SKIPRXDET OFFSET(13) NUMBITS(1) [],
        LFPSP0ALGN OFFSET(12) NUMBITS(1) [],
        P3P2TRANOK OFFSET(11) NUMBITS(1) [],
        P3EXSIGP2 OFFSET(10) NUMBITS(1) [],
        LFPSFILTER OFFSET(9) NUMBITS(1) [],
        RX_DETECT_TO_POLLING_LFPS_CONTROL OFFSET(8) NUMBITS(1) [],
        SSICEN OFFSET(7) NUMBITS(1) [],
        TX_SWING OFFSET(6) NUMBITS(1) [],
        TX_MARGIN OFFSET(3) NUMBITS(3) [],
        TX_DE_EPPHASIS OFFSET(1) NUMBITS(2) [],
        ELASTIC_BUFFER_MODE OFFSET(0) NUMBITS(1) [],
    ],
    pub Gusb3pipectlW [
        PHYSOFTRST OFFSET(31) NUMBITS(1) [],
        HSTPRTCMPL OFFSET(30) NUMBITS(1) [],
        U2SSINACTP3OK OFFSET(29) NUMBITS(1) [],
        DISRXDETP3 OFFSET(28) NUMBITS(1) [],
        UX_EXIT_IN_PX OFFSET(27) NUMBITS(1) [],
        PING_ENHANCEMENT_EN OFFSET(26) NUMBITS(1) [],
        U1U2EXITFAIL_TO_RECOV OFFSET(25) NUMBITS(1) [],
        REQUEST_P1P2P3 OFFSET(24) NUMBITS(1) [],
        STARTRXDETU3RXDET OFFSET(23) NUMBITS(1) [],
        DISRXDETU3RXDET OFFSET(22) NUMBITS(1) [],
        DELAYP1P2P3 OFFSET(19) NUMBITS(3) [],
        DELAYP1TRANS OFFSET(18) NUMBITS(1) [],
        SUSPENDENABLE OFFSET(17) NUMBITS(1) [],
        ABORTRXDETINU2 OFFSET(14) NUMBITS(1) [],
        SKIPRXDET OFFSET(13) NUMBITS(1) [],
        LFPSP0ALGN OFFSET(12) NUMBITS(1) [],
        P3P2TRANOK OFFSET(11) NUMBITS(1) [],
        P3EXSIGP2 OFFSET(10) NUMBITS(1) [],
        LFPSFILTER OFFSET(9) NUMBITS(1) [],
        RX_DETECT_TO_POLLING_LFPS_CONTROL OFFSET(8) NUMBITS(1) [],
        SSICEN OFFSET(7) NUMBITS(1) [],
        TX_SWING OFFSET(6) NUMBITS(1) [],
        TX_MARGIN OFFSET(3) NUMBITS(3) [],
        TX_DE_EPPHASIS OFFSET(1) NUMBITS(2) [],
        ELASTIC_BUFFER_MODE OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gtxfifosiz0 [
        TXFSTADDR_N OFFSET(16) NUMBITS(16) [],
        TXFDEP_N OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gtxfifosiz1 [
        TXFSTADDR_N OFFSET(16) NUMBITS(16) [],
        TXFDEP_N OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gtxfifosiz2 [
        TXFSTADDR_N OFFSET(16) NUMBITS(16) [],
        TXFDEP_N OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gtxfifosiz3 [
        TXFSTADDR_N OFFSET(16) NUMBITS(16) [],
        TXFDEP_N OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gtxfifosiz4 [
        TXFSTADDR_N OFFSET(16) NUMBITS(16) [],
        TXFDEP_N OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gtxfifosiz5 [
        TXFSTADDR_N OFFSET(16) NUMBITS(16) [],
        TXFDEP_N OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Grxfifosiz0 [
        RXFSTADDR_N OFFSET(16) NUMBITS(16) [],
        RXFDEP_N OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Grxfifosiz1 [
        RXFSTADDR_N OFFSET(16) NUMBITS(16) [],
        RXFDEP_N OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Grxfifosiz2 [
        RXFSTADDR_N OFFSET(16) NUMBITS(16) [],
        RXFDEP_N OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gevntsiz0R [
        EVNTINTRPTMASK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        EVENTSIZ OFFSET(0) NUMBITS(16) [],
    ],
    pub Gevntsiz0W [
        EVNTINTRPTMASK OFFSET(31) NUMBITS(1) [],
        EVENTSIZ OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gevntcount0R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        EVNTCOUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub Gevntcount0W [
        EVNTCOUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gevntsiz1R [
        EVNTINTRPTMASK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        EVENTSIZ OFFSET(0) NUMBITS(16) [],
    ],
    pub Gevntsiz1W [
        EVNTINTRPTMASK OFFSET(31) NUMBITS(1) [],
        EVENTSIZ OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gevntcount1R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        EVNTCOUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub Gevntcount1W [
        EVNTCOUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gevntsiz2R [
        EVNTINTRPTMASK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        EVENTSIZ OFFSET(0) NUMBITS(16) [],
    ],
    pub Gevntsiz2W [
        EVNTINTRPTMASK OFFSET(31) NUMBITS(1) [],
        EVENTSIZ OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gevntcount2R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        EVNTCOUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub Gevntcount2W [
        EVNTCOUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gevntsiz3R [
        EVNTINTRPTMASK OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(16) NUMBITS(15) [],
        EVENTSIZ OFFSET(0) NUMBITS(16) [],
    ],
    pub Gevntsiz3W [
        EVNTINTRPTMASK OFFSET(31) NUMBITS(1) [],
        EVENTSIZ OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Gevntcount3R [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        EVNTCOUNT OFFSET(0) NUMBITS(16) [],
    ],
    pub Gevntcount3W [
        EVNTCOUNT OFFSET(0) NUMBITS(16) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GtxfifopridevR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        GTXFIFOPRIDEV OFFSET(0) NUMBITS(6) [],
    ],
    pub GtxfifopridevW [
        GTXFIFOPRIDEV OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GtxfifoprihstR [
        RESERVED0 OFFSET(4) NUMBITS(28) [],
        GTXFIFOPRIHST OFFSET(0) NUMBITS(4) [],
    ],
    pub GtxfifoprihstW [
        GTXFIFOPRIHST OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GrxfifoprihstR [
        RESERVED0 OFFSET(3) NUMBITS(29) [],
        GRXFIFOPRIHST OFFSET(0) NUMBITS(3) [],
    ],
    pub GrxfifoprihstW [
        GRXFIFOPRIHST OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GfifopridbcR [
        RESERVED0 OFFSET(2) NUMBITS(30) [],
        GFIFOPRIDBC OFFSET(0) NUMBITS(2) [],
    ],
    pub GfifopridbcW [
        GFIFOPRIDBC OFFSET(0) NUMBITS(2) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GdmahlratioR [
        RESERVED0 OFFSET(13) NUMBITS(19) [],
        HSTRXFIFO OFFSET(8) NUMBITS(5) [],
        RESERVED1 OFFSET(5) NUMBITS(3) [],
        HSTTXFIFO OFFSET(0) NUMBITS(5) [],
    ],
    pub GdmahlratioW [
        HSTRXFIFO OFFSET(8) NUMBITS(5) [],
        HSTTXFIFO OFFSET(0) NUMBITS(5) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub GfladjR [
        GFLADJ_REFCLK_240MHZDECR_PLS1 OFFSET(31) NUMBITS(1) [],
        GFLADJ_REFCLK_240MHZ_DECR OFFSET(24) NUMBITS(7) [],
        GFLADJ_REFCLK_LPM_SEL OFFSET(23) NUMBITS(1) [],
        RESERVED0 OFFSET(22) NUMBITS(1) [],
        GFLADJ_REFCLK_FLADJ OFFSET(8) NUMBITS(14) [],
        GFLADJ_30MHZ_SDBND_SEL OFFSET(7) NUMBITS(1) [],
        RESERVED1 OFFSET(6) NUMBITS(1) [],
        GFLADJ_30MHZ OFFSET(0) NUMBITS(6) [],
    ],
    pub GfladjW [
        GFLADJ_REFCLK_240MHZDECR_PLS1 OFFSET(31) NUMBITS(1) [],
        GFLADJ_REFCLK_240MHZ_DECR OFFSET(24) NUMBITS(7) [],
        GFLADJ_REFCLK_LPM_SEL OFFSET(23) NUMBITS(1) [],
        GFLADJ_REFCLK_FLADJ OFFSET(8) NUMBITS(14) [],
        GFLADJ_30MHZ_SDBND_SEL OFFSET(7) NUMBITS(1) [],
        GFLADJ_30MHZ OFFSET(0) NUMBITS(6) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DcfgR [
        RESERVED0 OFFSET(25) NUMBITS(7) [],
        RESERVED1 OFFSET(24) NUMBITS(1) [],
        IGNSTRMPP OFFSET(23) NUMBITS(1) [],
        LPMCAP OFFSET(22) NUMBITS(1) [],
        NUMP OFFSET(17) NUMBITS(5) [],
        INTRNUM OFFSET(12) NUMBITS(5) [],
        RESERVED2 OFFSET(10) NUMBITS(2) [],
        DEVADDR OFFSET(3) NUMBITS(7) [],
        DEVSPD OFFSET(0) NUMBITS(3) [],
    ],
    pub DcfgW [
        IGNSTRMPP OFFSET(23) NUMBITS(1) [],
        LPMCAP OFFSET(22) NUMBITS(1) [],
        NUMP OFFSET(17) NUMBITS(5) [],
        INTRNUM OFFSET(12) NUMBITS(5) [],
        DEVADDR OFFSET(3) NUMBITS(7) [],
        DEVSPD OFFSET(0) NUMBITS(3) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DctlR [
        RUN_STOP OFFSET(31) NUMBITS(1) [],
        CSFTRST OFFSET(30) NUMBITS(1) [],
        RESERVED0 OFFSET(29) NUMBITS(1) [],
        HIRDTHRES OFFSET(24) NUMBITS(5) [],
        LPM_NYET_THRES OFFSET(20) NUMBITS(4) [],
        KEEPCONNECT OFFSET(19) NUMBITS(1) [],
        L1HIBERNATIONEN OFFSET(18) NUMBITS(1) [],
        CRS OFFSET(17) NUMBITS(1) [],
        CSS OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(13) NUMBITS(3) [],
        INITU2ENA OFFSET(12) NUMBITS(1) [],
        ACCEPTU2ENA OFFSET(11) NUMBITS(1) [],
        INITU1ENA OFFSET(10) NUMBITS(1) [],
        ACCEPTU1ENA OFFSET(9) NUMBITS(1) [],
        TSTCTL OFFSET(1) NUMBITS(4) [],
        RESERVED2 OFFSET(0) NUMBITS(1) [],
    ],
    pub DctlW [
        RUN_STOP OFFSET(31) NUMBITS(1) [],
        CSFTRST OFFSET(30) NUMBITS(1) [],
        HIRDTHRES OFFSET(24) NUMBITS(5) [],
        LPM_NYET_THRES OFFSET(20) NUMBITS(4) [],
        KEEPCONNECT OFFSET(19) NUMBITS(1) [],
        L1HIBERNATIONEN OFFSET(18) NUMBITS(1) [],
        CRS OFFSET(17) NUMBITS(1) [],
        CSS OFFSET(16) NUMBITS(1) [],
        INITU2ENA OFFSET(12) NUMBITS(1) [],
        ACCEPTU2ENA OFFSET(11) NUMBITS(1) [],
        INITU1ENA OFFSET(10) NUMBITS(1) [],
        ACCEPTU1ENA OFFSET(9) NUMBITS(1) [],
        ULSTCHNGREQ OFFSET(5) NUMBITS(4) [],
        TSTCTL OFFSET(1) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DevtenR [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        VENDEVTSTRCVDEN OFFSET(12) NUMBITS(1) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        RESERVED2 OFFSET(10) NUMBITS(1) [],
        ERRTICERREVTEN OFFSET(9) NUMBITS(1) [],
        RESERVED3 OFFSET(8) NUMBITS(1) [],
        SOFTEVTEN OFFSET(7) NUMBITS(1) [],
        U3L2L1SUSPEN OFFSET(6) NUMBITS(1) [],
        HIBERNATIONREQEVTEN OFFSET(5) NUMBITS(1) [],
        WKUPEVTEN OFFSET(4) NUMBITS(1) [],
        ULSTCNGEN OFFSET(3) NUMBITS(1) [],
        CONNECTDONEEVTEN OFFSET(2) NUMBITS(1) [],
        USBRSTEVTEN OFFSET(1) NUMBITS(1) [],
        DISSCONNEVTEN OFFSET(0) NUMBITS(1) [],
    ],
    pub DevtenW [
        VENDEVTSTRCVDEN OFFSET(12) NUMBITS(1) [],
        ERRTICERREVTEN OFFSET(9) NUMBITS(1) [],
        SOFTEVTEN OFFSET(7) NUMBITS(1) [],
        U3L2L1SUSPEN OFFSET(6) NUMBITS(1) [],
        HIBERNATIONREQEVTEN OFFSET(5) NUMBITS(1) [],
        WKUPEVTEN OFFSET(4) NUMBITS(1) [],
        ULSTCNGEN OFFSET(3) NUMBITS(1) [],
        CONNECTDONEEVTEN OFFSET(2) NUMBITS(1) [],
        USBRSTEVTEN OFFSET(1) NUMBITS(1) [],
        DISSCONNEVTEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DstsR [
        RESERVED0 OFFSET(30) NUMBITS(2) [],
        DCNRD OFFSET(29) NUMBITS(1) [],
        SRE OFFSET(28) NUMBITS(1) [],
        RESERVED1 OFFSET(26) NUMBITS(2) [],
        RSS OFFSET(25) NUMBITS(1) [],
        SSS OFFSET(24) NUMBITS(1) [],
        COREIDLE OFFSET(23) NUMBITS(1) [],
        DEVCTRLHLT OFFSET(22) NUMBITS(1) [],
        USBLNKST OFFSET(18) NUMBITS(4) [],
        RXFIFOEMPTY OFFSET(17) NUMBITS(1) [],
        SOFFN OFFSET(3) NUMBITS(14) [],
        CONNECTSPD OFFSET(0) NUMBITS(3) [],
    ],
    pub DstsW [
        SRE OFFSET(28) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub DgcmdR [
        RESERVED0 OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        RESERVED1 OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED2 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(8) [],
    ],
    pub DgcmdW [
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(8) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd0R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd0W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd1R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd1W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd2R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd2W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd3R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd3W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd4R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd4W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd5R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd5W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd6R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd6W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd7R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd7W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd8R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd8W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd9R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd9W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd10R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd10W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Depcmd11R [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        RESERVED0 OFFSET(9) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        RESERVED1 OFFSET(4) NUMBITS(4) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ],
    pub Depcmd11W [
        COMMANDPARAM OFFSET(16) NUMBITS(16) [],
        CMDSTATUS OFFSET(12) NUMBITS(4) [],
        HIPRI_FORCERM OFFSET(11) NUMBITS(1) [],
        CMDACT OFFSET(10) NUMBITS(1) [],
        CMDIOC OFFSET(8) NUMBITS(1) [],
        CMDTYP OFFSET(0) NUMBITS(4) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OcfgR [
        RESERVED0 OFFSET(6) NUMBITS(26) [],
        DISPRTPWRCUTOFF OFFSET(5) NUMBITS(1) [],
        OTGHIBDISMASK OFFSET(4) NUMBITS(1) [],
        OTGSFTRSTMSK OFFSET(3) NUMBITS(1) [],
        OTG_VERSION OFFSET(2) NUMBITS(1) [],
        HNPCAP OFFSET(1) NUMBITS(1) [],
        SRPCAP OFFSET(0) NUMBITS(1) [],
    ],
    pub OcfgW [
        DISPRTPWRCUTOFF OFFSET(5) NUMBITS(1) [],
        OTGHIBDISMASK OFFSET(4) NUMBITS(1) [],
        OTGSFTRSTMSK OFFSET(3) NUMBITS(1) [],
        OTG_VERSION OFFSET(2) NUMBITS(1) [],
        HNPCAP OFFSET(1) NUMBITS(1) [],
        SRPCAP OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OctlR [
        RESERVED0 OFFSET(8) NUMBITS(24) [],
        OTG3_GOERR OFFSET(7) NUMBITS(1) [],
        PERIMODE OFFSET(6) NUMBITS(1) [],
        PRTPWRCTL OFFSET(5) NUMBITS(1) [],
        HNPREQ OFFSET(4) NUMBITS(1) [],
        TERMSELDLPULSE OFFSET(2) NUMBITS(1) [],
        DEVSETHNPEN OFFSET(1) NUMBITS(1) [],
        HSTSETHNPEN OFFSET(0) NUMBITS(1) [],
    ],
    pub OctlW [
        OTG3_GOERR OFFSET(7) NUMBITS(1) [],
        PERIMODE OFFSET(6) NUMBITS(1) [],
        PRTPWRCTL OFFSET(5) NUMBITS(1) [],
        HNPREQ OFFSET(4) NUMBITS(1) [],
        SESREQ OFFSET(3) NUMBITS(1) [],
        TERMSELDLPULSE OFFSET(2) NUMBITS(1) [],
        DEVSETHNPEN OFFSET(1) NUMBITS(1) [],
        HSTSETHNPEN OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OevtR [
        DEVICEMODE OFFSET(31) NUMBITS(1) [],
        RESERVED0 OFFSET(28) NUMBITS(3) [],
        OTGXHCIRUNSTPSETEVNT OFFSET(27) NUMBITS(1) [],
        OTGDEVRUNSTPSETEVNT OFFSET(26) NUMBITS(1) [],
        OTGHIBENTRYEVNT OFFSET(25) NUMBITS(1) [],
        OTGCONIDSTSCHNGEVNT OFFSET(24) NUMBITS(1) [],
        HRRCONFNOTIFEVNT OFFSET(23) NUMBITS(1) [],
        HRRINITNOTIFEVNT OFFSET(22) NUMBITS(1) [],
        OTGADEVIDLEEVNT OFFSET(21) NUMBITS(1) [],
        OTGADEVBHOSTENDEVNT OFFSET(20) NUMBITS(1) [],
        OTGADEVHOSTEVNT OFFSET(19) NUMBITS(1) [],
        OTGADEVHNPCHNGEVNT OFFSET(18) NUMBITS(1) [],
        OTGADEVSRPDETEVNT OFFSET(17) NUMBITS(1) [],
        OTGADEVSESSENDDETEVNT OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        OTGBDEVBHOSTENDEVNT OFFSET(11) NUMBITS(1) [],
        OTGBDEVHNPCHNGEVNT OFFSET(10) NUMBITS(1) [],
        OTGBDEVSESSVLDDETEVNT OFFSET(9) NUMBITS(1) [],
        OTGBDEVVBUSCHNGEVNT OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(4) NUMBITS(4) [],
        BSESVLD OFFSET(3) NUMBITS(1) [],
        HSTNEGSTS OFFSET(2) NUMBITS(1) [],
        SESREQSTS OFFSET(1) NUMBITS(1) [],
        OEVTERROR OFFSET(0) NUMBITS(1) [],
    ],
    pub OevtW [
        OTGXHCIRUNSTPSETEVNT OFFSET(27) NUMBITS(1) [],
        OTGDEVRUNSTPSETEVNT OFFSET(26) NUMBITS(1) [],
        OTGHIBENTRYEVNT OFFSET(25) NUMBITS(1) [],
        OTGCONIDSTSCHNGEVNT OFFSET(24) NUMBITS(1) [],
        HRRCONFNOTIFEVNT OFFSET(23) NUMBITS(1) [],
        HRRINITNOTIFEVNT OFFSET(22) NUMBITS(1) [],
        OTGADEVIDLEEVNT OFFSET(21) NUMBITS(1) [],
        OTGADEVBHOSTENDEVNT OFFSET(20) NUMBITS(1) [],
        OTGADEVHOSTEVNT OFFSET(19) NUMBITS(1) [],
        OTGADEVHNPCHNGEVNT OFFSET(18) NUMBITS(1) [],
        OTGADEVSRPDETEVNT OFFSET(17) NUMBITS(1) [],
        OTGADEVSESSENDDETEVNT OFFSET(16) NUMBITS(1) [],
        OTGBDEVBHOSTENDEVNT OFFSET(11) NUMBITS(1) [],
        OTGBDEVHNPCHNGEVNT OFFSET(10) NUMBITS(1) [],
        OTGBDEVSESSVLDDETEVNT OFFSET(9) NUMBITS(1) [],
        OTGBDEVVBUSCHNGEVNT OFFSET(8) NUMBITS(1) [],
        OEVTERROR OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub OevtenR [
        RESERVED0 OFFSET(28) NUMBITS(4) [],
        OTGXHCIRUNSTPSETEVNTEN OFFSET(27) NUMBITS(1) [],
        OTGDEVRUNSTPSETEVNTEN OFFSET(26) NUMBITS(1) [],
        OTGHIBENTRYEVNTEN OFFSET(25) NUMBITS(1) [],
        OTGCONIDSTSCHNGEVNTEN OFFSET(24) NUMBITS(1) [],
        HRRCONFNOTIFEVNTEN OFFSET(23) NUMBITS(1) [],
        HRRINITNOTIFEVNTEN OFFSET(22) NUMBITS(1) [],
        OTGADEVIDLEEVNTEN OFFSET(21) NUMBITS(1) [],
        OTGADEVBHOSTENDEVNTEN OFFSET(20) NUMBITS(1) [],
        OTGADEVHOSTEVNTEN OFFSET(19) NUMBITS(1) [],
        OTGADEVHNPCHNGEVNTEN OFFSET(18) NUMBITS(1) [],
        OTGADEVSRPDETEVNTEN OFFSET(17) NUMBITS(1) [],
        OTGADEVSESSENDDETEVNTEN OFFSET(16) NUMBITS(1) [],
        RESERVED1 OFFSET(12) NUMBITS(4) [],
        OTGBDEVBHOSTENDEVNTEN OFFSET(11) NUMBITS(1) [],
        OTGBDEVHNPCHNGEVNTEN OFFSET(10) NUMBITS(1) [],
        OTGBDEVSESSVLDDETEVNTEN OFFSET(9) NUMBITS(1) [],
        OTGBDEVVBUSCHNGEVNTEN OFFSET(8) NUMBITS(1) [],
        RESERVED2 OFFSET(0) NUMBITS(8) [],
    ],
    pub OevtenW [
        OTGXHCIRUNSTPSETEVNTEN OFFSET(27) NUMBITS(1) [],
        OTGDEVRUNSTPSETEVNTEN OFFSET(26) NUMBITS(1) [],
        OTGHIBENTRYEVNTEN OFFSET(25) NUMBITS(1) [],
        OTGCONIDSTSCHNGEVNTEN OFFSET(24) NUMBITS(1) [],
        HRRCONFNOTIFEVNTEN OFFSET(23) NUMBITS(1) [],
        HRRINITNOTIFEVNTEN OFFSET(22) NUMBITS(1) [],
        OTGADEVIDLEEVNTEN OFFSET(21) NUMBITS(1) [],
        OTGADEVBHOSTENDEVNTEN OFFSET(20) NUMBITS(1) [],
        OTGADEVHOSTEVNTEN OFFSET(19) NUMBITS(1) [],
        OTGADEVHNPCHNGEVNTEN OFFSET(18) NUMBITS(1) [],
        OTGADEVSRPDETEVNTEN OFFSET(17) NUMBITS(1) [],
        OTGADEVSESSENDDETEVNTEN OFFSET(16) NUMBITS(1) [],
        OTGBDEVBHOSTENDEVNTEN OFFSET(11) NUMBITS(1) [],
        OTGBDEVHNPCHNGEVNTEN OFFSET(10) NUMBITS(1) [],
        OTGBDEVSESSVLDDETEVNTEN OFFSET(9) NUMBITS(1) [],
        OTGBDEVVBUSCHNGEVNTEN OFFSET(8) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub Osts [
        RESERVED0 OFFSET(14) NUMBITS(18) [],
        DEVRUNSTP OFFSET(13) NUMBITS(1) [],
        XHCIRUNSTP OFFSET(12) NUMBITS(1) [],
        OTGSTATE OFFSET(8) NUMBITS(4) [],
        RESERVED1 OFFSET(5) NUMBITS(3) [],
        PERIPHERALSTATE OFFSET(4) NUMBITS(1) [],
        XHCIPRTPOWER OFFSET(3) NUMBITS(1) [],
        BSESVLD OFFSET(2) NUMBITS(1) [],
        ASESVLD OFFSET(1) NUMBITS(1) [],
        CONIDSTS OFFSET(0) NUMBITS(1) [],
    ]
];
tock_registers::register_bitfields! [
    u32,
    pub AdpevtR [
        RESERVED0 OFFSET(29) NUMBITS(3) [],
        ADPPRBEVNT OFFSET(28) NUMBITS(1) [],
        ADPSNSEVNT OFFSET(27) NUMBITS(1) [],
        ADPTMOUTEVNT OFFSET(26) NUMBITS(1) [],
        ADPRSTCMPLTEVNT OFFSET(25) NUMBITS(1) [],
        RESERVED1 OFFSET(16) NUMBITS(9) [],
        RTIM OFFSET(0) NUMBITS(16) [],
    ],
    pub AdpevtW [
        ADPPRBEVNT OFFSET(28) NUMBITS(1) [],
        ADPSNSEVNT OFFSET(27) NUMBITS(1) [],
        ADPTMOUTEVNT OFFSET(26) NUMBITS(1) [],
        ADPRSTCMPLTEVNT OFFSET(25) NUMBITS(1) [],
    ]
];

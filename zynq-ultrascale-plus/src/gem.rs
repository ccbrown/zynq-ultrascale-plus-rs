use managed::{Managed, ManagedSlice};
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    registers::InMemoryRegister,
};
use zynq_ultrascale_plus_modules::gem::{self, *};

pub struct Controller {
    registers: &'static Registers,
}

/// Provides all of the storage required for GEM operation.
///
/// There are very specific requirements and recommendations for these objects. Refer to the
/// reference manual for details.
///
/// At a minimum:
///
///  - There must be the same number of receive descriptors and buffers.
///  - Receive buffers must be a multiple of 64 bytes in length.
///  - Receive buffers must be 64-byte aligned.
pub struct Storage<'a> {
    pub receive_descriptors: ManagedSlice<'a, ReceiveDescriptor>,
    pub receive_buffers: ManagedSlice<'a, ManagedSlice<'a, u8>>,
    pub transmit_descriptor: Managed<'a, TransmitDescriptor>,
    pub transmit_buffer: ManagedSlice<'a, u8>,
}

impl<'a> Storage<'a> {
    pub fn receive_buffer_len(&self) -> usize {
        self.receive_buffers[0].len()
    }

    pub fn transmit_buffer_len(&self) -> usize {
        self.transmit_buffer.len()
    }
}

#[cfg(feature = "alloc")]
impl<'a> Default for Storage<'a> {
    fn default() -> Self {
        use alloc::{boxed::Box, vec, vec::Vec};

        const RECEIVE_BUFFERS: usize = 64;
        const BUFFER_SIZE: usize = 1600;

        let mut receive_descriptors = Vec::with_capacity(RECEIVE_BUFFERS);
        receive_descriptors.resize_with(RECEIVE_BUFFERS, Default::default);
        let receive_buffers = vec![vec![0u8; BUFFER_SIZE]; RECEIVE_BUFFERS];

        Self {
            receive_descriptors: receive_descriptors.into(),
            receive_buffers: receive_buffers
                .into_iter()
                .map(ManagedSlice::Owned)
                .collect::<Vec<_>>()
                .into(),
            transmit_descriptor: Box::new(TransmitDescriptor::default()).into(),
            transmit_buffer: vec![0u8; BUFFER_SIZE].into(),
        }
    }
}

pub struct Config<'a> {
    pub mac_address: u64,
    pub storage: Storage<'a>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigurationError {
    NoPhyDevicePresent,
    BadStorage(&'static str),
}

impl<'a> Storage<'a> {
    fn initialize_descriptors(&mut self) -> Result<(), &'static str> {
        if self.receive_descriptors.is_empty() {
            return Err("no receive descriptors were given");
        } else if self.receive_descriptors.len() != self.receive_buffers.len() {
            return Err("the number of receive descriptors and receive buffers must be the same");
        }

        let n = self.receive_buffers.len();

        let receive_buffer_len = self.receive_buffers[0].len();
        if receive_buffer_len == 0 && receive_buffer_len % 64 != 0 {
            return Err("receive buffers must be a multiple of 64 bytes");
        }

        for (i, desc) in self.receive_descriptors.iter_mut().enumerate() {
            let addr = self.receive_buffers[i].as_ptr() as u64;
            if addr % 8 != 0 {
                return Err("receive buffers must be 64-bit aligned");
            }
            let word_addr = addr / 8;
            desc.word0.set((word_addr << 3) as u32);
            desc.word1.set(0);
            desc.word2.set((word_addr >> 29) as u32);
            desc.unused = 0;
            if i == n - 1 {
                desc.word0.modify(ReceiveDescriptorWord0::WRAP::SET);
            }
        }

        {
            let addr = self.transmit_buffer.as_ptr() as u64;
            self.transmit_descriptor.word0.set(addr as u32);
            self.transmit_descriptor.word1.set(0xc0000000);
            self.transmit_descriptor.word2.set((addr >> 32) as u32);
            self.transmit_descriptor.unused = 0;
        }

        Ok(())
    }
}

impl Controller {
    /// Initiatizes and returns the GEM 0 controller.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn gem0() -> Self {
        Self::new(&mut *GEM0)
    }

    /// Initiatizes and returns the GEM 1 controller.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn gem1() -> Self {
        Self::new(&mut *GEM1)
    }

    /// Initiatizes and returns the GEM 2 controller.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn gem2() -> Self {
        Self::new(&mut *GEM2)
    }

    /// Initiatizes and returns the GEM 3 controller.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn gem3() -> Self {
        Self::new(&mut *GEM3)
    }

    /// Creates a new UART controller.
    pub fn new(registers: &'static mut Registers) -> Self {
        Self { registers }
    }

    /// Provides raw access to the registers.
    ///
    /// # Safety
    /// Refer to the module's reference material to understand what is and isn't safe.
    pub unsafe fn registers(&mut self) -> &Registers {
        self.registers
    }

    /// Provides access to the PHY management interface.
    pub fn phy_management(&mut self) -> PhyManagement {
        let is_enabled = self
            .registers
            .network_control
            .is_set(NetworkControlR::MAN_PORT_EN);
        if !is_enabled {
            self.registers.network_control.set(
                NetworkControlW::MAN_PORT_EN::SET.modify(self.registers.network_control.get()),
            );
        }
        PhyManagement {
            disable_on_drop: !is_enabled,
            registers: self.registers,
        }
    }

    /// Configures and enables the controller.
    pub fn configure(
        mut self,
        mut config: Config,
    ) -> Result<ConfiguredController, ConfigurationError> {
        // reset / disable everything
        self.registers.network_control.set(0);
        self.registers
            .network_control
            .write(NetworkControlW::CLEAR_ALL_STATS_REGS::SET);
        self.registers.receive_status.set(0x0f);
        self.registers.transmit_status.set(0x0f);
        self.registers.int_disable.set(0x7fffeff);

        // set up the queues
        config
            .storage
            .initialize_descriptors()
            .map_err(ConfigurationError::BadStorage)?;
        let rx_queue_addr = config.storage.receive_descriptors.as_ptr() as u64;
        self.registers.receive_q_ptr.set(rx_queue_addr as u32);
        self.registers.receive_q1_ptr.set(rx_queue_addr as u32);
        self.registers
            .upper_rx_q_base_addr
            .set((rx_queue_addr >> 32) as u32);
        let tx_queue_addr =
            &mut *config.storage.transmit_descriptor as *mut TransmitDescriptor as u64;
        self.registers.transmit_q_ptr.set(tx_queue_addr as u32);
        self.registers.transmit_q1_ptr.set(tx_queue_addr as u32);
        self.registers
            .upper_tx_q_base_addr
            .set((tx_queue_addr >> 32) as u32);

        // configure networking
        self.registers.network_config.modify(
            NetworkConfig::FULL_DUPLEX::SET
                + NetworkConfig::GIGABIT_MODE_ENABLE::SET
                + NetworkConfig::NO_BROADCAST::CLEAR
                + NetworkConfig::MULTICAST_HASH_ENABLE::SET
                + NetworkConfig::RECEIVE_CHECKSUM_OFFLOAD_ENABLE::SET
                + NetworkConfig::PAUSE_ENABLE::SET
                + NetworkConfig::COPY_ALL_FRAMES::SET
                + NetworkConfig::RECEIVE_1536_BYTE_FRAMES::SET
                + NetworkConfig::SPEED::CLEAR
                + NetworkConfig::MDC_CLOCK_DIVISION.val(6),
        );

        // set the mac address
        self.registers
            .spec_add1_bottom
            .set(config.mac_address as u32);
        self.registers
            .spec_add1_top
            .set((config.mac_address >> 32) as u32);

        // configure dma
        self.registers.dma_config.write(
            DmaConfigW::RX_BUF_SIZE.val(config.storage.receive_buffer_len() as u32 / 64)
                + DmaConfigW::RX_PBUF_SIZE.val(3)
                + DmaConfigW::TX_PBUF_SIZE::SET
                + DmaConfigW::TX_PBUF_TCP_EN::SET
                + DmaConfigW::RX_BD_EXTENDED_MODE_EN::SET
                + DmaConfigW::TX_BD_EXTENDED_MODE_EN::SET
                + DmaConfigW::ENDIAN_SWAP_PACKET::CLEAR
                + DmaConfigW::AMBA_BURST_LENGTH.val(16),
        );

        // enable management
        self.registers
            .network_control
            .set(NetworkControlW::MAN_PORT_EN::SET.modify(self.registers.network_control.get()));

        let phy_device_id = match self.phy_management().device_ids().last() {
            Some(id) => id,
            None => return Err(ConfigurationError::NoPhyDevicePresent),
        };

        // TODO: configure the phy?

        // TODO: configure interrupts?

        // enable rx/tx
        self.registers.network_control.set(
            (NetworkControlW::ENABLE_TRANSMIT::SET + NetworkControlW::ENABLE_RECEIVE::SET)
                .modify(self.registers.network_control.get()),
        );

        Ok(ConfiguredController {
            link_status: None,
            phy_device_id,
            config,
            registers: self.registers,
        })
    }
}

pub struct ConfiguredController<'a> {
    link_status: Option<LinkStatus>,
    phy_device_id: DeviceId,
    config: Config<'a>,
    registers: &'static Registers,
}

pub struct ReceiveBuffer<'a> {
    descriptor: &'a mut ReceiveDescriptor,
    buffer: &'a mut [u8],
}

impl<'a> ReceiveBuffer<'a> {
    pub fn data(&mut self) -> &mut [u8] {
        let frame_length = self.descriptor.word1.read(ReceiveDescriptorWord1::LENGTH) as usize;
        &mut self.buffer[..frame_length]
    }

    pub fn consume(&mut self) {
        let desc = &self.descriptor;
        desc.word1.set(0);
        desc.word0.modify(ReceiveDescriptorWord0::OWNERSHIP::CLEAR);
    }
}

pub struct TransmitBuffer<'a> {
    registers: &'static Registers,
    descriptor: &'a mut TransmitDescriptor,
    buffer: &'a mut [u8],
}

impl<'a> TransmitBuffer<'a> {
    pub fn data(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    pub fn transmit(mut self, len: usize) {
        let len = self.data().len().min(len);
        self.descriptor.word1.write(
            TransmitDescriptorWord1::USED::CLEAR
                + TransmitDescriptorWord1::WRAP::SET
                + TransmitDescriptorWord1::LAST_BUFFER::SET
                + TransmitDescriptorWord1::LENGTH_OF_BUFFER.val(len as _),
        );
        self.registers.network_control.set(
            (NetworkControlW::TX_START_PCLK::SET).modify(self.registers.network_control.get()),
        );
        // TODO: transmit multiple frames at a time?
        loop {
            if self
                .registers
                .transmit_status
                .is_set(TransmitStatusR::TRANSMIT_COMPLETE)
            {
                self.registers
                    .transmit_status
                    .write(TransmitStatusW::TRANSMIT_COMPLETE::SET);
                return;
            }
            // TODO: wfi while we wait?
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LinkStatus {
    pub full_duplex: bool,
    pub speed: u32,
}

impl<'a> ConfiguredController<'a> {
    pub fn config(&self) -> &Config<'a> {
        &self.config
    }

    pub fn link_status(&self) -> Option<LinkStatus> {
        self.link_status
    }

    /// This function must be called regularly so the controller can respond to any changes in link
    /// status or auto-negotiation.
    pub fn poll_link_status(&mut self) {
        let phy_address = self.phy_device_id.address;
        let status = unsafe { self.phy_management().clause_22_read(phy_address, 0x11) };
        if (status & (1 << 10)) == 0 {
            self.link_status = None;
        } else {
            let new_link_status = LinkStatus {
                full_duplex: (status & (1 << 13)) != 0,
                speed: match (status >> 14) & 0x3 {
                    0 => 10,
                    1 => 100,
                    _ => 1000,
                },
            };
            if self.link_status != Some(new_link_status) {
                self.registers.network_config.modify(
                    NetworkConfig::FULL_DUPLEX.val(if new_link_status.full_duplex { 1 } else { 0 })
                        + NetworkConfig::GIGABIT_MODE_ENABLE
                            .val(if new_link_status.speed == 1000 { 1 } else { 0 })
                        + NetworkConfig::SPEED.val(if new_link_status.speed == 100 {
                            1
                        } else {
                            0
                        }),
                );
            }
            self.link_status = Some(new_link_status);
        }
    }

    /// Provides access to the PHY management interface.
    pub fn phy_management(&mut self) -> PhyManagement {
        PhyManagement {
            disable_on_drop: false,
            registers: self.registers,
        }
    }

    pub fn get_receive_buffer(&mut self) -> Option<ReceiveBuffer<'_>> {
        self.get_receive_and_transmit_buffers().map(|(rx, _)| rx)
    }

    pub fn get_receive_and_transmit_buffers(
        &mut self,
    ) -> Option<(ReceiveBuffer<'_>, TransmitBuffer<'_>)> {
        for (i, desc) in self.config.storage.receive_descriptors.iter().enumerate() {
            if desc.word0.is_set(ReceiveDescriptorWord0::OWNERSHIP) {
                if !desc.word1.matches_all(
                    ReceiveDescriptorWord1::START_OF_FRAME::SET
                        + ReceiveDescriptorWord1::END_OF_FRAME::SET,
                ) {
                    // TODO: should we reassemble frames that don't fit in one buffer?
                    desc.word1.set(0);
                    desc.word0.modify(ReceiveDescriptorWord0::OWNERSHIP::CLEAR);
                    continue;
                }
                return Some((
                    ReceiveBuffer {
                        descriptor: &mut self.config.storage.receive_descriptors[i],
                        buffer: &mut self.config.storage.receive_buffers[i],
                    },
                    TransmitBuffer {
                        registers: self.registers,
                        descriptor: &mut self.config.storage.transmit_descriptor,
                        buffer: &mut self.config.storage.transmit_buffer,
                    },
                ));
            }
        }
        None
    }

    pub fn get_transmit_buffer(&mut self) -> Option<TransmitBuffer<'_>> {
        Some(TransmitBuffer {
            registers: self.registers,
            descriptor: &mut self.config.storage.transmit_descriptor,
            buffer: &mut self.config.storage.transmit_buffer,
        })
    }
}

impl<'a> Drop for ConfiguredController<'a> {
    fn drop(&mut self) {
        self.registers.network_control.set(0);
        self.registers
            .network_control
            .write(NetworkControlW::CLEAR_ALL_STATS_REGS::SET);
        self.registers.receive_status.set(0x0f);
        self.registers.transmit_status.set(0x0f);
        self.registers.int_disable.set(0x7fffeff);
        self.registers.receive_q_ptr.set(0);
        self.registers.receive_q1_ptr.set(0);
        self.registers.transmit_q_ptr.set(0);
        self.registers.transmit_q1_ptr.set(0);
    }
}

#[repr(C)]
pub struct ReceiveDescriptor {
    word0: InMemoryRegister<u32, ReceiveDescriptorWord0::Register>,
    word1: InMemoryRegister<u32, ReceiveDescriptorWord1::Register>,
    word2: InMemoryRegister<u32, ReceiveDescriptorWord2::Register>,
    unused: u32,
}

impl Default for ReceiveDescriptor {
    fn default() -> Self {
        Self {
            word0: InMemoryRegister::new(0),
            word1: InMemoryRegister::new(0),
            word2: InMemoryRegister::new(0),
            unused: 0,
        }
    }
}

tock_registers::register_bitfields! [
    u32,
    pub ReceiveDescriptorWord0 [
        ADDRESS_LOW OFFSET(3) NUMBITS(28) [],
        WRAP OFFSET(1) NUMBITS(1) [],
        OWNERSHIP OFFSET(0) NUMBITS(1) [],
    ],
    pub ReceiveDescriptorWord1 [
        END_OF_FRAME OFFSET(15) NUMBITS(1) [],
        START_OF_FRAME OFFSET(14) NUMBITS(1) [],
        LENGTH OFFSET(0) NUMBITS(13) [],
    ],
    pub ReceiveDescriptorWord2 [
        ADDRESS_HIGH OFFSET(0) NUMBITS(32) [],
    ]
];

#[repr(C)]
pub struct TransmitDescriptor {
    word0: InMemoryRegister<u32, TransmitDescriptorWord0::Register>,
    word1: InMemoryRegister<u32, TransmitDescriptorWord1::Register>,
    word2: InMemoryRegister<u32, TransmitDescriptorWord2::Register>,
    unused: u32,
}

impl Default for TransmitDescriptor {
    fn default() -> Self {
        Self {
            word0: InMemoryRegister::new(0),
            word1: InMemoryRegister::new(0),
            word2: InMemoryRegister::new(0),
            unused: 0,
        }
    }
}

tock_registers::register_bitfields! [
    u32,
    pub TransmitDescriptorWord0 [
        ADDRESS_LOW OFFSET(0) NUMBITS(32) [],
    ],
    pub TransmitDescriptorWord1 [
        USED OFFSET(31) NUMBITS(1) [],
        WRAP OFFSET(30) NUMBITS(1) [],
        LAST_BUFFER OFFSET(15) NUMBITS(1) [],
        LENGTH_OF_BUFFER OFFSET(0) NUMBITS(14) [],
    ],
    pub TransmitDescriptorWord2 [
        ADDRESS_HIGH OFFSET(0) NUMBITS(32) [],
    ]
];

pub struct PhyManagement<'a> {
    disable_on_drop: bool,
    registers: &'a Registers,
}

impl<'a> Drop for PhyManagement<'a> {
    fn drop(&mut self) {
        if self.disable_on_drop {
            self.registers.network_control.set(
                NetworkControlW::MAN_PORT_EN::CLEAR.modify(self.registers.network_control.get()),
            );
        }
    }
}

impl<'a> PhyManagement<'a> {
    pub fn is_idle(&mut self) -> bool {
        self.registers
            .network_status
            .is_set(NetworkStatus::MAN_DONE)
    }

    pub fn wait_until_idle(&mut self) {
        while !self.is_idle() {}
    }

    /// Enumerates the devices attached to the controller.
    pub fn device_ids(&mut self) -> DeviceIds<'_, 'a> {
        DeviceIds {
            phy_management: self,
            next_address: 0,
        }
    }

    pub unsafe fn clause_22_read(&mut self, phy_address: u8, register_address: u8) -> u16 {
        self.wait_until_idle();
        self.registers.phy_management.write(
            gem::PhyManagement::WRITE1::SET
                + gem::PhyManagement::OPERATION.val(2)
                + gem::PhyManagement::PHY_ADDRESS.val(phy_address as _)
                + gem::PhyManagement::REGISTER_ADDRESS.val(register_address as _)
                + gem::PhyManagement::WRITE10.val(2),
        );
        self.wait_until_idle();
        self.registers
            .phy_management
            .read(gem::PhyManagement::PHY_WRITE_READ_DATA) as _
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceId {
    pub address: u8,
    pub oui: u32,
    pub model_number: u8,
    pub revision_number: u8,
}

impl DeviceId {
    fn new(address: u8, reg1: u16, reg2: u16) -> Self {
        DeviceId {
            address,
            oui: (reg1 as u32) << 6 | (reg2 & 0xFC00) as u32 >> 10,
            model_number: ((reg2 & 0x3F0) >> 4) as u8,
            revision_number: (reg2 & 0xF) as u8,
        }
    }
}

pub struct DeviceIds<'a, 'b> {
    next_address: u8,
    phy_management: &'a mut PhyManagement<'b>,
}

impl<'a, 'b> Iterator for DeviceIds<'a, 'b> {
    type Item = DeviceId;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next_address >= 32 {
                return None;
            }
            let address = self.next_address;
            self.next_address += 1;
            unsafe {
                let reg1 = self.phy_management.clause_22_read(address, 2);
                let reg2 = self.phy_management.clause_22_read(address, 3);
                if reg1 == 0xffff && reg2 == 0xffff {
                    continue;
                }
                return Some(DeviceId::new(address, reg1, reg2));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::is_qemu;
    use alloc::vec::Vec;

    #[test]
    fn test_phy_management() {
        let mut controller = unsafe { Controller::gem3() };

        let device_ids: Vec<_> = controller.phy_management().device_ids().collect();
        if is_qemu() {
            assert_eq!(
                device_ids,
                vec![
                    DeviceId {
                        address: 0,
                        oui: 0x50_43,
                        model_number: 33,
                        revision_number: 0
                    },
                    DeviceId {
                        address: 1,
                        oui: 0x50_43,
                        model_number: 33,
                        revision_number: 0
                    },
                ]
            );
        } else {
            assert_eq!(
                device_ids,
                vec![DeviceId {
                    address: 5,
                    oui: 0x08_00_28,
                    model_number: 35,
                    revision_number: 1
                }],
            );
        }
    }
}

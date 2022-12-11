use alloc::vec::Vec;
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    registers::InMemoryRegister,
};
use zynq_ultrascale_plus_modules::gem::{self, *};

pub struct Controller {
    registers: &'static Registers,
}

pub struct Config {
    pub mac_address: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigurationError {
    NoPhyDevicePresent,
}

pub const BUFFER_SIZE: usize = 1600;

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

    /// Provides access to the PHY management interface.
    pub fn phy_management(&mut self) -> PhyManagement {
        PhyManagement { controller: self }
    }

    /// Provides raw access to the registers.
    ///
    /// # Safety
    /// Refer to the module's reference material to understand what is and isn't safe.
    pub unsafe fn registers(&mut self) -> &Registers {
        self.registers
    }

    /// Configures and enables the controller.
    pub fn configure(self, config: Config) -> Result<ConfiguredController, ConfigurationError> {
        // reset / disable everything
        self.registers.network_control.set(0);
        self.registers
            .network_control
            .write(NetworkControlW::CLEAR_ALL_STATS_REGS::SET);
        self.registers.receive_status.set(0x0f);
        self.registers.transmit_status.set(0x0f);
        self.registers.int_disable.set(0x7fffeff);

        // set up the queues
        let receive_queue = ReceiveQueue::new(64, BUFFER_SIZE);
        let transmit_queue = TransmitQueue::new(1, BUFFER_SIZE);
        self.registers
            .receive_q_ptr
            .set(receive_queue.descriptors.as_ptr() as u32);
        self.registers
            .receive_q1_ptr
            .set(receive_queue.descriptors.as_ptr() as u32);
        self.registers
            .transmit_q_ptr
            .set(transmit_queue.descriptors.as_ptr() as u32);
        self.registers
            .transmit_q1_ptr
            .set(transmit_queue.descriptors.as_ptr() as u32);

        // configure networking
        self.registers.network_config.write(
            NetworkConfig::FULL_DUPLEX::SET
                + NetworkConfig::GIGABIT_MODE_ENABLE::SET
                + NetworkConfig::NO_BROADCAST::CLEAR
                + NetworkConfig::MULTICAST_HASH_ENABLE::SET
                + NetworkConfig::RECEIVE_CHECKSUM_OFFLOAD_ENABLE::SET
                + NetworkConfig::PAUSE_ENABLE::SET,
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
            DmaConfigW::RX_BUF_SIZE.val(BUFFER_SIZE as u32 / 64)
                + DmaConfigW::RX_PBUF_SIZE.val(3)
                + DmaConfigW::TX_PBUF_SIZE::SET
                + DmaConfigW::TX_PBUF_TCP_EN::SET
                + DmaConfigW::RX_BD_EXTENDED_MODE_EN::SET
                + DmaConfigW::TX_BD_EXTENDED_MODE_EN::SET
                + DmaConfigW::ENDIAN_SWAP_PACKET::CLEAR
                + DmaConfigW::AMBA_BURST_LENGTH.val(16),
        );

        // program the network control register
        self.registers
            .network_control
            .set(NetworkControlW::MAN_PORT_EN::SET.modify(self.registers.network_control.get()));

        // TODO: configure the phy and negotiate link speed?

        // TODO: configure interrupts?

        // enable rx/tx
        self.registers.network_control.set(
            (NetworkControlW::ENABLE_TRANSMIT::SET + NetworkControlW::ENABLE_RECEIVE::SET)
                .modify(self.registers.network_control.get()),
        );

        Ok(ConfiguredController {
            config,
            registers: self.registers,
            receive_queue,
            transmit_queue,
        })
    }
}

pub struct ConfiguredController {
    config: Config,
    registers: &'static Registers,
    receive_queue: ReceiveQueue,
    transmit_queue: TransmitQueue,
}

pub struct ReceiveBuffer<'a> {
    queue: &'a mut ReceiveQueue,
    index: usize,
}

impl<'a> ReceiveBuffer<'a> {
    pub fn data(&mut self) -> &mut [u8] {
        let desc = &self.queue.descriptors[self.index];
        let frame_length = desc.word1.read(ReceiveDescriptorWord1::LENGTH) as usize;
        &mut self.queue.buffers[self.index][..frame_length]
    }

    pub fn consume(&mut self) {
        let desc = &self.queue.descriptors[self.index];
        desc.word1.set(0);
        desc.word0.modify(ReceiveDescriptorWord0::OWNERSHIP::CLEAR);
    }
}

pub struct TransmitBuffer<'a> {
    registers: &'static Registers,
    queue: &'a mut TransmitQueue,
    index: usize,
}

impl<'a> TransmitBuffer<'a> {
    pub fn data(&mut self) -> &mut [u8] {
        &mut self.queue.buffers[self.index]
    }

    pub fn transmit(mut self, len: usize) {
        let len = self.data().len().min(len);
        let wrap = if self.index == self.queue.descriptors.len() - 1 {
            1
        } else {
            0
        };
        self.queue.descriptors[self.index].word1.write(
            TransmitDescriptorWord1::USED::CLEAR
                + TransmitDescriptorWord1::WRAP.val(wrap)
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

impl ConfiguredController {
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn get_receive_buffer(&mut self) -> Option<ReceiveBuffer> {
        self.get_receive_and_transmit_buffers().map(|(rx, _)| rx)
    }

    pub fn get_receive_and_transmit_buffers(&mut self) -> Option<(ReceiveBuffer, TransmitBuffer)> {
        for (i, desc) in self.receive_queue.descriptors.iter().enumerate() {
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
                let tx_buffer_index = self.get_transmit_buffer_index()?;
                return Some((
                    ReceiveBuffer {
                        queue: &mut self.receive_queue,
                        index: i,
                    },
                    TransmitBuffer {
                        registers: self.registers,
                        queue: &mut self.transmit_queue,
                        index: tx_buffer_index,
                    },
                ));
            }
        }
        None
    }

    pub fn get_transmit_buffer(&mut self) -> Option<TransmitBuffer> {
        let index = self.get_transmit_buffer_index()?;
        Some(TransmitBuffer {
            registers: self.registers,
            queue: &mut self.transmit_queue,
            index,
        })
    }

    fn get_transmit_buffer_index(&mut self) -> Option<usize> {
        for (i, desc) in self.transmit_queue.descriptors.iter().enumerate() {
            if desc.word1.is_set(TransmitDescriptorWord1::USED) {
                return Some(i);
            }
        }
        None
    }
}

impl Drop for ConfiguredController {
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
struct ReceiveDescriptor {
    word0: InMemoryRegister<u32, ReceiveDescriptorWord0::Register>,
    word1: InMemoryRegister<u32, ReceiveDescriptorWord1::Register>,
    word2: InMemoryRegister<u32, ReceiveDescriptorWord2::Register>,
    unused: u32,
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

struct ReceiveQueue {
    buffers: Vec<Vec<u8>>,
    descriptors: Vec<ReceiveDescriptor>,
}

impl ReceiveQueue {
    fn new(n: usize, buffer_size: usize) -> Self {
        let mut descriptors = Vec::with_capacity(n);
        let mut buffers = Vec::with_capacity(n);
        for i in 0..n {
            let mut buf = Vec::with_capacity(buffer_size);
            buf.resize(buffer_size, 0u8);
            let word_addr = buf.as_ptr() as u64 / 8;
            let desc = ReceiveDescriptor {
                word0: InMemoryRegister::new((word_addr << 3) as u32),
                word1: InMemoryRegister::new(0),
                word2: InMemoryRegister::new((word_addr >> 29) as u32),
                unused: 0,
            };
            if i == n - 1 {
                desc.word0.modify(ReceiveDescriptorWord0::WRAP::SET);
            }
            descriptors.push(desc);
            buffers.push(buf);
        }
        Self {
            buffers,
            descriptors,
        }
    }
}

#[repr(C)]
struct TransmitDescriptor {
    word0: InMemoryRegister<u32, TransmitDescriptorWord0::Register>,
    word1: InMemoryRegister<u32, TransmitDescriptorWord1::Register>,
    word2: InMemoryRegister<u32, TransmitDescriptorWord2::Register>,
    unused: u32,
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

struct TransmitQueue {
    buffers: Vec<Vec<u8>>,
    descriptors: Vec<TransmitDescriptor>,
}

impl TransmitQueue {
    fn new(n: usize, buffer_size: usize) -> Self {
        let mut descriptors = Vec::with_capacity(n);
        let mut buffers = Vec::with_capacity(n);
        for i in 0..n {
            let mut buf = Vec::with_capacity(buffer_size);
            buf.resize(buffer_size, 0u8);
            let addr = buf.as_ptr() as u64;
            let desc = TransmitDescriptor {
                word0: InMemoryRegister::new(addr as u32),
                word1: InMemoryRegister::new(0x80000000),
                word2: InMemoryRegister::new((addr >> 32) as u32),
                unused: 0,
            };
            if i == n - 1 {
                desc.word1.modify(TransmitDescriptorWord1::WRAP::SET);
            }
            descriptors.push(desc);
            buffers.push(buf);
        }
        Self {
            buffers,
            descriptors,
        }
    }
}

pub struct PhyManagement<'a> {
    controller: &'a mut Controller,
}

impl<'a> PhyManagement<'a> {
    pub fn is_idle(&mut self) -> bool {
        self.controller
            .registers
            .network_status
            .is_set(NetworkStatus::MAN_DONE)
    }

    pub fn wait_until_idle(&mut self) {
        while !self.is_idle() {}
    }

    /// Enumerates the devices attached to the controller.
    pub fn device_ids(&mut self) -> DeviceIds {
        DeviceIds {
            controller: self.controller,
            next_address: 0,
        }
    }

    pub unsafe fn clause_22_read(&mut self, phy_address: u8, register_address: u8) -> u16 {
        self.wait_until_idle();
        self.controller.registers.phy_management.write(
            gem::PhyManagement::WRITE1::SET
                + gem::PhyManagement::OPERATION.val(2)
                + gem::PhyManagement::PHY_ADDRESS.val(phy_address as _)
                + gem::PhyManagement::REGISTER_ADDRESS.val(register_address as _)
                + gem::PhyManagement::WRITE10.val(2),
        );
        self.wait_until_idle();
        self.controller
            .registers
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
            oui: ((reg1 & 0xfc) as u32) << 10
                | ((reg1 & 0x3ff) as u32) << 8
                | (reg2 & 0xFC00) as u32 >> 10,
            model_number: ((reg2 & 0x3F0) >> 4) as u8,
            revision_number: (reg2 & 0xF) as u8,
        }
    }
}

pub struct DeviceIds<'a> {
    next_address: u8,
    controller: &'a mut Controller,
}

impl<'a> Iterator for DeviceIds<'a> {
    type Item = DeviceId;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.next_address >= 32 {
                return None;
            }
            let address = self.next_address;
            self.next_address += 1;
            unsafe {
                let reg1 = self.controller.phy_management().clause_22_read(address, 2);
                let reg2 = self.controller.phy_management().clause_22_read(address, 2);
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
    use alloc::vec::Vec;

    #[test]
    fn test_phy_management() {
        let mut controller = unsafe { Controller::gem0() };

        let device_ids: Vec<_> = controller.phy_management().device_ids().collect();
        assert_eq!(device_ids.len(), 1);
        let device_id = device_ids.into_iter().next().unwrap();
        assert_eq!(
            device_id,
            DeviceId {
                address: 0,
                oui: 82176,
                model_number: 20,
                revision_number: 1
            }
        );
    }

    #[test]
    fn test_configured_controller() {
        let controller = unsafe { Controller::gem0() };
        controller.configure(Config { mac_address: 0 }).unwrap();
    }
}

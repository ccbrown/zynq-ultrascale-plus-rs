use crate::gem;
use smoltcp::{
    phy::{self, Checksum, DeviceCapabilities, Medium},
    time::Instant,
    Result,
};

pub struct Phy {
    gem_controller: gem::ConfiguredController,
}

impl Phy {
    pub fn new(gem_controller: gem::ConfiguredController) -> Self {
        Self { gem_controller }
    }
}

impl<'a> phy::Device<'a> for Phy {
    type RxToken = PhyRxToken<'a>;
    type TxToken = PhyTxToken<'a>;

    fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        let (rx_buf, tx_buf) = self.gem_controller.get_receive_and_transmit_buffers()?;
        Some((PhyRxToken(rx_buf), PhyTxToken(tx_buf)))
    }

    fn transmit(&'a mut self) -> Option<Self::TxToken> {
        let tx_buf = self.gem_controller.get_transmit_buffer()?;
        Some(PhyTxToken(tx_buf))
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = gem::BUFFER_SIZE;
        caps.max_burst_size = Some(1);
        caps.medium = Medium::Ethernet;
        caps.checksum.ipv4 = Checksum::Both;
        caps.checksum.udp = Checksum::Both;
        caps.checksum.tcp = Checksum::Both;
        caps
    }
}

pub struct PhyRxToken<'a>(gem::ReceiveBuffer<'a>);

impl<'a> phy::RxToken for PhyRxToken<'a> {
    fn consume<R, F>(mut self, _timestamp: Instant, f: F) -> Result<R>
    where
        F: FnOnce(&mut [u8]) -> Result<R>,
    {
        let ret = f(self.0.data());
        self.0.consume();
        ret
    }
}

pub struct PhyTxToken<'a>(gem::TransmitBuffer<'a>);

impl<'a> phy::TxToken for PhyTxToken<'a> {
    fn consume<R, F>(mut self, _timestamp: Instant, len: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut [u8]) -> Result<R>,
    {
        let result = f(&mut self.0.data()[..len]);
        if result.is_ok() {
            self.0.transmit(len);
        }
        result
    }
}

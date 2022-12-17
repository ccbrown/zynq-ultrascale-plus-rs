use crate::gem;
use smoltcp::{
    phy::{self, Checksum, DeviceCapabilities, Medium},
    time::Instant,
    Result,
};

pub struct Phy<'p> {
    gem_controller: gem::ConfiguredController<'p>,
}

impl<'p> Phy<'p> {
    pub fn new(gem_controller: gem::ConfiguredController<'p>) -> Self {
        Self { gem_controller }
    }

    pub fn gem_controller(&mut self) -> &mut gem::ConfiguredController<'p> {
        &mut self.gem_controller
    }
}

impl<'d, 'p> phy::Device<'d> for Phy<'p> {
    type RxToken = PhyRxToken<'d>;
    type TxToken = PhyTxToken<'d>;

    fn receive(&'d mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        let (rx_buf, tx_buf) = self.gem_controller.get_receive_and_transmit_buffers()?;
        Some((PhyRxToken(rx_buf), PhyTxToken(tx_buf)))
    }

    fn transmit(&'d mut self) -> Option<Self::TxToken> {
        let tx_buf = self.gem_controller.get_transmit_buffer()?;
        Some(PhyTxToken(tx_buf))
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = self.gem_controller.config().storage.transmit_buffer_len();
        caps.max_burst_size = Some(1);
        caps.medium = Medium::Ethernet;
        caps.checksum.ipv4 = Checksum::Both;
        caps.checksum.udp = Checksum::Both;
        caps.checksum.tcp = Checksum::Both;
        caps
    }
}

pub struct PhyRxToken<'d>(gem::ReceiveBuffer<'d>);

impl<'d> phy::RxToken for PhyRxToken<'d> {
    fn consume<R, F>(mut self, _timestamp: Instant, f: F) -> Result<R>
    where
        F: FnOnce(&mut [u8]) -> Result<R>,
    {
        let ret = f(self.0.data());
        self.0.consume();
        ret
    }
}

pub struct PhyTxToken<'d>(gem::TransmitBuffer<'d>);

impl<'d> phy::TxToken for PhyTxToken<'d> {
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

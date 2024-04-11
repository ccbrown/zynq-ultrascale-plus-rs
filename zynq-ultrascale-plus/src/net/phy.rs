use crate::gem;
use smoltcp::{
    iface::{Config, Interface},
    phy::{self, Checksum, DeviceCapabilities, Medium},
    time::Instant,
    wire::EthernetAddress,
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

    pub fn interface<'a>(&mut self, now: Instant) -> Interface {
        let hardware_addr = self.gem_controller.config().mac_address;
        let hardware_addr = EthernetAddress([
            (hardware_addr >> 40) as u8,
            (hardware_addr >> 32) as u8,
            (hardware_addr >> 24) as u8,
            (hardware_addr >> 16) as u8,
            (hardware_addr >> 8) as u8,
            hardware_addr as u8,
        ]);
        let config = Config::new(hardware_addr.into());
        Interface::new(config, self, now)
    }
}

impl<'p> phy::Device for Phy<'p> {
    type RxToken<'d> = PhyRxToken<'d> where Self: 'd;
    type TxToken<'d> = PhyTxToken<'d> where Self: 'd;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let (rx_buf, tx_buf) = self.gem_controller.get_receive_and_transmit_buffers()?;
        Some((PhyRxToken(rx_buf), PhyTxToken(tx_buf)))
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
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
    fn consume<R, F>(mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let ret = f(self.0.data());
        self.0.consume();
        ret
    }
}

pub struct PhyTxToken<'d>(gem::TransmitBuffer<'d>);

impl<'d> phy::TxToken for PhyTxToken<'d> {
    fn consume<R, F>(mut self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let result = f(&mut self.0.data()[..len]);
        self.0.transmit(len);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoltcp::{iface::SocketSet, socket::dhcpv4};

    #[test]
    fn test_interface_dhcp() {
        let start = aarch64_std::time::Instant::now();

        let controller = unsafe { gem::Controller::gem3() };

        let mut controller = controller
            .configure(gem::Config {
                mac_address: 0x02_00_00_00_00_01,
                storage: Default::default(),
            })
            .unwrap();

        while controller.link_status().is_none() {
            controller.poll_link_status();
        }

        let mut device = Phy::new(controller);

        let timestamp = Instant::from_millis(start.elapsed().as_millis() as i64);
        let mut iface = device.interface(timestamp);

        let dhcp_socket = dhcpv4::Socket::new();

        let mut sockets = SocketSet::new(vec![]);
        let dhcp_handle = sockets.add(dhcp_socket);

        loop {
            let timestamp = Instant::from_millis(start.elapsed().as_millis() as i64);
            iface.poll(timestamp, &mut device, &mut sockets);

            match sockets.get_mut::<dhcpv4::Socket>(dhcp_handle).poll() {
                None => continue,
                Some(dhcpv4::Event::Configured(_config)) => {
                    // success!
                    return;
                }
                Some(dhcpv4::Event::Deconfigured) => {}
            }
        }
    }
}

use crate::{gem, net::phy::Phy};
use alloc::{collections::BTreeMap, vec};
use smoltcp::iface::{InterfaceBuilder, NeighborCache, Routes};
use smoltcp::wire::{EthernetAddress, IpCidr, Ipv4Address};

pub type Interface = smoltcp::iface::Interface<'static, Phy>;

/// Creates an ethernet interface from a configured GEM controller.
pub fn from_gem_controller(gem_controller: gem::ConfiguredController) -> Interface {
    let hardware_addr = gem_controller.config().mac_address;
    let phy = Phy::new(gem_controller);
    let ip_addrs = [IpCidr::new(Ipv4Address::UNSPECIFIED.into(), 0)];
    let neighbor_cache = NeighborCache::new(BTreeMap::new());
    let hardware_addr = EthernetAddress([
        (hardware_addr >> 40) as u8,
        (hardware_addr >> 32) as u8,
        (hardware_addr >> 24) as u8,
        (hardware_addr >> 16) as u8,
        (hardware_addr >> 8) as u8,
        hardware_addr as u8,
    ]);
    let routes = Routes::new(BTreeMap::new());
    InterfaceBuilder::new(phy, vec![])
        .ip_addrs(ip_addrs)
        .neighbor_cache(neighbor_cache)
        .hardware_addr(hardware_addr.into())
        .routes(routes)
        .finalize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use smoltcp::{
        socket::{Dhcpv4Event, Dhcpv4Socket},
        time::{Duration, Instant},
    };

    #[test]
    fn test_interface_dhcp() {
        let controller = unsafe { gem::Controller::gem3() };
        let controller = controller
            .configure(gem::Config {
                mac_address: 0x02_00_00_00_00_01,
            })
            .unwrap();
        let mut iface = from_gem_controller(controller);

        let mut dhcp_socket = Dhcpv4Socket::new();
        dhcp_socket.set_max_lease_duration(Some(Duration::from_secs(10)));

        let dhcp_handle = iface.add_socket(dhcp_socket);

        let start = aarch64_std::time::Instant::now();
        loop {
            let timestamp = Instant::from_millis(start.elapsed().as_millis() as i64);
            if let Err(e) = iface.poll(timestamp) {
                panic!("poll error: {}", e);
            }

            match iface.get_socket::<Dhcpv4Socket>(dhcp_handle).poll() {
                None => continue,
                Some(Dhcpv4Event::Configured(_config)) => {
                    // success!
                    return;
                }
                Some(Dhcpv4Event::Deconfigured) => {
                    panic!("dhcp deconfigured");
                }
            }
        }
    }
}

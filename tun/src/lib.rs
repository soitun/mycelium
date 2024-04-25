//! Implementation of platform specific Tun interfaces used by the [`mycelium`] crate.

use std::net::Ipv6Addr;

/// Builder for configuration of a tun interface.
pub struct ConfigBuilder {
    /// Name for the tun interface. Platform restrictions apply.
    name: Option<String>,
    /// IPv6 address to assign to the tun interface, as well as the
    ip6: Option<(Ipv6Addr, u8)>,
}

impl ConfigBuilder {
    /// Create a new `ConfigBuilder`.
    pub fn new() -> Self {
        ConfigBuilder {
            name: None,
            ip6: None,
        }
    }

    /// Set the name of the tun interface which will be created.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the ipv6 address which will be assigned to created tun interface, as well as the size
    /// of the mask.
    pub fn ip6(mut self, ip6: Ipv6Addr, mask_size: u8) -> Self {
        self.ip6 = Some((ip6, mask_size));
        self
    }
}

use std::net::Ipv4Addr;
use std::time::Duration;

#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::*;

#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
pub use unix::*;

pub struct MulticastOptions {
    /// The maximal timeout before [`MulticastSocket::receive`] returns.
    ///
    /// If this is `None`, [`MulticastSocket::receive`] will block until there is data to read.
    pub read_timeout: Option<Duration>,
    pub loopback: bool,
    pub buffer_size: usize,
    /// The address to bind the socket to.
    ///
    /// Usually this will be Ipv4Addr::UNSPECIFIED, in order to listen for packets on all
    /// interfaces.
    pub bind_address: Ipv4Addr,
    pub reuse_addr: bool,
}

impl Default for MulticastOptions {
    fn default() -> Self {
        MulticastOptions {
            read_timeout: Some(Duration::from_secs(1)),
            loopback: true,
            buffer_size: 512,
            bind_address: Ipv4Addr::UNSPECIFIED,
            reuse_addr: true,
        }
    }
}

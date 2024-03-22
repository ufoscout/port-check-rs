use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, TcpStream, ToSocketAddrs};
use std::ops::RangeBounds;
use std::time::Duration;
 

/// Represents a port for an IP address
pub enum Port {
    /// Represents a port for an IPv4 address
    Ipv4(u16),
    /// Represents a port for an IPv6 address
    Ipv6(u16),
}

impl From<u16> for Port {
    fn from(port: u16) -> Self {
        Port::Ipv4(port)
    }
}

impl Port {
    /// Creates a new IPv4 port with the specified value
    pub fn new(port: u16) -> Self {
        Port::Ipv4(port)
    }

    /// Creates a new IPv4 port with the specified value
    pub fn ipv4(port: u16) -> Self {
        Port::Ipv4(port)
    }

    /// Creates a new IPv6 port with the specified value
    pub fn ipv6(port: u16) -> Self {
        Port::Ipv6(port)
    }

}

/// Represents a port range for an IP address
pub enum Ports<R: RangeBounds<u16> + std::iter::Iterator<Item = u16>> {
    /// Represents a port range for an IPv4 address
    Ipv4(R),
    /// Represents a port range for an IPv6 address
    Ipv6(R),
}

impl <R: RangeBounds<u16> + std::iter::Iterator<Item = u16>> Ports<R> {

    /// Creates a new IPv4 port range with the specified min and max values
    pub fn new(port_range: R) -> Self {
        Self::ipv4(port_range)
    }

    /// Creates a new IPv4 port range with the specified min and max values
    pub fn ipv4(port_range: R) -> Self {
        Ports::Ipv4(port_range)
    }

    /// Creates a new Ipv6 port range with the specified min and max values
    pub fn ipv6(port_range: R) -> Self {
        Ports::Ipv6(port_range)
    }

}

impl <R: RangeBounds<u16> + std::iter::Iterator<Item = u16>> From<R> for Ports<R> {
    fn from(port_range: R) -> Self {
        Ports::Ipv4(port_range)
    }
}

/// Attempts a TCP connection to an address and returns whether it succeeded
pub fn is_port_reachable<A: ToSocketAddrs>(address: A) -> bool {
    TcpStream::connect(address).is_ok()
}

/// Attempts a TCP connection to an address and returns whether it succeeded
pub fn is_port_reachable_with_timeout<A: ToSocketAddrs>(address: A, timeout: Duration) -> bool {
    match address.to_socket_addrs() {
        Ok(addrs) => {
            for address in addrs {
                if TcpStream::connect_timeout(&address, timeout).is_ok() {
                    return true;
                }
            }
            false
        }
        Err(_err) => false,
    }
}

/// Returns whether a port is available on the localhost
/// If the IP version is not specified, it defaults to IPv4. This happens when the port is specified as a number.
pub fn is_local_port_free<P: Into<Port>>(port: P) -> bool {
    match port.into() {
        Port::Ipv4(port) => is_local_ipv4_port_free(port),
        Port::Ipv6(port) => is_local_ipv6_port_free(port),
    }
}

/// Returns whether a port is available on the localhost for IPv4
pub fn is_local_ipv4_port_free(port: u16) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    TcpListener::bind(ipv4).is_ok()
}

/// Returns whether a port is available on the localhost for IPv6
pub fn is_local_ipv6_port_free(port: u16) -> bool {
    let ipv6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, port, 0, 0);
    TcpListener::bind(ipv6).is_ok()
}

/// Returns an available localhost port within the specified range.
/// If the IP version is not specified, it defaults to IPv4. This happens when the port range is specified as a range.
pub fn free_local_port_in_range<P: Into<Ports<R>>, R: RangeBounds<u16> + std::iter::Iterator<Item = u16>>(port_range: P) -> Option<u16> {
    match port_range.into() {
        Ports::Ipv4(port_range) => free_local_ipv4_port_in_range(port_range),
        Ports::Ipv6(port_range) => free_local_ipv6_port_in_range(port_range),
    }
}

/// Returns an available localhost port within the specified range for IPv4.
pub fn free_local_ipv4_port_in_range<R: RangeBounds<u16> + std::iter::Iterator<Item = u16>>(port_range: R) -> Option<u16> {
    port_range.into_iter().find(|port| is_local_ipv4_port_free(*port))
}

/// Returns an available localhost port within the specified range for IPv6.
pub fn free_local_ipv6_port_in_range<R: RangeBounds<u16> + std::iter::Iterator<Item = u16>>(port_range: R) -> Option<u16> {
    port_range.into_iter().find(|port| is_local_ipv6_port_free(*port))
}

/// Returns an available localhost port for IPv4
pub fn free_local_port() -> Option<u16> {
    free_local_ipv4_port()
}

/// Returns an available localhost port for IPv4
pub fn free_local_ipv4_port() -> Option<u16> {
    let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
    TcpListener::bind(socket)
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}

/// Returns an available localhost port for IPv6
pub fn free_local_ipv6_port() -> Option<u16> {
    let socket = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 0, 0, 0);
    TcpListener::bind(socket)
        .and_then(|listener| listener.local_addr())
        .map(|addr| addr.port())
        .ok()
}

#[cfg(test)]
mod tests {

    use serial_test::serial;

    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener};
    use std::thread::JoinHandle;
    use std::time::Instant;
    use std::{thread, time::Duration};

    #[test]
    #[serial]
    fn should_return_an_unused_port() {
        let result = free_local_port();
        assert!(result.is_some());
        assert!(is_local_port_free(result.unwrap()));
        assert!(is_local_ipv4_port_free(result.unwrap()));
    }

    #[test]
    #[serial]
    fn should_return_an_unused_ipv4_port() {
        let result = free_local_ipv4_port();
        assert!(result.is_some());
        assert!(is_local_port_free(result.unwrap()));
        assert!(is_local_ipv4_port_free(result.unwrap()));
    }

    #[test]
    #[serial]
    fn should_return_an_unused_ipv6_port() {
        let result = free_local_ipv6_port();
        assert!(result.is_some());
        assert!(is_local_port_free(result.unwrap()));
        assert!(is_local_ipv6_port_free(result.unwrap()));
    }

    #[test]
    #[serial]
    fn an_open_port_with_ip_v4_should_not_be_free() {

        let ipv4_and_ipv6_free_port = find_free_ipv4_and_ipv6_port();
        
        // Start a TCP listener on the IPv4 port
        let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, ipv4_and_ipv6_free_port);
        let (port, _handle) = start_tcp_listner(socket);

        // The ipv4 port should not be free
        assert!(!is_local_port_free(port));
        assert!(!is_local_port_free(Port::ipv4(port)));
        assert!(!is_local_ipv4_port_free(port));
        assert!(is_local_ipv6_port_free(port));
        assert!(is_local_port_free(Port::ipv6(port)));

        // Start a TCP listener on the IPv6 port
        let socket = SocketAddrV6::new(Ipv6Addr::LOCALHOST, ipv4_and_ipv6_free_port, 0, 0);
        let (port, _handle) = start_tcp_listner(socket);

        // The ipv6 port should not be free
        assert!(!is_local_port_free(port));
        assert!(!is_local_ipv4_port_free(port));
        assert!(!is_local_port_free(Port::ipv4(port)));
        assert!(!is_local_ipv6_port_free(port));
        assert!(!is_local_port_free(Port::ipv6(port)));

    }

    #[test]
    #[serial]
    fn an_open_port_with_ip_v6_should_not_be_free() {
        let ipv4_and_ipv6_free_port = find_free_ipv4_and_ipv6_port();
        
        // Start a TCP listener on the IPv6 port
        let socket = SocketAddrV6::new(Ipv6Addr::LOCALHOST, ipv4_and_ipv6_free_port, 0, 0);
        let (port, _handle) = start_tcp_listner(socket);

        // The ipv6 port should not be free
        assert!(is_local_port_free(port));
        assert!(is_local_ipv4_port_free(port));
        assert!(is_local_port_free(Port::ipv4(port)));
        assert!(!is_local_ipv6_port_free(port));
        assert!(!is_local_port_free(Port::ipv6(port)));

                // Start a TCP listener on the IPv4 port
                let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, ipv4_and_ipv6_free_port);
                let (port, _handle) = start_tcp_listner(socket);
        
                // The ipv4 port should not be free
        assert!(!is_local_port_free(port));
        assert!(!is_local_ipv4_port_free(port));
        assert!(!is_local_port_free(Port::ipv4(port)));
        assert!(!is_local_ipv6_port_free(port));
        assert!(!is_local_port_free(Port::ipv6(port)));
    }

    #[test]
    #[serial]
    fn should_return_an_unused_port_in_range() {
        let free_port = free_local_port().unwrap();
        let min = free_port - 100;
        let max = free_port;
        let port_found = free_local_port_in_range(min..max).unwrap();
        assert!(port_found >= min);
        assert!(port_found <= max);
    }

    #[test]
    #[serial]
    fn should_return_an_unused_ipv4_port_in_range() {
        let free_port = free_local_ipv4_port().unwrap();
        let min = free_port - 100;
        let max = free_port;
        let port_found = free_local_ipv4_port_in_range(min..max).unwrap();
        assert!(port_found >= min);
        assert!(port_found <= max);
    }

    #[test]
    #[serial]
    fn should_return_an_unused_ipv6_port_in_range() {
        let free_port = free_local_ipv6_port().unwrap();
        let min = free_port - 100;
        let max = free_port;
        let port_found = free_local_ipv6_port_in_range(min..max).unwrap();
        assert!(port_found >= min);
        assert!(port_found <= max);
    }

    #[test]
    #[serial]
    fn ipv4_port_should_be_reachable() {
        let ipv4_and_ipv6_free_port = find_free_ipv4_and_ipv6_port();
        let address_v4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, ipv4_and_ipv6_free_port);
        let address_v6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, ipv4_and_ipv6_free_port, 0, 0);

        assert!(!is_port_reachable(address_v4));
        assert!(!is_port_reachable(address_v6));

        // Start a TCP listener on the IPv4 port
        let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, ipv4_and_ipv6_free_port);
        let (_port, _handle) = start_tcp_listner(socket);

        assert!(is_port_reachable(address_v4));
        assert!(free_local_port_in_range(Ports::ipv4(ipv4_and_ipv6_free_port..=ipv4_and_ipv6_free_port)).is_none());
        assert!(!is_port_reachable(address_v6));
        assert!(free_local_port_in_range(Ports::ipv6(ipv4_and_ipv6_free_port..=ipv4_and_ipv6_free_port)).is_some());
    }

    #[test]
    #[serial]
    fn ipv6_port_should_be_reachable() {
        let ipv4_and_ipv6_free_port = find_free_ipv4_and_ipv6_port();
        let address_v4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, ipv4_and_ipv6_free_port);
        let address_v6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, ipv4_and_ipv6_free_port, 0, 0);

        assert!(!is_port_reachable(address_v4));
        assert!(!is_port_reachable(address_v6));

        // Start a TCP listener on the IPv4 port
        let socket = SocketAddrV6::new(Ipv6Addr::LOCALHOST, ipv4_and_ipv6_free_port,0,0);
        let (_port, _handle) = start_tcp_listner(socket);
        
        assert!(!is_port_reachable(address_v4));
        assert!(free_local_port_in_range(Ports::ipv4(ipv4_and_ipv6_free_port..=ipv4_and_ipv6_free_port)).is_some());
        assert!(is_port_reachable(address_v6));
        assert!(free_local_port_in_range(Ports::ipv6(ipv4_and_ipv6_free_port..=ipv4_and_ipv6_free_port)).is_none());
    }

    #[test]
    #[serial]
    fn ipv4_port_should_be_reachable_with_timeout() {
        let ipv4_and_ipv6_free_port = find_free_ipv4_and_ipv6_port();
        let address_v4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, ipv4_and_ipv6_free_port);
        let address_v6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, ipv4_and_ipv6_free_port, 0, 0);

        assert!(!is_port_reachable_with_timeout(address_v4, Duration::from_secs(2)));
        assert!(!is_port_reachable_with_timeout(address_v6, Duration::from_secs(2)));

        // Start a TCP listener on the IPv4 port
        let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, ipv4_and_ipv6_free_port);
        let (_port, _handle) = start_tcp_listner(socket);
        
        assert!(is_port_reachable_with_timeout(address_v4, Duration::from_secs(2)));
        assert!(!is_port_reachable_with_timeout(address_v6, Duration::from_secs(2)));
    }

    #[test]
    #[serial]
    fn ipv6_port_should_be_reachable_with_timeout() {
        let ipv4_and_ipv6_free_port = find_free_ipv4_and_ipv6_port();
        let address_v4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, ipv4_and_ipv6_free_port);
        let address_v6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, ipv4_and_ipv6_free_port, 0, 0);

        assert!(!is_port_reachable_with_timeout(address_v4, Duration::from_secs(2)));
        assert!(!is_port_reachable_with_timeout(address_v6, Duration::from_secs(2)));

        // Start a TCP listener on the IPv6 port
        let socket = SocketAddrV6::new(Ipv6Addr::LOCALHOST, ipv4_and_ipv6_free_port,0,0);
        let (_port, _handle) = start_tcp_listner(socket);
        
        assert!(!is_port_reachable_with_timeout(address_v4, Duration::from_secs(2)));
        assert!(is_port_reachable_with_timeout(address_v6, Duration::from_secs(2)));
    }

    #[test]
    #[serial]
    fn free_port_should_resolve_domain_name() {
        let available_port = free_local_port().unwrap();
        assert!(!is_port_reachable(format!("localhost:{}", available_port)));
    }

    #[test]
    #[serial]
    fn is_port_reachable_should_respect_timeout() {
        let timeout = 100;
        let start = Instant::now();

        assert!(!is_port_reachable_with_timeout(
            "198.19.255.255:1",
            Duration::from_millis(timeout)
        ));

        let elapsed = start.elapsed().subsec_millis() as u64;
        println!("Millis elapsed {}", elapsed);
        assert!(elapsed >= timeout);
        assert!(elapsed < 2 * timeout);
    }

    #[test]
    #[serial]
    fn free_port_with_timeout_should_resolve_domain_name() {
        let available_port = free_local_port().unwrap();
        assert!(!is_port_reachable_with_timeout(
            format!("localhost:{}", available_port),
            Duration::from_millis(10)
        ));
    }

    #[test]
    #[serial]
    fn free_ipv4_port_with_timeout_should_resolve_domain_name() {
        let available_port = free_local_ipv4_port().unwrap();
        assert!(!is_port_reachable_with_timeout(
            SocketAddrV4::new(Ipv4Addr::LOCALHOST, available_port),
            Duration::from_millis(10)
        ));
    }

    #[test]
    #[serial]
    fn free_ipv6_port_with_timeout_should_resolve_domain_name() {
        let available_port = free_local_ipv6_port().unwrap();
        assert!(!is_port_reachable_with_timeout(
            SocketAddrV6::new(Ipv6Addr::LOCALHOST, available_port,0,0),
            Duration::from_millis(10)
        ));
    }

    fn start_tcp_listner<A: ToSocketAddrs>(address: A) -> (u16, JoinHandle<()>) {
        let listener = TcpListener::bind(&address).unwrap();
        let port = listener.local_addr().unwrap().port();

        let handle = thread::spawn(move || loop {
            match listener.accept() {
                Ok(_) => {
                    println!("TCP connection received!");
                }
                Err(e) => {
                    println!("TCP connection error: {e:?}");
                }
            }
        });
        wait_until_reachable(address);
        (port, handle)
    }

    // Find a port which is free for both IPv4 and IPv6
    fn find_free_ipv4_and_ipv6_port() -> u16 {
        let mut port = 1024;
        while !is_local_ipv4_port_free(port) || !is_local_ipv6_port_free(port) {
            port += 1;
        }
        assert!(is_local_ipv4_port_free(port));
        assert!(is_local_ipv6_port_free(port));
        port
    }

    // Wait until a port is reachable
    fn wait_until_reachable<A: ToSocketAddrs>(address: A) {
        let mut port_reachable = false;
        while !port_reachable {
            port_reachable = is_port_reachable(&address);
            thread::sleep(Duration::from_millis(10));
        }
    }

    // fn start_udp_listner<A: ToSocketAddrs>(address: A) -> (u16, JoinHandle<()>) {
    //     let listener = UdpSocket::bind(address).unwrap();
    //     let port = listener.local_addr().unwrap().port();

    //     let handle = thread::spawn(move || loop {
    //         match listener.recv_from(&mut [0u8]) {
    //             Ok(_) => {
    //                 println!("UDP connection received!");
    //             }
    //             Err(e) => {
    //                 println!("UDP connection error: {e:?}");
    //             }
    //         }
    //     });

    //     (port, handle)
    // }
}

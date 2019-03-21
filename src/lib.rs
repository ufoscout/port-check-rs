use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs};

/// Attempts a TCP connection to an address an returns whether it succeeded
pub fn is_port_reachable<A: ToSocketAddrs>(address: A) -> bool {
    TcpStream::connect(address).is_ok()
}

/// Returns whether a port is available on the localhost
pub fn is_local_port_free(port: u16) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    TcpListener::bind(ipv4).is_ok()
}

/// Returns an available localhost port within the specified range.
///
/// 'min' and 'max' values are included in the range
///
pub fn free_local_port_in_range(min: u16, max: u16) -> Option<u16> {
    (min..max).find(|port| is_local_port_free(*port))
}

/// Returns an available localhost port
pub fn free_local_port() -> Option<u16> {
    let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
    TcpListener::bind(socket)
        .and_then(|listener| {
            listener.local_addr()
        })
        .and_then(|addr| {
            Ok(addr.port())
        })
        .ok()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
    use std::{thread, time::Duration};

    #[test]
    fn should_return_an_unused_port() {
        let result = free_local_port();
        assert!(result.is_some());
        assert!(is_local_port_free(result.unwrap()));
    }

    #[test]
    fn should_return_an_unused_port_in_range() {
        let free_port = free_local_port().unwrap();
        let min = free_port - 100;
        let max = free_port;
        let port_found = free_local_port_in_range(min, max).unwrap();
        assert!(port_found >= min);
        assert!(port_found <= max);
    }

    #[test]
    fn a_free_port_should_not_be_reachable() {
        let available_port = free_local_port().unwrap();
        assert!(!is_port_reachable(&format!("127.0.0.1:{}", available_port)));
    }

    #[test]
    fn an_open_port_should_be_reachable() {
        let socket = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0);
        let listener = TcpListener::bind(socket).unwrap();
        let listener_port = listener.local_addr().unwrap().to_string();

        thread::spawn(move || loop {
            match listener.accept() {
                Ok(_) => {
                    println!("Connection received!");
                }
                Err(_) => {
                    println!("Error in received connection!");
                }
            }
        });

        let mut port_reachable = false;
        while !port_reachable {
            println!("Check for available connections on {}", &listener_port);
            port_reachable = is_port_reachable(&listener_port);
            thread::sleep(Duration::from_millis(10));
        }
        assert!(port_reachable)
    }
}

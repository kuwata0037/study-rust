#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn socket_addr_v4() {
        let socket = SocketAddr::from(([127, 0, 0, 1], 8080));

        assert!(socket.is_ipv4());
        assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(socket.port(), 8080);
    }

    #[test]
    fn socket_addr_v4_from_str() {
        let socket: SocketAddr = "127.0.0.1:8080".parse().unwrap();

        assert!(socket.is_ipv4());
        assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(socket.port(), 8080);
    }
}

use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use crate::common::constant::set_server_address;


pub fn get_local_ipv4() -> Option<Ipv4Addr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    if let Ok(local_addr) = socket.local_addr() {
        if let IpAddr::V4(ipv4) = local_addr.ip() {
            return Some(ipv4);
        }
    }
    None
}

pub fn create_server_socket(ip: Ipv4Addr, port: &str) -> Option<UdpSocket> {
    let address = format!("{}:{}", ip, port);
    match UdpSocket::bind(&address) {
        Ok(socket) => {
            set_server_address(address);
            Some(socket)
        }
        Err(err) => {
            eprintln!("Failed to bind socket: {}", err);
            None
        }
    }
}


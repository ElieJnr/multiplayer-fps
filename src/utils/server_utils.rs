use std::{net::{IpAddr, Ipv4Addr, UdpSocket}, sync::Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVER_ADDRESS: Mutex<Option<String>> = Mutex::new(None);
}

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
    match UdpSocket::bind(format!("{}:{}", ip.to_string(), port)) {
        Ok(socket) => {
            let address = format!("{}:{}", ip.to_string(), port);
            let mut server_address = SERVER_ADDRESS.lock().unwrap(); 
            *server_address = Some(address);
            Some(socket)
        },
        Err(err) => {
            eprintln!("Failed to bind socket: {}", err);
            None
        }
    }
}

pub fn get_server_address() -> Option<String> {
    let server_address = SERVER_ADDRESS.lock().unwrap();
    server_address.clone()
}

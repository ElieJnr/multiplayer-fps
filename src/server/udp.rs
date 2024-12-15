use std::{
    env,
    net::{IpAddr, Ipv4Addr, UdpSocket},
    collections::HashSet,
};

use crate::client::udp::client_udp;

fn get_local_ipv4() -> Option<Ipv4Addr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    if let Ok(local_addr) = socket.local_addr() {
        if let IpAddr::V4(ipv4) = local_addr.ip() {
            return Some(ipv4);
        }
    }
    None
}

fn create_server_socket(ip: Ipv4Addr) -> Option<UdpSocket> {
    match UdpSocket::bind(format!("{}:{}", ip.to_string(), "8080")) {
        Ok(socket) => Some(socket),
        Err(err) => {
            eprintln!("Failed to bind socket: {}", err);
            None
        }
    }
}

pub fn run_socket() {
    let args: Vec<String> = env::args().collect();
    let client_addresses: HashSet<std::net::SocketAddr> = HashSet::new();

    match args.len() {
        1 => match get_local_ipv4() {
            Some(ip) => {
                if let Some(socket) = create_server_socket(ip) {
                    println!("Server is running on {}:8080", ip);
                    server(socket, client_addresses);
                }
            }
            None => eprintln!("Unable to determine local IP address."),
        },
        2 => {
            let host = &args[1];
            if client_udp(host).is_none() {
                eprintln!("Failed to create client socket");
            }
        }
        _ => eprintln!("Usage: cargo run or cargo run <IP_LOCAL:PORT>"),
    }
}

pub fn server(server_socket: UdpSocket, mut client_addresses: HashSet<std::net::SocketAddr>) {
    let mut buf = [0; 1024];
    loop {
        let (size, src) = match server_socket.recv_from(&mut buf) {
            Ok((size, src)) => (size, src),
            Err(err) => {
                eprintln!("Failed to receive data: {}", err);
                continue;
            }
        };

        client_addresses.insert(src);

        let message = String::from_utf8_lossy(&buf[..size]);
        println!("Received from {}: {}", src, message);

        // Send acknowledgment to sender
        let response = format!("Server received: {}", message);
        if let Err(err) = server_socket.send_to(response.as_bytes(), src) {
            eprintln!("Failed to send data to {}: {}", src, err);
        }

        // Broadcast to all other clients
        for client in &client_addresses {
            if *client != src {
                if let Err(err) = server_socket.send_to(message.as_bytes(), client) {
                    eprintln!("Failed to send data to {}: {}", client, err);
                }
            }
        }
    }
}

use std::{
    env,
    net::{IpAddr, Ipv4Addr, UdpSocket},
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
    let socket = match UdpSocket::bind(format!("{}:{}", ip.to_string(), "8080")) {
        Ok(socket) => socket,
        Err(err) => {
            eprintln!("Failed to bind socket: {}", err);

            return None;
        }
    };
    Some(socket)
}

pub fn run_socket() {
    let args: Vec<String> = env::args().collect();
    // let mut server_socket: Option<UdpSocket> = None;
    let mut clients_socket: Vec<UdpSocket> = Vec::new();
    match args.len() {
        1 => match get_local_ipv4() {
            Some(ip) => {
                let server_socket = create_server_socket(ip);
                if server_socket.is_some() {
                    println!("Server is running on {}:8080", ip);
                }
                if let Some(socket) = server_socket {
                    server(socket, clients_socket);
                }
            }
            None => eprintln!("can't run the server"),
        },
        2 => {
            let host = &args[1];
            if let Some(client_socket) = client_udp(host) {
                clients_socket.push(client_socket);
            } else {
                eprintln!("Failed to create client socket");
            }
        }
        _ => eprintln!("Usage: cargo r or cargo r <IP_LOCAL:PORT>"),
    }
}

pub fn server(server_socket: UdpSocket, _client_sockets: Vec<UdpSocket>) {
    let mut buf = [0; 1024];
    loop {
        let (size, src) = match server_socket.recv_from(&mut buf) {
            Ok((size, src)) => (size, src),
            Err(err) => {
                eprintln!("Failed to receive data: {}", err);
                return;
            }
        };

        let message = String::from_utf8_lossy(&buf[..size]);
        println!("Received from {}: {}", src, message);

        let response = format!("Server received: {}", message);

        if let Err(err) = server_socket.send_to(response.as_bytes(), src) {
            eprintln!("Failed to send data: {}", err);
        }
    }
}

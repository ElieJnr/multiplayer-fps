use crate::client::udp::client_udp;
use crate::utils::model::Message;
use serde_json;
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
};

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
    let mut choice = String::new();
    let client_addresses = HashMap::new();
    println!("Your choice please: \n1-Server \n2-Client");

    let _ = stdout().flush();
    stdin()
        .read_line(&mut choice)
        .expect("not a correct number");

    let choice_int = match choice.trim().parse::<u32>() {
        Ok(choice_int) => choice_int,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    match choice_int {
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
            if client_udp().is_none() {
                eprintln!("Failed to create client socket");
            }
        }
        _ => eprintln!("make a choice between 1 and 2"),
    }
}

pub fn server(server_socket: UdpSocket, mut client_addresses: HashMap<String, SocketAddr>) {
    let mut buf = [0; 1024];
    loop {
        let (size, src) = match server_socket.recv_from(&mut buf) {
            Ok((size, src)) => (size, src),
            Err(err) => {
                eprintln!("Failed to receive data: {}", err);
                continue;
            }
        };

        //

        // let message = match String::from_utf8_lossy(&buf[..size]);
        let message: Message = match serde_json::from_slice(&buf[..size]) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Erreur lors de la désérialisation: {}", e);
                continue;
            }
        };

        if message.message_type == "newconnection" {
            client_addresses.insert(message.message_content.new_connexion.name.clone(), src);
            println!("{} est connecté",message.message_content.new_connexion.name);

            // Broadcast to all other clients
            for (_, client_src) in &client_addresses {
                if *client_src != src {
                    let msg = format!(
                        "{} est connecté",
                        &message.message_content.new_connexion.name
                    );
                    if let Err(err) = server_socket.send_to(msg.as_bytes(), client_src) {
                        eprintln!("Failed to send data to {}: {}", client_src, err);
                    }
                }
            }
        }
    }
}


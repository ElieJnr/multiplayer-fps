use std::{
    io::{stdin, stdout, Write},
    net::{IpAddr, Ipv4Addr, UdpSocket},
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
    let mut choice = String::new();

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
                let server_socket = create_server_socket(ip);
                if server_socket.is_some() {
                    println!("Server is running on {}:8080", ip);
                }
                if let Some(socket) = server_socket {
                    server(socket);
                }
            }
            None => eprintln!("can't run the server"),
        },
        2 => {}
        _ => eprintln!("make a choice between 1 and 2"),
    }
}

pub fn server(server_socket: UdpSocket) {
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

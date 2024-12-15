use std::net::UdpSocket;

pub fn client_udp(host_addr: &str) -> Option<()> {
    // Création d'une socket UDP
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(err) => {
            eprintln!("Failed to bind client socket: {}", err);
            return None;
        }
    };

    // Connexion au serveur
    if let Err(err) = socket.connect(host_addr) {
        eprintln!("Failed to connect to server {}: {}", host_addr, err);
        return None;
    }

    
    println!("Connected to server at {}", host_addr);


    // Boucle principale pour envoyer/recevoir des messages
    let mut buffer = [0; 1024];

    let  input = "je suis connecté";

        // Envoi du message au serveur
        if let Err(err) = socket.send(input.as_bytes()) {
            eprintln!("Failed to send message: {}", err);
        }


    loop {

        // let mut input = String::new();

        // // Envoi du message au serveur
        // if let Err(err) = socket.send(input.as_bytes()) {
        //     eprintln!("Failed to send message: {}", err);
        //     continue;
        // }

        // Réception de la réponse du serveur
        match socket.recv(&mut buffer) {
            Ok(size) => {
                let response = String::from_utf8_lossy(&buffer[..size]);
                println!("Server response: {}", response);
            }
            Err(err) => eprintln!("Failed to receive response: {}", err),
        }
    }

    
}

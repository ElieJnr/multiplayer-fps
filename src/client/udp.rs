use serde_json;
use std::{io::stdin, net::UdpSocket};
use std::io::{self,Write};
use crate::utils::model::{AllOption,Message,NewConnexion};

pub fn client_udp() -> Option<()> {
    let mut name = String::new();
    let mut ip = String::new();

    // Demander l'adresse IP et le pseudo
    print!("Enter The Server IP Address: ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut ip).expect("failed to read the IP");
    print!("Enter Your Name: ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut name).expect("failed to read the name");

    // Trim les espaces superflus
    ip = ip.trim().to_string();
    name = name.trim().to_string();

    // Création d'une socket UDP
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(err) => {
            eprintln!("Failed to bind client socket: {}", err);
            return None;
        }
    };

    // Connexion au serveur
    if let Err(err) = socket.connect(&ip) {
        eprintln!("Failed to connect to server {}: {}", ip, err);
        return None;
    }

    println!("Connected to server at {}", ip);

    // Créer une nouvelle connexion
    let new_connexion = NewConnexion { name };

    // Créer le message avec les données de la connexion
    let msg = Message {
        message_type: "newconnection".to_string(),
        message_content: AllOption { new_connexion },
    };

    // Sérialiser le message en JSON
    let msg_json = match serde_json::to_string(&msg) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to serialize message: {}", err);
            return None;
        }
    };

    // Convertir le JSON en bytes
    let msg_bytes = msg_json.as_bytes();

    // Envoi du message sérialisé au serveur
    if let Err(err) = socket.send(msg_bytes) {
        eprintln!("Failed to send message: {}", err);
        return None;
    }

    // Boucle principale pour recevoir la réponse du serveur
    let mut buffer = [0; 1024]; // tampon pour recevoir les messages

    loop {
        match socket.recv(&mut buffer) {
            Ok(size) => {
                let response = String::from_utf8_lossy(&buffer[..size]);
                println!("From server: {}", response);
            }
            Err(err) => eprintln!("Failed to receive response: {}", err),
        }
    }
}

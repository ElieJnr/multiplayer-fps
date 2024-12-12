use std::net::UdpSocket;

pub fn client_udp(host: &str) -> Option<UdpSocket> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    if socket.connect(host).is_err() {
        eprintln!("erreur lors de la connexion au serveur");
        return None;
    }
    socket.send("je suis connecte".as_bytes()).unwrap();
    Some(socket)
}

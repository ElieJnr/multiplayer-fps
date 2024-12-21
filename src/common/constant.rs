use lazy_static::lazy_static;
use std::{net::UdpSocket, sync::Mutex};

// Ce fichier nous permettra de definir nos variables globales.
// lazy_static est utiliser pour l'initialisation performante(paresseuse) de nos variable
// et les mutex pour protéger les variables
// nous avons aussi des getters et setters.

lazy_static! {
    static ref SERVER_ADDRESS: Mutex<Option<String>> = Mutex::new(None);
    static ref GLOBAL_SOCKET: Mutex<Option<UdpSocket>> = Mutex::new(None);
}
pub const PORT: &str = "8080";

pub fn get_server_address() -> Option<String> {
    SERVER_ADDRESS.lock().ok()?.clone()
}

pub fn set_server_address(address: String) {
    if let Ok(mut server_address) = SERVER_ADDRESS.lock() {
        *server_address = Some(address);
    }
}

pub fn get_global_socket() -> Option<UdpSocket> {
    GLOBAL_SOCKET.lock().ok()?.as_ref()?.try_clone().ok()
}

pub fn set_global_socket(socket: UdpSocket) {
    if let Ok(mut global_socket) = GLOBAL_SOCKET.lock() {
        *global_socket = Some(socket);
    }
}

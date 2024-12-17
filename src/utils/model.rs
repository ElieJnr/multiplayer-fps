use serde::{Deserialize,Serialize};

// Structure représentant la nouvelle connexion
#[derive(Debug, Deserialize, Serialize)]
pub struct NewConnexion {
    pub name: String,
}

// Structure représentant le message à envoyer
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub message_type: String,
    pub message_content: AllOption,
}
// Structure englobant l'option de message
#[derive(Debug, Deserialize,Serialize)]
pub struct AllOption {
    pub new_connexion: NewConnexion,
}

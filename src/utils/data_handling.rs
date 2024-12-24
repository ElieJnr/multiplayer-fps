use super::model::GameMessage;

pub fn serialize_message(message: &GameMessage) -> Option<Vec<u8>> {
    match serde_json::to_string(message) {
        Ok(json) => {
            let msg_bytes = json.as_bytes().to_vec();
            Some(msg_bytes)
        },
        Err(err) => {
            eprintln!("Failed to serialize message: {}", err);
            None
        }
    }
}

pub fn deserialize_message(data: &[u8]) -> Option<GameMessage> {
    match serde_json::from_slice(data) {
        Ok(message) => Some(message),
        Err(e) => {
            eprintln!("Erreur lors de la désérialisation: {}", e);
            None
        }
    }
}
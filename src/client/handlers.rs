use crate::{common::protocol::*, utils::logger::*};


pub fn handle_disconnect(content: MessageContent) {
    if let MessageContent::Disconnect { reason } = content {
        display_warning(&format!("[{}]", reason));
    } else {
        display_warning(&format!("Received invalid content type for NewConnection"));
    }
}

pub fn handle_server_info(content: MessageContent) {
    if let MessageContent::ServerInfo { server_status } = content {
        display_info(&format!("[{}]", server_status));
    } else {
        display_error("Received invalid content type for ServerInfo");
    }
}

pub fn handle_new_connection(content: MessageContent) {
    if let MessageContent::NewConnection { name } = content {
        display_info(&format!("[{} has joined the game.]", name));
    } else {
        display_error("Received invalid content type for NewConnection");
    }
}

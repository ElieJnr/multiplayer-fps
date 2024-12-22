use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::graphics::resources::PlayerCountState;

pub const PORT: &str = "8080";
pub const MIN_PLAYERS: usize = 0; 

lazy_static! {
    pub static ref PLAYER_COUNT_STATE: Mutex<PlayerCountState> = Mutex::new(PlayerCountState::default());
}
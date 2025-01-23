pub const PORT: &str = "8080";


pub const DEFAULT_MIN_PLAYERS: usize = 2;
#[derive(Debug)]
pub struct PlayerCount {
    min_players: usize,
}

impl PlayerCount {
    pub fn new(min_players: usize) -> Self {
        PlayerCount { min_players }
    }

    pub fn get_min_players(&self) -> usize {
        self.min_players
    }

    pub fn set_min_players(&mut self, min_players: usize) {
        self.min_players = min_players;
    }
}

pub const PORT: &str = "8080";

pub const MODEL_URL: &str = "https://cdn.discordapp.com/attachments/1155879899626754200/1330883259739013231/player.glb?ex=678f991f&is=678e479f&hm=1f6c4394321a4fe7bc0045bf6a090424b82cc86417a43aae86c1b73481d702b7&";

pub const PATH_FOR_MODEL: &str = "model/player.glb";

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

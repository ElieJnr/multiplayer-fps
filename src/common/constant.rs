pub const PORT: &str = "8080";

pub const MODEL_URL: &str = "https://download1979.mediafire.com/lezuczltv1ng9_AzAOp6Xk8uYiMANPw6Kd_xDRWm77wt03LbTxuwEe04KOOQJ6Qq4aMsouFtoNLp5ZjEi8hp8Xp7f_U-98lhsnJp2NalDsAEHjDAqF1wNZeCwMxqBOHhw6Bc04tgep-OlcM2z-Gue6aOWD7Ao-UuepkK5SGVA7wSzT4/n6xa7bb1nus2ejq/player.glb";

pub const PATH_FOR_MODEL: &str = "assets/player.glb";

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

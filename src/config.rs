use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    io,
};

use serde::{Deserialize, Serialize};
use serenity::model::prelude::{ChannelId, GuildId};
use strum::{Display, EnumString};

const CONFIG_FILE: &str = "config.json";

#[derive(
    Debug, PartialEq, Eq, PartialOrd, EnumString, Display, Serialize, Deserialize, Clone, Copy,
)]
pub enum LeaderboardOrdering {
    LocalScore,
    GlobalScore,
    Stars,
}

#[derive(Serialize, Deserialize)]
pub struct GuildConfig {
    pub session_token: String,
    pub leaderboard_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct DailyLeaderboardConfig {
    pub guild_id: GuildId,
    pub hour: Option<usize>,
    pub ordering: LeaderboardOrdering,
}

#[derive(Serialize, Deserialize)]
pub struct DailyPuzzleConfig {
    pub guild_id: GuildId,
    pub hour: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub guild_configs: HashMap<GuildId, GuildConfig>,
    pub daily_leaderboard_configs: HashMap<ChannelId, DailyLeaderboardConfig>,
    pub daily_puzzle_configs: HashMap<ChannelId, DailyPuzzleConfig>,
}

impl Config {
    fn new() -> Config {
        Config {
            guild_configs: HashMap::new(),
            daily_leaderboard_configs: HashMap::new(),
            daily_puzzle_configs: HashMap::new(),
        }
    }

    pub fn get() -> Result<Self, std::io::Error> {
        // your turn...
        match read_to_string(CONFIG_FILE) {
            Ok(json) => {
                let config: Self = serde_json::from_str(&json)?;
                Ok(config)
            }
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(Config::new()),
                _ => Err(err),
            },
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        write(CONFIG_FILE, serde_json::to_string(self)?) // @ando: why can't we use Box<dyn Error> plz help tyvm
    }
}

impl Drop for Config {
    // Attempts to save the config when it goes out of scope
    fn drop(&mut self) {
        self.save().ok();
    }
}

mod leaderboard;
mod puzzle;

use std::{collections::HashMap, error::Error, sync::Arc};

use chrono::Utc;
use reqwest::Client;

pub use leaderboard::{fetch_leaderboard, Leaderboard, LeaderboardCacheEntry, LeaderboardCacheKey};
pub use puzzle::{fetch_puzzle_details, PuzzleDetails, PuzzleKey};

const CACHE_TTL_SECS: i64 = 900;

pub struct AOCData {
    leaderboards: HashMap<LeaderboardCacheKey, Arc<LeaderboardCacheEntry>>,
    http_client: Client,
    puzzles: HashMap<PuzzleKey, PuzzleDetails>,
}

impl AOCData {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            leaderboards: HashMap::new(),
            puzzles: HashMap::new(),
        }
    }

    pub async fn get_puzzle_details(
        &mut self,
        year: usize,
        day: usize,
    ) -> Result<PuzzleDetails, Box<dyn Error>> {
        let key = (year, day);
        match self.puzzles.get(&key) {
            Some(puzzle) => Ok(puzzle.clone()),
            _ => {
                let puzzle_details = fetch_puzzle_details(&self.http_client, year, day).await;
                puzzle_details.map(|pd| {
                    self.puzzles.insert(key, pd.clone());
                    pd
                })
            }
        }
    }

    pub async fn get_leaderboard(
        &mut self,
        event_id: &str,
        leaderboard_id: &str,
        session_token: &str,
        skip_cache: bool,
    ) -> Result<Arc<LeaderboardCacheEntry>, Box<dyn Error>> {
        let key = LeaderboardCacheKey::new(event_id, leaderboard_id);
        match self.leaderboards.get(&key) {
            // If we have an unexpired cache entry, return it
            Some(entry) if !entry.is_expired() && !skip_cache => Ok(entry.clone()),

            // Otherwise, fetch and then cache it
            _ => fetch_leaderboard(&self.http_client, event_id, leaderboard_id, session_token)
                .await
                .map(|leaderboard| {
                    let entry = Arc::new(LeaderboardCacheEntry {
                        leaderboard,
                        leaderboard_id: leaderboard_id.to_owned(),
                        created_at: Utc::now(),
                    });
                    self.leaderboards.insert(key, entry.clone());
                    entry
                }),
        }
    }
}

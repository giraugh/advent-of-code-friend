use std::{collections::HashMap, error::Error, sync::Arc};

use chrono::Utc;
use reqwest::{Client, StatusCode};
use serde::Deserialize;

const CACHE_TTL_SECS: i64 = 900;

pub struct AOCData {
    leaderboards: HashMap<LeaderboardCacheKey, Arc<LeaderboardCacheEntry>>,
    http_client: Client,
    puzzles: HashMap<PuzzleKey, PuzzleDetails>,
}

/// Unique identifier for a puzzle, consists of (year, day)
pub type PuzzleKey = (usize, usize);

/// Data associated with a puzzle
#[derive(Debug, Clone)]
pub struct PuzzleDetails {
    /// Name of the puzzle
    name: String,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct LeaderboardCacheKey(String, String);
impl LeaderboardCacheKey {
    pub fn new(event_id: &str, leaderboard_id: &str) -> Self {
        Self(event_id.to_owned(), leaderboard_id.to_owned())
    }
}

pub struct LeaderboardCacheEntry {
    pub leaderboard: Leaderboard,
    pub leaderboard_id: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl LeaderboardCacheEntry {
    pub fn is_expired(&self) -> bool {
        Utc::now()
            .signed_duration_since(self.created_at)
            .num_seconds()
            > CACHE_TTL_SECS
    }
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
    ) -> Result<Arc<LeaderboardCacheEntry>, Box<dyn Error>> {
        let key = LeaderboardCacheKey::new(event_id, leaderboard_id);
        match self.leaderboards.get(&key) {
            // If we have an unexpired cache entry, return it
            Some(entry) if !entry.is_expired() => Ok(entry.clone()),

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

#[derive(Deserialize)]
pub struct Leaderboard {
    /// What event this leaderboard is for. Typically this is the year (e.g 2020)
    pub event: String,

    /// ID of the user that owns this leaderboard
    pub owner_id: usize,

    /// Members and their leaderboard values
    pub members: HashMap<String, LeaderboardMember>,
}

#[derive(Deserialize)]
pub struct LeaderboardMember {
    /// Name of user
    pub name: Option<String>,

    /// Time of last star acquisition (unix seconds)
    pub last_star_ts: usize,

    /// ID of user
    pub id: usize,

    /// Number of stars user has collected
    pub stars: usize,

    /// Users global score in the leaderboard
    pub global_score: usize,

    /// Users local score in the leaderboard
    pub local_score: usize,

    /// The level of completion for each day of the event
    pub completion_day_level: HashMap<usize, HashMap<usize, CompletionDayLevelEntry>>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct CompletionDayLevelEntry {
    star_index: usize,
    get_star_ts: usize,
}

async fn fetch_puzzle_details(
    client: &reqwest::Client,
    year: usize,
    day: usize,
) -> Result<PuzzleDetails, Box<dyn Error>> {
    todo!()
}

async fn fetch_leaderboard(
    client: &reqwest::Client,
    event_id: &str,
    leaderboard_id: &str,
    session_token: &str,
) -> Result<Leaderboard, Box<dyn Error>> {
    // Fetch
    let res = client
        .get(format!(
            "https://adventofcode.com/{event_id}/leaderboard/private/view/{leaderboard_id}.json"
        ))
        .header("cookie", format!("session={session_token}"))
        .send()
        .await?;

    // Check that the request was successfull
    if res.status() == StatusCode::NOT_FOUND {
        return Err("No such leaderboard found".to_owned().into());
    }

    // Parse
    let leaderboard = res.json().await?;

    // Return result
    Ok(leaderboard)
}

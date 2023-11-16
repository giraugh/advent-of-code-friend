use std::{collections::HashMap, error::Error};

use chrono::Utc;
use reqwest::StatusCode;
use serde::Deserialize;

use super::CACHE_TTL_SECS;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct LeaderboardCacheKey(String, String);
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

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct CompletionDayLevelEntry {
    pub star_index: usize,
    pub get_star_ts: usize,
}

pub async fn fetch_leaderboard(
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

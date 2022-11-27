use std::collections::HashMap;

use serde::Deserialize;

static cache: Option<Box<CacheData>> = None;

struct CacheData {
    pub leaderboards: Vec<Leaderboard>,
}

#[derive(Deserialize, Debug)]
pub struct Leaderboard {
    /// What event this leaderboard is for. Typically this is the year (e.g 2020)
    pub event: String,

    /// ID of the user that owns this leaderboard
    pub owner_id: usize,

    /// Members and their leaderboard values
    pub members: HashMap<String, LeaderboardMember>,
}

#[derive(Deserialize, Debug)]
pub struct LeaderboardMember {
    /// Name of user
    pub name: String,

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

#[derive(Deserialize, Debug)]
pub struct CompletionDayLevelEntry {
    star_index: usize,
    get_star_ts: usize,
}

async fn fetch_leaderboard(
    client: &reqwest::Client,
    event_id: &str,
    leaderboard_id: &str,
    session_token: &str,
) -> Result<Leaderboard, reqwest::Error> {
    // Fetch
    let res = client
        .get(format!(
            "https://adventofcode.com/{event_id}/leaderboard/private/view/{leaderboard_id}.json"
        ))
        .header("cookie", format!("session={session_token}"))
        .send()
        .await?;

    // Parse
    let leaderboard = res.json().await?;

    // Return result
    Ok(leaderboard)
}

async fn get_cached_leaderboard(event_id: &str, leaderboard_id: &str) -> Option<Leaderboard> {
    // TODO
    None
}

pub async fn get_leaderboard(
    client: &reqwest::Client,
    event_id: &str,
    leaderboard_id: &str,
    session_token: &str,
) -> Option<Leaderboard> {
    // Attempt to get leaderboard from cache

    // Fetch the leaderboard
    let leaderboard = fetch_leaderboard(client, event_id, leaderboard_id, session_token)
        .await
        .ok();

    // Cache the result

    // Return leaderboard
    leaderboard
}

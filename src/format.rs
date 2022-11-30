use std::sync::Arc;

use serenity::builder::CreateEmbed;

use crate::{
    aoc::{Leaderboard, LeaderboardCacheEntry},
    config::LeaderboardOrdering,
};

pub const EMBED_COLOR: i32 = 0xFFFE60;

macro_rules! trunc {
    ($s:expr, $n: expr) => {{
        let mut s = $s.clone();
        if (s.len() > $n) {
            s.truncate($n - 3);
            format!("{}...", s)
        } else {
            s
        }
    }};
}

pub fn make_leaderboard_embed(
    leaderboard: Arc<LeaderboardCacheEntry>,
    ordering: LeaderboardOrdering,
) -> CreateEmbed {
    CreateEmbed::default()
        .title("🏆  Leaderboard")
        .description(leaderboard_embed_content(
            &leaderboard.leaderboard,
            ordering,
        ))
        .timestamp(leaderboard.created_at.to_rfc3339())
        .url(generate_leaderboard_url(
            &leaderboard.leaderboard.event,
            &leaderboard.leaderboard_id,
        ))
        .color(EMBED_COLOR)
        .footer(|f| f.text(format!("Year {}", leaderboard.leaderboard.event)))
        .to_owned()
}

pub fn leaderboard_embed_content(
    leaderboard: &Leaderboard,
    ordering: LeaderboardOrdering,
) -> String {
    // Collect member entries
    let mut members: Vec<_> = leaderboard
        .members
        .iter()
        .map(|(_, member)| member)
        .collect();

    // Get longest name
    let longest_name_len = members
        .iter()
        .map(|member| member.name.len())
        .max()
        .unwrap_or(20)
        .min(20);

    // Sort them
    members.sort_by(|a, b| match ordering {
        LeaderboardOrdering::LocalScore => a.local_score.cmp(&b.local_score),
        LeaderboardOrdering::GlobalScore => a.global_score.cmp(&b.global_score),
        LeaderboardOrdering::Stars => match a.stars.cmp(&b.stars) {
            std::cmp::Ordering::Equal => a.last_star_ts.cmp(&b.last_star_ts),
            x => x,
        },
    });

    let content: String = members
        .iter()
        .enumerate()
        .map(|(i, member)| {
            format!(
                "{}: {}  {} ⭐️\n",
                format_args!(
                    "{:0>width$}",
                    i + 1,
                    width = match members.len() {
                        x if x < 10 => 1,
                        x if x < 100 => 2,
                        _ => 3,
                    }
                ),
                format_args!(
                    "{:width$}",
                    trunc!(member.name, 20),
                    width = longest_name_len
                ),
                format_args!("{:0>2}", member.local_score),
            )
        })
        .collect();

    format!("```js\n{}```", content)
}

pub fn make_puzzle_embed(year: i32, day: u32, new: bool) -> CreateEmbed {
    let puzzle_url = generate_puzzle_url(year, day);

    CreateEmbed::default()
        .title(format!(
            "{} Day {day}, {year}",
            if new { "🎁  New Puzzle:" } else { "🧩 " }
        )) // TODO: Scrape name of puzzle from the page
        .description(&puzzle_url)
        .url(&puzzle_url)
        .color(EMBED_COLOR)
        .to_owned()
}

pub enum ResponseReason {
    Success,
    Error,
}

pub fn make_message_embed(reason: ResponseReason, message: &str) -> CreateEmbed {
    CreateEmbed::default()
        .title(match reason {
            ResponseReason::Success => "✅  Success",
            ResponseReason::Error => "❌  Error",
        })
        .color(match reason {
            ResponseReason::Success => 0x77B256,
            ResponseReason::Error => 0xDD2D44,
        })
        .description(message)
        .to_owned()
}

pub fn generate_leaderboard_url(year: &str, id: &str) -> String {
    format!(
        "https://adventofcode.com/{}/leaderboard/private/view/{}",
        year, id
    )
}

pub fn generate_puzzle_url(year: i32, day: u32) -> String {
    format!("https://adventofcode.com/{}/day/{}", year, day)
}

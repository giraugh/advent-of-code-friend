use std::fmt::Write;
use std::sync::Arc;

use serenity::builder::CreateEmbed;

use crate::{
    aoc::{Leaderboard, LeaderboardCacheEntry, PuzzleDetails},
    config::LeaderboardOrdering,
};

pub const EMBED_COLOR: i32 = 0xFFFE60;
const MAX_NAME_LENGTH: usize = 30;

macro_rules! trunc {
    ($s:expr, $n: expr) => {{
        let mut s = $s.clone();
        if (s.len() > $n) {
            s.truncate($n - 3);
            format!("{}...", s.trim())
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
    let mut members: Vec<_> = leaderboard.members.values().collect();

    // Get longest name
    let longest_name_len = members
        .iter()
        .map(|member| {
            member
                .name
                .to_owned()
                .unwrap_or(format!("Anon #{}", member.id))
                .len()
        })
        .max()
        .unwrap_or(MAX_NAME_LENGTH)
        .min(MAX_NAME_LENGTH);

    // Get longest score
    let longest_score_len = members
        .iter()
        .map(|member| match ordering {
            LeaderboardOrdering::LocalScore => member.local_score.to_string().len(),
            LeaderboardOrdering::GlobalScore => member.global_score.to_string().len(),
            LeaderboardOrdering::Stars => member.stars.to_string().len(),
        })
        .max()
        .unwrap_or(2);

    // Sort them
    // TODO: sort_by should be stable, but appears to reorder equal elements?
    members.sort_by(|a, b| match ordering {
        // Local score (default)
        LeaderboardOrdering::LocalScore => b.local_score.cmp(&a.local_score),
        // Global score (ties broken by local score)
        LeaderboardOrdering::GlobalScore => match b.global_score.cmp(&a.global_score) {
            std::cmp::Ordering::Equal => b.local_score.cmp(&a.local_score),
            x => x,
        },
        // Stars (ties broken by who got the most recent star first)
        LeaderboardOrdering::Stars => match b.stars.cmp(&a.stars) {
            std::cmp::Ordering::Equal => a.last_star_ts.cmp(&b.last_star_ts),
            x => x,
        },
    });

    let content: String = members
        .iter()
        .enumerate()
        .fold(String::new(), |mut out, (i, member)| {
            let _ = writeln!(
                out,
                "{}: {}  {} {}",
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
                    trunc!(
                        member
                            .name
                            .to_owned()
                            .unwrap_or(format!("Anon #{}", member.id)),
                        MAX_NAME_LENGTH
                    ),
                    width = longest_name_len,
                ),
                format_args!(
                    "{: >width$}",
                    match ordering {
                        LeaderboardOrdering::LocalScore => member.local_score,
                        LeaderboardOrdering::GlobalScore => member.global_score,
                        LeaderboardOrdering::Stars => member.stars,
                    },
                    width = longest_score_len,
                ),
                match ordering {
                    LeaderboardOrdering::Stars => "⭐️",
                    _ => "💎",
                },
            );
            out
        });

    format!("```js\n{}```", content)
}

pub fn make_puzzle_embed(
    year: usize,
    day: usize,
    details: Option<PuzzleDetails>,
    new: bool,
) -> CreateEmbed {
    let puzzle_url = generate_puzzle_url(year, day);

    let title_prefix = if new { "🎁  New Puzzle:" } else { "🧩 " };
    let title = match details {
        Some(PuzzleDetails { name }) => format!("{title_prefix} {name} (Day {day}, {year})"),
        None => format!("{title_prefix} Day {day}, {year}"),
    };

    CreateEmbed::default()
        .title(title)
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

pub fn generate_puzzle_url(year: usize, day: usize) -> String {
    format!("https://adventofcode.com/{}/day/{}", year, day)
}

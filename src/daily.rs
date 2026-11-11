use std::{collections::HashMap, fmt::Display, sync::Arc};

use chrono::{DateTime, Datelike, DurationRound, FixedOffset, Months, Timelike, Utc};
use serenity::{
    all::{ActivityData, CreateMessage},
    prelude::Context,
};
use tokio::{join, sync::Mutex};

use crate::{
    aoc::AOCData,
    config::Config,
    format::{make_leaderboard_embed, make_puzzle_embed},
};

const EST_SECS: i32 = -5 * 60 * 60;
pub const EST_TZ: FixedOffset = FixedOffset::east_opt(EST_SECS).unwrap();

pub fn is_puzzle_day(time: &DateTime<FixedOffset>) -> bool {
    time.month() == 12 && time.day() <= 12
}

fn duration_to_next_hour(now: DateTime<FixedOffset>) -> chrono::Duration {
    let one_hour = chrono::Duration::hours(1);
    let hour_start = now.duration_trunc(one_hour).expect("in range date");
    let hour_duration = now - hour_start;
    one_hour - hour_duration
}

fn duration_to_next_december(now: DateTime<FixedOffset>) -> chrono::Duration {
    let next_year = now
        .checked_add_months(Months::new(12))
        .and_then(|next_year| {
            next_year
                // Set first day of the month.
                .with_day(1)?
                // Zero out time component.
                .with_hour(0)?
                .with_minute(0)?
                .with_second(0)?
                .with_nanosecond(0)
        })
        .expect("in range date");

    // Jump backwards a few months to get to december.
    let next_december = next_year
        .checked_sub_months(Months::new(next_year.month() % 12))
        .expect("in range date");

    next_december - now
}

#[derive(Clone, Debug, Default)]
struct DailyPostJob {
    /// Timestamp of the last post.
    last_post: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DailyPostOutcome {
    /// Post the embeds, and try again.
    PostEmbeds,
    /// Try again at the start of the next hour.
    NextHour,
    /// Try again next december.
    NextDecember,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActivityDataMessage {
    Waiting,
    Day { day: usize, released: bool },
}
impl ActivityDataMessage {
    pub fn day_released(day: usize) -> Self {
        Self::Day {
            day,
            released: true,
        }
    }

    pub fn day_unreleased(day: usize) -> Self {
        Self::Day {
            day,
            released: false,
        }
    }
}

impl Display for ActivityDataMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Waiting => write!(f, "Waiting for Advent of Code"),
            Self::Day { day, released } => {
                write!(f, "Advent of Code Day {day}")?;

                if *released {
                    write!(f, " (released!)")?;
                }

                Ok(())
            }
        }
    }
}

impl DailyPostJob {
    pub async fn start(aoc_data: Arc<Mutex<AOCData>>, ctx: Context) -> ! {
        let mut job = Self::new();

        loop {
            let now = Utc::now().with_timezone(&EST_TZ);
            let (outcome, message) = job.step(now);
            log::info!(
                "The current EST time is {} (outcome: {outcome:?})",
                now.format("[%Y-%m-%d][%H:%M:%S]")
            );
            ctx.set_activity(Some(ActivityData::playing(message.to_string())));

            match outcome {
                DailyPostOutcome::NextHour => {
                    let duration_hour_remaining = duration_to_next_hour(now);
                    log::info!(
                        "Waiting for {} mins until next hour...",
                        duration_hour_remaining.num_minutes()
                    );

                    tokio::time::sleep(
                        duration_hour_remaining
                            .to_std()
                            .expect("duration greater than zero"),
                    )
                    .await;
                }
                DailyPostOutcome::NextDecember => {
                    let duration_until_december = duration_to_next_december(now);
                    log::info!(
                        "Waiting for {} days until next december...",
                        duration_until_december.num_days()
                    );

                    tokio::time::sleep(
                        duration_until_december
                            .to_std()
                            .expect("duration greater than zero"),
                    )
                    .await;
                }
                DailyPostOutcome::PostEmbeds => {
                    // Get config
                    let config = Config::get().expect("Failed to get config");

                    let year = now.year() as usize;
                    let day = now.day() as usize;
                    let hour = now.hour() as usize;

                    // Post embeds
                    let lb_task =
                        post_daily_leaderboards(&ctx, &config, year, hour, aoc_data.clone());
                    let pz_task =
                        post_daily_puzzles(&ctx, &config, year, day, hour, aoc_data.clone());
                    join!(lb_task, pz_task);
                }
            }
        }
    }

    fn new() -> Self {
        Self::default()
    }

    /// Step the job with a current time.
    fn step(&mut self, now: DateTime<FixedOffset>) -> (DailyPostOutcome, ActivityDataMessage) {
        // Have we even made it to december yet?
        if !is_puzzle_day(&now) {
            log::info!("Not a puzzle day");
            self.last_post = None;
            return (DailyPostOutcome::NextDecember, ActivityDataMessage::Waiting);
        }

        // Have we already posted today's messages?
        if let Some(last_post) = self.last_post
            && (last_post.day(), last_post.year()) == (now.day(), now.year())
        {
            log::info!("Already posted today");
            return (
                DailyPostOutcome::NextHour,
                ActivityDataMessage::day_released(now.day() as usize),
            );
        }

        // TODO: Fetch this from a config or elsewhere.
        const PUZZLE_RELEASE_HOUR: u32 = 12;

        // Has the puzzle been released?
        if now.hour() < PUZZLE_RELEASE_HOUR {
            log::info!("Too early for puzzle");
            return (
                DailyPostOutcome::NextHour,
                ActivityDataMessage::day_unreleased(now.day() as usize),
            );
        }

        // It's time to post something!
        log::info!("Puzzle ready");
        self.last_post = Some(now);
        (
            DailyPostOutcome::PostEmbeds,
            ActivityDataMessage::day_released(now.day() as usize),
        )
    }
}

pub async fn daily_posts(aoc_data: Arc<Mutex<AOCData>>, ctx: Context) {
    DailyPostJob::start(aoc_data, ctx).await;
}

pub async fn post_daily_leaderboards(
    ctx: &Context,
    config: &Config,
    year: usize,
    hour: usize,
    aoc_data: Arc<Mutex<AOCData>>,
) {
    // Get data
    let mut aoc_data = aoc_data.lock().await;

    // Get configs to be posted this hour
    let current_configs: HashMap<_, _> = config
        .daily_leaderboard_configs
        .iter()
        .filter(|config| config.1.hour == hour)
        .collect();
    log::info!("Found {} leaderboards to be posted", current_configs.len());

    // Post embeds
    for (channel_id, lb_config) in current_configs {
        // Get guild config
        if let Some(guild_config) = config.guild_configs.get(&lb_config.guild_id) {
            // Get leaderboard
            let leaderboard = aoc_data
                .get_leaderboard(
                    &year.to_string(),
                    &guild_config.leaderboard_id,
                    &guild_config.session_token,
                    false,
                )
                .await
                .expect("Failed to get leaderboard");

            // Create and send embed
            let embed = make_leaderboard_embed(leaderboard, lb_config.ordering);
            channel_id
                .send_message(&ctx.http, CreateMessage::new().embed(embed))
                .await
                .expect("Failed to send embed");
        }
    }
}

pub async fn post_daily_puzzles(
    ctx: &Context,
    config: &Config,
    year: usize,
    day: usize,
    hour: usize,
    aoc_data: Arc<Mutex<AOCData>>,
) {
    // Get configs to be posted this hour
    let current_configs: HashMap<_, _> = config
        .daily_puzzle_configs
        .iter()
        .filter(|config| config.1.hour == hour)
        .collect();
    log::info!("Found {} puzzles to be posted", current_configs.len());

    let puzzle_details = {
        let mut aoc_data = aoc_data.lock().await;
        aoc_data.get_puzzle_details(year, day).await
    }
    .ok();

    // Post embeds
    for channel_id in current_configs.keys() {
        // Create and send embed
        let embed = make_puzzle_embed(year, day, puzzle_details.clone(), true);
        channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(embed))
            .await
            .expect("Failed to send embed");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::FixedOffset;
    use rstest::rstest;

    #[rstest]
    #[case::nyd("2026-01-01T00:00:00Z", "2026-01-01T01:00:00Z")]
    #[case::partial_hour("2026-01-01T00:12:34Z", "2026-01-01T01:00:00Z")]
    #[case::advances_hour("2026-01-01T05:00:00Z", "2026-01-01T06:00:00Z")]
    #[case::advances_day("2026-01-01T23:00:00Z", "2026-01-02T00:00:00Z")]
    fn next_hour(#[case] now: DateTime<FixedOffset>, #[case] expected: DateTime<FixedOffset>) {
        let duration = duration_to_next_hour(now);
        assert_eq!(now.checked_add_signed(duration).unwrap(), expected);
    }

    #[rstest]
    #[case::nyd("2026-01-01T00:00:00Z", "2026-12-01T00:00:00Z")]
    #[case::partial("2026-05-23T12:34:56Z", "2026-12-01T00:00:00Z")]
    #[case::start_december("2026-12-01T00:00:00Z", "2027-12-01T00:00:00Z")]
    #[case::in_december("2026-12-13T00:00:00Z", "2027-12-01T00:00:00Z")]
    #[case::end_december("2026-12-31T23:59:59Z", "2027-12-01T00:00:00Z")]
    fn next_december(#[case] now: DateTime<FixedOffset>, #[case] expected: DateTime<FixedOffset>) {
        let duration = duration_to_next_december(now);
        assert_eq!(now.checked_add_signed(duration).unwrap(), expected);
    }

    #[rstest]
    #[case::start_of_year(
        "2026-01-01T00:00:00-05:00",
        None,
        DailyPostOutcome::NextDecember,
        ActivityDataMessage::Waiting
    )]
    #[case::during_the_year(
        "2026-07-23T17:39:50-05:00",
        None,
        DailyPostOutcome::NextDecember,
        ActivityDataMessage::Waiting
    )]
    #[case::right_before_december(
        "2026-11-30T23:59:59-05:00",
        None,
        DailyPostOutcome::NextDecember,
        ActivityDataMessage::Waiting
    )]
    #[case::december_first_midnight(
        "2026-12-01T00:00:00-05:00",
        None,
        DailyPostOutcome::NextHour,
        ActivityDataMessage::day_unreleased(1)
    )]
    #[case::right_before_first_release(
        // TODO: Update this test case with puzzle release hour.
        "2026-12-01T11:59:59-05:00",
        None,
        DailyPostOutcome::NextHour,
        ActivityDataMessage::day_unreleased(1)
    )]
    #[case::first_release_exact(
        // TODO: Update this test case with puzzle release hour.
        "2026-12-01T12:00:00-05:00",
        None,
        DailyPostOutcome::PostEmbeds,
        ActivityDataMessage::day_released(1)
    )]
    #[case::first_release_within_the_hour(
        // TODO: Update this test case with puzzle release hour.
        "2026-12-01T12:31:43-05:00",
        None,
        DailyPostOutcome::PostEmbeds,
        ActivityDataMessage::day_released(1)
    )]
    #[case::first_release_many_hours_later(
        "2026-12-01T18:00:00-05:00",
        None,
        DailyPostOutcome::PostEmbeds,
        ActivityDataMessage::day_released(1)
    )]
    #[case::already_released_exact(
        // TODO: Update this test case with puzzle release hour.
        "2026-12-01T12:00:00-05:00",
        Some("2026-12-01T12:00:00-05:00".parse().unwrap()),
        DailyPostOutcome::NextHour,
        ActivityDataMessage::day_released(1)
    )]
    #[case::already_released(
        "2026-12-01T18:00:00-05:00",
        Some("2026-12-01T12:00:00-05:00".parse().unwrap()),
        DailyPostOutcome::NextHour,
        ActivityDataMessage::day_released(1)
    )]
    #[case::follow_yesterday(
        // TODO: Update this test case with puzzle release hour.
        "2026-12-02T12:00:00-05:00",
        Some("2026-12-01T12:00:00-05:00".parse().unwrap()),
        DailyPostOutcome::PostEmbeds,
        ActivityDataMessage::day_released(2)
    )]
    #[case::mid_month_start(
        // TODO: Update this test case with puzzle release hour.
        "2026-12-03T12:00:00-05:00",
        None,
        DailyPostOutcome::PostEmbeds,
        ActivityDataMessage::day_released(3)
    )]
    #[case::aoc_end(
        // TODO: Update this test case with puzzle release hour.
        "2026-12-12T12:00:00-05:00",
        Some("2026-12-12T12:00:00-05:00".parse().unwrap()),
        DailyPostOutcome::NextHour,
        ActivityDataMessage::day_released(12)
    )]
    #[case::december_end(
        "2026-12-31T00:00:00-05:00",
        // TODO: Update this test case with puzzle release hour.
        Some("2026-12-12T12:00:00-05:00".parse().unwrap()),
        DailyPostOutcome::NextDecember,
        ActivityDataMessage::Waiting
    )]
    fn daily_post_job(
        #[case] now: DateTime<FixedOffset>,
        #[case] last_post: Option<DateTime<FixedOffset>>,
        #[case] expected_outcome: DailyPostOutcome,
        #[case] expected_message: ActivityDataMessage,
    ) {
        let mut job = DailyPostJob { last_post };
        let (outcome, message) = job.step(now);

        assert_eq!(outcome, expected_outcome);
        assert_eq!(message, expected_message);
    }
}

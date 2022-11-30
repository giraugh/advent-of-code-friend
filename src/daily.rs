use std::{sync::Arc, time::Duration};

use chrono::{Datelike, FixedOffset, Timelike, Utc};
use serenity::{model::prelude::Activity, prelude::Context};
use tokio::{join, sync::Mutex};

use crate::{
    aoc::AOCData,
    config::Config,
    format::{make_leaderboard_embed, make_puzzle_embed},
};

const EST_SECS: i32 = -5 * 60 * 60;

pub async fn daily_posts(aoc_data: Arc<Mutex<AOCData>>, ctx: Context) {
    // Create EST timezone
    let tz = FixedOffset::east_opt(EST_SECS).unwrap();

    loop {
        // Get current time in EST
        let time = Utc::now().with_timezone(&tz);
        let year = time.year() as usize;
        let day = time.day() as usize;
        let hour = time.hour() as usize;

        // Wait until the hour
        let mins_until_hour = 60 - time.minute() as u64;
        log::info!("Waiting for {mins_until_hour} mins until next hour...");
        tokio::time::sleep(Duration::from_secs(60 * mins_until_hour)).await;

        // Is it not december yet?
        if time.month() != 12 {
            // Set our activity
            ctx.set_activity(Activity::playing("Waiting for Advent of Code"))
                .await;

            // Keep waiting...
            continue;
        }

        // Set our activity
        ctx.set_activity(Activity::playing(format!("Advent of Code Day {}", day)))
            .await;

        // Get config
        let config = Config::get().expect("Failed to get config");

        // Post embeds
        let lb_task = post_daily_leaderboards(&ctx, &config, year, hour, aoc_data.clone());
        let pz_task = post_daily_puzzles(&ctx, &config, year, day, hour);
        join!(lb_task, pz_task);
    }
}

pub async fn post_daily_leaderboards(
    ctx: &Context,
    config: &Config,
    current_year: usize,
    current_hour: usize,
    aoc_data: Arc<Mutex<AOCData>>,
) {
    // Get data
    let mut aoc_data = aoc_data.lock().await;

    // Post embeds
    for (channel_id, lb_config) in &config.daily_leaderboard_configs {
        // Is it the right hour?
        if lb_config.hour != current_hour {
            continue;
        }

        // Get guild config
        if let Some(guild_config) = config.guild_configs.get(&lb_config.guild_id) {
            // Get leaderboard
            let leaderboard = aoc_data
                .get_leaderboard(
                    &current_year.to_string(),
                    &guild_config.leaderboard_id,
                    &guild_config.session_token,
                )
                .await
                .expect("Failed to get leaderboard");

            // Create and send embed
            let embed = make_leaderboard_embed(leaderboard, lb_config.ordering);
            channel_id
                .send_message(&ctx.http, |message| message.set_embed(embed))
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
) {
    // Post embeds
    for (channel_id, pz_config) in &config.daily_puzzle_configs {
        // Is it the right hour?
        if pz_config.hour != hour {
            continue;
        }

        // Create and send embed
        let embed = make_puzzle_embed(year, day, true);
        channel_id
            .send_message(&ctx.http, |message| message.set_embed(embed))
            .await
            .expect("Failed to send embed");
    }
}

pub mod timing;

use std::{collections::HashMap, sync::Arc};

use chrono::{Datelike, Timelike};
use serenity::{
    all::{ActivityData, CreateMessage},
    prelude::Context,
};
use tokio::{join, sync::Mutex};

use crate::{
    aoc::AOCData,
    config::Config,
    daily_posting::timing::{is_puzzle_day, now_etc, wait_till_next_hour_approx},
    format::{make_leaderboard_embed, make_puzzle_embed},
};

pub async fn make_daily_posts(aoc_data: Arc<Mutex<AOCData>>, ctx: Context) {
    loop {
        let time = now_etc();
        let year = time.year() as usize;
        let day = time.day() as usize;
        let hour = time.hour() as usize;

        log::info!(
            "The current EST time is {}",
            time.format("[%Y-%m-%d][%H:%M:%S]")
        );

        // Is it an AOC puzzle day?
        if is_puzzle_day(&time) {
            // Set our activity
            ctx.set_activity(Some(ActivityData::playing(format!(
                "Advent of Code {year} - Day {day}",
            ))));

            // Get config
            let config = Config::get().expect("Failed to get config");

            // Post embeds
            join!(
                post_daily_leaderboards(&ctx, &config, year, hour, aoc_data.clone()),
                post_daily_puzzles(&ctx, &config, year, day, hour, aoc_data.clone()),
            );
        } else {
            log::info!("Not a puzzle day, skipping daily posts");

            // Set our activity
            ctx.set_activity(Some(ActivityData::playing("Waiting for Advent of Code")));
        }

        // and wait till next hour
        wait_till_next_hour_approx().await;
    }
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

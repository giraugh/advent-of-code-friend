mod aoc;
mod bot;
mod commands;
mod config;
mod daily;
mod format;

use bot::Bot;
use dotenv::dotenv;
use fern::colors::{Color, ColoredLevelConfig};
use std::env;

#[tokio::main]
async fn main() {
    init_logging();

    // Load env from .env
    dotenv().ok();

    // Get the token from the env
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Start discord bot task
    let bot_thread = tokio::spawn(Bot::start(token));

    // Wait for bot
    bot_thread.await.unwrap();
}

fn init_logging() {
    // Init logging
    let log_colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                log_colors.color(record.level()),
                record.target(),
                message,
            ))
        })
        .level(log::LevelFilter::Error)
        .level_for("advent_of_code_friend", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

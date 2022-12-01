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
        .warn(Color::Yellow)
        .error(Color::Red);
    fern::Dispatch::new()
        .level(log::LevelFilter::Error)
        .level_for("advent_of_code_friend", log::LevelFilter::Info)
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}] {}",
                        log_colors.color(record.level()),
                        record.target(),
                        message,
                    ))
                })
                .chain(std::io::stdout()),
        )
        .chain(
            fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.level(),
                        record.target(),
                        message,
                    ))
                })
                .chain(fern::log_file("aoc-friend.log").unwrap()),
        )
        .apply()
        .unwrap();
}

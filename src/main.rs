mod aoc;
mod bot;
mod commands;
mod config;
mod format;

use bot::Bot;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Load env from .env
    dotenv().ok();

    // Get the token from the env
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Start discord bot as tokio task
    let bot_thread = tokio::spawn(Bot::start(token));

    // Wait for bot
    bot_thread.await.unwrap();
}

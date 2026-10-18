use serenity::{
    all::{CommandInteraction, CreateCommand},
    prelude::Context,
};

use crate::bot::Bot;

use super::extract_subcommand;

mod leaderboard;
mod puzzle;
mod unregister;

pub async fn run(bot: &Bot, ctx: &Context, command: &CommandInteraction) {
    match extract_subcommand(&command.data.options) {
        Some((subcommand, opts)) => match subcommand.as_str() {
            "leaderboard" => leaderboard::run(bot, ctx, command, &opts).await,
            "puzzle" => puzzle::run(bot, ctx, command, &opts).await,
            "unregister" => unregister::run(bot, ctx, command, &opts).await,
            _ => panic!("Unknown subcommand"),
        },
        None => panic!("Command group called without subcommand"),
    }
}

pub fn command_builder() -> CreateCommand {
    CreateCommand::new("daily")
        .description("Register a daily update to a specific channel")
        .add_option(leaderboard::command_builder())
        .add_option(puzzle::command_builder())
        .add_option(unregister::command_builder())
}

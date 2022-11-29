use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use crate::bot::Bot;

use super::extract_subcommand;

pub mod leaderboard;
pub mod puzzle;

pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    match extract_subcommand(&command.data.options) {
        Some(subcommand) => match subcommand.name.as_str() {
            "leaderboard" => leaderboard::run(bot, ctx, command, subcommand).await,
            "puzzle" => puzzle::run(bot, ctx, command, subcommand).await,
            _ => panic!("Unknown subcommand"),
        },
        None => panic!("Command group called without subcommand"),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("daily")
        .description("Register a daily update to a specific channel")
        .add_option(leaderboard::register())
        .add_option(puzzle::register())
}

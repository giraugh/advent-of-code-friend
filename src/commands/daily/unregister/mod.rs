use serenity::{
    all::{CommandDataOption, CommandInteraction, CommandOptionType, CreateCommandOption},
    prelude::Context,
};

use crate::bot::Bot;

use super::super::extract_subcommand;

mod leaderboard;
mod puzzle;

pub async fn run(
    bot: &Bot,
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[CommandDataOption],
) {
    match extract_subcommand(options) {
        Some((subcommand, options)) => match subcommand.as_str() {
            "leaderboard" => leaderboard::run(bot, ctx, interaction, &options).await,
            "puzzle" => puzzle::run(bot, ctx, interaction, &options).await,
            _ => panic!("Unknown subcommand"),
        },
        None => panic!("Command group called without subcommand"),
    }
}

pub fn command_builder() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommandGroup,
        "unregister",
        "Remove a daily update from a specific channel",
    )
    .add_sub_option(leaderboard::command_builder())
    .add_sub_option(puzzle::command_builder())
}

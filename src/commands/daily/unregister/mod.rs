use serenity::{
    builder::CreateApplicationCommandOption,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{ApplicationCommandInteraction, CommandDataOption},
    },
    prelude::Context,
};

use crate::bot::Bot;

use super::super::extract_subcommand;

mod leaderboard;
mod puzzle;

pub async fn run(
    bot: &Bot,
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    command: &CommandDataOption,
) {
    match extract_subcommand(&command.options) {
        Some(subcommand) => match subcommand.name.as_str() {
            "leaderboard" => leaderboard::run(bot, ctx, interaction, subcommand).await,
            "puzzle" => puzzle::run(bot, ctx, interaction, subcommand).await,
            _ => panic!("Unknown subcommand"),
        },
        None => panic!("Command group called without subcommand"),
    }
}

pub fn register() -> CreateApplicationCommandOption {
    CreateApplicationCommandOption::default()
        .name("unregister")
        .description("Remove a daily update from a specific channel")
        .kind(CommandOptionType::SubCommandGroup)
        .add_sub_option(leaderboard::register())
        .add_sub_option(puzzle::register())
        .to_owned()
}

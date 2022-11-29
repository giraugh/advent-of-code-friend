use crate::bot::config::LeaderboardOrdering;
use crate::bot::Bot;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::prelude::Context;

use super::{extract_string_option, CommandOptions};

// Options //

struct LeaderboardCommandOptions {
    ordering: LeaderboardOrdering,
}

impl CommandOptions for LeaderboardCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            ordering: extract_string_option(options_list, "ordering")
                .and_then(|ordering| ordering.parse().ok())
                .unwrap_or(LeaderboardOrdering::GlobalScore),
        }
    }
}

// Command //

pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    // Parse command options
    let options = LeaderboardCommandOptions::from_options_list(&command.data.options);

    // Get leaderboard
    let leaderboard = bot
        .get_registered_leaderboard(
            command
                .guild_id
                .expect("Command to have associated guild id"),
        )
        .await;

    // TODO: format leaderboard properly

    // Respond
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.content(format!("leaderboard: {:?}", leaderboard))
            })
        })
        .await
        .expect("failed to create interaction response");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leaderboard")
        .description("Uses the registered leaderboard ID to fetch and post the leaderboard.")
        .create_option(|option| {
            option
                .name("ordering")
                .description(
                    "Method used to order people in the leaderboard (default is local-score)",
                )
                .kind(CommandOptionType::String)
                .add_string_choice("local-score", LeaderboardOrdering::LocalScore)
                .add_string_choice("global-score", LeaderboardOrdering::GlobalScore)
                .add_string_choice("stars", LeaderboardOrdering::Stars)
        })
}

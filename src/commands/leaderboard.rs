use crate::bot::Bot;
use crate::commands::make_error_response;
use crate::config::LeaderboardOrdering;
use crate::format::{make_leaderboard_embed, make_message_embed, ResponseReason};

use chrono::{Datelike, Utc};
use serenity::all::{
    CommandDataOption, CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
    CreateInteractionResponseFollowup,
};
use serenity::prelude::Context;

use super::{extract_int_option, extract_string_option, CommandOptions};

// Options //

struct LeaderboardCommandOptions {
    ordering: LeaderboardOrdering,
    year: i32,
}

impl CommandOptions for LeaderboardCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            ordering: extract_string_option(options_list, "ordering")
                .and_then(|ordering| ordering.parse().ok())
                .unwrap_or(LeaderboardOrdering::LocalScore),
            year: extract_int_option(options_list, "year")
                .map(|v| v as i32)
                .unwrap_or_else(|| Utc::now().year()),
        }
    }
}

// Command //

pub async fn run(bot: &Bot, ctx: &Context, command: &CommandInteraction) {
    // Parse command options
    let options = LeaderboardCommandOptions::from_options_list(&command.data.options);

    if options.year > Utc::now().year() {
        command
            .create_response(
                &ctx.http,
                make_error_response("You can't use a year in the future 🗞️"),
            )
            .await
            .expect("failed to create interaction response");
        return;
    }

    // Defer response
    command.defer(&ctx.http).await.unwrap();

    // Get leaderboard
    let guild_id = command.guild_id.expect("command to have guild id");
    let leaderboard = bot.get_registered_leaderboard(guild_id, options.year).await;

    // Respond
    match leaderboard {
        // If we can get the leaderboard...
        Ok(leaderboard) => {
            // Respond
            command
                .create_followup(
                    &ctx.http,
                    CreateInteractionResponseFollowup::new()
                        .add_embed(make_leaderboard_embed(leaderboard, options.ordering)),
                )
                .await
                .expect("failed to create followup");
        }

        // If something went wrong...
        Err(error) => {
            command.create_followup(&ctx.http, CreateInteractionResponseFollowup::new()
                .add_embed(make_message_embed(ResponseReason::Error,
                &format!("Failed to get leaderboard: {}\n\nYour session token may have expired. Please try rerunning `/register` with a fresh token.", error),
            ))).await.expect("failed to send error followup");
        }
    }
}

pub fn command_builder() -> CreateCommand {
    CreateCommand::new("leaderboard")
        .description("Uses the registered leaderboard ID to fetch and post the leaderboard")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "ordering",
                "Method used to order people in the leaderboard (default: local-score",
            )
            .add_string_choice("local-score", LeaderboardOrdering::LocalScore)
            .add_string_choice("global-score", LeaderboardOrdering::GlobalScore)
            .add_string_choice("stars", LeaderboardOrdering::Stars),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "year",
                "Year to fetch leaderboard for (default: current-year)",
            )
            .min_int_value(2015),
        )
}

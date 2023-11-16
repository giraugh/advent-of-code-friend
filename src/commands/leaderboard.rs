use crate::bot::Bot;
use crate::config::LeaderboardOrdering;
use crate::format::{make_leaderboard_embed, make_message_embed, ResponseReason};

use chrono::{Datelike, Utc};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
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

pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    // Parse command options
    let options = LeaderboardCommandOptions::from_options_list(&command.data.options);

    if options.year > Utc::now().year() {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| {
                    message.ephemeral(true).add_embed(make_message_embed(
                        ResponseReason::Error,
                        "You can't use a year in the future 🗞️",
                    ))
                })
            })
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
                .create_followup_message(&ctx.http, |message| {
                    message.add_embed(make_leaderboard_embed(leaderboard, options.ordering))
                })
                .await
                .expect("failed to create interaction response");
        }

        // If something went wrong...
        Err(error) => {
            command
                .create_followup_message(&ctx.http, |message| {
                    message.add_embed(make_message_embed(
                        ResponseReason::Error,
                        &format!("Failed to get leaderboard: {}\n\nYour session token may have expired. Please try rerunning `/register` with a fresh token.", error),
                    ))
                })
                .await
                .expect("failed to send error response");
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leaderboard")
        .description("Uses the registered leaderboard ID to fetch and post the leaderboard.")
        .create_option(|option| {
            option
                .name("ordering")
                .description(
                    "Method used to order people in the leaderboard (default: local-score)",
                )
                .kind(CommandOptionType::String)
                .add_string_choice("local-score", LeaderboardOrdering::LocalScore)
                .add_string_choice("global-score", LeaderboardOrdering::GlobalScore)
                .add_string_choice("stars", LeaderboardOrdering::Stars)
        })
        .create_option(|option| {
            option
                .name("year")
                .description("Year to fetch leaderboard for (default: current year)")
                .kind(CommandOptionType::Integer)
                .min_int_value(2015)
        })
}

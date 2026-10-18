use crate::bot::Bot;
use crate::commands::make_error_response;
use crate::config::{Config, GuildConfig};
use crate::format::{make_message_embed, ResponseReason};
use chrono::{Datelike, Utc};
use serenity::all::{
    CommandDataOption, CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::prelude::Context;

use super::{extract_string_option, CommandOptions};

struct RegisterCommandOptions {
    session_token: String,
    leaderboard_id: String,
}

impl CommandOptions for RegisterCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            session_token: extract_string_option(options_list, "session_token")
                .expect("Didn't find session token"),
            leaderboard_id: extract_string_option(options_list, "leaderboard_id")
                .expect("Didn't find leaderboard id"),
        }
    }
}

pub async fn run(bot: &Bot, ctx: &Context, command: &CommandInteraction) {
    // Parse options
    let options = RegisterCommandOptions::from_options_list(&command.data.options);
    let mut config = Config::get().expect("Failed to load config");

    // Try fetching leaderboard and fail if it doesn't exist
    let can_fetch_leaderboard = {
        let mut aoc_data = bot.aoc_data.lock().await;
        aoc_data
            .get_leaderboard(
                &Utc::now().year().to_string(),
                &options.leaderboard_id,
                &options.session_token,
                true,
            )
            .await
            .is_ok()
    };

    if !can_fetch_leaderboard {
        command.create_response(
            &ctx.http,
            make_error_response(
                "Something went wrong while trying to fetch that leaderboard. Please check your session token and leaderboard ID are correct then try again."
            )
        )
            .await
            .expect("To create error response");
        return;
    }

    // Save data
    config.guild_configs.insert(
        command.guild_id.expect("Expected guild ID"),
        GuildConfig {
            session_token: options.session_token,
            leaderboard_id: options.leaderboard_id,
        },
    );

    // Respond
    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .ephemeral(true)
                    .add_embed(make_message_embed(
                        ResponseReason::Success,
                        "Successfully registered your leaderboard to this server. You can now set up daily announcements and run `/leaderboard`.\n\nRun this command again to change the registration details, or use `/unregister` to remove this registration."
                    ))
            ),
        )
        .await
        .expect("to respond to command");
}

pub fn command_builder() -> CreateCommand {
    CreateCommand::new("register")
        .description("Bind a session token and leaderboard ID to this server")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "session_token",
                "Your AoC session cookie (Valid for approx 30 days",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "leaderboard_id",
                "Can be found at the enf of a private leaderboard page URL",
            )
            .required(true),
        )
}

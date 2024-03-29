use crate::bot::Bot;
use crate::config::{Config, GuildConfig};
use crate::format::{make_message_embed, ResponseReason};
use chrono::{Datelike, Utc};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
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

pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
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
        command.create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.ephemeral(true).add_embed(make_message_embed(
                    ResponseReason::Error,
                    "Something went wrong while trying to fetch that leaderboard. Please check your session token and leaderboard ID are correct and try again.",
                ))
            })
        }).await.expect("to repond to command");
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
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.ephemeral(true).add_embed(make_message_embed(
                    ResponseReason::Success,
                    "Successfully registered your leaderboard to this server. You can now set up daily announcements and run `/leaderboard`.\n\nRun this command again to change the registration details, or use `/unregister` to remove this registration.",
                ))
            })
        })
        .await
        .expect("to respond to command");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("register")
        .description("Bind a session token and leaderboard ID to this server")
        .create_option(|option| {
            option
                .name("session_token")
                .description("Your AoC session cookie (valid for approx. 30 days)")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("leaderboard_id")
                .description("Can be found at the end of a private leaderboard page URL")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

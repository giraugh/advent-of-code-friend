use crate::bot::Bot;
use crate::config::{Config, DailyLeaderboardConfig, LeaderboardOrdering};
use crate::format::{make_message_embed, ResponseReason};
use serenity::all::{
    ChannelId, CommandDataOption, CommandInteraction, CommandOptionType, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::prelude::Context;

use super::super::{
    extract_channel_option, extract_int_option, extract_string_option, CommandOptions,
};

struct DailyLeaderboardCommandOptions {
    channel: ChannelId,
    hour: isize,
    ordering: LeaderboardOrdering,
}

impl CommandOptions for DailyLeaderboardCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            channel: extract_channel_option(options_list, "channel").expect("Didn't find channel"),
            hour: extract_int_option(options_list, "hour").unwrap_or(0),
            ordering: extract_string_option(options_list, "ordering")
                .and_then(|ordering| ordering.parse().ok())
                .unwrap_or(LeaderboardOrdering::GlobalScore),
        }
    }
}

pub async fn run(
    _bot: &Bot,
    ctx: &Context,
    interaction: &CommandInteraction,
    options: &[CommandDataOption],
) {
    // Parse options
    let options = DailyLeaderboardCommandOptions::from_options_list(options);

    // Save data
    let mut config = Config::get().expect("Failed to load config");
    config.daily_leaderboard_configs.insert(
        options.channel,
        DailyLeaderboardConfig {
            guild_id: interaction.guild_id.expect("guild id"),
            hour: options.hour as usize,
            ordering: options.ordering,
        },
    );

    // Respond
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .ephemeral(true)
                    .add_embed(make_message_embed(ResponseReason::Success,
                        &format!(
                            "Successfully registered daily leaderboards to <#{}>. They will be posted at **{}** every day of December.\n\n\
                            Run this command again to update the settings, or use `/daily unregister leaderboard` to remove this daily.",
                            options.channel,
                            format_args!("{:0>2}:00 EST", options.hour),
                        ),
                    )),
            ),
        )
        .await
        .expect("to respond to command");
}

pub fn command_builder() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "leaderboard",
        "Send a daily leaderboard during December in a specific channel",
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::Channel,
            "channel",
            "Where you want the leaderboards to be sent",
        )
        .required(true),
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::Integer,
            "hour",
            "The hour of the day to post the leaderboard in EST (default: midnight)",
        )
        .min_int_value(0)
        .max_int_value(23),
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "ordering",
            "Method used to order people in the leaderboard (default: local-score)",
        )
        .add_string_choice("local-score", LeaderboardOrdering::LocalScore)
        .add_string_choice("global-score", LeaderboardOrdering::GlobalScore)
        .add_string_choice("stars", LeaderboardOrdering::Stars),
    )
}

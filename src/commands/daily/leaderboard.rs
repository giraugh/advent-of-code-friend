use crate::bot::Bot;
use crate::config::{Config, DailyLeaderboardConfig, LeaderboardOrdering};
use crate::format::{make_message_embed, ResponseReason};
use serenity::builder::CreateApplicationCommandOption;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::prelude::PartialChannel;
use serenity::prelude::Context;

use super::super::{
    extract_channel_option, extract_int_option, extract_string_option, CommandOptions,
};

struct DailyLeaderboardCommandOptions {
    channel: PartialChannel,
    hour: Option<isize>,
    ordering: LeaderboardOrdering,
}

impl CommandOptions for DailyLeaderboardCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            channel: extract_channel_option(options_list, "channel").expect("Didn't find channel"),
            hour: extract_int_option(options_list, "hour"),
            ordering: extract_string_option(options_list, "ordering")
                .and_then(|ordering| ordering.parse().ok())
                .unwrap_or(LeaderboardOrdering::GlobalScore),
        }
    }
}

pub async fn run(
    _bot: &Bot,
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    command: &CommandDataOption,
) {
    // Parse options
    let options = DailyLeaderboardCommandOptions::from_options_list(&command.options);

    // Save data
    let mut config = Config::get().expect("Failed to load config");
    config.daily_leaderboard_configs.insert(
        options.channel.id,
        DailyLeaderboardConfig {
            hour: options.hour,
            ordering: options.ordering,
        },
    );

    // Respond
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.ephemeral(true).embed(|e| {
                    make_message_embed(
                        e,
                        ResponseReason::Success,
                        &format!(
                            "Successfully registered daily leaderboards to <#{}>. They will be posted at **{}** every day of December.\n\nRun this command again to update the settings, or use `/daily leaderboard unregister` to remove this daily.",
                            options.channel.id,
                            options.hour.unwrap_or(0), // TODO: or midnight EST
                        ),
                    )
                })
            })
        })
        .await
        .expect("to respond to command");
}

pub fn register() -> CreateApplicationCommandOption {
    CreateApplicationCommandOption::default()
        .name("leaderboard")
        .description("Send a daily leaderboard during December in a specific channel")
        .kind(CommandOptionType::SubCommand)
        .create_sub_option(|suboption| {
            suboption
                .name("channel")
                .description("Where you want the leaderboards to be sent")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .create_sub_option(|suboption| {
            suboption
                .name("hour")
                .description(
                    "The hour of the day to post the leaderboard (default is midnight EST)",
                )
                .kind(CommandOptionType::Integer)
                .min_int_value(0)
                .max_int_value(23)
        })
        .create_sub_option(|suboption| {
            suboption
                .name("ordering")
                .description(
                    "Method used to order people in the leaderboard (default is local-score)",
                )
                .kind(CommandOptionType::String)
                .add_string_choice("local-score", LeaderboardOrdering::LocalScore)
                .add_string_choice("global-score", LeaderboardOrdering::GlobalScore)
                .add_string_choice("stars", LeaderboardOrdering::Stars)
        })
        .to_owned()
}
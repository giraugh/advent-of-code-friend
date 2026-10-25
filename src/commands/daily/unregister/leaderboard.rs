use crate::bot::Bot;
use crate::config::Config;
use crate::format::{make_message_embed, ResponseReason};
use serenity::all::{
    ChannelId, CommandDataOption, CommandInteraction, CommandOptionType, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::prelude::Context;

use super::super::super::{extract_channel_option, CommandOptions};

struct DailyUnregisterLeaderboardCommandOptions {
    channel: ChannelId,
}

impl CommandOptions for DailyUnregisterLeaderboardCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            channel: extract_channel_option(options_list, "channel").expect("Didn't find channel"),
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
    let options = DailyUnregisterLeaderboardCommandOptions::from_options_list(options);

    // Save data
    let mut config = Config::get().expect("Failed to load config");
    let removed_channel_config = config.daily_leaderboard_configs.remove(&options.channel);

    // Respond
    let message_response = CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .add_embed(if removed_channel_config.is_some() {
            make_message_embed(ResponseReason::Success, &format!("Successfully removed the daily leaderboard from <#{}>", options.channel))
        } else {
            make_message_embed(ResponseReason::Error, "There was no daily leaderboard on that channel. You can set one up with `/daily leaderboard`.")
        });
    interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(message_response),
        )
        .await
        .expect("to respond to command");
}

pub fn command_builder() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "leaderboard",
        "Where the daily is registered",
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::Channel,
            "channel",
            "Where the daily is registered",
        )
        .required(true),
    )
}

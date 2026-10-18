use crate::bot::Bot;
use crate::config::Config;
use crate::format::{make_message_embed, ResponseReason};
use serenity::all::{
    ChannelId, CommandDataOption, CommandInteraction, CommandOptionType, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::prelude::Context;

use super::super::super::{extract_channel_option, CommandOptions};

struct DailyUnregisterPuzzleCommandOptions {
    channel: ChannelId,
}

impl CommandOptions for DailyUnregisterPuzzleCommandOptions {
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
    let options = DailyUnregisterPuzzleCommandOptions::from_options_list(options);

    // Save data
    let mut config = Config::get().expect("Failed to load config");
    let removed_puzzle_config = config.daily_puzzle_configs.remove(&options.channel);

    // Respond
    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message({
                if removed_puzzle_config.is_some() {
                    CreateInteractionResponseMessage::new().ephemeral(true).add_embed(make_message_embed(
                        ResponseReason::Success,
                        &format!("Successfully removed the daily puzzle from <#{}>", options.channel),
                    ))
                } else {
                    CreateInteractionResponseMessage::new().ephemeral(true).add_embed(make_message_embed(
                        ResponseReason::Error,
                        "There was no daily puzzle on that channel. You can set one up with `/daily puzzle`.",
                    ))
                }

        }))
        .await
        .expect("to respond to command");
}

pub fn command_builder() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommandGroup,
        "puzzle",
        "Remove a daily puzzle update in a specific channel",
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

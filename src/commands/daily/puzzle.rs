use crate::bot::Bot;
use crate::config::{Config, DailyPuzzleConfig};
use crate::format::{make_message_embed, ResponseReason};
use serenity::all::{
    ChannelId, CommandDataOption, CommandInteraction, CommandOptionType, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::prelude::Context;

use super::super::{extract_channel_option, extract_int_option, CommandOptions};

struct DailyPuzzleCommandOptions {
    channel: ChannelId,
    hour: isize,
}

impl CommandOptions for DailyPuzzleCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            channel: extract_channel_option(options_list, "channel").expect("Didn't find channel"),
            hour: extract_int_option(options_list, "hour").unwrap_or(0),
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
    let options = DailyPuzzleCommandOptions::from_options_list(options);

    // Save data
    let mut config = Config::get().expect("Failed to load config");
    config.daily_puzzle_configs.insert(
        options.channel,
        DailyPuzzleConfig {
            guild_id: interaction.guild_id.expect("guild id"),
            hour: options.hour as usize,
        },
    );

    // Respond
    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().ephemeral(true).add_embed(make_message_embed(
            ResponseReason::Success,
            &format!(
                "Successfully registered daily puzzles to <#{}>. They will be posted at **{}** every day of December.\n\nRun this command again to update the settings, or use `/daily unregister puzzle` to remove this daily.",
                options.channel,
                format_args!("{:0>2}:00 EST", options.hour),
            ),
        ))))
        .await
        .expect("to respond to command");
}

pub fn command_builder() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "puzzle",
        "Send a daily puzzle during December in a specific channel",
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::Channel,
            "channel",
            "Where you want the puzzles to be sent",
        )
        .required(true),
    )
    .add_sub_option(
        CreateCommandOption::new(
            CommandOptionType::Integer,
            "hour",
            "The hour of the day to post the puzzle in EST (default: midnight)",
        )
        .min_int_value(0)
        .max_int_value(23),
    )
}

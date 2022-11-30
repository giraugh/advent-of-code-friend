use crate::bot::Bot;
use crate::config::Config;
use crate::format::{make_message_embed, ResponseReason};
use serenity::builder::CreateApplicationCommandOption;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::prelude::PartialChannel;
use serenity::prelude::Context;

use super::super::super::{extract_channel_option, CommandOptions};

struct DailyUnregisterPuzzleCommandOptions {
    channel: PartialChannel,
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
    interaction: &ApplicationCommandInteraction,
    command: &CommandDataOption,
) {
    // Parse options
    let options = DailyUnregisterPuzzleCommandOptions::from_options_list(&command.options);

    // Save data
    let mut config = Config::get().expect("Failed to load config");
    let removed_puzzle_config = config.daily_puzzle_configs.remove(&options.channel.id);

    // Respond
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                if removed_puzzle_config.is_some() {
                    message.ephemeral(true).add_embed(make_message_embed(
                        ResponseReason::Success,
                        &format!("Successfully removed the daily puzzle from <#{}>", options.channel.id),
                    ))
                } else {
                    message.ephemeral(true).add_embed(make_message_embed(
                        ResponseReason::Error,
                        "There was no daily puzzle on that channel. You can set one up with `/daily puzzle`.",
                    ))
                }
            })
        })
        .await
        .expect("to respond to command");
}

pub fn register() -> CreateApplicationCommandOption {
    CreateApplicationCommandOption::default()
        .name("puzzle")
        .description("Remove a daily puzzle update in a specific channel")
        .kind(CommandOptionType::SubCommand)
        .create_sub_option(|option| {
            option
                .name("channel")
                .description("Where the daily is registered")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .to_owned()
}

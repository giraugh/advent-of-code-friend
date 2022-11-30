use crate::bot::Bot;
use crate::config::{Config, DailyPuzzleConfig};
use crate::format::{make_message_embed, ResponseReason};
use serenity::builder::CreateApplicationCommandOption;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::prelude::PartialChannel;
use serenity::prelude::Context;

use super::super::{extract_channel_option, extract_int_option, CommandOptions};

struct DailyPuzzleCommandOptions {
    channel: PartialChannel,
    hour: Option<isize>,
}

impl CommandOptions for DailyPuzzleCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            channel: extract_channel_option(options_list, "channel").expect("Didn't find channel"),
            hour: extract_int_option(options_list, "hour"),
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
    let options = DailyPuzzleCommandOptions::from_options_list(&command.options);

    // Save data
    let mut config = Config::get().expect("Failed to load config");
    config.daily_puzzle_configs.insert(
        options.channel.id,
        DailyPuzzleConfig {
            guild_id: interaction.guild_id.expect("guild id"),
            hour: options.hour.map(|h| h as usize),
        },
    );

    // Respond
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.ephemeral(true).add_embed(make_message_embed(
                    ResponseReason::Success,
                    &format!(
                        "Successfully registered daily puzzles to <#{}>. They will be posted at **{}** every day of December.\n\nRun this command again to update the settings, or use `/daily unregister puzzle` to remove this daily.",
                        options.channel.id,
                        format_args!("{:0>2}:00 EST", options.hour.unwrap_or(0)),
                    ),
                ))
            })
        })
        .await
        .expect("to respond to command");
}

pub fn register() -> CreateApplicationCommandOption {
    CreateApplicationCommandOption::default()
        .name("puzzle")
        .description("Send a daily puzzle during December in a specific channel")
        .kind(CommandOptionType::SubCommand)
        .create_sub_option(|option| {
            option
                .name("channel")
                .description("Where you want the puzzles to be sent")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .create_sub_option(|option| {
            option
                .name("hour")
                .description("The hour of the day to post the puzzle in EST (default: midnight)")
                .kind(CommandOptionType::Integer)
                .min_int_value(0)
                .max_int_value(23)
        })
        .to_owned()
}

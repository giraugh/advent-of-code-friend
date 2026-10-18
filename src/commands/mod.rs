use serenity::all::{
    ChannelId, CommandDataOption, CommandDataOptionValue, CommandOptionType,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};

use crate::format::{make_message_embed, ResponseReason};

pub mod daily;
pub mod help;
pub mod leaderboard;
pub mod puzzle;
pub mod register;
pub mod status;
pub mod unregister;

fn extract_string_option(options_list: &[CommandDataOption], option_name: &str) -> Option<String> {
    let option = options_list.iter().find(|opt| opt.name == option_name)?;
    let CommandDataOptionValue::String(value) = &option.value else {
        panic!("Expected string option");
    };
    Some(value.clone())
}

fn extract_int_option(options_list: &[CommandDataOption], option_name: &str) -> Option<isize> {
    let option = options_list.iter().find(|opt| opt.name == option_name)?;
    let CommandDataOptionValue::Integer(value) = option.value else {
        panic!("Expected integer option");
    };
    Some(value as isize)
}

fn extract_channel_option(
    options_list: &[CommandDataOption],
    option_name: &str,
) -> Option<ChannelId> {
    let option = options_list.iter().find(|opt| opt.name == option_name)?;
    let CommandDataOptionValue::Channel(value) = option.value else {
        panic!("Expected channel option");
    };
    Some(value)
}

/// Returns name and sub-options of a subcommand option
fn extract_subcommand(
    options_list: &[CommandDataOption],
) -> Option<(String, Vec<CommandDataOption>)> {
    let option = options_list.iter().find(|opt| {
        opt.kind() == CommandOptionType::SubCommand
            || opt.kind() == CommandOptionType::SubCommandGroup
    })?;
    let opts = match &option.value {
        CommandDataOptionValue::SubCommand(c) => c,
        CommandDataOptionValue::SubCommandGroup(c) => c,
        _ => panic!("Expected subcommand option"),
    };
    Some((option.name.clone(), opts.clone()))
}

trait CommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self;
}

pub fn make_error_response(message: &str) -> CreateInteractionResponse {
    CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .ephemeral(true)
            .add_embed(make_message_embed(ResponseReason::Error, message)),
    )
}

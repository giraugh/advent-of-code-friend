use serenity::model::prelude::{
    command::CommandOptionType,
    interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    PartialChannel,
};

pub mod daily;
pub mod leaderboard;
pub mod puzzle;
pub mod register;
pub mod unregister;

fn extract_string_option(options_list: &[CommandDataOption], option_name: &str) -> Option<String> {
    let option = options_list.iter().find(|opt| opt.name == option_name)?;
    option.resolved.clone().map(|v| match v {
        CommandDataOptionValue::String(s) => s,
        _ => panic!("Expected string option"),
    })
}

fn extract_int_option(options_list: &[CommandDataOption], option_name: &str) -> Option<isize> {
    let option = options_list.iter().find(|opt| opt.name == option_name)?;
    option.resolved.clone().map(|v| match v {
        CommandDataOptionValue::Integer(v) => v as isize,
        _ => panic!("Expected integer option"),
    })
}

fn extract_channel_option(
    options_list: &[CommandDataOption],
    option_name: &str,
) -> Option<PartialChannel> {
    let option = options_list.iter().find(|opt| opt.name == option_name)?;
    option.resolved.clone().map(|v| match v {
        CommandDataOptionValue::Channel(v) => v,
        _ => panic!("Expected channel option"),
    })
}

fn extract_subcommand(options_list: &[CommandDataOption]) -> Option<&CommandDataOption> {
    let option = options_list.iter().find(|opt| {
        opt.kind == CommandOptionType::SubCommand || opt.kind == CommandOptionType::SubCommandGroup
    })?;
    Some(option)
}

trait CommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self;
}

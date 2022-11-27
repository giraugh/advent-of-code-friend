use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub mod leaderboard;
pub mod puzzle;
pub mod register;

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
        _ => panic!("Expected string option"),
    })
}

trait CommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self;
}

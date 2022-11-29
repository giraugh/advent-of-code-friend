use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

use crate::bot::Bot;

use super::{extract_int_option, CommandOptions};

struct PuzzleCommandOptions {
    day: Option<usize>,
    year: Option<usize>,
}

// TODO: To think about: should these options do the defaulting to the current day/year? or should that happen in run()?

impl CommandOptions for PuzzleCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            day: extract_int_option(options_list, "day").map(|v| v as usize),
            year: extract_int_option(options_list, "year").map(|v| v as usize),
        }
    }
}
// Command //

pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    // Parse options
    let options = PuzzleCommandOptions::from_options_list(&command.data.options);

    // Respond
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.content(format!(
                    "pretend this is a puzzle. Day={:?} Year={:?}",
                    options.day, options.year,
                ))
            })
        })
        .await
        .expect("failed to create interaction response");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("puzzle")
        .description("Posts a link to the latest puzzle (or for a day/year you choose).")
        .create_option(|option| {
            option
                .name("day")
                .description("Day of December to fetch puzzle for (defaults to latest day)")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(25)
        })
        .create_option(|option| {
            option
                .name("year")
                .description("Year to fetch puzzle for (defaults to current year)")
                .kind(CommandOptionType::Integer)
                .min_int_value(2015)
        })
}

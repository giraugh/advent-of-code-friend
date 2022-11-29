use chrono::{Datelike, Utc};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

use crate::bot::Bot;
use crate::format::{make_message_embed, make_puzzle_embed, ResponseReason};

use super::{extract_int_option, CommandOptions};

struct PuzzleCommandOptions {
    day: Option<u32>,
    year: i32,
}

impl CommandOptions for PuzzleCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            day: extract_int_option(options_list, "day").map(|v| v as u32),
            year: extract_int_option(options_list, "year")
                .map(|v| v as i32)
                .unwrap_or_else(|| Utc::now().year()),
        }
    }
}
// Command //

pub async fn run(_bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    // Parse options
    let options = PuzzleCommandOptions::from_options_list(&command.data.options);

    // Respond
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                if Utc::now().month() != 12 && options.year == Utc::now().year() {
                    message.ephemeral(true).add_embed(make_message_embed(
                        ResponseReason::Error,
                        &format!(
                            "It's not yet December, please specify a year between 2015 and {}",
                            Utc::now().year() - 1,
                        ),
                    ))
                } else if options.year > Utc::now().year() {
                    message.ephemeral(true).add_embed(make_message_embed(
                        ResponseReason::Error,
                        "You can't use a year in the future 🗞️",
                    ))
                } else if options.day.is_none() && options.year != Utc::now().year() {
                    message.ephemeral(true).add_embed(make_message_embed(
                        ResponseReason::Error,
                        "When using a previous year, you must also specify a day",
                    ))
                } else {
                    message.add_embed(make_puzzle_embed(
                        options.year,
                        options.day.unwrap_or_else(|| Utc::now().day()),
                        false,
                    ))
                }
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

use chrono::{Datelike, FixedOffset, Utc};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

use crate::bot::Bot;
use crate::daily::EST_SECS;
use crate::format::{make_message_embed, make_puzzle_embed, ResponseReason};

use super::{extract_int_option, CommandOptions};

struct PuzzleCommandOptions {
    day: Option<usize>,
    year: usize,
}

impl CommandOptions for PuzzleCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            day: extract_int_option(options_list, "day").map(|v| v as usize),
            year: extract_int_option(options_list, "year")
                .map(|v| v as usize)
                .unwrap_or_else(|| Utc::now().year() as usize),
        }
    }
}
// Command //

pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    // Parse options
    let options = PuzzleCommandOptions::from_options_list(&command.data.options);

    // Get current year
    let tz = FixedOffset::east_opt(EST_SECS).unwrap();
    let time = Utc::now().with_timezone(&tz);
    let year = time.year() as usize;
    let day = time.day() as usize;

    let error = if Utc::now().month() != 12 && options.year == year {
        Some(format!(
            "It's not yet December, please specify a year between 2015 and {}",
            year - 1,
        ))
    } else if options.year > year {
        Some("You can't use a year in the future 🗞️".to_owned())
    } else if options.day.is_none() && options.year != year {
        Some("When using a previous year, you must also specify a day".to_owned())
    } else {
        None
    };

    if let Some(error_str) = error {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| {
                    message
                        .ephemeral(true)
                        .add_embed(make_message_embed(ResponseReason::Error, &error_str))
                })
            })
            .await
            .expect("failed to create interaction response");
    } else {
        // Defer response
        command.defer(&ctx.http).await.unwrap();

        let puzzle_details = {
            let mut aoc_data = bot.aoc_data.lock().await;
            aoc_data
                .get_puzzle_details(options.year, options.day.unwrap_or(day))
                .await
        }
        .ok();

        command
            .create_followup_message(&ctx.http, |message| {
                message.add_embed(make_puzzle_embed(
                    options.year,
                    options.day.unwrap_or(day),
                    puzzle_details,
                    false,
                ))
            })
            .await
            .expect("failed to create interaction response");
    };
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
                .description("Year to fetch puzzle for (default: current year)")
                .kind(CommandOptionType::Integer)
                .min_int_value(2015)
        })
}

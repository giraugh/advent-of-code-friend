use chrono::{Datelike, Utc};
use serenity::all::{
    CommandDataOption, CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage,
};
use serenity::prelude::Context;

use crate::bot::Bot;
use crate::daily_posting::clock::{Clock, SystemClock};
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

pub async fn run(bot: &Bot, ctx: &Context, command: &CommandInteraction) {
    // Parse options
    let options = PuzzleCommandOptions::from_options_list(&command.data.options);

    // Get current year
    let clock = SystemClock {};
    let time = clock.now_etc();
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
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .ephemeral(true)
                        .add_embed(make_message_embed(ResponseReason::Error, &error_str)),
                ),
            )
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
            .create_followup(
                &ctx.http,
                CreateInteractionResponseFollowup::new().embed(make_puzzle_embed(
                    options.year,
                    options.day.unwrap_or(day),
                    puzzle_details,
                    false,
                )),
            )
            .await
            .expect("failed to create interaction response");
    };
}

pub fn command_builder() -> CreateCommand {
    CreateCommand::new("puzzle")
        .description("Posts a link to the latest puzzle (or for a day/year you choose).")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "day",
                "Day of December to fetch puzzle for (defaults to latest day)",
            )
            .min_int_value(1)
            .max_int_value(25),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "year",
                "Year to fetch puzzle for (default: current year)",
            )
            .min_int_value(2015),
        )
}

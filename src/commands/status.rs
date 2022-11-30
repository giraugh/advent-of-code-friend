use std::collections::HashMap;

use crate::bot::Bot;
use crate::config::{Config, DailyLeaderboardConfig, DailyPuzzleConfig};
use crate::format::EMBED_COLOR;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;

trait NotEmptyOr {
    fn not_empty_or(self, or: &str) -> Self;
}

impl NotEmptyOr for String {
    fn not_empty_or(self, or: &str) -> Self {
        if self.is_empty() {
            or.to_string()
        } else {
            self
        }
    }
}

pub async fn run(_bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    let guild_id = command.guild_id.expect("guild id expected");
    // Get data
    let config = Config::get().expect("Failed to load config");

    let guild_config = config.guild_configs.get(&guild_id);

    let daily_leaderboard_configs: HashMap<&ChannelId, &DailyLeaderboardConfig> = config
        .daily_leaderboard_configs
        .iter()
        .filter(|config| config.1.guild_id == guild_id.to_string())
        .collect();

    let daily_puzzle_configs: HashMap<&ChannelId, &DailyPuzzleConfig> = config
        .daily_puzzle_configs
        .iter()
        .filter(|config| config.1.guild_id == guild_id.to_string())
        .collect();

    // Respond
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.ephemeral(true).embed(|embed| {
                    embed
                        .title("📋  Status")
                        .description(if guild_config.is_some() {
                            format!(
                                "✅ This server has a registered leaderboard (`{}`)",
                                guild_config.unwrap().leaderboard_id
                            )
                        } else {
                            String::from("❌ This server does not have a registered leaderboard")
                        })
                        .field(
                            "Daily Leaderboards",
                            daily_leaderboard_configs
                                .iter()
                                .map(|config| {
                                    format!(
                                        "<#{}> at {:0>2}:00",
                                        config.0,
                                        config.1.hour.unwrap_or(0)
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join("\n")
                                .not_empty_or("There are no daily leaderboards set up"),
                            false,
                        )
                        .field(
                            "Daily Puzzles",
                            daily_puzzle_configs
                                .iter()
                                .map(|config| {
                                    format!(
                                        "<#{}> at {:0>2}:00",
                                        config.0,
                                        config.1.hour.unwrap_or(0)
                                    )
                                })
                                .collect::<Vec<String>>()
                                .join("\n")
                                .not_empty_or("There are no daily puzzles set up"),
                            false,
                        )
                        .color(EMBED_COLOR)
                })
            })
        })
        .await
        .expect("to respond to command");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("status")
        .description("Display any registration and dailies set up for this server")
}

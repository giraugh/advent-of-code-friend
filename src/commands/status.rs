use std::collections::HashMap;

use crate::bot::Bot;
use crate::config::{Config, DailyLeaderboardConfig, DailyPuzzleConfig};
use crate::format::EMBED_COLOR;
use serenity::all::{
    CommandInteraction, CreateCommand, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
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

pub async fn run(_bot: &Bot, ctx: &Context, command: &CommandInteraction) {
    let guild_id = command.guild_id.expect("guild id expected");
    // Get data
    let config = Config::get().expect("Failed to load config");

    let guild_config = config.guild_configs.get(&guild_id);

    let daily_leaderboard_configs: HashMap<&ChannelId, &DailyLeaderboardConfig> = config
        .daily_leaderboard_configs
        .iter()
        .filter(|config| config.1.guild_id == guild_id)
        .collect();

    let daily_puzzle_configs: HashMap<&ChannelId, &DailyPuzzleConfig> = config
        .daily_puzzle_configs
        .iter()
        .filter(|config| config.1.guild_id == guild_id)
        .collect();

    // Respond
    let response = CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .embed(
            CreateEmbed::new()
                .title("📋  Status")
                .description(match guild_config {
                    Some(config) => {
                        format!(
                            "✅ This server has a registered leaderboard (`{}`)",
                            config.leaderboard_id
                        )
                    }
                    None => String::from("❌ This server does not have a registered leaderboard"),
                })
                .field(
                    "Daily Leaderboards",
                    daily_leaderboard_configs
                        .iter()
                        .map(|config| format!("<#{}> at {:0>2}:00", config.0, config.1.hour))
                        .collect::<Vec<String>>()
                        .join("\n")
                        .not_empty_or("There are no daily leaderboards set up"),
                    false,
                )
                .field(
                    "Daily Puzzles",
                    daily_puzzle_configs
                        .iter()
                        .map(|config| format!("<#{}> at {:0>2}:00", config.0, config.1.hour))
                        .collect::<Vec<String>>()
                        .join("\n")
                        .not_empty_or("There are no daily puzzles set up"),
                    false,
                )
                .color(EMBED_COLOR),
        );

    command
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
        .expect("to respond to command");
}

pub fn command_builder() -> CreateCommand {
    CreateCommand::new("status")
        .description("Display any registration and dailies set up for this server")
}

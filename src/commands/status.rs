use std::collections::HashMap;

use crate::bot::Bot;
use crate::config::{Config, DailyLeaderboardConfig, DailyPuzzleConfig};
use crate::format::make_status_embed;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;

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
                message.ephemeral(true).add_embed(make_status_embed(
                    guild_config,
                    daily_leaderboard_configs,
                    daily_puzzle_configs,
                ))
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

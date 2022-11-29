use std::sync::Arc;

use crate::aoc::{Leaderboard, LeaderboardCacheEntry};
use crate::bot::config::LeaderboardOrdering;
use crate::bot::Bot;

use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOption,
};
use serenity::model::Timestamp;
use serenity::prelude::Context;

use super::{extract_string_option, CommandOptions};

// Options //

struct LeaderboardCommandOptions {
    ordering: LeaderboardOrdering,
}

impl CommandOptions for LeaderboardCommandOptions {
    fn from_options_list(options_list: &[CommandDataOption]) -> Self {
        Self {
            ordering: extract_string_option(options_list, "ordering")
                .and_then(|ordering| ordering.parse().ok())
                .unwrap_or(LeaderboardOrdering::GlobalScore),
        }
    }
}

// #[async_trait]
// trait DeferEphemeral {
//     async fn defer_ephemeral(&self, http: &Arc<Http>);
// }

// #[async_trait]
// impl DeferEphemeral for ApplicationCommandInteraction {
//     async fn defer_ephemeral(&self, http: &Arc<Http>) {
//         self.create_interaction_response(http, |response| {
//             response.kind(InteractionResponseType::DeferredUpdateMessage)
//         })
//         .await
//         .unwrap();
//     }
// }

// Command //

macro_rules! trunc {
    ($s:expr, $n: expr) => {{
        let mut s = $s.clone();
        if (s.len() > $n) {
            s.truncate($n - 3);
            format!("{}...", s)
        } else {
            s
        }
    }};
}

pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    // Parse command options
    let options = LeaderboardCommandOptions::from_options_list(&command.data.options);

    // Defer response
    command.defer(&ctx.http).await.unwrap();

    // Get leaderboard
    let guild_id = command.guild_id.expect("command to have guild id");
    let leaderboard = bot.get_registered_leaderboard(guild_id).await;

    // Respond
    match leaderboard {
        // If we can get the leaderboard...
        Ok(leaderboard) => {
            // Respond
            command
                .create_followup_message(&ctx.http, |message| {
                    message
                        .embed(|embed| make_leaderboard_embed(embed, leaderboard, options.ordering))
                })
                .await
                .expect("failed to create interaction response");
        }

        // If something went wrong...
        Err(error) => {
            command
                .create_followup_message(&ctx.http, |message| {
                    message.content(format!("Failed to get leaderboard: {}", error))
                })
                .await
                .expect("failed to send error response");
        }
    }
}

fn make_leaderboard_embed(
    create_embed: &mut CreateEmbed,
    leaderboard: Arc<LeaderboardCacheEntry>,
    ordering: LeaderboardOrdering,
) -> &mut CreateEmbed {
    create_embed
        .title("Leaderboard")
        .description(leaderboard_embed_content(
            &leaderboard.leaderboard,
            ordering,
        ))
        .timestamp(leaderboard.created_at.to_rfc3339())
        .url(format!(
            "https://adventofcode.com/{}/leaderboard/private/view/{}",
            leaderboard.leaderboard.event, leaderboard.leaderboard_id
        ))
}

fn leaderboard_embed_content(leaderboard: &Leaderboard, ordering: LeaderboardOrdering) -> String {
    // Collect member entries
    let mut members: Vec<_> = leaderboard
        .members
        .iter()
        .map(|(_, member)| member)
        .collect();

    // Get longest name
    let longest_name_len = members
        .iter()
        .map(|member| member.name.len())
        .max()
        .unwrap_or(20)
        .min(20);

    // Sort them
    members.sort_by(|a, b| match ordering {
        LeaderboardOrdering::LocalScore => a.local_score.cmp(&b.local_score),
        LeaderboardOrdering::GlobalScore => a.global_score.cmp(&b.global_score),
        LeaderboardOrdering::Stars => match a.stars.cmp(&b.stars) {
            std::cmp::Ordering::Equal => a.last_star_ts.cmp(&b.last_star_ts),
            x => x,
        },
    });

    let content: String = members
        .iter()
        .enumerate()
        .map(|(i, member)| {
            format!(
                "{}: {} {} ⭐️\n",
                format_args!(
                    "{:0>width$}",
                    i + 1,
                    width = match members.len() {
                        x if x < 10 => 1,
                        x if x < 100 => 2,
                        _ => 3,
                    }
                ),
                format_args!(
                    "{:width$}",
                    trunc!(member.name, 20),
                    width = longest_name_len
                ),
                format_args!("{:0>2}", member.local_score),
            )
        })
        .collect();

    format!("```js\n{}```", content)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("leaderboard")
        .description("Uses the registered leaderboard ID to fetch and post the leaderboard.")
        .create_option(|option| {
            option
                .name("ordering")
                .description(
                    "Method used to order people in the leaderboard (default is local-score)",
                )
                .kind(CommandOptionType::String)
                .add_string_choice("local-score", LeaderboardOrdering::LocalScore)
                .add_string_choice("global-score", LeaderboardOrdering::GlobalScore)
                .add_string_choice("stars", LeaderboardOrdering::Stars)
        })
}

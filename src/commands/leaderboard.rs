use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOptionValue,
};
use strum::{Display, EnumString};

#[derive(Debug, PartialEq, PartialOrd, EnumString, Display)]
enum LeaderboardOrdering {
    LocalScore,
    GlobalScore,
    Stars,
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    // Parse options
    let mut ordering: LeaderboardOrdering = LeaderboardOrdering::LocalScore;
    for option in &command.data.options {
        match option.name.as_str() {
            "ordering" => {
                if let Some(CommandDataOptionValue::String(value)) = &option.resolved {
                    ordering = value.parse().unwrap()
                }
            }
            name => panic!("Unknown option {:?}", name),
        }
    }

    // Respond
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.content(format!(
                    "imagine this is a leaderboard. ordering: {:?}",
                    ordering
                ))
            })
        })
        .await
        .expect("to respond to command");
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

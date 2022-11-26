use advent_of_code_friend::{Config, GuildConfig};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOptionValue,
};

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    // Parse options
    let mut session_token = None;
    let mut leaderboard_id = None;
    for option in &command.data.options {
        match option.name.as_str() {
            "session_token" => {
                if let Some(CommandDataOptionValue::String(value)) = &option.resolved {
                    println!("session_token: {}", value);
                    session_token = Some(value);
                }
            }
            "leaderboard_id" => {
                if let Some(CommandDataOptionValue::String(value)) = &option.resolved {
                    println!("leaderboard_id: {}", value);
                    leaderboard_id = Some(value);
                }
            }
            name => panic!("Unknown option {:?}", name),
        }
    }

    // Check stuff
    if session_token == None || leaderboard_id == None {
        panic!("aaa")
    }

    // Save data
    let mut config = Config::get().expect("Failed to load config");
    config.guild_configs.insert(
        command.guild_id.expect("Expected guild ID"),
        GuildConfig {
            session_token: session_token.unwrap().to_string(),
            leaderboard_id: leaderboard_id.unwrap().to_string(),
        },
    );

    // Respond
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| message.content("ok :+1:"))
        })
        .await
        .expect("to respond to command");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("register")
        .description("Bind a session token and leaderboard ID to this server")
        .create_option(|option| {
            option
                .name("session_token")
                .description("Your AoC session cookie (valid for approx. 30 days)")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("leaderboard_id")
                .description("Can be found at the end of a private leaderboard page URL")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

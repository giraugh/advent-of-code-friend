mod commands;

use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{gateway::Ready, id::GuildId, prelude::interaction::Interaction},
    prelude::*,
    Client,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // If this interaction is a command
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "leaderboard" => commands::leaderboard::run(&ctx, &command).await,
                "puzzle" => commands::puzzle::run(&ctx, &command).await,
                _ => {}
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot is connected as {}", ready.user.name);

        // For now, we will register local commands, to do so get the guild id
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        // Setup guild commands
        GuildId::set_application_commands(&guild_id, &ctx.http, |commmands| {
            commmands
                .create_application_command(commands::leaderboard::register)
                .create_application_command(commands::puzzle::register)
        })
        .await
        .expect("to have created guild commands");
    }
}

#[tokio::main]
async fn main() {
    // Load env from .env
    dotenv().ok();

    // Get the token from the env
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build client
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("to create client");

    // Attempt to start client
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

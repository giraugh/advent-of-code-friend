use serenity::{
    async_trait,
    model::{gateway::Ready, id::GuildId, prelude::interaction::Interaction},
    prelude::*,
};
use std::{env, sync::Arc};

use crate::{
    aoc::{AOCData, LeaderboardCacheEntry},
    commands,
    config::Config,
    daily,
};

pub struct Bot {
    pub aoc_data: Arc<Mutex<AOCData>>,
}

impl Bot {
    pub async fn start(token: String) {
        // Create bot data

        // Create client
        let mut client = Client::builder(token, GatewayIntents::empty())
            .event_handler(Bot {
                aoc_data: Arc::new(Mutex::new(AOCData::new())),
            })
            .await
            .expect("to create client");

        // Attempt to start client
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }

    pub async fn get_registered_leaderboard(
        &self,
        guild_id: GuildId,
        year: i32,
    ) -> Result<Arc<LeaderboardCacheEntry>, String> {
        // Get config for guild
        let config = Config::get().expect("Failed to load config");
        let guild_config = config
            .guild_configs
            .get(&guild_id)
            .ok_or_else(|| "server has no registered leaderboard".to_owned())?;

        // Get leaderboard
        let leaderboard = {
            let mut aoc_data = self.aoc_data.lock().await;
            aoc_data
                .get_leaderboard(
                    &year.to_string(),
                    &guild_config.leaderboard_id,
                    &guild_config.session_token,
                )
                .await
        };
        leaderboard.map_err(|e| e.to_string())
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // If this interaction is a command
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "register" => commands::register::run(self, &ctx, &command).await,
                "unregister" => commands::unregister::run(self, &ctx, &command).await,
                "leaderboard" => commands::leaderboard::run(self, &ctx, &command).await,
                "puzzle" => commands::puzzle::run(self, &ctx, &command).await,
                "daily" => commands::daily::run(self, &ctx, &command).await,
                "status" => commands::status::run(self, &ctx, &command).await,
                "help" => commands::help::run(self, &ctx, &command).await,
                _ => {}
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!(
            "Bot is connected as {}#{}",
            ready.user.name,
            ready.user.discriminator
        );

        // Start daily posting thread
        let daily_thread = tokio::spawn(daily::daily_posts(self.aoc_data.clone(), ctx.clone()));

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
                .create_application_command(commands::register::register)
                .create_application_command(commands::unregister::register)
                .create_application_command(commands::leaderboard::register)
                .create_application_command(commands::puzzle::register)
                .create_application_command(commands::daily::register)
                .create_application_command(commands::status::register)
                .create_application_command(commands::help::register)
        })
        .await
        .expect("to have created guild commands");

        // Wait for threads
        daily_thread.await.unwrap();
    }
}

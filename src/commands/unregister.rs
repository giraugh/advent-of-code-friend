use crate::bot::Bot;
use crate::config::Config;
use crate::format::{make_message_embed, ResponseReason};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(_bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    // Save data
    let mut config = Config::get().expect("Failed to load config");
    let removed_guild_id = config
        .guild_configs
        .remove(&command.guild_id.expect("Expected guild ID"));

    // Respond
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                if removed_guild_id.is_some() {
                    // TODO: Clear dailies here maybe?
                    message.ephemeral(true).embed(|e| {
                        make_message_embed(
                            e,
                            ResponseReason::Success,
                            "Successfully unregistered this server.",
                        )
                    })
                } else {
                    message.ephemeral(true).embed(|e| {
                        make_message_embed(
                            e,
                            ResponseReason::Error,
                            "There was no registration associated with this server. You can set one up with `/register`.",
                        )
                    })
                }
            })
        })
        .await
        .expect("to respond to command");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("unregister")
        .description("Remove a registered leaderboard and session token from this server")
}

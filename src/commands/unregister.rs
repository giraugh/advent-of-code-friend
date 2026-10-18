use crate::bot::Bot;
use crate::config::Config;
use crate::format::{make_message_embed, ResponseReason};
use serenity::all::{
    CommandInteraction, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::prelude::Context;

pub async fn run(_bot: &Bot, ctx: &Context, command: &CommandInteraction) {
    // Save data
    let mut config = Config::get().expect("Failed to load config");
    let removed_guild_config = config
        .guild_configs
        .remove(&command.guild_id.expect("Expected guild ID"));

    // Message differs by success state
    let response_message = CreateInteractionResponseMessage::new()
        .add_embed(if removed_guild_config.is_some() {
            make_message_embed(ResponseReason::Success, "Successfully unregistered this server." )
        } else {
            make_message_embed(ResponseReason::Error, "There was no registration associated with this server. You can set one up with /register")
        })
        .ephemeral(true);

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(response_message),
        )
        .await
        .expect("to respond to command");
}

pub fn command_builder() -> CreateCommand {
    CreateCommand::new("unregister")
        .description("Remove a registered leaderboard and session token from this server")
}

use crate::bot::Bot;
use crate::format::EMBED_COLOR;
use serenity::all::{
    CommandInteraction, CreateCommand, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::prelude::Context;

pub async fn run(_bot: &Bot, ctx: &Context, command: &CommandInteraction) {
    // Respond
    command
        .create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().ephemeral(true).embed(CreateEmbed::new()
            .title("🛟  Help")
        .description(
            "This is a Discord bot for posting leaderboards and puzzles from [Advent of Code](https://adventofcode.com/).\n\n\
            To get started, use the `/register` command in a server to set up your session token and leaderboard id. You can \
            find your leaderboard id by visiting the private leaderboard you want to use and checking the number at the end of the url.\n\n\
            For example, if your leaderboard is at the following url: `https://adventofcode.com/2022/leaderboard/private/view/1234567`, \
            then your leaderboard id is `1234567`.\n\n\
            To get the session token, you'll need to check the network tab of developer tools while visiting the AoC website to find the \
            `session` cookie in the request headers.\n\n\
            For more detailed info, including descriptions on all the commands, check out the [Github repository](https://github.com/giraugh/advent-of-code-friend#readme)."
        )
        .color(EMBED_COLOR))))
        .await
        .expect("to respond to command");
}

pub fn command_builder() -> CreateCommand {
    CreateCommand::new("help").description("Show setup information for this bot")
}

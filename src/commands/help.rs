use crate::bot::Bot;
use crate::format::EMBED_COLOR;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(_bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    // Respond
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|message| {
                message.ephemeral(true).embed(|embed| {
                    embed
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
                        .color(EMBED_COLOR)
                })
            })
        })
        .await
        .expect("to respond to command");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("help")
        .description("Show setup information for this bot")
}

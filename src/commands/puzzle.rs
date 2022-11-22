use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .interaction_response_data(|message| message.content("pretend this is a puzzle"))
        })
        .await
        .expect("to respond to command");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("puzzle")
        .description("Posts a link to the latest puzzle (or for a day/year you choose).")
        .create_option(|option| {
            option
                .name("day")
                .description("Day of December to fetch puzzle for (defaults to latest day)")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(25)
        })
        .create_option(|option| {
            option
                .name("year")
                .description("Year to fetch puzzle for (defaults to current year)")
                .kind(CommandOptionType::Integer)
                .min_int_value(2015)
        })
}

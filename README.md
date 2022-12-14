# Advent of Code Friend

Discord bot for Advent of Code leaderboards 🎖

Please note that because Advent of Code releases puzzles at midnight EST, this bot and all times it uses are also EST.

## Commands

### `/register <session_token> <leaderboard_id>`

Sets up the bot to use this leaderboard in the server you run the command in. All `leaderboard` commands in this server will use the token and leaderboard provided to fetch the API.

### `/unregister`

Clears the session token and leaderboard ID being used by this server.

### `/leaderboard [ordering] [year]`

Uses the registered leaderboard ID to fetch the leaderboard and post it. You can specify a custom ordering method, and optionally a year, otherwise it will attempt to fetch from the current year.

### `/puzzle [day] [year]`

Posts a link to the latest puzzle (or for a day/year you choose). Note that you can't specify a year without also specifying a day.

### `/daily leaderboard <channel> [hour] [ordering]`

Register the bot to send the leaderboard into a channel you specify at a specific time every day (of December). By default it will send at midnight EST. You can also change the ordering used by the leaderboard that's sent. This will always send the leaderboard for the current year.

Note: Registering the same channel twice will override the previous registration.

### `/daily puzzle <channel> [hour]`

Register the bot to send the latest puzzle into a channel you specify at a specific time every day (of December). By default it will send at midnight EST. Thsi will always send puzzles from the current year.

Note: Registering the same channel twice will override the previous registration.

### `/daily unregister leaderboard <channel>`

Clear any leaderboard registration bound to this channel.

### `/daily unregister puzzle <channel>`

Clear any puzzle registration bound to this channel.

### `/status`

Displays the current registration and dailies set up in the server (if any).

### `/help`

Get info on how to set up the bot.

## Contributing

### Issues/Feature Requests

If you find any issues with the bot, or you have a cool feature request, you can [create an issue](https://github.com/giraugh/advent-of-code-friend/issues/new/choose) on this repository.

### Development

AOC Friend is written in [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html), you can use Cargo to build and run the bot.
If you're looking for something to contribute to, consider checking out issues tagged with [good first issue](https://github.com/giraugh/advent-of-code-friend/labels/good%20first%20issue).

Make sure you create a `.env` file with your Discord bot token and run `cargo run` to start the bot. (or use `cargo watch -x run` if you have [cargo-watch](https://crates.io/crates/cargo-watch) installed to restart when files change)
You can then compile an executable with `cargo build`.

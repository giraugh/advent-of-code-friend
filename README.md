# Advent of Code Friend

Discord bot for advent of code leaderboards 🎖

## Commands

### `/register <session_token> <leaderboard_id>`

Sets up the bot to use this leaderboard in the server you run the command in. All `leaderboard` commands in this server will use the token and leaderboard provided to fetch the API.

### `/unregister`

Clears the session token and leaderboard ID being used by this server.

### `/leaderboard [ordering]`

Uses the registered leaderboard ID to fetch the leaderboard and post it. You can specify a custom ordering method.

### `/puzzle [day] [year]`

Posts a link to the latest puzzle (or for a day/year you choose).

### `/daily leaderboard <channel> [time] [ordering]`

Register the bot to send the leaderboard into a channel you specify at a specific time every day (of December). By default it will send at midnight EST. You can also change the ordering used by the leaderboard that's sent.

Note: Registering the same channel twice will override the previous registration.

### `/daily leaderboard unregister <channel>`

Clear any leaderboard registration bound to this channel.

### `/daily puzzle <channel> [time]`

Register the bot to send the latest puzzle into a channel you specify at a specific time every day (of December). By default it will send at midnight EST.

### `/daily puzzle unregister <channel>`

Clear any puzzle registration bound to this channel.

## Contributing

### Issues/Feature Requests

If you find any issues with the bot, or you have a cool feature request, you can [create an issue](https://github.com/giraugh/advent-of-code-friend/issues/new/choose) on this repository.

### Development

AOC Friend is written in [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html), you can use Cargo to build and run the bot.
If you're looking for something to contribute to, consider checking out issues tagged with [good first issue](https://github.com/giraugh/advent-of-code-friend/labels/good%20first%20issue).

Make sure you create a `.env` file with your Discord bot token and run `cargo watch` to start the bot in watch mode (it will restart the bot when you edit files).
You can then compile an executable with `cargo build`.

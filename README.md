# Rust Discord Bot

A Discord bot written in Rust using the [serenity](https://github.com/serenity-rs/serenity) crate. The project includes a small command set for utility, dice rolling, cleanup, and number-theory helpers.

## Features

- Slash commands for Discord interactions
- Docker-based run configuration
- Simple bot startup with token and guild ID from environment variables
- Number-factorization helper command for educational math work

## Commands

| Command | Description |
| --- | --- |
| `/ping` | Replies with a friendly message. |
| `/echo text:<text>` | Repeats the text you provide. |
| `/roll dice:<MdN>` | Rolls dice in the form `2d6`, `d20`, etc. |
| `/cleanup amount:<1-500>` | Deletes the bot's recent messages in the channel after confirmation. |
| `/factor n:<integer>` | Finds the prime factors of an integer greater than or equal to 2. |

## Requirements

- [Docker](https://www.docker.com/) (recommended)
- [Rust](https://www.rust-lang.org/tools/install) and Cargo (for local development)
- A Discord bot token
- Your Discord server (guild) ID

## Discord setup

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications).
2. Create an application and open the Bot section.
3. Copy your bot token.
4. Under OAuth2 > URL Generator, select:
   - Scope: `bot`, `applications.commands`
   - Permissions: `View Channels`, `Send Messages`, `Read Message History`, and any other permission you want the bot to use.
5. Use the generated invite URL to add the bot to your server.
6. Enable Developer Mode in Discord, then right-click your server and copy the server ID.

## Configuration

Create a `.env` file in the project root:

```env
DISCORD_TOKEN=your_bot_token
GUILD_ID=your_server_id
```

Do not commit `.env` to version control.

## Run with Docker

```bash
git clone https://github.com/Wai-Theetat/Rust-Practice_discord-bot.git
cd Rust-Practice_discord-bot
docker compose up -d --build
```

Check the logs:

```bash
docker compose logs -f bot
```

Expected output includes a message like:

```text
<Bot Name> is connected!
Registered N commands
```

Then press Ctrl+R in Discord and type `/` to see the bot commands.

### Useful Docker commands

```bash
docker compose logs -f bot
docker compose down
docker compose restart bot
docker compose ps
```

## Run locally (development)

On PowerShell:

```powershell
$env:DISCORD_TOKEN="your_bot_token"
$env:GUILD_ID="your_server_id"
cargo run
```

On Bash/Zsh:

```bash
export DISCORD_TOKEN=your_bot_token
export GUILD_ID=your_server_id
cargo run
```

You can also validate the project with:

```bash
cargo check
cargo test
```

## Project structure

```text
.
├── Dockerfile
├── docker-compose.yml
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── handler.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── ping.rs
│   │   ├── echo.rs
│   │   ├── roll.rs
│   │   ├── cleanup.rs
│   │   └── factor.rs
│   └── math/
│       ├── mod.rs
│       ├── gcd.rs
│       ├── modular_arithmetic.rs
│       ├── miller_rabin.rs
│       ├── pollard_rho.rs
│       ├── factorization.rs
│       └── formatting.rs
└── .gitignore
```

## Notes

- The bot registers slash commands on startup via `GuildId::set_commands`.
- Commands that need async work or button interactions (such as `/cleanup` and `/factor`) are handled in a dedicated path.
- The `factor` command uses a math module for prime factorization and formatting output.

## Troubleshooting

| Problem | Fix |
| --- | --- |
| `Token Not Found!` | Make sure `.env` exists and contains `DISCORD_TOKEN`. |
| `GUILD_ID Not Found!` | Add `GUILD_ID` to `.env` and restart the bot. |
| `Invalid authentication` | Check whether the bot token is valid. |
| Commands not showing | Refresh Discord (`Ctrl+R`) and confirm the Guild ID is correct. |
| `/cleanup` does nothing | Ensure the bot has permission to read message history and view the channel. |
| Docker build issues | Rebuild with `docker compose up -d --build` after changes. |

## Security

- Keep your Discord token private.
- Never commit `.env` or any secrets into Git.
- If a token is exposed, regenerate it in the Discord Developer Portal.

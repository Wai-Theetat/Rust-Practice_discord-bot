# rust-discord-bot

A small Discord bot written in Rust with [serenity](https://github.com/serenity-rs/serenity). Runs in Docker.

## Commands

| Command | What it does |
|---|---|
| `/ping` | Replies with a message. |
| `/echo text:<text>` | Repeats your text. |
| `/roll dice:<MdN>` | Rolls dice. Example: `2d6`, `d20`. Max 100 dice, 2 to 20 faces. |
| `/cleanup amount:<1-500>` | Deletes the bot's own recent messages in the channel, after a confirm button. Needs Manage Messages to see the command. Other users' and bots' messages are never touched. |

## Requirements

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) (Linux containers; WSL2 backend on Windows)
- A Discord bot token and your server (guild) ID
- Optional, for development: Rust (stable, recent enough for your edition)

## Discord setup (one time)

1. Open the [Developer Portal](https://discord.com/developers/applications), create an application, open the **Bot** tab, and copy the token.
2. **OAuth2 > URL Generator**: scopes `bot` and `applications.commands`. Permissions: View Channels, Send Messages, Read Message History.
3. Open the generated URL and add the bot to your server.
4. Get your server ID: Discord **Settings > Advanced > Developer Mode** on, then right-click the server and **Copy Server ID**.

## Run with Docker

```
git clone <your-repo-url>
cd rust-discord-bot
```

Copy `.env.example` to `.env` and fill it in:

```
DISCORD_TOKEN=your_token
GUILD_ID=your_server_id
```

No quotes, no spaces around `=`. Never commit `.env`.

Build and start:

```
docker compose up -d --build
```

Check logs:

```
docker compose logs -f bot
```

Expected output: `<bot name> is connected!` and `Registered N commands`.
In Discord press Ctrl+R, then type `/` to see the commands.

### Daily commands

| Action | Command |
|---|---|
| Logs | `docker compose logs -f bot` |
| Stop | `docker compose down` |
| Restart | `docker compose restart bot` |
| After code change | `docker compose up -d --build` |
| After `.env` change | `docker compose up -d` |
| Status | `docker compose ps` |

## Run without Docker (development)

PowerShell:

```
$env:DISCORD_TOKEN="your_token"
$env:GUILD_ID="your_server_id"
cargo run
```

Bash:

```
export DISCORD_TOKEN=your_token
export GUILD_ID=your_server_id
cargo run
```

Do not run this and Docker at the same time. Two bots with the same token both reply.

Checks:

```
cargo check
cargo test
```

## Project layout

```
src/
  main.rs            start the client
  handler.rs         event handler, routes interactions
  commands/
    mod.rs           command list and router
    ping.rs
    echo.rs
    roll.rs
    cleanup.rs       async command with confirm buttons
```

### Add a command

1. Create `src/commands/<name>.rs` with `register()` and `run()`.
2. In `commands/mod.rs`: add `pub mod <name>;`, add `<name>::register()` to `all()`, add an arm to `run()`.
3. Rebuild: `docker compose up -d --build`.

Commands that need `.await` or buttons (like `cleanup`) are routed directly in `handler.rs`.

## Troubleshooting

| Problem | Fix |
|---|---|
| `Token Not Found!` | `.env` missing, or saved as `.env.txt`. |
| `GUILD_ID Not Found!` | Add `GUILD_ID` to `.env`, then `docker compose up -d`. |
| `Invalid authentication` | Wrong or reset token. Update `.env`. |
| `Missing Access` when registering | Re-invite the bot with the `applications.commands` scope. |
| Commands not showing | Press Ctrl+R in Discord. Check `GUILD_ID`. |
| Docker build: binary `not found` | `BIN_NAME` in the Dockerfile must match `name` in `Cargo.toml` exactly (case sensitive). |
| Docker build killed (`SIGKILL`, `137`) | Out of memory. Raise WSL2 memory in `%UserProfile%\.wslconfig`. |
| `/cleanup` deletes nothing | Bot needs View Channel and Read Message History in that channel. |

## Security

- `.env` holds the token. It is in `.gitignore` and `.dockerignore`. Never commit it.
- Token leaked? Reset it in the Developer Portal, update `.env`, run `docker compose up -d`.

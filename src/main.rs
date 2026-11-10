mod commands;
mod handler;
mod math;

use handler::Handler;
use serenity::model::id::GuildId;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
	let token = std::env::var("DISCORD_TOKEN").expect("Token Not Found!");

	let intents = GatewayIntents::non_privileged();

	let guild_id = std::env::var("GUILD_ID")
	.expect("GUILD_ID Not Found!")
	.parse::<u64>()
	.expect("GUILD_ID must be a number");

	let guild_id = GuildId::new(guild_id);

	let mut client = Client::builder(&token, intents)
		.event_handler(Handler { guild_id })
		.await
		.expect("Client Error.");

	if let Err(msg) = client.start().await {
		println!("Error: {}", msg)
	}
}
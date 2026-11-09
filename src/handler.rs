use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use crate::commands;

pub struct Handler {
    pub guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        match self
            .guild_id
            .set_commands(&ctx.http, commands::all())
            .await
        {
            Ok(list) => println!("Registered {} commands", list.len()),
            Err(err) => eprintln!("Could not register commands: {err}"),
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                // Async command with buttons: handled on its own path.
                if command.data.name == "cleanup" {
                    commands::cleanup::start(&ctx, &command).await;
                    return;
                }

                let Some(reply) = commands::run(&command) else {
                    eprintln!("Unknown command: {}", command.data.name);
                    return;
                };

                let response = CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content(reply),
                );

                if let Err(error) = command.create_response(&ctx.http, response).await {
                    eprintln!("Could not reply to /{}: {error}", command.data.name);
                }
            }

            Interaction::Component(component) => {
                if commands::cleanup::owns_button(&component.data.custom_id) {
                    commands::cleanup::handle_button(&ctx, &component).await;
                }
            }

            _ => {}
        }
    }
}
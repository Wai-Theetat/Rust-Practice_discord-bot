
use serenity::builder::{
    CreateCommand,
    CreateCommandOption,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
    EditInteractionResponse,
};

use serenity::model::application::{
    CommandInteraction,
    CommandOptionType,
};

use serenity::prelude::*;

use crate::math::factorization::factorize;
use crate::math::formatting::format_factorization;

pub fn register() -> CreateCommand {
    CreateCommand::new("factor")
        .description("Find the prime factors of an integer.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "n",
                "The integer to factorize",
            )
            .required(true)
            .min_int_value(2)
            .max_int_value(9_007_199_254_740_991),
        )
}

pub async fn start(ctx: &Context, command: &CommandInteraction) {
    let n = command
        .data
        .options
        .iter()
        .find(|option| option.name == "n")
        .and_then(|option| option.value.as_i64())
        .and_then(|n| u64::try_from(n).ok())
        .filter(|&n| n >= 2);

    let Some(n) = n else {
        let _ = command.create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("Please provide an integer greater than or equal to 2.")
                    .ephemeral(true),
            ),
        ).await;

        return;
    };

    // Defer the response while factorization runs.
    if let Err(error) = command.defer(&ctx.http).await {
        eprintln!("Failed to defer factor command: {error}");
        return;
    }

    // Run CPU-intensive calculation in a blocking thread.
    let result = tokio::task::spawn_blocking(move || factorize(n)).await;

    let response = match result {
        Ok(Ok(factors)) => format_factorization(n, &factors),
        Ok(Err(error)) => format!("Factorization failed: {error}"),
        Err(error) => {
            eprintln!("Factorization task failed: {error}");
            "Factorization task failed unexpectedly.".to_string()
        }
    };

    if let Err(error) = command.edit_response(
        &ctx.http,
        EditInteractionResponse::new().content(response),
    ).await {
        eprintln!("Failed to edit factor response: {error}");
    }
}

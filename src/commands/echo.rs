use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandInteraction, CommandOptionType};

pub fn register() -> CreateCommand {
    CreateCommand::new("echo")
        .description("Repeat your text.")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "text", "What to repeat")
                .required(true),
        )
}

pub fn run(command: &CommandInteraction) -> String {
    let text = command
        .data
        .options
        .iter()
        .find(|option| option.name == "text")
        .and_then(|option| option.value.as_str());

    match text {
        Some(text) => text.to_string(),
        None => "I don't know what you say~".to_string(),
    }
}
use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Reply with a message.")
}

pub fn run(_command: &CommandInteraction) -> String {
    "Popi po po pipo~".to_string()
}
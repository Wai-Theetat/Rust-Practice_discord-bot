use serenity::builder::CreateCommand;
use serenity::model::application::CommandInteraction;

pub mod cleanup;
pub mod echo;
pub mod ping;
pub mod roll;
pub mod factor;

/// Every command definition sent to Discord on startup.
pub fn all() -> Vec<CreateCommand> {
    vec![
        ping::register(),
        echo::register(),
        roll::register(),
        cleanup::register(),
		factor::register(),
    ]
}

pub fn run(command: &CommandInteraction) -> Option<String> {
    match command.data.name.as_str() {
        "ping" => Some(ping::run(command)),
        "echo" => Some(echo::run(command)),
        "roll" => Some(roll::run(command)),

        _ => None,
    }
}
use rand::RngExt;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandInteraction, CommandOptionType};

const MAX_DICE: u32 = 100;
const MAX_FACES: u32 = 20;

pub fn register() -> CreateCommand {
	CreateCommand::new("roll")
		.description("Roll dice, like 2d6.")
		.add_option(
			CreateCommandOption::new(CommandOptionType::String, "dice", "Format MdN, like 2d6")
				.required(true),
		)
}

/// Parse "MdN" into (dice count, face count). "d20" means 1d20.
fn parse_dice(input: &str) -> Result<(u32, u32), String> {
	let input = input.trim().to_lowercase();

	let Some((count, faces)) = input.split_once('d') else {
		return Err("Use format MdN, like 2d6.".to_string());
	};

	let count = if count.is_empty() {
		1
	} else {
		count
			.parse::<u32>()
			.map_err(|_| "Dice count must be a number.".to_string())?
	};

	let faces = faces
		.parse::<u32>()
		.map_err(|_| "Face count must be a number.".to_string())?;

	if !(1..=MAX_DICE).contains(&count) {
		return Err(format!("Dice count must be 1 to {MAX_DICE}."));
	}
	if !(2..=MAX_FACES).contains(&faces) {
		return Err(format!("Faces must be 2 to {MAX_FACES}."));
	}

	Ok((count, faces))
}

pub fn run(command: &CommandInteraction) -> String {
	let input = command
		.data
		.options
		.iter()
		.find(|option| option.name == "dice")
		.and_then(|option| option.value.as_str());

	let Some(input) = input else {
		return "Tell me what to roll, like 2d6.".to_string();
	};

	let (count, faces) = match parse_dice(input) {
		Ok(values) => values,
		Err(message) => return message,
	};

	let mut rng = rand::rng();
	let rolls: Vec<u32> = (0..count).map(|_| rng.random_range(1..=faces)).collect();

	let total: u32 = rolls.iter().sum();
	let list = rolls
		.iter()
		.map(|roll| roll.to_string())
		.collect::<Vec<_>>()
		.join(" + ");

		format!("{count}d{faces}: {list} = {total}")
}
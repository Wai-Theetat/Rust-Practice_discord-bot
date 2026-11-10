use serenity::builder::{
	CreateActionRow, CreateButton, CreateCommand, CreateCommandOption, CreateInteractionResponse,
	CreateInteractionResponseMessage, EditInteractionResponse, GetMessages,
};
use serenity::model::application::{
	ButtonStyle, CommandInteraction, CommandOptionType, ComponentInteraction,
};
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::permissions::Permissions;
use serenity::prelude::*;

const MAX_SCAN: u32 = 500;
const ID_PREFIX: &str = "cleanup:";

// ---------- registration ----------

pub fn register() -> CreateCommand {
	CreateCommand::new("cleanup")
		.description("Delete my own recent messages in this channel.")
		// Hidden from users without Manage Messages. Admins can change this in settings.
		.default_member_permissions(Permissions::MANAGE_MESSAGES)
		.add_option(
			CreateCommandOption::new(
				CommandOptionType::Integer,
				"amount",
				"How many recent messages to scan (1-500)",
			)
			.min_int_value(1)
			.max_int_value(500)
			.required(true),
		)
}

// ---------- button id encoding ----------
#[derive(Debug, PartialEq)]
enum ButtonAction {
	Confirm { amount: u32, user: u64 },
	Cancel { user: u64 },
}

fn parse_custom_id(id: &str) -> Option<ButtonAction> {
	let rest = id.strip_prefix(ID_PREFIX)?;
	let mut parts = rest.split(':');

	match parts.next()? {
		"confirm" => {
			let amount = parts.next()?.parse().ok()?;
			let user = parts.next()?.parse().ok()?;
			Some(ButtonAction::Confirm { amount, user })
		}
		"cancel" => {
			let user = parts.next()?.parse().ok()?;
			Some(ButtonAction::Cancel { user })
		}
		_ => None,
	}
}

/// True if this button belongs to the cleanup command.
pub fn owns_button(custom_id: &str) -> bool {
	custom_id.starts_with(ID_PREFIX)
}

// ---------- 1: slash command, show confirm buttons ----------
pub async fn start(ctx: &Context, command: &CommandInteraction) {
	let amount = command
		.data
		.options
		.iter()
		.find(|option| option.name == "amount")
		.and_then(|option| option.value.as_i64())
		.and_then(|n| u32::try_from(n).ok())
		.filter(|n| (1..=MAX_SCAN).contains(n));

	let Some(amount) = amount else {
		send_command_reply(ctx, command, format!("Amount must be 1 to {MAX_SCAN}."), None).await;
		return;
	};

	let user = command.user.id.get();
	let confirm = CreateButton::new(format!("{ID_PREFIX}confirm:{amount}:{user}"))
		.label("Delete")
		.style(ButtonStyle::Danger);
	let cancel = CreateButton::new(format!("{ID_PREFIX}cancel:{user}"))
		.label("Cancel")
		.style(ButtonStyle::Secondary);
	let row = CreateActionRow::Buttons(vec![confirm, cancel]);

	let text = format!(
		"Delete **my** messages among the last {amount} in this channel? \
		Messages from users and other bots stay."
	);
	send_command_reply(ctx, command, text, Some(row)).await;
}

async fn send_command_reply(
	ctx: &Context,
	command: &CommandInteraction,
	text: String,
	row: Option<CreateActionRow>,
) {
	let mut message = CreateInteractionResponseMessage::new()
		.content(text)
		.ephemeral(true);
	if let Some(row) = row {
		message = message.components(vec![row]);
	}

	if let Err(error) = command
		.create_response(&ctx.http, CreateInteractionResponse::Message(message))
		.await
	{
		eprintln!("Could not reply to /cleanup: {error}");
	}
}

// ---------- 2: button pressed ----------

pub async fn handle_button(ctx: &Context, component: &ComponentInteraction) {
	let Some(action) = parse_custom_id(&component.data.custom_id) else {
		return;
	};

	let (ButtonAction::Confirm { user, .. } | ButtonAction::Cancel { user }) = &action;

	// Only the person who ran the command is able to press the button.
	if component.user.id.get() != *user {
		let message = CreateInteractionResponseMessage::new()
			.content("This button is not for you.")
			.ephemeral(true);
		if let Err(error) = component
			.create_response(&ctx.http, CreateInteractionResponse::Message(message))
			.await
		{
			eprintln!("Could not reply to button: {error}");
		}
		return;
	}

	match action {
		ButtonAction::Cancel { .. } => {
			update_message(ctx, component, "Cancelled.").await;
		}
		ButtonAction::Confirm { amount, .. } => {
			update_message(ctx, component, "Deleting...").await;

			let summary = delete_own_messages(ctx, component.channel_id, amount).await;

			if let Err(error) = component
				.edit_response(&ctx.http, EditInteractionResponse::new().content(summary))
				.await
			{
				eprintln!("Could not edit final reply: {error}");
			}
		}
	}
}

/// Replace the message text and remove the buttons.
async fn update_message(ctx: &Context, component: &ComponentInteraction, text: &str) {
	let message = CreateInteractionResponseMessage::new()
		.content(text)
		.components(vec![]);

	if let Err(error) = component
		.create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(message))
		.await
	{
		eprintln!("Could not update message: {error}");
	}
}

// ---------- core function  ----------

async fn delete_own_messages(ctx: &Context, channel_id: ChannelId, amount: u32) -> String {
	
	let bot_id = match ctx.http.get_current_user().await {
		Ok(user) => user.id,
		Err(error) => {
			eprintln!("Could not get my own user: {error}");
			return "Could not find my own user ID.".to_string();
		}
	};

	let mut before: Option<MessageId> = None;
	let mut scanned: u32 = 0;
	let mut deleted: u32 = 0;
	let mut failed: u32 = 0;
	let mut fetch_failed = false;

	while scanned < amount {
		// Discord returns at most 100 messages per request.
		let page_size = (amount - scanned).min(100) as u8;

		let mut builder = GetMessages::new().limit(page_size);
		if let Some(id) = before {
			builder = builder.before(id);
		}

		let messages = match channel_id.messages(&ctx.http, builder).await {
			Ok(messages) => messages,
			Err(error) => {
				eprintln!("Could not fetch messages: {error}");
				fetch_failed = true;
				break;
			}
		};

		if messages.is_empty() {
			break;
		}

		scanned += messages.len() as u32;
		
		// Newest first
		before = messages.last().map(|message| message.id);

		for message in messages.iter().filter(|m| m.author.id == bot_id) {
			match message.delete(&ctx.http).await {
				Ok(_) => deleted += 1,
				Err(error) => {
					failed += 1;
					eprintln!("Could not delete {}: {error}", message.id);
				}
			}
		}
	}

	let mut summary = format!("Scanned {scanned} messages. Deleted {deleted} of mine.");
	if failed > 0 {
		summary.push_str(&format!(" {failed} failed."));
	}
	if fetch_failed {
		summary.push_str(" Could not read channel history. I need View Channel and Read Message History.");
	}
	summary
}

/*
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_confirm() {
		assert_eq!(
			parse_custom_id("cleanup:confirm:100:12345"),
			Some(ButtonAction::Confirm { amount: 100, user: 12345 })
		);
	}

	#[test]
	fn parses_cancel() {
		assert_eq!(
			parse_custom_id("cleanup:cancel:12345"),
			Some(ButtonAction::Cancel { user: 12345 })
		);
	}

	#[test]
	fn rejects_bad_ids() {
		assert_eq!(parse_custom_id("other:confirm:1:2"), None);
		assert_eq!(parse_custom_id("cleanup:confirm:abc:2"), None);
		assert_eq!(parse_custom_id("cleanup:confirm:5"), None);
		assert_eq!(parse_custom_id("cleanup:nuke:5"), None);
		assert_eq!(parse_custom_id("cleanup:"), None);
	}

	#[test]
	fn owns_only_cleanup_ids() {
		assert!(owns_button("cleanup:cancel:1"));
		assert!(!owns_button("other:cancel:1"));
	}
}
 */
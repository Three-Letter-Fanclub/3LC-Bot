use crate::common::{Context, Error};

/// Names and shames those responsible for this bot
#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let version = env!("CARGO_PKG_VERSION");

    tracing::info!(requester = %ctx.author().id , "/about requested");

    // Links are in <> to suppress the embed
    let message = ctx
        .send(poise::CreateReply::default().content(format!(
            "\
3LC BOT v{version}
Code by: <@277914802071011328>
Contribute to the problem @<https://github.com/Three-Letter-Fanclub/3LC-Bot>
            "
        )))
        .await?
        .message()
        .await?
        .id;

    tracing::info!(message_id = %message, requester = %ctx.author().id , "/about reply sent");

    Ok(())
}

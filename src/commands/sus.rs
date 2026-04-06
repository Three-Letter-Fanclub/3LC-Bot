use crate::common::{Context, Error};
use crate::sus::sus::sus_image;
use poise::{serenity_prelude as serenity};
use std::string::ToString;
use tempfile::tempdir;

/// Converts an image to a bunch of twerking
/// crewmates from the hit deduction game
/// Among Us
#[poise::command(slash_command)]
pub async fn sus(
    ctx: Context<'_>,
    #[description = "image to sus"] attachment: serenity::Attachment,
    #[description = "Number of crewmates, default 21"] width: Option<u32>,
) -> Result<(), Error> {
    ctx.defer().await?;
    tracing::info!(attachment = %attachment.url, author = %ctx.author() ,"Received /sus");

    let bytes = reqwest::get(&attachment.url).await?.bytes().await?;
    tracing::info!(attachment = %attachment.url, "Downloaded attachment");

    let input_image = image::load_from_memory(&bytes)?;
    tracing::info!("Parsed attachment");

    let temp_dir = tempdir()
        .inspect_err(|e| tracing::error!(error = %e, "Failed to create temporary directory"))
        .expect("Failed to create temporary directory");

    let result_path = temp_dir.path().to_path_buf().join("result.gif");

    tracing::info!(path = %result_path.to_str().unwrap(), "Sussing image to path: ");

    sus_image(input_image, result_path.as_path(), width.unwrap_or(21u32))
        .inspect_err(|e| tracing::error!(error = %e, "Failed to sus image"))
        .unwrap();

    tracing::info!(author = %ctx.author(), "Image sussed, replying");

    let message = ctx
        .send(
            poise::CreateReply::default()
                .attachment(serenity::CreateAttachment::path(result_path).await?),
        )
        .await?
        .message()
        .await?
        .id;

    tracing::info!(message_id = %message, "Reply successfully sent");
    Ok(())
}

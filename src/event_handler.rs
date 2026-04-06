use crate::sus::sus::sus_image;
use poise::serenity_prelude as serenity;
use serenity::all::{Attachment, CreateAttachment, CreateMessage, Message};
use serenity::async_trait;
use serenity::prelude::*;
use tempfile::tempdir;

pub struct Handler;

fn is_dm(message: &Message) -> bool {
    message.guild_id.is_none()
}

fn find_image_attachment(message: &Message) -> Option<&Attachment> {
    message
        .attachments
        .iter()
        .find(|a| a.content_type.as_deref().unwrap_or("").contains("image"))
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::Context, msg: serenity::Message) {
        // Ignore messages not explicitly mentioning the bot
        // or DM's
        if !is_dm(&msg)
            && !msg
                .mentions
                .iter()
                .any(|u| u.id == ctx.cache.current_user().id)
            || msg.author.id == ctx.cache.current_user().id
        // Ignore messages from us
        {
            return;
        }

        if is_dm(&msg) {
            tracing::info!(message=%msg.content, "Bot DM'ed ")
        } else {
            tracing::info!(content = %msg.content,"Message @mentioned the bot");
        }

        let mut maybe_attachment = find_image_attachment(&msg);

        if maybe_attachment.is_none() {
            let maybe_message = &msg.referenced_message;
            if maybe_message.is_none() {
                tracing::info!("No referenced message found");
                return;
            }

            let referenced_message = maybe_message.as_ref().unwrap();
            maybe_attachment = find_image_attachment(referenced_message);
        }

        if maybe_attachment.is_none() {
            tracing::info!("No image attachments found");
            return;
        }

        let attachment = maybe_attachment.unwrap();
        tracing::info!(message_id = %msg.id, attachment = %attachment.url, "Message mentioning bot references an attached image, sussing");

        let bytes = reqwest::get(&attachment.url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        tracing::info!(attachment = %attachment.url, "Downloaded attachment");

        let input_image = image::load_from_memory(&bytes).unwrap();
        tracing::info!("Parsed attachment");

        let temp_dir = tempdir()
            .inspect_err(|e| tracing::error!(error = %e, "Failed to create temporary directory"))
            .expect("Failed to create temporary directory");

        let result_path = temp_dir.path().to_path_buf().join("result.gif");

        tracing::info!(path = %result_path.to_str().unwrap(), "Sussing image to path: ");

        match sus_image(input_image, result_path.as_path(), 21u32) {
            Ok(_) => (),
            Err(e) => {
                tracing::error!(error = %e, "Failed to sus image");
                return;
            }
        }

        tracing::info!(author = %msg.author, "Image sussed, replying");

        let response_attachment = CreateAttachment::path(result_path).await.unwrap();

        let response = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .reference_message(&msg)
                    .add_file(response_attachment),
            )
            .await
            .unwrap();

        tracing::info!(response = %response.id, "Response sent")
    }
}

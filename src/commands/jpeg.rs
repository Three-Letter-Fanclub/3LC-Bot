use crate::common::{Context, Error};
use image::codecs::jpeg::JpegEncoder;
use poise::serenity_prelude as serenity;
use std::io::Cursor;
use std::path::Path;

const QUALITY_DEFAULT: u8 = 1u8;

/// More JPEG
#[poise::command(slash_command)]
pub async fn more_jpeg(
    ctx: Context<'_>,
    #[description = "image to JPEG"] attachment: serenity::Attachment,
) -> Result<(), Error> {
    ctx.defer().await?;

    tracing::info!(attachment = %attachment.url, author = %ctx.author() ,"Received /more_jpeg");

    let bytes = reqwest::get(&attachment.url).await?.bytes().await?;
    tracing::info!(attachment = %attachment.url, "Downloaded attachment");

    let input_image = image::load_from_memory(&bytes)?;
    tracing::info!("Parsed attachment");

    let mut buffer = Cursor::new(Vec::new());
    let encoder = JpegEncoder::new_with_quality(&mut buffer, QUALITY_DEFAULT);
    input_image.write_with_encoder(encoder)?;

    let filename_no_attachment = Path::new(&attachment.filename)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    ctx.send(
        poise::CreateReply::default().attachment(serenity::CreateAttachment::bytes(
            buffer.into_inner(),
            format!("{}.jpeg", filename_no_attachment),
        )),
    )
    .await?;

    Ok(())
}

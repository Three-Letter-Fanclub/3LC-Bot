use poise::serenity_prelude as serenity;
use serenity::GuildId;
mod commands;
mod common;
mod event_handler;
mod sus;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::DIRECT_MESSAGES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::sus::sus(),
                commands::jpeg::more_jpeg(),
                commands::about::about(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // Debug builds register for the test server only
                if cfg!(debug_assertions) {
                    tracing::info!("Debug build, registering in test server only");
                    poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        GuildId::from(962075783239712789u64),
                    )
                    .await?;
                } else {
                    tracing::info!("Release build, registering globally");
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }

                Ok(())
            })
        })
        .build();

    tracing::info!("Starting bot");
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(event_handler::Handler)
        .await;
    client.unwrap().start().await.unwrap();
}

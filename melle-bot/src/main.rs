use anyhow::Context as _;
use shuttle_runtime::SecretStore;
use tracing::error;
use serenity::prelude::*;

mod models;
mod api;
mod handlers;
mod commands;
mod utils;
mod config;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let champion_names = api::fetch_champion_names().await.unwrap_or_else(|why| {
        error!("Failed to fetch champion names: {:?}", why);
        vec![]
    });

    let trait_to_champions = api::build_trait_dataset().await.unwrap_or_else(|why| {
        error!("Failed to build TFT dataset: {:?}", why);
        std::collections::HashMap::new()
    });

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(handlers::Bot { 
            champion_names,
            trait_to_champions,
        })
        .await
        .expect("Err creating client");

    Ok(client.into())
}

use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}

fn string_builder() -> String {
    let name = get_random_name();
    let role = get_random_role();
    let office = get_random_office();

    format!("Name: {}, Role: {}, Office: {}", name, role, office)
}

fn get_random_name() -> String {
    let names = "Alice,Bob,Eric";
    let names_vec: Vec<&str> = names.split(",").collect();
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..names_vec.len());

    let selected_name = names_vec[random_index];

    selected_name.to_string()
}

fn get_random_role() -> String {
    let roles = "Admin,User,Owner";
    let roles_vec: Vec<&str> = roles.split(",").collect();
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..roles_vec.len());

    let selected_role = roles_vec[random_index];

    selected_role.to_string()
}

fn get_random_office() -> String {
    let office = "New York,Los Angeles,Chicago";
    let office_vec: Vec<&str> = office.split(",").collect();
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..office_vec.len());

    let selected_office = office_vec[random_index];

    selected_office.to_string()
}
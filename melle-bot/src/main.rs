use anyhow::Context as _;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use rand::{thread_rng, Rng};
use utils::{string_builder, fill_builder};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // Handles recursive responses from the bot
        if msg.author.bot {
            return;
        }

        // Convert to lower case to make filtering easier
        let content_lower = msg.content.to_lowercase();
        if msg.content == "!hello" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", why);
            }
        }

        // Det skulle passa dig
        if msg.content == "!vadskullepassamig" {
            if let Err(why) = msg.channel_id.say(&ctx.http, string_builder()).await {
                error!("Error sending message: {:?}", why);
            }
        }

        // Flex 5 comp 
        if msg.content == "!fill5" {
            let combined_message = fill_builder();
            if let Err(why) = msg.channel_id.say(&ctx.http, combined_message).await {
                error!("Error sending message: {:?}", why);
            }
        }

        // He bli inge kaffe
        if content_lower.contains("kaffe") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "He bli INGE kaffe!!").await {
                error!("Error sending message: {:?}", why);
            }
        }

        if content_lower.contains("långt") || content_lower.contains("kort") || content_lower.contains("km") || content_lower.contains("mil") || content_lower.contains("kilometer") || content_lower.contains("meter") || content_lower.contains("distans") {
            if let Err(why) = msg.channel_id.say(&ctx.http, "De e lika långt för meeej som för deeej").await {
                error!("Error sending message: {:?}", why);
            }
        } 

        // Leave this here to avoid async thread swapping
        let chance_of_responding = 10;

        let random = {
            let mut rng = thread_rng();
            rng.gen_range(0..100)
        };

        if random < chance_of_responding {
            let responses = vec! [
                "MEN!",
                "Dö inte här nu Berg... 😰\nMen vad GÖR DU!? 😱",
                "Men jag hade ju rödbuff... 🤠",
                "Jaa e död, jaa e död. 😵",
                "I live boys, I live! 😎",
                "Någon borde städa upp här, det ser fördjävligt ut. 🧐",
                "Jag fyller år, då får jag spela hela tiden. 🥳",
                "Jag har en aggressiv ⚔ spelstil! 💪",
                "Mellebajs, mellebajs! 💩",
                "Back in the botlane boooys! 😎",
                "Ni förståååår inte 🤡"
            ];

            let random_vec_index = {
                let mut rng = thread_rng();
                rng.gen_range(0..responses.len())
            };

            let response = responses[random_vec_index];
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                error!("Error sending message: {:?}", why);
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

mod utils;

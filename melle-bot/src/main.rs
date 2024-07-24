use anyhow::Context as _;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use rand::{thread_rng, Rng};
use reqwest::Error;
use serde::Deserialize;

struct Bot{
    champion_names: Vec<String>,
}

#[derive(Deserialize)]
struct ChampionData {
    data: std::collections::HashMap<String, Champion>,
}

#[derive(Deserialize)]
struct Champion {
    name: String,
}

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
            let response = utils::string_builder(&self.champion_names);
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                error!("Error sending message: {:?}", why);
            }
        }

        // Flex 5 comp 
        if msg.content == "!fill5" {
            let combined_message = utils::fill_builder(&self.champion_names);
            if let Err(why) = msg.channel_id.say(&ctx.http, combined_message).await {
                error!("Error sending message: {:?}", why);
            }
        }

        // Helper funktion för att få info kring kommandon
        if msg.content == "!help" {
            let response = "Jag är riktiga Melle, JA E INGEN BOT. Jag kan hjälpa dig att välja vad som skulle passa dig. Skriv: \n* `!vadskullepassamig` så räknar jag ut vad som skulle passa dig. \n* `!flex5` om du istället vill att jag räknar ut vad som skulle passa er som lag. \nMina tips är baserade på årtionden av erfarenhet och en oändlig rad meriter, inte minst som SJUTTONDE BÄSTA Jarvan EUW!";
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
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

//    let riot_api_key = secrets
//        .get("RIOT_API_KEY")
//        .context("'RIOT_API_KEY' was not found")?;

    let champion_names = fetch_champion_names().await.unwrap_or_else(|why| {
        error!("Failed to fetch champion names: {:?}", why);
        vec![]
    });

    
    println!("List of champs {:?}", champion_names);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot { champion_names })
        .await
        .expect("Err creating client");

    Ok(client.into())
}

async fn fetch_champion_names() -> Result<Vec<String>, Error> {
    let url = "http://ddragon.leagueoflegends.com/cdn/14.10.1/data/en_US/champion.json";
    let response = reqwest::get(url).await?.json::<ChampionData>().await?;
    let champion_names = response.data.values().map(|champ| champ.name.clone()).collect();
    Ok(champion_names)
}

mod utils;

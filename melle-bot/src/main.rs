use anyhow::Context as _;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use rand::{rng, Rng};
use reqwest::Error;
use serde::Deserialize;

struct Bot{
    champion_names: Vec<String>,
    trait_to_champions: std::collections::HashMap<String, TraitData>,
}

pub struct TraitData {
    pub name: String,
    pub champions: Vec<String>, // Champion names
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

        // TFT comp suggestion
        if msg.content == "!tftcomp" {
            let response = utils::tft_comp_builder(&self.trait_to_champions);
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                error!("Error sending message: {:?}", why);
            }
        }

        // Helper funktion för att få info kring kommandon
        if msg.content == "!help" {
            let response = "Jag är riktiga Melle, JA E INGEN BOT. Jag kan hjälpa dig att välja vad som skulle passa dig. Skriv: \n* `!vadskullepassamig` så räknar jag ut vad som skulle passa dig. \n* `!fill5` om du istället vill att jag räknar ut vad som skulle passa er som lag. \n* `!tftcomp` för att få ett TFT-comp förslag. \nMina tips är baserade på årtionden av erfarenhet och en oändlig rad meriter, inte minst som SJUTTONDE BÄSTA Jarvan EUW!";
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
            let mut rng = rng();
            rng.random_range(0..100)
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
                "Ni förståååår inte 🤡",
                "Det där är en dålig meme... 👎"
            ];

            let random_vec_index = {
                let mut rng = rng();
                rng.random_range(0..responses.len())
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

    let trait_to_champions = build_trait_dataset().await.unwrap_or_else(|why| {
        error!("Failed to build TFT dataset: {:?}", why);
        std::collections::HashMap::new()
    });
    
    println!("List of champs {:?}", champion_names);
    println!("TFT traits with champions: {}", trait_to_champions.len());
    
    // Test the TFT comp builder locally
    println!("\n=== TESTING TFT COMP BUILDER ===");
    for i in 1..=5 {
        let test_result = utils::tft_comp_builder(&trait_to_champions);
        println!("Test {}: {}", i, test_result);
    }
    println!("=== END TEST ===\n");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot { 
            champion_names,
            trait_to_champions,
        })
        .await
        .expect("Err creating client");

    Ok(client.into())
}

async fn fetch_champion_names() -> Result<Vec<String>, Error> {
    let url = "http://ddragon.leagueoflegends.com/cdn/16.2.1/data/en_US/champion.json";
    let response = reqwest::get(url).await?.json::<ChampionData>().await?;
    let champion_names = response.data.values().map(|champ| champ.name.clone()).collect();
    Ok(champion_names)
}

async fn build_trait_dataset() -> Result<std::collections::HashMap<String, TraitData>, Error> {
    // Use Community Dragon which has champion traits
    let url = "https://raw.communitydragon.org/latest/cdragon/tft/en_us.json";
    println!("Fetching TFT data from: {}", url);
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    println!("Successfully fetched TFT data");
    
    // Debug: Check if sets exists
    if !response.get("sets").is_some() {
        println!("ERROR: No 'sets' field in response!");
        println!("Top-level keys: {:?}", response.as_object().map(|o| o.keys().collect::<Vec<_>>()));
        return Ok(std::collections::HashMap::new());
    }
    
    // First, build a map of trait_id -> trait_name
    // Also create a map for matching without the "TFT16_" prefix
    let mut trait_names: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut trait_names_suffix: std::collections::HashMap<String, (String, String)> = std::collections::HashMap::new(); // suffix -> (full_id, name)
    if let Some(sets) = response.get("sets") {
        println!("Found 'sets' field");
        if let Some(set) = sets.get("16") {
            println!("Found set 16");
            if let Some(traits_obj) = set.get("traits") {
                println!("Found 'traits' field in set 16");
                if let Some(traits_array) = traits_obj.as_array() {
                    println!("Traits is an array with {} items", traits_array.len());
                    for trait_val in traits_array {
                        if let Some(trait_obj) = trait_val.as_object() {
                            let id = trait_obj.get("apiName")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            
                            let name = trait_obj.get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            
                            // Filter out augments and tutorial traits
                            if !id.contains("MechanicTrait") && 
                               !id.contains("Tutorial") &&
                               !id.contains("Teamup") &&
                               !id.is_empty() &&
                               !name.is_empty() {
                                trait_names.insert(id.clone(), name.clone());
                                
                                // Also store by suffix (part after last underscore) for matching
                                if let Some(underscore_pos) = id.rfind('_') {
                                    let suffix = &id[underscore_pos + 1..];
                                    trait_names_suffix.insert(suffix.to_string(), (id.clone(), name.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("Found {} valid traits", trait_names.len());
    
    // Now build trait -> champions mapping
    let mut trait_to_champions: std::collections::HashMap<String, TraitData> = std::collections::HashMap::new();
    let mut total_champions_processed = 0;
    
    if let Some(sets) = response.get("sets") {
        if let Some(set) = sets.get("16") {
            println!("Processing champions from set 16");
            if let Some(champions_obj) = set.get("champions") {
                println!("Found 'champions' field");
                if let Some(champions_array) = champions_obj.as_array() {
                    println!("Champions is an array with {} items", champions_array.len());
                    for champ_val in champions_array {
                        if let Some(champ_obj) = champ_val.as_object() {
                            let champion_name = champ_obj.get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            
                            if champion_name.is_empty() {
                                continue;
                            }
                            
                            total_champions_processed += 1;
                            
                            // Extract traits array - traits can be strings or objects
                            if let Some(traits_array) = champ_obj.get("traits") {
                                if let Some(traits) = traits_array.as_array() {
                                    for trait_val in traits {
                                        // Handle both string and object formats
                                        let trait_id = if let Some(trait_str) = trait_val.as_str() {
                                            Some(trait_str.to_string())
                                        } else if let Some(trait_obj) = trait_val.as_object() {
                                            // Try to get apiName or name field
                                            trait_obj.get("apiName")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string())
                                                .or_else(|| {
                                                    trait_obj.get("name")
                                                        .and_then(|v| v.as_str())
                                                        .map(|s| s.to_string())
                                                })
                                        } else {
                                            None
                                        };
                                        
                                        if let Some(trait_id) = trait_id {
                                            // Filter out augments
                                            if !trait_id.contains("MechanicTrait") && 
                                               !trait_id.contains("Tutorial") &&
                                               !trait_id.contains("Teamup") {
                                                
                                                // Try to find trait - first try exact match, then try by suffix
                                                let (full_trait_id, trait_name) = if let Some(name) = trait_names.get(&trait_id) {
                                                    // Exact match found
                                                    (trait_id.clone(), name.clone())
                                                } else if let Some((full_id, name)) = trait_names_suffix.get(&trait_id) {
                                                    // Match by suffix (e.g., "Shurima" matches "TFT16_Shurima")
                                                    (full_id.clone(), name.clone())
                                                } else {
                                                    // No match found
                                                    if total_champions_processed <= 3 {
                                                        println!("DEBUG: Champion '{}' has trait '{}' which is not in trait_names", champion_name, trait_id);
                                                    }
                                                    continue;
                                                };
                                                
                                                trait_to_champions
                                                    .entry(full_trait_id.clone())
                                                    .or_insert_with(|| TraitData {
                                                        name: trait_name.clone(),
                                                        champions: Vec::new(),
                                                    })
                                                    .champions.push(champion_name.clone());
                                            }
                                        } else if total_champions_processed <= 3 {
                                            println!("DEBUG: Champion '{}' has trait that couldn't be parsed: {:?}", champion_name, trait_val);
                                        }
                                    }
                                } else if total_champions_processed <= 3 {
                                    println!("DEBUG: Champion '{}' has traits that aren't an array: {:?}", champion_name, traits_array);
                                }
                            } else if total_champions_processed <= 3 {
                                println!("DEBUG: Champion '{}' has no traits field", champion_name);
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("\n=== TFT TRAIT DATASET (Set 16) ===");
    println!("Processed {} champions", total_champions_processed);
    println!("Total traits with champions: {}\n", trait_to_champions.len());
    
    if trait_to_champions.is_empty() {
        println!("WARNING: No traits found! This might indicate a data structure issue.");
        println!("Sample trait_names keys: {:?}", trait_names.keys().take(5).collect::<Vec<_>>());
    }
    
    // Sort by trait name for easier reading
    let mut sorted_traits: Vec<(&String, &TraitData)> = trait_to_champions.iter().collect();
    sorted_traits.sort_by(|a, b| a.1.name.cmp(&b.1.name));
    
    for (trait_id, data) in sorted_traits {
        println!("{} ({}):", data.name, trait_id);
        println!("  Champions ({})", data.champions.len());
        for champion in &data.champions {
            println!("    - {}", champion);
        }
        println!();
    }
    
    Ok(trait_to_champions)
}

mod utils;

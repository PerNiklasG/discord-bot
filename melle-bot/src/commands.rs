use crate::models::TraitData;
use serenity::model::channel::Message;
use serenity::prelude::Context;
use tracing::error;

async fn send_message(ctx: &Context, msg: &Message, content: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        error!("Error sending message: {:?}", why);
    }
}

pub async fn handle_hello(ctx: &Context, msg: &Message) {
    send_message(ctx, msg, "world!").await;
}

pub async fn handle_vadskullepassamig(ctx: &Context, msg: &Message, champion_names: &[String]) {
    let response = crate::utils::string_builder(champion_names);
    send_message(ctx, msg, &response).await;
}

pub async fn handle_fill5(ctx: &Context, msg: &Message, champion_names: &[String]) {
    let combined_message = crate::utils::fill_builder(champion_names);
    send_message(ctx, msg, &combined_message).await;
}

pub async fn handle_tftcomp(
    ctx: &Context, 
    msg: &Message, 
    trait_to_champions: &std::collections::HashMap<String, TraitData>
) {
    let response = crate::utils::tft_comp_builder(trait_to_champions);
    send_message(ctx, msg, &response).await;
}

pub async fn handle_help(ctx: &Context, msg: &Message) {
    let response = "Jag är riktiga Melle, JA E INGEN BOT. Jag kan hjälpa dig att välja vad som skulle passa dig. Skriv: \n* `!vadskullepassamig` så räknar jag ut vad som skulle passa dig. \n* `!fill5` om du istället vill att jag räknar ut vad som skulle passa er som lag. \n* `!tftcomp` för att få ett TFT-comp förslag. \nMina tips är baserade på årtionden av erfarenhet och en oändlig rad meriter, inte minst som SJUTTONDE BÄSTA Jarvan EUW!";
    send_message(ctx, msg, response).await;
}

pub async fn handle_kaffe(ctx: &Context, msg: &Message) {
    send_message(ctx, msg, "He bli INGE kaffe!!").await;
}

pub async fn handle_distans(ctx: &Context, msg: &Message) {
    send_message(ctx, msg, "De e lika långt för meeej som för deeej").await;
}

const RANDOM_RESPONSES: &[&str] = &[
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

pub async fn handle_random_response(ctx: &Context, msg: &Message) {
    use rand::{rng, Rng};
    let random_index = {
        let mut rng = rng();
        rng.random_range(0..RANDOM_RESPONSES.len())
    };
    send_message(ctx, msg, RANDOM_RESPONSES[random_index]).await;
}

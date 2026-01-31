use crate::models::TraitData;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use rand::{rng, Rng};

pub struct Bot {
    pub champion_names: Vec<String>,
    pub trait_to_champions: std::collections::HashMap<String, TraitData>,
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

        // Handle commands
        match msg.content.as_str() {
            "!hello" => {
                crate::commands::handle_hello(&ctx, &msg).await;
            }
            "!vadskullepassamig" => {
                crate::commands::handle_vadskullepassamig(&ctx, &msg, &self.champion_names).await;
            }
            "!fill5" => {
                crate::commands::handle_fill5(&ctx, &msg, &self.champion_names).await;
            }
            "!tftcomp" => {
                crate::commands::handle_tftcomp(&ctx, &msg, &self.trait_to_champions).await;
            }
            "!help" => {
                crate::commands::handle_help(&ctx, &msg).await;
            }
            _ => {}
        }

        // Handle keyword triggers
        if content_lower.contains("kaffe") {
            crate::commands::handle_kaffe(&ctx, &msg).await;
        }

        if crate::config::DISTANCE_KEYWORDS.iter().any(|keyword| content_lower.contains(keyword)) {
            crate::commands::handle_distans(&ctx, &msg).await;
        }

        // Random responses
        let random = {
            let mut rng = rng();
            rng.random_range(0..100)
        };

        if random < crate::config::RANDOM_RESPONSE_CHANCE as u32 {
            crate::commands::handle_random_response(&ctx, &msg).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
}

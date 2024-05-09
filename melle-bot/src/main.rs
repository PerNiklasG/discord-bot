use anyhow::Context as _;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::{error, info};
use rand::{thread_rng, Rng};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // Handles recursive responses from the bot
        if msg.author.bot {
            return;
        }

        let content_lower = msg.content.to_lowercase();
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }

        // Det skulle passa dig
        if msg.content == "!vadskullepassamig" {
            if let Err(e) = msg.channel_id.say(&ctx.http, string_builder()).await {
                error!("Error sending message: {:?}", e);
            }
        }

        // He bli inge kaffe
        if content_lower.contains("kaffe") {
            if let Err(e) = msg.channel_id.say(&ctx.http, "He bli INGE kaffe!!").await {
                error!("Error sending message: {:?}", e);
            }
        }

        if content_lower.contains("långt") || content_lower.contains("kort") || content_lower.contains("km") || content_lower.contains("mil") || content_lower.contains("kilometer") || content_lower.contains("meter") || content_lower.contains("distans") {
            if let Err(e) = msg.channel_id.say(&ctx.http, "De e lika långt för meeej som för deeej").await {
                error!("Error sending message: {:?}", e);
            }
        } 

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
                "Jag har en aggressiv ⚔ spelstil! 💪"
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

fn string_builder() -> String {
    let build = get_random_build();
    let champ = get_random_champ();
    let role = get_random_role();

    format!("Du borde testa {} {} {}, deeet hade passat DIG!", build, champ, role)
}

fn get_random_champ() -> String {
    let champs = "Ahri,Akali,Alistar,Amumu,Anivia,Annie,Ashe,Azir,Akshan,Aurelion Sol,Aphelios,Blitzcrank,Brand,Braum,Bard,Belveth,Caitlyn,Cassiopeia,Cho'Gath,Corki,Camille,Darius,Diana,Dr. Mundo,Draven,Elise,Evelynn,Ekko,Ezreal,Fiddlesticks,Fiora,Fizz,Galio,Gangplank,Garen,Gnar,Gragas,Graves,Gwen,Hecarim,Heimerdinger,Irelia,Illaoi,Ivern,Janna,Jarvan IV,Jax,Jayce,Jinx,Jhin,Kalista,Karma,Karthus,Kassadin,Katarina,Kindred,Kayle,Kennen,Kha'Zix,Kog'Maw,Kled,Kayn,Kai'sa,K’Sante,LeBlanc,Lee Sin,Leona,Lissandra,Lucian,Lulu,Lux,Lillia,Malphite,Malzahar,Maokai,Master Yi,Milio,Miss Fortune,Mordekaiser,Morgana,Nami,Nasus,Nautilus,Nidalee,Nocturne,Nunu,Nilah,Neeko,Olaf,Orianna,Ornn,Pantheon,Poppy,Pyke,Quinn,Qiyana,Rammus,Rek'Sai,Renekton,Rengar,Riven,Rumble,Ryze,Renata,Rell,Rakan,Sejuani,Shaco,Shen,Shyvana,Singed,Sion,Sivir,Skarner,Sona,Soraka,Swain,Syndra,Senna,Sett,Samira,Seraphine,Sylas,Talon,Taric,Teemo,Thresh,Tristana,Trundle,Tryndamere,Twisted Fate,Twitch,Tahm kench,Taliyah,Udyr,Urgot,Varus,Vayne,Veigar,Vel'Koz,Vi,Viktor,Vladimir,Volibear,Vex,Viego,Warwick,Wukong,Xerath,Xin Zhao,Xayah,Yasuo,Yorick,Yuumi,Yone,Zac,Zed,Ziggs,Zilean,Zyra,Zeri,Zoe";
    let champs_vec: Vec<&str> = champs.split(",").collect();
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..champs_vec.len());

    let selected_champ = champs_vec[random_index];

    selected_champ.to_string()
}

fn get_random_build() -> String {
    let builds = "AD,AP,Tank,On-Hit,Ability Haste";
    let builds_vec: Vec<&str> = builds.split(",").collect();

    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..builds_vec.len());

    let selected_build = builds_vec[random_index];

    selected_build.to_string()
}

fn get_random_role() -> String {
    let role = "Top,Mid,Jungle,Bot,Supp";
    let role_vec: Vec<&str> = role.split(",").collect();

    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..role_vec.len());

    let selected_role = role_vec[random_index];

    selected_role.to_string()
}

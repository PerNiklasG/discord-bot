// utils.rs
use rand::{thread_rng, Rng};
use std::sync::{Arc, RwLock};

pub static BUILDS: &str = "AD,AP,Tank,On-Hit,Ability Haste";
pub static ROLES: &str = "Top,Mid,Jungle,Bot,Supp";
pub static CHAMPS: &str = "Ahri,Akali,Alistar,Amumu,Anivia,Annie,Ashe,Azir,Akshan,Aurelion Sol,Aphelios,Blitzcrank,Brand,Braum,Briar,Bard,Belveth,Caitlyn,Cassiopeia,Cho'Gath,Corki,Camille,Darius,Diana,Dr. Mundo,Draven,Elise,Evelynn,Ekko,Ezreal,Fiddlesticks,Fiora,Fizz,Galio,Gangplank,Garen,Gnar,Gragas,Graves,Gwen,Hecarim,Heimerdinger,Hwei,Irelia,Illaoi,Ivern,Janna,Jarvan IV,Jax,Jayce,Jinx,Jhin,Kalista,Karma,Karthus,Kassadin,Katarina,Kindred,Kayle,Kennen,Kha'Zix,Kog'Maw,Kled,Kayn,Kai'sa,K’Sante,LeBlanc,Lee Sin,Leona,Lissandra,Lucian,Lulu,Lux,Lillia,Malphite,Malzahar,Maokai,Master Yi,Milio,Miss Fortune,Mordekaiser,Morgana,Nafiri,Nami,Nasus,Nautilus,Nidalee,Nocturne,Nunu,Nilah,Neeko,Olaf,Orianna,Ornn,Pantheon,Poppy,Pyke,Quinn,Qiyana,Rammus,Rek'Sai,Renekton,Rengar,Riven,Rumble,Ryze,Renata,Rell,Rakan,Sejuani,Shaco,Shen,Shyvana,Singed,Sion,Sivir,Skarner,Sona,Soraka,Swain,Syndra,Senna,Sett,Samira,Seraphine,Smolder,Sylas,Talon,Taric,Teemo,Thresh,Tristana,Trundle,Tryndamere,Twisted Fate,Twitch,Tahm kench,Taliyah,Udyr,Urgot,Varus,Vayne,Veigar,Vel'Koz,Vi,Viktor,Vladimir,Volibear,Vex,Viego,Warwick,Wukong,Xerath,Xin Zhao,Xayah,Yasuo,Yorick,Yuumi,Yone,Zac,Zed,Ziggs,Zilean,Zyra,Zeri,Zoe";

pub fn string_builder() -> String {
    format!("Du borde testa {} {} {}, deeet hade passat DIG!", get_random(BUILDS.to_string()), get_random(CHAMPS.to_string()), get_random(ROLES.to_string()))
}

pub fn get_random(s: String) -> String {
    let vec: Vec<&str> = s.split(",").collect();
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..vec.len());

    let result = vec[random_index];

    result.to_string()
}

pub fn get_random_list(champion_names: Arc<RwLock<Vec<String>>>) -> String {
    let names = champion_names.read().unwrap();
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..names.len());

    names[random_index].clone()
}

pub fn fill_builder() -> String {
    let positions = ["Top", "Mid", "Jungle", "Bot", "Supp"];
    let team: Vec<String> = positions.iter().map(|&pos| {
        format!("{}: {} {}", pos, get_random(BUILDS.to_string()), get_random(CHAMPS.to_string()))
    }).collect();

    format!(
        "Jag tycker att ni borde testa: \n{}\nDeeeet hade passat ER!",
        team.join("\n")
    )
}
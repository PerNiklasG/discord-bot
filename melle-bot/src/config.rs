pub const DATA_DRAGON_VERSION: &str = "16.2.1";
pub const TFT_SET: &str = "16";
pub const COMMUNITY_DRAGON_URL: &str = "https://raw.communitydragon.org/latest/cdragon/tft/en_us.json";

pub fn data_dragon_champion_url() -> String {
    format!("http://ddragon.leagueoflegends.com/cdn/{}/data/en_US/champion.json", DATA_DRAGON_VERSION)
}

pub const RANDOM_RESPONSE_CHANCE: u8 = 10;

pub const DISTANCE_KEYWORDS: &[&str] = &[
    "långt", "kort", "km", "mil", "kilometer", "meter", "distans"
];

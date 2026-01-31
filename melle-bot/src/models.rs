use serde::Deserialize;

pub struct TraitData {
    pub name: String,
    pub champions: Vec<String>, // Champion names
}

#[derive(Deserialize)]
pub struct ChampionData {
    pub data: std::collections::HashMap<String, Champion>,
}

#[derive(Deserialize)]
pub struct Champion {
    pub name: String,
}

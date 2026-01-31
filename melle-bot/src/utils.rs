// utils.rs
use rand::{rng, Rng};

pub static BUILDS: &str = "AD,AP,Tank,On-Hit,Ability Haste";
pub static ROLES: &str = "Top,Mid,Jungle,Bot,Supp";

pub fn string_builder(champion_names: &[String]) -> String {
    format!("Du borde testa {} {} {}, deeet hade passat DIG!", get_random(BUILDS.to_string()), get_random_list(champion_names), get_random(ROLES.to_string()))
}

pub fn get_random(s: String) -> String {
    let vec: Vec<&str> = s.split(",").collect();
    let mut rng = rng();
    let random_index = rng.random_range(0..vec.len());

    let result = vec[random_index];

    result.to_string()
}

pub fn get_random_list(input: &[String]) -> String {
    let mut rng = rng();
    let random_index = rng.random_range(0..input.len());

    input[random_index].clone()
}

pub fn fill_builder(champion_names: &[String]) -> String {
    let positions = ["Top", "Mid", "Jungle", "Bot", "Supp"];
    let team: Vec<String> = positions.iter().map(|&pos| {
        format!("{}: {} {}", pos, get_random(BUILDS.to_string()), get_random_list(champion_names))
    }).collect();

    format!(
        "Jag tycker att ni borde testa: \n{}\nDeeeet hade passat ER!",
        team.join("\n")
    )
}

pub fn tft_comp_builder(trait_to_champions: &std::collections::HashMap<String, crate::TraitData>) -> String {
    use crate::TraitData;
    
    // Minimum number of champions required for a trait to be selectable
    // Change this to 4 or any other number as needed
    const MIN_CHAMPIONS: usize = 1;
    
    // Filter traits that have at least MIN_CHAMPIONS champions
    let valid_traits: Vec<(&String, &TraitData)> = trait_to_champions.iter()
        .filter(|(_, data)| data.champions.len() >= MIN_CHAMPIONS)
        .collect();
    
    if valid_traits.is_empty() {
        return "Kunde inte hitta några TFT-traits!".to_string();
    }
    
    // Pick a random trait
    let mut rng = rng();
    let trait_index = rng.random_range(0..valid_traits.len());
    let (_, selected_trait_data) = valid_traits[trait_index];
    
    // Pick a random champion from this trait
    let champ_index = rng.random_range(0..selected_trait_data.champions.len());
    let selected_champion = &selected_trait_data.champions[champ_index];
    
    format!("{} {}", selected_trait_data.name, selected_champion)
}
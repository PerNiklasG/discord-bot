// utils.rs
use rand::{thread_rng, Rng};

pub static BUILDS: &str = "AD,AP,Tank,On-Hit,Ability Haste";
pub static ROLES: &str = "Top,Mid,Jungle,Bot,Supp";

pub fn string_builder(champion_names: &[String]) -> String {
    format!("Du borde testa {} {} {}, deeet hade passat DIG!", get_random(BUILDS.to_string()), get_random_list(champion_names), get_random(ROLES.to_string()))
}

pub fn get_random(s: String) -> String {
    let vec: Vec<&str> = s.split(",").collect();
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..vec.len());

    let result = vec[random_index];

    result.to_string()
}

pub fn get_random_list(input: &[String]) -> String {
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..input.len());

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
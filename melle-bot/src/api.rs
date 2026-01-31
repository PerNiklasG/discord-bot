use crate::models::TraitData;
use crate::config;
use reqwest::Error;

pub async fn fetch_champion_names() -> Result<Vec<String>, Error> {
    use crate::models::ChampionData;
    
    let response = reqwest::get(config::data_dragon_champion_url()).await?.json::<ChampionData>().await?;
    let champion_names = response.data.values().map(|champ| champ.name.clone()).collect();
    Ok(champion_names)
}

fn extract_trait_id(trait_val: &serde_json::Value) -> Option<String> {
    trait_val.as_str()
        .map(|s| s.to_string())
        .or_else(|| {
            trait_val.as_object()
                .and_then(|obj| {
                    obj.get("apiName")
                        .or_else(|| obj.get("name"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
        })
}

fn is_valid_trait(trait_id: &str) -> bool {
    !trait_id.contains("MechanicTrait") 
        && !trait_id.contains("Tutorial")
        && !trait_id.contains("Teamup")
        && !trait_id.is_empty()
}

fn get_set_data(response: &serde_json::Value) -> Option<&serde_json::Value> {
    response.get("sets")?.get(config::TFT_SET)
}

pub async fn build_trait_dataset() -> Result<std::collections::HashMap<String, TraitData>, Error> {
    let response = reqwest::get(config::COMMUNITY_DRAGON_URL).await?.json::<serde_json::Value>().await?;
    
    let set_data = match get_set_data(&response) {
        Some(data) => data,
        None => return Ok(std::collections::HashMap::new()),
    };
    
    // Build trait_id -> trait_name mapping
    let mut trait_names: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut trait_names_suffix: std::collections::HashMap<String, (String, String)> = std::collections::HashMap::new();
    
    if let Some(traits_array) = set_data.get("traits").and_then(|v| v.as_array()) {
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
                
                if is_valid_trait(&id) && !name.is_empty() {
                    trait_names.insert(id.clone(), name.clone());
                    
                    if let Some(underscore_pos) = id.rfind('_') {
                        let suffix = &id[underscore_pos + 1..];
                        trait_names_suffix.insert(suffix.to_string(), (id, name));
                    }
                }
            }
        }
    }
    
    // Build trait -> champions mapping
    let mut trait_to_champions: std::collections::HashMap<String, TraitData> = std::collections::HashMap::new();
    
    if let Some(champions_array) = set_data.get("champions").and_then(|v| v.as_array()) {
        for champ_val in champions_array {
            if let Some(champ_obj) = champ_val.as_object() {
                let champion_name = champ_obj.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                
                if champion_name.is_empty() {
                    continue;
                }
                
                if let Some(traits_array) = champ_obj.get("traits").and_then(|v| v.as_array()) {
                    for trait_val in traits_array {
                        if let Some(trait_id) = extract_trait_id(trait_val) {
                            if is_valid_trait(&trait_id) {
                                let (full_trait_id, trait_name) = match trait_names.get(&trait_id) {
                                    Some(name) => (trait_id.clone(), name.clone()),
                                    None => match trait_names_suffix.get(&trait_id) {
                                        Some((full_id, name)) => (full_id.clone(), name.clone()),
                                        None => continue,
                                    }
                                };
                                
                                trait_to_champions
                                    .entry(full_trait_id)
                                    .or_insert_with(|| TraitData {
                                        name: trait_name.clone(),
                                        champions: Vec::new(),
                                    })
                                    .champions.push(champion_name.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(trait_to_champions)
}

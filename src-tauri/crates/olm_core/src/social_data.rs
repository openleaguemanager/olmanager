use crate::domain::social::{SocialAccount, SocialTemplate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MatchTexts {
    pub team_loser: HashMap<String, Vec<String>>,
    pub team_loser_stomp: HashMap<String, Vec<String>>,
    pub team_loser_close: HashMap<String, Vec<String>>,
    pub fan_reaction_won: HashMap<String, Vec<String>>,
    pub fan_reaction_lost: HashMap<String, Vec<String>>,
    pub bouzys_vs_fnatic: HashMap<String, Vec<String>>,
}

pub fn load_social_accounts(data_base: &Path) -> Option<Vec<SocialAccount>> {
    let path = data_base.join("social").join("accounts.json");
    if !path.exists() {
        return None;
    }
    let json = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&json).ok()
}

pub fn load_social_templates(data_base: &Path) -> Option<Vec<SocialTemplate>> {
    let path = data_base.join("social").join("templates.json");
    if !path.exists() {
        return None;
    }
    let json = std::fs::read_to_string(&path).ok()?;
    let file: SocialTemplateFile = serde_json::from_str(&json).ok()?;
    Some(file.templates)
}

pub fn load_match_texts(data_base: &Path) -> Option<MatchTexts> {
    let path = data_base.join("social").join("match_texts.json");
    if !path.exists() {
        return None;
    }
    let json = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&json).ok()
}

#[derive(Debug, Deserialize)]
struct SocialTemplateFile {
    templates: Vec<SocialTemplate>,
}

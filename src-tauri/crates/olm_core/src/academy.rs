use crate::domain::team::{Team, TeamKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

use log::info;

pub use crate::domain::team::ErlAssignmentRule;

const BASE_ACADEMY_ACQUISITION_COST: i64 = 100_000;
const REPUTATION_COST_MULTIPLIER: i64 = 40_000;

// ---------------------------------------------------------------------------
// Types — acquisition options
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErlLeagueDefinition {
    pub id: String,
    pub name: String,
    pub country_code: String,
    pub region: String,
    pub reputation: u8,
    pub nearby_country_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcademyAcquisitionOption {
    pub source_team_id: String,
    pub name: String,
    pub short_name: String,
    pub logo_url: Option<String>,
    pub erl_league_id: String,
    pub erl_league_name: String,
    pub country_code: String,
    pub assignment_rule: ErlAssignmentRule,
    pub fallback_reason: Option<String>,
    pub reputation: u8,
    pub development_level: u8,
    pub acquisition_cost: i64,
}

pub type AcademyCreationOption = AcademyAcquisitionOption;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErlAcademyCandidate {
    pub source_team_id: String,
    pub name: String,
    pub short_name: String,
    pub logo_url: Option<String>,
    pub erl_league_id: String,
    pub country_code: String,
    pub reputation: u8,
    pub development_level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcademyError {
    ParentMustBeMainTeam { team_id: String },
    AcademyAlreadyExists { parent_team_id: String, academy_team_id: String },
    InsufficientFunds { available: i64, required: i64 },
    UnrelatedAcademy { parent_team_id: String, academy_team_id: String },
}

// ---------------------------------------------------------------------------
// Types — seed catalog
// ---------------------------------------------------------------------------

/// Internal seed type for academy player data read from JSON.
#[derive(Debug, Clone)]
pub struct AcademyPlayerSeed {
    pub role: String,
    pub nickname: String,
    pub full_name: String,
    pub nationality: String,
    pub dob: Option<String>,
    pub image_url: String,
}

/// Internal seed type for academy team data read from JSON.
#[derive(Debug, Clone)]
pub struct AcademyTeamSeed {
    pub league_id: String,
    pub league_name: String,
    pub country_code: String,
    pub team_name: String,
    pub short_name: String,
    pub logo_url: Option<String>,
    pub players: Vec<AcademyPlayerSeed>,
}

// ---------------------------------------------------------------------------
// Acquisition logic
// ---------------------------------------------------------------------------

pub fn eligible_academy_acquisition_options(
    team_country_code: &str,
    leagues: &[ErlLeagueDefinition],
    candidates: &[ErlAcademyCandidate],
) -> Vec<AcademyAcquisitionOption> {
    candidates
        .iter()
        .filter_map(|candidate| {
            let league = leagues
                .iter()
                .find(|league| league.id == candidate.erl_league_id)?;
            let is_domestic = country_matches(&candidate.country_code, team_country_code);
            let assignment_rule = if is_domestic {
                ErlAssignmentRule::Domestic
            } else {
                ErlAssignmentRule::Fallback
            };

            Some(AcademyAcquisitionOption {
                source_team_id: candidate.source_team_id.clone(),
                name: candidate.name.clone(),
                short_name: candidate.short_name.clone(),
                logo_url: candidate.logo_url.clone(),
                erl_league_id: league.id.clone(),
                erl_league_name: league.name.clone(),
                country_code: candidate.country_code.clone(),
                assignment_rule,
                fallback_reason: None,
                reputation: candidate.reputation,
                development_level: candidate.development_level,
                acquisition_cost: acquisition_cost_for_candidate(candidate),
            })
        })
        .collect()
}

pub fn eligible_academy_creation_options(
    team_country_code: &str,
    catalog: &[ErlLeagueDefinition],
) -> Vec<AcademyCreationOption> {
    let domestic: Vec<_> = catalog
        .iter()
        .filter(|erl| country_matches(&erl.country_code, team_country_code))
        .map(|erl| acquisition_option_from_league(erl, ErlAssignmentRule::Domestic, None))
        .collect();

    if !domestic.is_empty() {
        return domestic;
    }

    catalog
        .iter()
        .filter(|erl| {
            erl.nearby_country_codes.is_empty()
                || erl.nearby_country_codes
                    .iter()
                    .any(|country| country_matches(country, team_country_code))
        })
        .map(|erl| {
            acquisition_option_from_league(
                erl,
                ErlAssignmentRule::Fallback,
                Some(format!(
                    "{} has no domestic ERL; {} is configured as nearby",
                    team_country_code, erl.id
                )),
            )
        })
        .collect()
}

pub fn validate_academy_creation(
    parent: &Team,
    option: &AcademyCreationOption,
) -> Result<(), AcademyError> {
    validate_academy_acquisition(parent, option)
}

pub fn validate_academy_acquisition(
    parent: &Team,
    option: &AcademyAcquisitionOption,
) -> Result<(), AcademyError> {
    if !parent.is_main() {
        return Err(AcademyError::ParentMustBeMainTeam {
            team_id: parent.id.clone(),
        });
    }

    if let Some(academy_team_id) = &parent.academy_team_id {
        return Err(AcademyError::AcademyAlreadyExists {
            parent_team_id: parent.id.clone(),
            academy_team_id: academy_team_id.clone(),
        });
    }

    if parent.finance < option.acquisition_cost {
        return Err(AcademyError::InsufficientFunds {
            available: parent.finance,
            required: option.acquisition_cost,
        });
    }

    Ok(())
}

pub fn validate_parent_academy_link(parent: &Team, academy: &Team) -> Result<(), AcademyError> {
    let linked_from_parent = parent.academy_team_id.as_deref() == Some(academy.id.as_str());
    let linked_from_academy = academy.parent_team_id.as_deref() == Some(parent.id.as_str());

    if academy.team_kind == TeamKind::Academy && (linked_from_parent || linked_from_academy) {
        return Ok(());
    }

    Err(AcademyError::UnrelatedAcademy {
        parent_team_id: parent.id.clone(),
        academy_team_id: academy.id.clone(),
    })
}

fn acquisition_option_from_league(
    erl: &ErlLeagueDefinition,
    assignment_rule: ErlAssignmentRule,
    fallback_reason: Option<String>,
) -> AcademyCreationOption {
    AcademyAcquisitionOption {
        source_team_id: format!("{}-academy-candidate", erl.id),
        name: erl.name.clone(),
        short_name: erl.id.clone(),
        logo_url: None,
        erl_league_id: erl.id.clone(),
        erl_league_name: erl.name.clone(),
        country_code: erl.country_code.clone(),
        assignment_rule,
        fallback_reason,
        reputation: erl.reputation,
        development_level: erl.reputation,
        acquisition_cost: acquisition_cost_for_reputation(erl.reputation),
    }
}

fn acquisition_cost_for_candidate(candidate: &ErlAcademyCandidate) -> i64 {
    BASE_ACADEMY_ACQUISITION_COST
        + i64::from(candidate.reputation) * REPUTATION_COST_MULTIPLIER
        + i64::from(candidate.development_level) * 20_000
}

fn acquisition_cost_for_reputation(reputation: u8) -> i64 {
    BASE_ACADEMY_ACQUISITION_COST + i64::from(reputation) * REPUTATION_COST_MULTIPLIER
}

fn country_matches(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

// ---------------------------------------------------------------------------
// Seed catalog — reads tier 2+ competitions and builds academy seeds
// ---------------------------------------------------------------------------

pub fn normalize_key(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect()
}

/// Generate a team ID for an academy seed team.
pub fn seed_team_id(league_id: &str, team_name: &str) -> String {
    let academy_id = format!("academy-{}-{}", league_id, slugify_key(team_name));
    if academy_id == format!("academy-{}-", league_id) {
        format!("academy-{}", league_id)
    } else {
        academy_id
    }
}

fn slugify_key(value: &str) -> String {
    let slug: String = value
        .to_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect();
    slug
        .trim_matches('-')
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn role_to_canonical(role: &str) -> String {
    match normalize_key(role).as_str() {
        "top" | "toplaner" => "top",
        "jungle" | "jungler" => "jungle",
        "mid" | "midlaner" | "middle" => "mid",
        "adc" | "bot" | "bottom" => "adc",
        "support" | "sup" => "support",
        _ => "mid",
    }
    .to_string()
}

/// Read tier 2+ competition manifests and build academy team seeds.
pub fn academy_seed_catalog() -> &'static Vec<AcademyTeamSeed> {
    static CATALOG: OnceLock<Vec<AcademyTeamSeed>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let cwd = match std::env::current_dir().ok() {
            Some(d) => d,
            None => return vec![],
        };
        let comps_dir = {
            let mut d = cwd.clone();
            d.push("data");
            d.push("competitions");
            if d.is_dir() { d }
            else {
                d = cwd;
                d.push("..");
                d.push("data");
                d.push("competitions");
                if d.is_dir() { d } else { return vec![] }
            }
        };

        let data_base = comps_dir.parent().and_then(|p| {
            let d = p.to_path_buf();
            if d.join("teams").is_dir() { Some(d) } else { None }
        }).unwrap_or_else(|| {
            let mut d = comps_dir.clone();
            d.pop();
            d
        });

        let entries = match std::fs::read_dir(&comps_dir) {
            Ok(e) => e,
            Err(_) => return vec![],
        };

        let mut teams = Vec::new();
        for entry in entries.flatten() {
            let dir_path = entry.path();
            if !dir_path.is_dir() { continue; }
            let league_id = match dir_path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };

            let manifest_path = dir_path.join("manifest.json");
            let manifest_json = match std::fs::read_to_string(&manifest_path) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let manifest: serde_json::Value = match serde_json::from_str(&manifest_json) {
                Ok(v) => v,
                Err(_) => continue,
            };

            if manifest["legacy"].as_bool().unwrap_or(false) { continue; }
            let tier = manifest["tier"].as_u64().unwrap_or(1);
            if tier <= 1 { continue; }

            let league_name = manifest["name"].as_str().unwrap_or(&league_id).to_string();
            let country_code = manifest["country"].as_str().unwrap_or("EU").to_string();

            info!("[academy] loading tier {} league: {} ({})", tier, league_name, league_id);

            let teams_path = data_base.join("teams").join(format!("{}_teams.json", league_id));
            let json_str = match std::fs::read_to_string(&teams_path) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let Some(teams_data) = serde_json::from_str::<serde_json::Value>(&json_str).ok() else { continue; };
            let Some(team_entries) = teams_data["teams"].as_array() else { continue; };

            let players_path = data_base.join("players").join(format!("{}_players.json", league_id));
            let mut players_by_team_id: HashMap<String, Vec<AcademyPlayerSeed>> = HashMap::new();
            if let Ok(players_json) = std::fs::read_to_string(&players_path) {
                if let Ok(players_data) = serde_json::from_str::<serde_json::Value>(&players_json) {
                    if let Some(all_players) = players_data["players"].as_array() {
                        for player in all_players {
                            if let Some(tid) = player["team_id"].as_str() {
                                let seed = AcademyPlayerSeed {
                                    role: player["position"].as_str().unwrap_or("Mid").to_string(),
                                    nickname: player["match_name"].as_str().unwrap_or("Unknown").to_string(),
                                    full_name: player["full_name"].as_str().unwrap_or("Unknown").to_string(),
                                    nationality: player["nationality"].as_str().unwrap_or("Unknown").to_string(),
                                    dob: player["date_of_birth"].as_str().map(|s| s.to_string()),
                                    image_url: player["profile_image_url"].as_str().unwrap_or("").to_string(),
                                };
                                players_by_team_id.entry(tid.to_string()).or_default().push(seed);
                            }
                        }
                    }
                }
            }

            for entry in team_entries {
                let team_id = entry["id"].as_str().unwrap_or("").to_string();
                let team_name = entry["name"].as_str().unwrap_or("Unknown").to_string();
                let short_name = entry["short_name"].as_str().unwrap_or("ACD").to_string();
                let logo_url = entry["logo_url"].as_str().map(|s| s.to_string());

                let seed_players = players_by_team_id.remove(&team_id).unwrap_or_default();
                teams.push(AcademyTeamSeed {
                    league_id: league_id.clone(),
                    league_name: league_name.clone(),
                    country_code: country_code.clone(),
                    team_name,
                    short_name,
                    logo_url,
                    players: seed_players,
                });
            }
        }
        teams
    })
}

// ---------------------------------------------------------------------------
// ERL league catalog — reads tier 2+ competition manifests
// ---------------------------------------------------------------------------

/// Scan `data/competitions/` for tier 2+ manifests and build ErlLeagueDefinition entries.
pub fn academy_erl_catalog() -> &'static [ErlLeagueDefinition] {
    static CATALOG: OnceLock<Vec<ErlLeagueDefinition>> = OnceLock::new();
    CATALOG.get_or_init(catalogs_from_tier2_manifests)
}

fn catalogs_from_tier2_manifests() -> Vec<ErlLeagueDefinition> {
    use crate::generator::definitions::CompetitionManifest;

    let cwd = match std::env::current_dir().ok() {
        Some(d) => d,
        None => return vec![],
    };
    let comps_dir = {
        let mut d = cwd.clone();
        d.push("data");
        d.push("competitions");
        if d.is_dir() { d }
        else {
            d = cwd;
            d.push("..");
            d.push("data");
            d.push("competitions");
            if d.is_dir() { d } else { return vec![] }
        }
    };

    let mut catalogs = Vec::new();
    let entries = match std::fs::read_dir(&comps_dir) {
        Ok(e) => e,
        Err(_) => return catalogs,
    };

    for entry in entries.flatten() {
        let dir_path = entry.path();
        if !dir_path.is_dir() { continue; }
        let manifest_path = dir_path.join("manifest.json");
        if !manifest_path.exists() { continue; }
        let league_id = match dir_path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };

        let json_str = match std::fs::read_to_string(&manifest_path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        if let Ok(manifest) = serde_json::from_str::<CompetitionManifest>(&json_str) {
            // Skip legacy competitions
            if manifest.legacy { continue; }
            // Only tier 2+ competitions are ERL / academy sources
            if manifest.tier.unwrap_or(1) <= 1 { continue; }

            let country_code = manifest.country.clone().unwrap_or_default();
            let region = manifest.region.clone();
            let reputation = manifest.reputation.unwrap_or(3);
            let nearby = manifest.nearby_country_codes.clone();

            catalogs.push(ErlLeagueDefinition {
                id: league_id,
                name: manifest.name,
                country_code,
                region,
                reputation,
                nearby_country_codes: nearby,
            });
        }
    }
    catalogs
}

/// Build ErlAcademyCandidate entries from the academy seed catalog + ERL league catalog.
pub fn academy_candidate_catalog() -> &'static [ErlAcademyCandidate] {
    static CATALOG: OnceLock<Vec<ErlAcademyCandidate>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        // Build a lookup: erl_league_id → (reputation, development_level) from manifests
        let erl_reputations: std::collections::HashMap<String, (u8, u8)> = academy_erl_catalog()
            .iter()
            .map(|erl| {
                let dev_level = match erl.reputation {
                    5 => 4,
                    4 => 3,
                    3 => 2,
                    _ => 1,
                };
                (erl.id.clone(), (erl.reputation, dev_level))
            })
            .collect();

        academy_seed_catalog()
            .iter()
            .map(|seed| {
                let (reputation, development_level) = erl_reputations
                    .get(&seed.league_id)
                    .copied()
                    .unwrap_or((3, 2));

                ErlAcademyCandidate {
                    source_team_id: seed_team_id(&seed.league_id, &seed.team_name),
                    name: seed.team_name.clone(),
                    short_name: seed.short_name.clone(),
                    logo_url: seed.logo_url.clone(),
                    erl_league_id: seed.league_id.clone(),
                    country_code: seed.country_code.clone(),
                    reputation,
                    development_level,
                }
            })
            .collect()
    })
}


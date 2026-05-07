use serde::{Deserialize, Serialize};
#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct IncludedPlayerEntry {
    pub player_id: String,
    pub player_name: String,
    pub player_ovr: u8,
    pub player_position: String,
    pub player_profile_image_url: Option<String>,
    pub valuation: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct TransferHistoryEntry {
    pub id: String,
    pub player_id: String,
    pub player_name: String,
    pub player_ovr: u8,
    pub player_position: String,
    pub player_profile_image_url: Option<String>,
    pub from_team_id: String,
    pub from_team_name: String,
    pub to_team_id: String,
    pub to_team_name: String,
    pub fee: u64,
    pub annual_wage: u32,
    pub contract_years: u8,
    pub date: String,
    pub is_user_involved: bool,
    pub is_user_buying: bool,
    pub was_negotiated: bool,
    pub initial_offer_fee: Option<u64>,
    pub negotiation_rounds: u8,
    pub included_players: Vec<IncludedPlayerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct TransferHistory {
    pub entries: Vec<TransferHistoryEntry>,
}

impl Default for TransferHistory {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

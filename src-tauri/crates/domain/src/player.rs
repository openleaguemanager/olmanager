use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub match_name: String,
    pub full_name: String,
    pub date_of_birth: String,
    pub nationality: String,
    #[serde(default)]
    pub football_nation: String,
    #[serde(default)]
    pub birth_country: Option<String>,
    #[serde(default)]
    pub profile_image_url: Option<String>,

    pub position: Position,

    // The player's natural/preferred position (never changed by formation logic)
    #[serde(default)]
    pub natural_position: Position,

    // Alternate positions this player can also play (with reduced effectiveness)
    #[serde(default)]
    pub alternate_positions: Vec<Position>,

    #[serde(default)]
    pub footedness: Footedness,

    #[serde(default = "default_weak_foot")]
    pub weak_foot: u8,

    // Core attributes 0-100
    pub attributes: PlayerAttributes,

    // Dynamic match/season values
    pub condition: u8, // 0-100 (short-term energy; depletes during matches, recovers daily)
    pub morale: u8,    // 0-100
    /// Long-term physical shape (0–100). Determines how fast condition depletes and
    /// recovers, and modulates injury risk. Changes slowly over weeks.
    #[serde(default = "default_fitness")]
    pub fitness: u8,

    pub injury: Option<Injury>,
    pub team_id: Option<String>,

    // Traits / flairs derived from attributes
    #[serde(default)]
    pub traits: Vec<PlayerTrait>,

    // Contract & value
    pub contract_end: Option<String>,
    pub wage: u32, // weekly wage
    pub market_value: u64,

    // Season stats
    pub stats: PlayerSeasonStats,

    // Career history
    pub career: Vec<CareerEntry>,

    // Individual training focus override (takes priority over group and team default)
    #[serde(default)]
    pub training_focus: Option<crate::team::TrainingFocus>,

    // Transfer status
    #[serde(default)]
    pub transfer_listed: bool,
    #[serde(default)]
    pub loan_listed: bool,
    #[serde(default)]
    pub transfer_offers: Vec<TransferOffer>,
    #[serde(default)]
    pub morale_core: PlayerMoraleCore,
    #[serde(default = "default_potential_base")]
    pub potential_base: u8,
    #[serde(default)]
    pub potential_revealed: Option<u8>,
    #[serde(default)]
    pub potential_research_started_on: Option<String>,
    #[serde(default)]
    pub potential_research_eta_days: Option<u8>,
    #[serde(default)]
    pub champion_training_target: Option<String>,
    #[serde(default)]
    pub champion_training_targets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Position {
    #[default]
    Goalkeeper,
    Defender,
    Midfielder,
    Forward,
    RightBack,
    CenterBack,
    LeftBack,
    RightWingBack,
    LeftWingBack,
    DefensiveMidfielder,
    CentralMidfielder,
    AttackingMidfielder,
    RightMidfielder,
    LeftMidfielder,
    RightWinger,
    LeftWinger,
    Striker,
}

impl Position {
    pub fn is_legacy_bucket(&self) -> bool {
        matches!(
            self,
            Position::Goalkeeper | Position::Defender | Position::Midfielder | Position::Forward
        )
    }

    pub fn to_group_position(&self) -> Position {
        match self {
            Position::Goalkeeper => Position::Goalkeeper,
            Position::Defender
            | Position::RightBack
            | Position::CenterBack
            | Position::LeftBack
            | Position::RightWingBack
            | Position::LeftWingBack => Position::Defender,
            Position::Midfielder
            | Position::DefensiveMidfielder
            | Position::CentralMidfielder
            | Position::AttackingMidfielder
            | Position::RightMidfielder
            | Position::LeftMidfielder => Position::Midfielder,
            Position::Forward
            | Position::RightWinger
            | Position::LeftWinger
            | Position::Striker => Position::Forward,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Footedness {
    Left,
    #[default]
    Right,
    Both,
}

/// Player attributes for League of Legends themed manager.
/// Replaces 19 football-specific attributes with 9 LoL stats.
/// Uses serde aliases for backward compatibility with legacy save files.
#[derive(Debug, Clone, Serialize)]
pub struct PlayerAttributes {
    /// Technical skill and champion execution (formerly dribbling)
    #[serde(alias = "dribbling", default = "default_attr")]
    pub mechanics: u8,

    /// 1v1 and 2v2 lane phase performance (formerly shooting)
    #[serde(alias = "shooting", default = "default_attr")]
    pub laning: u8,

    /// Coordination in 5v5 engagements (formerly teamwork)
    #[serde(alias = "teamwork", default = "default_attr")]
    pub teamfighting: u8,

    /// Map awareness and objective control (formerly vision)
    #[serde(alias = "vision", default = "default_attr")]
    pub macro_play: u8,

    /// Performance stability across games (formerly decisions)
    #[serde(alias = "decisions", default = "default_attr")]
    pub consistency: u8,

    /// In-game leadership and calls (formerly leadership)
    #[serde(alias = "leadership", default = "default_attr")]
    pub shotcalling: u8,

    /// Champion versatility and mastery (formerly agility)
    #[serde(alias = "agility", default = "default_attr")]
    pub champion_pool: u8,

    /// Focus and tilt resistance (formerly composure)
    #[serde(alias = "composure", default = "default_attr")]
    pub discipline: u8,

    /// Pressure handling and recovery (formerly stamina)
    #[serde(alias = "stamina", default = "default_attr")]
    pub mental_resilience: u8,
}

/// Legacy 19-field attribute structure for backward compatibility deserialization
#[derive(Debug, Clone, Deserialize)]
struct LegacyAttributes {
    // Physical (4)
    #[serde(default)]
    pace: Option<u8>,
    stamina: Option<u8>,
    strength: Option<u8>,
    agility: Option<u8>,

    // Technical (5)
    passing: Option<u8>,
    shooting: Option<u8>,
    tackling: Option<u8>,
    dribbling: Option<u8>,
    defending: Option<u8>,

    // Mental (7)
    positioning: Option<u8>,
    vision: Option<u8>,
    decisions: Option<u8>,
    composure: Option<u8>,
    aggression: Option<u8>,
    teamwork: Option<u8>,
    leadership: Option<u8>,

    // Goalkeeper (3)
    handling: Option<u8>,
    reflexes: Option<u8>,
    aerial: Option<u8>,

    // New LoL fields (for forward compatibility)
    mechanics: Option<u8>,
    laning: Option<u8>,
    teamfighting: Option<u8>,
    macro_play: Option<u8>,
    consistency: Option<u8>,
    shotcalling: Option<u8>,
    champion_pool: Option<u8>,
    discipline: Option<u8>,
    mental_resilience: Option<u8>,
}

impl<'de> serde::Deserialize<'de> for PlayerAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let legacy = LegacyAttributes::deserialize(deserializer)?;

        // If new format present (all 9 fields), use directly
        if let (
            Some(m),
            Some(l),
            Some(t),
            Some(mp),
            Some(c),
            Some(s),
            Some(cp),
            Some(d),
            Some(mr),
        ) = (
            legacy.mechanics,
            legacy.laning,
            legacy.teamfighting,
            legacy.macro_play,
            legacy.consistency,
            legacy.shotcalling,
            legacy.champion_pool,
            legacy.discipline,
            legacy.mental_resilience,
        ) {
            return Ok(PlayerAttributes {
                mechanics: m,
                laning: l,
                teamfighting: t,
                macro_play: mp,
                consistency: c,
                shotcalling: s,
                champion_pool: cp,
                discipline: d,
                mental_resilience: mr,
            });
        }

        // Otherwise, map from legacy format
        // Mapping table from design.md:
        // pace + dribbling -> mechanics
        // shooting -> laning
        // teamwork -> teamfighting
        // vision -> macro_play
        // decisions -> consistency
        // leadership -> shotcalling
        // agility -> champion_pool
        // composure -> discipline
        // stamina -> mental_resilience

        let mechanics = match (legacy.pace, legacy.dribbling) {
            (Some(p), Some(d)) => (p + d) / 2,
            (Some(p), None) => p,
            (None, Some(d)) => d,
            (None, None) => 50,
        };

        let laning = legacy.shooting.unwrap_or(50);
        let teamfighting = legacy.teamwork.unwrap_or(50);
        let macro_play = legacy.vision.unwrap_or(50);
        let consistency = legacy.decisions.unwrap_or(50);
        let shotcalling = legacy.leadership.unwrap_or(50);
        let champion_pool = legacy.agility.unwrap_or(50);
        let discipline = legacy.composure.unwrap_or(50);
        let mental_resilience = legacy.stamina.unwrap_or(50);

        Ok(PlayerAttributes {
            mechanics,
            laning,
            teamfighting,
            macro_play,
            consistency,
            shotcalling,
            champion_pool,
            discipline,
            mental_resilience,
        })
    }
}

impl Default for PlayerAttributes {
    fn default() -> Self {
        PlayerAttributes {
            mechanics: 50,
            laning: 50,
            teamfighting: 50,
            macro_play: 50,
            consistency: 50,
            shotcalling: 50,
            champion_pool: 50,
            discipline: 50,
            mental_resilience: 50,
        }
    }
}

fn default_attr() -> u8 {
    50
}

fn default_weak_foot() -> u8 {
    2
}

fn default_fitness() -> u8 {
    75
}

fn default_potential_base() -> u8 {
    99
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Injury {
    pub name: String,
    pub days_remaining: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerIssueCategory {
    Contract,
    PlayingTime,
    Morale,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayerIssue {
    pub category: PlayerIssueCategory,
    pub severity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct RecentTreatmentMemory {
    pub action_key: String,
    pub times_recently_used: u8,
}

impl Default for RecentTreatmentMemory {
    fn default() -> Self {
        Self {
            action_key: String::new(),
            times_recently_used: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerPromiseKind {
    PlayingTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RenewalSessionStatus {
    #[default]
    Idle,
    Open,
    Agreed,
    Blocked,
    Stalled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RenewalSessionOutcome {
    #[default]
    None,
    AcceptedByManager,
    AcceptedByAssistant,
    RejectedByPlayer,
    BlockedByManager,
    Stalled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ContractRenewalState {
    pub status: RenewalSessionStatus,
    pub manager_blocked_until: Option<String>,
    pub last_attempt_date: Option<String>,
    pub last_assistant_attempt_date: Option<String>,
    pub last_outcome: Option<RenewalSessionOutcome>,
    pub conversation_round: u8,
}

impl Default for ContractRenewalState {
    fn default() -> Self {
        Self {
            status: RenewalSessionStatus::Idle,
            manager_blocked_until: None,
            last_attempt_date: None,
            last_assistant_attempt_date: None,
            last_outcome: None,
            conversation_round: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PlayerPromise {
    pub kind: PlayerPromiseKind,
    pub matches_remaining: u8,
}

impl Default for PlayerPromise {
    fn default() -> Self {
        Self {
            kind: PlayerPromiseKind::PlayingTime,
            matches_remaining: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PlayerMoraleCore {
    pub manager_trust: u8,
    pub unresolved_issue: Option<PlayerIssue>,
    pub recent_treatment: Option<RecentTreatmentMemory>,
    pub pending_promise: Option<PlayerPromise>,
    pub talk_cooldown_until: Option<String>,
    pub renewal_state: Option<ContractRenewalState>,
}

impl Default for PlayerMoraleCore {
    fn default() -> Self {
        Self {
            manager_trust: 50,
            unresolved_issue: None,
            recent_treatment: None,
            pending_promise: None,
            talk_cooldown_until: None,
            renewal_state: None,
        }
    }
}

fn default_transfer_offer_status() -> TransferOfferStatus {
    TransferOfferStatus::Pending
}

fn default_transfer_offer_date() -> String {
    String::new()
}

fn default_transfer_offer_round() -> u8 {
    0
}

fn default_transfer_offer_destination_team_id() -> Option<String> {
    None
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PlayerSeasonStats {
    pub appearances: u32,
    pub goals: u32,
    pub assists: u32,
    pub clean_sheets: u32,
    pub yellow_cards: u32,
    pub red_cards: u32,
    pub avg_rating: f32,
    pub minutes_played: u32,
    pub shots: u32,
    pub shots_on_target: u32,
    pub passes_completed: u32,
    pub passes_attempted: u32,
    pub tackles_won: u32,
    pub interceptions: u32,
    pub fouls_committed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareerEntry {
    pub season: u32,
    pub team_id: String,
    pub team_name: String,
    pub appearances: u32,
    pub goals: u32,
    pub assists: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOffer {
    pub id: String,
    pub from_team_id: String,
    #[serde(default = "default_transfer_offer_destination_team_id")]
    pub destination_team_id: Option<String>,
    pub fee: u64,
    pub wage_offered: u32,
    #[serde(default)]
    pub last_manager_fee: Option<u64>,
    #[serde(default = "default_transfer_offer_round")]
    pub negotiation_round: u8,
    #[serde(default)]
    pub suggested_counter_fee: Option<u64>,
    #[serde(default = "default_transfer_offer_status")]
    pub status: TransferOfferStatus,
    #[serde(default = "default_transfer_offer_date")]
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferOfferStatus {
    Pending,
    Accepted,
    Rejected,
    Withdrawn,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerTrait {
    // Physical
    Speedster, // pace >= 85
    Tank,      // strength >= 85 && stamina >= 75
    Agile,     // agility >= 85
    Tireless,  // stamina >= 90
    // Technical
    Playmaker,    // passing >= 80 && vision >= 80
    Sharpshooter, // shooting >= 85
    Dribbler,     // dribbling >= 85
    BallWinner,   // tackling >= 80 && aggression >= 70
    Rock,         // defending >= 85 && positioning >= 75
    // Mental
    Leader,     // leadership >= 85 && teamwork >= 75
    CoolHead,   // composure >= 85 && decisions >= 80
    Visionary,  // vision >= 85
    HotHead,    // aggression >= 85 && composure < 50
    TeamPlayer, // teamwork >= 85
    // Goalkeeper
    SafeHands,       // handling >= 85 (GK only)
    CatReflexes,     // reflexes >= 85 (GK only)
    AerialDominance, // aerial >= 85
    // Combo / Special
    CompleteForward, // FWD: shooting >= 75 && dribbling >= 75 && pace >= 70 && strength >= 70
    Engine,          // MID: stamina >= 85 && pace >= 70 && teamwork >= 75
    SetPieceSpecialist, // passing >= 80 && shooting >= 75 && vision >= 75
}

/// Derive traits purely from a player's LoL attributes.
/// Maps from football attribute conditions to LoL stats:
/// - Speedster: pace >= 85 -> mechanics >= 85
/// - Tank: strength >= 85 && stamina >= 75 -> teamfighting >= 85 && mental_resilience >= 75
/// - Agile: agility >= 85 -> champion_pool >= 85
/// - Tireless: stamina >= 90 -> mental_resilience >= 90
/// - Playmaker: passing >= 80 && vision >= 80 -> macro_play >= 80 && shotcalling >= 80
/// - Sharpshooter: shooting >= 85 -> laning >= 85
/// - Dribbler: dribbling >= 85 -> mechanics >= 85
/// - BallWinner: tackling >= 80 && aggression >= 70 -> discipline >= 80 && teamfighting >= 70
/// - Rock: defending >= 85 && positioning >= 75 -> teamfighting >= 85 && macro_play >= 75
/// - Leader: leadership >= 85 && teamwork >= 75 -> shotcalling >= 85 && teamfighting >= 75
/// - CoolHead: composure >= 85 && decisions >= 80 -> discipline >= 85 && consistency >= 80
/// - Visionary: vision >= 85 -> macro_play >= 85
/// - HotHead: aggression >= 85 && composure < 50 -> low discipline, high teamfighting
/// - TeamPlayer: teamwork >= 85 -> teamfighting >= 85
/// - CompleteForward: shooting >= 75 && dribbling >= 75 && pace >= 70 && strength >= 70 -> mechanics >= 75 && laning >= 75 && champion_pool >= 70
/// - Engine: stamina >= 85 && pace >= 70 && teamwork >= 75 -> mental_resilience >= 85 && mechanics >= 70 && teamfighting >= 75
/// - SetPieceSpecialist: passing >= 80 && shooting >= 75 && vision >= 75 -> macro_play >= 80 && laning >= 75 && shotcalling >= 75
pub fn compute_traits(attrs: &PlayerAttributes, _position: &Position) -> Vec<PlayerTrait> {
    let mut traits = Vec::new();

    // Mechanical stats
    if attrs.mechanics >= 85 {
        traits.push(PlayerTrait::Speedster);
        traits.push(PlayerTrait::Dribbler);
    }

    // Teamfighting + Mental Resilience -> Tank
    if attrs.teamfighting >= 85 && attrs.mental_resilience >= 75 {
        traits.push(PlayerTrait::Tank);
    }

    // Champion Pool -> Agile
    if attrs.champion_pool >= 85 {
        traits.push(PlayerTrait::Agile);
    }

    // Mental Resilience -> Tireless
    if attrs.mental_resilience >= 90 {
        traits.push(PlayerTrait::Tireless);
    }

    // Macro Play + Shotcalling -> Playmaker
    if attrs.macro_play >= 80 && attrs.shotcalling >= 80 {
        traits.push(PlayerTrait::Playmaker);
    }

    // Laning -> Sharpshooter
    if attrs.laning >= 85 {
        traits.push(PlayerTrait::Sharpshooter);
    }

    // Discipline + Teamfighting -> BallWinner
    if attrs.discipline >= 80 && attrs.teamfighting >= 70 {
        traits.push(PlayerTrait::BallWinner);
    }

    // Teamfighting + Macro Play -> Rock
    if attrs.teamfighting >= 85 && attrs.macro_play >= 75 {
        traits.push(PlayerTrait::Rock);
    }

    // Shotcalling + Teamfighting -> Leader
    if attrs.shotcalling >= 85 && attrs.teamfighting >= 75 {
        traits.push(PlayerTrait::Leader);
    }

    // Discipline + Consistency -> CoolHead
    if attrs.discipline >= 85 && attrs.consistency >= 80 {
        traits.push(PlayerTrait::CoolHead);
    }

    // Macro Play -> Visionary
    if attrs.macro_play >= 85 {
        traits.push(PlayerTrait::Visionary);
    }

    // HotHead: High teamfighting + low discipline
    if attrs.teamfighting >= 75 && attrs.discipline < 50 {
        traits.push(PlayerTrait::HotHead);
    }

    // Teamfighting -> TeamPlayer
    if attrs.teamfighting >= 85 {
        traits.push(PlayerTrait::TeamPlayer);
    }

    // Combo traits
    // CompleteForward: mechanics >= 75 && laning >= 75 && champion_pool >= 70
    if attrs.mechanics >= 75 && attrs.laning >= 75 && attrs.champion_pool >= 70 {
        traits.push(PlayerTrait::CompleteForward);
    }

    // Engine: mental_resilience >= 85 && mechanics >= 70 && teamfighting >= 75
    if attrs.mental_resilience >= 85 && attrs.mechanics >= 70 && attrs.teamfighting >= 75 {
        traits.push(PlayerTrait::Engine);
    }

    // SetPieceSpecialist: macro_play >= 80 && laning >= 75 && shotcalling >= 75
    if attrs.macro_play >= 80 && attrs.laning >= 75 && attrs.shotcalling >= 75 {
        traits.push(PlayerTrait::SetPieceSpecialist);
    }

    traits
}

impl Player {
    pub fn new(
        id: String,
        match_name: String,
        full_name: String,
        date_of_birth: String,
        nationality: String,
        position: Position,
        attributes: PlayerAttributes,
    ) -> Self {
        let traits = compute_traits(&attributes, &position);
        let football_nation = crate::identity::normalize_football_nation_code(&nationality);
        let birth_country = crate::identity::derive_birth_country_code(&nationality);
        Self {
            id,
            match_name,
            full_name,
            date_of_birth,
            nationality,
            football_nation,
            birth_country,
            profile_image_url: None,
            natural_position: position.clone(),
            position,
            alternate_positions: Vec::new(),
            footedness: Footedness::default(),
            weak_foot: default_weak_foot(),
            attributes,
            condition: 100,
            morale: 100,
            fitness: 75,
            injury: None,
            team_id: None,
            traits,
            contract_end: None,
            wage: 0,
            market_value: 0,
            stats: PlayerSeasonStats::default(),
            career: Vec::new(),
            training_focus: None,
            transfer_listed: false,
            loan_listed: false,
            transfer_offers: Vec::new(),
            morale_core: PlayerMoraleCore::default(),
            potential_base: default_potential_base(),
            potential_revealed: None,
            potential_research_started_on: None,
            potential_research_eta_days: None,
            champion_training_target: None,
            champion_training_targets: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_attributes() -> PlayerAttributes {
        PlayerAttributes {
            mechanics: 70,
            laning: 72,
            teamfighting: 65,
            macro_play: 68,
            consistency: 74,
            shotcalling: 61,
            champion_pool: 58,
            discipline: 69,
            mental_resilience: 56,
        }
    }

    #[test]
    fn player_new_defaults_footedness_and_weak_foot() {
        let player = Player::new(
            "p-001".to_string(),
            "J. Smith".to_string(),
            "John Smith".to_string(),
            "2000-01-15".to_string(),
            "GB".to_string(),
            Position::Midfielder,
            sample_attributes(),
        );

        assert_eq!(player.footedness, Footedness::Right);
        assert_eq!(player.weak_foot, 2);
    }

    #[test]
    fn position_group_conversion_maps_granular_positions_back_to_legacy_groups() {
        assert_eq!(Position::RightBack.to_group_position(), Position::Defender);
        assert_eq!(
            Position::AttackingMidfielder.to_group_position(),
            Position::Midfielder,
        );
        assert_eq!(Position::LeftWinger.to_group_position(), Position::Forward);
    }

    #[test]
    fn player_deserialization_defaults_missing_foot_fields() {
        let player: Player = serde_json::from_value(serde_json::json!({
            "id": "p-legacy",
            "match_name": "J. Legacy",
            "full_name": "John Legacy",
            "date_of_birth": "2000-01-15",
            "nationality": "GB",
            "position": "Midfielder",
            "natural_position": "Midfielder",
            "alternate_positions": [],
            "attributes": sample_attributes(),
            "condition": 100,
            "morale": 100,
            "injury": null,
            "team_id": null,
            "traits": [],
            "contract_end": null,
            "wage": 0,
            "market_value": 0,
            "stats": {},
            "career": [],
            "transfer_listed": false,
            "loan_listed": false,
            "transfer_offers": [],
            "morale_core": {}
        }))
        .expect("legacy player json should deserialize");

        assert_eq!(player.footedness, Footedness::Right);
        assert_eq!(player.weak_foot, 2);
        assert_eq!(player.natural_position, Position::Midfielder);
        assert_eq!(player.potential_base, 99);
        assert_eq!(player.potential_revealed, None);
    }
}

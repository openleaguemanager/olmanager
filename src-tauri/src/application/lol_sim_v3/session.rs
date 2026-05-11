use std::collections::HashMap;
use std::sync::Mutex;

use super::LolSimV3WorldState;

#[derive(Default)]
pub struct LolSimV3StoreState {
    pub sessions: Mutex<HashMap<String, LolSimV3Session>>,
}

#[derive(Debug, Clone)]
pub struct LolSimV3Session {
    pub id: String,
    pub world: LolSimV3WorldState,
}

use tauri::State;

use crate::application::lol_sim_v3::{
    dispose, init, reset, run_to_completion, tick, LolSimV3DisposeRequest, LolSimV3DisposeResponse,
    LolSimV3InitRequest, LolSimV3ResetRequest, LolSimV3RunToCompletionRequest,
    LolSimV3RunToCompletionResponse, LolSimV3StoreState, LolSimV3TickRequest, LolSimV3TickResponse,
};

#[tauri::command]
pub fn lol_sim_v3_init(
    state: State<'_, LolSimV3StoreState>,
    request: LolSimV3InitRequest,
) -> Result<LolSimV3TickResponse, String> {
    init(&state, request)
}

#[tauri::command]
pub fn lol_sim_v3_tick(
    state: State<'_, LolSimV3StoreState>,
    request: LolSimV3TickRequest,
) -> Result<LolSimV3TickResponse, String> {
    tick(&state, request)
}

#[tauri::command]
pub fn lol_sim_v3_reset(
    state: State<'_, LolSimV3StoreState>,
    request: LolSimV3ResetRequest,
) -> Result<LolSimV3TickResponse, String> {
    reset(&state, request)
}

#[tauri::command]
pub fn lol_sim_v3_dispose(
    state: State<'_, LolSimV3StoreState>,
    request: LolSimV3DisposeRequest,
) -> Result<LolSimV3DisposeResponse, String> {
    dispose(&state, request)
}

#[tauri::command]
pub async fn lol_sim_v3_run_to_completion(
    state: State<'_, LolSimV3StoreState>,
    request: LolSimV3RunToCompletionRequest,
) -> Result<LolSimV3RunToCompletionResponse, String> {
    run_to_completion(&state, request)
}

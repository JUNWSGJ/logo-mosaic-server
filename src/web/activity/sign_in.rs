use std::sync::Arc;
use anyhow::Result;
use axum::{extract::State,Json};
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::{ ActivityRepo, ApiError, ApiResponse, AppState};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitySignInReq {
    pub activity_id: String,
    pub seq: String,
} 


/// 活动签到
pub async fn activity_sign_in_handler(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<ActivitySignInReq>,
) -> Result<ApiResponse<()>, ApiError> {
    info!("activity_sign_in, activiti_id: {}, seq: {}", req.activity_id, req.seq);
    app_state.activity_repo.mark_grid_of_activity(req.activity_id.as_str(), req.seq.as_str())
        .map_err(|e| ApiError::BizError("MARK_GRID_FAILED".into(), format!("mark grid failed, activity_id: {}, seq: {}, error: {}", req.activity_id, req.seq, e)))?;

    Ok(ApiResponse::ok(()))
}
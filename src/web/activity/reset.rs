use std::sync::Arc;
use anyhow::Result;
use axum::extract::{Query, State};
use serde::Deserialize;
use tracing::info;
use crate::{ActivityRepo, ApiError, ApiResponse, AppState};


#[derive(Deserialize)]
pub struct ActivityResetReq {
    pub id: String,
}

/// 活动信息重置
pub async fn activity_reset_in_handler(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<ActivityResetReq>
) -> Result<ApiResponse<()>, ApiError> {
    info!("reset activity");
    app_state.activity_repo.reset_activity(req.id.as_str())?;
    Ok(ApiResponse::ok(()))
}


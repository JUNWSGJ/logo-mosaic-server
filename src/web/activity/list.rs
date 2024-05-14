use std::sync::Arc;
use anyhow::Result;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use crate::{ActivityRepo, ApiError, ApiResponse, AppState};



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityListReply {
    pub activities: Vec<ActivityInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityInfo {
    pub id: String,
    pub name: String,
}


pub async fn activity_list_handler(
    State(app_state): State<Arc<AppState>>) -> Result<ApiResponse<ActivityListReply>, ApiError> {
    let acticities = app_state.activity_repo.list_activities();
    let activities = acticities.iter().map(|activity| ActivityInfo{
        id: activity.id.clone(),
        name: activity.name.clone(),
    }).collect();
    let reply = ActivityListReply{
        activities,
    };
    Ok(ApiResponse::ok(reply))

}

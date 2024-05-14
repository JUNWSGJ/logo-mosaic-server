use std::sync::Arc;
use anyhow::Result;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{ActivityDO, ActivityGridDO, ActivityRepo, ApiError, ApiResponse, AppState, GridShape, Point};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityCreateReq {
    pub name: String,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub canvas_color: String,
    pub grids: Vec<ActivityGrid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityGrid {
    pub seq: String,
    pub points: Vec<Point>,
    pub shape: GridShape,
    pub marked: bool,
    pub marked_color: String,
    pub unmarked_color: String,

}



pub async fn activity_create_handler(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<ActivityCreateReq>) -> Result<ApiResponse<()>, ApiError> {
    let activity_id = Uuid::new_v4().to_string();
    app_state.activity_repo.insert_activity(ActivityDO{
        id: activity_id.clone(),
        name: req.name.clone(),
        canvas_width: req.canvas_width,
        canvas_height: req.canvas_height,
        canvas_color: req.canvas_color,
        grids: req.grids.iter().map(|grid| ActivityGridDO{
            seq: grid.seq.clone(),
            points: grid.points.clone(),
            shape: grid.shape.clone(),
            marked: grid.marked,
            marked_color: grid.marked_color.clone(),
            unmarked_color: grid.unmarked_color.clone(),
        }).collect(),
    })?;
    Ok(ApiResponse::ok(()))
}

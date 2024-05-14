use std::sync::Arc;
use anyhow::Result;
use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
use crate::{ActivityDO, ActivityRepo, ApiError, ApiResponse, AppState, GridShape, Point};



#[derive(Deserialize)]
pub struct ActivityDetailQueryReq {
    pub id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDetailReply{

    pub id: String,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub canvas_color: String,
    pub grids: Vec<ActivityGrid>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityGrid {
    pub seq: String,
    pub points: Vec<Point>,
    pub shape: GridShape,
    pub marked: bool,
    pub marked_color: String,
    pub unmarked_color: String,

}
 

pub async fn activity_detail_handler(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<ActivityDetailQueryReq>,
) -> Result<ApiResponse<ActivityDetailReply>, ApiError> {

    let activity = app_state.activity_repo.get_activity(req.id.as_str());
    if activity.is_none() {
        return Err(ApiError::BizError("ACTIVITY_NOT_FOUND".into(), format!("activity not found, id: {}", req.id)));
    }
    let activity = activity.unwrap();

    Ok(ApiResponse::ok(to_activity_detail_reply(activity)))
}

fn to_activity_detail_reply(activity: ActivityDO) -> ActivityDetailReply {

    let grids = activity.grids.iter().map(|grid| ActivityGrid{
        seq: grid.seq.clone(),
        points: grid.points.clone(),
        shape: grid.shape.clone(),
        marked: grid.marked,
        marked_color: grid.marked_color.clone(),
        unmarked_color: grid.unmarked_color.clone(),
    }).collect();

    ActivityDetailReply{
        id: activity.id.clone(),
        canvas_width: activity.canvas_width,
        canvas_height: activity.canvas_height,
        canvas_color: activity.canvas_color.clone(),
        grids,
    }
}
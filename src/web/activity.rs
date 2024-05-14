use std::sync::Arc;

use axum::{extract::{Query, State}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;
use crate::{ActivityDO, ActivityGridDO, ActivityRepo, ApiError, ApiResponse, AppState, GridShape, Point};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivitySignInReq {
    pub activity_id: String,
    pub seq: String,
} 


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MosaicGrid {
    // 序号
    pub seq: String,
    // 点坐标
    pub points: Vec<Point>,
    // 形状
    pub shape: GridShape,
    // 颜色
    pub color: String,
    // 是否标记
    pub marked: bool,
}


#[derive(Deserialize)]
struct ActivityDetailQueryReq {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivityDetailReply{

    pub id: String,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub canvas_color: String,
    pub grids: Vec<MosaicGrid>,
}



async fn activity_detail_handler(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<ActivityDetailQueryReq>,
) -> Result<ApiResponse<ActivityDetailReply>, ApiError> {

    
    Ok(ApiResponse::ok(ActivityDetailReply{
        id: req.id.clone(),
        canvas_width: 1000,
        canvas_height: 800,
        canvas_color: "#373737".to_string(),
        grids: vec![
            MosaicGrid{
                seq: "1".to_string(),
                points: vec![Point{x: 25, y: 40}, Point{x: 75, y: 40}, Point{x: 50, y: 0}],
                shape: GridShape::Triangle,
                color: "#9099A2".to_string(),
                marked: false,
            },
            MosaicGrid{
                seq: "2".to_string(),
                points: vec![Point{x: 75, y: 40}, Point{x: 125, y: 40}, Point{x: 100, y: 0}],
                shape: GridShape::Triangle,
                color: "#00ff00".to_string(),
                marked: false,
            },
            MosaicGrid{
                seq: "3".to_string(),
                points: vec![Point{x: 75, y: 40}, Point{x: 125, y: 40}, Point{x: 100, y: 80}],
                shape: GridShape::Triangle,
                color: "#0000ff".to_string(),
                marked: false,
            },
        ]
    }))
}


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


async fn activity_list_handler(
    State(app_state): State<Arc<AppState>>,
) -> Result<ApiResponse<ActivityListReply>, ApiError> {
    let acticities = app_state.activity_repo.list_activities();
    let activities = acticities.iter().map(|activity| ActivityInfo{
        id: activity.id.clone(),
        name: activity.name.clone(),
    }).collect();
    Ok(ApiResponse::ok(ActivityListReply{
        activities,
    }))

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivityCreateReq {
    pub name: String,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub canvas_color: String,
    pub grids: Vec<ActivityGrid>,
}

#[derive(Debug, Clone, Deserialize)]
struct ActivityGrid {
    pub seq: String,
    pub points: Vec<Point>,
    pub shape: GridShape,
    pub marked: bool,
    pub marked_color: String,
    pub unmarked_color: String,

}



async fn activity_create_handler(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<ActivityCreateReq>,
) -> Result<ApiResponse<String>, ApiError> {
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
    Ok(ApiResponse::ok(activity_id))

}



/// 活动签到
async fn activity_sign_in_handler(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<ActivitySignInReq>,
) -> Result<ApiResponse<()>, ApiError> {
    info!("activity_sign_in, activiti_id: {}, seq: {}", req.activity_id, req.seq);
    Ok(ApiResponse::ok(()))
}

/// 活动信息重置
async fn activity_reset_in_handler(
    State(app_state): State<Arc<AppState>>,
    Query(req): Query<ActivityDetailQueryReq>
) -> Result<ApiResponse<()>, ApiError> {
    info!("reset activity");
    app_state.activity_repo.reset_activity(req.id.as_str())?;
    Ok(ApiResponse::ok(()))
}

pub fn activity_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/list", get(activity_list_handler))
        .route("/detail", get(activity_detail_handler))
        .route("/create", post(activity_create_handler))
        .route("/signIn", post(activity_sign_in_handler))
        .route("/reset", get(activity_reset_in_handler))

}

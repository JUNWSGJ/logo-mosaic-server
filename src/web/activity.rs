use std::sync::Arc;

use axum::{extract::{Query, State}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use crate::{ApiError, ApiResponse, AppState, Point};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MosaicGrid {
    // 序号
    pub seq: String,
    // 点坐标
    pub points: Vec<Point>,
    // 形状
    pub shape: String,
    // 颜色
    pub color: String,
    // 是否标记
    pub marked: bool,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActivityDetailReply{

    pub id: String,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub grids: Vec<MosaicGrid>,

}


async fn activity_detail_handler(
    State(app_state): State<Arc<AppState>>,
    Query(id): Query<String>,
) -> Result<ApiResponse<ActivityDetailReply>, ApiError> {

    let mut images = Vec::new();
    app_state.image_map.iter().for_each(|item| {
        images.push(item.value().clone());
    });
    
    Ok(ApiResponse::ok(ActivityDetailReply{
        id: id.clone(),
        canvas_width: 1000,
        canvas_height: 800,
        grids: vec![],
    }))
}







pub fn activity_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/detail", get(activity_detail_handler))

}

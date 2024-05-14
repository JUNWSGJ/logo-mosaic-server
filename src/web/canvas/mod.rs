
use std::sync::Arc;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::{generate_enmty_canvas_grids, ApiError, ApiResponse, AppState, GridFillOptions, GridShape, Point};


/// 将logo图片转换为canvas上马赛克的形状
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCanvasGridsReq{
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub grid_shape: GridShape,
    pub grid_size: (u32, u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCanvasGridsReply{

    pub grids: Vec<MosaicGrid>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicGrid {
    // 序号
    pub seq: String,
    // 点坐标
    pub points: Vec<Point>,
    // 形状
    pub shape: GridShape,
    // 是否选中
    pub selected: bool,
    // 初始颜色
    pub unmarked_color: String,
    // 标记颜色
    pub marked_color: String,

}


pub async fn get_canvas_grids_handler(
    Json(req): Json<GetCanvasGridsReq>,
) -> Result<ApiResponse<GetCanvasGridsReply>, ApiError> {
    let (w, h) = req.grid_size;
    let options = match req.grid_shape {
        GridShape::Rectangle => {
            GridFillOptions::Rectangle(w, h)
        }
        GridShape::Triangle => {
            GridFillOptions::Triangle(w, h)
        }
    };
    
    let grids = generate_enmty_canvas_grids(
        req.canvas_width, req.canvas_height, options
    ).map_err(|e| ApiError::BizError("GENERATE_CANVAS_GRIDS_ERROR".to_string(), e.to_string()))?;
    let grids = grids.into_iter().map(|grid| MosaicGrid{
        seq: grid.seq,
        points: grid.points,
        shape: grid.shape,
        selected: false,
        unmarked_color: "#9099A2ff".to_string(),
        marked_color: "#ff0000ff".to_string(),
    }).collect();
    
    let reply = GetCanvasGridsReply{
        grids
    };
    Ok(ApiResponse::ok(reply))
}


pub fn canvas_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/generate_canvas_grids", post(get_canvas_grids_handler))
}


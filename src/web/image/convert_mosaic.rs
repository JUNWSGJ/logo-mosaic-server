use std::{str::FromStr, sync::Arc};

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{generate_canvas_grids_by_image_path, ApiError, ApiResponse, AppState, AvgColorCompareParam, Color, EliminateBgColorParam, GridFillOptions, GridPickCmd, GridPickStrategy, GridShape, ImageRepo, Point};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicGridsConvertReq{
    pub image_id: String,
    pub grid_shape: GridShape,
    pub grid_size: Vec<u32>,
    pub grid_pick_strategy: GridPickStrategy,
    pub grid_pick_options: GridPickOptions,
    pub grid_selected_color: String,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridPickOptions{

    pub color_distance_range:Option<(u8, u8)>,
    pub remaining_ratio: Option<f32>,
    pub target_color: Option<String>,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MosaicGridsConvertReply {
    // 画布宽度
    pub canvas_width: u32,
    // 画布高度
    pub canvas_height: u32,
    // 网格信息
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
    pub shape: String,
    // 是否选中
    pub selected: bool,
    pub color: String,
    pub avg_color: Option<String>,
    pub color_distance: Option<f32>,
    pub remaining_area_ratio: Option<f32>,
}


/// 给定图片和参数，给出多边形马赛克填充的canvas数据
pub async fn convert_to_mosaic_grids(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<MosaicGridsConvertReq>,
) -> Result<ApiResponse<MosaicGridsConvertReply>, ApiError> {

    info!("convert image into mosaic grids, req: {:?}", req);
    let image_id = req.image_id;
    let image_info;

    match app_state.image_repo.get_image(image_id.as_str()) {
        Some(image) => image_info = image,
        None => return Err(ApiError::BizError("IMAGE_NOT_FOUND".to_string(), "image not found".to_string())),
    }


    let fill_options = match req.grid_shape {
        GridShape::Triangle => {
            let grid_size =  req.grid_size;
            GridFillOptions::Triangle(grid_size[0], grid_size[1])

        },
        GridShape::Rectangle => {
            let grid_size =  req.grid_size;
            GridFillOptions::Rectangle(grid_size[0], grid_size[1])
        },
    };
    
    info!("fill_options: {:?}", fill_options);

    
    let pick_strategy = match req.grid_pick_strategy {
        GridPickStrategy::AvgColorCompare => {
            let color_str = req.grid_pick_options.target_color.as_ref().unwrap().as_str();
            let range = req.grid_pick_options.color_distance_range.unwrap();
            let min_distance = range.0  as f32;
            let max_distance = range.1  as f32;
            GridPickCmd::AvgColorCompare(AvgColorCompareParam{
                color: Color::from_str(color_str).unwrap(),
                min_distance,
                max_distance,
            })
        },
        GridPickStrategy::EliminateBgColor => {
            GridPickCmd::EliminateBgColor(EliminateBgColorParam{
                color: "#ffffffff".into(),
                min_remaining_ratio: req.grid_pick_options.remaining_ratio.unwrap_or(0.1),
            })
        }
    };


    info!("pick_strategy: {:?}", pick_strategy);

    let grids = generate_canvas_grids_by_image_path(image_info.path.as_str(), fill_options, pick_strategy)
        .map_err(|e| ApiError::BizError("IMAGE_NOT_FOUND".to_string(), e.to_string()))?;


    let mut mosaic_grids = Vec::with_capacity(grids.len());
    for grid in &grids{
        // debug!("grid seq: {:?}, ext: {:?}", grid.seq, grid.ext);

        let mut selected = false;
        if let Some(v) = grid.ext.selected {
            selected = v;
        }

        let avg_color = grid.ext.avg_color;

        let mosaic_grid = MosaicGrid{
            seq: grid.seq.clone(),
            points: grid.points.clone(),
            shape: grid.shape.into(),
            selected,
            color: req.grid_selected_color.clone(),
            avg_color: avg_color.map(|c| c.to_rgba_string()),
            color_distance: grid.ext.color_distance,
            remaining_area_ratio: grid.ext.remaining_area_ratio,
        };
        mosaic_grids.push(mosaic_grid);
    } 

    let reply = MosaicGridsConvertReply{
        canvas_width: image_info.width,
        canvas_height: image_info.height,
        grids: mosaic_grids,
    };
    Ok(ApiResponse::ok(reply))
}

use std::sync::Arc;
use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use tracing::{debug, info};
use crate::{generate_canvas_grids_by_image_path, ApiError, ApiResponse, AppState, AvgColorCompareParam, EliminateBgColorParam, GridFillOptions, GridPickStrategy, ImageInfo, Point, Value, GRID_EXT_SELECTED};


/// 将logo图片转换为canvas上马赛克的形状
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogoImageConvertReq{
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub fill_shape_type: String,
    pub fill_shape_size: Vec<u32>,
    pub logo_image_id: String
}


/// logo图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogoImageInfo{
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub url: String,
    pub bg_color:(u8, u8, u8)
}

// logo图片列表查询
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogoImageListReply{
    pub logo_images: Vec<ImageInfo>
}




/// 查询所有图片信息
async fn image_list_handler(State(app_state): State<Arc<AppState>>) -> Result<ApiResponse<LogoImageListReply>, ApiError> {
    let mut images = Vec::new();
    app_state.image_map.iter().for_each(|item| {
        images.push(item.value().clone());
    });
    
    Ok(ApiResponse::ok(LogoImageListReply{
        logo_images: images
    }))
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MosaicGridsConvertReq{
    pub image_id: String,
    pub grid_shape: String,
    pub grid_size: Vec<u32>,
    pub grid_pick_strategy: String,
    pub grid_pick_options: Vec<String>,



}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MosaicGridsConvertStrategy{

    pub name: String,
    pub options: Vec<f32>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MosaicGridsConvertReply {
    // 画布宽度
    pub canvas_width: u32,
    // 画布高度
    pub canvas_height: u32,
    // 网格信息
    pub grids: Vec<MosaicGrid>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MosaicGrid {
    // 序号
    pub seq: String,
    // 点坐标
    pub points: Vec<Point>,
    // 形状
    pub shape: String,
    // 是否选中
    pub selected: bool,

}

// async fn image_list_handler(State(app_state): State<Arc<AppState>>) -> Result<ApiResponse<LogoImageListReply>, ApiError> 

/// 给定图片和参数，给出多边形马赛克填充的canvas数据
async fn convert_to_mosaic_grids(
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<MosaicGridsConvertReq>,
) -> Result<ApiResponse<MosaicGridsConvertReply>, ApiError> {

    info!("convert image into mosaic grids, req: {:?}", req);
    let image_id = req.image_id;
    if !app_state.image_map.contains_key(&image_id) {
        return Err(ApiError::BizError("IMAGE_NOT_FOUND".to_string(), "image not found".to_string()));
    }

    let image_info = get_image_by_id(image_id, app_state)?;


    let fill_options = match req.grid_shape.as_str() {
        "triangle" => {
            let grid_size =  req.grid_size;
            GridFillOptions::Triangle(grid_size[0], grid_size[1])

        },
        _ => { 
            // return Err(ApiError::BizError("UNSUPPORTED_GRID_SHAPE".to_string(), "unsupported grid shape".to_string());
            return Err(ApiError::InvalidParameter("grid_shape".to_string(), "unsupported grid shape".to_string()));
        }
    };
    
    info!("fill_options: {:?}", fill_options);


    // let pick_strategy = GridPickStrategy::EliminateBgColor(EliminateBgColorParam{
    //     color: image_info.bg_color.clone(),
    //     min_ratio: 0.3,
    // });
    let pick_strategy = GridPickStrategy::AvgColorCompare(AvgColorCompareParam{
        color: (255, 255, 255),
        min_distance: 0.3,
        max_distance: 1.0,
    });

    info!("pick_strategy: {:?}", pick_strategy);

    let grids = generate_canvas_grids_by_image_path(image_info.path.as_str(), fill_options, pick_strategy)
        .map_err(|e| ApiError::BizError("IMAGE_NOT_FOUND".to_string(), e.to_string()))?;

    let mut mosaic_grids = Vec::with_capacity(grids.len());
    for grid in &grids{
        debug!("grid seq: {:?}, ext: {:?}", grid.seq, grid.ext);

        let selected: bool = if let Some(Value::Bool(value)) = grid.ext.get(GRID_EXT_SELECTED) {
            *value
        } else {
            false  
        };
        let mosaic_grid = MosaicGrid{
            seq: grid.seq.clone(),
            points: grid.points.clone(),
            shape: grid.shape.into(),
            selected,
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



pub fn image_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/list", get(image_list_handler))
        .route("/convert_to_mosaic_grids", post(convert_to_mosaic_grids))
}


fn get_image_by_id(image_id: String, app_state: Arc<AppState>) -> Result<ImageInfo, ApiError> {
    if let Some(image_info) = app_state.image_map.get(&image_id) {
        Ok(image_info.clone())
    } else {
        Err(ApiError::BizError(
            "IMAGE_NOT_FOUND".to_string(), 
            format!("image not found with id: {}", image_id).to_string()
        ))
    }

}
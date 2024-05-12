use std::sync::Arc;
use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Map;

use crate::{AppState, ImageInfo, Point};



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
    pub url: String
}

// logo图片列表查询
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LogoImageListReply{
    pub logo_images: Vec<ImageInfo>
}




/// 查询所有图片信息
async fn image_list_handler(State(app_state): State<Arc<AppState>>) -> Json<LogoImageListReply> {
    let mut images = Vec::new();
    app_state.image_map.iter().for_each(|item| {
        images.push(item.value().clone());
    });
    
    Json(LogoImageListReply{
        logo_images: images
    })
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MosaicGridsConvertReq{
    pub image_id: String,
    pub grid_shape: String,
    pub grid_size: Vec<u32>,
    pub convert_strategy: MosaicGridsConvertStrategy,


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
    // 颜色
    pub color: (u8, u8, u8),
    // 是否选中
    pub selected: bool,

}





/// 给定图片和参数，给出多边形马赛克填充的canvas数据
async fn convert_to_mosaic_grids(Json(req): Json<MosaicGridsConvertReq>) -> Json<MosaicGridsConvertReply> {


    Json(MosaicGridsConvertReply{
        canvas_width: 968,
        canvas_height: 698,
        grids: vec![
            MosaicGrid{
                seq: "1".into(),
                points: vec![
                    Point{x: 0, y: 0},
                    Point{x: 10, y: 0},
                    Point{x: 10, y: 10},
                ],
                color: (255, 255, 255),
                selected: true,
            }
        ]
    })
}



pub fn image_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/list", get(image_list_handler))
        .route("/convert_to_mosaic_grids", post(convert_to_mosaic_grids))
}
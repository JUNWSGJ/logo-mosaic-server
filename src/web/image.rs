use std::sync::Arc;
use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};

use crate::AppState;



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
    pub logo_images: Vec<LogoImageInfo>
}




/// 查询所有图片信息
async fn image_list_handler() -> Json<LogoImageListReply> {
    Json(LogoImageListReply{
        logo_images: vec![
            LogoImageInfo{
                id: "1".into(),
                name: "logo1".into(),
                width: 698,
                height: 968,
                url: "images/logo1.png".into()
            },
        ]
    })
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MosaicGridsConvertReq{
    pub image_id: String,
    pub grid_shape: String,
    pub grid_size: Vec<u32>,
    pub convert_strategy: String,
    pub strategy_params: Vec<u32>,

}

struct MosaicGridsConvertStrategy{
    pub name: String,

}




/// 给定图片和参数，给出多边形马赛克填充的canvas数据
async fn convert_Into_mosaic_grids() -> Json<LogoImageListReply> {
    Json(LogoImageListReply{
        logo_images: vec![
            LogoImageInfo{
                id: "1".into(),
                name: "logo1".into(),
                width: 698,
                height: 968,
                url: "images/logo1.png".into()
            },
        ]
    })
}



pub fn image_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/list", get(image_list_handler))
}
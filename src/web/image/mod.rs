mod list;
mod convert_mosaic;

use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use self::{list::image_list_handler, convert_mosaic::convert_to_mosaic_grids};
use crate::AppState;


/// 将logo图片转换为canvas上马赛克的形状
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LogoImageConvertReq{
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub fill_shape_type: String,
    pub fill_shape_size: Vec<u32>,
    pub logo_image_id: String
}







#[derive(Debug, Clone, Serialize, Deserialize)]
struct MosaicGridsConvertStrategy{

    pub name: String,
    pub options: Vec<f32>,

}



pub fn image_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/list", get(image_list_handler))
        .route("/convert_to_mosaic_grids", post(convert_to_mosaic_grids))
}


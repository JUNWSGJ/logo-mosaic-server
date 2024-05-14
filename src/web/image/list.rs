use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{ApiError, ApiResponse, AppState, Color, ImageRepo};




/// logo图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoImageInfo{
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub path: String,
    pub bg_color:String
}

// logo图片列表查询
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoImageListReply{
    pub logo_images: Vec<LogoImageInfo>
}


/// 查询所有图片信息
pub async fn image_list_handler(State(app_state): State<Arc<AppState>>) -> Result<ApiResponse<LogoImageListReply>, ApiError> {
    let mut images = Vec::new();
    let image_data_list = app_state.image_repo.list_images();
    for image_data in image_data_list{
        let image = LogoImageInfo{
            id: image_data.id.clone(),
            width: image_data.width,
            height: image_data.height,
            name: image_data.name.clone(),
            path: image_data.path.clone(),
            bg_color: Color::from_rgb(image_data.bg_color).to_string(),
        };
        images.push(image);
    }
    
    Ok(ApiResponse::ok(LogoImageListReply{
        logo_images: images
    }))
}

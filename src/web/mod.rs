mod canvas;
mod image;

use std::sync::Arc;
use axum::Router;
use canvas::canvas_routes;
use dashmap::DashMap;
use image::image_routes;

use crate::ImageInfo;


#[derive(Debug)]
pub struct AppState {
    /// logo图片存储路径
    pub logo_image_dir_path: &'static str,
    /// 前端静态资源路径
    pub static_path: &'static str,
    /// logo图片集
    pub image_map: DashMap<String, ImageInfo>,
    
}


pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/canvas", canvas_routes())
        .nest("/image", image_routes())
}
mod canvas;
mod image;
mod activity;

use std::sync::Arc;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Router};
use canvas::canvas_routes;
use dashmap::DashMap;
use image::image_routes;
use serde::Serialize;
use thiserror::Error;
use crate::ImageInfo;

#[derive(Error, Debug)]
pub enum ApiError{
    #[error("Internal server error")]
    InternalServerError,
    #[error("BizError, code:{0}, message:{1}")]
    BizError(String, String),
    #[error("InvalidParameter, message:{1}")]
    InvalidParameter(String, String),
}


#[derive(Debug, Clone,Serialize)]
pub struct ApiResponse<T> {
    pub code: String,
    pub message: String,
    pub data: T,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERRROR".to_owned(), "Internal Server Error".to_owned()),
            // 为其他错误类型匹配相应的HTTP状态码和消息
            ApiError::BizError(code, message) => (StatusCode::OK, code, message),
            ApiError::InvalidParameter(field, message) => (
                StatusCode::BAD_REQUEST, "INVALID_PARAMETER".to_string(), format!("参数[{}]无效：{}", field, message)
            ),
        };

        let api_response = ApiResponse {
            code,
            message,
            data: (),
        };
        (status, serde_json::to_string(&api_response).unwrap_or_default()).into_response()
    }
}

impl <T> ApiResponse<T> 
where T: Serialize{
    pub fn ok(data: T) -> Self {
        Self {
            code: "SUCCESS".to_string(),
            message: "success".to_string(),
            data,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize, // 假设数据部分需要序列化，这里使用serde的Serialize trait
{
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).expect("ApiResponse serialization failed");
        (StatusCode::OK, body).into_response()
    }
}






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
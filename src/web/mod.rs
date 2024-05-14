mod image;
mod activity;
mod canvas;


use std::sync::Arc;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json, Router};
use serde::Serialize;
use thiserror::Error;

use image::image_routes;
use activity::activity_routes;
use canvas::canvas_routes;
use crate::{repo::ActivityMemoryRepo, ImageMemoryRepo};

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
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub success: bool,
    pub err_code: Option<String>,
    pub err_message: Option<String>,
    pub data: T,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERRROR".into(), "Internal Server Error".into()),
            // 为其他错误类型匹配相应的HTTP状态码和消息
            ApiError::BizError(code, message) => (StatusCode::OK, code, message),
            ApiError::InvalidParameter(field, message) => (
                StatusCode::BAD_REQUEST, "INVALID_PARAMETER".into(), format!("参数[{}]无效：{}", field, message)
            ),
        };

        let api_response = ApiResponse {
            success: false,
            err_code: Some(code),
            err_message: Some(message),
            data: (),
        };
        let json_response = Json(api_response);
        (status, json_response.into_response()).into_response()
    }
}

impl <T> ApiResponse<T> 
where T: Serialize{
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            err_code: None,
            err_message: None,
            data,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize, // 假设数据部分需要序列化，这里使用serde的Serialize trait
{
    fn into_response(self) -> Response {
        let json_response = Json(self);
        (StatusCode::OK, json_response.into_response()).into_response()
    }
}




pub struct AppState {
    /// logo图片存储路径
    pub logo_image_dir_path: &'static str,
    /// 前端静态资源路径
    pub static_path: &'static str,
    /// logo图片集
    pub image_repo: ImageMemoryRepo,
    /// 活动repo
    pub activity_repo: ActivityMemoryRepo,
    
}


pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/canvas", canvas_routes())
        .nest("/image", image_routes())
        .nest("/activity", activity_routes())
}
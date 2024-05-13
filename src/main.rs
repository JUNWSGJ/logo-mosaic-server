use anyhow::Result;
use axum::{ http::StatusCode, routing::get_service};
use dashmap::DashMap;
use logo_process::{api_routes, AppState, ImageInfo};
use tower_http::services::ServeDir;
use std::{net::SocketAddr, sync::Arc};
use tracing::{debug, info};



#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8002));
    debug!("debug log is enabled");
    info!("Serving on {}", addr);

    let logo_image_dir_path= "images";
    // 加载logo图片
    let image_map =  mock_all_images();

    let app_state = Arc::new(AppState { 
        logo_image_dir_path,
        static_path: "./logo-mosaic-web/dist",
        image_map,
     });

    let static_service = get_service(
        ServeDir::new(app_state.static_path),)
            .handle_error(|_| async move { (StatusCode::INTERNAL_SERVER_ERROR, "internal server error") }
    );
    let image_service = get_service(
        ServeDir::new(app_state.logo_image_dir_path),)
            .handle_error(|_| async move { (StatusCode::INTERNAL_SERVER_ERROR, "internal server error") }
    );

    let router = axum::Router::new()
        .nest("/api", api_routes())
        .nest_service("/images", image_service)
        .fallback(static_service)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}




fn mock_all_images() -> DashMap<String, ImageInfo>{
    let image_map = dashmap::DashMap::new();
    image_map.insert("1".to_string(), ImageInfo{
        id: "1".to_string(),
        width: 968,
        height: 698,
        name: "logo1.png".to_string(),
        path: "images/logo1.png".to_string(),
        bg_color: (255, 255, 255)
    },);
    image_map
}
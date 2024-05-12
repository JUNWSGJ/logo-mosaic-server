use anyhow::Result;
use axum::{ http::StatusCode, routing::get_service};
use logo_process::{api_routes, AppState, load_all_image_info};
use tower_http::services::ServeDir;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;



#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8002));
    info!("Serving on {}", addr);

    let logo_image_dir_path= "images";
    // 加载logo图片
    let image_map =  load_all_image_info(logo_image_dir_path)?;
 

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





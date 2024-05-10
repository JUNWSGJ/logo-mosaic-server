use anyhow::Result;
use axum::{extract::State, http::StatusCode, response::Redirect, routing::{get, get_service}, Router};
// use axum_extra::
use tower_http::services::ServeDir;

use std::{net::SocketAddr,sync::Arc};
use tracing::info;

#[derive(Debug)]
struct AppState {
    
}

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    info!("Serving on {}", addr);
    let app_state = Arc::new(AppState {  });

    let static_service = get_service(
        ServeDir::new("./logo-mosaic-web/dist"))
            .handle_error(|_| async move { (StatusCode::INTERNAL_SERVER_ERROR, "internal server error") }
    );

    let router = axum::Router::new()
        .route("/api/auth", get(|| async { "API: /auth" }))
        .fallback( static_service)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}




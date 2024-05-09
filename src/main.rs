use anyhow::Result;

use std::{net::SocketAddr,sync::Arc};
use tracing::info;
use tower_http::services::ServeDir;

#[derive(Debug)]
struct HttpServeState {
    
}

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    info!("Serving on {}", addr);
    let state = HttpServeState {  };

    let router = axum::Router::new()
        .nest_service("/ui", ServeDir::new("./static"))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

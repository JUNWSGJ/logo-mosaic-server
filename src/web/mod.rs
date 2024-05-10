mod canvas;

use std::sync::Arc;
use axum::Router;
use crate::AppState;
use canvas::canvas_routes;



pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/canvas", canvas_routes())
}
mod list;
mod detail;
mod sign_in;
mod reset;
mod create;

use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use crate::AppState;



use self::{
    detail::activity_detail_handler,
    list::activity_list_handler,
    reset::activity_reset_in_handler, 
    sign_in::activity_sign_in_handler, 
    create::activity_create_handler
};


// async fn activity_create_handler() -> anyhow::Result<String> {
//     Ok("I am alive".to_string())
// }


pub fn activity_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/list", get(activity_list_handler))
        .route("/detail", get(activity_detail_handler))
        .route("/signIn", post(activity_sign_in_handler))
        .route("/reset", get(activity_reset_in_handler))
        .route("/create", post(activity_create_handler))
}

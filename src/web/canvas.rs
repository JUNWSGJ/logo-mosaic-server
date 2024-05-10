use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{AppState, FillShapeOptions, Point, process::generate_canvas_shapes};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetEmptyCanvasDataReq{
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub fill_shape_type: String,
    pub fill_shape_size: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetEmptyCanvasDataReply{
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub fill_shapes: Vec<Vec<Point>>,
}



async fn caculate_empty_canvas_data(Json(req): Json<GetEmptyCanvasDataReq>) -> Json<GetEmptyCanvasDataReply> {
    match req.fill_shape_type.as_str() {
        "triangle" => {
            let fill_shape_size = req.fill_shape_size;
            let fill_shape_options = FillShapeOptions::Triangle(fill_shape_size[0], fill_shape_size[1]);
            let fill_shapes = generate_canvas_shapes(req.canvas_width, req.canvas_height, fill_shape_options).unwrap();
            GetEmptyCanvasDataReply{
                canvas_width: req.canvas_width,
                canvas_height: req.canvas_height,
                fill_shapes,
            }.into()
        }
        _ => {
            Json(GetEmptyCanvasDataReply{
                canvas_width: req.canvas_width,
                canvas_height: req.canvas_height,
                fill_shapes: vec![],
            })
        }
    }

    
}



pub fn canvas_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/calculate_empty_canvas_data", post(caculate_empty_canvas_data))
}
mod canvas_generate;
mod canvas_shape_pick;
mod image_process;

pub use image_process::{draw_empty_canvas, load_all_image_info};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::{FillShapeOptions, Point, ShapePickStrategy};
use canvas_generate::get_all_triangles_of_canvas;
use canvas_shape_pick::pick_triangles_from_canvas;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub path: String,
}



/// 指定画布的宽高，以及填充画布的图形信息（形状及尺寸）生成画布上的所有图形的坐标
pub fn generate_canvas_shapes(canvas_width: u32, canvas_height: u32, options: FillShapeOptions) -> Result<Vec<Vec<Point>>> {
    match options {
        FillShapeOptions::Triangle(w, h) => {
            get_all_triangles_of_canvas(canvas_width, canvas_height, w, h)
        }
    }
}

/// 依据指定logo图，从画布上挑选出拼接成该图中logo形状的所有图形，logo图的宽高和画布宽高一致
pub fn pick_shapes_from_canvas(shapes: Vec<Vec<Point>>, image_path: &str, strategy: ShapePickStrategy) -> Result<Vec<Vec<Point>>> {
    // TODO: 暂时只有三角形
    pick_triangles_from_canvas(shapes, image_path, strategy)
    
}

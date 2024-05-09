mod process;

use anyhow::Result;
pub use process::{generate_canvas_shapes};
// pub use image_process::filter_triangles_with_image;


// 画布生成
pub trait CanvasGenerator{
    fn generate(options: FillShapeOptions, width: u32, height: u32) -> Result<Vec<Vec<Point>>>;
}



/// 画布填充选项
pub enum FillShapeOptions {
    // 三角形（宽，高）
    Triangle(u32, u32),
    // // 矩形（宽，高）
    // Rectangle(u32, u32),
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct TrianglePoints([Point; 3]);

#[derive(Debug, Clone, Copy)]
pub struct RectanglePoints([Point; 4]);


#[derive(Debug, Clone, Copy)]
pub enum ShapePoints{
    Triangle(TrianglePoints),
    Rectangle(RectanglePoints),
}


#[derive(Debug, Clone)]
pub struct ShapeData {
    pub seq: String,
    pub points: ShapePoints,
}

impl TrianglePoints {
    // 添加一个公共方法来遍历点
    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.0.iter()
    }
}

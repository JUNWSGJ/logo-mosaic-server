mod process;

use enum_dispatch::enum_dispatch;
use anyhow::Result;
use image::Rgba;
use std::str::FromStr;
pub use process::*;


#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}




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


pub struct Color(pub Rgba<u8>);

impl FromStr for Color{
    type Err = anyhow::Error;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        assert!(hex.starts_with('#'), "Hex color must start with '#'");
        let (r, g, b) = if hex.len() == 7 {
            (
                u8::from_str_radix(&hex[1..3], 16).unwrap(),
                u8::from_str_radix(&hex[3..5], 16).unwrap(),
                u8::from_str_radix(&hex[5..7], 16).unwrap(),
            )
        } else {
            panic!("Invalid hex color length")
        };
        // 假设透明度为255（完全不透明），可以根据需要调整
        Ok(Color(image::Rgba([r, g, b, 255])))
    }
}
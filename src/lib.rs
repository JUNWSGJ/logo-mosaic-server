mod process;
mod web;

use anyhow::Result;
use image::Rgba;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
pub use process::*;
pub use web::*;




#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}


/// 画布填充选项
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FillShapeOptions {
    // 三角形（宽，高）
    Triangle(u32, u32),
    // // 矩形（宽，高）
    // Rectangle(u32, u32),
}

pub enum ShapePickStrategy{

    AvgColorCompare(AvgColorCompareParam)

}

pub struct AvgColorCompareParam{
    
    // 目标颜色
    pub target: (u32, u32, u32),

    // 差值范围 [0, 1]
    pub distance_range: (f32, f32),
   
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
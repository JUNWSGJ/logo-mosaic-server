mod process;
mod web;
mod repo;
mod utils;

use anyhow::Result;
use image::Rgba;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
pub use process::*;
pub use web::*;
pub use repo::*;



#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GridShape {
    #[serde(rename = "triangle")]
    Triangle,
    // Rectangle(u32, u32),
}

impl From<GridShape> for String {
    fn from(shape: GridShape) -> Self {
        match shape {
            GridShape::Triangle => "triangle".to_string(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
    pub seq: String,
    pub shape: GridShape,
    pub points: Vec<Point>,
    pub ext: GridExt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridExt {
    pub avg_color: Option<(u8, u8, u8)>,
    pub color_distance: Option<f32>,
    pub remaining_area_ratio: Option<f32>,
    pub selected: Option<bool>,
}

impl Default for GridExt {
    fn default() -> Self {
        Self {
            avg_color: None,
            color_distance: None,
            remaining_area_ratio: None,
            selected: None,
        }
    }
}



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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasGridGenerateParams {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub grid_fill_options: GridFillOptions,
    pub grid_pick_strategy: GridPickCmd,
}



/// 画布填充选项
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GridFillOptions {
    // 三角形（宽，高）
    Triangle(u32, u32),
    // // 矩形（宽，高）
    // Rectangle(u32, u32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GridPickCmd{
    // 区域平均色值比较
    AvgColorCompare(AvgColorCompareParam),
    // 剔除背景色后，
    EliminateBgColor(EliminateBgColorParam),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GridPickStrategy{
    // 计算格子内像素点的平均色值，与目标色值比较差值
    AvgColorCompare,
    // 剔除背景色，根据剩余像素点的占比来选择格子
    EliminateBgColor,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AvgColorCompareParam{
    // 要比较的颜色
    pub color: (u8, u8, u8),
    pub min_distance: f32,
    pub max_distance: f32,
}


/// 剔除背景色
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EliminateBgColorParam{
    // 要剔除的背景色
    pub color: (u8, u8, u8),
    // 剩余区域的最小占比
    pub min_remaining_ratio: f32,
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub path: String,
    pub bg_color: (u8, u8, u8),
}


pub struct Color(pub Rgba<u8>);

impl Color {
    pub fn from_rgb((r,g, b): (u8,u8,u8)) -> Self {
        Color(image::Rgba([r, g, b, 255]))
    }

    fn to_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0[0], self.0[1], self.0[2])
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        (self.0[0], self.0[1], self.0[2])
    }

}

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
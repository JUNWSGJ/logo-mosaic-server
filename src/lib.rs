mod process;
mod web;

use anyhow::Result;
use image::Rgba;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
pub use process::*;
pub use web::*;

pub const GRID_EXT_AVG_COLOR: &'static str = "avg_color";
pub const GRID_EXT_COLOR_DISTANCE: &'static str = "color_distance";
pub const GRID_EXT_REMAINING_AREA_RATIO: &'static str = "remaining_area_ratio";
pub const GRID_EXT_REMOVED_BG_COLOR: &'static str = "eliminated_bg_color";
pub const GRID_EXT_SELECTED: &'static str = "selected";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    Int(i32),
    String(String),
    Rgb((u8, u8, u8)),
    U32(u32),
    U32Array(Vec<u32>),
    F32(f32),
    F32Array(Vec<f32>),
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GridShape {
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
pub struct Grid {
    seq: String,
    shape: GridShape,
    points: Vec<Point>,
    ext: HashMap<String, Value>,
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
    pub grid_pick_strategy: GridPickStrategy,
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
pub enum GridPickStrategy{

    // 区域平均色值比较
    AvgColorCompare(AvgColorCompareParam),
    // 剔除背景色后，
    EliminateBgColor(EliminateBgColorParam),
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
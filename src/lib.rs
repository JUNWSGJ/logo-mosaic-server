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
pub use utils::calc_color_distance;



#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GridShape {
    #[serde(rename = "triangle")]
    Triangle,
    #[serde(rename = "rectangle")]
    Rectangle,
}

impl From<GridShape> for String {
    fn from(shape: GridShape) -> Self {
        match shape {
            GridShape::Triangle => "triangle".to_string(),
            GridShape::Rectangle => "rectangle".to_string(),
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
    pub avg_color: Option<Color>,
    pub color_distance: Option<f32>,
    pub remaining_area_ratio: Option<f32>,
    pub selected: Option<bool>,
    pub fill_color: Option<Color>,
    pub border_color: Option<Color>,
}

impl Default for GridExt {
    fn default() -> Self {
        Self {
            avg_color: None,
            color_distance: None,
            remaining_area_ratio: None,
            selected: None,
            fill_color: None,
            border_color: None,
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
    Rectangle(u32, u32),
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
    pub color: Color,
    pub min_distance: f32,
    pub max_distance: f32,
}


/// 剔除背景色
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EliminateBgColorParam{
    // 要剔除的背景色
    pub color: Color,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color((u8, u8, u8, u8));

impl Color {
    pub fn from_rgb((r,g, b): (u8,u8,u8)) -> Self {
        Color((r, g, b, 255))
    }

    pub fn from_rgba((r,g, b, a): (u8,u8,u8,u8)) -> Self {
        Color((r, g, b, a))
    }

    pub fn to_string(&self) -> String {
        self.to_rgba_string()
    }
    pub fn to_rgb_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0.0, self.0.1, self.0.2)
    }
    pub fn to_rgba_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.0.0, self.0.1, self.0.2, self.0.3)
    }
    pub fn to_rgb(&self) -> (u8,u8,u8) {
        (self.0.0, self.0.1, self.0.2)
    }
    pub fn to_rgba(&self) -> (u8,u8,u8,u8) {
        (self.0.0, self.0.1, self.0.2, self.0.3)
    }

}

impl FromStr for Color{
    type Err = anyhow::Error;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        assert!(hex.starts_with('#'), "Hex color must start with '#'");
        let (r, g, b, a) = if hex.len() == 7 {
            (
                u8::from_str_radix(&hex[1..3], 16).unwrap(),
                u8::from_str_radix(&hex[3..5], 16).unwrap(),
                u8::from_str_radix(&hex[5..7], 16).unwrap(),
                255,
            )
        } else if hex.len() == 9 {
            (
                u8::from_str_radix(&hex[1..3], 16).unwrap(),
                u8::from_str_radix(&hex[3..5], 16).unwrap(),
                u8::from_str_radix(&hex[5..7], 16).unwrap(),
                u8::from_str_radix(&hex[7..9], 16).unwrap(),
            )
        } else {
            panic!("Invalid hex color length")
        };
        
        Ok(Color((r, g, b, a)))
    }
    
}

impl ToString for Color {
    fn to_string(&self) -> String {
        self.to_rgba_string()
    }
    
}

impl Into<Rgba<u8>> for Color {
    fn into(self) -> Rgba<u8> {
        Rgba([self.0.0, self.0.1, self.0.2, self.0.3])
    }
}

impl Into<Color> for &str {
    fn into(self) -> Color {
        Color::from_str(self).unwrap()
    }
}
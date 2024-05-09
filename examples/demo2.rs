use std::str::FromStr;

use anyhow::Result;
use image::Rgba;
use logo_process::{generate_canvas_shapes,FillShapeOptions, draw_empty_canvas};

struct Color(Rgba<u8>);

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


fn main() -> Result<()> {
    // 设置矩形画布的尺寸,背景色
    // "#373737"
    // "#9099A2"
    // "#984B43"
    // "#EAC67A"

    let canvas_width = 1000;
    let canvas_height = 800;
    // let bg_color = Rgba([37u8, 37u8, 37u8, 255u8]);
    let bg_color = Color::from_str("#373737")?.0;
    let polygon_color = Color::from_str("#9099A2")?.0;
    let polygon_board_color = Color::from_str("#ffffff")?.0;
    // 设置三角形的水平边长和高度
    let triangle_width = 50;
    let triangle_height = 40;

    let fill_options = FillShapeOptions::Triangle(triangle_width, triangle_height);
    let polygons = generate_canvas_shapes(fill_options, canvas_width, canvas_height)?;

    draw_empty_canvas(canvas_width, canvas_height, bg_color, polygon_color, polygon_board_color, polygons, "canvas.png".into())?;
    Ok(())
}

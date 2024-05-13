

use std::str::FromStr;
use anyhow::Result;
use logo_process::{generate_canvas_grids,GridFillOptions, draw_empty_canvas, Color};




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

    let fill_options = GridFillOptions::Triangle(triangle_width, triangle_height);
    let polygons = generate_canvas_grids( canvas_width, canvas_height, fill_options)?;

    draw_empty_canvas(canvas_width, canvas_height, bg_color, polygon_color, polygon_board_color, polygons, "canvas.png".into())?;
    Ok(())
}

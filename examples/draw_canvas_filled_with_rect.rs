

use std::str::FromStr;
use anyhow::Result;
use logo_process::{Color, get_canvas_grids_filled_with_trianles, draw_canvas_with_grids};


fn main() -> Result<()> {
    // 设置矩形画布的尺寸,背景色
    // "#373737"
    // "#9099A2"
    // "#984B43"
    // "#EAC67A"

    let canvas_width = 1000;
    let canvas_height = 800;
    // let bg_color = Rgba([37u8, 37u8, 37u8, 255u8]);
    let canvas_color = Color::from_str("#373737")?.0;
    let grid_fill_color = Color::from_str("#9099A2")?.0;
    let grid_border_color = Color::from_str("#ffffff")?.0;
    // 设置三角形的水平边长和高度
    let triangle_width = 50;
    let triangle_height = 40;

    let grids = get_canvas_grids_filled_with_trianles( canvas_width, canvas_height, triangle_width, triangle_height)?;

    draw_canvas_with_grids(
        canvas_width, 
        canvas_height, 
        canvas_color, 
        grids, 
        grid_border_color, 
        grid_fill_color, 
        "output/canvas_filled_with_triangles.png".into())?;
    Ok(())
}

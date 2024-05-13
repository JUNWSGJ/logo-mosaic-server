use std::str::FromStr;

use image::{ImageBuffer, Rgba};
use logo_process::{draw_canvas_with_grids, generate_canvas_grids_from_logo_image, AvgColorCompareParam, Color, GridFillOptions, GridPickStrategy};
use anyhow::Result;

fn main() -> Result<()>{
    

    let image_path = "images/logo1.png";
    let fill_options = GridFillOptions::Triangle(20, 16);
    let pick_strategy = GridPickStrategy::AvgColorCompare(AvgColorCompareParam {
        color: (255, 255, 255),
        min_distance: 0.5,
        max_distance: 1.0
    });
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(image_path).unwrap().to_rgba8();
    let canvas_width = img.width();
    let canvas_height = img.height();
    let canvas_color = Color::from_str("#373737")?.0;
    let grid_fill_color = Color::from_str("#9099A2")?.0;
    let grid_border_color = Color::from_str("#ffffff")?.0;

    let grids = generate_canvas_grids_from_logo_image(
        &img, fill_options, pick_strategy)?;

    // 填充格子
    draw_canvas_with_grids(
        canvas_width, canvas_height, canvas_color,
        grids, grid_border_color, grid_fill_color,
        "output/logo_mosaic_grids.png".into())?;

    Ok(())
}
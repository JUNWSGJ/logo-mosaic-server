use std::path::PathBuf;
use image::{ImageBuffer, Rgba};
use anyhow::Result;
use imageproc::drawing::{draw_hollow_polygon_mut, draw_polygon_mut};

use crate::{Grid, Point};


/// 画带有格子的画布
pub fn draw_canvas_with_grids(
    canvas_width: u32, 
    canvas_height: u32,
    canvas_color: Rgba<u8>,
    grids: Vec<Grid>,
    path: PathBuf,
) -> Result<()> {
    // 创建一个新的空白画布
    let mut img = ImageBuffer::from_pixel(canvas_width,  canvas_height, canvas_color);
    
    // 填充格子
    for grid in &grids {
        let mut points: Vec<imageproc::point::Point<i32>> = Vec::with_capacity(grid.points.len());
        for point in &grid.points {
            points.push(imageproc::point::Point {
                x: point.x as i32,
                y: point.y as i32,
            });
        }
        // println!("正在绘制格子：{:?}", grid);
        let fill_color = grid.ext.fill_color.expect("Each grid should have an fill_color when drawing.");
        draw_polygon_mut(&mut img, &points, fill_color.into());

    }

    // 画格子的边框
    for grid in &grids  {
        let mut points: Vec<imageproc::point::Point<f32>> = Vec::with_capacity(grid.points.len());
        for point in &grid.points {
            points.push(imageproc::point::Point {
                x: point.x as f32,
                y: point.y as f32,
            });
        }
        let grid_border_color = grid.ext.border_color.expect("Each grid should have an border_color when drawing.");
        draw_hollow_polygon_mut(&mut img, &points, grid_border_color.into());
    }

    // 保存图像
    img.save(path)?;
    
    Ok(())

}


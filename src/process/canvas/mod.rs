mod triangle;
mod reactangle;

use image::RgbaImage;
use anyhow::Result;
use self::triangle::{
    calc_average_color_in_triangle, 
    calc_remaining_area_ratio_in_triangle, 
    genarate_canvas_grids_filled_with_trianles
};
use self::reactangle::{calc_average_color_in_rectangle, calc_remaining_area_ratio_in_rectangle, genarate_canvas_grids_filled_with_rectanles};
use crate::{Color, Grid, GridFillOptions, GridShape};

/// 生成空画布格子信息(格子形状支持：三角形，矩形)
pub fn generate_enmty_canvas_grids(
    canvas_width: u32, canvas_height: u32, options: GridFillOptions) -> Result<Vec<Grid>> {
    match options {
        GridFillOptions::Triangle(w, h) => {
            genarate_canvas_grids_filled_with_trianles(canvas_width, canvas_height, w, h)
        }
        GridFillOptions::Rectangle(w, h) => {
            genarate_canvas_grids_filled_with_rectanles(canvas_width, canvas_height, w, h)
        },
    }
}

/// 计算格子的平均色值
pub fn calc_avg_color_of_grid(img: &RgbaImage, grid: &Grid) -> Result<Color>{
    match grid.shape {
        GridShape::Triangle => {
            let points = &grid.points;
            calc_average_color_in_triangle(img, points)
        },
        GridShape::Rectangle => {
            let points = &grid.points;
            calc_average_color_in_rectangle(img, points)
        }
    }
}

/// 计算格子内像素剔除指定颜色后的像素占比
pub fn calc_remaining_area_ratio_in_grid(
    img: &RgbaImage, 
    grid: &Grid,
    bg_color: Color) -> Result<f32>{
    match grid.shape {
            GridShape::Triangle => {
                let points = &grid.points;
                assert_eq!(points.len(), 3, "Each triangle should have 3 points.");
                let triangle:[(u32, u32); 3] = [
                    (points[0].x, points[0].y),
                    (points[1].x, points[1].y),
                    (points[2].x, points[2].y),
                ];
                // 计算剔除背景色后的剩余区域占比
                calc_remaining_area_ratio_in_triangle(img, bg_color, triangle)
            },
            GridShape::Rectangle => {
                let points = &grid.points;
                calc_remaining_area_ratio_in_rectangle(img, bg_color, points)
            }

    }
    
}

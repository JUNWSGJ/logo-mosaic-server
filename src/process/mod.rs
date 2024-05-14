mod canvas;
mod canvas_grid_pick;
mod image_draw;

pub use image_draw::draw_canvas_with_grids;
pub use canvas::generate_enmty_canvas_grids;

use image::{ImageBuffer, Rgba, RgbaImage};
use anyhow::Result;
use tracing::debug;
use crate::{ calc_color_distance, Grid, GridFillOptions, GridPickCmd};
use canvas::{calc_avg_color_of_grid, calc_remaining_area_ratio_in_grid};






pub fn generate_canvas_grids_by_image_path(
    image_path: &str, 
    fill_options: GridFillOptions, 
    pick_strategy: GridPickCmd) -> Result<Vec<Grid>>  {
    // 加载图片，确定画布的宽高
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(image_path).unwrap().to_rgba8();
    generate_canvas_grids_from_logo_image(&img, fill_options, pick_strategy)
}


/// 根据指定logo图片，生成对应的马赛克格子信息
/// image_path: logo图片的路径
/// fill_options: 画布填充的图形形状及尺寸
/// pick_strategy: 选择格子的策略
pub fn generate_canvas_grids_from_logo_image(
    img: &RgbaImage,
    fill_options: GridFillOptions, 
    pick_strategy: GridPickCmd) -> Result<Vec<Grid>> {

    debug!("generate_canvas_grids_from_logo_image, image width: {:?}, height: {:?}", img.width(), img.height());
    
    // 根据画布填充参数（填充的图形形状及尺寸），grid选择策略，生成画布上的grid信息
    let canvas_width = img.width();
    let canvas_height = img.height();
    let mut grids = generate_enmty_canvas_grids(canvas_width, canvas_height, fill_options)?;

    debug!("generate_enmty_canvas_grids, grids: {:?}", grids);

    // 根据指定的格子选取策略，从画布上挑选出拼接成logo的所有图形
    
    pick_grids_by_strategy(&img, &mut grids, pick_strategy)?;
    
    Ok(grids)
}



fn pick_grids_by_strategy(
    img: &RgbaImage, 
    grids: &mut Vec<Grid>, 
    pick_strategy: GridPickCmd) -> Result<()> {

    for grid in grids {
        match pick_strategy {
            GridPickCmd::AvgColorCompare(param) => {
                // fill_avg_color(img, grids)
                let avg_color = calc_avg_color_of_grid(img, grid)?;
                grid.ext.avg_color = Some(avg_color);
                // 计算差值
                let distance = calc_color_distance(avg_color.to_rgb(), param.color.to_rgb());
                grid.ext.color_distance = Some(distance);
                let selected = distance >= param.min_distance && distance <= param.max_distance;
                grid.ext.selected = Some(selected);

            },
            GridPickCmd::EliminateBgColor(param) => {
                let remaining_area_ratio = calc_remaining_area_ratio_in_grid(img, grid, param.color)?;
                grid.ext.remaining_area_ratio = Some(remaining_area_ratio);
                let selected =  remaining_area_ratio >= param.min_remaining_ratio;
                grid.ext.selected = Some(selected);
            },
        }
    }
    Ok(())
    
}

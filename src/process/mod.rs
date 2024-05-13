mod canvas_generate;
mod canvas_grid_pick;
mod image_process;




use image::{ImageBuffer, Rgba, RgbaImage};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::{Grid, GridFillOptions, GridPickStrategy, Point, Value, GRID_EXT_COLOR_DISTANCE, GRID_EXT_REMAINING_AREA_RATIO, GRID_EXT_SELECTED};

pub use image_process::{draw_empty_canvas, load_all_image_info, draw_canvas_with_grids};
pub use canvas_generate::{get_all_triangles_of_canvas, get_canvas_grids_filled_with_trianles};
use self::canvas_grid_pick::{calc_avg_color_of_grid, calc_remaining_area_ratio_in_grid, calculate_color_diff};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub path: String,
    pub bg_color: (u8, u8, u8),
}



pub fn generate_canvas_grids_by_image_path(
    image_path: &str, 
    fill_options: GridFillOptions, 
    pick_strategy: GridPickStrategy) -> Result<Vec<Grid>>  {
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
    pick_strategy: GridPickStrategy) -> Result<Vec<Grid>> {

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



/// 生成空画布信息
fn generate_enmty_canvas_grids(canvas_width: u32, canvas_height: u32, options: GridFillOptions) -> Result<Vec<Grid>> {
    match options {
        GridFillOptions::Triangle(w, h) => {
            get_canvas_grids_filled_with_trianles(canvas_width, canvas_height, w, h)
        }
    }
}


fn pick_grids_by_strategy(
    img: &RgbaImage, 
    grids: &mut Vec<Grid>, 
    pick_strategy: GridPickStrategy) -> Result<()> {

    for grid in grids {
        match pick_strategy {
            GridPickStrategy::AvgColorCompare(param) => {
                // fill_avg_color(img, grids)
                let avg_color = calc_avg_color_of_grid(img, grid)?;
                grid.ext.insert("avg_color".into(), Value::Rgb(avg_color));
                // 计算差值
                let distance = calculate_color_diff(avg_color, param.color);
                grid.ext.insert(GRID_EXT_COLOR_DISTANCE.into(), Value::F32(distance));
                if distance >= param.min_distance && distance <= param.max_distance {
                    grid.ext.insert(GRID_EXT_SELECTED.into(), Value::Bool(true));
                }

            },
            GridPickStrategy::EliminateBgColor(param) => {
                let remaining_area_ratio = calc_remaining_area_ratio_in_grid(img, grid, param.color)?;
                grid.ext.insert(GRID_EXT_REMAINING_AREA_RATIO.into(), Value::F32(remaining_area_ratio));
                if remaining_area_ratio >= param.min_remaining_ratio {
                    grid.ext.insert(GRID_EXT_SELECTED.into(), Value::Bool(true));
                }
            },
        }
    }
    Ok(())
    
}

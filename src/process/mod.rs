mod canvas_generate;
mod canvas_grid_pick;
mod image_process;




use image::{ImageBuffer, Rgba, RgbaImage};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::{Grid, GridFillOptions, GridPickStrategy, Point};

pub use image_process::{draw_empty_canvas, load_all_image_info};
pub use canvas_generate::{get_all_triangles_of_canvas, get_canvas_grids_filled_with_trianles};
pub use canvas_grid_pick::{fill_avg_color, fill_remaining_area_ratio_after_eliminate_bg_color, calculate_color_diff};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub path: String,
    pub bg_color: (u8, u8, u8),
}




/// 指定画布的宽高，以及填充画布的图形信息（形状及尺寸）生成画布上的所有图形的坐标
pub fn generate_canvas_grids(canvas_width: u32, canvas_height: u32, options: GridFillOptions) -> Result<Vec<Vec<Point>>> {
    match options {
        GridFillOptions::Triangle(w, h) => {
            get_all_triangles_of_canvas(canvas_width, canvas_height, w, h)
        }
    }
}

/// 依据指定logo图，从画布上挑选出拼接成该图中logo形状的所有图形，logo图的宽高和画布宽高一致
// pub fn pick_shapes_from_canvas(shapes: Vec<Vec<Point>>, image_path: &str, strategy: GridPickStrategy) -> Result<Vec<Vec<Point>>> {
//     // TODO: 暂时只有三角形
//     pick_triangles_from_canvas(shapes, image_path, strategy)
    
// }




// /// 生成画布格子，每个格子内包含可提供筛选的信息。
// pub fn generate_canvas_grids_with_pick_strategy(image_path: &str, fill_options: GridFillOptions, pick_strategy: GridPickStrategy) -> Result<Vec<Grid>> {
//     // 加载图片，确定画布的宽高
//     match options {
//         GridFillOptions::Triangle(w, h) => {
//             get_canvas_grids_filled_with_trianles(canvas_width, canvas_height, w, h)
//         }
//     }
// }



/// 依据指定logo图，从画布上挑选出拼接成该图中logo形状的所有图形，logo图的宽高和画布宽高一致
pub fn generate_canvas_grids_from_logo_image(image_path: &str, fill_options: GridFillOptions, pick_strategy: GridPickStrategy) -> Result<Vec<Grid>> {
    // 加载图片，确定画布的宽高
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(image_path).unwrap().to_rgba8();
    debug!("generate_canvas_grids_from_logo_image, image width: {:?}, height: {:?}", img.width(), img.height());
    
    // 根据画布填充参数（填充的图形形状及尺寸），grid选择策略，生成画布上的grid信息
    let canvas_width = img.width();
    let canvas_height = img.height();
    let mut grids = generate_enmty_canvas_grids(canvas_width, canvas_height, fill_options)?;

    debug!("generate_enmty_canvas_grids, grids: {:?}", grids);

    // 根据指定的格子选取策略，从画布上挑选出拼接成logo的所有图形
    fill_pick_info_for_grids(&img, &mut grids,  pick_strategy)?;

    // 挑选格子
    pick_grids(&mut grids,  pick_strategy)?;
    
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


/// 填充格子的选取信息
fn fill_pick_info_for_grids(img: &RgbaImage, grids: &mut Vec<Grid>, pick_strategy: GridPickStrategy) -> Result<()> {
    match pick_strategy {
        // 如果根据平均颜色比较的策略选取格子，则添加格子的平均颜色信息
        GridPickStrategy::AvgColorCompare(_) => {
            fill_avg_color(img, grids)
        },
        // 如果根据背景颜色消除的策略选取格子，则添加格子消除背景颜色后剩余区域的占比信息
        GridPickStrategy::EliminateBgColor(param) => {
            fill_remaining_area_ratio_after_eliminate_bg_color(img, param.color, grids)
        },

    }
}

fn pick_grids(grids: &mut Vec<Grid>, pick_strategy: GridPickStrategy) -> Result<()> {
    for grid in grids.iter_mut() {
        grid.pick(&pick_strategy)?
    }

    Ok(())

}
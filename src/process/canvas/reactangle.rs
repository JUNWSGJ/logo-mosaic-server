use anyhow::Result;
use image::RgbaImage;

use crate::{calc_color_distance, Color, Grid, GridShape, Point};

/// 生成由三角形格子填充的画布的所有格子信息
pub fn genarate_canvas_grids_filled_with_rectanles(
    canvas_width: u32,
    canvas_height: u32,
    rect_width: u32,
    rect_height: u32,
)-> Result<Vec<Grid>>{

    let rows = canvas_height / rect_height as u32;
    let mut grids = Vec::with_capacity(1024);

    // 绘制矩形
    for row in 0..rows {
        let mut x = 0;
        let y = row * rect_height;
        let mut seq = 0;
        while x + rect_width <= canvas_width {
            // 计算矩形四个顶点坐标
            let points = vec![
                Point::new(x, y ),
                Point::new( x + rect_width, y ),
                Point::new( x + rect_width, y + rect_height ),
                Point::new( x, y + rect_height ),
            ].iter().cloned().collect();
            // println!("绘制倒三角形, 三点坐标: {:?}", &points);
            seq += 1;
            grids.push(Grid{
                seq: format!("R{}C{}", row+1, seq),
                shape: GridShape::Rectangle,
                points,
                ext: Default::default(),
            });
            x = x + rect_width;
        }
        
    }
    Ok(grids)
    
}




/// 计算矩形区域的平均色值
pub fn calc_average_color_in_rectangle(img: &RgbaImage, points: &Vec<Point>) -> Result<Color> {

    assert_eq!(points.len(), 4, "Each rectangle should have 4 points.");
    // 计算矩形所在最小矩形区域的边界点，用于遍历该区域内所有的像素点
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    
    let mut total_r = 0;
    let mut total_g = 0;
    let mut total_b = 0;
    let mut total_a = 0;
    let mut pixel_count = 0;

    // println!("三角形坐标区域，min_x:{}, min_y:{}, max_x:{}, max_y:{}", min_x, min_y, max_x, max_y);

    for x in min_x..max_x {
        for y in min_y..max_y {
            let pixel = img.get_pixel(x, y);
            total_r += pixel[0] as u32;
            total_g += pixel[1] as u32;
            total_b += pixel[2] as u32;
            total_a += pixel[3] as u32;
            pixel_count += 1;
        }
    }

    if pixel_count == 0 {
        anyhow::bail!("The rectangle does not cover any pixels.");
    }

    Ok(Color::from_rgba((
        (total_r as f32 / pixel_count as f32 ) as u8,
        (total_g as f32 / pixel_count as f32 ) as u8,
        (total_b as f32 / pixel_count as f32 ) as u8,
        (total_a as f32 / pixel_count as f32 ) as u8,
    )))
}



/// 计算三角形区域剔除掉背景色后的剩余区域占比
pub fn calc_remaining_area_ratio_in_rectangle(img: &RgbaImage, bg_color: Color, points: &Vec<Point>) -> Result<f32> {
    // 计算三角形边界框，用于遍历
    assert_eq!(points.len(), 4, "Each rectangle should have 4 points.");
    // 计算矩形所在最小矩形区域的边界点，用于遍历该区域内所有的像素点
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    // 将坐标点转为imageproc的Point类型
    let mut total_pixel_count = 0;
    let mut remaining_count = 0;

    for x in min_x..max_x {
        for y in min_y..max_y {
            let pixel = img.get_pixel(x, y);
            total_pixel_count += 1;
            let color1 = (pixel[0], pixel[1], pixel[2]);
            let color2 = (bg_color.0 .0, bg_color.0 .1, bg_color.0 .2);
            let distance = calc_color_distance(color1, color2);
            if distance > 5.0{
                remaining_count += 1;
            }
        }
    }

    if total_pixel_count == 0 {
        anyhow::bail!("The rectangle does not cover any pixels.");
    }

    Ok(remaining_count as f32 / total_pixel_count as f32)

}
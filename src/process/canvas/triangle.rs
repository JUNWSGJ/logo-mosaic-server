use anyhow::Result;
use image::RgbaImage;
use tracing::error;
use crate::{calc_color_distance, Color, Grid, GridShape, Point};


/// 生成由三角形格子填充的画布的所有格子信息
pub fn genarate_canvas_grids_filled_with_trianles(
    canvas_width: u32,
    canvas_height: u32,
    triangle_width: u32,
    triangle_height: u32,
)-> Result<Vec<Grid>>{

    let rows = canvas_height / triangle_height as u32;
    let mut grids = Vec::with_capacity(1024);

    // 绘制三角形，第一行倒三角形开头，第二行则为正三角形开头
    for row in 0..rows {
        // 绘制倒三角形，计算起始x坐标和y坐标
        let mut x = if row % 2 == 0 { 0 } else { triangle_width / 2 };
        let y = row * triangle_height;

        let mut seq = 0;
        while x + triangle_width <= canvas_width {
            // 计算三角形三个顶点坐标
            let points = get_down_triangle_points(x, y, triangle_width, triangle_height);
            // println!("绘制倒三角形, 三点坐标: {:?}", &points);
            seq += 1;
            grids.push(Grid{
                seq: format!("R{}D{}", row+1, seq),
                shape: GridShape::Triangle,
                points: points.to_vec(),
                ext: Default::default(),
            });
            x = x + triangle_width;
        }

        // 绘制正三角形
        seq = 0;
        let mut x = if row % 2 == 0 { triangle_width / 2 } else { 0 };
        let y = (row + 1) * triangle_height;
        while x + triangle_width <= canvas_width {
            let points = get_up_triangle_points(x, y, triangle_width, triangle_height);
            // println!("绘制正三角形, 三点坐标: {:?}", &points);
            seq += 1;
            grids.push(Grid{
                seq: format!("R{}U{}", row+1, seq),
                shape: GridShape::Triangle,
                points: points.to_vec(),
                ext: Default::default(),
            });
            x = x + triangle_width;
        }
    }


    Ok(grids)
}

/// 计算三角形区域的平均色值
pub fn calc_average_color_in_triangle(img: &RgbaImage, points: &Vec<Point>) -> Result<Color> {

    assert_eq!(points.len(), 3, "Each polygon should have 3 points.");
    let triangle:[(u32, u32); 3] = [
        (points[0].x, points[0].y),
        (points[1].x, points[1].y),
        (points[2].x, points[2].y),
    ];
    // 计算三角形所在最小矩形区域的边界点，用于遍历该区域内所有的像素点
    let (min_x, min_y, max_x, max_y) = triangle.iter().fold(
        (std::u32::MAX, std::u32::MAX, 0, 0),
        |(min_x, min_y, max_x, max_y), &(x, y)| {
            (
                min_x.min(x as u32),
                min_y.min(y as u32),
                max_x.max(x as u32),
                max_y.max(y as u32),
            )
        },
    );

    let mut total_r = 0;
    let mut total_g = 0;
    let mut total_b = 0;
    let mut total_a = 0;
    let mut pixel_count = 0;

    // println!("三角形坐标区域，min_x:{}, min_y:{}, max_x:{}, max_y:{}", min_x, min_y, max_x, max_y);

    // 将坐标点转为imageproc的Point类型
    let points: [Point; 3] = triangle.map(|(x, y)| Point::new(x, y));

    for x in min_x..max_x {
        for y in min_y..max_y {
            if is_point_inside_triangle((x as f32, y as f32), points) {

                let pixel = img.get_pixel(x, y);
                total_r += pixel[0] as u32;
                total_g += pixel[1] as u32;
                total_b += pixel[2] as u32;
                total_a += pixel[3] as u32;
                pixel_count += 1;
            }
        }
    }

    if pixel_count == 0 {
        error!("The triangle does not cover any pixels, triangle:{:?}", triangle);
        panic!("The triangle does not cover any pixels in the image.");
    }

    let rgba = (
        (total_r as f32 / pixel_count as f32 ) as u8,
        (total_g as f32 / pixel_count as f32 ) as u8,
        (total_b as f32 / pixel_count as f32 ) as u8,
        (total_a as f32 / pixel_count as f32 ) as u8,
    );
    Ok(Color::from_rgba(rgba))
}


/// 计算三角形区域剔除掉背景色后的剩余区域占比
pub fn calc_remaining_area_ratio_in_triangle(img: &RgbaImage, bg_color: Color, triangle: [(u32, u32); 3]) -> Result<f32> {
    // 计算三角形边界框，用于遍历
    let (min_x, min_y, max_x, max_y) = triangle.iter().fold(
        (std::u32::MAX, std::u32::MAX, 0, 0),
        |(min_x, min_y, max_x, max_y), &(x, y)| {
            (
                min_x.min(x as u32),
                min_y.min(y as u32),
                max_x.max(x as u32),
                max_y.max(y as u32),
            )
        },
    );

    // 将坐标点转为imageproc的Point类型
    let points: [Point; 3] = triangle.map(|(x, y)| Point::new(x, y));

    let mut total_pixel_count = 0;
    let mut remaining_count = 0;

    for x in min_x..max_x {
        for y in min_y..max_y {
            if is_point_inside_triangle((x as f32, y as f32), points) {
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
    }

    if total_pixel_count == 0 {
        println!("三角形区域未覆盖任何点，triangle:{:?}", triangle);
        panic!("The triangle does not cover any pixels in the image.");
    }

    Ok(remaining_count as f32 / total_pixel_count as f32)

}


/// 判断点是否在三角形内部
fn is_point_inside_triangle(point: (f32, f32), vertices: [Point; 3]) -> bool {
    // 这里简化处理，直接使用向量叉乘法判断，实际应用中可能需要更精确的判断方法
    let (x, y) = point;
    let (v0, v1, v2) = (vertices[0], vertices[1], vertices[2]);
    let a = ((v1.y as f32 - v0.y as f32) * (x - v0.x as f32) + (v0.x as f32 - v1.x as f32) * (y - v0.y as f32)) > 0.0;
    let b = ((v2.y as f32 - v1.y as f32) * (x - v1.x as f32) + (v1.x as f32 - v2.x as f32) * (y - v1.y as f32)) > 0.0;
    let c = ((v0.y as f32 - v2.y as f32) * (x - v2.x as f32) + (v2.x as f32 - v0.x as f32) * (y - v2.y as f32)) > 0.0;
    a == b && b == c
}




///计算倒三角形的三角形顶点坐标； 传入水平方向的第一个点的坐标，及三角形的边长和高，返回三个点的坐标
fn get_down_triangle_points(x: u32, y: u32, side: u32, height: u32) -> [Point; 3] {
    [
        Point { x, y },
        Point { x: x + side, y },
        Point {
            x: x + side / 2,
            y: y + height,
        },
    ]
}
/// 计算正三角形的三角形顶点坐标；传入水平方向的第一个点的坐标，及三角形的边长和高，返回三个点的坐标
fn get_up_triangle_points(x: u32, y: u32, width: u32, height: u32) -> [Point; 3] {
    [
        Point { x, y },
        Point { x: x + width, y },
        Point {
            x: x + width / 2,
            y: y - height,
        },
    ]
}

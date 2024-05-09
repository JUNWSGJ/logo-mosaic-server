use std::path::PathBuf;

// use image::RgbImage;
// use imageproc::point::Point;
use image::{ImageBuffer, Rgba};
use anyhow::Result;
use imageproc::drawing::{draw_hollow_polygon_mut, draw_polygon_mut};

use crate::Point;
// use crate::TrianglePoints;


/// 画初始画布
pub fn draw_init_canvas(
    canvas_width: u32, 
    canvas_height: u32,
    bg_color: Rgba<u8>,
    polyon_color: Rgba<u8>,
    polyon_board_color: Rgba<u8>,
    polygons: Vec<Vec<Point>>,
    path: PathBuf,
) -> Result<()> {

    // 设置矩形画布的尺寸,背景色
    // "#373737"
    // "#9099A2"
    // "#984B43"
    // "#EAC67A"


    // 创建一个新的空白画布
    let mut img = ImageBuffer::from_pixel(canvas_width,  canvas_height, bg_color);

    for polygon in polygons.into_iter() {
        //填充多边形
        let mut points: Vec<imageproc::point::Point<i32>> = Vec::with_capacity(polygon.len());
        for point in polygon.iter() {
            points.push(imageproc::point::Point {
                x: point.x as i32,
                y: point.y as i32,
            });
        }
        println!("正在绘制多边形：{:?}", polygon);
        draw_polygon_mut(&mut img, &points, polyon_color);

        // 画多边形的边框
        let mut points: Vec<imageproc::point::Point<f32>> = Vec::with_capacity(polygon.len());
        for point in polygon.iter() {
            points.push(imageproc::point::Point {
                x: point.x as f32,
                y: point.y as f32,
            });
        }
        draw_hollow_polygon_mut(&mut img, &points, polyon_board_color);
    }

    // 保存图像
    img.save(path.clone())?;
    println!("图片保存成功,路径：{:?}！", path);
    Ok(())
}



// pub fn filter_triangles_with_image(&img: &RgbImage, triangles: &Vec<TrianglePoints>) -> Vec<TrianglePoints>{
//     let x = img.par_pixels().map(|(x, y, pixel)| {
//         let avg_color = average_color_in_triangle(&img, triangle);
//         let diff = color_diff(avg_color, target_color);
//         (x, y, diff)
//     }).collect()

// }


// fn average_color_in_triangle(image: &RgbImage, triangle: TrianglePoints) -> [f32; 3] {
//     // 将坐标点转为imageproc的Point类型
//     let [p1, p2, p3] = triangle.0;
//     let points: [Point<f32>; 3] = [
//         Point::new(p1.x as f32, p1.y as f32),
//         Point::new(p2.x as f32, p2.y as f32),
//         Point::new(p3.x as f32, p3.y as f32),
//     ];

//     // 计算三角形边界框，用于遍历
//     let (min_x, min_y, max_x, max_y) = points.iter().fold(
//         (std::u32::MAX, std::u32::MAX, 0, 0),
//         |(min_x, min_y, max_x, max_y), &(x, y)| {
//             (
//                 min_x.min(x as u32),
//                 min_y.min(y as u32),
//                 max_x.max(x as u32),
//                 max_y.max(y as u32),
//             )
//         },
//     );

//     let mut total_r = 0;
//     let mut total_g = 0;
//     let mut total_b = 0;
//     let mut pixel_count = 0;

//     for x in min_x..=max_x {
//         for y in min_y..=max_y {
//             if is_point_inside_triangle((x as f32, y as f32), points) {
//                 let pixel = image.get_pixel(x, y);
//                 total_r += pixel[0] as u32;
//                 total_g += pixel[1] as u32;
//                 total_b += pixel[2] as u32;
//                 pixel_count += 1;
//             }
//         }
//     }

//     if pixel_count == 0 {
//         panic!("The triangle does not cover any pixels in the image.");
//     }

//     [
//         total_r as f32 / pixel_count as f32,
//         total_g as f32 / pixel_count as f32,
//         total_b as f32 / pixel_count as f32,
//     ]
// }

// // 判断点是否在三角形内部
// fn is_point_inside_triangle(point: (f32, f32), vertices: [Point<f32>; 3]) -> bool {
//     // 这里简化处理，直接使用向量叉乘法判断，实际应用中可能需要更精确的判断方法
//     let (x, y) = point;
//     let (v0, v1, v2) = (vertices[0], vertices[1], vertices[2]);
//     let a = ((v1.y - v0.y) * (x - v0.x) + (v0.x - v1.x) * (y - v0.y)) > 0.0;
//     let b = ((v2.y - v1.y) * (x - v1.x) + (v1.x - v2.x) * (y - v1.y)) > 0.0;
//     let c = ((v0.y - v2.y) * (x - v2.x) + (v2.x - v0.x) * (y - v2.y)) > 0.0;
//     a == b && b == c
// }

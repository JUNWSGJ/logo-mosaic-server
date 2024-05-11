use anyhow::Result;
use image::{RgbImage, ImageBuffer, Rgb};

use crate::{AvgColorCompareParam, Point, ShapePickStrategy};

pub fn pick_triangles_from_canvas(triangles: Vec<Vec<Point>>, image_path: &str, strategy: ShapePickStrategy) -> Result<Vec<Vec<Point>>> {
    // 加载图片并转换为RgbImage
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = image::open(image_path)?.to_rgb8();

    match strategy {
        ShapePickStrategy::AvgColorCompare(param) => {
            pick_triangles_by_avg_color(&img, &triangles, param)
        }
    }

}



fn pick_triangles_by_avg_color(img: &RgbImage, triangles: &Vec<Vec<Point>>, param: AvgColorCompareParam )-> Result<Vec<Vec<Point>>>{
    
    let mut picked_triangles = Vec::with_capacity(triangles.len());

    let target_color = (param.target.0 as f32, param.target.1 as  f32,param.target.2 as f32);
    // 定义三角形顶点坐标
    triangles.iter().for_each(|points| {
        assert_eq!(points.len(), 3, "Each polygon should have 3 points.");
        let triangle:[(u32, u32); 3] = [
            (points[0].x, points[0].y),
            (points[1].x, points[1].y),
            (points[2].x, points[2].y),
        ];
        // 计算平均色值
        
        let avg_color = average_color_in_triangle(img, triangle);
        
        // 根据平均色值，判断是否需要保留
        let distance = calculate_color_diff(avg_color, target_color);
        let (min_value, max_value) = param.distance_range;
        println!(">>>>>>计算三角形平均色值, 平均色值:{:?}, 差值:{:?}, 三角形点坐标: {:?},", avg_color, distance, points);
        if distance >= min_value && distance <= max_value {
            picked_triangles.push(points.clone());
        }
    });

    Ok(picked_triangles)

}



fn average_color_in_triangle(image: &RgbImage, triangle: [(u32, u32); 3]) -> (f32, f32, f32) {

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

    let mut total_r = 0;
    let mut total_g = 0;
    let mut total_b = 0;
    let mut pixel_count = 0;

    // println!("三角形坐标区域，min_x:{}, min_y:{}, max_x:{}, max_y:{}", min_x, min_y, max_x, max_y);

    // 将坐标点转为imageproc的Point类型
    let points: [Point; 3] = triangle.map(|(x, y)| Point::new(x, y));

    for x in min_x..max_x {
        for y in min_y..max_y {
            if is_point_inside_triangle((x as f32, y as f32), points) {

                let pixel = image.get_pixel(x, y);
                total_r += pixel[0] as u32;
                total_g += pixel[1] as u32;
                total_b += pixel[2] as u32;
                pixel_count += 1;
            }
        }
    }

    if pixel_count == 0 {
        println!("三角形区域未覆盖任何点，triangle:{:?}", triangle);
        panic!("The triangle does not cover any pixels in the image.");
    }

    (
        total_r as f32 / pixel_count as f32,
        total_g as f32 / pixel_count as f32,
        total_b as f32 / pixel_count as f32,
    )
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


/// 计算两个色值的差异,返回值范围:[0,1]
/// 当输入的两个色值完全相同时，返回值为0
/// 当输入的两个色值完全不同时，返回值为1
fn calculate_color_diff(color1: (f32,f32,f32), color2: (f32,f32,f32)) -> f32{
    const SQRT_3: f32 = 1.7320508075688772; // 直接定义sqrt(3)

    // 将RGB值从[0, 255]转换为[0, 1]
    let color1_normalized = (
        color1.0 as f32 / 255.0,
        color1.1 as f32 / 255.0,
        color1.2 as f32 / 255.0,
    );
    let color2_normalized = (
        color2.0 as f32 / 255.0,
        color2.1 as f32 / 255.0,
        color2.2 as f32 / 255.0,
    );

    // 分别计算R、G、B三个通道的差值的平方
    let diff_r = (color1_normalized.0 - color2_normalized.0).powi(2);
    let diff_g = (color1_normalized.1 - color2_normalized.1).powi(2);
    let diff_b = (color1_normalized.2 - color2_normalized.2).powi(2);
    
    // 欧氏距离的平方根，然后归一化到[0, 1]区间
    ((diff_r + diff_g + diff_b).sqrt() / SQRT_3).clamp(0.0, 1.0)
}

// test
#[cfg(test)]
#[test]
fn test_calculate_color_diff(){
    let color1 = (255.0, 255.0, 255.0);
    let color2 = (0.0, 0.0, 0.0);
    let diff = calculate_color_diff(color1, color2);
    assert_eq!(diff, 1.0);

}
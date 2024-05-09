use anyhow::Result;
use image::{GenericImage, RgbImage};

fn main() -> Result<()> {
    // 加载图片并转换为RgbImage
    let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::open("assets/logo1.png")?.to_rgb8();

    let height = img.height();
    let width = img.width();
    println!("image width:{}, height: {}", height, width);

    // 定义三角形顶点坐标
    // let triangle = [(100, 100), (200, 100), (150, 200)];

    // 计算平均色值
    // let avg_color = average_color_in_triangle(&img, triangle);
    // println!("{:?}", avg_color);
    Ok(())
}

// fn average_color_in_triangle(image: &RgbImage, triangle: [(u32, u32); 3]) -> [f32; 3] {
//     // 将坐标点转为imageproc的Point类型
//     let points: [ImageProcPoint; 3] = triangle.map(|(x, y)| ImageProcPoint::new(x as f32, y as f32));

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
// fn is_point_inside_triangle(point: (f32, f32), vertices: [ImageProcPoint; 3]) -> bool {
//     // 这里简化处理，直接使用向量叉乘法判断，实际应用中可能需要更精确的判断方法
//     let (x, y) = point;
//     let (v0, v1, v2) = (vertices[0], vertices[1], vertices[2]);
//     let a = ((v1.y - v0.y) * (x - v0.x) + (v0.x - v1.x) * (y - v0.y)) > 0.0;
//     let b = ((v2.y - v1.y) * (x - v1.x) + (v1.x - v2.x) * (y - v1.y)) > 0.0;
//     let c = ((v0.y - v2.y) * (x - v2.x) + (v2.x - v0.x) * (y - v2.y)) > 0.0;
//     a == b && b == c
// }

use anyhow::Result;
use image::{RgbImage, ImageBuffer, Rgb};
use logo_process::{generate_canvas_grids, GridFillOptions, Point};

fn main() -> Result<()> {
    // 加载图片并转换为RgbImage
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = image::open("images/logo1.png")?.to_rgb8();

    let canvas_width = img.width();
    let canvas_height = img.height();
    println!("image width:{}, height: {}", canvas_width, canvas_height);
    let triangle_width = 50;
    let triangle_height = 40;

    let fill_options = GridFillOptions::Triangle(triangle_width, triangle_height);
    let polygons = generate_canvas_grids( canvas_width, canvas_height, fill_options)?;
    

    // 定义三角形顶点坐标
    polygons.iter().for_each(|points| {
        assert_eq!(points.len(), 3, "Each polygon should have 3 points.");
        let triangle:[(u32, u32); 3] = [
            (points[0].x, points[0].y),
            (points[1].x, points[1].y),
            (points[2].x, points[2].y),
        ];
        // 计算平均色值
        let avg_color = average_color_in_triangle(&img, triangle);
        println!("triangle: {:?}, avg_color:{:?}", points, avg_color);
    });

    
    Ok(())
}

fn average_color_in_triangle(image: &RgbImage, triangle: [(u32, u32); 3]) -> [f32; 3] {
    

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

    // 将坐标点转为imageproc的Point类型
    let points: [Point; 3] = triangle.map(|(x, y)| Point::new(x, y));

    for x in min_x..=max_x {
        for y in min_y..=max_y {
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
        panic!("The triangle does not cover any pixels in the image.");
    }

    [
        total_r as f32 / pixel_count as f32,
        total_g as f32 / pixel_count as f32,
        total_b as f32 / pixel_count as f32,
    ]
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

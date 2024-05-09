use anyhow::Result;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::{draw_hollow_polygon_mut, draw_polygon_mut};
use logo_process::*;





fn main() -> Result<()> {
    // 设置矩形画布的尺寸,背景色
    // "#373737"
    // "#9099A2"
    // "#984B43"
    // "#EAC67A"

    let canvas_width = 500;
    let canvas_height = 400;
    let bg_color = Rgba([0u8, 0u8, 0u8, 255u8]);

    // 设置三角形的水平边长和高度
    let triangle_width = 50;
    let triangle_height = 40;

    println!("triangle, side: {}, height: {}", triangle_width, triangle_height);
    let fill_options = FillShapeOptions::Triangle(triangle_width, triangle_height);
    let triangles = generate_canvas_shapes(fill_options, canvas_width, canvas_height)?;



    //读取logo图片
    // let logo_img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::open("assets/logo1.png")?.to_rgb8();

    // let filtered_triangles = filter_triangles_with_image(&logo_img, &triangles);


    // 创建一个新的空白画布
    let mut img = ImageBuffer::from_pixel(canvas_width, canvas_height, bg_color);

    for triangle_points in triangles.into_iter() {
        //填充三角形
        let mut points: Vec<imageproc::point::Point<i32>> = Vec::with_capacity(3);
        for point in triangle_points.iter() {
            points.push(imageproc::point::Point {
                x: point.x as i32,
                y: point.y as i32,
            });
        }
        println!("正在绘制三角形：{:?}", triangle_points);
        draw_polygon_mut(&mut img, &points, Rgba([128, 128, 128, 100]));

        // 画三角形的边框
        let mut points: Vec<imageproc::point::Point<f32>> = Vec::with_capacity(3);
        for point in triangle_points.iter() {
            points.push(imageproc::point::Point {
                x: point.x as f32,
                y: point.y as f32,
            });
        }
        draw_hollow_polygon_mut(&mut img, &points, Rgba([255u8, 255u8, 255u8, 100u8]));
    }

    // 保存图像
    img.save("output/triangles.png")?;
    println!("图片保存成功！");
    Ok(())
}

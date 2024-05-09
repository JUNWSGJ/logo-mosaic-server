use image::{ImageBuffer, Rgba};
use imageproc::{drawing::draw_polygon_mut, point::Point};

fn main() {
    // 创建一个800x600的灰色背景图像
    let mut img = ImageBuffer::from_pixel(800, 600, Rgba([128u8, 128u8, 128u8, 255u8]));

    // 在图像上绘制一个绿色三角形
    let triangle1_points = vec![Point::new(0, 0), Point::new(100, 0), Point::new(50, 87)];
    draw_polygon_mut(&mut img, &triangle1_points, Rgba([0u8, 255u8, 0u8, 255u8]));

    let triangle2_points = vec![Point::new(50, 87), Point::new(150, 87), Point::new(100, 0)];
    draw_polygon_mut(&mut img, &triangle2_points, Rgba([255u8, 0u8, 0u8, 255u8]));

    // 保存图像为PNG
    img.save("demo2.png").unwrap();
}




use image::{ImageBuffer, Rgba};
use imageproc::{drawing::draw_polygon_mut, point::Point};

fn main() {
    // 创建一个800x600的灰色背景图像

    // let mut img = ImageBuffer::from_pixel(canvas_width, canvas_height, Rgba([128u8, 128u8, 128u8, 255u8]));
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open("images/logo2.png").unwrap().to_rgba8();
    println!(">>>>>>image width: {:?}, height: {:?}", img.width(), img.height());

    // 在图像上绘制一个绿色三角形
    let triangle1_points = vec![Point::new(0, 720), Point::new(50, 720), Point::new(25, 760)];
    draw_polygon_mut(&mut img, &triangle1_points, Rgba([0u8, 255u8, 0u8, 255u8]));

    // let triangle2_points = vec![Point::new(50, 87), Point::new(150, 87), Point::new(100, 0)];
    // draw_polygon_mut(&mut img, &triangle2_points, Rgba([255u8, 0u8, 0u8, 255u8]));

    // 保存图像为PNG
    img.save("demo2.png").unwrap();
    
}




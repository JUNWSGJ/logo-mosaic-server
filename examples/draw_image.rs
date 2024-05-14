use anyhow::Result;
use image::{ImageBuffer, Rgba};
use imageproc::{drawing::draw_polygon_mut, point::Point};
use logo_process::Color;

fn main() -> Result<()>  {
    // 创建一个800x600的灰色背景图像

    let fill_color = Into::<Color>::into("#aaff55ff");
    let mut img = ImageBuffer::from_pixel(800, 600, Rgba([128u8, 128u8, 128u8, 255u8]));
    println!(">>>>>>image width: {:?}, height: {:?}", img.width(), img.height());

    // 在图像上绘制一个绿色三角形
    let triangle = vec![Point::new(100, 100), Point::new(200, 100), Point::new(150, 200)];
    
    draw_polygon_mut(&mut img, &triangle, fill_color.into());


    // 保存图像为PNG
    img.save("./output/simple_demo.png")?;
    println!(">>>>>>image saved as output/simple_demo.png");
    Ok(())
    
}




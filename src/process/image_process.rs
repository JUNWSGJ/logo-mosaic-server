use std::path::{Path, PathBuf};

use dashmap::DashMap;
// use image::RgbImage;
// use imageproc::point::Point;
use image::{ImageBuffer, Rgba};
use anyhow::Result;
use imageproc::drawing::{draw_hollow_polygon_mut, draw_polygon_mut};

use crate::{ImageInfo, Point};
// use crate::TrianglePoints;


/// 从指定目录加载所有图片（png, jpeg）
pub fn load_all_image_info(logo_image_dir_path: &str) -> Result<DashMap<String, ImageInfo>> {
    
    let image_map = DashMap::new();
    let dir_path = Path::new(logo_image_dir_path);
    
    dir_path.read_dir()?.for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().unwrap().to_str().unwrap();
            if ext == "png" || ext == "jpeg" {
                // 加载图片并获取尺寸
                match image::open(path.clone()) {
                    Ok(img) => {
                        println!("Image: {:?}, Width: {}, Height: {}", path.display(), img.width(), img.height());
                        let image_name = path.file_name().unwrap().to_str().unwrap().to_string();
                        image_map.insert(image_name.clone(), ImageInfo {
                            id: image_name.clone(),
                            width: img.width(),
                            height: img.height(),
                            path: dir_path.join(image_name.clone()).to_str().unwrap().to_string(),
                            name: image_name,
                        });
                    },
                    Err(e) => {
                    eprintln!("Error loading image {}: {}", path.display(), e);
                    }
            }
            }
        }
    });
    Ok(image_map)
}



/// 画初始画布
pub fn draw_empty_canvas(
    canvas_width: u32, 
    canvas_height: u32,
    bg_color: Rgba<u8>,
    polygon_color: Rgba<u8>,
    polygon_board_color: Rgba<u8>,
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
        // println!("正在绘制多边形：{:?}", polygon);
        draw_polygon_mut(&mut img, &points, polygon_color);

        // 画多边形的边框
        let mut points: Vec<imageproc::point::Point<f32>> = Vec::with_capacity(polygon.len());
        for point in polygon.iter() {
            points.push(imageproc::point::Point {
                x: point.x as f32,
                y: point.y as f32,
            });
        }
        draw_hollow_polygon_mut(&mut img, &points, polygon_board_color);
    }

    // 保存图像
    img.save(path.clone())?;
    println!("图片保存成功,路径：{:?}！", path);
    Ok(())
}

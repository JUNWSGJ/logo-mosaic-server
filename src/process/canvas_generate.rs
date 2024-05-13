use crate::{Grid, GridShape, Point};
use anyhow::Result;


pub fn get_canvas_grids_filled_with_trianles(
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


pub fn get_all_triangles_of_canvas(
    canvas_width: u32,
    canvas_height: u32,
    triangle_width: u32,
    triangle_height: u32,
) -> Result<Vec<Vec<Point>>> {
    // 计算下整个画布有多少行
    let rows = canvas_height / triangle_height as u32;
    println!("一共有 {} 行", rows);

    let mut triangle_points: Vec<Vec<Point>> = Vec::with_capacity(1024);

    // 绘制三角形，第一行倒三角形开头，第二行则为正三角形开头
    for row in 0..rows {
        // 绘制倒三角形，计算起始x坐标和y坐标
        let mut x = if row % 2 == 0 { 0 } else { triangle_width / 2 };
        let y = row * triangle_height;
        while x + triangle_width <= canvas_width {
            // 计算三角形三个顶点坐标
            let points = get_down_triangle_points(x, y, triangle_width, triangle_height);
            // println!("绘制倒三角形, 三点坐标: {:?}", &points);
            triangle_points.push(points.to_vec());
            x = x + triangle_width;
        }

        // 绘制正三角形
        let mut x = if row % 2 == 0 { triangle_width / 2 } else { 0 };
        let y = (row + 1) * triangle_height;
        while x + triangle_width <= canvas_width {
            let points = get_up_triangle_points(x, y, triangle_width, triangle_height);
            // println!("绘制正三角形, 三点坐标: {:?}", &points);
            triangle_points.push(points.to_vec());
            x = x + triangle_width;
        }
    }
    Ok(triangle_points)
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

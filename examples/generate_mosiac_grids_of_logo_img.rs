use std::str::FromStr;

use logo_process::{draw_empty_canvas, generate_canvas_shapes, pick_shapes_from_canvas, AvgColorCompareParam, Color, FillShapeOptions, ShapePickStrategy};
use anyhow::Result;

fn main() -> Result<()>{
    let canvas_width = 968;
    let canvas_height = 698;
    let bg_color = Color::from_str("#373737")?.0;
    let polygon_color = Color::from_str("#9099A2")?.0;
    let polygon_board_color = Color::from_str("#ffffff")?.0;

    let canvas_shapes = generate_canvas_shapes(canvas_width, canvas_height, FillShapeOptions::Triangle(20, 16))?;

    let pick_strategy = ShapePickStrategy::AvgColorCompare(AvgColorCompareParam {
        target: (255, 255, 255),
        distance_range: (0.5, 1.0)
    });
    let picked_shapes = pick_shapes_from_canvas(canvas_shapes, "images/logo1.png", pick_strategy)?;

    println!(">>>>>>picked grids: {:?}", picked_shapes.len());


    draw_empty_canvas(canvas_width, canvas_height, bg_color, polygon_color, polygon_board_color, picked_shapes, "output/logo1_canvas3.png".into())?;

    Ok(())
}
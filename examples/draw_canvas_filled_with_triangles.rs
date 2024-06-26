use anyhow::Result;
use logo_process::{draw_canvas_with_grids, generate_enmty_canvas_grids, Color, GridFillOptions};


fn main() -> Result<()> {
    // 设置矩形画布的尺寸,背景色
    let canvas_width = 1000;
    let canvas_height = 800;
    let canvas_color = Into::<Color>::into("#373737ff").into();
    // 设置填充画布的三角形的尺寸和颜色
    let triangle_width = 50;
    let triangle_height = 40;
    let grid_fill_color = "#9099A2ff".into();
    let grid_border_color = "#ffffffff".into();
    let mut grids = generate_enmty_canvas_grids( 
        canvas_width, canvas_height, 
        GridFillOptions::Triangle(triangle_width, triangle_height)
    )?;
    grids.iter_mut().for_each(|grid| {
        grid.ext.border_color = Some(grid_border_color);
        grid.ext.fill_color = Some(grid_fill_color);
    });

    draw_canvas_with_grids(canvas_width, canvas_height, canvas_color, grids, "output/triangle_grids.png".into())?;
    Ok(())
}

use anyhow::Result;
use logo_process::{draw_canvas_with_grids, generate_enmty_canvas_grids, Color, GridFillOptions};


fn main() -> Result<()> {
    // 设置画布的尺寸,颜色
    let canvas_width = 1000;
    let canvas_height = 800;
    let canvas_color = Into::<Color>::into("#373737ff").into();
    // 设置填充画布的矩形的尺寸和颜色
    let grid_fill_color = "#9099A2ff".into();
    let grid_border_color = "#ffffffff".into();
    let rectangle_width = 50;
    let rectangle_height = 50;

    //生成格子
    let mut grids = generate_enmty_canvas_grids( 
        canvas_width, canvas_height,
        GridFillOptions::Rectangle(rectangle_width, rectangle_height))?;
    // 设置格子颜色
    grids.iter_mut().for_each(|grid| {
        grid.ext.border_color = Some(grid_border_color);
        grid.ext.fill_color = Some(grid_fill_color);
    });
    //在画布上画格子
    draw_canvas_with_grids(
        canvas_width, 
        canvas_height, 
        canvas_color,
        grids, 
        "output/restangle_grids.png".into())?;
    Ok(())
}

use image::{ImageBuffer, Rgba};
use logo_process::{draw_canvas_with_grids, generate_canvas_grids_from_logo_image, AvgColorCompareParam, Color, GridFillOptions, GridPickCmd};
use anyhow::Result;

fn main() -> Result<()>{

    let image_path = "images/logo1.png";
    let fill_options = GridFillOptions::Triangle(10, 8);
    // let fill_options = GridFillOptions::Rectangle(10, 10);
    let pick_strategy = GridPickCmd::AvgColorCompare(AvgColorCompareParam {
        color: Into::<Color>::into("#ffffffff"),
        min_distance: 50.0,
        max_distance: 100.0
    });

    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = image::open(image_path).unwrap().to_rgba8();
    let canvas_width = img.width();
    let canvas_height = img.height();
    let canvas_color = Into::<Color>::into("#373737ff").into();
    let mut grids = generate_canvas_grids_from_logo_image(
        &img, fill_options, pick_strategy)?;

    grids.iter_mut().for_each(|grid| {
        let avg_color = grid.ext.avg_color.unwrap();
        let color_distance = grid.ext.color_distance.unwrap();
        println!(">>>>>>grid, seq:{:?}, selected: {}, avg_color:{:?}, color_distance: {}", 
            grid.seq, grid.ext.selected.unwrap(), avg_color.to_rgb(), color_distance);

        grid.ext.border_color = Some("#ffffffff".into());
        grid.ext.fill_color = Some("#9099A2ff".into());
        if grid.ext.selected.unwrap() {
            grid.ext.fill_color = Some("#ff0000ff".into());
        }
    });

    // 填充格子
    draw_canvas_with_grids(
        canvas_width, canvas_height, canvas_color,
        grids,
        "output/logo_mosaic_grids.png".into())?;
    println!(">>>>>>image saved as output/logo_mosaic_grids.png");
    Ok(())
}
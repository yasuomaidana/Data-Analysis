use std::path::PathBuf;
use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::coord::Shift;
use plotters::drawing::{DrawingArea, IntoDrawingArea};
use plotters::element::Circle;
use plotters::prelude::{BLUE, RED, RGBColor, WHITE};
use plotters::style::BLACK;
use polars::prelude::DataFrame;
use polars_io;

use polars_io::prelude::{CsvReader, CsvReadOptions};
use polars_io::SerReader;


fn color_map(diagnosis: &str) -> &RGBColor {
    match diagnosis {
        "M" => &BLUE,
        "B" => &RED,
        _ => &BLACK,
    }
}

fn relational_plot(image: &DrawingArea<BitMapBackend, Shift>,
                   x_name: &str,
                   y_name: &str,
                   z_name: &str,
                   data: &DataFrame,
                   color_map: &dyn Fn(&str) -> &RGBColor) {
    let x = data.column(x_name).expect("x column not found");
    let y = data.column(y_name).unwrap();
    let z = data.column(z_name).unwrap();

    let x_max = x.max::<f32>().unwrap().unwrap();
    let x_min = x.min::<f32>().unwrap().unwrap();
    let y_max = y.max::<f32>().unwrap().unwrap();
    let y_min = y.min::<f32>().unwrap().unwrap();


    let mut ctx = ChartBuilder::on(image)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Relational Plot", ("sans-serif", 40))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();



    ctx.configure_mesh()
        .draw()
        .unwrap();

    ctx.draw_series(
        z.str()
            .unwrap()
            .into_iter()
            .zip(x.f64().unwrap().into_iter().zip(y.f64().unwrap().into_iter()))
            .map(|(z, (x, y))| {
                Circle::new(
                    (x.unwrap() as f32, y.unwrap() as f32),
                    3,
                    color_map(z.unwrap())
                )
            }),
    )
        .unwrap();
}
fn main() {
    let filename = "data/KNNAlgorithmDataset.csv";
    let path = PathBuf::from(filename);

    let csv_options = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path))
        .unwrap();

    let data_frame = CsvReader::try_from(csv_options)
        .expect(format!("Error reading file: {}", filename).as_str()).finish()
        .expect(format!("Error reading file: {}", filename).as_str());

    println!("{:?}", data_frame);

    // Plotting the data
    let root_area =
        BitMapBackend::new("relational_plot.png", (1024, 768))
            .into_drawing_area();

    root_area.fill(&WHITE).unwrap();

    relational_plot(&root_area,
                    "radius_mean",
                    "texture_mean",
                    "diagnosis",
                    &data_frame, &color_map);

    root_area.present().expect("Error saving image");
}

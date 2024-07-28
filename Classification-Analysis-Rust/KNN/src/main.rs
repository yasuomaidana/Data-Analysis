use plotly::common::{Mode, Title};
use plotly::{ Layout, Plot};
use plotly::Scatter;
use polars::prelude::DataFrame;
use polars_io::prelude::{CsvReader, CsvReadOptions};
use polars_io::SerReader;
use std::path::PathBuf;
use plotly::ImageFormat::PNG;
use plotly::layout::Axis;

fn relational_plot(x_name: &str, y_name: &str, z_name: &str, data: &DataFrame) {
    let x = data.column(x_name).expect("x column not found");
    let y = data.column(y_name).unwrap();
    let z = data.column(z_name).unwrap();

    let x_values: Vec<f64> = x.f64().unwrap().into_iter().map(|v| v.unwrap()).collect();
    let y_values: Vec<f64> = y.f64().unwrap().into_iter().map(|v| v.unwrap()).collect();
    let z_values: Vec<&str> = z.str().unwrap().into_iter().map(|v| v.unwrap()).collect();

    let trace = Scatter::new(x_values, y_values)
        .mode(Mode::Markers)
        .marker(plotly::common::Marker::new().color_array(z_values.iter().map(|&v| match v {
            "M" => "blue",
            "B" => "red",
            _ => "black",
        }).collect()));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    let layout = Layout::new().x_axis(Axis::new().title(Title::from("X Axis")))
        .y_axis(Axis::new().title(Title::from("Y Axis")))
        .title(Title::from("My Plot"));
    plot.set_layout(layout);

    // plot.show(); showup the plot in the browser
    plot.write_image("KNN/relational_plot.png",PNG, 800, 600,1.0);

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
    relational_plot("radius_mean", "texture_mean", "diagnosis", &data_frame);
}

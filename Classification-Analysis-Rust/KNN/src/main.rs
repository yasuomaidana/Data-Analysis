use plotly::common::{Title};
use plotly::{Layout, Plot};
use data_frame_plotter::single_relational_plot;
use polars::prelude::DataFrame;
use polars_io::prelude::{CsvReader, CsvReadOptions};
use polars_io::SerReader;
use std::path::PathBuf;
use plotly::ImageFormat::PNG;
use plotly::layout::Axis;

fn plotting_relational_plot(x_name: &str, y_name: &str, z_name: &str, data: &DataFrame) {

    let trace = single_relational_plot(x_name, y_name, z_name, data);

    let x_axis = Vec::from([Some(Box::new(Axis::new().title(Title::from(x_name))))]);
    let y_axis = Vec::from([Some(Box::new(Axis::new().title(Title::from(y_name))))]);


    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .x_axis(x_axis)
        .y_axis(y_axis)
        .title(Title::from("My Plot title"));
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
    plotting_relational_plot("radius_mean", "texture_mean", "diagnosis", &data_frame);
}

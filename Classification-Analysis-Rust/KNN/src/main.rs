use data_frame_plotter::single_relational_plot;
use plotly::common::Title;
use plotly::layout::Axis;
use plotly::ImageFormat::PNG;
use plotly::{Layout, Plot};
use polars::prelude::DataFrame;
use polars_io::prelude::{CsvReadOptions, CsvReader};
use polars_io::SerReader;
use std::path::PathBuf;
use tokio_compat_02::FutureExt;
use data_loader::{download_file, KaggleFile};

fn plotting_relational_plot(x_name: &str, y_name: &str, z_name: &str, data: &DataFrame, title: &str) {
    let traces = single_relational_plot(x_name, y_name, z_name, data);

    let x_axis = Vec::from([Some(Box::new(Axis::new().title(Title::from(x_name))))]);
    let y_axis = Vec::from([Some(Box::new(Axis::new().title(Title::from(y_name))))]);

    let mut plot = Plot::new();
    for trace in traces {
        plot.add_trace(trace);
    }

    let layout = Layout::new()
        .x_axis(x_axis)
        .y_axis(y_axis)
        .title(Title::from(title));
    plot.set_layout(layout);

    // plot.show(); showup the plot in the browser
    plot.write_image("KNN/relational_plot.png", PNG, 800, 600, 1.0);
}

#[tokio::main]
async fn main() {
    let kaggle_file = KaggleFile::new_csv(
        "uciml/iris".to_string(),
        "Iris".to_string());
    download_file(kaggle_file.clone()).compat().await;
    let filename = kaggle_file.full_name();
    let path = PathBuf::from(&filename);

    let csv_options = CsvReadOptions::default()
        .with_has_header(true)
        // .with_skip_rows(2)
        .try_into_reader_with_file_path(Some(path))
        .unwrap();

    let data_frame = CsvReader::try_from(csv_options)
        .expect(format!("Error reading file: {}", &filename).as_str())
        .finish()
        .expect(format!("Error reading file: {}", &filename).as_str());

    println!("{:?}", data_frame);
    let data_frame = data_frame.drop_nulls::<String>(None).expect("Failed to drop null values");
    println!("{:?}", data_frame.get_column_names());
    // Plotting the data
    plotting_relational_plot("SepalLengthCm",
                             "SepalWidthCm",
                             "Species", &data_frame,
                             "Iris Dataset");

}

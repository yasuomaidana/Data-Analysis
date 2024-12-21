use data_frame_plotter::single_relational_plot;
use data_loader::{download_file, KaggleFile};
use itertools::Itertools;
use linfa::prelude::{Fit, Predict};
use linfa::Dataset;
use linfa_clustering::KMeans;
use linfa_nn::distance::{Distance, L2Dist};
use plotly::common::Title;
use plotly::layout::Axis;
use plotly::ImageFormat::PNG;
use plotly::{Layout, Plot};
use polars::prelude::{DataFrame, Float64Type, IndexOrder};
use polars_core::prelude::{NamedFrom, Series};
use polars_io::prelude::{CsvReadOptions, CsvReader};
use polars_io::SerReader;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tokio_compat_02::FutureExt;

fn plotting_relational_plot(
    x_name: &str,
    y_name: &str,
    z_name: &str,
    data: &DataFrame,
    title: &str,
) {
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
    plot.write_image(format!("k_means/{title}.png"), PNG, 800, 600, 1.0);
}

fn refine_labels(dataframe: &mut DataFrame, y_column: &str, y_pred_column: &str) {
    let categories = dataframe
        .select_series([y_column])
        .unwrap()
        .get(0)
        .expect("Non existent series")
        .unique()
        .unwrap();

    println!("{:?}", categories);
    let predicted_categories = dataframe
        .select_series([y_pred_column])
        .unwrap()
        .get(0)
        .unwrap()
        .unique()
        .unwrap();

    let mut best_labels: Series = Default::default();
    let mut best_label_count = 0;
    for perm in (0..predicted_categories.len()).permutations(predicted_categories.len()) {
        let casted_labels: Series = dataframe
            .column(y_pred_column)
            .unwrap()
            .iter()
            .map(|x| {
                categories
                    .get(perm[x.get_str().unwrap().to_string().parse::<usize>().unwrap()])
                    .unwrap()
                    .get_str()
                    .unwrap()
                    .to_string()
            })
            .collect();
        // println!("{:?}", casted_labels);
        let check: usize = casted_labels
            .iter()
            .zip(dataframe.column(y_column).unwrap().iter())
            .map(|(x, y)| {
                let x = x.get_str().unwrap();
                let y = y.get_str().unwrap();
                if x == y {
                    1
                } else {
                    0
                }
            })
            .sum();
        if check > best_label_count {
            best_label_count = check;
            best_labels = casted_labels;
        }
    }
    dataframe
        .replace_or_add(y_pred_column.into(), best_labels)
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct KMeansModel<F: linfa::Float, D: Distance<F>> {
    model: KMeans<F, D>,
}

#[tokio::main]
async fn main() {
    let kaggle_file = KaggleFile::new_csv("uciml/iris".to_string(), "Iris".to_string());
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
    let data_frame = data_frame
        .drop_nulls::<String>(None)
        .expect("Failed to drop null values");
    println!("{:?}", data_frame.get_column_names());
    // Plotting the data
    plotting_relational_plot(
        "SepalLengthCm",
        "SepalWidthCm",
        "Species",
        &data_frame,
        "Iris Dataset",
    );

    let x = data_frame
        .select(["SepalLengthCm", "SepalWidthCm"])
        .unwrap();
    let y = data_frame.select(["Species"]).unwrap();
    let features = x.to_ndarray::<Float64Type>(IndexOrder::C).unwrap();
    let target = y.to_ndarray::<Float64Type>(IndexOrder::C).unwrap();

    let dataset = Dataset::new(features, target);
    let n_clusters = 3;

    let k_means = KMeans::params(n_clusters)
        .tolerance(1e-4)
        .max_n_iterations(300)
        .fit(&dataset)
        .unwrap();

    let labels = k_means.predict(&dataset);
    println!("{:?}", labels);

    // let mut data_frame = data_frame.clone();
    let labels = labels
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>();
    let labels = Series::new("labels".into(), labels);
    let mut binding = data_frame.clone();
    let data_frame = binding.replace_or_add("labels".into(), labels).unwrap();

    refine_labels(data_frame, "Species", "labels");

    plotting_relational_plot(
        "SepalLengthCm",
        "SepalWidthCm",
        "labels",
        &data_frame,
        "Predicted labels",
    );

    let model = KMeansModel { model: k_means };

    let model = serde_json::to_string(&model).unwrap();
    fs::write("generated/k_means_model.json", model.clone()).expect("Unable to write file");
    
    let model: KMeansModel<f64, L2Dist> = serde_json::from_str(&model).unwrap();
    model.model.predict(&dataset);
}

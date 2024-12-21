use data_loader::{download_file, KaggleFile};
use polars::frame::DataFrame;
use polars::prelude::{CsvReadOptions, SerReader, SortMultipleOptions};
use tokio_compat_02::FutureExt;

#[tokio::main]
async fn main() {
    let kaggle_file = KaggleFile::new_csv(
        "unanimad/dataisbeautiful".to_string(),
        "r_dataisbeautiful_posts".to_string(),
    );
    download_file(kaggle_file.clone()).compat().await;
    println!("Downloaded file: {}", kaggle_file.full_name());
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(kaggle_file.full_name().into()))
        .expect("Failed to read csv file")
        .finish()
        .expect("Failed to read csv file");
    println!("{:?}", df.shape());
    println!("{:?}", df.head(Some(5)));
    println!("{:?}", df.schema());
    let selection = df
        .select_series(["title", "score", "num_comments"])
        .expect("Failed to select columns");
    let df = DataFrame::new(selection)
        .expect("Failed to create DataFrame")
        .drop_nulls::<String>(None)
        .expect("Failed to drop null values")
        .sort(
            ["score"],
            SortMultipleOptions::default().with_order_reversed(),
        )
        .expect("Failed to sort DataFrame");
    println!("{:?}", df.head(Some(5)));
}

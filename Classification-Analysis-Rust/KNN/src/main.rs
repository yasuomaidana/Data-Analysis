use std::path::PathBuf;
use polars_io;

use polars_io::prelude::{CsvReader, CsvReadOptions};
use polars_io::SerReader;

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

    
}

use polars::export::rayon::iter::IntoParallelRefIterator;
use polars::export::rayon::iter::ParallelIterator;
use polars::frame::DataFrame;
use polars_io::prelude::{CsvReadOptions, CsvReader};
use polars_io::SerReader;
use std::path::PathBuf;

pub fn read_dataframe(file_name: &str) -> DataFrame {
    // Read the data
    let filename = file_name;
    let path = PathBuf::from(&filename);

    let csv_options = CsvReadOptions::default()
        .with_has_header(true)
        .with_skip_rows(2)
        .try_into_reader_with_file_path(Some(path))
        .unwrap();

    CsvReader::try_from(csv_options)
        .expect(format!("Error reading file: {}", &filename).as_str())
        .finish()
        .expect(format!("Error reading file: {}", &filename).as_str())
}

pub fn find_columns_with_single_values(df: &DataFrame) -> Vec<String> {
    df.get_columns()
        .par_iter()
        .filter_map(|series| {
            if series.unique().unwrap().len() <= 1 {
                Some(series.name().to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_dataframe() {
        let filename = "../../Classification Analysis/KNN/Data-Mushroom/mushrooms.csv";
        let df = read_dataframe(filename);
        assert_eq!(df.shape(), (8124, 23));
    }
}

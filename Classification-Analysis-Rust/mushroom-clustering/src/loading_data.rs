use std::path::PathBuf;
use polars::frame::DataFrame;
use polars_io::prelude::{CsvReadOptions, CsvReader};
use polars_io::SerReader;

pub fn read_dataframe(file_name:&str)->DataFrame{
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_read_dataframe(){
        let filename = "../../Classification Analysis/KNN/Data-Mushroom/mushrooms.csv";
        let df = read_dataframe(filename);
        assert_eq!(df.shape(), (8124, 23));
    }
}
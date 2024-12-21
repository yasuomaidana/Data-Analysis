use polars::export::rayon::iter::IntoParallelRefIterator;
use polars::export::rayon::iter::ParallelIterator;
use polars::frame::DataFrame;
use std::path::PathBuf;
use polars::prelude::{LazyCsvReader, LazyFileListReader, LazyFrame};

pub fn read_dataframe(file_name: &str) -> LazyFrame {
    // Read the data
    let filename = file_name;
    let path = PathBuf::from(&filename);
    
    LazyCsvReader::new(path)
        .with_has_header(true)
        .with_skip_rows(2)
        .finish().expect(format!("Error reading file: {}", &filename).as_str())
        
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
        let df = df.collect().unwrap();
        assert_eq!(df.shape(), (8124, 23));
    }
}

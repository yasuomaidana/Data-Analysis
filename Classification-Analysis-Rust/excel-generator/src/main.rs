use polars::prelude::{CsvReadOptions, SerReader};
use rust_xlsxwriter::{ColNum, RowNum, Workbook};

fn main() {
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("data/r_dataisbeautiful_posts.csv".into()))
        .expect("Failed to read csv file")
        .finish()
        .expect("Failed to read csv file");
    let lines_by_excel_files = 6;
    let number_of_files = 5;
    for i in 0..number_of_files {
        let start = i * lines_by_excel_files;
        let sub_df = df.slice(start, lines_by_excel_files as usize);
        let file_name = format!("generated/r_dataisbeautiful_posts_{}.xlsx", i);
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        for (row_idx, row) in sub_df.iter().enumerate() {
            for (col_idx, value) in row.iter().enumerate() {
                worksheet
                    .write_string(col_idx as RowNum, row_idx as ColNum, value.to_string())
                    .expect("Failed to write to worksheet");
            }
        }
        workbook.save(file_name).unwrap()
    }
}

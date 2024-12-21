pub mod loading_data;

use crate::loading_data::{find_columns_with_single_values, read_dataframe};

fn main() {
    let filename = "../Classification Analysis/KNN/Data-Mushroom/mushrooms.csv";
    let df = read_dataframe(filename);
    println!("Head");
    println!("{:?}", df.head(Some(5)));
    
    let y = df.select(["class"]).unwrap();
    println!("{:?}", y.head(Some(2)));
    let x = df.drop("class").unwrap();
    println!("{:?}", x.head(Some(2)));

    println!("Columns with single value");
    let columns_to_drop = find_columns_with_single_values(&x);
    println!("{:?}", columns_to_drop);
    
    
}
use mushroom_clustering::loading_data;

fn main() {
    let filename = "../Classification Analysis/KNN/Data-Mushroom/mushrooms.csv";
    let df = loading_data::read_dataframe(filename);
    println!("{:?}", df.head(Some(5)));
}

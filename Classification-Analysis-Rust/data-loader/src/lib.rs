use std::path::PathBuf;
use kaggle::archive::unzip;
use kaggle::KaggleApiClient;
use tokio::fs::remove_file;

pub struct KaggleFile{
    dataset: String,
    name: String,
    file_type: String
}

impl KaggleFile{
    pub fn new_csv(dataset:String, name: String) -> Self{
        Self{
            dataset,
            name,
            file_type: "csv".to_string()
        }
    }
    pub fn file_name(&self) -> String{
        format!("{}.{}", self.name, self.file_type)
    }
}

pub async fn download_dataset(name: impl AsRef<str>) {
    let kaggle = KaggleApiClient::builder().build().unwrap();
    let path_buf = PathBuf::from("data");
    let data_set_name = name.as_ref().split("/").last().unwrap();
    // check if the dataset is already downloaded
    println!("Checking if dataset {} is already downloaded...", data_set_name);

    if path_buf.join(data_set_name.to_owned() +".zip").exists(){
        println!("Dataset already downloaded");
        return;
    }
    else{
        println!("Downloading dataset...");
        let dataset =kaggle
            .dataset_download_all_files(name, Some(path_buf), None)
            .await.expect("Failed to download dataset");
        unzip(&dataset, "data").expect("Failed to unzip dataset");
    }

}

pub async fn download_file(file: KaggleFile) {
    let dataset_name = &file.dataset;
    let path_buf = PathBuf::from("data");

    // check if the dataset is already downloaded
    println!("Checking if file {} is already downloaded...", file.file_name());
    let file_path = path_buf.join(file.file_name());
    if file_path.exists(){
        println!("File already downloaded");
        return;
    }
    else{
        println!("Downloading file...");
        let kaggle = KaggleApiClient::builder().build().unwrap();
        let file = kaggle.dataset_download_file(dataset_name, file.file_name(), Some(path_buf), None)
            .await.expect("Failed to download file");
        unzip(&file, "data").expect("Failed to unzip file");
        remove_file(&file).await.expect("Failed to remove file");
        println!("File downloaded in {:?}", file);
    }

}
#[cfg(test)]
mod tests {
    // use super::*;
}

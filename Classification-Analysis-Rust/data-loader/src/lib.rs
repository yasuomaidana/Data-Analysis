use std::path::PathBuf;
use kaggle::archive::unzip;
use kaggle::KaggleApiClient;

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
            .dataset_download_all_files("unanimad/dataisbeautiful", Some(path_buf), None)
            .await.expect("Failed to download dataset");
        unzip(&dataset, "data").expect("Failed to unzip dataset");
    }

}
#[cfg(test)]
mod tests {
    use super::*;

}

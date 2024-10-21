use data_loader::{ download_file, KaggleFile};
use tokio_compat_02::FutureExt;


#[tokio::main]
async fn main(){
    let kaggle_file = KaggleFile::new_csv("unanimad/dataisbeautiful".to_string(), "r_dataisbeautiful_posts".to_string());
    download_file(kaggle_file).compat().await;
}

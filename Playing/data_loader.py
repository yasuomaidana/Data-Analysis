import kaggle
def load_data(dataset:str, path:str="./Data"):
    kaggle.api.authenticate()
    kaggle.api.dataset_download_files(dataset,path,unzip=True)
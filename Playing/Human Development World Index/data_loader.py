import kaggle
kaggle.api.authenticate()  # Authenticates using your kaggle.json
kaggle.api.dataset_download_files('iamsouravbanerjee/human-development-index-dataset', path='./Data', unzip=True)
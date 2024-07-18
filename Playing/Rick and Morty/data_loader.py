import kaggle
kaggle.api.authenticate()  # Authenticates using your kaggle.json
kaggle.api.dataset_download_files('neuromusic/avocado-prices', path='./Data', unzip=True)
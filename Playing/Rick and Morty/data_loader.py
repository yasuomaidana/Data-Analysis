import kaggle
kaggle.api.authenticate()  # Authenticates using your kaggle.json
kaggle.api.dataset_download_files('andradaolteanu/rickmorty-scripts', path='./Data', unzip=True)
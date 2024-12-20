import zipfile
import os

if __name__ == '__main__':
    
    # Path to the zip file
    zip_file_path = 'Data-Mushroom.zip'
    # Directory to extract the contents
    extract_dir = 'Data-Mushroom'
    
    # Create the directory if it doesn't exist
    os.makedirs(extract_dir, exist_ok=True)
    
    # Unzip the file
    with zipfile.ZipFile(zip_file_path, 'r') as zip_ref:
        zip_ref.extractall(extract_dir)
    
    print(f'Contents of {zip_file_path} have been extracted to {extract_dir}')
mod model;

use mnist::{Mnist, MnistBuilder};
use tch::nn::Module;
use tch::{nn, Device, Tensor};

const TRAIN_SIZE: usize = 50000;
const VAL_SIZE: usize = 10000;
const TEST_SIZE: usize =10000;

pub fn image_to_tensor(data: Vec<u8>, dim1: usize, dim2: usize, dim3: usize) -> Tensor {
    // Convert vector of u8 to vector of f64
    let data_float: Vec<f64> = data.into_iter().map(|x| (x as f64) / 255.0).collect();

    // Create a tensor from the normalized data
    Tensor::from_slice(&data_float).view([dim1 as i64, dim2 as i64, dim3 as i64])
}

pub fn labels_to_tensor(data: Vec<u8>, dim1: usize, dim2: usize) -> Tensor {
    // Convert u8 to i64 since labels typically don't need floating point precision
    let labels_int: Vec<i64> = data.into_iter().map(|x| x as i64).collect();

    // Assuming dim2 would be 1 for typical classification tasks where each label is a single number
    // If your labels have a different structure, adjust accordingly
    Tensor::from_slice(&labels_int).view([dim1 as i64, dim2 as i64])
}

fn main() {
    // Create a simple neural network model
    let vs = nn::VarStore::new(Device::Mps);
    let net = nn::seq()
        .add(nn::linear(&vs.root(), 10, 5, Default::default()))
        .add(nn::linear(&vs.root(), 5, 10, Default::default()));

    // Create a dummy input tensor
    let input = Tensor::randn(&[1, 10], (tch::Kind::Float, Device::Mps));

    // Forward pass
    let output = net.forward(&input);

    // Print the shape of the output tensor
    println!("Output shape: {:?}", output.size());

    let Mnist {
        trn_img,
        trn_lbl,
        val_img,
        val_lbl,
        tst_img,
        tst_lbl,
    } = MnistBuilder::new()
        .download_and_extract()
        .label_format_digit()
        .training_set_length(TRAIN_SIZE as u32)
        .validation_set_length(VAL_SIZE as u32)
        .test_set_length(TEST_SIZE as u32)
        .finalize();

    // Convert images to tensors
    let trn_img_tensor = image_to_tensor(trn_img, TRAIN_SIZE, 28, 28);
    let val_img_tensor = image_to_tensor(val_img, VAL_SIZE, 28, 28);
    let tst_img_tensor = image_to_tensor(tst_img, TEST_SIZE, 28, 28);

    // Convert labels to tensors
    let trn_lbl_tensor = labels_to_tensor(trn_lbl, TRAIN_SIZE, 1);
    let val_lbl_tensor = labels_to_tensor(val_lbl, VAL_SIZE, 1);
    let tst_lbl_tensor = labels_to_tensor(tst_lbl, TEST_SIZE, 1);

    println!("Training set image tensor shape: {:?}", trn_img_tensor.size());
    println!("Training set label tensor shape: {:?}", trn_lbl_tensor.size());
    println!("Validation set image tensor shape: {:?}", val_img_tensor.size());
    println!("Validation set label tensor shape: {:?}", val_lbl_tensor.size());
    println!("Test set image tensor shape: {:?}", tst_img_tensor.size());
    println!("Test set label tensor shape: {:?}", tst_lbl_tensor.size());


}

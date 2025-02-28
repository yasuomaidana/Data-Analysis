mod model;

use crate::model::MnistCnn;
use mnist::{Mnist, MnistBuilder};
use std::time::Instant;
// use tch::nn::{Adam, Module, OptimizerConfig};
use tch::nn::{Adam, OptimizerConfig};
use tch::{nn, Device, Tensor};

const TRAIN_SIZE: usize = 50000;
const VAL_SIZE: usize = 10000;
const TEST_SIZE: usize = 10000;
const BATCH_SIZE: i64 = 256;
// const N_EPOCHS: i64 = 50;
const N_EPOCHS: i64 = 5;
const HEIGHT: usize = 28;
const WIDTH: usize = 28;

pub fn image_to_tensor(data: Vec<u8>, dim1: usize, dim2: usize, dim3: usize) -> Tensor {
    // Convert vector of u8 to vector of f64
    let data_float: Vec<f32> = data.into_iter().map(|x| (x as f32) / 255.0).collect();

    // Create a tensor from the normalized data
    Tensor::from_slice(&data_float).view([dim1 as i64, 1i64, dim2 as i64, dim3 as i64])
}

pub fn labels_to_tensor(data: Vec<u8>, dim1: usize) -> Tensor {
    // Convert u8 to i64 since labels typically don't need floating point precision
    let labels_int: Vec<i64> = data.into_iter().map(|x| x as i64).collect();

    // Assuming dim2 would be 1 for typical classification tasks where each label is a single number
    // If your labels have a different structure, adjust accordingly
    Tensor::from_slice(&labels_int).view([dim1 as i64])
}

fn main() {
    // Create a simple neural network model
    let device = Device::Mps;
    let mut vs = nn::VarStore::new(device);
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
    let train_data = image_to_tensor(trn_img, TRAIN_SIZE, HEIGHT, WIDTH).to_device(device);
    let train_lbl = labels_to_tensor(trn_lbl, TRAIN_SIZE).to_device(device);
    let test_data = image_to_tensor(tst_img, TEST_SIZE, HEIGHT, WIDTH).to_device(device);
    let test_lbl = labels_to_tensor(tst_lbl, TEST_SIZE).to_device(device);
    let val_data = image_to_tensor(val_img, VAL_SIZE, HEIGHT, WIDTH).to_device(device);
    let val_lbl = labels_to_tensor(val_lbl, VAL_SIZE).to_device(device);

    println!("Input shape: {:?}", train_data.size());
    println!("Label shape: {:?}", train_lbl.size());

    // Convert labels to tensors

    // Define the model
    let model = match MnistCnn::load(&mut vs, "mnist_model.ot") {
        Ok(model) => model,
        Err(_) => {
            let mut model = MnistCnn::new(&vs);
            // Define the optimizer
            let mut optimizer = Adam::default().build(&vs, 1e-3).unwrap();
            let criterion = |x: &Tensor, y: &Tensor| x.cross_entropy_for_logits(y);

            let start_time = Instant::now();

            // Train the model
            model.train(
                &train_data,
                &train_lbl,
                TRAIN_SIZE,
                &val_data,
                &val_lbl,
                BATCH_SIZE,
                N_EPOCHS,
                &mut optimizer,
                &criterion,
                device,
            );

            // Save the model
            model
                .save("mnist_model.ot", &vs)
                .expect("Failed to save model");

            let elapsed_time = start_time.elapsed();
            println!("Training time: {:?}", elapsed_time);
            model
        }
    };

    let accuracy = model.test(&test_data, &test_lbl, device);
    println!("Test accuracy: {}", accuracy);
    assert_ne!(
        accuracy, 0.0,
        "Model accuracy should be something different than 0.0"
    );
}

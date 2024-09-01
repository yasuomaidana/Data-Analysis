use std::fmt::{Debug, Formatter};
use std::path::Path;
use tch::nn::{conv2d, linear, Conv2D, Linear, VarStore};
use tch::{nn::ModuleT, TchError, Tensor};

// Renaming variables to follow Rust conventions
struct MnistCnn {
    conv_layer1: Conv2D,
    conv_layer2: Conv2D,
    fully_connected1: Linear,
    fully_connected2: Linear,
}

impl MnistCnn {
    fn new(vs: &VarStore) -> Self {
        let root = &vs.root();
        let conv_layer1 = conv2d(root, 1, 32, 5, Default::default());
        let conv_layer2 = conv2d(root, 32, 64, 5, Default::default()); // Assuming input size remains consistent
        // Correcting the input size for fc layer considering the output from previous layers
        // Assuming input image of 28x28, after convolutions and pooling, we get 7x7
        let fully_connected1 = linear(root, 1024, 256, Default::default());
        let fully_connected2 = linear(root, 256, 10, Default::default());

        MnistCnn { conv_layer1, conv_layer2, fully_connected1, fully_connected2 }
    }
    // Function to save the model
    pub fn save(&self, filename: &str, vs:&VarStore) -> Result<(), TchError> {
        let path = Path::new(filename);
        vs.save(path)?;
        Ok(())
    }

    // Function to load the model
    pub fn load(vs: &mut VarStore, filename: &str) -> Result<Self, TchError> {
        let path = Path::new(filename);
        vs.load(path)?;
        Ok(MnistCnn::new(vs))
    }
}

impl Debug for MnistCnn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MnistCnn")
            .field("conv_layer1", &self.conv_layer1)
            .field("conv_layer2", &self.conv_layer2)
            .field("fully_connected1", &self.fully_connected1)
            .field("fully_connected2", &self.fully_connected2)
            .finish()
    }
}

impl ModuleT for MnistCnn {
    fn forward_t(&self, xs: &Tensor, _train: bool) -> Tensor {
        // Initial input shape: (batch_size, 1, 28, 28)
        xs
            // After conv_layer1: (batch_size, 32, 28, 28) -> (batch_size, 32, 12, 12) due to padding and pooling
            .apply(&self.conv_layer1)  //(batch_size, 32, 24, 24)
            .relu()
            .max_pool2d_default(2) //(batch_size, 32, 12, 12)
            // After conv_layer2: (batch_size, 64, 12, 12) -> (batch_size, 64, 4, 4)
            .apply(&self.conv_layer2) //(batch_size, 64, 8, 8)
            .relu()
            .max_pool2d_default(2) //(batch_size, 64, 4, 4)
            // Flattening the output for the fully connected layer (batch_size, 64, 4, 4) -> (batch_size, 1024)
            .view([-1, 1024])
            // After fully_connected1: (batch_size, 256)
            .apply(&self.fully_connected1)
            .relu()
            // After fully_connected2: (batch_size, 10) for 10 classes of MNIST
            .apply(&self.fully_connected2) //(batch_size, 10)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use tch::nn::VarStore;
    use tch::{Device, Kind};


    #[test]
    fn test_forward_pass() {
        // Initialize the device (CPU or GPU if available)
        let device = Device::Mps; // or Device::Cuda(0) if CUDA is available

        // Create a var store for model parameters
        let vs = VarStore::new(device);

        // Initialize the model
        let model = MnistCnn::new(&vs);

        // Create a random input tensor (batch size 1, 1 channel, 28x28 image)
        let input = Tensor::randn(&[1, 1, 28, 28], (Kind::Float, device));

        // Forward pass
        let output = model.forward_t(&input, false);

        // Check if output shape is as expected (batch size 1, 10 classes)
        assert_eq!(output.size(), [1, 10]);
    }

    #[test]
    fn test_debug_format() {
        let device = Device::Mps; // Using CPU for simplicity in this test
        let vs = VarStore::new(device);
        let model = MnistCnn::new(&vs);

        let debug_output = format!("{:?}", model);
        assert!(debug_output.contains("MnistCnn"));
        assert!(debug_output.contains("conv_layer1"));
        assert!(debug_output.contains("conv_layer2"));
        assert!(debug_output.contains("fully_connected1"));
        assert!(debug_output.contains("fully_connected2"));
    }

    #[test]
    fn test_model_save_and_load() {

        // Create a temporary file_name for this test
        let file_path = "./tmp_model.pth";

        // Set up device
        let device = Device::Mps; // or use Device::Cuda(0) if available

        // Create a var store and model
        let vs = VarStore::new(device);
        let model = MnistCnn::new(&vs);

        // Save the model
        model.save(file_path, &vs).expect("Failed to save model");

        // Load the model into a new var store to simulate a fresh start
        let mut new_vs = VarStore::new(device);
        let loaded_model = MnistCnn::load(&mut new_vs, file_path).expect("Failed to load model");

        let debug_output = format!("{:?}", loaded_model);
        assert!(debug_output.contains("MnistCnn"));
        assert!(debug_output.contains("conv_layer1"));
        assert!(debug_output.contains("conv_layer2"));
        assert!(debug_output.contains("fully_connected1"));
        assert!(debug_output.contains("fully_connected2"));

        // Here you might want to check if the loaded model's weights are similar to the original model's weights
        // However, directly comparing tensors might require more complex logic or might not be straightforward due to how tch-rs handles tensors.

        // Attempt to delete the file
        fs::remove_file(file_path).expect("Could not remove temporary file");
        // Optionally, you could check if the file was indeed deleted
        assert!(!Path::new(file_path).exists(), "The model file was not deleted after the test");
    }
}
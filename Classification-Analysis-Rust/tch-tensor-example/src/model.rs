use std::fmt::{Debug, Formatter};
use tch::nn::{conv2d, linear, Conv2D, Linear, VarStore};
use tch::{nn::ModuleT, Tensor};

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
}

impl Debug for MnistCnn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MnistCnn")
    }
}

impl ModuleT for MnistCnn {
    fn forward_t(&self, xs: &Tensor, _train: bool) -> Tensor {
        xs
            .apply(&self.conv_layer1)
            .relu()
            .max_pool2d_default(2)
            .apply(&self.conv_layer2)
            .relu()
            .max_pool2d_default(2)
            // Flattening the output for the fully connected layer
            .view([-1, 1024])
            .apply(&self.fully_connected1)
            .relu()
            .apply(&self.fully_connected2)
    }
}

#[cfg(test)]
mod tests {
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
}
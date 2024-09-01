use std::fmt::{Debug, Formatter};
use tch::{nn, Tensor, nn::Module};

struct MNIST_CNN {
    conv1: nn::Conv2D,
    conv2: nn::Conv2D,
    fc1: nn::Linear,
    fc2: nn::Linear,
}

impl MNIST_CNN {
    fn new(vs: &nn::Path) -> Self {
        let conv1 = nn::conv2d(vs, 1, 32, 5, Default::default());
        let conv2 = nn::conv2d(vs, 32, 64, 5, Default::default());
        let fc1 = nn::linear(vs, 1024, 256, Default::default());
        let fc2 = nn::linear(vs, 256, 10, Default::default());

        MNIST_CNN { conv1, conv2, fc1, fc2 }
    }
}

impl Debug for MNIST_CNN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MNIST_CNN")
    }
}

impl Module for MNIST_CNN {
    fn forward(&self, xs: &Tensor) -> Tensor {
        let mut x = xs.apply(&self.conv1);
        x = x.relu();
        x = x.max_pool2d_default(2);

        x = x.apply(&self.conv2);
        x = x.relu();
        x = x.max_pool2d_default(2);

        // x = x.view([-1, 7*7*64]);
        x = x.view([-1, 1024]);
        x = x.apply(&self.fc1);
        x = x.relu();

        x.apply(&self.fc2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tch::{nn, Device, Tensor};

    #[test]
    fn test_mnist_cnn_forward() {
        let device = Device::Cpu;  // or Device::Cuda if you have CUDA
        let vs = nn::VarStore::new(device);
        let model = MNIST_CNN::new(&vs.root());

        // Generate a random input tensor for testing
        let batch_size = 2;
        let input_tensor = Tensor::rand(&[batch_size, 1, 28, 28], (tch::Kind::Float, device));

        println!("Input shape: {:?}", input_tensor.size());
        // // Perform a forward pass
        let output = model.forward(&input_tensor);

        // Output shape should be (batch_size, 10)
        println!("Output shape: {:?}", output.size());
        assert_eq!(output.size(), &[batch_size, 10]);
    }
}

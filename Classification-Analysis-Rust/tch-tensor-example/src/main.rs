use tch::nn::Module;
use tch::{nn, Tensor};

fn main() {
    // Create a simple neural network model
    let vs = nn::VarStore::new(tch::Device::Cpu);
    let net = nn::seq()
        .add(nn::linear(&vs.root(), 10, 5, Default::default()))
        .add(nn::linear(&vs.root(), 5, 10, Default::default()));

    // Create a dummy input tensor
    let input = Tensor::randn(&[1, 10], (tch::Kind::Float, tch::Device::Cpu));

    // Forward pass
    let output = net.forward(&input);

    // Print the shape of the output tensor
    println!("Output shape: {:?}", output.size());
}

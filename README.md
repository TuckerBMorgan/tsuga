[![crates.io](https://img.shields.io/crates/v/tsuga.svg)](https://crates.io/crates/tsuga)
[![Documentation](https://docs.rs/tsuga/badge.svg)](https://docs.rs/tsuga)

## Tsuga
### An early-stage machine learning library in Rust

Tsuga is an early-stage machine learning library in Rust for building neural networks. It uses `ndarray` as the linear algebra backend, and operates primarily on two-dimensional `f32` arrays (`Array2<f32>` types). At the moment, it's primary function has been for testing out various ideas for APIs, as an educational exercise, and probably isn't yet suitable for serious use. Most of the project's focus so far has been on the image-processing domain, although the tools  and layout should generally applicable to higher/lower-dimensional datasets as well.

To use `tsuga` as a library, add the following to your `Cargo.toml` file:
```toml
[dependencies]
tsuga = "0.1"
ndarray = "0.13"
```

For development, I recommend cloning only the most recent version--the training data has been included in past commits, which can lead to unnecessarily large file sizes for the entire history. This can be done using
```bash
$ git clone --depth=1 https://github.com/quietlychris/tsuga.git
```

### Fully-Connected Network Example for MNIST
Tsuga currently uses the [Builder](https://xaeroxe.github.io/init-struct-pattern/) pattern for constructing fully-connected networks. Since networks are complex compound structures, this pattern helps to make the layout of the network explicit and modular.

The following is a reduced-code example of building a network to train on/evaluate the MNIST (or Fashion MNIST) data set. Including unpacking the MNIST binary files, this network achieves:
- An accuracy of ~91.5% over 1000 iterations in 3.65 seconds  
- An accuracy of ~97.1% over 10,000 iterations in 29.43 seconds

This example can be run using `$ cargo run --release --example mnist`


```rust
use ndarray::prelude::*;
use tsuga::prelude::*;

fn main() {
    // Builds the MNIST data from a binary into ndarray Array2<f32> structures
    // labels are built with one-hot encoding format
    // ([60_000, 784], [60_000, 10], [10_000, 784], [10_000, 10] )
    let (input, output, test_input, test_output) = mnist_as_ndarray();
    println!("Successfully unpacked the MNIST dataset into Array2<f32> format!");

    let mut layers_cfg: Vec<FCLayer> = Vec::new();
    let sigmoid_layer_0 = FCLayer::new("sigmoid", 128);
    layers_cfg.push(sigmoid_layer_0);
    let sigmoid_layer_1 = FCLayer::new("sigmoid", 64);
    layers_cfg.push(sigmoid_layer_1);

    let mut fcn = FullyConnectedNetwork::default(input, output)
        .add_layers(layers_cfg)
        .iterations(1000)
        .learnrate(0.01)
        .batch_size(200)
        .build();

    fcn.train();

    println!("Test input shape = {:?}", test_input.shape());
    println!("Test output shape = {:?}", test_output.shape());

    let test_result = fcn.evaluate(test_input);
    compare_results(test_result, test_output);
}
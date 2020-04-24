extern crate ndarray as nd;
use nd::prelude::*;

use serde::{Deserialize, Serialize};

use crate::fc_layer::*;
use crate::fc_network::*;

#[derive(Debug, Clone)]
pub struct Model {
    pub w: Vec<Array2<f64>>,
    pub layers_cfg: Vec<FCLayer>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ModelAsVec {
    pub w: Vec<Vec<f64>>,
    pub shape: (usize, usize),
    pub layers_cfg: Vec<FCLayer>,
}

impl Model {
    pub fn evaluate(self, input: Array2<f64>) -> Array2<f64> {
        let l = self.layers_cfg.len();
        let output: Array2<f64> =
            Array::zeros((input.shape()[0], self.layers_cfg[l - 1].output_size));
        let addl_layers = self.layers_cfg.clone().drain(0..l - 1).collect();
        let mut network = FullyConnectedNetwork::default(input, output.clone())
            .add_layers(addl_layers)
            .iterations(1)
            .build();
        network.update_weights(self.w);
        network.forward_pass();

        // println!("Network built from model:\n{:#?}",network);
        network.a[l].clone()
    }

    pub fn to_toml() {}
}

// TO_DO: Still working on saving the model
// Serde doesn't like the Array structure...actually, we might just be able to convert the Array to String
// with the .to_string() method (or maybe .as_bytes()) which could be fine
impl ModelAsVec {
    /*fn from_model(model: Model) -> Self {
        for i in 0..model.layers_cfg.len()
            ModelAsVec {

            }
    }*/
}
use super::layer::Layer;

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(layout: Vec<u32>) -> Network {
        let mut layers = Vec::with_capacity(layout.len());

        for (index, nb_neurons) in layout.into_iter().enumerate() {
            layers.push(Layer::new(nb_neurons, find_previous_layer(&layers, index)));
        }

        Network {
            layers,
        }
    }

    pub fn predict(&mut self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers.get(0).expect("Network has no layer ?!").set_input_value(inputs);

        for layer in self.layers.iter_mut().skip(1) {
            layer.compute_layer();
        }

        self.layers.last().unwrap().get_values()
    }
}

// -------
// HELPERS
// -------

fn find_previous_layer(layers: &Vec<Layer>, index: usize) -> Option<&Layer> {
    match index {
        0 => None,
        i => layers.get(i - 1),
    }
}

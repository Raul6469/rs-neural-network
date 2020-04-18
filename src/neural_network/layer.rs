use std::rc::Rc;
use std::cell::RefCell;

use super::neuron::Neuron;

pub struct Layer {
    neurons: Vec<Rc<RefCell<Neuron>>>,
}

impl Layer {
    pub fn new(nb_neurons: u32, previous_layer: Option<&Layer>) -> Layer {
        let mut neurons = Vec::with_capacity(nb_neurons as usize);

        let predecessors: Option<&Vec<Rc<RefCell<Neuron>>>> = match previous_layer {
            Some(previous_layer) => Some(&previous_layer.get_neurons()),
            None => None,
        };

        for _ in 0..nb_neurons {
            let neuron = Neuron::new(&predecessors);
            neurons.push(Rc::new(RefCell::new(neuron)));
        }
        
        Layer {
            neurons,
        }
    }

    pub fn get_neurons(&self) -> &Vec<Rc<RefCell<Neuron>>> {
        &self.neurons
    }

    pub fn get_values(&self) -> Vec<f32> {
        let mut values = Vec::with_capacity(self.neurons.len());

        for neuron in self.neurons.iter() {
            values.push(neuron.borrow().get_inner_value());
        }

        values
    }

    pub fn set_input_value(&self, inputs: Vec<f32>) {
        assert_eq!(self.neurons.len(), inputs.len(), "Number of inputs and of neurons in first layer must match");

        for i in 0..self.neurons.len() {
            self.neurons
                .get(i).expect("Neuron not found")
                .borrow_mut()
                .set_value(*inputs.get(i).expect("No input value found"));
        }
    }

    pub fn compute_layer(&mut self) {
        for neuron in self.neurons.iter() {
            neuron.borrow_mut().compute_value();
        }
    }
}

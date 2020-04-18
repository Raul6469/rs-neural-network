use std::rc::Rc;
use std::cell::RefCell;

use super::synapse::Synapse;

const ACTIVATION_THRESHOLD: f32 = 0.5f32;

#[derive(Debug)]
pub struct Neuron {
    synapses: Vec<Synapse>,
    value: Option<f32>,
}

impl Neuron {
    pub fn new(predecessors: &Option<&Vec<Rc<RefCell<Neuron>>>>) -> Neuron {
        let mut synapses = Vec::new();

        if let Some(predecessors) = predecessors {
            for predecessor in *predecessors {
                synapses.push(Synapse::new(predecessor));
            }
        }
        
        Neuron {
            synapses,
            value: None,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = Some(value);
    }

    pub fn get_inner_value(&self) -> f32 {
        self.value.expect("Neuron inner value was not computed yet")
    }

    pub fn compute_value(&mut self) -> f32 {
        let combination_value = self.synapses.iter().fold(0f32, |acc, synapse| {
            acc + synapse.get_weighted_input()
        });

        let value = combination_value * activation_function(combination_value);

        self.value = Some(value);

        value
    }
}

fn activation_function(value: f32) -> f32 {
    if sigmoid(value) >= ACTIVATION_THRESHOLD {
        1f32
    } else {
        0f32
    }
}

fn sigmoid(value: f32) -> f32 {
    1f32 / (1f32 - (-value).exp())
}
